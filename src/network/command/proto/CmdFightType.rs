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

//! Generated file from `CmdFightType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdFightType)
pub enum CmdFightType {
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightTypeNone)
    CmdFightTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightKickOutScNotify)
    CmdFightKickOutScNotify = 30039,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightHeartBeatCsReq)
    CmdFightHeartBeatCsReq = 30027,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightEnterScRsp)
    CmdFightEnterScRsp = 30091,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightEnterCsReq)
    CmdFightEnterCsReq = 30020,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightSessionStopScNotify)
    CmdFightSessionStopScNotify = 30070,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightGeneralScRsp)
    CmdFightGeneralScRsp = 30054,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightGeneralCsReq)
    CmdFightGeneralCsReq = 30059,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightGeneralScNotify)
    CmdFightGeneralScNotify = 30077,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightLeaveScNotify)
    CmdFightLeaveScNotify = 30067,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightHeartBeatScRsp)
    CmdFightHeartBeatScRsp = 30021,
}

impl ::protobuf::Enum for CmdFightType {
    const NAME: &'static str = "CmdFightType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdFightType> {
        match value {
            0 => ::std::option::Option::Some(CmdFightType::CmdFightTypeNone),
            30039 => ::std::option::Option::Some(CmdFightType::CmdFightKickOutScNotify),
            30027 => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatCsReq),
            30091 => ::std::option::Option::Some(CmdFightType::CmdFightEnterScRsp),
            30020 => ::std::option::Option::Some(CmdFightType::CmdFightEnterCsReq),
            30070 => ::std::option::Option::Some(CmdFightType::CmdFightSessionStopScNotify),
            30054 => ::std::option::Option::Some(CmdFightType::CmdFightGeneralScRsp),
            30059 => ::std::option::Option::Some(CmdFightType::CmdFightGeneralCsReq),
            30077 => ::std::option::Option::Some(CmdFightType::CmdFightGeneralScNotify),
            30067 => ::std::option::Option::Some(CmdFightType::CmdFightLeaveScNotify),
            30021 => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdFightType> {
        match str {
            "CmdFightTypeNone" => ::std::option::Option::Some(CmdFightType::CmdFightTypeNone),
            "CmdFightKickOutScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightKickOutScNotify),
            "CmdFightHeartBeatCsReq" => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatCsReq),
            "CmdFightEnterScRsp" => ::std::option::Option::Some(CmdFightType::CmdFightEnterScRsp),
            "CmdFightEnterCsReq" => ::std::option::Option::Some(CmdFightType::CmdFightEnterCsReq),
            "CmdFightSessionStopScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightSessionStopScNotify),
            "CmdFightGeneralScRsp" => ::std::option::Option::Some(CmdFightType::CmdFightGeneralScRsp),
            "CmdFightGeneralCsReq" => ::std::option::Option::Some(CmdFightType::CmdFightGeneralCsReq),
            "CmdFightGeneralScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightGeneralScNotify),
            "CmdFightLeaveScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightLeaveScNotify),
            "CmdFightHeartBeatScRsp" => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdFightType] = &[
        CmdFightType::CmdFightTypeNone,
        CmdFightType::CmdFightKickOutScNotify,
        CmdFightType::CmdFightHeartBeatCsReq,
        CmdFightType::CmdFightEnterScRsp,
        CmdFightType::CmdFightEnterCsReq,
        CmdFightType::CmdFightSessionStopScNotify,
        CmdFightType::CmdFightGeneralScRsp,
        CmdFightType::CmdFightGeneralCsReq,
        CmdFightType::CmdFightGeneralScNotify,
        CmdFightType::CmdFightLeaveScNotify,
        CmdFightType::CmdFightHeartBeatScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdFightType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdFightType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdFightType::CmdFightTypeNone => 0,
            CmdFightType::CmdFightKickOutScNotify => 1,
            CmdFightType::CmdFightHeartBeatCsReq => 2,
            CmdFightType::CmdFightEnterScRsp => 3,
            CmdFightType::CmdFightEnterCsReq => 4,
            CmdFightType::CmdFightSessionStopScNotify => 5,
            CmdFightType::CmdFightGeneralScRsp => 6,
            CmdFightType::CmdFightGeneralCsReq => 7,
            CmdFightType::CmdFightGeneralScNotify => 8,
            CmdFightType::CmdFightLeaveScNotify => 9,
            CmdFightType::CmdFightHeartBeatScRsp => 10,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdFightType {
    fn default() -> Self {
        CmdFightType::CmdFightTypeNone
    }
}

impl CmdFightType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdFightType>("CmdFightType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdFightType.proto*\xca\x02\n\x0cCmdFightType\x12\x14\n\x10CmdFigh\
    tTypeNone\x10\0\x12\x1d\n\x17CmdFightKickOutScNotify\x10\xd7\xea\x01\x12\
    \x1c\n\x16CmdFightHeartBeatCsReq\x10\xcb\xea\x01\x12\x18\n\x12CmdFightEn\
    terScRsp\x10\x8b\xeb\x01\x12\x18\n\x12CmdFightEnterCsReq\x10\xc4\xea\x01\
    \x12!\n\x1bCmdFightSessionStopScNotify\x10\xf6\xea\x01\x12\x1a\n\x14CmdF\
    ightGeneralScRsp\x10\xe6\xea\x01\x12\x1a\n\x14CmdFightGeneralCsReq\x10\
    \xeb\xea\x01\x12\x1d\n\x17CmdFightGeneralScNotify\x10\xfd\xea\x01\x12\
    \x1b\n\x15CmdFightLeaveScNotify\x10\xf3\xea\x01\x12\x1c\n\x16CmdFightHea\
    rtBeatScRsp\x10\xc5\xea\x01b\x06proto3\
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
            enums.push(CmdFightType::generated_enum_descriptor_data());
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
