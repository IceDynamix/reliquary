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

//! Generated file from `KAOOHLAKBPN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KAOOHLAKBPN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KAOOHLAKBPN {
    // message fields
    // @@protoc_insertion_point(field:KAOOHLAKBPN.LGKAJPNIHGM)
    pub LGKAJPNIHGM: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.IPNHCCODNDI)
    pub IPNHCCODNDI: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.BIODAPMJNGM)
    pub BIODAPMJNGM: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.GCECNKELLIM)
    pub GCECNKELLIM: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.POJPNLGAPMM)
    pub POJPNLGAPMM: ::std::vec::Vec<super::BNFNHHIKLPK::BNFNHHIKLPK>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.NAIIKFNECLJ)
    pub NAIIKFNECLJ: ::std::vec::Vec<super::DBGLAHAIACB::DBGLAHAIACB>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.IDGGIBBPPLK)
    pub IDGGIBBPPLK: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.NCJHNPAAKAC)
    pub NCJHNPAAKAC: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.LMDMCHBMLAJ)
    pub LMDMCHBMLAJ: ::std::vec::Vec<super::HDODGJEDIAF::HDODGJEDIAF>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.AEGLDOEJDBL)
    pub AEGLDOEJDBL: u32,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.KACIOCMFANP)
    pub KACIOCMFANP: ::protobuf::MessageField<super::PGJBMPEDGFM::PGJBMPEDGFM>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.FFOIKMKDBNF)
    pub FFOIKMKDBNF: ::protobuf::MessageField<super::EHCDBAGADEA::EHCDBAGADEA>,
    // @@protoc_insertion_point(field:KAOOHLAKBPN.BFODMCKIMLF)
    pub BFODMCKIMLF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KAOOHLAKBPN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KAOOHLAKBPN {
    fn default() -> &'a KAOOHLAKBPN {
        <KAOOHLAKBPN as ::protobuf::Message>::default_instance()
    }
}

impl KAOOHLAKBPN {
    pub fn new() -> KAOOHLAKBPN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(15);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGKAJPNIHGM",
            |m: &KAOOHLAKBPN| { &m.LGKAJPNIHGM },
            |m: &mut KAOOHLAKBPN| { &mut m.LGKAJPNIHGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPNHCCODNDI",
            |m: &KAOOHLAKBPN| { &m.IPNHCCODNDI },
            |m: &mut KAOOHLAKBPN| { &mut m.IPNHCCODNDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &KAOOHLAKBPN| { &m.JKOCJIMAGBN },
            |m: &mut KAOOHLAKBPN| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIODAPMJNGM",
            |m: &KAOOHLAKBPN| { &m.BIODAPMJNGM },
            |m: &mut KAOOHLAKBPN| { &mut m.BIODAPMJNGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCECNKELLIM",
            |m: &KAOOHLAKBPN| { &m.GCECNKELLIM },
            |m: &mut KAOOHLAKBPN| { &mut m.GCECNKELLIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "POJPNLGAPMM",
            |m: &KAOOHLAKBPN| { &m.POJPNLGAPMM },
            |m: &mut KAOOHLAKBPN| { &mut m.POJPNLGAPMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NAIIKFNECLJ",
            |m: &KAOOHLAKBPN| { &m.NAIIKFNECLJ },
            |m: &mut KAOOHLAKBPN| { &mut m.NAIIKFNECLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDGGIBBPPLK",
            |m: &KAOOHLAKBPN| { &m.IDGGIBBPPLK },
            |m: &mut KAOOHLAKBPN| { &mut m.IDGGIBBPPLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCJHNPAAKAC",
            |m: &KAOOHLAKBPN| { &m.NCJHNPAAKAC },
            |m: &mut KAOOHLAKBPN| { &mut m.NCJHNPAAKAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMDMCHBMLAJ",
            |m: &KAOOHLAKBPN| { &m.LMDMCHBMLAJ },
            |m: &mut KAOOHLAKBPN| { &mut m.LMDMCHBMLAJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &KAOOHLAKBPN| { &m.DNMJBNNJLEL },
            |m: &mut KAOOHLAKBPN| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEGLDOEJDBL",
            |m: &KAOOHLAKBPN| { &m.AEGLDOEJDBL },
            |m: &mut KAOOHLAKBPN| { &mut m.AEGLDOEJDBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PGJBMPEDGFM::PGJBMPEDGFM>(
            "KACIOCMFANP",
            |m: &KAOOHLAKBPN| { &m.KACIOCMFANP },
            |m: &mut KAOOHLAKBPN| { &mut m.KACIOCMFANP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EHCDBAGADEA::EHCDBAGADEA>(
            "FFOIKMKDBNF",
            |m: &KAOOHLAKBPN| { &m.FFOIKMKDBNF },
            |m: &mut KAOOHLAKBPN| { &mut m.FFOIKMKDBNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFODMCKIMLF",
            |m: &KAOOHLAKBPN| { &m.BFODMCKIMLF },
            |m: &mut KAOOHLAKBPN| { &mut m.BFODMCKIMLF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KAOOHLAKBPN>(
            "KAOOHLAKBPN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KAOOHLAKBPN {
    const NAME: &'static str = "KAOOHLAKBPN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.LGKAJPNIHGM = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.IPNHCCODNDI = is.read_uint32()?;
                },
                24 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                32 => {
                    self.BIODAPMJNGM = is.read_uint32()?;
                },
                40 => {
                    self.GCECNKELLIM = is.read_uint32()?;
                },
                50 => {
                    self.POJPNLGAPMM.push(is.read_message()?);
                },
                58 => {
                    self.NAIIKFNECLJ.push(is.read_message()?);
                },
                64 => {
                    self.IDGGIBBPPLK = is.read_uint32()?;
                },
                80 => {
                    self.NCJHNPAAKAC = is.read_uint32()?;
                },
                90 => {
                    self.LMDMCHBMLAJ.push(is.read_message()?);
                },
                96 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                104 => {
                    self.AEGLDOEJDBL = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KACIOCMFANP)?;
                },
                130 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FFOIKMKDBNF)?;
                },
                136 => {
                    self.BFODMCKIMLF = is.read_uint32()?;
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
        if self.LGKAJPNIHGM != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.LGKAJPNIHGM.value());
        }
        if self.IPNHCCODNDI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IPNHCCODNDI);
        }
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JKOCJIMAGBN);
        }
        if self.BIODAPMJNGM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BIODAPMJNGM);
        }
        if self.GCECNKELLIM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GCECNKELLIM);
        }
        for value in &self.POJPNLGAPMM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.NAIIKFNECLJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IDGGIBBPPLK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IDGGIBBPPLK);
        }
        if self.NCJHNPAAKAC != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NCJHNPAAKAC);
        }
        for value in &self.LMDMCHBMLAJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DNMJBNNJLEL);
        }
        if self.AEGLDOEJDBL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.AEGLDOEJDBL);
        }
        if let Some(v) = self.KACIOCMFANP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FFOIKMKDBNF.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BFODMCKIMLF != 0 {
            my_size += ::protobuf::rt::uint32_size(17, self.BFODMCKIMLF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LGKAJPNIHGM != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.LGKAJPNIHGM))?;
        }
        if self.IPNHCCODNDI != 0 {
            os.write_uint32(2, self.IPNHCCODNDI)?;
        }
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(3, self.JKOCJIMAGBN)?;
        }
        if self.BIODAPMJNGM != 0 {
            os.write_uint32(4, self.BIODAPMJNGM)?;
        }
        if self.GCECNKELLIM != 0 {
            os.write_uint32(5, self.GCECNKELLIM)?;
        }
        for v in &self.POJPNLGAPMM {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.NAIIKFNECLJ {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.IDGGIBBPPLK != 0 {
            os.write_uint32(8, self.IDGGIBBPPLK)?;
        }
        if self.NCJHNPAAKAC != 0 {
            os.write_uint32(10, self.NCJHNPAAKAC)?;
        }
        for v in &self.LMDMCHBMLAJ {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(12, self.DNMJBNNJLEL)?;
        }
        if self.AEGLDOEJDBL != 0 {
            os.write_uint32(13, self.AEGLDOEJDBL)?;
        }
        if let Some(v) = self.KACIOCMFANP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.FFOIKMKDBNF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        }
        if self.BFODMCKIMLF != 0 {
            os.write_uint32(17, self.BFODMCKIMLF)?;
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

    fn new() -> KAOOHLAKBPN {
        KAOOHLAKBPN::new()
    }

    fn clear(&mut self) {
        self.LGKAJPNIHGM = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.IPNHCCODNDI = 0;
        self.JKOCJIMAGBN = 0;
        self.BIODAPMJNGM = 0;
        self.GCECNKELLIM = 0;
        self.POJPNLGAPMM.clear();
        self.NAIIKFNECLJ.clear();
        self.IDGGIBBPPLK = 0;
        self.NCJHNPAAKAC = 0;
        self.LMDMCHBMLAJ.clear();
        self.DNMJBNNJLEL = 0;
        self.AEGLDOEJDBL = 0;
        self.KACIOCMFANP.clear();
        self.FFOIKMKDBNF.clear();
        self.BFODMCKIMLF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KAOOHLAKBPN {
        static instance: KAOOHLAKBPN = KAOOHLAKBPN {
            LGKAJPNIHGM: ::protobuf::EnumOrUnknown::from_i32(0),
            IPNHCCODNDI: 0,
            JKOCJIMAGBN: 0,
            BIODAPMJNGM: 0,
            GCECNKELLIM: 0,
            POJPNLGAPMM: ::std::vec::Vec::new(),
            NAIIKFNECLJ: ::std::vec::Vec::new(),
            IDGGIBBPPLK: 0,
            NCJHNPAAKAC: 0,
            LMDMCHBMLAJ: ::std::vec::Vec::new(),
            DNMJBNNJLEL: 0,
            AEGLDOEJDBL: 0,
            KACIOCMFANP: ::protobuf::MessageField::none(),
            FFOIKMKDBNF: ::protobuf::MessageField::none(),
            BFODMCKIMLF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KAOOHLAKBPN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KAOOHLAKBPN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KAOOHLAKBPN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KAOOHLAKBPN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KAOOHLAKBPN.proto\x1a\x10AvatarType.proto\x1a\x11BNFNHHIKLPK.proto\
    \x1a\x11DBGLAHAIACB.proto\x1a\x11EHCDBAGADEA.proto\x1a\x11HDODGJEDIAF.pr\
    oto\x1a\x11PGJBMPEDGFM.proto\"\xde\x04\n\x0bKAOOHLAKBPN\x12-\n\x0bLGKAJP\
    NIHGM\x18\x01\x20\x01(\x0e2\x0b.AvatarTypeR\x0bLGKAJPNIHGM\x12\x20\n\x0b\
    IPNHCCODNDI\x18\x02\x20\x01(\rR\x0bIPNHCCODNDI\x12\x20\n\x0bJKOCJIMAGBN\
    \x18\x03\x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\n\x0bBIODAPMJNGM\x18\x04\x20\
    \x01(\rR\x0bBIODAPMJNGM\x12\x20\n\x0bGCECNKELLIM\x18\x05\x20\x01(\rR\x0b\
    GCECNKELLIM\x12.\n\x0bPOJPNLGAPMM\x18\x06\x20\x03(\x0b2\x0c.BNFNHHIKLPKR\
    \x0bPOJPNLGAPMM\x12.\n\x0bNAIIKFNECLJ\x18\x07\x20\x03(\x0b2\x0c.DBGLAHAI\
    ACBR\x0bNAIIKFNECLJ\x12\x20\n\x0bIDGGIBBPPLK\x18\x08\x20\x01(\rR\x0bIDGG\
    IBBPPLK\x12\x20\n\x0bNCJHNPAAKAC\x18\n\x20\x01(\rR\x0bNCJHNPAAKAC\x12.\n\
    \x0bLMDMCHBMLAJ\x18\x0b\x20\x03(\x0b2\x0c.HDODGJEDIAFR\x0bLMDMCHBMLAJ\
    \x12\x20\n\x0bDNMJBNNJLEL\x18\x0c\x20\x01(\rR\x0bDNMJBNNJLEL\x12\x20\n\
    \x0bAEGLDOEJDBL\x18\r\x20\x01(\rR\x0bAEGLDOEJDBL\x12.\n\x0bKACIOCMFANP\
    \x18\x0f\x20\x01(\x0b2\x0c.PGJBMPEDGFMR\x0bKACIOCMFANP\x12.\n\x0bFFOIKMK\
    DBNF\x18\x10\x20\x01(\x0b2\x0c.EHCDBAGADEAR\x0bFFOIKMKDBNF\x12\x20\n\x0b\
    BFODMCKIMLF\x18\x11\x20\x01(\rR\x0bBFODMCKIMLFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::AvatarType::file_descriptor().clone());
            deps.push(super::BNFNHHIKLPK::file_descriptor().clone());
            deps.push(super::DBGLAHAIACB::file_descriptor().clone());
            deps.push(super::EHCDBAGADEA::file_descriptor().clone());
            deps.push(super::HDODGJEDIAF::file_descriptor().clone());
            deps.push(super::PGJBMPEDGFM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KAOOHLAKBPN::generated_message_descriptor_data());
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