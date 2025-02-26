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

//! Generated file from `GetReplayTokenCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetReplayTokenCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetReplayTokenCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.IDMMJJAJENK)
    pub IDMMJJAJENK: ::protobuf::EnumOrUnknown<super::OJEADLMKHBO::OJEADLMKHBO>,
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.AFEHLMFIBMD)
    pub AFEHLMFIBMD: u32,
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.KIHBIGPFKKN)
    pub KIHBIGPFKKN: ::std::string::String,
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.CFONLBPOABP)
    pub CFONLBPOABP: u32,
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.CMPBKBBKAOA)
    pub CMPBKBBKAOA: u32,
    // @@protoc_insertion_point(field:GetReplayTokenCsReq.BBEMIDHMNLM)
    pub BBEMIDHMNLM: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:GetReplayTokenCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetReplayTokenCsReq {
    fn default() -> &'a GetReplayTokenCsReq {
        <GetReplayTokenCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetReplayTokenCsReq {
    pub fn new() -> GetReplayTokenCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDMMJJAJENK",
            |m: &GetReplayTokenCsReq| { &m.IDMMJJAJENK },
            |m: &mut GetReplayTokenCsReq| { &mut m.IDMMJJAJENK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFEHLMFIBMD",
            |m: &GetReplayTokenCsReq| { &m.AFEHLMFIBMD },
            |m: &mut GetReplayTokenCsReq| { &mut m.AFEHLMFIBMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIHBIGPFKKN",
            |m: &GetReplayTokenCsReq| { &m.KIHBIGPFKKN },
            |m: &mut GetReplayTokenCsReq| { &mut m.KIHBIGPFKKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFONLBPOABP",
            |m: &GetReplayTokenCsReq| { &m.CFONLBPOABP },
            |m: &mut GetReplayTokenCsReq| { &mut m.CFONLBPOABP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMPBKBBKAOA",
            |m: &GetReplayTokenCsReq| { &m.CMPBKBBKAOA },
            |m: &mut GetReplayTokenCsReq| { &mut m.CMPBKBBKAOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBEMIDHMNLM",
            |m: &GetReplayTokenCsReq| { &m.BBEMIDHMNLM },
            |m: &mut GetReplayTokenCsReq| { &mut m.BBEMIDHMNLM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetReplayTokenCsReq>(
            "GetReplayTokenCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetReplayTokenCsReq {
    const NAME: &'static str = "GetReplayTokenCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.IDMMJJAJENK = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.AFEHLMFIBMD = is.read_uint32()?;
                },
                50 => {
                    self.KIHBIGPFKKN = is.read_string()?;
                },
                96 => {
                    self.CFONLBPOABP = is.read_uint32()?;
                },
                16 => {
                    self.CMPBKBBKAOA = is.read_uint32()?;
                },
                114 => {
                    self.BBEMIDHMNLM = is.read_string()?;
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
        if self.IDMMJJAJENK != ::protobuf::EnumOrUnknown::new(super::OJEADLMKHBO::OJEADLMKHBO::REPLAY_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.IDMMJJAJENK.value());
        }
        if self.AFEHLMFIBMD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.AFEHLMFIBMD);
        }
        if !self.KIHBIGPFKKN.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.KIHBIGPFKKN);
        }
        if self.CFONLBPOABP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.CFONLBPOABP);
        }
        if self.CMPBKBBKAOA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CMPBKBBKAOA);
        }
        if !self.BBEMIDHMNLM.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.BBEMIDHMNLM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IDMMJJAJENK != ::protobuf::EnumOrUnknown::new(super::OJEADLMKHBO::OJEADLMKHBO::REPLAY_TYPE_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.IDMMJJAJENK))?;
        }
        if self.AFEHLMFIBMD != 0 {
            os.write_uint32(9, self.AFEHLMFIBMD)?;
        }
        if !self.KIHBIGPFKKN.is_empty() {
            os.write_string(6, &self.KIHBIGPFKKN)?;
        }
        if self.CFONLBPOABP != 0 {
            os.write_uint32(12, self.CFONLBPOABP)?;
        }
        if self.CMPBKBBKAOA != 0 {
            os.write_uint32(2, self.CMPBKBBKAOA)?;
        }
        if !self.BBEMIDHMNLM.is_empty() {
            os.write_string(14, &self.BBEMIDHMNLM)?;
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

    fn new() -> GetReplayTokenCsReq {
        GetReplayTokenCsReq::new()
    }

    fn clear(&mut self) {
        self.IDMMJJAJENK = ::protobuf::EnumOrUnknown::new(super::OJEADLMKHBO::OJEADLMKHBO::REPLAY_TYPE_NONE);
        self.AFEHLMFIBMD = 0;
        self.KIHBIGPFKKN.clear();
        self.CFONLBPOABP = 0;
        self.CMPBKBBKAOA = 0;
        self.BBEMIDHMNLM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetReplayTokenCsReq {
        static instance: GetReplayTokenCsReq = GetReplayTokenCsReq {
            IDMMJJAJENK: ::protobuf::EnumOrUnknown::from_i32(0),
            AFEHLMFIBMD: 0,
            KIHBIGPFKKN: ::std::string::String::new(),
            CFONLBPOABP: 0,
            CMPBKBBKAOA: 0,
            BBEMIDHMNLM: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetReplayTokenCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetReplayTokenCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetReplayTokenCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplayTokenCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19GetReplayTokenCsReq.proto\x1a\x11OJEADLMKHBO.proto\"\xef\x01\n\x13\
    GetReplayTokenCsReq\x12.\n\x0bIDMMJJAJENK\x18\x05\x20\x01(\x0e2\x0c.OJEA\
    DLMKHBOR\x0bIDMMJJAJENK\x12\x20\n\x0bAFEHLMFIBMD\x18\t\x20\x01(\rR\x0bAF\
    EHLMFIBMD\x12\x20\n\x0bKIHBIGPFKKN\x18\x06\x20\x01(\tR\x0bKIHBIGPFKKN\
    \x12\x20\n\x0bCFONLBPOABP\x18\x0c\x20\x01(\rR\x0bCFONLBPOABP\x12\x20\n\
    \x0bCMPBKBBKAOA\x18\x02\x20\x01(\rR\x0bCMPBKBBKAOA\x12\x20\n\x0bBBEMIDHM\
    NLM\x18\x0e\x20\x01(\tR\x0bBBEMIDHMNLMb\x06proto3\
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
            deps.push(super::OJEADLMKHBO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetReplayTokenCsReq::generated_message_descriptor_data());
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
