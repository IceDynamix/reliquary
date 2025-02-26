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

//! Generated file from `CmdChatType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdChatType)
pub enum CmdChatType {
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdChatTypeNone)
    CmdChatTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdRevcMsgScNotify)
    CmdRevcMsgScNotify = 3958,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdBatchMarkChatEmojiScRsp)
    CmdBatchMarkChatEmojiScRsp = 3922,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetLoginChatInfoScRsp)
    CmdGetLoginChatInfoScRsp = 3980,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetLoginChatInfoCsReq)
    CmdGetLoginChatInfoCsReq = 3947,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetPrivateChatHistoryScRsp)
    CmdGetPrivateChatHistoryScRsp = 3956,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdPrivateMsgOfflineUsersScNotify)
    CmdPrivateMsgOfflineUsersScNotify = 3924,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetChatEmojiListScRsp)
    CmdGetChatEmojiListScRsp = 3911,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdSendMsgCsReq)
    CmdSendMsgCsReq = 3901,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetPrivateChatHistoryCsReq)
    CmdGetPrivateChatHistoryCsReq = 3930,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdBatchMarkChatEmojiCsReq)
    CmdBatchMarkChatEmojiCsReq = 3949,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdSendMsgScRsp)
    CmdSendMsgScRsp = 3968,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdMarkChatEmojiCsReq)
    CmdMarkChatEmojiCsReq = 3905,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetChatFriendHistoryCsReq)
    CmdGetChatFriendHistoryCsReq = 3997,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdMarkChatEmojiScRsp)
    CmdMarkChatEmojiScRsp = 3914,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetChatEmojiListCsReq)
    CmdGetChatEmojiListCsReq = 3928,
    // @@protoc_insertion_point(enum_value:CmdChatType.CmdGetChatFriendHistoryScRsp)
    CmdGetChatFriendHistoryScRsp = 3976,
}

impl ::protobuf::Enum for CmdChatType {
    const NAME: &'static str = "CmdChatType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdChatType> {
        match value {
            0 => ::std::option::Option::Some(CmdChatType::CmdChatTypeNone),
            3958 => ::std::option::Option::Some(CmdChatType::CmdRevcMsgScNotify),
            3922 => ::std::option::Option::Some(CmdChatType::CmdBatchMarkChatEmojiScRsp),
            3980 => ::std::option::Option::Some(CmdChatType::CmdGetLoginChatInfoScRsp),
            3947 => ::std::option::Option::Some(CmdChatType::CmdGetLoginChatInfoCsReq),
            3956 => ::std::option::Option::Some(CmdChatType::CmdGetPrivateChatHistoryScRsp),
            3924 => ::std::option::Option::Some(CmdChatType::CmdPrivateMsgOfflineUsersScNotify),
            3911 => ::std::option::Option::Some(CmdChatType::CmdGetChatEmojiListScRsp),
            3901 => ::std::option::Option::Some(CmdChatType::CmdSendMsgCsReq),
            3930 => ::std::option::Option::Some(CmdChatType::CmdGetPrivateChatHistoryCsReq),
            3949 => ::std::option::Option::Some(CmdChatType::CmdBatchMarkChatEmojiCsReq),
            3968 => ::std::option::Option::Some(CmdChatType::CmdSendMsgScRsp),
            3905 => ::std::option::Option::Some(CmdChatType::CmdMarkChatEmojiCsReq),
            3997 => ::std::option::Option::Some(CmdChatType::CmdGetChatFriendHistoryCsReq),
            3914 => ::std::option::Option::Some(CmdChatType::CmdMarkChatEmojiScRsp),
            3928 => ::std::option::Option::Some(CmdChatType::CmdGetChatEmojiListCsReq),
            3976 => ::std::option::Option::Some(CmdChatType::CmdGetChatFriendHistoryScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdChatType> {
        match str {
            "CmdChatTypeNone" => ::std::option::Option::Some(CmdChatType::CmdChatTypeNone),
            "CmdRevcMsgScNotify" => ::std::option::Option::Some(CmdChatType::CmdRevcMsgScNotify),
            "CmdBatchMarkChatEmojiScRsp" => ::std::option::Option::Some(CmdChatType::CmdBatchMarkChatEmojiScRsp),
            "CmdGetLoginChatInfoScRsp" => ::std::option::Option::Some(CmdChatType::CmdGetLoginChatInfoScRsp),
            "CmdGetLoginChatInfoCsReq" => ::std::option::Option::Some(CmdChatType::CmdGetLoginChatInfoCsReq),
            "CmdGetPrivateChatHistoryScRsp" => ::std::option::Option::Some(CmdChatType::CmdGetPrivateChatHistoryScRsp),
            "CmdPrivateMsgOfflineUsersScNotify" => ::std::option::Option::Some(CmdChatType::CmdPrivateMsgOfflineUsersScNotify),
            "CmdGetChatEmojiListScRsp" => ::std::option::Option::Some(CmdChatType::CmdGetChatEmojiListScRsp),
            "CmdSendMsgCsReq" => ::std::option::Option::Some(CmdChatType::CmdSendMsgCsReq),
            "CmdGetPrivateChatHistoryCsReq" => ::std::option::Option::Some(CmdChatType::CmdGetPrivateChatHistoryCsReq),
            "CmdBatchMarkChatEmojiCsReq" => ::std::option::Option::Some(CmdChatType::CmdBatchMarkChatEmojiCsReq),
            "CmdSendMsgScRsp" => ::std::option::Option::Some(CmdChatType::CmdSendMsgScRsp),
            "CmdMarkChatEmojiCsReq" => ::std::option::Option::Some(CmdChatType::CmdMarkChatEmojiCsReq),
            "CmdGetChatFriendHistoryCsReq" => ::std::option::Option::Some(CmdChatType::CmdGetChatFriendHistoryCsReq),
            "CmdMarkChatEmojiScRsp" => ::std::option::Option::Some(CmdChatType::CmdMarkChatEmojiScRsp),
            "CmdGetChatEmojiListCsReq" => ::std::option::Option::Some(CmdChatType::CmdGetChatEmojiListCsReq),
            "CmdGetChatFriendHistoryScRsp" => ::std::option::Option::Some(CmdChatType::CmdGetChatFriendHistoryScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdChatType] = &[
        CmdChatType::CmdChatTypeNone,
        CmdChatType::CmdRevcMsgScNotify,
        CmdChatType::CmdBatchMarkChatEmojiScRsp,
        CmdChatType::CmdGetLoginChatInfoScRsp,
        CmdChatType::CmdGetLoginChatInfoCsReq,
        CmdChatType::CmdGetPrivateChatHistoryScRsp,
        CmdChatType::CmdPrivateMsgOfflineUsersScNotify,
        CmdChatType::CmdGetChatEmojiListScRsp,
        CmdChatType::CmdSendMsgCsReq,
        CmdChatType::CmdGetPrivateChatHistoryCsReq,
        CmdChatType::CmdBatchMarkChatEmojiCsReq,
        CmdChatType::CmdSendMsgScRsp,
        CmdChatType::CmdMarkChatEmojiCsReq,
        CmdChatType::CmdGetChatFriendHistoryCsReq,
        CmdChatType::CmdMarkChatEmojiScRsp,
        CmdChatType::CmdGetChatEmojiListCsReq,
        CmdChatType::CmdGetChatFriendHistoryScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdChatType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdChatType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdChatType::CmdChatTypeNone => 0,
            CmdChatType::CmdRevcMsgScNotify => 1,
            CmdChatType::CmdBatchMarkChatEmojiScRsp => 2,
            CmdChatType::CmdGetLoginChatInfoScRsp => 3,
            CmdChatType::CmdGetLoginChatInfoCsReq => 4,
            CmdChatType::CmdGetPrivateChatHistoryScRsp => 5,
            CmdChatType::CmdPrivateMsgOfflineUsersScNotify => 6,
            CmdChatType::CmdGetChatEmojiListScRsp => 7,
            CmdChatType::CmdSendMsgCsReq => 8,
            CmdChatType::CmdGetPrivateChatHistoryCsReq => 9,
            CmdChatType::CmdBatchMarkChatEmojiCsReq => 10,
            CmdChatType::CmdSendMsgScRsp => 11,
            CmdChatType::CmdMarkChatEmojiCsReq => 12,
            CmdChatType::CmdGetChatFriendHistoryCsReq => 13,
            CmdChatType::CmdMarkChatEmojiScRsp => 14,
            CmdChatType::CmdGetChatEmojiListCsReq => 15,
            CmdChatType::CmdGetChatFriendHistoryScRsp => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdChatType {
    fn default() -> Self {
        CmdChatType::CmdChatTypeNone
    }
}

impl CmdChatType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdChatType>("CmdChatType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CmdChatType.proto*\x93\x04\n\x0bCmdChatType\x12\x13\n\x0fCmdChatTy\
    peNone\x10\0\x12\x17\n\x12CmdRevcMsgScNotify\x10\xf6\x1e\x12\x1f\n\x1aCm\
    dBatchMarkChatEmojiScRsp\x10\xd2\x1e\x12\x1d\n\x18CmdGetLoginChatInfoScR\
    sp\x10\x8c\x1f\x12\x1d\n\x18CmdGetLoginChatInfoCsReq\x10\xeb\x1e\x12\"\n\
    \x1dCmdGetPrivateChatHistoryScRsp\x10\xf4\x1e\x12&\n!CmdPrivateMsgOfflin\
    eUsersScNotify\x10\xd4\x1e\x12\x1d\n\x18CmdGetChatEmojiListScRsp\x10\xc7\
    \x1e\x12\x14\n\x0fCmdSendMsgCsReq\x10\xbd\x1e\x12\"\n\x1dCmdGetPrivateCh\
    atHistoryCsReq\x10\xda\x1e\x12\x1f\n\x1aCmdBatchMarkChatEmojiCsReq\x10\
    \xed\x1e\x12\x14\n\x0fCmdSendMsgScRsp\x10\x80\x1f\x12\x1a\n\x15CmdMarkCh\
    atEmojiCsReq\x10\xc1\x1e\x12!\n\x1cCmdGetChatFriendHistoryCsReq\x10\x9d\
    \x1f\x12\x1a\n\x15CmdMarkChatEmojiScRsp\x10\xca\x1e\x12\x1d\n\x18CmdGetC\
    hatEmojiListCsReq\x10\xd8\x1e\x12!\n\x1cCmdGetChatFriendHistoryScRsp\x10\
    \x88\x1fb\x06proto3\
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
            enums.push(CmdChatType::generated_enum_descriptor_data());
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
