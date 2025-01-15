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

//! Generated file from `MDHBKFGIJJL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MDHBKFGIJJL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MDHBKFGIJJL {
    // message fields
    // @@protoc_insertion_point(field:MDHBKFGIJJL.FFKNMAONGIB)
    pub FFKNMAONGIB: u32,
    // @@protoc_insertion_point(field:MDHBKFGIJJL.AMAFBBDILID)
    pub AMAFBBDILID: ::protobuf::EnumOrUnknown<super::DifficultyAdjustmentType::DifficultyAdjustmentType>,
    // @@protoc_insertion_point(field:MDHBKFGIJJL.LEEACMNGDFL)
    pub LEEACMNGDFL: ::protobuf::EnumOrUnknown<super::PIKBNAHDDFL::PIKBNAHDDFL>,
    // special fields
    // @@protoc_insertion_point(special_field:MDHBKFGIJJL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MDHBKFGIJJL {
    fn default() -> &'a MDHBKFGIJJL {
        <MDHBKFGIJJL as ::protobuf::Message>::default_instance()
    }
}

impl MDHBKFGIJJL {
    pub fn new() -> MDHBKFGIJJL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFKNMAONGIB",
            |m: &MDHBKFGIJJL| { &m.FFKNMAONGIB },
            |m: &mut MDHBKFGIJJL| { &mut m.FFKNMAONGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AMAFBBDILID",
            |m: &MDHBKFGIJJL| { &m.AMAFBBDILID },
            |m: &mut MDHBKFGIJJL| { &mut m.AMAFBBDILID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEEACMNGDFL",
            |m: &MDHBKFGIJJL| { &m.LEEACMNGDFL },
            |m: &mut MDHBKFGIJJL| { &mut m.LEEACMNGDFL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MDHBKFGIJJL>(
            "MDHBKFGIJJL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MDHBKFGIJJL {
    const NAME: &'static str = "MDHBKFGIJJL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FFKNMAONGIB = is.read_uint32()?;
                },
                88 => {
                    self.AMAFBBDILID = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.LEEACMNGDFL = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.FFKNMAONGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FFKNMAONGIB);
        }
        if self.AMAFBBDILID != ::protobuf::EnumOrUnknown::new(super::DifficultyAdjustmentType::DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(11, self.AMAFBBDILID.value());
        }
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::PIKBNAHDDFL::PIKBNAHDDFL::DIFFICULTY_AJUSTMENT_SOURCE_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.LEEACMNGDFL.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FFKNMAONGIB != 0 {
            os.write_uint32(1, self.FFKNMAONGIB)?;
        }
        if self.AMAFBBDILID != ::protobuf::EnumOrUnknown::new(super::DifficultyAdjustmentType::DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.AMAFBBDILID))?;
        }
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::PIKBNAHDDFL::PIKBNAHDDFL::DIFFICULTY_AJUSTMENT_SOURCE_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.LEEACMNGDFL))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MDHBKFGIJJL {
        MDHBKFGIJJL::new()
    }

    fn clear(&mut self) {
        self.FFKNMAONGIB = 0;
        self.AMAFBBDILID = ::protobuf::EnumOrUnknown::new(super::DifficultyAdjustmentType::DifficultyAdjustmentType::DIFFICULTY_AJUSTMENT_TYPE_DEFAULT);
        self.LEEACMNGDFL = ::protobuf::EnumOrUnknown::new(super::PIKBNAHDDFL::PIKBNAHDDFL::DIFFICULTY_AJUSTMENT_SOURCE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MDHBKFGIJJL {
        static instance: MDHBKFGIJJL = MDHBKFGIJJL {
            FFKNMAONGIB: 0,
            AMAFBBDILID: ::protobuf::EnumOrUnknown::from_i32(0),
            LEEACMNGDFL: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MDHBKFGIJJL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MDHBKFGIJJL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MDHBKFGIJJL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MDHBKFGIJJL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MDHBKFGIJJL.proto\x1a\x1eDifficultyAdjustmentType.proto\x1a\x11PIK\
    BNAHDDFL.proto\"\x9c\x01\n\x0bMDHBKFGIJJL\x12\x20\n\x0bFFKNMAONGIB\x18\
    \x01\x20\x01(\rR\x0bFFKNMAONGIB\x12;\n\x0bAMAFBBDILID\x18\x0b\x20\x01(\
    \x0e2\x19.DifficultyAdjustmentTypeR\x0bAMAFBBDILID\x12.\n\x0bLEEACMNGDFL\
    \x18\x07\x20\x01(\x0e2\x0c.PIKBNAHDDFLR\x0bLEEACMNGDFLb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::DifficultyAdjustmentType::file_descriptor().clone());
            deps.push(super::PIKBNAHDDFL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MDHBKFGIJJL::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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