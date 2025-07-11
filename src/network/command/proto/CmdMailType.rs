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

//! Generated file from `CmdMailType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMailType)
pub enum CmdMailType {
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMailTypeNone)
    CmdMailTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdGetMailScRsp)
    CmdGetMailScRsp = 891,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdDelMailScRsp)
    CmdDelMailScRsp = 821,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdDelMailCsReq)
    CmdDelMailCsReq = 827,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdTakeMailAttachmentScRsp)
    CmdTakeMailAttachmentScRsp = 859,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdGetMailCsReq)
    CmdGetMailCsReq = 820,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdNewMailScNotify)
    CmdNewMailScNotify = 854,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMarkReadMailScRsp)
    CmdMarkReadMailScRsp = 839,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdTakeMailAttachmentCsReq)
    CmdTakeMailAttachmentCsReq = 870,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMarkReadMailCsReq)
    CmdMarkReadMailCsReq = 867,
}

impl ::protobuf::Enum for CmdMailType {
    const NAME: &'static str = "CmdMailType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMailType> {
        match value {
            0 => ::std::option::Option::Some(CmdMailType::CmdMailTypeNone),
            891 => ::std::option::Option::Some(CmdMailType::CmdGetMailScRsp),
            821 => ::std::option::Option::Some(CmdMailType::CmdDelMailScRsp),
            827 => ::std::option::Option::Some(CmdMailType::CmdDelMailCsReq),
            859 => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentScRsp),
            820 => ::std::option::Option::Some(CmdMailType::CmdGetMailCsReq),
            854 => ::std::option::Option::Some(CmdMailType::CmdNewMailScNotify),
            839 => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailScRsp),
            870 => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentCsReq),
            867 => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMailType> {
        match str {
            "CmdMailTypeNone" => ::std::option::Option::Some(CmdMailType::CmdMailTypeNone),
            "CmdGetMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdGetMailScRsp),
            "CmdDelMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdDelMailScRsp),
            "CmdDelMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdDelMailCsReq),
            "CmdTakeMailAttachmentScRsp" => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentScRsp),
            "CmdGetMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdGetMailCsReq),
            "CmdNewMailScNotify" => ::std::option::Option::Some(CmdMailType::CmdNewMailScNotify),
            "CmdMarkReadMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailScRsp),
            "CmdTakeMailAttachmentCsReq" => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentCsReq),
            "CmdMarkReadMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMailType] = &[
        CmdMailType::CmdMailTypeNone,
        CmdMailType::CmdGetMailScRsp,
        CmdMailType::CmdDelMailScRsp,
        CmdMailType::CmdDelMailCsReq,
        CmdMailType::CmdTakeMailAttachmentScRsp,
        CmdMailType::CmdGetMailCsReq,
        CmdMailType::CmdNewMailScNotify,
        CmdMailType::CmdMarkReadMailScRsp,
        CmdMailType::CmdTakeMailAttachmentCsReq,
        CmdMailType::CmdMarkReadMailCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdMailType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMailType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMailType::CmdMailTypeNone => 0,
            CmdMailType::CmdGetMailScRsp => 1,
            CmdMailType::CmdDelMailScRsp => 2,
            CmdMailType::CmdDelMailCsReq => 3,
            CmdMailType::CmdTakeMailAttachmentScRsp => 4,
            CmdMailType::CmdGetMailCsReq => 5,
            CmdMailType::CmdNewMailScNotify => 6,
            CmdMailType::CmdMarkReadMailScRsp => 7,
            CmdMailType::CmdTakeMailAttachmentCsReq => 8,
            CmdMailType::CmdMarkReadMailCsReq => 9,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMailType {
    fn default() -> Self {
        CmdMailType::CmdMailTypeNone
    }
}

impl CmdMailType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMailType>("CmdMailType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CmdMailType.proto*\x8b\x02\n\x0bCmdMailType\x12\x13\n\x0fCmdMailTy\
    peNone\x10\0\x12\x14\n\x0fCmdGetMailScRsp\x10\xfb\x06\x12\x14\n\x0fCmdDe\
    lMailScRsp\x10\xb5\x06\x12\x14\n\x0fCmdDelMailCsReq\x10\xbb\x06\x12\x1f\
    \n\x1aCmdTakeMailAttachmentScRsp\x10\xdb\x06\x12\x14\n\x0fCmdGetMailCsRe\
    q\x10\xb4\x06\x12\x17\n\x12CmdNewMailScNotify\x10\xd6\x06\x12\x19\n\x14C\
    mdMarkReadMailScRsp\x10\xc7\x06\x12\x1f\n\x1aCmdTakeMailAttachmentCsReq\
    \x10\xe6\x06\x12\x19\n\x14CmdMarkReadMailCsReq\x10\xe3\x06b\x06proto3\
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
            enums.push(CmdMailType::generated_enum_descriptor_data());
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
