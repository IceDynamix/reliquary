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

//! Generated file from `CmdMailType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMailType)
pub enum CmdMailType {
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMailTypeNone)
    CmdMailTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdGetMailScRsp)
    CmdGetMailScRsp = 895,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdTakeMailAttachmentCsReq)
    CmdTakeMailAttachmentCsReq = 852,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdDelMailScRsp)
    CmdDelMailScRsp = 828,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdTakeMailAttachmentScRsp)
    CmdTakeMailAttachmentScRsp = 874,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMarkReadMailScRsp)
    CmdMarkReadMailScRsp = 827,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdNewMailScNotify)
    CmdNewMailScNotify = 824,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdGetMailCsReq)
    CmdGetMailCsReq = 836,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdDelMailCsReq)
    CmdDelMailCsReq = 867,
    // @@protoc_insertion_point(enum_value:CmdMailType.CmdMarkReadMailCsReq)
    CmdMarkReadMailCsReq = 884,
}

impl ::protobuf::Enum for CmdMailType {
    const NAME: &'static str = "CmdMailType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMailType> {
        match value {
            0 => ::std::option::Option::Some(CmdMailType::CmdMailTypeNone),
            895 => ::std::option::Option::Some(CmdMailType::CmdGetMailScRsp),
            852 => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentCsReq),
            828 => ::std::option::Option::Some(CmdMailType::CmdDelMailScRsp),
            874 => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentScRsp),
            827 => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailScRsp),
            824 => ::std::option::Option::Some(CmdMailType::CmdNewMailScNotify),
            836 => ::std::option::Option::Some(CmdMailType::CmdGetMailCsReq),
            867 => ::std::option::Option::Some(CmdMailType::CmdDelMailCsReq),
            884 => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMailType> {
        match str {
            "CmdMailTypeNone" => ::std::option::Option::Some(CmdMailType::CmdMailTypeNone),
            "CmdGetMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdGetMailScRsp),
            "CmdTakeMailAttachmentCsReq" => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentCsReq),
            "CmdDelMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdDelMailScRsp),
            "CmdTakeMailAttachmentScRsp" => ::std::option::Option::Some(CmdMailType::CmdTakeMailAttachmentScRsp),
            "CmdMarkReadMailScRsp" => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailScRsp),
            "CmdNewMailScNotify" => ::std::option::Option::Some(CmdMailType::CmdNewMailScNotify),
            "CmdGetMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdGetMailCsReq),
            "CmdDelMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdDelMailCsReq),
            "CmdMarkReadMailCsReq" => ::std::option::Option::Some(CmdMailType::CmdMarkReadMailCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMailType] = &[
        CmdMailType::CmdMailTypeNone,
        CmdMailType::CmdGetMailScRsp,
        CmdMailType::CmdTakeMailAttachmentCsReq,
        CmdMailType::CmdDelMailScRsp,
        CmdMailType::CmdTakeMailAttachmentScRsp,
        CmdMailType::CmdMarkReadMailScRsp,
        CmdMailType::CmdNewMailScNotify,
        CmdMailType::CmdGetMailCsReq,
        CmdMailType::CmdDelMailCsReq,
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
            CmdMailType::CmdTakeMailAttachmentCsReq => 2,
            CmdMailType::CmdDelMailScRsp => 3,
            CmdMailType::CmdTakeMailAttachmentScRsp => 4,
            CmdMailType::CmdMarkReadMailScRsp => 5,
            CmdMailType::CmdNewMailScNotify => 6,
            CmdMailType::CmdGetMailCsReq => 7,
            CmdMailType::CmdDelMailCsReq => 8,
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
    peNone\x10\0\x12\x14\n\x0fCmdGetMailScRsp\x10\xff\x06\x12\x1f\n\x1aCmdTa\
    keMailAttachmentCsReq\x10\xd4\x06\x12\x14\n\x0fCmdDelMailScRsp\x10\xbc\
    \x06\x12\x1f\n\x1aCmdTakeMailAttachmentScRsp\x10\xea\x06\x12\x19\n\x14Cm\
    dMarkReadMailScRsp\x10\xbb\x06\x12\x17\n\x12CmdNewMailScNotify\x10\xb8\
    \x06\x12\x14\n\x0fCmdGetMailCsReq\x10\xc4\x06\x12\x14\n\x0fCmdDelMailCsR\
    eq\x10\xe3\x06\x12\x19\n\x14CmdMarkReadMailCsReq\x10\xf4\x06b\x06proto3\
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
