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

//! Generated file from `SubmitOfferingItemScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SubmitOfferingItemScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SubmitOfferingItemScRsp {
    // message fields
    // @@protoc_insertion_point(field:SubmitOfferingItemScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SubmitOfferingItemScRsp.PPJDAPBPOAP)
    pub PPJDAPBPOAP: ::protobuf::MessageField<super::HKMKCHNPKDC::HKMKCHNPKDC>,
    // special fields
    // @@protoc_insertion_point(special_field:SubmitOfferingItemScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SubmitOfferingItemScRsp {
    fn default() -> &'a SubmitOfferingItemScRsp {
        <SubmitOfferingItemScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SubmitOfferingItemScRsp {
    pub fn new() -> SubmitOfferingItemScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SubmitOfferingItemScRsp| { &m.retcode },
            |m: &mut SubmitOfferingItemScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HKMKCHNPKDC::HKMKCHNPKDC>(
            "PPJDAPBPOAP",
            |m: &SubmitOfferingItemScRsp| { &m.PPJDAPBPOAP },
            |m: &mut SubmitOfferingItemScRsp| { &mut m.PPJDAPBPOAP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SubmitOfferingItemScRsp>(
            "SubmitOfferingItemScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SubmitOfferingItemScRsp {
    const NAME: &'static str = "SubmitOfferingItemScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.retcode = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PPJDAPBPOAP)?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        if let Some(v) = self.PPJDAPBPOAP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
        }
        if let Some(v) = self.PPJDAPBPOAP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> SubmitOfferingItemScRsp {
        SubmitOfferingItemScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.PPJDAPBPOAP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SubmitOfferingItemScRsp {
        static instance: SubmitOfferingItemScRsp = SubmitOfferingItemScRsp {
            retcode: 0,
            PPJDAPBPOAP: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SubmitOfferingItemScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SubmitOfferingItemScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SubmitOfferingItemScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubmitOfferingItemScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dSubmitOfferingItemScRsp.proto\x1a\x11HKMKCHNPKDC.proto\"c\n\x17Sub\
    mitOfferingItemScRsp\x12\x18\n\x07retcode\x18\x0f\x20\x01(\rR\x07retcode\
    \x12.\n\x0bPPJDAPBPOAP\x18\x0e\x20\x01(\x0b2\x0c.HKMKCHNPKDCR\x0bPPJDAPB\
    POAPb\x06proto3\
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
            deps.push(super::HKMKCHNPKDC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SubmitOfferingItemScRsp::generated_message_descriptor_data());
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
