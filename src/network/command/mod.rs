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

#[cfg(all(not(feature = "proto-limited"), not(feature = "proto-rqa")))]
pub mod proto;

#[cfg(all(feature = "proto-limited"))]
pub mod proto {
    pub mod BlackInfo;
    pub mod PlayerGetTokenScRsp;
}

#[cfg(all(feature = "proto-rqa"))]
pub mod proto {
    pub mod Avatar;
    pub mod AvatarSkillTree;
    pub mod AvatarSync;
    pub mod BLPMHFGIOAC;
    pub mod BasicInfo;
    pub mod BlackInfo;
    pub mod BoardDataSync;
    pub mod DMBMPAHKHLA;
    pub mod DoGachaScRsp;
    pub mod EquipRelic;
    pub mod Equipment;
    pub mod FHABEIKAFBO;
    pub mod GEMJDHNLKLC;
    pub mod GKDIHIFFHFD;
    pub mod GNIEJGNKKGG;
    pub mod GachaItem;
    pub mod GetAvatarDataScRsp;
    pub mod GetBagScRsp;
    pub mod GetMultiPathAvatarInfoScRsp;
    pub mod GrowthTargetFunctionType;
    pub mod HPNIICAAAJK;
    pub mod HeadIcon;
    pub mod IKAMMKLBOCO;
    pub mod Item;
    pub mod ItemList;
    pub mod LOPCJEOJHCB;
    pub mod LPFMHAJHDMM;
    pub mod Material;
    pub mod MessageGroupStatus;
    pub mod MessageSectionStatus;
    pub mod Mission;
    pub mod MissionStatus;
    pub mod MissionSync;
    pub mod MissionSyncType;
    pub mod MultiPathAvatarType;
    pub mod MultiPathAvatarTypeInfo;
    pub mod PileItem;
    pub mod PlayerGetTokenScRsp;
    pub mod PlayerLoginScRsp;
    pub mod PlayerSyncScNotify;
    pub mod Quest;
    pub mod QuestStatus;
    pub mod RecycleMaterial;
    pub mod Relic;
    pub mod RelicAffix;
    pub mod TurnFoodSwitch;
    pub mod WaitDelResource;
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

        let data = bytes[12..12 + data_len as usize + header_len as usize].to_vec();
        Ok(GameCommand {
            command_id,
            header_len,
            data_len,
            proto_data: data,
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
    #[error("decryption key is missing for command")]
    DecryptionKeyMissing,
}
