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

//! Generated file from `StartChallengeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartChallengeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartChallengeScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartChallengeScRsp.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::KHLGBOHOGPD::KHLGBOHOGPD>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.EMDECAJPAPM)
    pub EMDECAJPAPM: ::protobuf::MessageField<super::FHGPCKGFGAO::FHGPCKGFGAO>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.CKLHHOLMBOO)
    pub CKLHHOLMBOO: ::std::vec::Vec<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.CLPCCICOGCE)
    pub CLPCCICOGCE: ::protobuf::MessageField<super::NDACGJLONGF::NDACGJLONGF>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartChallengeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartChallengeScRsp {
    fn default() -> &'a StartChallengeScRsp {
        <StartChallengeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartChallengeScRsp {
    pub fn new() -> StartChallengeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KHLGBOHOGPD::KHLGBOHOGPD>(
            "HLDLDAPNILF",
            |m: &StartChallengeScRsp| { &m.HLDLDAPNILF },
            |m: &mut StartChallengeScRsp| { &mut m.HLDLDAPNILF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FHGPCKGFGAO::FHGPCKGFGAO>(
            "EMDECAJPAPM",
            |m: &StartChallengeScRsp| { &m.EMDECAJPAPM },
            |m: &mut StartChallengeScRsp| { &mut m.EMDECAJPAPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CKLHHOLMBOO",
            |m: &StartChallengeScRsp| { &m.CKLHHOLMBOO },
            |m: &mut StartChallengeScRsp| { &mut m.CKLHHOLMBOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NDACGJLONGF::NDACGJLONGF>(
            "CLPCCICOGCE",
            |m: &StartChallengeScRsp| { &m.CLPCCICOGCE },
            |m: &mut StartChallengeScRsp| { &mut m.CLPCCICOGCE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartChallengeScRsp| { &m.retcode },
            |m: &mut StartChallengeScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartChallengeScRsp>(
            "StartChallengeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartChallengeScRsp {
    const NAME: &'static str = "StartChallengeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EMDECAJPAPM)?;
                },
                98 => {
                    self.CKLHHOLMBOO.push(is.read_message()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CLPCCICOGCE)?;
                },
                24 => {
                    self.retcode = is.read_uint32()?;
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
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.CKLHHOLMBOO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.CLPCCICOGCE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.CKLHHOLMBOO {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if let Some(v) = self.CLPCCICOGCE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
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

    fn new() -> StartChallengeScRsp {
        StartChallengeScRsp::new()
    }

    fn clear(&mut self) {
        self.HLDLDAPNILF.clear();
        self.EMDECAJPAPM.clear();
        self.CKLHHOLMBOO.clear();
        self.CLPCCICOGCE.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartChallengeScRsp {
        static instance: StartChallengeScRsp = StartChallengeScRsp {
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            EMDECAJPAPM: ::protobuf::MessageField::none(),
            CKLHHOLMBOO: ::std::vec::Vec::new(),
            CLPCCICOGCE: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartChallengeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartChallengeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartChallengeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartChallengeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19StartChallengeScRsp.proto\x1a\x11FHGPCKGFGAO.proto\x1a\x11FJPJJEIJ\
    LLP.proto\x1a\x11KHLGBOHOGPD.proto\x1a\x11NDACGJLONGF.proto\"\xef\x01\n\
    \x13StartChallengeScRsp\x12.\n\x0bHLDLDAPNILF\x18\x08\x20\x01(\x0b2\x0c.\
    KHLGBOHOGPDR\x0bHLDLDAPNILF\x12.\n\x0bEMDECAJPAPM\x18\x02\x20\x01(\x0b2\
    \x0c.FHGPCKGFGAOR\x0bEMDECAJPAPM\x12.\n\x0bCKLHHOLMBOO\x18\x0c\x20\x03(\
    \x0b2\x0c.FJPJJEIJLLPR\x0bCKLHHOLMBOO\x12.\n\x0bCLPCCICOGCE\x18\x06\x20\
    \x01(\x0b2\x0c.NDACGJLONGFR\x0bCLPCCICOGCE\x12\x18\n\x07retcode\x18\x03\
    \x20\x01(\rR\x07retcodeb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::FHGPCKGFGAO::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            deps.push(super::KHLGBOHOGPD::file_descriptor().clone());
            deps.push(super::NDACGJLONGF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartChallengeScRsp::generated_message_descriptor_data());
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
