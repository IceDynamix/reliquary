//! Contains generated code to parse the payload of [`crate::network::GameCommand`]s
//!
//! For code generation, refer to [reliquary-codegen](https://github.com/IceDynamix/reliquary-codegen)
//!
//! [reliquary::network::GamePacket]s can be parsed into the corresponding protobuf struct like this
//! ```no_run
//! use protobuf::Message;
//! use reliquary::network::GameCommand;
//! use reliquary::network::gen::proto::GetBagScRsp::GetBagScRsp;
//!
//! let command: GameCommand = /* ... */;
//!
//! let parsed = command.parse_proto::<GetBagScRsp>().unwrap();
//! // alternatively
//! let parsed2 = GetBagScRsp::parse_from_bytes(&command.proto_data).unwrap();
//! ```

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod proto;
pub mod command_id;
