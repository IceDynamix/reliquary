use std::fmt::Write;

fn bytes_as_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
}

use tracing::{debug, info, info_span, instrument, warn};

use crate::sniffer::gen::command_id::command_id_to_str;
use crate::sniffer::gen::proto::PlayerGetTokenScRsp::PlayerGetTokenScRsp;
use crate::sniffer::connection::{ConnectionPacket, parse_connection_packet};
use crate::sniffer::crypto::{decrypt_command, new_key_from_seed};
use crate::sniffer::kcp::KcpSniffer;

pub mod connection;
pub mod gen;

mod kcp;
mod crypto;

const HSR_PORTS: [u16; 2] = [23301, 23302];

pub enum GamePacket {
    Connection(ConnectionPacket),
    Commands(Vec<GameCommand>),
}

// layout
// indices       type   name
//   0..4        u32    header magic const
//   0..6        u16    packet type id
//   6..8        u16    header length (unsure)
//   8..12       u32    data length
//  12..12+len   var    protobuf data
// len..len+4    u32    tail magic const
#[derive(Debug, Clone)]
pub struct GameCommand {
    command_id: u16,
    #[allow(unused)]
    header_len: u16,
    #[allow(unused)]
    data_len: u32,
    proto_data: Vec<u8>,
}

impl GameCommand {
    const HEADER_LEN: usize = 12;
    const TAIL_LEN: usize = 4;

    #[instrument(skip(bytes), fields(len = bytes.len()))]
    pub fn try_new(bytes: Vec<u8>) -> Option<Self> {
        let header_overhead = Self::HEADER_LEN + Self::TAIL_LEN;
        if bytes.len() < header_overhead {
            warn!(len=bytes.len(), "game command header incomplete");
            return None;
        }

        // skip header magic const
        let command_id = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        let header_len = u16::from_be_bytes(bytes[6..8].try_into().unwrap());
        let data_len = u32::from_be_bytes(bytes[8..12].try_into().unwrap());

        let data = bytes[12..12 + data_len as usize].to_vec();
        Some(GameCommand {
            command_id,
            header_len,
            data_len,
            proto_data: data,
        })
    }

    pub fn get_command_name(&self) -> Option<&str> {
        command_id_to_str(self.command_id)
    }

    pub fn parse_proto<T: protobuf::Message>(&self) -> protobuf::Result<T> {
        T::parse_from_bytes(&self.proto_data)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PacketDirection {
    Sent,
    Received,
}

#[derive(Default)]
pub struct GameSniffer {
    sent_kcp: Option<KcpSniffer>,
    recv_kcp: Option<KcpSniffer>,
    key: Option<Vec<u8>>,
}

impl GameSniffer {
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip_all, fields(len = bytes.len()))]
    pub fn receive_packet(&mut self, bytes: Vec<u8>) -> Option<GamePacket> {
        let packet = parse_connection_packet(&HSR_PORTS, bytes)?;
        match packet {
            ConnectionPacket::HandshakeRequested
            | ConnectionPacket::HandshakeEstablished
            | ConnectionPacket::Disconnected => {
                Some(GamePacket::Connection(packet))
            }

            ConnectionPacket::SegmentData(direction, kcp_seg) => {
                let commands = self.receive_kcp_segment(direction, &kcp_seg);
                match commands {
                    Some(commands) => {
                        Some(GamePacket::Commands(commands))
                    }
                    None => {
                        Some(GamePacket::Connection(ConnectionPacket::SegmentData(direction, kcp_seg)))
                    }
                }
            }
        }
    }

    fn receive_kcp_segment(&mut self, direction: PacketDirection, kcp_seg: &[u8]) -> Option<Vec<GameCommand>> {
        let kcp = match direction {
            PacketDirection::Sent => &mut self.sent_kcp,
            PacketDirection::Received => &mut self.recv_kcp
        };

        if let None = kcp {
            let new_kcp = KcpSniffer::try_new(&kcp_seg)?;
            *kcp = Some(new_kcp);
        }

        if let Some(kcp) = kcp {
            let commands = kcp.receive_segments(kcp_seg)
                .into_iter()
                .filter_map(|data| self.receive_command(data))
                .collect();

            return Some(commands);
        }

        None
    }

    fn receive_command(&mut self, mut data: Vec<u8>) -> Option<GameCommand> {
        decrypt_command(&mut self.key, &mut data);
        let command = GameCommand::try_new(data)?;

        match command.get_command_name() {
            Some(command_name) => {
                let span = info_span!("command", command_name);
                let _enter = span.enter();

                info!("received");

                if command_name == "PlayerGetTokenScRsp" {
                    let token_command = command.parse_proto::<PlayerGetTokenScRsp>().unwrap();
                    let seed = token_command.secret_key_seed;
                    info!(?seed, "setting new session key");
                    self.key = Some(new_key_from_seed(seed));
                }

                Some(command)
            }
            None => {
                debug!(command.command_id, "undefined");
                None
            }
        }
    }
}
