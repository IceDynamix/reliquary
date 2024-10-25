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

//! Generated file from `GetGameStateServiceConfigScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetGameStateServiceConfigScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetGameStateServiceConfigScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetGameStateServiceConfigScRsp.KKGILJCIPJO)
    pub KKGILJCIPJO: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:GetGameStateServiceConfigScRsp.IEKAEGAJCEG)
    pub IEKAEGAJCEG: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:GetGameStateServiceConfigScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetGameStateServiceConfigScRsp.NLKANAKEMEJ)
    pub NLKANAKEMEJ: ::std::vec::Vec<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:GetGameStateServiceConfigScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetGameStateServiceConfigScRsp {
    fn default() -> &'a GetGameStateServiceConfigScRsp {
        <GetGameStateServiceConfigScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetGameStateServiceConfigScRsp {
    pub fn new() -> GetGameStateServiceConfigScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KKGILJCIPJO",
            |m: &GetGameStateServiceConfigScRsp| { &m.KKGILJCIPJO },
            |m: &mut GetGameStateServiceConfigScRsp| { &mut m.KKGILJCIPJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IEKAEGAJCEG",
            |m: &GetGameStateServiceConfigScRsp| { &m.IEKAEGAJCEG },
            |m: &mut GetGameStateServiceConfigScRsp| { &mut m.IEKAEGAJCEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetGameStateServiceConfigScRsp| { &m.retcode },
            |m: &mut GetGameStateServiceConfigScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NLKANAKEMEJ",
            |m: &GetGameStateServiceConfigScRsp| { &m.NLKANAKEMEJ },
            |m: &mut GetGameStateServiceConfigScRsp| { &mut m.NLKANAKEMEJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetGameStateServiceConfigScRsp>(
            "GetGameStateServiceConfigScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetGameStateServiceConfigScRsp {
    const NAME: &'static str = "GetGameStateServiceConfigScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.KKGILJCIPJO.push(is.read_string()?);
                },
                10 => {
                    self.IEKAEGAJCEG.push(is.read_string()?);
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                106 => {
                    self.NLKANAKEMEJ.push(is.read_string()?);
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
        for value in &self.KKGILJCIPJO {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        for value in &self.IEKAEGAJCEG {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        for value in &self.NLKANAKEMEJ {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KKGILJCIPJO {
            os.write_string(11, &v)?;
        };
        for v in &self.IEKAEGAJCEG {
            os.write_string(1, &v)?;
        };
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        for v in &self.NLKANAKEMEJ {
            os.write_string(13, &v)?;
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

    fn new() -> GetGameStateServiceConfigScRsp {
        GetGameStateServiceConfigScRsp::new()
    }

    fn clear(&mut self) {
        self.KKGILJCIPJO.clear();
        self.IEKAEGAJCEG.clear();
        self.retcode = 0;
        self.NLKANAKEMEJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetGameStateServiceConfigScRsp {
        static instance: GetGameStateServiceConfigScRsp = GetGameStateServiceConfigScRsp {
            KKGILJCIPJO: ::std::vec::Vec::new(),
            IEKAEGAJCEG: ::std::vec::Vec::new(),
            retcode: 0,
            NLKANAKEMEJ: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetGameStateServiceConfigScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetGameStateServiceConfigScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetGameStateServiceConfigScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGameStateServiceConfigScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$GetGameStateServiceConfigScRsp.proto\"\xa0\x01\n\x1eGetGameStateServi\
    ceConfigScRsp\x12\x20\n\x0bKKGILJCIPJO\x18\x0b\x20\x03(\tR\x0bKKGILJCIPJ\
    O\x12\x20\n\x0bIEKAEGAJCEG\x18\x01\x20\x03(\tR\x0bIEKAEGAJCEG\x12\x18\n\
    \x07retcode\x18\x02\x20\x01(\rR\x07retcode\x12\x20\n\x0bNLKANAKEMEJ\x18\
    \r\x20\x03(\tR\x0bNLKANAKEMEJb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetGameStateServiceConfigScRsp::generated_message_descriptor_data());
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
