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

//! Generated file from `GetStuffType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:GetStuffType)
pub enum GetStuffType {
    // @@protoc_insertion_point(enum_value:GetStuffType.UNKNOW)
    UNKNOW = 0,
    // @@protoc_insertion_point(enum_value:GetStuffType.MISSION_REWARD)
    MISSION_REWARD = 1,
    // @@protoc_insertion_point(enum_value:GetStuffType.EVENT_BUY_STUFF)
    EVENT_BUY_STUFF = 2,
    // @@protoc_insertion_point(enum_value:GetStuffType.MARKET_BUY_STUFF)
    MARKET_BUY_STUFF = 3,
    // @@protoc_insertion_point(enum_value:GetStuffType.QUEST_REWARD)
    QUEST_REWARD = 4,
    // @@protoc_insertion_point(enum_value:GetStuffType.INITIAL)
    INITIAL = 5,
    // @@protoc_insertion_point(enum_value:GetStuffType.PHASE_FINISH_REWARD)
    PHASE_FINISH_REWARD = 6,
}

impl ::protobuf::Enum for GetStuffType {
    const NAME: &'static str = "GetStuffType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GetStuffType> {
        match value {
            0 => ::std::option::Option::Some(GetStuffType::UNKNOW),
            1 => ::std::option::Option::Some(GetStuffType::MISSION_REWARD),
            2 => ::std::option::Option::Some(GetStuffType::EVENT_BUY_STUFF),
            3 => ::std::option::Option::Some(GetStuffType::MARKET_BUY_STUFF),
            4 => ::std::option::Option::Some(GetStuffType::QUEST_REWARD),
            5 => ::std::option::Option::Some(GetStuffType::INITIAL),
            6 => ::std::option::Option::Some(GetStuffType::PHASE_FINISH_REWARD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<GetStuffType> {
        match str {
            "UNKNOW" => ::std::option::Option::Some(GetStuffType::UNKNOW),
            "MISSION_REWARD" => ::std::option::Option::Some(GetStuffType::MISSION_REWARD),
            "EVENT_BUY_STUFF" => ::std::option::Option::Some(GetStuffType::EVENT_BUY_STUFF),
            "MARKET_BUY_STUFF" => ::std::option::Option::Some(GetStuffType::MARKET_BUY_STUFF),
            "QUEST_REWARD" => ::std::option::Option::Some(GetStuffType::QUEST_REWARD),
            "INITIAL" => ::std::option::Option::Some(GetStuffType::INITIAL),
            "PHASE_FINISH_REWARD" => ::std::option::Option::Some(GetStuffType::PHASE_FINISH_REWARD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [GetStuffType] = &[
        GetStuffType::UNKNOW,
        GetStuffType::MISSION_REWARD,
        GetStuffType::EVENT_BUY_STUFF,
        GetStuffType::MARKET_BUY_STUFF,
        GetStuffType::QUEST_REWARD,
        GetStuffType::INITIAL,
        GetStuffType::PHASE_FINISH_REWARD,
    ];
}

impl ::protobuf::EnumFull for GetStuffType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("GetStuffType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for GetStuffType {
    fn default() -> Self {
        GetStuffType::UNKNOW
    }
}

impl GetStuffType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<GetStuffType>("GetStuffType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12GetStuffType.proto*\x91\x01\n\x0cGetStuffType\x12\n\n\x06UNKNOW\
    \x10\0\x12\x12\n\x0eMISSION_REWARD\x10\x01\x12\x13\n\x0fEVENT_BUY_STUFF\
    \x10\x02\x12\x14\n\x10MARKET_BUY_STUFF\x10\x03\x12\x10\n\x0cQUEST_REWARD\
    \x10\x04\x12\x0b\n\x07INITIAL\x10\x05\x12\x17\n\x13PHASE_FINISH_REWARD\
    \x10\x06b\x06proto3\
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
            enums.push(GetStuffType::generated_enum_descriptor_data());
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
