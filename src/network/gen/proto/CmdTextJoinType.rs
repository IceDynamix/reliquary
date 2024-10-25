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

//! Generated file from `CmdTextJoinType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTextJoinType)
pub enum CmdTextJoinType {
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinTypeNone)
    CmdTextJoinTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinBatchSaveCsReq)
    CmdTextJoinBatchSaveCsReq = 3879,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinQueryCsReq)
    CmdTextJoinQueryCsReq = 3883,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinSaveCsReq)
    CmdTextJoinSaveCsReq = 3898,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinSaveScRsp)
    CmdTextJoinSaveScRsp = 3871,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinQueryScRsp)
    CmdTextJoinQueryScRsp = 3842,
    // @@protoc_insertion_point(enum_value:CmdTextJoinType.CmdTextJoinBatchSaveScRsp)
    CmdTextJoinBatchSaveScRsp = 3877,
}

impl ::protobuf::Enum for CmdTextJoinType {
    const NAME: &'static str = "CmdTextJoinType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTextJoinType> {
        match value {
            0 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinTypeNone),
            3879 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinBatchSaveCsReq),
            3883 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinQueryCsReq),
            3898 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinSaveCsReq),
            3871 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinSaveScRsp),
            3842 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinQueryScRsp),
            3877 => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinBatchSaveScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTextJoinType> {
        match str {
            "CmdTextJoinTypeNone" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinTypeNone),
            "CmdTextJoinBatchSaveCsReq" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinBatchSaveCsReq),
            "CmdTextJoinQueryCsReq" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinQueryCsReq),
            "CmdTextJoinSaveCsReq" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinSaveCsReq),
            "CmdTextJoinSaveScRsp" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinSaveScRsp),
            "CmdTextJoinQueryScRsp" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinQueryScRsp),
            "CmdTextJoinBatchSaveScRsp" => ::std::option::Option::Some(CmdTextJoinType::CmdTextJoinBatchSaveScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTextJoinType] = &[
        CmdTextJoinType::CmdTextJoinTypeNone,
        CmdTextJoinType::CmdTextJoinBatchSaveCsReq,
        CmdTextJoinType::CmdTextJoinQueryCsReq,
        CmdTextJoinType::CmdTextJoinSaveCsReq,
        CmdTextJoinType::CmdTextJoinSaveScRsp,
        CmdTextJoinType::CmdTextJoinQueryScRsp,
        CmdTextJoinType::CmdTextJoinBatchSaveScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTextJoinType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTextJoinType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTextJoinType::CmdTextJoinTypeNone => 0,
            CmdTextJoinType::CmdTextJoinBatchSaveCsReq => 1,
            CmdTextJoinType::CmdTextJoinQueryCsReq => 2,
            CmdTextJoinType::CmdTextJoinSaveCsReq => 3,
            CmdTextJoinType::CmdTextJoinSaveScRsp => 4,
            CmdTextJoinType::CmdTextJoinQueryScRsp => 5,
            CmdTextJoinType::CmdTextJoinBatchSaveScRsp => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTextJoinType {
    fn default() -> Self {
        CmdTextJoinType::CmdTextJoinTypeNone
    }
}

impl CmdTextJoinType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTextJoinType>("CmdTextJoinType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdTextJoinType.proto*\xd8\x01\n\x0fCmdTextJoinType\x12\x17\n\x13C\
    mdTextJoinTypeNone\x10\0\x12\x1e\n\x19CmdTextJoinBatchSaveCsReq\x10\xa7\
    \x1e\x12\x1a\n\x15CmdTextJoinQueryCsReq\x10\xab\x1e\x12\x19\n\x14CmdText\
    JoinSaveCsReq\x10\xba\x1e\x12\x19\n\x14CmdTextJoinSaveScRsp\x10\x9f\x1e\
    \x12\x1a\n\x15CmdTextJoinQueryScRsp\x10\x82\x1e\x12\x1e\n\x19CmdTextJoin\
    BatchSaveScRsp\x10\xa5\x1eb\x06proto3\
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
            enums.push(CmdTextJoinType::generated_enum_descriptor_data());
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
