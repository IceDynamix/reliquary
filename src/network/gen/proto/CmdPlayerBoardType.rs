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

//! Generated file from `CmdPlayerBoardType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdPlayerBoardType)
pub enum CmdPlayerBoardType {
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdPlayerBoardTypeNone)
    CmdPlayerBoardTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdGetPlayerBoardDataCsReq)
    CmdGetPlayerBoardDataCsReq = 2898,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetSignatureCsReq)
    CmdSetSignatureCsReq = 2838,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdGetPlayerBoardDataScRsp)
    CmdGetPlayerBoardDataScRsp = 2871,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetIsDisplayAvatarInfoScRsp)
    CmdSetIsDisplayAvatarInfoScRsp = 2812,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetDisplayAvatarCsReq)
    CmdSetDisplayAvatarCsReq = 2879,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetHeadIconCsReq)
    CmdSetHeadIconCsReq = 2883,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetSignatureScRsp)
    CmdSetSignatureScRsp = 2878,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetAssistAvatarScRsp)
    CmdSetAssistAvatarScRsp = 2889,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetIsDisplayAvatarInfoCsReq)
    CmdSetIsDisplayAvatarInfoCsReq = 2833,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetDisplayAvatarScRsp)
    CmdSetDisplayAvatarScRsp = 2877,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdUnlockHeadIconScNotify)
    CmdUnlockHeadIconScNotify = 2828,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetHeadIconScRsp)
    CmdSetHeadIconScRsp = 2842,
    // @@protoc_insertion_point(enum_value:CmdPlayerBoardType.CmdSetAssistAvatarCsReq)
    CmdSetAssistAvatarCsReq = 2856,
}

impl ::protobuf::Enum for CmdPlayerBoardType {
    const NAME: &'static str = "CmdPlayerBoardType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdPlayerBoardType> {
        match value {
            0 => ::std::option::Option::Some(CmdPlayerBoardType::CmdPlayerBoardTypeNone),
            2898 => ::std::option::Option::Some(CmdPlayerBoardType::CmdGetPlayerBoardDataCsReq),
            2838 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetSignatureCsReq),
            2871 => ::std::option::Option::Some(CmdPlayerBoardType::CmdGetPlayerBoardDataScRsp),
            2812 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoScRsp),
            2879 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetDisplayAvatarCsReq),
            2883 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetHeadIconCsReq),
            2878 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetSignatureScRsp),
            2889 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetAssistAvatarScRsp),
            2833 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoCsReq),
            2877 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetDisplayAvatarScRsp),
            2828 => ::std::option::Option::Some(CmdPlayerBoardType::CmdUnlockHeadIconScNotify),
            2842 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetHeadIconScRsp),
            2856 => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetAssistAvatarCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdPlayerBoardType> {
        match str {
            "CmdPlayerBoardTypeNone" => ::std::option::Option::Some(CmdPlayerBoardType::CmdPlayerBoardTypeNone),
            "CmdGetPlayerBoardDataCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdGetPlayerBoardDataCsReq),
            "CmdSetSignatureCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetSignatureCsReq),
            "CmdGetPlayerBoardDataScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdGetPlayerBoardDataScRsp),
            "CmdSetIsDisplayAvatarInfoScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoScRsp),
            "CmdSetDisplayAvatarCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetDisplayAvatarCsReq),
            "CmdSetHeadIconCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetHeadIconCsReq),
            "CmdSetSignatureScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetSignatureScRsp),
            "CmdSetAssistAvatarScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetAssistAvatarScRsp),
            "CmdSetIsDisplayAvatarInfoCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoCsReq),
            "CmdSetDisplayAvatarScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetDisplayAvatarScRsp),
            "CmdUnlockHeadIconScNotify" => ::std::option::Option::Some(CmdPlayerBoardType::CmdUnlockHeadIconScNotify),
            "CmdSetHeadIconScRsp" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetHeadIconScRsp),
            "CmdSetAssistAvatarCsReq" => ::std::option::Option::Some(CmdPlayerBoardType::CmdSetAssistAvatarCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdPlayerBoardType] = &[
        CmdPlayerBoardType::CmdPlayerBoardTypeNone,
        CmdPlayerBoardType::CmdGetPlayerBoardDataCsReq,
        CmdPlayerBoardType::CmdSetSignatureCsReq,
        CmdPlayerBoardType::CmdGetPlayerBoardDataScRsp,
        CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoScRsp,
        CmdPlayerBoardType::CmdSetDisplayAvatarCsReq,
        CmdPlayerBoardType::CmdSetHeadIconCsReq,
        CmdPlayerBoardType::CmdSetSignatureScRsp,
        CmdPlayerBoardType::CmdSetAssistAvatarScRsp,
        CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoCsReq,
        CmdPlayerBoardType::CmdSetDisplayAvatarScRsp,
        CmdPlayerBoardType::CmdUnlockHeadIconScNotify,
        CmdPlayerBoardType::CmdSetHeadIconScRsp,
        CmdPlayerBoardType::CmdSetAssistAvatarCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdPlayerBoardType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdPlayerBoardType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdPlayerBoardType::CmdPlayerBoardTypeNone => 0,
            CmdPlayerBoardType::CmdGetPlayerBoardDataCsReq => 1,
            CmdPlayerBoardType::CmdSetSignatureCsReq => 2,
            CmdPlayerBoardType::CmdGetPlayerBoardDataScRsp => 3,
            CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoScRsp => 4,
            CmdPlayerBoardType::CmdSetDisplayAvatarCsReq => 5,
            CmdPlayerBoardType::CmdSetHeadIconCsReq => 6,
            CmdPlayerBoardType::CmdSetSignatureScRsp => 7,
            CmdPlayerBoardType::CmdSetAssistAvatarScRsp => 8,
            CmdPlayerBoardType::CmdSetIsDisplayAvatarInfoCsReq => 9,
            CmdPlayerBoardType::CmdSetDisplayAvatarScRsp => 10,
            CmdPlayerBoardType::CmdUnlockHeadIconScNotify => 11,
            CmdPlayerBoardType::CmdSetHeadIconScRsp => 12,
            CmdPlayerBoardType::CmdSetAssistAvatarCsReq => 13,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdPlayerBoardType {
    fn default() -> Self {
        CmdPlayerBoardType::CmdPlayerBoardTypeNone
    }
}

impl CmdPlayerBoardType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdPlayerBoardType>("CmdPlayerBoardType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdPlayerBoardType.proto*\xc0\x03\n\x12CmdPlayerBoardType\x12\x1a\
    \n\x16CmdPlayerBoardTypeNone\x10\0\x12\x1f\n\x1aCmdGetPlayerBoardDataCsR\
    eq\x10\xd2\x16\x12\x19\n\x14CmdSetSignatureCsReq\x10\x96\x16\x12\x1f\n\
    \x1aCmdGetPlayerBoardDataScRsp\x10\xb7\x16\x12#\n\x1eCmdSetIsDisplayAvat\
    arInfoScRsp\x10\xfc\x15\x12\x1d\n\x18CmdSetDisplayAvatarCsReq\x10\xbf\
    \x16\x12\x18\n\x13CmdSetHeadIconCsReq\x10\xc3\x16\x12\x19\n\x14CmdSetSig\
    natureScRsp\x10\xbe\x16\x12\x1c\n\x17CmdSetAssistAvatarScRsp\x10\xc9\x16\
    \x12#\n\x1eCmdSetIsDisplayAvatarInfoCsReq\x10\x91\x16\x12\x1d\n\x18CmdSe\
    tDisplayAvatarScRsp\x10\xbd\x16\x12\x1e\n\x19CmdUnlockHeadIconScNotify\
    \x10\x8c\x16\x12\x18\n\x13CmdSetHeadIconScRsp\x10\x9a\x16\x12\x1c\n\x17C\
    mdSetAssistAvatarCsReq\x10\xa8\x16b\x06proto3\
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
            enums.push(CmdPlayerBoardType::generated_enum_descriptor_data());
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
