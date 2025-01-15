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

//! Generated file from `PKEJPPIEKKE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PKEJPPIEKKE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PKEJPPIEKKE {
    // message fields
    // @@protoc_insertion_point(field:PKEJPPIEKKE.PELPFKPFLEB)
    pub PELPFKPFLEB: ::std::vec::Vec<super::EBJPELAONKH::EBJPELAONKH>,
    // @@protoc_insertion_point(field:PKEJPPIEKKE.EGDJNCHDHHM)
    pub EGDJNCHDHHM: u32,
    // @@protoc_insertion_point(field:PKEJPPIEKKE.DNEFENPIIKM)
    pub DNEFENPIIKM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PKEJPPIEKKE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PKEJPPIEKKE {
    fn default() -> &'a PKEJPPIEKKE {
        <PKEJPPIEKKE as ::protobuf::Message>::default_instance()
    }
}

impl PKEJPPIEKKE {
    pub fn new() -> PKEJPPIEKKE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PELPFKPFLEB",
            |m: &PKEJPPIEKKE| { &m.PELPFKPFLEB },
            |m: &mut PKEJPPIEKKE| { &mut m.PELPFKPFLEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EGDJNCHDHHM",
            |m: &PKEJPPIEKKE| { &m.EGDJNCHDHHM },
            |m: &mut PKEJPPIEKKE| { &mut m.EGDJNCHDHHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNEFENPIIKM",
            |m: &PKEJPPIEKKE| { &m.DNEFENPIIKM },
            |m: &mut PKEJPPIEKKE| { &mut m.DNEFENPIIKM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PKEJPPIEKKE>(
            "PKEJPPIEKKE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PKEJPPIEKKE {
    const NAME: &'static str = "PKEJPPIEKKE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.PELPFKPFLEB.push(is.read_message()?);
                },
                16 => {
                    self.EGDJNCHDHHM = is.read_uint32()?;
                },
                24 => {
                    self.DNEFENPIIKM = is.read_uint32()?;
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
        for value in &self.PELPFKPFLEB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.EGDJNCHDHHM != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.EGDJNCHDHHM);
        }
        if self.DNEFENPIIKM != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DNEFENPIIKM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PELPFKPFLEB {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.EGDJNCHDHHM != 0 {
            os.write_uint32(2, self.EGDJNCHDHHM)?;
        }
        if self.DNEFENPIIKM != 0 {
            os.write_uint32(3, self.DNEFENPIIKM)?;
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

    fn new() -> PKEJPPIEKKE {
        PKEJPPIEKKE::new()
    }

    fn clear(&mut self) {
        self.PELPFKPFLEB.clear();
        self.EGDJNCHDHHM = 0;
        self.DNEFENPIIKM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PKEJPPIEKKE {
        static instance: PKEJPPIEKKE = PKEJPPIEKKE {
            PELPFKPFLEB: ::std::vec::Vec::new(),
            EGDJNCHDHHM: 0,
            DNEFENPIIKM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PKEJPPIEKKE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PKEJPPIEKKE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PKEJPPIEKKE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PKEJPPIEKKE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PKEJPPIEKKE.proto\x1a\x11EBJPELAONKH.proto\"\x81\x01\n\x0bPKEJPPIE\
    KKE\x12.\n\x0bPELPFKPFLEB\x18\x01\x20\x03(\x0b2\x0c.EBJPELAONKHR\x0bPELP\
    FKPFLEB\x12\x20\n\x0bEGDJNCHDHHM\x18\x02\x20\x01(\rR\x0bEGDJNCHDHHM\x12\
    \x20\n\x0bDNEFENPIIKM\x18\x03\x20\x01(\rR\x0bDNEFENPIIKMb\x06proto3\
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
            deps.push(super::EBJPELAONKH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PKEJPPIEKKE::generated_message_descriptor_data());
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