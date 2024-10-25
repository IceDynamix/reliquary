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

//! Generated file from `CHIBEOEPPML.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CHIBEOEPPML)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHIBEOEPPML {
    // message fields
    // @@protoc_insertion_point(field:CHIBEOEPPML.JHEKFKEENLA)
    pub JHEKFKEENLA: ::protobuf::MessageField<super::DGIIKCODGKN::DGIIKCODGKN>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.GIKHHHNGEHC)
    pub GIKHHHNGEHC: ::protobuf::MessageField<super::ABCOKNJADLI::ABCOKNJADLI>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.FLMJGOGKGGE)
    pub FLMJGOGKGGE: ::std::vec::Vec<super::DKABNLJDNPK::DKABNLJDNPK>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.LDKMCDMDLKL)
    pub LDKMCDMDLKL: ::protobuf::MessageField<super::IJMLICAGJNO::IJMLICAGJNO>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.GIEACPCMKPN)
    pub GIEACPCMKPN: ::protobuf::MessageField<super::DDOJFOMJAGF::DDOJFOMJAGF>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.EGNGHDKOAAC)
    pub EGNGHDKOAAC: ::std::vec::Vec<super::CNKEFABFHGG::CNKEFABFHGG>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.PAINLBMGDBC)
    pub PAINLBMGDBC: ::std::vec::Vec<super::JBJDAIHGHKN::JBJDAIHGHKN>,
    // @@protoc_insertion_point(field:CHIBEOEPPML.BPDEFCDBKBA)
    pub BPDEFCDBKBA: ::protobuf::MessageField<super::LMKDEKEIDFB::LMKDEKEIDFB>,
    // special fields
    // @@protoc_insertion_point(special_field:CHIBEOEPPML.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHIBEOEPPML {
    fn default() -> &'a CHIBEOEPPML {
        <CHIBEOEPPML as ::protobuf::Message>::default_instance()
    }
}

impl CHIBEOEPPML {
    pub fn new() -> CHIBEOEPPML {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DGIIKCODGKN::DGIIKCODGKN>(
            "JHEKFKEENLA",
            |m: &CHIBEOEPPML| { &m.JHEKFKEENLA },
            |m: &mut CHIBEOEPPML| { &mut m.JHEKFKEENLA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ABCOKNJADLI::ABCOKNJADLI>(
            "GIKHHHNGEHC",
            |m: &CHIBEOEPPML| { &m.GIKHHHNGEHC },
            |m: &mut CHIBEOEPPML| { &mut m.GIKHHHNGEHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FLMJGOGKGGE",
            |m: &CHIBEOEPPML| { &m.FLMJGOGKGGE },
            |m: &mut CHIBEOEPPML| { &mut m.FLMJGOGKGGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IJMLICAGJNO::IJMLICAGJNO>(
            "LDKMCDMDLKL",
            |m: &CHIBEOEPPML| { &m.LDKMCDMDLKL },
            |m: &mut CHIBEOEPPML| { &mut m.LDKMCDMDLKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DDOJFOMJAGF::DDOJFOMJAGF>(
            "GIEACPCMKPN",
            |m: &CHIBEOEPPML| { &m.GIEACPCMKPN },
            |m: &mut CHIBEOEPPML| { &mut m.GIEACPCMKPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EGNGHDKOAAC",
            |m: &CHIBEOEPPML| { &m.EGNGHDKOAAC },
            |m: &mut CHIBEOEPPML| { &mut m.EGNGHDKOAAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PAINLBMGDBC",
            |m: &CHIBEOEPPML| { &m.PAINLBMGDBC },
            |m: &mut CHIBEOEPPML| { &mut m.PAINLBMGDBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMKDEKEIDFB::LMKDEKEIDFB>(
            "BPDEFCDBKBA",
            |m: &CHIBEOEPPML| { &m.BPDEFCDBKBA },
            |m: &mut CHIBEOEPPML| { &mut m.BPDEFCDBKBA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHIBEOEPPML>(
            "CHIBEOEPPML",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHIBEOEPPML {
    const NAME: &'static str = "CHIBEOEPPML";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JHEKFKEENLA)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GIKHHHNGEHC)?;
                },
                34 => {
                    self.FLMJGOGKGGE.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LDKMCDMDLKL)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GIEACPCMKPN)?;
                },
                98 => {
                    self.EGNGHDKOAAC.push(is.read_message()?);
                },
                90 => {
                    self.PAINLBMGDBC.push(is.read_message()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BPDEFCDBKBA)?;
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
        if let Some(v) = self.JHEKFKEENLA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GIKHHHNGEHC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.FLMJGOGKGGE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.LDKMCDMDLKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GIEACPCMKPN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.EGNGHDKOAAC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PAINLBMGDBC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.BPDEFCDBKBA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.JHEKFKEENLA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.GIKHHHNGEHC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        for v in &self.FLMJGOGKGGE {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if let Some(v) = self.LDKMCDMDLKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.GIEACPCMKPN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        for v in &self.EGNGHDKOAAC {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.PAINLBMGDBC {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.BPDEFCDBKBA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> CHIBEOEPPML {
        CHIBEOEPPML::new()
    }

    fn clear(&mut self) {
        self.JHEKFKEENLA.clear();
        self.GIKHHHNGEHC.clear();
        self.FLMJGOGKGGE.clear();
        self.LDKMCDMDLKL.clear();
        self.GIEACPCMKPN.clear();
        self.EGNGHDKOAAC.clear();
        self.PAINLBMGDBC.clear();
        self.BPDEFCDBKBA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHIBEOEPPML {
        static instance: CHIBEOEPPML = CHIBEOEPPML {
            JHEKFKEENLA: ::protobuf::MessageField::none(),
            GIKHHHNGEHC: ::protobuf::MessageField::none(),
            FLMJGOGKGGE: ::std::vec::Vec::new(),
            LDKMCDMDLKL: ::protobuf::MessageField::none(),
            GIEACPCMKPN: ::protobuf::MessageField::none(),
            EGNGHDKOAAC: ::std::vec::Vec::new(),
            PAINLBMGDBC: ::std::vec::Vec::new(),
            BPDEFCDBKBA: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CHIBEOEPPML {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHIBEOEPPML").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHIBEOEPPML {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHIBEOEPPML {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CHIBEOEPPML.proto\x1a\x11ABCOKNJADLI.proto\x1a\x11CNKEFABFHGG.prot\
    o\x1a\x11DDOJFOMJAGF.proto\x1a\x11DGIIKCODGKN.proto\x1a\x11DKABNLJDNPK.p\
    roto\x1a\x11IJMLICAGJNO.proto\x1a\x11JBJDAIHGHKN.proto\x1a\x11LMKDEKEIDF\
    B.proto\"\x8d\x03\n\x0bCHIBEOEPPML\x12.\n\x0bJHEKFKEENLA\x18\x0f\x20\x01\
    (\x0b2\x0c.DGIIKCODGKNR\x0bJHEKFKEENLA\x12.\n\x0bGIKHHHNGEHC\x18\x03\x20\
    \x01(\x0b2\x0c.ABCOKNJADLIR\x0bGIKHHHNGEHC\x12.\n\x0bFLMJGOGKGGE\x18\x04\
    \x20\x03(\x0b2\x0c.DKABNLJDNPKR\x0bFLMJGOGKGGE\x12.\n\x0bLDKMCDMDLKL\x18\
    \x02\x20\x01(\x0b2\x0c.IJMLICAGJNOR\x0bLDKMCDMDLKL\x12.\n\x0bGIEACPCMKPN\
    \x18\t\x20\x01(\x0b2\x0c.DDOJFOMJAGFR\x0bGIEACPCMKPN\x12.\n\x0bEGNGHDKOA\
    AC\x18\x0c\x20\x03(\x0b2\x0c.CNKEFABFHGGR\x0bEGNGHDKOAAC\x12.\n\x0bPAINL\
    BMGDBC\x18\x0b\x20\x03(\x0b2\x0c.JBJDAIHGHKNR\x0bPAINLBMGDBC\x12.\n\x0bB\
    PDEFCDBKBA\x18\r\x20\x01(\x0b2\x0c.LMKDEKEIDFBR\x0bBPDEFCDBKBAb\x06proto\
    3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::ABCOKNJADLI::file_descriptor().clone());
            deps.push(super::CNKEFABFHGG::file_descriptor().clone());
            deps.push(super::DDOJFOMJAGF::file_descriptor().clone());
            deps.push(super::DGIIKCODGKN::file_descriptor().clone());
            deps.push(super::DKABNLJDNPK::file_descriptor().clone());
            deps.push(super::IJMLICAGJNO::file_descriptor().clone());
            deps.push(super::JBJDAIHGHKN::file_descriptor().clone());
            deps.push(super::LMKDEKEIDFB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CHIBEOEPPML::generated_message_descriptor_data());
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
