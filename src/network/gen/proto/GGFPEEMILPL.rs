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

//! Generated file from `GGFPEEMILPL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GGFPEEMILPL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GGFPEEMILPL {
    // message fields
    // @@protoc_insertion_point(field:GGFPEEMILPL.EKIGFGDLMPE)
    pub EKIGFGDLMPE: ::protobuf::MessageField<super::GJNPFNDEHGL::GJNPFNDEHGL>,
    // @@protoc_insertion_point(field:GGFPEEMILPL.LDMCMAKIMBP)
    pub LDMCMAKIMBP: ::std::vec::Vec<super::FIPNNGPAFCA::FIPNNGPAFCA>,
    // @@protoc_insertion_point(field:GGFPEEMILPL.NBFEDKIOOOH)
    pub NBFEDKIOOOH: ::protobuf::MessageField<super::FIPNNGPAFCA::FIPNNGPAFCA>,
    // special fields
    // @@protoc_insertion_point(special_field:GGFPEEMILPL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GGFPEEMILPL {
    fn default() -> &'a GGFPEEMILPL {
        <GGFPEEMILPL as ::protobuf::Message>::default_instance()
    }
}

impl GGFPEEMILPL {
    pub fn new() -> GGFPEEMILPL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GJNPFNDEHGL::GJNPFNDEHGL>(
            "EKIGFGDLMPE",
            |m: &GGFPEEMILPL| { &m.EKIGFGDLMPE },
            |m: &mut GGFPEEMILPL| { &mut m.EKIGFGDLMPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LDMCMAKIMBP",
            |m: &GGFPEEMILPL| { &m.LDMCMAKIMBP },
            |m: &mut GGFPEEMILPL| { &mut m.LDMCMAKIMBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FIPNNGPAFCA::FIPNNGPAFCA>(
            "NBFEDKIOOOH",
            |m: &GGFPEEMILPL| { &m.NBFEDKIOOOH },
            |m: &mut GGFPEEMILPL| { &mut m.NBFEDKIOOOH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GGFPEEMILPL>(
            "GGFPEEMILPL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GGFPEEMILPL {
    const NAME: &'static str = "GGFPEEMILPL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EKIGFGDLMPE)?;
                },
                66 => {
                    self.LDMCMAKIMBP.push(is.read_message()?);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NBFEDKIOOOH)?;
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
        if let Some(v) = self.EKIGFGDLMPE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.LDMCMAKIMBP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.NBFEDKIOOOH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.EKIGFGDLMPE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        for v in &self.LDMCMAKIMBP {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if let Some(v) = self.NBFEDKIOOOH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> GGFPEEMILPL {
        GGFPEEMILPL::new()
    }

    fn clear(&mut self) {
        self.EKIGFGDLMPE.clear();
        self.LDMCMAKIMBP.clear();
        self.NBFEDKIOOOH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GGFPEEMILPL {
        static instance: GGFPEEMILPL = GGFPEEMILPL {
            EKIGFGDLMPE: ::protobuf::MessageField::none(),
            LDMCMAKIMBP: ::std::vec::Vec::new(),
            NBFEDKIOOOH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GGFPEEMILPL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GGFPEEMILPL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GGFPEEMILPL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GGFPEEMILPL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GGFPEEMILPL.proto\x1a\x11FIPNNGPAFCA.proto\x1a\x11GJNPFNDEHGL.prot\
    o\"\x9d\x01\n\x0bGGFPEEMILPL\x12.\n\x0bEKIGFGDLMPE\x18\r\x20\x01(\x0b2\
    \x0c.GJNPFNDEHGLR\x0bEKIGFGDLMPE\x12.\n\x0bLDMCMAKIMBP\x18\x08\x20\x03(\
    \x0b2\x0c.FIPNNGPAFCAR\x0bLDMCMAKIMBP\x12.\n\x0bNBFEDKIOOOH\x18\x01\x20\
    \x01(\x0b2\x0c.FIPNNGPAFCAR\x0bNBFEDKIOOOHb\x06proto3\
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
            deps.push(super::FIPNNGPAFCA::file_descriptor().clone());
            deps.push(super::GJNPFNDEHGL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GGFPEEMILPL::generated_message_descriptor_data());
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