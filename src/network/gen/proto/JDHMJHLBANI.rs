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

//! Generated file from `JDHMJHLBANI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JDHMJHLBANI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JDHMJHLBANI {
    // message fields
    // @@protoc_insertion_point(field:JDHMJHLBANI.EDHMCHMFHHN)
    pub EDHMCHMFHHN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JDHMJHLBANI.AHCGCLKLGIB)
    pub AHCGCLKLGIB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JDHMJHLBANI.KMMBOINECPE)
    pub KMMBOINECPE: ::std::vec::Vec<super::BFGDBMGOKLL::BFGDBMGOKLL>,
    // @@protoc_insertion_point(field:JDHMJHLBANI.relic_list)
    pub relic_list: ::std::vec::Vec<super::HMEBDMNACOM::HMEBDMNACOM>,
    // @@protoc_insertion_point(field:JDHMJHLBANI.FHONFJHLGEK)
    pub FHONFJHLGEK: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:JDHMJHLBANI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JDHMJHLBANI {
    fn default() -> &'a JDHMJHLBANI {
        <JDHMJHLBANI as ::protobuf::Message>::default_instance()
    }
}

impl JDHMJHLBANI {
    pub fn new() -> JDHMJHLBANI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EDHMCHMFHHN",
            |m: &JDHMJHLBANI| { &m.EDHMCHMFHHN },
            |m: &mut JDHMJHLBANI| { &mut m.EDHMCHMFHHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AHCGCLKLGIB",
            |m: &JDHMJHLBANI| { &m.AHCGCLKLGIB },
            |m: &mut JDHMJHLBANI| { &mut m.AHCGCLKLGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KMMBOINECPE",
            |m: &JDHMJHLBANI| { &m.KMMBOINECPE },
            |m: &mut JDHMJHLBANI| { &mut m.KMMBOINECPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "relic_list",
            |m: &JDHMJHLBANI| { &m.relic_list },
            |m: &mut JDHMJHLBANI| { &mut m.relic_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FHONFJHLGEK",
            |m: &JDHMJHLBANI| { &m.FHONFJHLGEK },
            |m: &mut JDHMJHLBANI| { &mut m.FHONFJHLGEK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JDHMJHLBANI>(
            "JDHMJHLBANI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JDHMJHLBANI {
    const NAME: &'static str = "JDHMJHLBANI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.EDHMCHMFHHN)?;
                },
                112 => {
                    self.EDHMCHMFHHN.push(is.read_uint32()?);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.AHCGCLKLGIB)?;
                },
                8 => {
                    self.AHCGCLKLGIB.push(is.read_uint32()?);
                },
                50 => {
                    self.KMMBOINECPE.push(is.read_message()?);
                },
                98 => {
                    self.relic_list.push(is.read_message()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.FHONFJHLGEK)?;
                },
                104 => {
                    self.FHONFJHLGEK.push(is.read_uint32()?);
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
        for value in &self.EDHMCHMFHHN {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        for value in &self.AHCGCLKLGIB {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for value in &self.KMMBOINECPE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.relic_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.FHONFJHLGEK {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EDHMCHMFHHN {
            os.write_uint32(14, *v)?;
        };
        for v in &self.AHCGCLKLGIB {
            os.write_uint32(1, *v)?;
        };
        for v in &self.KMMBOINECPE {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.relic_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.FHONFJHLGEK {
            os.write_uint32(13, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> JDHMJHLBANI {
        JDHMJHLBANI::new()
    }

    fn clear(&mut self) {
        self.EDHMCHMFHHN.clear();
        self.AHCGCLKLGIB.clear();
        self.KMMBOINECPE.clear();
        self.relic_list.clear();
        self.FHONFJHLGEK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JDHMJHLBANI {
        static instance: JDHMJHLBANI = JDHMJHLBANI {
            EDHMCHMFHHN: ::std::vec::Vec::new(),
            AHCGCLKLGIB: ::std::vec::Vec::new(),
            KMMBOINECPE: ::std::vec::Vec::new(),
            relic_list: ::std::vec::Vec::new(),
            FHONFJHLGEK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JDHMJHLBANI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JDHMJHLBANI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JDHMJHLBANI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JDHMJHLBANI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JDHMJHLBANI.proto\x1a\x11BFGDBMGOKLL.proto\x1a\x11HMEBDMNACOM.prot\
    o\"\xd0\x01\n\x0bJDHMJHLBANI\x12\x20\n\x0bEDHMCHMFHHN\x18\x0e\x20\x03(\r\
    R\x0bEDHMCHMFHHN\x12\x20\n\x0bAHCGCLKLGIB\x18\x01\x20\x03(\rR\x0bAHCGCLK\
    LGIB\x12.\n\x0bKMMBOINECPE\x18\x06\x20\x03(\x0b2\x0c.BFGDBMGOKLLR\x0bKMM\
    BOINECPE\x12+\n\nrelic_list\x18\x0c\x20\x03(\x0b2\x0c.HMEBDMNACOMR\treli\
    cList\x12\x20\n\x0bFHONFJHLGEK\x18\r\x20\x03(\rR\x0bFHONFJHLGEKb\x06prot\
    o3\
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
            deps.push(super::BFGDBMGOKLL::file_descriptor().clone());
            deps.push(super::HMEBDMNACOM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JDHMJHLBANI::generated_message_descriptor_data());
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