// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `LobbyModifyType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LobbyModifyType)
pub enum LobbyModifyType {
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_None)
    LobbyModifyType_None = 0,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_Idle)
    LobbyModifyType_Idle = 1,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_Ready)
    LobbyModifyType_Ready = 2,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_Operating)
    LobbyModifyType_Operating = 3,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_CancelMatch)
    LobbyModifyType_CancelMatch = 4,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_Match)
    LobbyModifyType_Match = 5,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_QuitLobby)
    LobbyModifyType_QuitLobby = 6,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_KickOut)
    LobbyModifyType_KickOut = 7,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_TimeOut)
    LobbyModifyType_TimeOut = 8,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_JoinLobby)
    LobbyModifyType_JoinLobby = 9,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_LobbyDismiss)
    LobbyModifyType_LobbyDismiss = 10,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_MatchTimeOut)
    LobbyModifyType_MatchTimeOut = 11,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_FightStart)
    LobbyModifyType_FightStart = 12,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_Logout)
    LobbyModifyType_Logout = 13,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_FightEnd)
    LobbyModifyType_FightEnd = 14,
    // @@protoc_insertion_point(enum_value:LobbyModifyType.LobbyModifyType_FightRoomDestroyInInit)
    LobbyModifyType_FightRoomDestroyInInit = 15,
}

impl ::protobuf::Enum for LobbyModifyType {
    const NAME: &'static str = "LobbyModifyType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LobbyModifyType> {
        match value {
            0 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_None),
            1 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Idle),
            2 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Ready),
            3 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Operating),
            4 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_CancelMatch),
            5 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Match),
            6 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_QuitLobby),
            7 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_KickOut),
            8 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_TimeOut),
            9 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_JoinLobby),
            10 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_LobbyDismiss),
            11 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_MatchTimeOut),
            12 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightStart),
            13 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Logout),
            14 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightEnd),
            15 => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightRoomDestroyInInit),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LobbyModifyType> {
        match str {
            "LobbyModifyType_None" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_None),
            "LobbyModifyType_Idle" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Idle),
            "LobbyModifyType_Ready" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Ready),
            "LobbyModifyType_Operating" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Operating),
            "LobbyModifyType_CancelMatch" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_CancelMatch),
            "LobbyModifyType_Match" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Match),
            "LobbyModifyType_QuitLobby" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_QuitLobby),
            "LobbyModifyType_KickOut" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_KickOut),
            "LobbyModifyType_TimeOut" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_TimeOut),
            "LobbyModifyType_JoinLobby" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_JoinLobby),
            "LobbyModifyType_LobbyDismiss" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_LobbyDismiss),
            "LobbyModifyType_MatchTimeOut" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_MatchTimeOut),
            "LobbyModifyType_FightStart" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightStart),
            "LobbyModifyType_Logout" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_Logout),
            "LobbyModifyType_FightEnd" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightEnd),
            "LobbyModifyType_FightRoomDestroyInInit" => ::std::option::Option::Some(LobbyModifyType::LobbyModifyType_FightRoomDestroyInInit),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LobbyModifyType] = &[
        LobbyModifyType::LobbyModifyType_None,
        LobbyModifyType::LobbyModifyType_Idle,
        LobbyModifyType::LobbyModifyType_Ready,
        LobbyModifyType::LobbyModifyType_Operating,
        LobbyModifyType::LobbyModifyType_CancelMatch,
        LobbyModifyType::LobbyModifyType_Match,
        LobbyModifyType::LobbyModifyType_QuitLobby,
        LobbyModifyType::LobbyModifyType_KickOut,
        LobbyModifyType::LobbyModifyType_TimeOut,
        LobbyModifyType::LobbyModifyType_JoinLobby,
        LobbyModifyType::LobbyModifyType_LobbyDismiss,
        LobbyModifyType::LobbyModifyType_MatchTimeOut,
        LobbyModifyType::LobbyModifyType_FightStart,
        LobbyModifyType::LobbyModifyType_Logout,
        LobbyModifyType::LobbyModifyType_FightEnd,
        LobbyModifyType::LobbyModifyType_FightRoomDestroyInInit,
    ];
}

impl ::protobuf::EnumFull for LobbyModifyType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LobbyModifyType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LobbyModifyType {
    fn default() -> Self {
        LobbyModifyType::LobbyModifyType_None
    }
}

impl LobbyModifyType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LobbyModifyType>("LobbyModifyType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15LobbyModifyType.proto*\xfd\x03\n\x0fLobbyModifyType\x12\x18\n\x14L\
    obbyModifyType_None\x10\0\x12\x18\n\x14LobbyModifyType_Idle\x10\x01\x12\
    \x19\n\x15LobbyModifyType_Ready\x10\x02\x12\x1d\n\x19LobbyModifyType_Ope\
    rating\x10\x03\x12\x1f\n\x1bLobbyModifyType_CancelMatch\x10\x04\x12\x19\
    \n\x15LobbyModifyType_Match\x10\x05\x12\x1d\n\x19LobbyModifyType_QuitLob\
    by\x10\x06\x12\x1b\n\x17LobbyModifyType_KickOut\x10\x07\x12\x1b\n\x17Lob\
    byModifyType_TimeOut\x10\x08\x12\x1d\n\x19LobbyModifyType_JoinLobby\x10\
    \t\x12\x20\n\x1cLobbyModifyType_LobbyDismiss\x10\n\x12\x20\n\x1cLobbyMod\
    ifyType_MatchTimeOut\x10\x0b\x12\x1e\n\x1aLobbyModifyType_FightStart\x10\
    \x0c\x12\x1a\n\x16LobbyModifyType_Logout\x10\r\x12\x1c\n\x18LobbyModifyT\
    ype_FightEnd\x10\x0e\x12*\n&LobbyModifyType_FightRoomDestroyInInit\x10\
    \x0fb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(LobbyModifyType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
