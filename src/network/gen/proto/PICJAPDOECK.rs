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

//! Generated file from `PICJAPDOECK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PICJAPDOECK)
pub enum PICJAPDOECK {
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE)
    ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE = 0,
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT)
    ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT = 1,
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON)
    ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON = 2,
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION)
    ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION = 3,
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION)
    ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION = 4,
    // @@protoc_insertion_point(enum_value:PICJAPDOECK.ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE)
    ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE = 5,
}

impl ::protobuf::Enum for PICJAPDOECK {
    const NAME: &'static str = "PICJAPDOECK";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PICJAPDOECK> {
        match value {
            0 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE),
            1 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT),
            2 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON),
            3 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION),
            4 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION),
            5 => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PICJAPDOECK> {
        match str {
            "ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE),
            "ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT),
            "ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON),
            "ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION),
            "ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION),
            "ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE" => ::std::option::Option::Some(PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PICJAPDOECK] = &[
        PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE,
        PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT,
        PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON,
        PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION,
        PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION,
        PICJAPDOECK::ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_MODE,
    ];
}

impl ::protobuf::EnumFull for PICJAPDOECK {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PICJAPDOECK").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PICJAPDOECK {
    fn default() -> Self {
        PICJAPDOECK::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE
    }
}

impl PICJAPDOECK {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PICJAPDOECK>("PICJAPDOECK")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PICJAPDOECK.proto*\x91\x02\n\x0bPICJAPDOECK\x12&\n\"ROGUE_UNLOCK_F\
    UNCTION_TYPE_MIRACLE\x10\0\x12(\n$ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT\
    \x10\x01\x12.\n*ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON\x10\x02\x12(\
    \n$ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION\x10\x03\x12(\n$ROGUE_UNLOCK_FUNT\
    ION_TYPE_COLLECTION\x10\x04\x12,\n(ROGUE_UNLOCK_FUNTION_TYPE_TOURN_GOD_M\
    ODE\x10\x05b\x06proto3\
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
            enums.push(PICJAPDOECK::generated_enum_descriptor_data());
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