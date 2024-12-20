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

//! Generated file from `CmdQuestType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdQuestType)
pub enum CmdQuestType {
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdQuestTypeNone)
    CmdQuestTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdGetQuestRecordCsReq)
    CmdGetQuestRecordCsReq = 934,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdTakeQuestRewardScRsp)
    CmdTakeQuestRewardScRsp = 946,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdBatchGetQuestDataScRsp)
    CmdBatchGetQuestDataScRsp = 990,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdQuestRecordScNotify)
    CmdQuestRecordScNotify = 980,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdTakeQuestOptionalRewardCsReq)
    CmdTakeQuestOptionalRewardCsReq = 974,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdBatchGetQuestDataCsReq)
    CmdBatchGetQuestDataCsReq = 948,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdGetQuestRecordScRsp)
    CmdGetQuestRecordScRsp = 937,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdGetQuestDataScRsp)
    CmdGetQuestDataScRsp = 920,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdFinishQuestScRsp)
    CmdFinishQuestScRsp = 947,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdGetQuestDataCsReq)
    CmdGetQuestDataCsReq = 959,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdFinishQuestCsReq)
    CmdFinishQuestCsReq = 916,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdTakeQuestOptionalRewardScRsp)
    CmdTakeQuestOptionalRewardScRsp = 930,
    // @@protoc_insertion_point(enum_value:CmdQuestType.CmdTakeQuestRewardCsReq)
    CmdTakeQuestRewardCsReq = 903,
}

impl ::protobuf::Enum for CmdQuestType {
    const NAME: &'static str = "CmdQuestType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdQuestType> {
        match value {
            0 => ::std::option::Option::Some(CmdQuestType::CmdQuestTypeNone),
            934 => ::std::option::Option::Some(CmdQuestType::CmdGetQuestRecordCsReq),
            946 => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestRewardScRsp),
            990 => ::std::option::Option::Some(CmdQuestType::CmdBatchGetQuestDataScRsp),
            980 => ::std::option::Option::Some(CmdQuestType::CmdQuestRecordScNotify),
            974 => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestOptionalRewardCsReq),
            948 => ::std::option::Option::Some(CmdQuestType::CmdBatchGetQuestDataCsReq),
            937 => ::std::option::Option::Some(CmdQuestType::CmdGetQuestRecordScRsp),
            920 => ::std::option::Option::Some(CmdQuestType::CmdGetQuestDataScRsp),
            947 => ::std::option::Option::Some(CmdQuestType::CmdFinishQuestScRsp),
            959 => ::std::option::Option::Some(CmdQuestType::CmdGetQuestDataCsReq),
            916 => ::std::option::Option::Some(CmdQuestType::CmdFinishQuestCsReq),
            930 => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestOptionalRewardScRsp),
            903 => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdQuestType> {
        match str {
            "CmdQuestTypeNone" => ::std::option::Option::Some(CmdQuestType::CmdQuestTypeNone),
            "CmdGetQuestRecordCsReq" => ::std::option::Option::Some(CmdQuestType::CmdGetQuestRecordCsReq),
            "CmdTakeQuestRewardScRsp" => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestRewardScRsp),
            "CmdBatchGetQuestDataScRsp" => ::std::option::Option::Some(CmdQuestType::CmdBatchGetQuestDataScRsp),
            "CmdQuestRecordScNotify" => ::std::option::Option::Some(CmdQuestType::CmdQuestRecordScNotify),
            "CmdTakeQuestOptionalRewardCsReq" => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestOptionalRewardCsReq),
            "CmdBatchGetQuestDataCsReq" => ::std::option::Option::Some(CmdQuestType::CmdBatchGetQuestDataCsReq),
            "CmdGetQuestRecordScRsp" => ::std::option::Option::Some(CmdQuestType::CmdGetQuestRecordScRsp),
            "CmdGetQuestDataScRsp" => ::std::option::Option::Some(CmdQuestType::CmdGetQuestDataScRsp),
            "CmdFinishQuestScRsp" => ::std::option::Option::Some(CmdQuestType::CmdFinishQuestScRsp),
            "CmdGetQuestDataCsReq" => ::std::option::Option::Some(CmdQuestType::CmdGetQuestDataCsReq),
            "CmdFinishQuestCsReq" => ::std::option::Option::Some(CmdQuestType::CmdFinishQuestCsReq),
            "CmdTakeQuestOptionalRewardScRsp" => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestOptionalRewardScRsp),
            "CmdTakeQuestRewardCsReq" => ::std::option::Option::Some(CmdQuestType::CmdTakeQuestRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdQuestType] = &[
        CmdQuestType::CmdQuestTypeNone,
        CmdQuestType::CmdGetQuestRecordCsReq,
        CmdQuestType::CmdTakeQuestRewardScRsp,
        CmdQuestType::CmdBatchGetQuestDataScRsp,
        CmdQuestType::CmdQuestRecordScNotify,
        CmdQuestType::CmdTakeQuestOptionalRewardCsReq,
        CmdQuestType::CmdBatchGetQuestDataCsReq,
        CmdQuestType::CmdGetQuestRecordScRsp,
        CmdQuestType::CmdGetQuestDataScRsp,
        CmdQuestType::CmdFinishQuestScRsp,
        CmdQuestType::CmdGetQuestDataCsReq,
        CmdQuestType::CmdFinishQuestCsReq,
        CmdQuestType::CmdTakeQuestOptionalRewardScRsp,
        CmdQuestType::CmdTakeQuestRewardCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdQuestType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdQuestType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdQuestType::CmdQuestTypeNone => 0,
            CmdQuestType::CmdGetQuestRecordCsReq => 1,
            CmdQuestType::CmdTakeQuestRewardScRsp => 2,
            CmdQuestType::CmdBatchGetQuestDataScRsp => 3,
            CmdQuestType::CmdQuestRecordScNotify => 4,
            CmdQuestType::CmdTakeQuestOptionalRewardCsReq => 5,
            CmdQuestType::CmdBatchGetQuestDataCsReq => 6,
            CmdQuestType::CmdGetQuestRecordScRsp => 7,
            CmdQuestType::CmdGetQuestDataScRsp => 8,
            CmdQuestType::CmdFinishQuestScRsp => 9,
            CmdQuestType::CmdGetQuestDataCsReq => 10,
            CmdQuestType::CmdFinishQuestCsReq => 11,
            CmdQuestType::CmdTakeQuestOptionalRewardScRsp => 12,
            CmdQuestType::CmdTakeQuestRewardCsReq => 13,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdQuestType {
    fn default() -> Self {
        CmdQuestType::CmdQuestTypeNone
    }
}

impl CmdQuestType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdQuestType>("CmdQuestType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdQuestType.proto*\xad\x03\n\x0cCmdQuestType\x12\x14\n\x10CmdQues\
    tTypeNone\x10\0\x12\x1b\n\x16CmdGetQuestRecordCsReq\x10\xa6\x07\x12\x1c\
    \n\x17CmdTakeQuestRewardScRsp\x10\xb2\x07\x12\x1e\n\x19CmdBatchGetQuestD\
    ataScRsp\x10\xde\x07\x12\x1b\n\x16CmdQuestRecordScNotify\x10\xd4\x07\x12\
    $\n\x1fCmdTakeQuestOptionalRewardCsReq\x10\xce\x07\x12\x1e\n\x19CmdBatch\
    GetQuestDataCsReq\x10\xb4\x07\x12\x1b\n\x16CmdGetQuestRecordScRsp\x10\
    \xa9\x07\x12\x19\n\x14CmdGetQuestDataScRsp\x10\x98\x07\x12\x18\n\x13CmdF\
    inishQuestScRsp\x10\xb3\x07\x12\x19\n\x14CmdGetQuestDataCsReq\x10\xbf\
    \x07\x12\x18\n\x13CmdFinishQuestCsReq\x10\x94\x07\x12$\n\x1fCmdTakeQuest\
    OptionalRewardScRsp\x10\xa2\x07\x12\x1c\n\x17CmdTakeQuestRewardCsReq\x10\
    \x87\x07b\x06proto3\
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
            enums.push(CmdQuestType::generated_enum_descriptor_data());
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
