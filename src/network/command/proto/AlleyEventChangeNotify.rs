// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `AlleyEventChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AlleyEventChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlleyEventChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:AlleyEventChangeNotify.NFLDODIABCL)
    pub NFLDODIABCL: ::protobuf::MessageField<super::HEHAOMIAMGL::HEHAOMIAMGL>,
    // @@protoc_insertion_point(field:AlleyEventChangeNotify.HCNLDIBEACA)
    pub HCNLDIBEACA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AlleyEventChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlleyEventChangeNotify {
    fn default() -> &'a AlleyEventChangeNotify {
        <AlleyEventChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl AlleyEventChangeNotify {
    pub fn new() -> AlleyEventChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HEHAOMIAMGL::HEHAOMIAMGL>(
            "NFLDODIABCL",
            |m: &AlleyEventChangeNotify| { &m.NFLDODIABCL },
            |m: &mut AlleyEventChangeNotify| { &mut m.NFLDODIABCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCNLDIBEACA",
            |m: &AlleyEventChangeNotify| { &m.HCNLDIBEACA },
            |m: &mut AlleyEventChangeNotify| { &mut m.HCNLDIBEACA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlleyEventChangeNotify>(
            "AlleyEventChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlleyEventChangeNotify {
    const NAME: &'static str = "AlleyEventChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NFLDODIABCL)?;
                },
                64 => {
                    self.HCNLDIBEACA = is.read_uint32()?;
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
        if let Some(v) = self.NFLDODIABCL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.HCNLDIBEACA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.HCNLDIBEACA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NFLDODIABCL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.HCNLDIBEACA != 0 {
            os.write_uint32(8, self.HCNLDIBEACA)?;
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

    fn new() -> AlleyEventChangeNotify {
        AlleyEventChangeNotify::new()
    }

    fn clear(&mut self) {
        self.NFLDODIABCL.clear();
        self.HCNLDIBEACA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlleyEventChangeNotify {
        static instance: AlleyEventChangeNotify = AlleyEventChangeNotify {
            NFLDODIABCL: ::protobuf::MessageField::none(),
            HCNLDIBEACA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlleyEventChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlleyEventChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlleyEventChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlleyEventChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cAlleyEventChangeNotify.proto\x1a\x11HEHAOMIAMGL.proto\"j\n\x16Alle\
    yEventChangeNotify\x12.\n\x0bNFLDODIABCL\x18\x0e\x20\x01(\x0b2\x0c.HEHAO\
    MIAMGLR\x0bNFLDODIABCL\x12\x20\n\x0bHCNLDIBEACA\x18\x08\x20\x01(\rR\x0bH\
    CNLDIBEACAb\x06proto3\
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
            deps.push(super::HEHAOMIAMGL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AlleyEventChangeNotify::generated_message_descriptor_data());
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
