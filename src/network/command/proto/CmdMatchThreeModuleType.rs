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

//! Generated file from `CmdMatchThreeModuleType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMatchThreeModuleType)
pub enum CmdMatchThreeModuleType {
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeModuleTypeNone)
    CmdMatchThreeModuleTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeSetBirdPosCsReq)
    CmdMatchThreeSetBirdPosCsReq = 7417,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeLevelEndScRsp)
    CmdMatchThreeLevelEndScRsp = 7437,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeSetBirdPosScRsp)
    CmdMatchThreeSetBirdPosScRsp = 7427,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeSyncDataScNotify)
    CmdMatchThreeSyncDataScNotify = 7436,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeGetDataScRsp)
    CmdMatchThreeGetDataScRsp = 7446,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeLevelEndCsReq)
    CmdMatchThreeLevelEndCsReq = 7402,
    // @@protoc_insertion_point(enum_value:CmdMatchThreeModuleType.CmdMatchThreeGetDataCsReq)
    CmdMatchThreeGetDataCsReq = 7404,
}

impl ::protobuf::Enum for CmdMatchThreeModuleType {
    const NAME: &'static str = "CmdMatchThreeModuleType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMatchThreeModuleType> {
        match value {
            0 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeModuleTypeNone),
            7417 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosCsReq),
            7437 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeLevelEndScRsp),
            7427 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosScRsp),
            7436 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSyncDataScNotify),
            7446 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeGetDataScRsp),
            7402 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeLevelEndCsReq),
            7404 => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMatchThreeModuleType> {
        match str {
            "CmdMatchThreeModuleTypeNone" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeModuleTypeNone),
            "CmdMatchThreeSetBirdPosCsReq" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosCsReq),
            "CmdMatchThreeLevelEndScRsp" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeLevelEndScRsp),
            "CmdMatchThreeSetBirdPosScRsp" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosScRsp),
            "CmdMatchThreeSyncDataScNotify" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeSyncDataScNotify),
            "CmdMatchThreeGetDataScRsp" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeGetDataScRsp),
            "CmdMatchThreeLevelEndCsReq" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeLevelEndCsReq),
            "CmdMatchThreeGetDataCsReq" => ::std::option::Option::Some(CmdMatchThreeModuleType::CmdMatchThreeGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMatchThreeModuleType] = &[
        CmdMatchThreeModuleType::CmdMatchThreeModuleTypeNone,
        CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosCsReq,
        CmdMatchThreeModuleType::CmdMatchThreeLevelEndScRsp,
        CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosScRsp,
        CmdMatchThreeModuleType::CmdMatchThreeSyncDataScNotify,
        CmdMatchThreeModuleType::CmdMatchThreeGetDataScRsp,
        CmdMatchThreeModuleType::CmdMatchThreeLevelEndCsReq,
        CmdMatchThreeModuleType::CmdMatchThreeGetDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdMatchThreeModuleType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMatchThreeModuleType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMatchThreeModuleType::CmdMatchThreeModuleTypeNone => 0,
            CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosCsReq => 1,
            CmdMatchThreeModuleType::CmdMatchThreeLevelEndScRsp => 2,
            CmdMatchThreeModuleType::CmdMatchThreeSetBirdPosScRsp => 3,
            CmdMatchThreeModuleType::CmdMatchThreeSyncDataScNotify => 4,
            CmdMatchThreeModuleType::CmdMatchThreeGetDataScRsp => 5,
            CmdMatchThreeModuleType::CmdMatchThreeLevelEndCsReq => 6,
            CmdMatchThreeModuleType::CmdMatchThreeGetDataCsReq => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMatchThreeModuleType {
    fn default() -> Self {
        CmdMatchThreeModuleType::CmdMatchThreeModuleTypeNone
    }
}

impl CmdMatchThreeModuleType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMatchThreeModuleType>("CmdMatchThreeModuleType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dCmdMatchThreeModuleType.proto*\xa6\x02\n\x17CmdMatchThreeModuleTyp\
    e\x12\x1f\n\x1bCmdMatchThreeModuleTypeNone\x10\0\x12!\n\x1cCmdMatchThree\
    SetBirdPosCsReq\x10\xf99\x12\x1f\n\x1aCmdMatchThreeLevelEndScRsp\x10\x8d\
    :\x12!\n\x1cCmdMatchThreeSetBirdPosScRsp\x10\x83:\x12\"\n\x1dCmdMatchThr\
    eeSyncDataScNotify\x10\x8c:\x12\x1e\n\x19CmdMatchThreeGetDataScRsp\x10\
    \x96:\x12\x1f\n\x1aCmdMatchThreeLevelEndCsReq\x10\xea9\x12\x1e\n\x19CmdM\
    atchThreeGetDataCsReq\x10\xec9b\x06proto3\
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
            enums.push(CmdMatchThreeModuleType::generated_enum_descriptor_data());
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
