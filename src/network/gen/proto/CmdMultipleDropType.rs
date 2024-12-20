// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `CmdMultipleDropType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMultipleDropType)
pub enum CmdMultipleDropType {
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdMultipleDropTypeNone)
    CmdMultipleDropTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdGetPlayerReturnMultiDropInfoScRsp)
    CmdGetPlayerReturnMultiDropInfoScRsp = 4639,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdGetPlayerReturnMultiDropInfoCsReq)
    CmdGetPlayerReturnMultiDropInfoCsReq = 4646,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdGetMultipleDropInfoCsReq)
    CmdGetMultipleDropInfoCsReq = 4659,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdMultipleDropInfoScNotify)
    CmdMultipleDropInfoScNotify = 4603,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdMultipleDropInfoNotify)
    CmdMultipleDropInfoNotify = 4653,
    // @@protoc_insertion_point(enum_value:CmdMultipleDropType.CmdGetMultipleDropInfoScRsp)
    CmdGetMultipleDropInfoScRsp = 4620,
}

impl ::protobuf::Enum for CmdMultipleDropType {
    const NAME: &'static str = "CmdMultipleDropType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMultipleDropType> {
        match value {
            0 => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropTypeNone),
            4639 => ::std::option::Option::Some(CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoScRsp),
            4646 => ::std::option::Option::Some(CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoCsReq),
            4659 => ::std::option::Option::Some(CmdMultipleDropType::CmdGetMultipleDropInfoCsReq),
            4603 => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropInfoScNotify),
            4653 => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropInfoNotify),
            4620 => ::std::option::Option::Some(CmdMultipleDropType::CmdGetMultipleDropInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMultipleDropType> {
        match str {
            "CmdMultipleDropTypeNone" => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropTypeNone),
            "CmdGetPlayerReturnMultiDropInfoScRsp" => ::std::option::Option::Some(CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoScRsp),
            "CmdGetPlayerReturnMultiDropInfoCsReq" => ::std::option::Option::Some(CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoCsReq),
            "CmdGetMultipleDropInfoCsReq" => ::std::option::Option::Some(CmdMultipleDropType::CmdGetMultipleDropInfoCsReq),
            "CmdMultipleDropInfoScNotify" => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropInfoScNotify),
            "CmdMultipleDropInfoNotify" => ::std::option::Option::Some(CmdMultipleDropType::CmdMultipleDropInfoNotify),
            "CmdGetMultipleDropInfoScRsp" => ::std::option::Option::Some(CmdMultipleDropType::CmdGetMultipleDropInfoScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMultipleDropType] = &[
        CmdMultipleDropType::CmdMultipleDropTypeNone,
        CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoScRsp,
        CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoCsReq,
        CmdMultipleDropType::CmdGetMultipleDropInfoCsReq,
        CmdMultipleDropType::CmdMultipleDropInfoScNotify,
        CmdMultipleDropType::CmdMultipleDropInfoNotify,
        CmdMultipleDropType::CmdGetMultipleDropInfoScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMultipleDropType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMultipleDropType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMultipleDropType::CmdMultipleDropTypeNone => 0,
            CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoScRsp => 1,
            CmdMultipleDropType::CmdGetPlayerReturnMultiDropInfoCsReq => 2,
            CmdMultipleDropType::CmdGetMultipleDropInfoCsReq => 3,
            CmdMultipleDropType::CmdMultipleDropInfoScNotify => 4,
            CmdMultipleDropType::CmdMultipleDropInfoNotify => 5,
            CmdMultipleDropType::CmdGetMultipleDropInfoScRsp => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMultipleDropType {
    fn default() -> Self {
        CmdMultipleDropType::CmdMultipleDropTypeNone
    }
}

impl CmdMultipleDropType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMultipleDropType>("CmdMultipleDropType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdMultipleDropType.proto*\x8e\x02\n\x13CmdMultipleDropType\x12\
    \x1b\n\x17CmdMultipleDropTypeNone\x10\0\x12)\n$CmdGetPlayerReturnMultiDr\
    opInfoScRsp\x10\x9f$\x12)\n$CmdGetPlayerReturnMultiDropInfoCsReq\x10\xa6\
    $\x12\x20\n\x1bCmdGetMultipleDropInfoCsReq\x10\xb3$\x12\x20\n\x1bCmdMult\
    ipleDropInfoScNotify\x10\xfb#\x12\x1e\n\x19CmdMultipleDropInfoNotify\x10\
    \xad$\x12\x20\n\x1bCmdGetMultipleDropInfoScRsp\x10\x8c$b\x06proto3\
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
            enums.push(CmdMultipleDropType::generated_enum_descriptor_data());
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
