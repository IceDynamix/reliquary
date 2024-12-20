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

//! Generated file from `HKLJAJFHCPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:HKLJAJFHCPE)
pub enum HKLJAJFHCPE {
    // @@protoc_insertion_point(enum_value:HKLJAJFHCPE.ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE)
    ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE = 0,
    // @@protoc_insertion_point(enum_value:HKLJAJFHCPE.ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT)
    ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT = 1,
    // @@protoc_insertion_point(enum_value:HKLJAJFHCPE.ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON)
    ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON = 2,
    // @@protoc_insertion_point(enum_value:HKLJAJFHCPE.ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION)
    ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION = 3,
    // @@protoc_insertion_point(enum_value:HKLJAJFHCPE.ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION)
    ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION = 4,
}

impl ::protobuf::Enum for HKLJAJFHCPE {
    const NAME: &'static str = "HKLJAJFHCPE";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HKLJAJFHCPE> {
        match value {
            0 => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE),
            1 => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT),
            2 => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON),
            3 => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION),
            4 => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<HKLJAJFHCPE> {
        match str {
            "ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE" => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE),
            "ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT" => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT),
            "ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON" => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON),
            "ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION" => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION),
            "ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION" => ::std::option::Option::Some(HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [HKLJAJFHCPE] = &[
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE,
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT,
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON,
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION,
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNTION_TYPE_COLLECTION,
    ];
}

impl ::protobuf::EnumFull for HKLJAJFHCPE {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("HKLJAJFHCPE").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for HKLJAJFHCPE {
    fn default() -> Self {
        HKLJAJFHCPE::ROGUE_UNLOCK_FUNCTION_TYPE_MIRACLE
    }
}

impl HKLJAJFHCPE {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<HKLJAJFHCPE>("HKLJAJFHCPE")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HKLJAJFHCPE.proto*\xe3\x01\n\x0bHKLJAJFHCPE\x12&\n\"ROGUE_UNLOCK_F\
    UNCTION_TYPE_MIRACLE\x10\0\x12(\n$ROGUE_UNLOCK_FUNCTION_TYPE_SHOW_HINT\
    \x10\x01\x12.\n*ROGUE_UNLOCK_FUNCTION_TYPE_COSMOS_BAN_AEON\x10\x02\x12(\
    \n$ROGUE_UNLOCK_FUNTION_TYPE_EXHIBITION\x10\x03\x12(\n$ROGUE_UNLOCK_FUNT\
    ION_TYPE_COLLECTION\x10\x04b\x06proto3\
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
            enums.push(HKLJAJFHCPE::generated_enum_descriptor_data());
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
