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

//! Generated file from `CmdMarbleType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMarbleType)
pub enum CmdMarbleType {
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleTypeNone)
    CmdMarbleTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleUpdateShownSealCsReq)
    CmdMarbleUpdateShownSealCsReq = 8277,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleGetDataScRsp)
    CmdMarbleGetDataScRsp = 8283,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleLevelFinishCsReq)
    CmdMarbleLevelFinishCsReq = 8278,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleUpdateShownSealScRsp)
    CmdMarbleUpdateShownSealScRsp = 8273,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleLevelFinishScRsp)
    CmdMarbleLevelFinishScRsp = 8280,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarblePvpDataUpdateScNotify)
    CmdMarblePvpDataUpdateScNotify = 8279,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleUnlockSealScNotify)
    CmdMarbleUnlockSealScNotify = 8290,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleShopBuyCsReq)
    CmdMarbleShopBuyCsReq = 8274,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleShopBuyScRsp)
    CmdMarbleShopBuyScRsp = 8272,
    // @@protoc_insertion_point(enum_value:CmdMarbleType.CmdMarbleGetDataCsReq)
    CmdMarbleGetDataCsReq = 8284,
}

impl ::protobuf::Enum for CmdMarbleType {
    const NAME: &'static str = "CmdMarbleType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMarbleType> {
        match value {
            0 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleTypeNone),
            8277 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUpdateShownSealCsReq),
            8283 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleGetDataScRsp),
            8278 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleLevelFinishCsReq),
            8273 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUpdateShownSealScRsp),
            8280 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleLevelFinishScRsp),
            8279 => ::std::option::Option::Some(CmdMarbleType::CmdMarblePvpDataUpdateScNotify),
            8290 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUnlockSealScNotify),
            8274 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleShopBuyCsReq),
            8272 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleShopBuyScRsp),
            8284 => ::std::option::Option::Some(CmdMarbleType::CmdMarbleGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMarbleType> {
        match str {
            "CmdMarbleTypeNone" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleTypeNone),
            "CmdMarbleUpdateShownSealCsReq" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUpdateShownSealCsReq),
            "CmdMarbleGetDataScRsp" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleGetDataScRsp),
            "CmdMarbleLevelFinishCsReq" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleLevelFinishCsReq),
            "CmdMarbleUpdateShownSealScRsp" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUpdateShownSealScRsp),
            "CmdMarbleLevelFinishScRsp" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleLevelFinishScRsp),
            "CmdMarblePvpDataUpdateScNotify" => ::std::option::Option::Some(CmdMarbleType::CmdMarblePvpDataUpdateScNotify),
            "CmdMarbleUnlockSealScNotify" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleUnlockSealScNotify),
            "CmdMarbleShopBuyCsReq" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleShopBuyCsReq),
            "CmdMarbleShopBuyScRsp" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleShopBuyScRsp),
            "CmdMarbleGetDataCsReq" => ::std::option::Option::Some(CmdMarbleType::CmdMarbleGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMarbleType] = &[
        CmdMarbleType::CmdMarbleTypeNone,
        CmdMarbleType::CmdMarbleUpdateShownSealCsReq,
        CmdMarbleType::CmdMarbleGetDataScRsp,
        CmdMarbleType::CmdMarbleLevelFinishCsReq,
        CmdMarbleType::CmdMarbleUpdateShownSealScRsp,
        CmdMarbleType::CmdMarbleLevelFinishScRsp,
        CmdMarbleType::CmdMarblePvpDataUpdateScNotify,
        CmdMarbleType::CmdMarbleUnlockSealScNotify,
        CmdMarbleType::CmdMarbleShopBuyCsReq,
        CmdMarbleType::CmdMarbleShopBuyScRsp,
        CmdMarbleType::CmdMarbleGetDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdMarbleType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMarbleType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMarbleType::CmdMarbleTypeNone => 0,
            CmdMarbleType::CmdMarbleUpdateShownSealCsReq => 1,
            CmdMarbleType::CmdMarbleGetDataScRsp => 2,
            CmdMarbleType::CmdMarbleLevelFinishCsReq => 3,
            CmdMarbleType::CmdMarbleUpdateShownSealScRsp => 4,
            CmdMarbleType::CmdMarbleLevelFinishScRsp => 5,
            CmdMarbleType::CmdMarblePvpDataUpdateScNotify => 6,
            CmdMarbleType::CmdMarbleUnlockSealScNotify => 7,
            CmdMarbleType::CmdMarbleShopBuyCsReq => 8,
            CmdMarbleType::CmdMarbleShopBuyScRsp => 9,
            CmdMarbleType::CmdMarbleGetDataCsReq => 10,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMarbleType {
    fn default() -> Self {
        CmdMarbleType::CmdMarbleTypeNone
    }
}

impl CmdMarbleType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMarbleType>("CmdMarbleType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdMarbleType.proto*\xe5\x02\n\rCmdMarbleType\x12\x15\n\x11CmdMarb\
    leTypeNone\x10\0\x12\"\n\x1dCmdMarbleUpdateShownSealCsReq\x10\xd5@\x12\
    \x1a\n\x15CmdMarbleGetDataScRsp\x10\xdb@\x12\x1e\n\x19CmdMarbleLevelFini\
    shCsReq\x10\xd6@\x12\"\n\x1dCmdMarbleUpdateShownSealScRsp\x10\xd1@\x12\
    \x1e\n\x19CmdMarbleLevelFinishScRsp\x10\xd8@\x12#\n\x1eCmdMarblePvpDataU\
    pdateScNotify\x10\xd7@\x12\x20\n\x1bCmdMarbleUnlockSealScNotify\x10\xe2@\
    \x12\x1a\n\x15CmdMarbleShopBuyCsReq\x10\xd2@\x12\x1a\n\x15CmdMarbleShopB\
    uyScRsp\x10\xd0@\x12\x1a\n\x15CmdMarbleGetDataCsReq\x10\xdc@b\x06proto3\
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
            enums.push(CmdMarbleType::generated_enum_descriptor_data());
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
