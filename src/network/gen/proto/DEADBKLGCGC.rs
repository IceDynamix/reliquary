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

//! Generated file from `DEADBKLGCGC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DEADBKLGCGC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DEADBKLGCGC {
    // message fields
    // @@protoc_insertion_point(field:DEADBKLGCGC.DMIPEOHGLMB)
    pub DMIPEOHGLMB: u32,
    // @@protoc_insertion_point(field:DEADBKLGCGC.NMOJLKDFCOH)
    pub NMOJLKDFCOH: ::protobuf::EnumOrUnknown<super::NODCDFGPJPP::NODCDFGPJPP>,
    // @@protoc_insertion_point(field:DEADBKLGCGC.PLLDGMICHGH)
    pub PLLDGMICHGH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DEADBKLGCGC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DEADBKLGCGC {
    fn default() -> &'a DEADBKLGCGC {
        <DEADBKLGCGC as ::protobuf::Message>::default_instance()
    }
}

impl DEADBKLGCGC {
    pub fn new() -> DEADBKLGCGC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMIPEOHGLMB",
            |m: &DEADBKLGCGC| { &m.DMIPEOHGLMB },
            |m: &mut DEADBKLGCGC| { &mut m.DMIPEOHGLMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMOJLKDFCOH",
            |m: &DEADBKLGCGC| { &m.NMOJLKDFCOH },
            |m: &mut DEADBKLGCGC| { &mut m.NMOJLKDFCOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLLDGMICHGH",
            |m: &DEADBKLGCGC| { &m.PLLDGMICHGH },
            |m: &mut DEADBKLGCGC| { &mut m.PLLDGMICHGH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DEADBKLGCGC>(
            "DEADBKLGCGC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DEADBKLGCGC {
    const NAME: &'static str = "DEADBKLGCGC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.DMIPEOHGLMB = is.read_uint32()?;
                },
                80 => {
                    self.NMOJLKDFCOH = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.PLLDGMICHGH = is.read_uint32()?;
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
        if self.DMIPEOHGLMB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DMIPEOHGLMB);
        }
        if self.NMOJLKDFCOH != ::protobuf::EnumOrUnknown::new(super::NODCDFGPJPP::NODCDFGPJPP::MAP_INFO_CHEST_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.NMOJLKDFCOH.value());
        }
        if self.PLLDGMICHGH != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PLLDGMICHGH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DMIPEOHGLMB != 0 {
            os.write_uint32(12, self.DMIPEOHGLMB)?;
        }
        if self.NMOJLKDFCOH != ::protobuf::EnumOrUnknown::new(super::NODCDFGPJPP::NODCDFGPJPP::MAP_INFO_CHEST_TYPE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.NMOJLKDFCOH))?;
        }
        if self.PLLDGMICHGH != 0 {
            os.write_uint32(14, self.PLLDGMICHGH)?;
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

    fn new() -> DEADBKLGCGC {
        DEADBKLGCGC::new()
    }

    fn clear(&mut self) {
        self.DMIPEOHGLMB = 0;
        self.NMOJLKDFCOH = ::protobuf::EnumOrUnknown::new(super::NODCDFGPJPP::NODCDFGPJPP::MAP_INFO_CHEST_TYPE_NONE);
        self.PLLDGMICHGH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DEADBKLGCGC {
        static instance: DEADBKLGCGC = DEADBKLGCGC {
            DMIPEOHGLMB: 0,
            NMOJLKDFCOH: ::protobuf::EnumOrUnknown::from_i32(0),
            PLLDGMICHGH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DEADBKLGCGC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DEADBKLGCGC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DEADBKLGCGC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DEADBKLGCGC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DEADBKLGCGC.proto\x1a\x11NODCDFGPJPP.proto\"\x81\x01\n\x0bDEADBKLG\
    CGC\x12\x20\n\x0bDMIPEOHGLMB\x18\x0c\x20\x01(\rR\x0bDMIPEOHGLMB\x12.\n\
    \x0bNMOJLKDFCOH\x18\n\x20\x01(\x0e2\x0c.NODCDFGPJPPR\x0bNMOJLKDFCOH\x12\
    \x20\n\x0bPLLDGMICHGH\x18\x0e\x20\x01(\rR\x0bPLLDGMICHGHb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::NODCDFGPJPP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DEADBKLGCGC::generated_message_descriptor_data());
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