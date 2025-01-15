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

//! Generated file from `PKOLKGLPCFO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PKOLKGLPCFO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PKOLKGLPCFO {
    // message fields
    // @@protoc_insertion_point(field:PKOLKGLPCFO.MHALGAFIHHC)
    pub MHALGAFIHHC: ::std::vec::Vec<super::AGNHABPJABC::AGNHABPJABC>,
    // @@protoc_insertion_point(field:PKOLKGLPCFO.GHELEOAFBHC)
    pub GHELEOAFBHC: u32,
    // @@protoc_insertion_point(field:PKOLKGLPCFO.JLBBJFPNHEA)
    pub JLBBJFPNHEA: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PKOLKGLPCFO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PKOLKGLPCFO {
    fn default() -> &'a PKOLKGLPCFO {
        <PKOLKGLPCFO as ::protobuf::Message>::default_instance()
    }
}

impl PKOLKGLPCFO {
    pub fn new() -> PKOLKGLPCFO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MHALGAFIHHC",
            |m: &PKOLKGLPCFO| { &m.MHALGAFIHHC },
            |m: &mut PKOLKGLPCFO| { &mut m.MHALGAFIHHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GHELEOAFBHC",
            |m: &PKOLKGLPCFO| { &m.GHELEOAFBHC },
            |m: &mut PKOLKGLPCFO| { &mut m.GHELEOAFBHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLBBJFPNHEA",
            |m: &PKOLKGLPCFO| { &m.JLBBJFPNHEA },
            |m: &mut PKOLKGLPCFO| { &mut m.JLBBJFPNHEA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PKOLKGLPCFO>(
            "PKOLKGLPCFO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PKOLKGLPCFO {
    const NAME: &'static str = "PKOLKGLPCFO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.MHALGAFIHHC.push(is.read_message()?);
                },
                64 => {
                    self.GHELEOAFBHC = is.read_uint32()?;
                },
                40 => {
                    self.JLBBJFPNHEA = is.read_bool()?;
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
        for value in &self.MHALGAFIHHC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.GHELEOAFBHC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GHELEOAFBHC);
        }
        if self.JLBBJFPNHEA != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MHALGAFIHHC {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.GHELEOAFBHC != 0 {
            os.write_uint32(8, self.GHELEOAFBHC)?;
        }
        if self.JLBBJFPNHEA != false {
            os.write_bool(5, self.JLBBJFPNHEA)?;
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

    fn new() -> PKOLKGLPCFO {
        PKOLKGLPCFO::new()
    }

    fn clear(&mut self) {
        self.MHALGAFIHHC.clear();
        self.GHELEOAFBHC = 0;
        self.JLBBJFPNHEA = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PKOLKGLPCFO {
        static instance: PKOLKGLPCFO = PKOLKGLPCFO {
            MHALGAFIHHC: ::std::vec::Vec::new(),
            GHELEOAFBHC: 0,
            JLBBJFPNHEA: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PKOLKGLPCFO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PKOLKGLPCFO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PKOLKGLPCFO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PKOLKGLPCFO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PKOLKGLPCFO.proto\x1a\x11AGNHABPJABC.proto\"\x81\x01\n\x0bPKOLKGLP\
    CFO\x12.\n\x0bMHALGAFIHHC\x18\x01\x20\x03(\x0b2\x0c.AGNHABPJABCR\x0bMHAL\
    GAFIHHC\x12\x20\n\x0bGHELEOAFBHC\x18\x08\x20\x01(\rR\x0bGHELEOAFBHC\x12\
    \x20\n\x0bJLBBJFPNHEA\x18\x05\x20\x01(\x08R\x0bJLBBJFPNHEAb\x06proto3\
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
            deps.push(super::AGNHABPJABC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PKOLKGLPCFO::generated_message_descriptor_data());
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