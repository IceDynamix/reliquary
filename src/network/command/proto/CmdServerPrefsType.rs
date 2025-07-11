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

//! Generated file from `CmdServerPrefsType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdServerPrefsType)
pub enum CmdServerPrefsType {
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdServerPrefsTypeNone)
    CmdServerPrefsTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdGetServerPrefsDataCsReq)
    CmdGetServerPrefsDataCsReq = 6167,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdGetAllServerPrefsDataScRsp)
    CmdGetAllServerPrefsDataScRsp = 6191,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdUpdateServerPrefsDataCsReq)
    CmdUpdateServerPrefsDataCsReq = 6127,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdGetAllServerPrefsDataCsReq)
    CmdGetAllServerPrefsDataCsReq = 6120,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdUpdateServerPrefsDataScRsp)
    CmdUpdateServerPrefsDataScRsp = 6121,
    // @@protoc_insertion_point(enum_value:CmdServerPrefsType.CmdGetServerPrefsDataScRsp)
    CmdGetServerPrefsDataScRsp = 6139,
}

impl ::protobuf::Enum for CmdServerPrefsType {
    const NAME: &'static str = "CmdServerPrefsType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdServerPrefsType> {
        match value {
            0 => ::std::option::Option::Some(CmdServerPrefsType::CmdServerPrefsTypeNone),
            6167 => ::std::option::Option::Some(CmdServerPrefsType::CmdGetServerPrefsDataCsReq),
            6191 => ::std::option::Option::Some(CmdServerPrefsType::CmdGetAllServerPrefsDataScRsp),
            6127 => ::std::option::Option::Some(CmdServerPrefsType::CmdUpdateServerPrefsDataCsReq),
            6120 => ::std::option::Option::Some(CmdServerPrefsType::CmdGetAllServerPrefsDataCsReq),
            6121 => ::std::option::Option::Some(CmdServerPrefsType::CmdUpdateServerPrefsDataScRsp),
            6139 => ::std::option::Option::Some(CmdServerPrefsType::CmdGetServerPrefsDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdServerPrefsType> {
        match str {
            "CmdServerPrefsTypeNone" => ::std::option::Option::Some(CmdServerPrefsType::CmdServerPrefsTypeNone),
            "CmdGetServerPrefsDataCsReq" => ::std::option::Option::Some(CmdServerPrefsType::CmdGetServerPrefsDataCsReq),
            "CmdGetAllServerPrefsDataScRsp" => ::std::option::Option::Some(CmdServerPrefsType::CmdGetAllServerPrefsDataScRsp),
            "CmdUpdateServerPrefsDataCsReq" => ::std::option::Option::Some(CmdServerPrefsType::CmdUpdateServerPrefsDataCsReq),
            "CmdGetAllServerPrefsDataCsReq" => ::std::option::Option::Some(CmdServerPrefsType::CmdGetAllServerPrefsDataCsReq),
            "CmdUpdateServerPrefsDataScRsp" => ::std::option::Option::Some(CmdServerPrefsType::CmdUpdateServerPrefsDataScRsp),
            "CmdGetServerPrefsDataScRsp" => ::std::option::Option::Some(CmdServerPrefsType::CmdGetServerPrefsDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdServerPrefsType] = &[
        CmdServerPrefsType::CmdServerPrefsTypeNone,
        CmdServerPrefsType::CmdGetServerPrefsDataCsReq,
        CmdServerPrefsType::CmdGetAllServerPrefsDataScRsp,
        CmdServerPrefsType::CmdUpdateServerPrefsDataCsReq,
        CmdServerPrefsType::CmdGetAllServerPrefsDataCsReq,
        CmdServerPrefsType::CmdUpdateServerPrefsDataScRsp,
        CmdServerPrefsType::CmdGetServerPrefsDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdServerPrefsType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdServerPrefsType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdServerPrefsType::CmdServerPrefsTypeNone => 0,
            CmdServerPrefsType::CmdGetServerPrefsDataCsReq => 1,
            CmdServerPrefsType::CmdGetAllServerPrefsDataScRsp => 2,
            CmdServerPrefsType::CmdUpdateServerPrefsDataCsReq => 3,
            CmdServerPrefsType::CmdGetAllServerPrefsDataCsReq => 4,
            CmdServerPrefsType::CmdUpdateServerPrefsDataScRsp => 5,
            CmdServerPrefsType::CmdGetServerPrefsDataScRsp => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdServerPrefsType {
    fn default() -> Self {
        CmdServerPrefsType::CmdServerPrefsTypeNone
    }
}

impl CmdServerPrefsType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdServerPrefsType>("CmdServerPrefsType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdServerPrefsType.proto*\x82\x02\n\x12CmdServerPrefsType\x12\x1a\
    \n\x16CmdServerPrefsTypeNone\x10\0\x12\x1f\n\x1aCmdGetServerPrefsDataCsR\
    eq\x10\x970\x12\"\n\x1dCmdGetAllServerPrefsDataScRsp\x10\xaf0\x12\"\n\
    \x1dCmdUpdateServerPrefsDataCsReq\x10\xef/\x12\"\n\x1dCmdGetAllServerPre\
    fsDataCsReq\x10\xe8/\x12\"\n\x1dCmdUpdateServerPrefsDataScRsp\x10\xe9/\
    \x12\x1f\n\x1aCmdGetServerPrefsDataScRsp\x10\xfb/b\x06proto3\
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
            enums.push(CmdServerPrefsType::generated_enum_descriptor_data());
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
