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

//! Generated file from `IFFOFLNBBCN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IFFOFLNBBCN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IFFOFLNBBCN {
    // message fields
    // @@protoc_insertion_point(field:IFFOFLNBBCN.LCKDHIPGPFB)
    pub LCKDHIPGPFB: i64,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.JGCKOHEPJHP)
    pub JGCKOHEPJHP: ::std::string::String,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.OIHDIEEGLJL)
    pub OIHDIEEGLJL: u32,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.ACKNCEHGNFI)
    pub ACKNCEHGNFI: u32,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.KJFIGLJDJFP)
    pub KJFIGLJDJFP: ::protobuf::MessageField<super::AGMNDLBCMHJ::AGMNDLBCMHJ>,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.KLKAIDFHBAG)
    pub KLKAIDFHBAG: u32,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.HDADMIPMHFH)
    pub HDADMIPMHFH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.JLLEILPNCCA)
    pub JLLEILPNCCA: ::std::string::String,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.FKBDJAIEBGN)
    pub FKBDJAIEBGN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IFFOFLNBBCN.ABKHBIFLEMP)
    pub ABKHBIFLEMP: i64,
    // special fields
    // @@protoc_insertion_point(special_field:IFFOFLNBBCN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IFFOFLNBBCN {
    fn default() -> &'a IFFOFLNBBCN {
        <IFFOFLNBBCN as ::protobuf::Message>::default_instance()
    }
}

impl IFFOFLNBBCN {
    pub fn new() -> IFFOFLNBBCN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCKDHIPGPFB",
            |m: &IFFOFLNBBCN| { &m.LCKDHIPGPFB },
            |m: &mut IFFOFLNBBCN| { &mut m.LCKDHIPGPFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JGCKOHEPJHP",
            |m: &IFFOFLNBBCN| { &m.JGCKOHEPJHP },
            |m: &mut IFFOFLNBBCN| { &mut m.JGCKOHEPJHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OIHDIEEGLJL",
            |m: &IFFOFLNBBCN| { &m.OIHDIEEGLJL },
            |m: &mut IFFOFLNBBCN| { &mut m.OIHDIEEGLJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACKNCEHGNFI",
            |m: &IFFOFLNBBCN| { &m.ACKNCEHGNFI },
            |m: &mut IFFOFLNBBCN| { &mut m.ACKNCEHGNFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AGMNDLBCMHJ::AGMNDLBCMHJ>(
            "KJFIGLJDJFP",
            |m: &IFFOFLNBBCN| { &m.KJFIGLJDJFP },
            |m: &mut IFFOFLNBBCN| { &mut m.KJFIGLJDJFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KLKAIDFHBAG",
            |m: &IFFOFLNBBCN| { &m.KLKAIDFHBAG },
            |m: &mut IFFOFLNBBCN| { &mut m.KLKAIDFHBAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HDADMIPMHFH",
            |m: &IFFOFLNBBCN| { &m.HDADMIPMHFH },
            |m: &mut IFFOFLNBBCN| { &mut m.HDADMIPMHFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLLEILPNCCA",
            |m: &IFFOFLNBBCN| { &m.JLLEILPNCCA },
            |m: &mut IFFOFLNBBCN| { &mut m.JLLEILPNCCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FKBDJAIEBGN",
            |m: &IFFOFLNBBCN| { &m.FKBDJAIEBGN },
            |m: &mut IFFOFLNBBCN| { &mut m.FKBDJAIEBGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ABKHBIFLEMP",
            |m: &IFFOFLNBBCN| { &m.ABKHBIFLEMP },
            |m: &mut IFFOFLNBBCN| { &mut m.ABKHBIFLEMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IFFOFLNBBCN>(
            "IFFOFLNBBCN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IFFOFLNBBCN {
    const NAME: &'static str = "IFFOFLNBBCN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.LCKDHIPGPFB = is.read_int64()?;
                },
                66 => {
                    self.JGCKOHEPJHP = is.read_string()?;
                },
                48 => {
                    self.OIHDIEEGLJL = is.read_uint32()?;
                },
                32 => {
                    self.ACKNCEHGNFI = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KJFIGLJDJFP)?;
                },
                40 => {
                    self.KLKAIDFHBAG = is.read_uint32()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.HDADMIPMHFH)?;
                },
                112 => {
                    self.HDADMIPMHFH.push(is.read_uint32()?);
                },
                90 => {
                    self.JLLEILPNCCA = is.read_string()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.FKBDJAIEBGN)?;
                },
                80 => {
                    self.FKBDJAIEBGN.push(is.read_uint32()?);
                },
                16 => {
                    self.ABKHBIFLEMP = is.read_int64()?;
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
        if self.LCKDHIPGPFB != 0 {
            my_size += ::protobuf::rt::int64_size(13, self.LCKDHIPGPFB);
        }
        if !self.JGCKOHEPJHP.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.JGCKOHEPJHP);
        }
        if self.OIHDIEEGLJL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.OIHDIEEGLJL);
        }
        if self.ACKNCEHGNFI != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ACKNCEHGNFI);
        }
        if let Some(v) = self.KJFIGLJDJFP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KLKAIDFHBAG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KLKAIDFHBAG);
        }
        for value in &self.HDADMIPMHFH {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if !self.JLLEILPNCCA.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.JLLEILPNCCA);
        }
        for value in &self.FKBDJAIEBGN {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.ABKHBIFLEMP != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.ABKHBIFLEMP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LCKDHIPGPFB != 0 {
            os.write_int64(13, self.LCKDHIPGPFB)?;
        }
        if !self.JGCKOHEPJHP.is_empty() {
            os.write_string(8, &self.JGCKOHEPJHP)?;
        }
        if self.OIHDIEEGLJL != 0 {
            os.write_uint32(6, self.OIHDIEEGLJL)?;
        }
        if self.ACKNCEHGNFI != 0 {
            os.write_uint32(4, self.ACKNCEHGNFI)?;
        }
        if let Some(v) = self.KJFIGLJDJFP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.KLKAIDFHBAG != 0 {
            os.write_uint32(5, self.KLKAIDFHBAG)?;
        }
        for v in &self.HDADMIPMHFH {
            os.write_uint32(14, *v)?;
        };
        if !self.JLLEILPNCCA.is_empty() {
            os.write_string(11, &self.JLLEILPNCCA)?;
        }
        for v in &self.FKBDJAIEBGN {
            os.write_uint32(10, *v)?;
        };
        if self.ABKHBIFLEMP != 0 {
            os.write_int64(2, self.ABKHBIFLEMP)?;
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

    fn new() -> IFFOFLNBBCN {
        IFFOFLNBBCN::new()
    }

    fn clear(&mut self) {
        self.LCKDHIPGPFB = 0;
        self.JGCKOHEPJHP.clear();
        self.OIHDIEEGLJL = 0;
        self.ACKNCEHGNFI = 0;
        self.KJFIGLJDJFP.clear();
        self.KLKAIDFHBAG = 0;
        self.HDADMIPMHFH.clear();
        self.JLLEILPNCCA.clear();
        self.FKBDJAIEBGN.clear();
        self.ABKHBIFLEMP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IFFOFLNBBCN {
        static instance: IFFOFLNBBCN = IFFOFLNBBCN {
            LCKDHIPGPFB: 0,
            JGCKOHEPJHP: ::std::string::String::new(),
            OIHDIEEGLJL: 0,
            ACKNCEHGNFI: 0,
            KJFIGLJDJFP: ::protobuf::MessageField::none(),
            KLKAIDFHBAG: 0,
            HDADMIPMHFH: ::std::vec::Vec::new(),
            JLLEILPNCCA: ::std::string::String::new(),
            FKBDJAIEBGN: ::std::vec::Vec::new(),
            ABKHBIFLEMP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IFFOFLNBBCN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IFFOFLNBBCN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IFFOFLNBBCN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IFFOFLNBBCN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IFFOFLNBBCN.proto\x1a\x11AGMNDLBCMHJ.proto\"\xef\x02\n\x0bIFFOFLNB\
    BCN\x12\x20\n\x0bLCKDHIPGPFB\x18\r\x20\x01(\x03R\x0bLCKDHIPGPFB\x12\x20\
    \n\x0bJGCKOHEPJHP\x18\x08\x20\x01(\tR\x0bJGCKOHEPJHP\x12\x20\n\x0bOIHDIE\
    EGLJL\x18\x06\x20\x01(\rR\x0bOIHDIEEGLJL\x12\x20\n\x0bACKNCEHGNFI\x18\
    \x04\x20\x01(\rR\x0bACKNCEHGNFI\x12.\n\x0bKJFIGLJDJFP\x18\x0f\x20\x01(\
    \x0b2\x0c.AGMNDLBCMHJR\x0bKJFIGLJDJFP\x12\x20\n\x0bKLKAIDFHBAG\x18\x05\
    \x20\x01(\rR\x0bKLKAIDFHBAG\x12\x20\n\x0bHDADMIPMHFH\x18\x0e\x20\x03(\rR\
    \x0bHDADMIPMHFH\x12\x20\n\x0bJLLEILPNCCA\x18\x0b\x20\x01(\tR\x0bJLLEILPN\
    CCA\x12\x20\n\x0bFKBDJAIEBGN\x18\n\x20\x03(\rR\x0bFKBDJAIEBGN\x12\x20\n\
    \x0bABKHBIFLEMP\x18\x02\x20\x01(\x03R\x0bABKHBIFLEMPb\x06proto3\
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
            deps.push(super::AGMNDLBCMHJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IFFOFLNBBCN::generated_message_descriptor_data());
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
