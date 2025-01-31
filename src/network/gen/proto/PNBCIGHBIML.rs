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

//! Generated file from `PNBCIGHBIML.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PNBCIGHBIML)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PNBCIGHBIML {
    // message fields
    // @@protoc_insertion_point(field:PNBCIGHBIML.IIECAOOIJBN)
    pub IIECAOOIJBN: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:PNBCIGHBIML.ANJDBOGMCIG)
    pub ANJDBOGMCIG: ::protobuf::MessageField<super::GECGIJIIHDI::GECGIJIIHDI>,
    // @@protoc_insertion_point(field:PNBCIGHBIML.ELOEFHGKDFG)
    pub ELOEFHGKDFG: ::std::vec::Vec<super::GECGIJIIHDI::GECGIJIIHDI>,
    // special fields
    // @@protoc_insertion_point(special_field:PNBCIGHBIML.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PNBCIGHBIML {
    fn default() -> &'a PNBCIGHBIML {
        <PNBCIGHBIML as ::protobuf::Message>::default_instance()
    }
}

impl PNBCIGHBIML {
    pub fn new() -> PNBCIGHBIML {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIECAOOIJBN",
            |m: &PNBCIGHBIML| { &m.IIECAOOIJBN },
            |m: &mut PNBCIGHBIML| { &mut m.IIECAOOIJBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GECGIJIIHDI::GECGIJIIHDI>(
            "ANJDBOGMCIG",
            |m: &PNBCIGHBIML| { &m.ANJDBOGMCIG },
            |m: &mut PNBCIGHBIML| { &mut m.ANJDBOGMCIG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ELOEFHGKDFG",
            |m: &PNBCIGHBIML| { &m.ELOEFHGKDFG },
            |m: &mut PNBCIGHBIML| { &mut m.ELOEFHGKDFG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PNBCIGHBIML>(
            "PNBCIGHBIML",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PNBCIGHBIML {
    const NAME: &'static str = "PNBCIGHBIML";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.IIECAOOIJBN = is.read_bytes()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ANJDBOGMCIG)?;
                },
                26 => {
                    self.ELOEFHGKDFG.push(is.read_message()?);
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
        if !self.IIECAOOIJBN.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.IIECAOOIJBN);
        }
        if let Some(v) = self.ANJDBOGMCIG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.ELOEFHGKDFG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.IIECAOOIJBN.is_empty() {
            os.write_bytes(1, &self.IIECAOOIJBN)?;
        }
        if let Some(v) = self.ANJDBOGMCIG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.ELOEFHGKDFG {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> PNBCIGHBIML {
        PNBCIGHBIML::new()
    }

    fn clear(&mut self) {
        self.IIECAOOIJBN.clear();
        self.ANJDBOGMCIG.clear();
        self.ELOEFHGKDFG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PNBCIGHBIML {
        static instance: PNBCIGHBIML = PNBCIGHBIML {
            IIECAOOIJBN: ::std::vec::Vec::new(),
            ANJDBOGMCIG: ::protobuf::MessageField::none(),
            ELOEFHGKDFG: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PNBCIGHBIML {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PNBCIGHBIML").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PNBCIGHBIML {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PNBCIGHBIML {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PNBCIGHBIML.proto\x1a\x11GECGIJIIHDI.proto\"\x8f\x01\n\x0bPNBCIGHB\
    IML\x12\x20\n\x0bIIECAOOIJBN\x18\x01\x20\x01(\x0cR\x0bIIECAOOIJBN\x12.\n\
    \x0bANJDBOGMCIG\x18\x02\x20\x01(\x0b2\x0c.GECGIJIIHDIR\x0bANJDBOGMCIG\
    \x12.\n\x0bELOEFHGKDFG\x18\x03\x20\x03(\x0b2\x0c.GECGIJIIHDIR\x0bELOEFHG\
    KDFGb\x06proto3\
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
            deps.push(super::GECGIJIIHDI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PNBCIGHBIML::generated_message_descriptor_data());
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
