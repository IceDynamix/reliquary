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

//! Generated file from `CmdTarotBookType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTarotBookType)
pub enum CmdTarotBookType {
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookTypeNone)
    CmdTarotBookTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookFinishInteractionScRsp)
    CmdTarotBookFinishInteractionScRsp = 8158,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookGetDataCsReq)
    CmdTarotBookGetDataCsReq = 8157,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookUnlockStoryCsReq)
    CmdTarotBookUnlockStoryCsReq = 8144,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookFinishStoryScRsp)
    CmdTarotBookFinishStoryScRsp = 8154,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookUnlockStoryScRsp)
    CmdTarotBookUnlockStoryScRsp = 8141,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookGetDataScRsp)
    CmdTarotBookGetDataScRsp = 8148,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookOpenPackCsReq)
    CmdTarotBookOpenPackCsReq = 8152,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookFinishInteractionCsReq)
    CmdTarotBookFinishInteractionCsReq = 8146,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookModifyEnergyScNotify)
    CmdTarotBookModifyEnergyScNotify = 8155,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookFinishStoryCsReq)
    CmdTarotBookFinishStoryCsReq = 8159,
    // @@protoc_insertion_point(enum_value:CmdTarotBookType.CmdTarotBookOpenPackScRsp)
    CmdTarotBookOpenPackScRsp = 8142,
}

impl ::protobuf::Enum for CmdTarotBookType {
    const NAME: &'static str = "CmdTarotBookType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTarotBookType> {
        match value {
            0 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookTypeNone),
            8158 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishInteractionScRsp),
            8157 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookGetDataCsReq),
            8144 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookUnlockStoryCsReq),
            8154 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishStoryScRsp),
            8141 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookUnlockStoryScRsp),
            8148 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookGetDataScRsp),
            8152 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookOpenPackCsReq),
            8146 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishInteractionCsReq),
            8155 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookModifyEnergyScNotify),
            8159 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishStoryCsReq),
            8142 => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookOpenPackScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTarotBookType> {
        match str {
            "CmdTarotBookTypeNone" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookTypeNone),
            "CmdTarotBookFinishInteractionScRsp" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishInteractionScRsp),
            "CmdTarotBookGetDataCsReq" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookGetDataCsReq),
            "CmdTarotBookUnlockStoryCsReq" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookUnlockStoryCsReq),
            "CmdTarotBookFinishStoryScRsp" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishStoryScRsp),
            "CmdTarotBookUnlockStoryScRsp" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookUnlockStoryScRsp),
            "CmdTarotBookGetDataScRsp" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookGetDataScRsp),
            "CmdTarotBookOpenPackCsReq" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookOpenPackCsReq),
            "CmdTarotBookFinishInteractionCsReq" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishInteractionCsReq),
            "CmdTarotBookModifyEnergyScNotify" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookModifyEnergyScNotify),
            "CmdTarotBookFinishStoryCsReq" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookFinishStoryCsReq),
            "CmdTarotBookOpenPackScRsp" => ::std::option::Option::Some(CmdTarotBookType::CmdTarotBookOpenPackScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTarotBookType] = &[
        CmdTarotBookType::CmdTarotBookTypeNone,
        CmdTarotBookType::CmdTarotBookFinishInteractionScRsp,
        CmdTarotBookType::CmdTarotBookGetDataCsReq,
        CmdTarotBookType::CmdTarotBookUnlockStoryCsReq,
        CmdTarotBookType::CmdTarotBookFinishStoryScRsp,
        CmdTarotBookType::CmdTarotBookUnlockStoryScRsp,
        CmdTarotBookType::CmdTarotBookGetDataScRsp,
        CmdTarotBookType::CmdTarotBookOpenPackCsReq,
        CmdTarotBookType::CmdTarotBookFinishInteractionCsReq,
        CmdTarotBookType::CmdTarotBookModifyEnergyScNotify,
        CmdTarotBookType::CmdTarotBookFinishStoryCsReq,
        CmdTarotBookType::CmdTarotBookOpenPackScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTarotBookType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTarotBookType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTarotBookType::CmdTarotBookTypeNone => 0,
            CmdTarotBookType::CmdTarotBookFinishInteractionScRsp => 1,
            CmdTarotBookType::CmdTarotBookGetDataCsReq => 2,
            CmdTarotBookType::CmdTarotBookUnlockStoryCsReq => 3,
            CmdTarotBookType::CmdTarotBookFinishStoryScRsp => 4,
            CmdTarotBookType::CmdTarotBookUnlockStoryScRsp => 5,
            CmdTarotBookType::CmdTarotBookGetDataScRsp => 6,
            CmdTarotBookType::CmdTarotBookOpenPackCsReq => 7,
            CmdTarotBookType::CmdTarotBookFinishInteractionCsReq => 8,
            CmdTarotBookType::CmdTarotBookModifyEnergyScNotify => 9,
            CmdTarotBookType::CmdTarotBookFinishStoryCsReq => 10,
            CmdTarotBookType::CmdTarotBookOpenPackScRsp => 11,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTarotBookType {
    fn default() -> Self {
        CmdTarotBookType::CmdTarotBookTypeNone
    }
}

impl CmdTarotBookType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTarotBookType>("CmdTarotBookType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdTarotBookType.proto*\xaf\x03\n\x10CmdTarotBookType\x12\x18\n\
    \x14CmdTarotBookTypeNone\x10\0\x12'\n\"CmdTarotBookFinishInteractionScRs\
    p\x10\xde?\x12\x1d\n\x18CmdTarotBookGetDataCsReq\x10\xdd?\x12!\n\x1cCmdT\
    arotBookUnlockStoryCsReq\x10\xd0?\x12!\n\x1cCmdTarotBookFinishStoryScRsp\
    \x10\xda?\x12!\n\x1cCmdTarotBookUnlockStoryScRsp\x10\xcd?\x12\x1d\n\x18C\
    mdTarotBookGetDataScRsp\x10\xd4?\x12\x1e\n\x19CmdTarotBookOpenPackCsReq\
    \x10\xd8?\x12'\n\"CmdTarotBookFinishInteractionCsReq\x10\xd2?\x12%\n\x20\
    CmdTarotBookModifyEnergyScNotify\x10\xdb?\x12!\n\x1cCmdTarotBookFinishSt\
    oryCsReq\x10\xdf?\x12\x1e\n\x19CmdTarotBookOpenPackScRsp\x10\xce?b\x06pr\
    oto3\
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
            enums.push(CmdTarotBookType::generated_enum_descriptor_data());
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
