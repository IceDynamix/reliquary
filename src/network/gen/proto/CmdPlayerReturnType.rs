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

//! Generated file from `CmdPlayerReturnType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdPlayerReturnType)
pub enum CmdPlayerReturnType {
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTypeNone)
    CmdPlayerReturnTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnSignScRsp)
    CmdPlayerReturnSignScRsp = 4558,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnForceFinishScNotify)
    CmdPlayerReturnForceFinishScNotify = 4505,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnSignCsReq)
    CmdPlayerReturnSignCsReq = 4568,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRewardCsReq)
    CmdPlayerReturnTakeRewardCsReq = 4597,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnPointChangeScNotify)
    CmdPlayerReturnPointChangeScNotify = 4524,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnStartScNotify)
    CmdPlayerReturnStartScNotify = 4501,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRewardScRsp)
    CmdPlayerReturnTakeRewardScRsp = 4576,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakePointRewardCsReq)
    CmdPlayerReturnTakePointRewardCsReq = 4530,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnInfoQueryScRsp)
    CmdPlayerReturnInfoQueryScRsp = 4511,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnInfoQueryCsReq)
    CmdPlayerReturnInfoQueryCsReq = 4528,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakePointRewardScRsp)
    CmdPlayerReturnTakePointRewardScRsp = 4556,
}

impl ::protobuf::Enum for CmdPlayerReturnType {
    const NAME: &'static str = "CmdPlayerReturnType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdPlayerReturnType> {
        match value {
            0 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTypeNone),
            4558 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignScRsp),
            4505 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify),
            4568 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignCsReq),
            4597 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq),
            4524 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify),
            4501 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnStartScNotify),
            4576 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp),
            4530 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq),
            4511 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp),
            4528 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq),
            4556 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdPlayerReturnType> {
        match str {
            "CmdPlayerReturnTypeNone" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTypeNone),
            "CmdPlayerReturnSignScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignScRsp),
            "CmdPlayerReturnForceFinishScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify),
            "CmdPlayerReturnSignCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignCsReq),
            "CmdPlayerReturnTakeRewardCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq),
            "CmdPlayerReturnPointChangeScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify),
            "CmdPlayerReturnStartScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnStartScNotify),
            "CmdPlayerReturnTakeRewardScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp),
            "CmdPlayerReturnTakePointRewardCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq),
            "CmdPlayerReturnInfoQueryScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp),
            "CmdPlayerReturnInfoQueryCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq),
            "CmdPlayerReturnTakePointRewardScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdPlayerReturnType] = &[
        CmdPlayerReturnType::CmdPlayerReturnTypeNone,
        CmdPlayerReturnType::CmdPlayerReturnSignScRsp,
        CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify,
        CmdPlayerReturnType::CmdPlayerReturnSignCsReq,
        CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq,
        CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify,
        CmdPlayerReturnType::CmdPlayerReturnStartScNotify,
        CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp,
        CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq,
        CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp,
        CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq,
        CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdPlayerReturnType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdPlayerReturnType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdPlayerReturnType::CmdPlayerReturnTypeNone => 0,
            CmdPlayerReturnType::CmdPlayerReturnSignScRsp => 1,
            CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify => 2,
            CmdPlayerReturnType::CmdPlayerReturnSignCsReq => 3,
            CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq => 4,
            CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify => 5,
            CmdPlayerReturnType::CmdPlayerReturnStartScNotify => 6,
            CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp => 7,
            CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq => 8,
            CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp => 9,
            CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq => 10,
            CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp => 11,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdPlayerReturnType {
    fn default() -> Self {
        CmdPlayerReturnType::CmdPlayerReturnTypeNone
    }
}

impl CmdPlayerReturnType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdPlayerReturnType>("CmdPlayerReturnType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdPlayerReturnType.proto*\xcb\x03\n\x13CmdPlayerReturnType\x12\
    \x1b\n\x17CmdPlayerReturnTypeNone\x10\0\x12\x1d\n\x18CmdPlayerReturnSign\
    ScRsp\x10\xce#\x12'\n\"CmdPlayerReturnForceFinishScNotify\x10\x99#\x12\
    \x1d\n\x18CmdPlayerReturnSignCsReq\x10\xd8#\x12#\n\x1eCmdPlayerReturnTak\
    eRewardCsReq\x10\xf5#\x12'\n\"CmdPlayerReturnPointChangeScNotify\x10\xac\
    #\x12!\n\x1cCmdPlayerReturnStartScNotify\x10\x95#\x12#\n\x1eCmdPlayerRet\
    urnTakeRewardScRsp\x10\xe0#\x12(\n#CmdPlayerReturnTakePointRewardCsReq\
    \x10\xb2#\x12\"\n\x1dCmdPlayerReturnInfoQueryScRsp\x10\x9f#\x12\"\n\x1dC\
    mdPlayerReturnInfoQueryCsReq\x10\xb0#\x12(\n#CmdPlayerReturnTakePointRew\
    ardScRsp\x10\xcc#b\x06proto3\
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
            enums.push(CmdPlayerReturnType::generated_enum_descriptor_data());
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
