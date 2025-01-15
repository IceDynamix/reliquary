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

//! Generated file from `INLDKPMBPGJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:INLDKPMBPGJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct INLDKPMBPGJ {
    // message fields
    // @@protoc_insertion_point(field:INLDKPMBPGJ.NPPMLHHDBHE)
    pub NPPMLHHDBHE: u32,
    // @@protoc_insertion_point(field:INLDKPMBPGJ.OPGBMPJAJCL)
    pub OPGBMPJAJCL: u32,
    // @@protoc_insertion_point(field:INLDKPMBPGJ.KIIIKODIIDP)
    pub KIIIKODIIDP: ::protobuf::EnumOrUnknown<super::OOLLLBPBIEL::OOLLLBPBIEL>,
    // special fields
    // @@protoc_insertion_point(special_field:INLDKPMBPGJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a INLDKPMBPGJ {
    fn default() -> &'a INLDKPMBPGJ {
        <INLDKPMBPGJ as ::protobuf::Message>::default_instance()
    }
}

impl INLDKPMBPGJ {
    pub fn new() -> INLDKPMBPGJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPPMLHHDBHE",
            |m: &INLDKPMBPGJ| { &m.NPPMLHHDBHE },
            |m: &mut INLDKPMBPGJ| { &mut m.NPPMLHHDBHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPGBMPJAJCL",
            |m: &INLDKPMBPGJ| { &m.OPGBMPJAJCL },
            |m: &mut INLDKPMBPGJ| { &mut m.OPGBMPJAJCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIIIKODIIDP",
            |m: &INLDKPMBPGJ| { &m.KIIIKODIIDP },
            |m: &mut INLDKPMBPGJ| { &mut m.KIIIKODIIDP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<INLDKPMBPGJ>(
            "INLDKPMBPGJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for INLDKPMBPGJ {
    const NAME: &'static str = "INLDKPMBPGJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.NPPMLHHDBHE = is.read_uint32()?;
                },
                80 => {
                    self.OPGBMPJAJCL = is.read_uint32()?;
                },
                64 => {
                    self.KIIIKODIIDP = is.read_enum_or_unknown()?;
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
        if self.NPPMLHHDBHE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NPPMLHHDBHE);
        }
        if self.OPGBMPJAJCL != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OPGBMPJAJCL);
        }
        if self.KIIIKODIIDP != ::protobuf::EnumOrUnknown::new(super::OOLLLBPBIEL::OOLLLBPBIEL::ROGUE_MODIFIER_CONTENT_DEFINITE) {
            my_size += ::protobuf::rt::int32_size(8, self.KIIIKODIIDP.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NPPMLHHDBHE != 0 {
            os.write_uint32(5, self.NPPMLHHDBHE)?;
        }
        if self.OPGBMPJAJCL != 0 {
            os.write_uint32(10, self.OPGBMPJAJCL)?;
        }
        if self.KIIIKODIIDP != ::protobuf::EnumOrUnknown::new(super::OOLLLBPBIEL::OOLLLBPBIEL::ROGUE_MODIFIER_CONTENT_DEFINITE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.KIIIKODIIDP))?;
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

    fn new() -> INLDKPMBPGJ {
        INLDKPMBPGJ::new()
    }

    fn clear(&mut self) {
        self.NPPMLHHDBHE = 0;
        self.OPGBMPJAJCL = 0;
        self.KIIIKODIIDP = ::protobuf::EnumOrUnknown::new(super::OOLLLBPBIEL::OOLLLBPBIEL::ROGUE_MODIFIER_CONTENT_DEFINITE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static INLDKPMBPGJ {
        static instance: INLDKPMBPGJ = INLDKPMBPGJ {
            NPPMLHHDBHE: 0,
            OPGBMPJAJCL: 0,
            KIIIKODIIDP: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for INLDKPMBPGJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("INLDKPMBPGJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for INLDKPMBPGJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INLDKPMBPGJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11INLDKPMBPGJ.proto\x1a\x11OOLLLBPBIEL.proto\"\x81\x01\n\x0bINLDKPMB\
    PGJ\x12\x20\n\x0bNPPMLHHDBHE\x18\x05\x20\x01(\rR\x0bNPPMLHHDBHE\x12\x20\
    \n\x0bOPGBMPJAJCL\x18\n\x20\x01(\rR\x0bOPGBMPJAJCL\x12.\n\x0bKIIIKODIIDP\
    \x18\x08\x20\x01(\x0e2\x0c.OOLLLBPBIELR\x0bKIIIKODIIDPb\x06proto3\
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
            deps.push(super::OOLLLBPBIEL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(INLDKPMBPGJ::generated_message_descriptor_data());
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