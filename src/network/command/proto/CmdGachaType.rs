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

//! Generated file from `CmdGachaType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdGachaType)
pub enum CmdGachaType {
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGachaTypeNone)
    CmdGachaTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdDoGachaCsReq)
    CmdDoGachaCsReq = 1931,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaCeilingScRsp)
    CmdGetGachaCeilingScRsp = 1948,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaInfoScRsp)
    CmdGetGachaInfoScRsp = 1932,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGachaDecideItemChangeScNotify)
    CmdGachaDecideItemChangeScNotify = 1972,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdDoGachaScRsp)
    CmdDoGachaScRsp = 1940,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdExchangeGachaCeilingCsReq)
    CmdExchangeGachaCeilingCsReq = 1971,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaInfoCsReq)
    CmdGetGachaInfoCsReq = 1995,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdExchangeGachaCeilingScRsp)
    CmdExchangeGachaCeilingScRsp = 1952,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdSetGachaDecideItemCsReq)
    CmdSetGachaDecideItemCsReq = 1922,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdGetGachaCeilingCsReq)
    CmdGetGachaCeilingCsReq = 1976,
    // @@protoc_insertion_point(enum_value:CmdGachaType.CmdSetGachaDecideItemScRsp)
    CmdSetGachaDecideItemScRsp = 1956,
}

impl ::protobuf::Enum for CmdGachaType {
    const NAME: &'static str = "CmdGachaType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdGachaType> {
        match value {
            0 => ::std::option::Option::Some(CmdGachaType::CmdGachaTypeNone),
            1931 => ::std::option::Option::Some(CmdGachaType::CmdDoGachaCsReq),
            1948 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingScRsp),
            1932 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoScRsp),
            1972 => ::std::option::Option::Some(CmdGachaType::CmdGachaDecideItemChangeScNotify),
            1940 => ::std::option::Option::Some(CmdGachaType::CmdDoGachaScRsp),
            1971 => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingCsReq),
            1995 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoCsReq),
            1952 => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingScRsp),
            1922 => ::std::option::Option::Some(CmdGachaType::CmdSetGachaDecideItemCsReq),
            1976 => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingCsReq),
            1956 => ::std::option::Option::Some(CmdGachaType::CmdSetGachaDecideItemScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdGachaType> {
        match str {
            "CmdGachaTypeNone" => ::std::option::Option::Some(CmdGachaType::CmdGachaTypeNone),
            "CmdDoGachaCsReq" => ::std::option::Option::Some(CmdGachaType::CmdDoGachaCsReq),
            "CmdGetGachaCeilingScRsp" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingScRsp),
            "CmdGetGachaInfoScRsp" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoScRsp),
            "CmdGachaDecideItemChangeScNotify" => ::std::option::Option::Some(CmdGachaType::CmdGachaDecideItemChangeScNotify),
            "CmdDoGachaScRsp" => ::std::option::Option::Some(CmdGachaType::CmdDoGachaScRsp),
            "CmdExchangeGachaCeilingCsReq" => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingCsReq),
            "CmdGetGachaInfoCsReq" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaInfoCsReq),
            "CmdExchangeGachaCeilingScRsp" => ::std::option::Option::Some(CmdGachaType::CmdExchangeGachaCeilingScRsp),
            "CmdSetGachaDecideItemCsReq" => ::std::option::Option::Some(CmdGachaType::CmdSetGachaDecideItemCsReq),
            "CmdGetGachaCeilingCsReq" => ::std::option::Option::Some(CmdGachaType::CmdGetGachaCeilingCsReq),
            "CmdSetGachaDecideItemScRsp" => ::std::option::Option::Some(CmdGachaType::CmdSetGachaDecideItemScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdGachaType] = &[
        CmdGachaType::CmdGachaTypeNone,
        CmdGachaType::CmdDoGachaCsReq,
        CmdGachaType::CmdGetGachaCeilingScRsp,
        CmdGachaType::CmdGetGachaInfoScRsp,
        CmdGachaType::CmdGachaDecideItemChangeScNotify,
        CmdGachaType::CmdDoGachaScRsp,
        CmdGachaType::CmdExchangeGachaCeilingCsReq,
        CmdGachaType::CmdGetGachaInfoCsReq,
        CmdGachaType::CmdExchangeGachaCeilingScRsp,
        CmdGachaType::CmdSetGachaDecideItemCsReq,
        CmdGachaType::CmdGetGachaCeilingCsReq,
        CmdGachaType::CmdSetGachaDecideItemScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdGachaType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdGachaType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdGachaType::CmdGachaTypeNone => 0,
            CmdGachaType::CmdDoGachaCsReq => 1,
            CmdGachaType::CmdGetGachaCeilingScRsp => 2,
            CmdGachaType::CmdGetGachaInfoScRsp => 3,
            CmdGachaType::CmdGachaDecideItemChangeScNotify => 4,
            CmdGachaType::CmdDoGachaScRsp => 5,
            CmdGachaType::CmdExchangeGachaCeilingCsReq => 6,
            CmdGachaType::CmdGetGachaInfoCsReq => 7,
            CmdGachaType::CmdExchangeGachaCeilingScRsp => 8,
            CmdGachaType::CmdSetGachaDecideItemCsReq => 9,
            CmdGachaType::CmdGetGachaCeilingCsReq => 10,
            CmdGachaType::CmdSetGachaDecideItemScRsp => 11,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdGachaType {
    fn default() -> Self {
        CmdGachaType::CmdGachaTypeNone
    }
}

impl CmdGachaType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdGachaType>("CmdGachaType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdGachaType.proto*\xf1\x02\n\x0cCmdGachaType\x12\x14\n\x10CmdGach\
    aTypeNone\x10\0\x12\x14\n\x0fCmdDoGachaCsReq\x10\x8b\x0f\x12\x1c\n\x17Cm\
    dGetGachaCeilingScRsp\x10\x9c\x0f\x12\x19\n\x14CmdGetGachaInfoScRsp\x10\
    \x8c\x0f\x12%\n\x20CmdGachaDecideItemChangeScNotify\x10\xb4\x0f\x12\x14\
    \n\x0fCmdDoGachaScRsp\x10\x94\x0f\x12!\n\x1cCmdExchangeGachaCeilingCsReq\
    \x10\xb3\x0f\x12\x19\n\x14CmdGetGachaInfoCsReq\x10\xcb\x0f\x12!\n\x1cCmd\
    ExchangeGachaCeilingScRsp\x10\xa0\x0f\x12\x1f\n\x1aCmdSetGachaDecideItem\
    CsReq\x10\x82\x0f\x12\x1c\n\x17CmdGetGachaCeilingCsReq\x10\xb8\x0f\x12\
    \x1f\n\x1aCmdSetGachaDecideItemScRsp\x10\xa4\x0fb\x06proto3\
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
            enums.push(CmdGachaType::generated_enum_descriptor_data());
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
