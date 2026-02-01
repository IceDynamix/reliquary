//! Parse network packets transmitted between the game and the server
//!
//! Packets are built up in following layers depending on the purpose of the packet:
//!
//! - Packets for connection management ([`GamePacket::Connection`])
//!     - **Ethernet/IP/UDP**, handled using [`etherparse`]
//!     - **[`ConnectionPacket`]**, containing events for connection establishment/disconnection
//! - Packets for game commands ([`GamePacket::Commands`])
//!     - **Ethernet/IP/UDP**, handled using [`etherparse`]
//!     - **KCP**, handled using [`kcp`]
//!         - The KCP header contains an extra field that needs to be removed
//!           to be compatible with the regular KCP protocol
//!     - **[`GameCommand`]**, encrypted using XOR
//!     - **Protobuf**, payload, needs to be parsed into using the types generated in [`gen::proto`]
//!
//! [`GameCommand`]s are encrypted using an XOR-key.
//! One of the first packets sent is a request for a new key from a seed.
//! That key is used for the rest of the packets.
//! This means the recording for packets needs to start before the game starts (train hyperdrive).
//!
//! ## Example
//! ```
//! use reliquary::network::{GamePacket, GameSniffer, ConnectionPacket};
//!
//! let packets: Vec<Vec<u8>> = vec![/**/];
//!
//! let mut sniffer = GameSniffer::new();
//! for packet in packets {
//!     match sniffer.receive_packet(packet) {
//!         Ok(packets) => {
//!             for pkt in packets {
//!                 match pkt {
//!                     GamePacket::Connection(ConnectionPacket::Disconnected) => {
//!                         println!("Disconnected!");
//!                         break;
//!                     }
//!                     GamePacket::Commands { conv_id, result: Ok(command) } => {
//!                         println!("[{}] {:?}", conv_id, command.get_command_name());
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!

use std::collections::{HashMap, VecDeque};
use std::fmt::Write;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use thiserror::Error;
use tracing::{info, info_span, instrument, trace, warn};

use crate::network::command::{GameCommand, GameCommandError};
use crate::network::connection::{parse_connection_packet, ConnectionPacketError};
use crate::network::crypto::{decrypt_command, get_game_version, lookup_initial_key, new_key_from_seed};
use crate::network::command::proto::PlayerGetTokenScRsp::PlayerGetTokenScRsp;
use crate::network::kcp::{KcpError, KcpSniffer};
use crate::network::command::command_id;

fn bytes_as_hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

pub mod command;

mod connection;
mod crypto;
mod kcp;

const PORTS: [u16; 2] = [23301, 23302];

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    ConnectionPacket(#[from] ConnectionPacketError),
    #[error(transparent)]
    Kcp(#[from] KcpError),
    #[error(transparent)]
    GameCommand(#[from] GameCommandError),
}

/// Top-level packet sent by the game
pub enum GamePacket {
    Connection(ConnectionPacket),
    Commands {
        conv_id: u32,
        result: Result<GameCommand, GameCommandError>,
    },
}

/// Packet for connection management
pub enum ConnectionPacket {
    HandshakeRequested,
    Disconnected,
    HandshakeEstablished { conv_id: u32 },
    SegmentData(PacketDirection, Vec<u8>),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PacketDirection {
    Sent,
    Received,
}

/// Per-conversation state for KCP and command handling
struct ConversationSniffer {
    conv_id: u32,
    sent_kcp: KcpSniffer,
    recv_kcp: KcpSniffer,
    key: Option<Vec<u8>>,
    // Commands sent by client that couldn't be decrypted due to a version mismatch.
    // We buffer the raw command bytes and retry after we learn the new session key
    // from PlayerGetTokenScRsp.
    pending_sent: VecDeque<Vec<u8>>,
    // Commands that were successfully retried after setting the new key. These are
    // emitted with the next receive_packet() response.
    deferred_commands: Vec<GameCommand>,
}

impl ConversationSniffer {
    fn new(conv_id: u32) -> Self {
        Self {
            conv_id,
            sent_kcp: KcpSniffer::new(conv_id),
            recv_kcp: KcpSniffer::new(conv_id),
            key: None,
            pending_sent: VecDeque::new(),
            deferred_commands: Vec::new(),
        }
    }

    fn receive_kcp_segment(
        &mut self,
        direction: PacketDirection,
        kcp_seg: &[u8],
        initial_keys: &HashMap<u32, Vec<u8>>,
    ) -> Result<Vec<Result<GameCommand, GameCommandError>>, KcpError> {
        let kcp = match direction {
            PacketDirection::Sent => &mut self.sent_kcp,
            PacketDirection::Received => &mut self.recv_kcp,
        };

        kcp.receive_segments(kcp_seg)
            .map(|segments| {
                segments
                    .into_iter()
                    .map(|data| self.receive_command(direction, data, initial_keys))
                    .collect()
            })
    }

    #[instrument(skip_all, fields(conv_id = self.conv_id, len = data.len()))]
    fn receive_command(
        &mut self,
        direction: PacketDirection,
        mut data: Vec<u8>,
        initial_keys: &HashMap<u32, Vec<u8>>,
    ) -> Result<GameCommand, GameCommandError> {
        // Keep original bytes in case we need to buffer on version mismatch (client-sent only)
        let original_data = if matches!(direction, PacketDirection::Sent) {
            Some(data.clone())
        } else {
            None
        };

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => match lookup_initial_key(initial_keys, get_game_version(&data)) {
                Some(key) => {
                    self.key = Some(key);
                    self.key.as_ref().unwrap()
                }
                None => return Err(GameCommandError::DecryptionKeyMissing),
            },
        };

        decrypt_command(key, &mut data);

        let decrypted_version = get_game_version(&data);
        if decrypted_version != 0 {
            warn!(
                decrypted = decrypted_version,
                "decrypted version does not match expected version"
            );
            
            // If the client sent this command, we likely used an outdated key.
            // Buffer the original bytes and retry after we get PlayerGetTokenScRsp.
            if let Some(orig) = original_data {
                self.pending_sent.push_back(orig);
            }
            return Err(GameCommandError::VersionMismatch);
        }

        let command = GameCommand::try_new(data)?;

        let span = info_span!("command", ?command);
        let _enter = span.enter();

        info!("received");
        trace!(data = BASE64_STANDARD.encode(&command.proto_data), "data");

        if command.command_id == command_id::PlayerGetTokenScRsp {
            let token_command = command.parse_proto::<PlayerGetTokenScRsp>().unwrap();
            let seed = token_command.secret_key_seed;
            info!(?seed, "setting new session key");
            self.key = Some(new_key_from_seed(seed));

            // Now that we have the new session key, retry any buffered client-sent
            // commands that previously failed with VersionMismatch.
            while let Some(bytes) = self.pending_sent.pop_front() {
                match self.receive_command(PacketDirection::Sent, bytes.clone(), initial_keys) {
                    Ok(cmd) => {
                        // Defer emission to receive_packet so we can include
                        // these in the same output batch.
                        self.deferred_commands.push(cmd);
                    }
                    Err(GameCommandError::VersionMismatch) => {
                        // Still mismatched; push back and stop to avoid a busy loop.
                        // We'll try again when another key update occurs.
                        // (Shouldn't normally happen.)
                        // Re-queue at the front to preserve order for next retry.
                        self.pending_sent.push_front(bytes);
                        break;
                    }
                    Err(e) => {
                        warn!(?e, "failed to retry buffered sent command");
                    }
                }
            }
        }

        Ok(command)
    }

    fn take_deferred_commands(&mut self) -> Vec<GameCommand> {
        std::mem::take(&mut self.deferred_commands)
    }
}

#[derive(Default)]
pub struct GameSniffer {
    conversations: HashMap<u32, ConversationSniffer>,
    initial_keys: HashMap<u32, Vec<u8>>,
}

impl GameSniffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_initial_keys(mut self, initial_keys: HashMap<u32, Vec<u8>>) -> Self {
        self.initial_keys = initial_keys;
        self
    }

    #[instrument(skip_all, fields(len = bytes.len()))]
    pub fn receive_packet(&mut self, bytes: Vec<u8>) -> Result<Vec<GamePacket>, NetworkError> {
        let packet = parse_connection_packet(&PORTS, bytes)?;

        match packet {
            ConnectionPacket::HandshakeRequested => {
                info!("handshake requested");
                Ok(vec![GamePacket::Connection(packet)])
            }

            ConnectionPacket::HandshakeEstablished { conv_id } => {
                info!(conv_id, "handshake established, creating new conversation");
                self.conversations.insert(conv_id, ConversationSniffer::new(conv_id));
                Ok(vec![GamePacket::Connection(packet)])
            }

            ConnectionPacket::Disconnected => {
                Ok(vec![GamePacket::Connection(packet)])
            }

            ConnectionPacket::SegmentData(direction, kcp_seg) => {
                let conv_id = kcp::validate_kcp_segment(&kcp_seg)?;
                
                // Get or create conversation
                let conversation = self.conversations
                    .entry(conv_id)
                    .or_insert_with(|| ConversationSniffer::new(conv_id));

                let mut commands: Vec<GamePacket> = conversation
                    .receive_kcp_segment(direction, &kcp_seg, &self.initial_keys)?
                    .into_iter()
                    .map(|result| GamePacket::Commands { conv_id, result })
                    .collect();

                // Emit any deferred commands from key update
                for cmd in conversation.take_deferred_commands() {
                    commands.push(GamePacket::Commands { conv_id, result: Ok(cmd) });
                }

                Ok(commands)
            }
        }
    }
}
