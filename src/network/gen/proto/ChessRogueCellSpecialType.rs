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

//! Generated file from `ChessRogueCellSpecialType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ChessRogueCellSpecialType)
pub enum ChessRogueCellSpecialType {
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED = 1,
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE = 2,
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED = 3,
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED = 4,
    // @@protoc_insertion_point(enum_value:ChessRogueCellSpecialType.CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP)
    CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP = 5,
}

impl ::protobuf::Enum for ChessRogueCellSpecialType {
    const NAME: &'static str = "ChessRogueCellSpecialType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChessRogueCellSpecialType> {
        match value {
            0 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE),
            1 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED),
            2 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE),
            3 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED),
            4 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED),
            5 => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ChessRogueCellSpecialType> {
        match str {
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE),
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED),
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE),
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED),
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED),
            "CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP" => ::std::option::Option::Some(ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ChessRogueCellSpecialType] = &[
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE,
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_LOCKED,
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_REPLICATE,
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED,
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED,
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_STAMP,
    ];
}

impl ::protobuf::EnumFull for ChessRogueCellSpecialType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ChessRogueCellSpecialType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ChessRogueCellSpecialType {
    fn default() -> Self {
        ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE
    }
}

impl ChessRogueCellSpecialType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ChessRogueCellSpecialType>("ChessRogueCellSpecialType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fChessRogueCellSpecialType.proto*\x98\x02\n\x19ChessRogueCellSpecia\
    lType\x12&\n\"CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE\x10\0\x12(\n$CHESS_ROGU\
    E_CELL_SPECIAL_TYPE_LOCKED\x10\x01\x12+\n'CHESS_ROGUE_CELL_SPECIAL_TYPE_\
    REPLICATE\x10\x02\x12+\n'CHESS_ROGUE_CELL_SPECIAL_TYPE_PROTECTED\x10\x03\
    \x12&\n\"CHESS_ROGUE_CELL_SPECIAL_TYPE_SEED\x10\x04\x12'\n#CHESS_ROGUE_C\
    ELL_SPECIAL_TYPE_STAMP\x10\x05b\x06proto3\
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
            enums.push(ChessRogueCellSpecialType::generated_enum_descriptor_data());
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
