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

//! Generated file from `GetExpeditionDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetExpeditionDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetExpeditionDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.DLHILDEHBMI)
    pub DLHILDEHBMI: u32,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.AJMJNIMFDOO)
    pub AJMJNIMFDOO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.CLOGAEMKPPM)
    pub CLOGAEMKPPM: ::std::vec::Vec<super::JLHHCIJPOPD::JLHHCIJPOPD>,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.GNJKNMOAEHA)
    pub GNJKNMOAEHA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.LEOJCIHOGPM)
    pub LEOJCIHOGPM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetExpeditionDataScRsp.KLMGFFABHCA)
    pub KLMGFFABHCA: ::std::vec::Vec<super::IFBDJJLGDNJ::IFBDJJLGDNJ>,
    // special fields
    // @@protoc_insertion_point(special_field:GetExpeditionDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetExpeditionDataScRsp {
    fn default() -> &'a GetExpeditionDataScRsp {
        <GetExpeditionDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetExpeditionDataScRsp {
    pub fn new() -> GetExpeditionDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLHILDEHBMI",
            |m: &GetExpeditionDataScRsp| { &m.DLHILDEHBMI },
            |m: &mut GetExpeditionDataScRsp| { &mut m.DLHILDEHBMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AJMJNIMFDOO",
            |m: &GetExpeditionDataScRsp| { &m.AJMJNIMFDOO },
            |m: &mut GetExpeditionDataScRsp| { &mut m.AJMJNIMFDOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CLOGAEMKPPM",
            |m: &GetExpeditionDataScRsp| { &m.CLOGAEMKPPM },
            |m: &mut GetExpeditionDataScRsp| { &mut m.CLOGAEMKPPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GNJKNMOAEHA",
            |m: &GetExpeditionDataScRsp| { &m.GNJKNMOAEHA },
            |m: &mut GetExpeditionDataScRsp| { &mut m.GNJKNMOAEHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetExpeditionDataScRsp| { &m.retcode },
            |m: &mut GetExpeditionDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LEOJCIHOGPM",
            |m: &GetExpeditionDataScRsp| { &m.LEOJCIHOGPM },
            |m: &mut GetExpeditionDataScRsp| { &mut m.LEOJCIHOGPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KLMGFFABHCA",
            |m: &GetExpeditionDataScRsp| { &m.KLMGFFABHCA },
            |m: &mut GetExpeditionDataScRsp| { &mut m.KLMGFFABHCA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetExpeditionDataScRsp>(
            "GetExpeditionDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetExpeditionDataScRsp {
    const NAME: &'static str = "GetExpeditionDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.DLHILDEHBMI = is.read_uint32()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.AJMJNIMFDOO)?;
                },
                40 => {
                    self.AJMJNIMFDOO.push(is.read_uint32()?);
                },
                90 => {
                    self.CLOGAEMKPPM.push(is.read_message()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.GNJKNMOAEHA)?;
                },
                24 => {
                    self.GNJKNMOAEHA.push(is.read_uint32()?);
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.LEOJCIHOGPM)?;
                },
                96 => {
                    self.LEOJCIHOGPM.push(is.read_uint32()?);
                },
                34 => {
                    self.KLMGFFABHCA.push(is.read_message()?);
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
        if self.DLHILDEHBMI != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DLHILDEHBMI);
        }
        for value in &self.AJMJNIMFDOO {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.CLOGAEMKPPM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.GNJKNMOAEHA {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        for value in &self.LEOJCIHOGPM {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        for value in &self.KLMGFFABHCA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DLHILDEHBMI != 0 {
            os.write_uint32(9, self.DLHILDEHBMI)?;
        }
        for v in &self.AJMJNIMFDOO {
            os.write_uint32(5, *v)?;
        };
        for v in &self.CLOGAEMKPPM {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.GNJKNMOAEHA {
            os.write_uint32(3, *v)?;
        };
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        for v in &self.LEOJCIHOGPM {
            os.write_uint32(12, *v)?;
        };
        for v in &self.KLMGFFABHCA {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> GetExpeditionDataScRsp {
        GetExpeditionDataScRsp::new()
    }

    fn clear(&mut self) {
        self.DLHILDEHBMI = 0;
        self.AJMJNIMFDOO.clear();
        self.CLOGAEMKPPM.clear();
        self.GNJKNMOAEHA.clear();
        self.retcode = 0;
        self.LEOJCIHOGPM.clear();
        self.KLMGFFABHCA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetExpeditionDataScRsp {
        static instance: GetExpeditionDataScRsp = GetExpeditionDataScRsp {
            DLHILDEHBMI: 0,
            AJMJNIMFDOO: ::std::vec::Vec::new(),
            CLOGAEMKPPM: ::std::vec::Vec::new(),
            GNJKNMOAEHA: ::std::vec::Vec::new(),
            retcode: 0,
            LEOJCIHOGPM: ::std::vec::Vec::new(),
            KLMGFFABHCA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetExpeditionDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetExpeditionDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetExpeditionDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetExpeditionDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cGetExpeditionDataScRsp.proto\x1a\x11IFBDJJLGDNJ.proto\x1a\x11JLHHC\
    IJPOPD.proto\"\x9a\x02\n\x16GetExpeditionDataScRsp\x12\x20\n\x0bDLHILDEH\
    BMI\x18\t\x20\x01(\rR\x0bDLHILDEHBMI\x12\x20\n\x0bAJMJNIMFDOO\x18\x05\
    \x20\x03(\rR\x0bAJMJNIMFDOO\x12.\n\x0bCLOGAEMKPPM\x18\x0b\x20\x03(\x0b2\
    \x0c.JLHHCIJPOPDR\x0bCLOGAEMKPPM\x12\x20\n\x0bGNJKNMOAEHA\x18\x03\x20\
    \x03(\rR\x0bGNJKNMOAEHA\x12\x18\n\x07retcode\x18\x02\x20\x01(\rR\x07retc\
    ode\x12\x20\n\x0bLEOJCIHOGPM\x18\x0c\x20\x03(\rR\x0bLEOJCIHOGPM\x12.\n\
    \x0bKLMGFFABHCA\x18\x04\x20\x03(\x0b2\x0c.IFBDJJLGDNJR\x0bKLMGFFABHCAb\
    \x06proto3\
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
            deps.push(super::IFBDJJLGDNJ::file_descriptor().clone());
            deps.push(super::JLHHCIJPOPD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetExpeditionDataScRsp::generated_message_descriptor_data());
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
