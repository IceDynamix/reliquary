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

//! Generated file from `CmdTrackPhotoActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTrackPhotoActivityType)
pub enum CmdTrackPhotoActivityType {
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdTrackPhotoActivityTypeNone)
    CmdTrackPhotoActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdStartTrackPhotoStageScRsp)
    CmdStartTrackPhotoStageScRsp = 7559,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdQuitTrackPhotoStageScRsp)
    CmdQuitTrackPhotoStageScRsp = 7553,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdGetTrackPhotoActivityDataCsReq)
    CmdGetTrackPhotoActivityDataCsReq = 7552,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdGetTrackPhotoActivityDataScRsp)
    CmdGetTrackPhotoActivityDataScRsp = 7555,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdQuitTrackPhotoStageCsReq)
    CmdQuitTrackPhotoStageCsReq = 7556,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdSettleTrackPhotoStageCsReq)
    CmdSettleTrackPhotoStageCsReq = 7554,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdStartTrackPhotoStageCsReq)
    CmdStartTrackPhotoStageCsReq = 7551,
    // @@protoc_insertion_point(enum_value:CmdTrackPhotoActivityType.CmdSettleTrackPhotoStageScRsp)
    CmdSettleTrackPhotoStageScRsp = 7557,
}

impl ::protobuf::Enum for CmdTrackPhotoActivityType {
    const NAME: &'static str = "CmdTrackPhotoActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTrackPhotoActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdTrackPhotoActivityTypeNone),
            7559 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdStartTrackPhotoStageScRsp),
            7553 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageScRsp),
            7552 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataCsReq),
            7555 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataScRsp),
            7556 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageCsReq),
            7554 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageCsReq),
            7551 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdStartTrackPhotoStageCsReq),
            7557 => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTrackPhotoActivityType> {
        match str {
            "CmdTrackPhotoActivityTypeNone" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdTrackPhotoActivityTypeNone),
            "CmdStartTrackPhotoStageScRsp" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdStartTrackPhotoStageScRsp),
            "CmdQuitTrackPhotoStageScRsp" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageScRsp),
            "CmdGetTrackPhotoActivityDataCsReq" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataCsReq),
            "CmdGetTrackPhotoActivityDataScRsp" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataScRsp),
            "CmdQuitTrackPhotoStageCsReq" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageCsReq),
            "CmdSettleTrackPhotoStageCsReq" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageCsReq),
            "CmdStartTrackPhotoStageCsReq" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdStartTrackPhotoStageCsReq),
            "CmdSettleTrackPhotoStageScRsp" => ::std::option::Option::Some(CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTrackPhotoActivityType] = &[
        CmdTrackPhotoActivityType::CmdTrackPhotoActivityTypeNone,
        CmdTrackPhotoActivityType::CmdStartTrackPhotoStageScRsp,
        CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageScRsp,
        CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataCsReq,
        CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataScRsp,
        CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageCsReq,
        CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageCsReq,
        CmdTrackPhotoActivityType::CmdStartTrackPhotoStageCsReq,
        CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTrackPhotoActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTrackPhotoActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTrackPhotoActivityType::CmdTrackPhotoActivityTypeNone => 0,
            CmdTrackPhotoActivityType::CmdStartTrackPhotoStageScRsp => 1,
            CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageScRsp => 2,
            CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataCsReq => 3,
            CmdTrackPhotoActivityType::CmdGetTrackPhotoActivityDataScRsp => 4,
            CmdTrackPhotoActivityType::CmdQuitTrackPhotoStageCsReq => 5,
            CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageCsReq => 6,
            CmdTrackPhotoActivityType::CmdStartTrackPhotoStageCsReq => 7,
            CmdTrackPhotoActivityType::CmdSettleTrackPhotoStageScRsp => 8,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTrackPhotoActivityType {
    fn default() -> Self {
        CmdTrackPhotoActivityType::CmdTrackPhotoActivityTypeNone
    }
}

impl CmdTrackPhotoActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTrackPhotoActivityType>("CmdTrackPhotoActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fCmdTrackPhotoActivityType.proto*\xe0\x02\n\x19CmdTrackPhotoActivit\
    yType\x12!\n\x1dCmdTrackPhotoActivityTypeNone\x10\0\x12!\n\x1cCmdStartTr\
    ackPhotoStageScRsp\x10\x87;\x12\x20\n\x1bCmdQuitTrackPhotoStageScRsp\x10\
    \x81;\x12&\n!CmdGetTrackPhotoActivityDataCsReq\x10\x80;\x12&\n!CmdGetTra\
    ckPhotoActivityDataScRsp\x10\x83;\x12\x20\n\x1bCmdQuitTrackPhotoStageCsR\
    eq\x10\x84;\x12\"\n\x1dCmdSettleTrackPhotoStageCsReq\x10\x82;\x12!\n\x1c\
    CmdStartTrackPhotoStageCsReq\x10\xff:\x12\"\n\x1dCmdSettleTrackPhotoStag\
    eScRsp\x10\x85;b\x06proto3\
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
            enums.push(CmdTrackPhotoActivityType::generated_enum_descriptor_data());
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
