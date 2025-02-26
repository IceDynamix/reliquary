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

//! Generated file from `CmdReplayType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdReplayType)
pub enum CmdReplayType {
    // @@protoc_insertion_point(enum_value:CmdReplayType.CmdReplayTypeNone)
    CmdReplayTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdReplayType.CmdGetReplayTokenCsReq)
    CmdGetReplayTokenCsReq = 3501,
    // @@protoc_insertion_point(enum_value:CmdReplayType.CmdGetPlayerReplayInfoCsReq)
    CmdGetPlayerReplayInfoCsReq = 3558,
    // @@protoc_insertion_point(enum_value:CmdReplayType.CmdGetPlayerReplayInfoScRsp)
    CmdGetPlayerReplayInfoScRsp = 3524,
    // @@protoc_insertion_point(enum_value:CmdReplayType.CmdGetReplayTokenScRsp)
    CmdGetReplayTokenScRsp = 3568,
}

impl ::protobuf::Enum for CmdReplayType {
    const NAME: &'static str = "CmdReplayType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdReplayType> {
        match value {
            0 => ::std::option::Option::Some(CmdReplayType::CmdReplayTypeNone),
            3501 => ::std::option::Option::Some(CmdReplayType::CmdGetReplayTokenCsReq),
            3558 => ::std::option::Option::Some(CmdReplayType::CmdGetPlayerReplayInfoCsReq),
            3524 => ::std::option::Option::Some(CmdReplayType::CmdGetPlayerReplayInfoScRsp),
            3568 => ::std::option::Option::Some(CmdReplayType::CmdGetReplayTokenScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdReplayType> {
        match str {
            "CmdReplayTypeNone" => ::std::option::Option::Some(CmdReplayType::CmdReplayTypeNone),
            "CmdGetReplayTokenCsReq" => ::std::option::Option::Some(CmdReplayType::CmdGetReplayTokenCsReq),
            "CmdGetPlayerReplayInfoCsReq" => ::std::option::Option::Some(CmdReplayType::CmdGetPlayerReplayInfoCsReq),
            "CmdGetPlayerReplayInfoScRsp" => ::std::option::Option::Some(CmdReplayType::CmdGetPlayerReplayInfoScRsp),
            "CmdGetReplayTokenScRsp" => ::std::option::Option::Some(CmdReplayType::CmdGetReplayTokenScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdReplayType] = &[
        CmdReplayType::CmdReplayTypeNone,
        CmdReplayType::CmdGetReplayTokenCsReq,
        CmdReplayType::CmdGetPlayerReplayInfoCsReq,
        CmdReplayType::CmdGetPlayerReplayInfoScRsp,
        CmdReplayType::CmdGetReplayTokenScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdReplayType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdReplayType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdReplayType::CmdReplayTypeNone => 0,
            CmdReplayType::CmdGetReplayTokenCsReq => 1,
            CmdReplayType::CmdGetPlayerReplayInfoCsReq => 2,
            CmdReplayType::CmdGetPlayerReplayInfoScRsp => 3,
            CmdReplayType::CmdGetReplayTokenScRsp => 4,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdReplayType {
    fn default() -> Self {
        CmdReplayType::CmdReplayTypeNone
    }
}

impl CmdReplayType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdReplayType>("CmdReplayType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdReplayType.proto*\xa4\x01\n\rCmdReplayType\x12\x15\n\x11CmdRepl\
    ayTypeNone\x10\0\x12\x1b\n\x16CmdGetReplayTokenCsReq\x10\xad\x1b\x12\x20\
    \n\x1bCmdGetPlayerReplayInfoCsReq\x10\xe6\x1b\x12\x20\n\x1bCmdGetPlayerR\
    eplayInfoScRsp\x10\xc4\x1b\x12\x1b\n\x16CmdGetReplayTokenScRsp\x10\xf0\
    \x1bb\x06proto3\
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
            enums.push(CmdReplayType::generated_enum_descriptor_data());
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
