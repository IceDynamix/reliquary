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

//! Generated file from `CmdWolfBroType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdWolfBroType)
pub enum CmdWolfBroType {
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroTypeNone)
    CmdWolfBroTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdGetWolfBroGameDataCsReq)
    CmdGetWolfBroGameDataCsReq = 6543,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameExplodeMonsterScRsp)
    CmdWolfBroGameExplodeMonsterScRsp = 6526,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdArchiveWolfBroGameScRsp)
    CmdArchiveWolfBroGameScRsp = 6537,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdArchiveWolfBroGameCsReq)
    CmdArchiveWolfBroGameCsReq = 6502,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameActivateBulletCsReq)
    CmdWolfBroGameActivateBulletCsReq = 6544,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdGetWolfBroGameDataScRsp)
    CmdGetWolfBroGameDataScRsp = 6509,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdRestoreWolfBroGameArchiveCsReq)
    CmdRestoreWolfBroGameArchiveCsReq = 6536,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameActivateBulletScRsp)
    CmdWolfBroGameActivateBulletScRsp = 6532,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdQuitWolfBroGameScRsp)
    CmdQuitWolfBroGameScRsp = 6515,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameExplodeMonsterCsReq)
    CmdWolfBroGameExplodeMonsterCsReq = 6523,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameDataChangeScNotify)
    CmdWolfBroGameDataChangeScNotify = 6545,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameUseBulletCsReq)
    CmdWolfBroGameUseBulletCsReq = 6507,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdQuitWolfBroGameCsReq)
    CmdQuitWolfBroGameCsReq = 6527,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGamePickupBulletScRsp)
    CmdWolfBroGamePickupBulletScRsp = 6520,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGamePickupBulletCsReq)
    CmdWolfBroGamePickupBulletCsReq = 6540,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdRestoreWolfBroGameArchiveScRsp)
    CmdRestoreWolfBroGameArchiveScRsp = 6517,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdStartWolfBroGameScRsp)
    CmdStartWolfBroGameScRsp = 6546,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdStartWolfBroGameCsReq)
    CmdStartWolfBroGameCsReq = 6504,
    // @@protoc_insertion_point(enum_value:CmdWolfBroType.CmdWolfBroGameUseBulletScRsp)
    CmdWolfBroGameUseBulletScRsp = 6550,
}

impl ::protobuf::Enum for CmdWolfBroType {
    const NAME: &'static str = "CmdWolfBroType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdWolfBroType> {
        match value {
            0 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroTypeNone),
            6543 => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataCsReq),
            6526 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp),
            6537 => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameScRsp),
            6502 => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameCsReq),
            6544 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq),
            6509 => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataScRsp),
            6536 => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq),
            6532 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp),
            6515 => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameScRsp),
            6523 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq),
            6545 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameDataChangeScNotify),
            6507 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletCsReq),
            6527 => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameCsReq),
            6520 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp),
            6540 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq),
            6517 => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp),
            6546 => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameScRsp),
            6504 => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameCsReq),
            6550 => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdWolfBroType> {
        match str {
            "CmdWolfBroTypeNone" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroTypeNone),
            "CmdGetWolfBroGameDataCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataCsReq),
            "CmdWolfBroGameExplodeMonsterScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp),
            "CmdArchiveWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameScRsp),
            "CmdArchiveWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdArchiveWolfBroGameCsReq),
            "CmdWolfBroGameActivateBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq),
            "CmdGetWolfBroGameDataScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdGetWolfBroGameDataScRsp),
            "CmdRestoreWolfBroGameArchiveCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq),
            "CmdWolfBroGameActivateBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp),
            "CmdQuitWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameScRsp),
            "CmdWolfBroGameExplodeMonsterCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq),
            "CmdWolfBroGameDataChangeScNotify" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameDataChangeScNotify),
            "CmdWolfBroGameUseBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletCsReq),
            "CmdQuitWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdQuitWolfBroGameCsReq),
            "CmdWolfBroGamePickupBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp),
            "CmdWolfBroGamePickupBulletCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq),
            "CmdRestoreWolfBroGameArchiveScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp),
            "CmdStartWolfBroGameScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameScRsp),
            "CmdStartWolfBroGameCsReq" => ::std::option::Option::Some(CmdWolfBroType::CmdStartWolfBroGameCsReq),
            "CmdWolfBroGameUseBulletScRsp" => ::std::option::Option::Some(CmdWolfBroType::CmdWolfBroGameUseBulletScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdWolfBroType] = &[
        CmdWolfBroType::CmdWolfBroTypeNone,
        CmdWolfBroType::CmdGetWolfBroGameDataCsReq,
        CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp,
        CmdWolfBroType::CmdArchiveWolfBroGameScRsp,
        CmdWolfBroType::CmdArchiveWolfBroGameCsReq,
        CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq,
        CmdWolfBroType::CmdGetWolfBroGameDataScRsp,
        CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq,
        CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp,
        CmdWolfBroType::CmdQuitWolfBroGameScRsp,
        CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq,
        CmdWolfBroType::CmdWolfBroGameDataChangeScNotify,
        CmdWolfBroType::CmdWolfBroGameUseBulletCsReq,
        CmdWolfBroType::CmdQuitWolfBroGameCsReq,
        CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp,
        CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq,
        CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp,
        CmdWolfBroType::CmdStartWolfBroGameScRsp,
        CmdWolfBroType::CmdStartWolfBroGameCsReq,
        CmdWolfBroType::CmdWolfBroGameUseBulletScRsp,
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
            CmdWolfBroType::CmdGetWolfBroGameDataCsReq => 1,
            CmdWolfBroType::CmdWolfBroGameExplodeMonsterScRsp => 2,
            CmdWolfBroType::CmdArchiveWolfBroGameScRsp => 3,
            CmdWolfBroType::CmdArchiveWolfBroGameCsReq => 4,
            CmdWolfBroType::CmdWolfBroGameActivateBulletCsReq => 5,
            CmdWolfBroType::CmdGetWolfBroGameDataScRsp => 6,
            CmdWolfBroType::CmdRestoreWolfBroGameArchiveCsReq => 7,
            CmdWolfBroType::CmdWolfBroGameActivateBulletScRsp => 8,
            CmdWolfBroType::CmdQuitWolfBroGameScRsp => 9,
            CmdWolfBroType::CmdWolfBroGameExplodeMonsterCsReq => 10,
            CmdWolfBroType::CmdWolfBroGameDataChangeScNotify => 11,
            CmdWolfBroType::CmdWolfBroGameUseBulletCsReq => 12,
            CmdWolfBroType::CmdQuitWolfBroGameCsReq => 13,
            CmdWolfBroType::CmdWolfBroGamePickupBulletScRsp => 14,
            CmdWolfBroType::CmdWolfBroGamePickupBulletCsReq => 15,
            CmdWolfBroType::CmdRestoreWolfBroGameArchiveScRsp => 16,
            CmdWolfBroType::CmdStartWolfBroGameScRsp => 17,
            CmdWolfBroType::CmdStartWolfBroGameCsReq => 18,
            CmdWolfBroType::CmdWolfBroGameUseBulletScRsp => 19,
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
    WolfBroTypeNone\x10\0\x12\x1f\n\x1aCmdGetWolfBroGameDataCsReq\x10\x8f3\
    \x12&\n!CmdWolfBroGameExplodeMonsterScRsp\x10\xfe2\x12\x1f\n\x1aCmdArchi\
    veWolfBroGameScRsp\x10\x893\x12\x1f\n\x1aCmdArchiveWolfBroGameCsReq\x10\
    \xe62\x12&\n!CmdWolfBroGameActivateBulletCsReq\x10\x903\x12\x1f\n\x1aCmd\
    GetWolfBroGameDataScRsp\x10\xed2\x12&\n!CmdRestoreWolfBroGameArchiveCsRe\
    q\x10\x883\x12&\n!CmdWolfBroGameActivateBulletScRsp\x10\x843\x12\x1c\n\
    \x17CmdQuitWolfBroGameScRsp\x10\xf32\x12&\n!CmdWolfBroGameExplodeMonster\
    CsReq\x10\xfb2\x12%\n\x20CmdWolfBroGameDataChangeScNotify\x10\x913\x12!\
    \n\x1cCmdWolfBroGameUseBulletCsReq\x10\xeb2\x12\x1c\n\x17CmdQuitWolfBroG\
    ameCsReq\x10\xff2\x12$\n\x1fCmdWolfBroGamePickupBulletScRsp\x10\xf82\x12\
    $\n\x1fCmdWolfBroGamePickupBulletCsReq\x10\x8c3\x12&\n!CmdRestoreWolfBro\
    GameArchiveScRsp\x10\xf52\x12\x1d\n\x18CmdStartWolfBroGameScRsp\x10\x923\
    \x12\x1d\n\x18CmdStartWolfBroGameCsReq\x10\xe82\x12!\n\x1cCmdWolfBroGame\
    UseBulletScRsp\x10\x963b\x06proto3\
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
