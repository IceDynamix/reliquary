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

//! Generated file from `MNFFJHFPKPJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MNFFJHFPKPJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MNFFJHFPKPJ {
    // message fields
    // @@protoc_insertion_point(field:MNFFJHFPKPJ.JEHOPBNJPPL)
    pub JEHOPBNJPPL: ::std::vec::Vec<super::KMFBLAGAOEE::KMFBLAGAOEE>,
    // @@protoc_insertion_point(field:MNFFJHFPKPJ.LOIAMEJHCIL)
    pub LOIAMEJHCIL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MNFFJHFPKPJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MNFFJHFPKPJ {
    fn default() -> &'a MNFFJHFPKPJ {
        <MNFFJHFPKPJ as ::protobuf::Message>::default_instance()
    }
}

impl MNFFJHFPKPJ {
    pub fn new() -> MNFFJHFPKPJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JEHOPBNJPPL",
            |m: &MNFFJHFPKPJ| { &m.JEHOPBNJPPL },
            |m: &mut MNFFJHFPKPJ| { &mut m.JEHOPBNJPPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOIAMEJHCIL",
            |m: &MNFFJHFPKPJ| { &m.LOIAMEJHCIL },
            |m: &mut MNFFJHFPKPJ| { &mut m.LOIAMEJHCIL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MNFFJHFPKPJ>(
            "MNFFJHFPKPJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MNFFJHFPKPJ {
    const NAME: &'static str = "MNFFJHFPKPJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.JEHOPBNJPPL.push(is.read_message()?);
                },
                56 => {
                    self.LOIAMEJHCIL = is.read_uint32()?;
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
        for value in &self.JEHOPBNJPPL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LOIAMEJHCIL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LOIAMEJHCIL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.JEHOPBNJPPL {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.LOIAMEJHCIL != 0 {
            os.write_uint32(7, self.LOIAMEJHCIL)?;
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

    fn new() -> MNFFJHFPKPJ {
        MNFFJHFPKPJ::new()
    }

    fn clear(&mut self) {
        self.JEHOPBNJPPL.clear();
        self.LOIAMEJHCIL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MNFFJHFPKPJ {
        static instance: MNFFJHFPKPJ = MNFFJHFPKPJ {
            JEHOPBNJPPL: ::std::vec::Vec::new(),
            LOIAMEJHCIL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MNFFJHFPKPJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MNFFJHFPKPJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MNFFJHFPKPJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MNFFJHFPKPJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MNFFJHFPKPJ.proto\x1a\x11KMFBLAGAOEE.proto\"_\n\x0bMNFFJHFPKPJ\x12\
    .\n\x0bJEHOPBNJPPL\x18\x05\x20\x03(\x0b2\x0c.KMFBLAGAOEER\x0bJEHOPBNJPPL\
    \x12\x20\n\x0bLOIAMEJHCIL\x18\x07\x20\x01(\rR\x0bLOIAMEJHCILb\x06proto3\
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
            deps.push(super::KMFBLAGAOEE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MNFFJHFPKPJ::generated_message_descriptor_data());
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
