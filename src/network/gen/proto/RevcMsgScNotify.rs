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

//! Generated file from `RevcMsgScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RevcMsgScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RevcMsgScNotify {
    // message fields
    // @@protoc_insertion_point(field:RevcMsgScNotify.HPPKGALLDBH)
    pub HPPKGALLDBH: ::std::string::String,
    // @@protoc_insertion_point(field:RevcMsgScNotify.PMOMEPOFCAA)
    pub PMOMEPOFCAA: ::protobuf::EnumOrUnknown<super::ChatType::ChatType>,
    // @@protoc_insertion_point(field:RevcMsgScNotify.COMDILNOLGF)
    pub COMDILNOLGF: u32,
    // @@protoc_insertion_point(field:RevcMsgScNotify.HNDLNGBBDNJ)
    pub HNDLNGBBDNJ: u32,
    // @@protoc_insertion_point(field:RevcMsgScNotify.CALKNOOBLFB)
    pub CALKNOOBLFB: ::protobuf::EnumOrUnknown<super::MsgType::MsgType>,
    // @@protoc_insertion_point(field:RevcMsgScNotify.OBHOABOLNIH)
    pub OBHOABOLNIH: u32,
    // @@protoc_insertion_point(field:RevcMsgScNotify.BNABNCCMILM)
    pub BNABNCCMILM: ::protobuf::MessageField<super::LFCKPPDLIJI::LFCKPPDLIJI>,
    // special fields
    // @@protoc_insertion_point(special_field:RevcMsgScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RevcMsgScNotify {
    fn default() -> &'a RevcMsgScNotify {
        <RevcMsgScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RevcMsgScNotify {
    pub fn new() -> RevcMsgScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPPKGALLDBH",
            |m: &RevcMsgScNotify| { &m.HPPKGALLDBH },
            |m: &mut RevcMsgScNotify| { &mut m.HPPKGALLDBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMOMEPOFCAA",
            |m: &RevcMsgScNotify| { &m.PMOMEPOFCAA },
            |m: &mut RevcMsgScNotify| { &mut m.PMOMEPOFCAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COMDILNOLGF",
            |m: &RevcMsgScNotify| { &m.COMDILNOLGF },
            |m: &mut RevcMsgScNotify| { &mut m.COMDILNOLGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNDLNGBBDNJ",
            |m: &RevcMsgScNotify| { &m.HNDLNGBBDNJ },
            |m: &mut RevcMsgScNotify| { &mut m.HNDLNGBBDNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CALKNOOBLFB",
            |m: &RevcMsgScNotify| { &m.CALKNOOBLFB },
            |m: &mut RevcMsgScNotify| { &mut m.CALKNOOBLFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBHOABOLNIH",
            |m: &RevcMsgScNotify| { &m.OBHOABOLNIH },
            |m: &mut RevcMsgScNotify| { &mut m.OBHOABOLNIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LFCKPPDLIJI::LFCKPPDLIJI>(
            "BNABNCCMILM",
            |m: &RevcMsgScNotify| { &m.BNABNCCMILM },
            |m: &mut RevcMsgScNotify| { &mut m.BNABNCCMILM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RevcMsgScNotify>(
            "RevcMsgScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RevcMsgScNotify {
    const NAME: &'static str = "RevcMsgScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.HPPKGALLDBH = is.read_string()?;
                },
                88 => {
                    self.PMOMEPOFCAA = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.COMDILNOLGF = is.read_uint32()?;
                },
                40 => {
                    self.HNDLNGBBDNJ = is.read_uint32()?;
                },
                24 => {
                    self.CALKNOOBLFB = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.OBHOABOLNIH = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BNABNCCMILM)?;
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
        if !self.HPPKGALLDBH.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.HPPKGALLDBH);
        }
        if self.PMOMEPOFCAA != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.PMOMEPOFCAA.value());
        }
        if self.COMDILNOLGF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.COMDILNOLGF);
        }
        if self.HNDLNGBBDNJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.HNDLNGBBDNJ);
        }
        if self.CALKNOOBLFB != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.CALKNOOBLFB.value());
        }
        if self.OBHOABOLNIH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.OBHOABOLNIH);
        }
        if let Some(v) = self.BNABNCCMILM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.HPPKGALLDBH.is_empty() {
            os.write_string(2, &self.HPPKGALLDBH)?;
        }
        if self.PMOMEPOFCAA != ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.PMOMEPOFCAA))?;
        }
        if self.COMDILNOLGF != 0 {
            os.write_uint32(10, self.COMDILNOLGF)?;
        }
        if self.HNDLNGBBDNJ != 0 {
            os.write_uint32(5, self.HNDLNGBBDNJ)?;
        }
        if self.CALKNOOBLFB != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.CALKNOOBLFB))?;
        }
        if self.OBHOABOLNIH != 0 {
            os.write_uint32(4, self.OBHOABOLNIH)?;
        }
        if let Some(v) = self.BNABNCCMILM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> RevcMsgScNotify {
        RevcMsgScNotify::new()
    }

    fn clear(&mut self) {
        self.HPPKGALLDBH.clear();
        self.PMOMEPOFCAA = ::protobuf::EnumOrUnknown::new(super::ChatType::ChatType::CHAT_TYPE_NONE);
        self.COMDILNOLGF = 0;
        self.HNDLNGBBDNJ = 0;
        self.CALKNOOBLFB = ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE);
        self.OBHOABOLNIH = 0;
        self.BNABNCCMILM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RevcMsgScNotify {
        static instance: RevcMsgScNotify = RevcMsgScNotify {
            HPPKGALLDBH: ::std::string::String::new(),
            PMOMEPOFCAA: ::protobuf::EnumOrUnknown::from_i32(0),
            COMDILNOLGF: 0,
            HNDLNGBBDNJ: 0,
            CALKNOOBLFB: ::protobuf::EnumOrUnknown::from_i32(0),
            OBHOABOLNIH: 0,
            BNABNCCMILM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RevcMsgScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RevcMsgScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RevcMsgScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RevcMsgScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RevcMsgScNotify.proto\x1a\x0eChatType.proto\x1a\x11LFCKPPDLIJI.pro\
    to\x1a\rMsgType.proto\"\xa2\x02\n\x0fRevcMsgScNotify\x12\x20\n\x0bHPPKGA\
    LLDBH\x18\x02\x20\x01(\tR\x0bHPPKGALLDBH\x12+\n\x0bPMOMEPOFCAA\x18\x0b\
    \x20\x01(\x0e2\t.ChatTypeR\x0bPMOMEPOFCAA\x12\x20\n\x0bCOMDILNOLGF\x18\n\
    \x20\x01(\rR\x0bCOMDILNOLGF\x12\x20\n\x0bHNDLNGBBDNJ\x18\x05\x20\x01(\rR\
    \x0bHNDLNGBBDNJ\x12*\n\x0bCALKNOOBLFB\x18\x03\x20\x01(\x0e2\x08.MsgTypeR\
    \x0bCALKNOOBLFB\x12\x20\n\x0bOBHOABOLNIH\x18\x04\x20\x01(\rR\x0bOBHOABOL\
    NIH\x12.\n\x0bBNABNCCMILM\x18\x01\x20\x01(\x0b2\x0c.LFCKPPDLIJIR\x0bBNAB\
    NCCMILMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::ChatType::file_descriptor().clone());
            deps.push(super::LFCKPPDLIJI::file_descriptor().clone());
            deps.push(super::MsgType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RevcMsgScNotify::generated_message_descriptor_data());
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
