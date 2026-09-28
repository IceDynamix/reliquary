//! Contains generated code to parse the payload of [`crate::network::GameCommand`]s
//!
//! For code generation, refer to [reliquary-codegen](https://github.com/IceDynamix/reliquary-codegen)
//!
//! [reliquary::network::GamePacket]s can be parsed into the corresponding protobuf struct like this
//! ```no_run
//! use reliquary::network::GameCommand;
//! use reliquary::network::command::proto::PlayerGetTokenScRsp::PlayerGetTokenScRsp;
//!
//! let command: GameCommand;
//! let parsed = command.parse_proto::<PlayerGetTokenScRsp>().unwrap();
//! ```
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::fmt;
use thiserror::Error;
use tracing::{instrument, warn};

pub mod command_id;

#[cfg(all(not(feature = "proto-limited"), not(feature = "proto-rqa"), not(feature = "proto-auth")))]
pub mod proto;

#[cfg(any(feature = "proto-limited",feature = "proto-rqa",feature = "proto-auth"))]
pub mod proto {
    #[cfg(feature = "proto-auth")]
    pub mod GetAuthkeyScRsp;
    pub mod BlackInfo;
    pub mod PlayerGetTokenScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod Avatar;
    #[cfg(feature = "proto-rqa")]
    pub mod AvatarPathData;
    #[cfg(feature = "proto-rqa")]
    pub mod AvatarPathSkillTree;
    #[cfg(feature = "proto-rqa")]
    pub mod AvatarSync;
    #[cfg(feature = "proto-rqa")]
    pub mod DoGachaScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod EquipRelic;
    #[cfg(feature = "proto-rqa")]
    pub mod Equipment;
    #[cfg(feature = "proto-rqa")]
    pub mod GachaCeiling;
    #[cfg(feature = "proto-rqa")]
    pub mod GachaCeilingAvatar;
    #[cfg(feature = "proto-rqa")]
    pub mod GachaInfo;
    #[cfg(feature = "proto-rqa")]
    pub mod GachaItem;
    #[cfg(feature = "proto-rqa")]
    pub mod GetAvatarDataScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod GetBagScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod GetGachaInfoScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod GroupStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod HeadFrameInfo;
    #[cfg(feature = "proto-rqa")]
    pub mod HeadIconData;
    #[cfg(feature = "proto-rqa")]
    pub mod Item;
    #[cfg(feature = "proto-rqa")]
    pub mod ItemList;
    #[cfg(feature = "proto-rqa")]
    pub mod Material;
    #[cfg(feature = "proto-rqa")]
    pub mod MessageGroupStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod MessageSectionStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod Mission;
    #[cfg(feature = "proto-rqa")]
    pub mod MissionCustomValue;
    #[cfg(feature = "proto-rqa")]
    pub mod MissionStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod MissionSync;
    #[cfg(feature = "proto-rqa")]
    pub mod PileItem;
    #[cfg(feature = "proto-rqa")]
    pub mod PlayerBasicInfo;
    #[cfg(feature = "proto-rqa")]
    pub mod PlayerLoginScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod PlayerSyncScNotify;
    #[cfg(feature = "proto-rqa")]
    pub mod Quest;
    #[cfg(feature = "proto-rqa")]
    pub mod QuestStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod Relic;
    #[cfg(feature = "proto-rqa")]
    pub mod RelicAffix;
    #[cfg(feature = "proto-rqa")]
    pub mod SectionStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod SetAvatarEnhancedIdScRsp;
    #[cfg(feature = "proto-rqa")]
    pub mod SyncStatus;
    #[cfg(feature = "proto-rqa")]
    pub mod TurnFoodSwitch;
    #[cfg(feature = "proto-rqa")]
    pub mod WaitDelResource;

    // Untranslated dependencies
    #[cfg(feature = "proto-rqa")]
    pub mod APAMFCKFHLL;
    #[cfg(feature = "proto-rqa")]
    pub mod BKGPJOBNMKJ;
    #[cfg(feature = "proto-rqa")]
    pub mod CCHMHOONEGG;
    #[cfg(feature = "proto-rqa")]
    pub mod CFMDKNCDDAL;
    #[cfg(feature = "proto-rqa")]
    pub mod CJLCPMDGIBO;
    #[cfg(feature = "proto-rqa")]
    pub mod IAIDGGGMBPJ;
    #[cfg(feature = "proto-rqa")]
    pub mod LHCBKDNHGCG;
    #[cfg(feature = "proto-rqa")]
    pub mod MJBANFPPEFH;
    #[cfg(feature = "proto-rqa")]
    pub mod NEIMLKNMDBM;
    #[cfg(feature = "proto-rqa")]
    pub mod NHGJGMAEBCI;
    #[cfg(feature = "proto-rqa")]
    pub mod NKDNFDCFNCP;
    #[cfg(feature = "proto-rqa")]
    pub mod NLLLAAEJOBP;
    #[cfg(feature = "proto-rqa")]
    pub mod OCKCHBDFGNL;
    #[cfg(feature = "proto-rqa")]
    pub mod ONHKODAFEMH;
}

/// Game command header.
///
/// Contains the type of the command in `command_id`
/// and the data encoded in protobuf in `proto_data`
///
/// ## Bit Layout
/// | Bit indices     |  Type |  Name |
/// | - | - | - |
/// |   0..4      |  `u32`  |  Header (magic constant) |
/// |   0..6      |  `u16`  |  command_id |
/// |   6..8      |  `u16`  |  header_len (unsure) |
/// |   8..12     |  `u32`  |  data_len |
/// |  12..12+data_len |  variable  |  proto_data |
/// | data_len..data_len+4  |  `u32`  |  Tail (magic constant) |
#[derive(Clone)]
pub struct GameCommand {
    pub command_id: u16,
    #[allow(unused)]
    pub header_len: u16,
    #[allow(unused)]
    pub data_len: u32,
    #[allow(unused)]
    pub proto_header: Vec<u8>,
    pub proto_data: Vec<u8>,
}

impl GameCommand {
    const HEADER_LEN: usize = 12;
    const TAIL_LEN: usize = 4;

    #[instrument(skip(bytes), fields(len = bytes.len()))]
    pub fn try_new(bytes: Vec<u8>) -> Result<Self, GameCommandError> {
        let header_overhead = Self::HEADER_LEN + Self::TAIL_LEN;
        if bytes.len() < header_overhead {
            warn!(len = bytes.len(), "game command header incomplete");
            return Err(GameCommandError::HeaderTooShort {
                expected: header_overhead,
                actual: bytes.len(),
            });
        }

        // skip header magic const
        let command_id = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
        let header_len = u16::from_be_bytes(bytes[6..8].try_into().unwrap());
        let data_len = u32::from_be_bytes(bytes[8..12].try_into().unwrap());

        let data_start = 12 + header_len as usize;
        let data_end = data_start + data_len as usize;

        if data_end > bytes.len() {
            warn!(len = bytes.len(), "game command buffer too short");
            return Err(GameCommandError::CommandTooShort {
                expected: data_end,
                actual: bytes.len(),
            });
        }

        let proto_header = bytes[12..data_start].to_vec();
        let proto_data = bytes[data_start..data_end].to_vec();

        Ok(GameCommand {
            command_id,
            header_len,
            data_len,
            proto_header,
            proto_data,
        })
    }

    pub fn get_command_name(&self) -> Option<&str> {
        command_id::command_id_to_str(self.command_id)
    }

    pub fn parse_proto<T: protobuf::Message>(&self) -> protobuf::Result<T> {
        T::parse_from_bytes(&self.proto_data)
    }
}

impl fmt::Debug for GameCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameCommand")
            .field("command_id", &self.command_id)
            .field("command_name", &self.get_command_name())
            .field("header_len", &self.header_len)
            .field("data_len", &self.data_len)
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum GameCommandError {
    #[error("command header must be at least {expected} bytes, but was {actual}")]
    HeaderTooShort { expected: usize, actual: usize },
    #[error("command buffer must be at least {expected} bytes, but was {actual}")]
    CommandTooShort { expected: usize, actual: usize },
    #[error("decryption key is missing for command")]
    DecryptionKeyMissing,
    #[error("decrypted version does not match expected version")]
    VersionMismatch,
}
