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

//! Generated file from `CmdWolfBroType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdWolfBroType)
pub enum CmdWolfBroType {
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroTypeNone)
    CmdWolfBroTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdRestoreWolfBroGameArchiveScRsp)
    CmdRestoreWolfBroGameArchiveScRsp = 6553,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameUseBulletScRsp)
    CmdWolfBroGameUseBulletScRsp = 6530,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGamePickupBulletScRsp)
    CmdWolfBroGamePickupBulletScRsp = 6548,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameExplodeMonsterScRsp)
    CmdWolfBroGameExplodeMonsterScRsp = 6561,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdGetWolfBroGameDataScRsp)
    CmdGetWolfBroGameDataScRsp = 6516,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdArchiveWolfBroGameScRsp)
    CmdArchiveWolfBroGameScRsp = 6546,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdRestoreWolfBroGameArchiveCsReq)
    CmdRestoreWolfBroGameArchiveCsReq = 6539,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameActivateBulletCsReq)
    CmdWolfBroGameActivateBulletCsReq = 6590,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdStartWolfBroGameCsReq)
    CmdStartWolfBroGameCsReq = 6559,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdGetWolfBroGameDataCsReq)
    CmdGetWolfBroGameDataCsReq = 6580,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdQuitWolfBroGameScRsp)
    CmdQuitWolfBroGameScRsp = 6537,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameExplodeMonsterCsReq)
    CmdWolfBroGameExplodeMonsterCsReq = 6519,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameDataChangeScNotify)
    CmdWolfBroGameDataChangeScNotify = 6547,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameUseBulletCsReq)
    CmdWolfBroGameUseBulletCsReq = 6574,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGamePickupBulletCsReq)
    CmdWolfBroGamePickupBulletCsReq = 6575,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameActivateBulletScRsp)
    CmdWolfBroGameActivateBulletScRsp = 6579,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdStartWolfBroGameScRsp)
    CmdStartWolfBroGameScRsp = 6520,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdArchiveWolfBroGameCsReq)
    CmdArchiveWolfBroGameCsReq = 6503,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdQuitWolfBroGameCsReq)
    CmdQuitWolfBroGameCsReq = 6534,
}

impl ::protobuf::Enum for CmdWolfBroType {
    const NAME: &'static str = "CmdWolfBroType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdWolfBroType> {
        match value {
            0 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroTypeNone),
            6553 => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp),
            6530 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletScRsp),
            6548 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp),
            6561 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp),
            6516 => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataScRsp),
            6546 => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameScRsp),
            6539 => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq),
            6590 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq),
            6559 => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameCsReq),
            6580 => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataCsReq),
            6537 => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameScRsp),
            6519 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq),
            6547 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameDataChangeScNotify),
            6574 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletCsReq),
            6575 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq),
            6579 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp),
            6520 => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameScRsp),
            6503 => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameCsReq),
            6534 => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdWolfBroType> {
        match str {
            "CmdWolfBroTypeNone" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroTypeNone),
            "CmdRestoreWolfBroGameArchiveScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp),
            "CmdWolfBroGameUseBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletScRsp),
            "CmdWolfBroGamePickupBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp),
            "CmdWolfBroGameExplodeMonsterScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp),
            "CmdGetWolfBroGameDataScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataScRsp),
            "CmdArchiveWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameScRsp),
            "CmdRestoreWolfBroGameArchiveCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq),
            "CmdWolfBroGameActivateBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq),
            "CmdStartWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameCsReq),
            "CmdGetWolfBroGameDataCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataCsReq),
            "CmdQuitWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameScRsp),
            "CmdWolfBroGameExplodeMonsterCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq),
            "CmdWolfBroGameDataChangeScNotify" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameDataChangeScNotify),
            "CmdWolfBroGameUseBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletCsReq),
            "CmdWolfBroGamePickupBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq),
            "CmdWolfBroGameActivateBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp),
            "CmdStartWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameScRsp),
            "CmdArchiveWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameCsReq),
            "CmdQuitWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdWolfBroType] = &[
        CmdWolfBroType::CmdWolfBroTypeNone,
        CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp,
        CmdWolfBroType::CmdWolfBroGameUseBulletScRsp,
        CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp,
        CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp,
        CmdWolfBroType::CmdGetWolfBroGameDataScRsp,
        CmdWolfBroType::CmdArchiveWolfBroGameScRsp,
        CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq,
        CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq,
        CmdWolfBroType::CmdStartWolfBroGameCsReq,
        CmdWolfBroType::CmdGetWolfBroGameDataCsReq,
        CmdWolfBroType::CmdQuitWolfBroGameScRsp,
        CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq,
        CmdWolfBroType::CmdWolfBroGameDataChangeScNotify,
        CmdWolfBroType::CmdWolfBroGameUseBulletCsReq,
        CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq,
        CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp,
        CmdWolfBroType::CmdStartWolfBroGameScRsp,
        CmdWolfBroType::CmdArchiveWolfBroGameCsReq,
        CmdWolfBroType::CmdQuitWolfBroGameCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdWolfBroType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdWolfBroType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdWolfBroType::CmdWolfBroTypeNone => 0,
            CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp => 1,
            CmdWolfBroType::CmdWolfBroGameUseBulletScRsp => 2,
            CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp => 3,
            CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp => 4,
            CmdWolfBroType::CmdGetWolfBroGameDataScRsp => 5,
            CmdWolfBroType::CmdArchiveWolfBroGameScRsp => 6,
            CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq => 7,
            CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq => 8,
            CmdWolfBroType::CmdStartWolfBroGameCsReq => 9,
            CmdWolfBroType::CmdGetWolfBroGameDataCsReq => 10,
            CmdWolfBroType::CmdQuitWolfBroGameScRsp => 11,
            CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq => 12,
            CmdWolfBroType::CmdWolfBroGameDataChangeScNotify => 13,
            CmdWolfBroType::CmdWolfBroGameUseBulletCsReq => 14,
            CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq => 15,
            CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp => 16,
            CmdWolfBroType::CmdStartWolfBroGameScRsp => 17,
            CmdWolfBroType::CmdArchiveWolfBroGameCsReq => 18,
            CmdWolfBroType::CmdQuitWolfBroGameCsReq => 19,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdWolfBroType {
    fn default() -> Self {
        CmdWolfBroType::CmdWolfBroTypeNone
    }
}

impl CmdWolfBroType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdWolfBroType>("CmdWolfBroType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdWolfBroType.proto*\xcf\x05\n\x0eCmdWolfBroType\x12\x16\n\x12Cmd\
    WolfBroTypeNone\x10\0\x12&\n!CmdRestoreWolfBroGameArchiveScRsp\x10\x993\
    \x12!\n\x1cCmdWolfBroGameUseBulletScRsp\x10\x823\x12$\n\x1fCmdWolfBroGam\
    ePickupBulletScRsp\x10\x943\x12&\n!CmdWolfBroGameExplodeMonsterScRsp\x10\
    \xa13\x12\x1f\n\x1aCmdGetWolfBroGameDataScRsp\x10\xf42\x12\x1f\n\x1aCmdA\
    rchiveWolfBroGameScRsp\x10\x923\x12&\n!CmdRestoreWolfBroGameArchiveCsReq\
    \x10\x8b3\x12&\n!CmdWolfBroGameActivateBulletCsReq\x10\xbe3\x12\x1d\n\
    \x18CmdStartWolfBroGameCsReq\x10\x9f3\x12\x1f\n\x1aCmdGetWolfBroGameData\
    CsReq\x10\xb43\x12\x1c\n\x17CmdQuitWolfBroGameScRsp\x10\x893\x12&\n!CmdW\
    olfBroGameExplodeMonsterCsReq\x10\xf72\x12%\n\x20CmdWolfBroGameDataChang\
    eScNotify\x10\x933\x12!\n\x1cCmdWolfBroGameUseBulletCsReq\x10\xae3\x12$\
    \n\x1fCmdWolfBroGamePickupBulletCsReq\x10\xaf3\x12&\n!CmdWolfBroGameActi\
    vateBulletScRsp\x10\xb33\x12\x1d\n\x18CmdStartWolfBroGameScRsp\x10\xf82\
    \x12\x1f\n\x1aCmdArchiveWolfBroGameCsReq\x10\xe72\x12\x1c\n\x17CmdQuitWo\
    lfBroGameCsReq\x10\x863b\x06proto3\
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
            enums.push(CmdWolfBroType::generated_enum_descriptor_data());
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
