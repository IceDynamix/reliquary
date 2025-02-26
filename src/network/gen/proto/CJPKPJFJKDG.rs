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

//! Generated file from `CJPKPJFJKDG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CJPKPJFJKDG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CJPKPJFJKDG {
    // message fields
    // @@protoc_insertion_point(field:CJPKPJFJKDG.FEJGMNNFFLG)
    pub FEJGMNNFFLG: ::protobuf::MessageField<super::PMBNJOONHPA::PMBNJOONHPA>,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.MJJBNDAKOIF)
    pub MJJBNDAKOIF: ::std::string::String,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.OGCKDLKCABG)
    pub OGCKDLKCABG: i64,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.KHNCEDGFPGL)
    pub KHNCEDGFPGL: u32,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.HJEJFNFAMPN)
    pub HJEJFNFAMPN: i64,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.BHMHLPCHKLG)
    pub BHMHLPCHKLG: ::protobuf::MessageField<super::CFCDHLPOOGC::CFCDHLPOOGC>,
    // @@protoc_insertion_point(field:CJPKPJFJKDG.EJJCCGGCNMK)
    pub EJJCCGGCNMK: ::protobuf::MessageField<super::BKFFNNAIODC::BKFFNNAIODC>,
    // special fields
    // @@protoc_insertion_point(special_field:CJPKPJFJKDG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CJPKPJFJKDG {
    fn default() -> &'a CJPKPJFJKDG {
        <CJPKPJFJKDG as ::protobuf::Message>::default_instance()
    }
}

impl CJPKPJFJKDG {
    pub fn new() -> CJPKPJFJKDG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PMBNJOONHPA::PMBNJOONHPA>(
            "FEJGMNNFFLG",
            |m: &CJPKPJFJKDG| { &m.FEJGMNNFFLG },
            |m: &mut CJPKPJFJKDG| { &mut m.FEJGMNNFFLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MJJBNDAKOIF",
            |m: &CJPKPJFJKDG| { &m.MJJBNDAKOIF },
            |m: &mut CJPKPJFJKDG| { &mut m.MJJBNDAKOIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGCKDLKCABG",
            |m: &CJPKPJFJKDG| { &m.OGCKDLKCABG },
            |m: &mut CJPKPJFJKDG| { &mut m.OGCKDLKCABG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHNCEDGFPGL",
            |m: &CJPKPJFJKDG| { &m.KHNCEDGFPGL },
            |m: &mut CJPKPJFJKDG| { &mut m.KHNCEDGFPGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJEJFNFAMPN",
            |m: &CJPKPJFJKDG| { &m.HJEJFNFAMPN },
            |m: &mut CJPKPJFJKDG| { &mut m.HJEJFNFAMPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CFCDHLPOOGC::CFCDHLPOOGC>(
            "BHMHLPCHKLG",
            |m: &CJPKPJFJKDG| { &m.BHMHLPCHKLG },
            |m: &mut CJPKPJFJKDG| { &mut m.BHMHLPCHKLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BKFFNNAIODC::BKFFNNAIODC>(
            "EJJCCGGCNMK",
            |m: &CJPKPJFJKDG| { &m.EJJCCGGCNMK },
            |m: &mut CJPKPJFJKDG| { &mut m.EJJCCGGCNMK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CJPKPJFJKDG>(
            "CJPKPJFJKDG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CJPKPJFJKDG {
    const NAME: &'static str = "CJPKPJFJKDG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FEJGMNNFFLG)?;
                },
                18 => {
                    self.MJJBNDAKOIF = is.read_string()?;
                },
                40 => {
                    self.OGCKDLKCABG = is.read_int64()?;
                },
                48 => {
                    self.KHNCEDGFPGL = is.read_uint32()?;
                },
                104 => {
                    self.HJEJFNFAMPN = is.read_int64()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BHMHLPCHKLG)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EJJCCGGCNMK)?;
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
        if let Some(v) = self.FEJGMNNFFLG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.MJJBNDAKOIF.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.MJJBNDAKOIF);
        }
        if self.OGCKDLKCABG != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.OGCKDLKCABG);
        }
        if self.KHNCEDGFPGL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KHNCEDGFPGL);
        }
        if self.HJEJFNFAMPN != 0 {
            my_size += ::protobuf::rt::int64_size(13, self.HJEJFNFAMPN);
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EJJCCGGCNMK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.FEJGMNNFFLG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if !self.MJJBNDAKOIF.is_empty() {
            os.write_string(2, &self.MJJBNDAKOIF)?;
        }
        if self.OGCKDLKCABG != 0 {
            os.write_int64(5, self.OGCKDLKCABG)?;
        }
        if self.KHNCEDGFPGL != 0 {
            os.write_uint32(6, self.KHNCEDGFPGL)?;
        }
        if self.HJEJFNFAMPN != 0 {
            os.write_int64(13, self.HJEJFNFAMPN)?;
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.EJJCCGGCNMK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> CJPKPJFJKDG {
        CJPKPJFJKDG::new()
    }

    fn clear(&mut self) {
        self.FEJGMNNFFLG.clear();
        self.MJJBNDAKOIF.clear();
        self.OGCKDLKCABG = 0;
        self.KHNCEDGFPGL = 0;
        self.HJEJFNFAMPN = 0;
        self.BHMHLPCHKLG.clear();
        self.EJJCCGGCNMK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CJPKPJFJKDG {
        static instance: CJPKPJFJKDG = CJPKPJFJKDG {
            FEJGMNNFFLG: ::protobuf::MessageField::none(),
            MJJBNDAKOIF: ::std::string::String::new(),
            OGCKDLKCABG: 0,
            KHNCEDGFPGL: 0,
            HJEJFNFAMPN: 0,
            BHMHLPCHKLG: ::protobuf::MessageField::none(),
            EJJCCGGCNMK: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CJPKPJFJKDG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CJPKPJFJKDG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CJPKPJFJKDG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CJPKPJFJKDG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CJPKPJFJKDG.proto\x1a\x11BKFFNNAIODC.proto\x1a\x11CFCDHLPOOGC.prot\
    o\x1a\x11PMBNJOONHPA.proto\"\xa5\x02\n\x0bCJPKPJFJKDG\x12.\n\x0bFEJGMNNF\
    FLG\x18\x0f\x20\x01(\x0b2\x0c.PMBNJOONHPAR\x0bFEJGMNNFFLG\x12\x20\n\x0bM\
    JJBNDAKOIF\x18\x02\x20\x01(\tR\x0bMJJBNDAKOIF\x12\x20\n\x0bOGCKDLKCABG\
    \x18\x05\x20\x01(\x03R\x0bOGCKDLKCABG\x12\x20\n\x0bKHNCEDGFPGL\x18\x06\
    \x20\x01(\rR\x0bKHNCEDGFPGL\x12\x20\n\x0bHJEJFNFAMPN\x18\r\x20\x01(\x03R\
    \x0bHJEJFNFAMPN\x12.\n\x0bBHMHLPCHKLG\x18\x0e\x20\x01(\x0b2\x0c.CFCDHLPO\
    OGCR\x0bBHMHLPCHKLG\x12.\n\x0bEJJCCGGCNMK\x18\x0c\x20\x01(\x0b2\x0c.BKFF\
    NNAIODCR\x0bEJJCCGGCNMKb\x06proto3\
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
            deps.push(super::BKFFNNAIODC::file_descriptor().clone());
            deps.push(super::CFCDHLPOOGC::file_descriptor().clone());
            deps.push(super::PMBNJOONHPA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CJPKPJFJKDG::generated_message_descriptor_data());
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
