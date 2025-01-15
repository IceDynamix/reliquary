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

//! Generated file from `LLOBBCHJGDO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LLOBBCHJGDO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LLOBBCHJGDO {
    // message fields
    // @@protoc_insertion_point(field:LLOBBCHJGDO.OJCMGOILFCF)
    pub OJCMGOILFCF: i32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.BILOIOMNADC)
    pub BILOIOMNADC: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.FKLMFFJFHNE)
    pub FKLMFFJFHNE: bool,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.GGHLKCMNEPE)
    pub GGHLKCMNEPE: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.HPBLJJBFCBN)
    pub HPBLJJBFCBN: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.OMCLDMOBEBC)
    pub OMCLDMOBEBC: ::protobuf::MessageField<super::PPAGBCJIBNL::PPAGBCJIBNL>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.NCGJIDNGIBE)
    pub NCGJIDNGIBE: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.KEFHKELPBHC)
    pub KEFHKELPBHC: ::protobuf::EnumOrUnknown<super::ChessRogueDiceType::ChessRogueDiceType>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.DAIBNMGPBFC)
    pub DAIBNMGPBFC: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.CFHHNMMMIHM)
    pub CFHHNMMMIHM: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.LMBDAABGIPC)
    pub LMBDAABGIPC: ::protobuf::MessageField<super::HJPEJBLBDIK::HJPEJBLBDIK>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.AOGLMOLIAPN)
    pub AOGLMOLIAPN: bool,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.OLGAOIBGCHH)
    pub OLGAOIBGCHH: ::protobuf::MessageField<super::EBOIOIKFLFJ::EBOIOIKFLFJ>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.HELNGDNKCLD)
    pub HELNGDNKCLD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.NNAAPJKOHDF)
    pub NNAAPJKOHDF: ::protobuf::EnumOrUnknown<super::ChessRogueDiceStatus::ChessRogueDiceStatus>,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.FIOECJMCKAM)
    pub FIOECJMCKAM: u32,
    // @@protoc_insertion_point(field:LLOBBCHJGDO.CKJPAJFAKAB)
    pub CKJPAJFAKAB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LLOBBCHJGDO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LLOBBCHJGDO {
    fn default() -> &'a LLOBBCHJGDO {
        <LLOBBCHJGDO as ::protobuf::Message>::default_instance()
    }
}

impl LLOBBCHJGDO {
    pub fn new() -> LLOBBCHJGDO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(17);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJCMGOILFCF",
            |m: &LLOBBCHJGDO| { &m.OJCMGOILFCF },
            |m: &mut LLOBBCHJGDO| { &mut m.OJCMGOILFCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BILOIOMNADC",
            |m: &LLOBBCHJGDO| { &m.BILOIOMNADC },
            |m: &mut LLOBBCHJGDO| { &mut m.BILOIOMNADC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKLMFFJFHNE",
            |m: &LLOBBCHJGDO| { &m.FKLMFFJFHNE },
            |m: &mut LLOBBCHJGDO| { &mut m.FKLMFFJFHNE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGHLKCMNEPE",
            |m: &LLOBBCHJGDO| { &m.GGHLKCMNEPE },
            |m: &mut LLOBBCHJGDO| { &mut m.GGHLKCMNEPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPBLJJBFCBN",
            |m: &LLOBBCHJGDO| { &m.HPBLJJBFCBN },
            |m: &mut LLOBBCHJGDO| { &mut m.HPBLJJBFCBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PPAGBCJIBNL::PPAGBCJIBNL>(
            "OMCLDMOBEBC",
            |m: &LLOBBCHJGDO| { &m.OMCLDMOBEBC },
            |m: &mut LLOBBCHJGDO| { &mut m.OMCLDMOBEBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCGJIDNGIBE",
            |m: &LLOBBCHJGDO| { &m.NCGJIDNGIBE },
            |m: &mut LLOBBCHJGDO| { &mut m.NCGJIDNGIBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KEFHKELPBHC",
            |m: &LLOBBCHJGDO| { &m.KEFHKELPBHC },
            |m: &mut LLOBBCHJGDO| { &mut m.KEFHKELPBHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAIBNMGPBFC",
            |m: &LLOBBCHJGDO| { &m.DAIBNMGPBFC },
            |m: &mut LLOBBCHJGDO| { &mut m.DAIBNMGPBFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFHHNMMMIHM",
            |m: &LLOBBCHJGDO| { &m.CFHHNMMMIHM },
            |m: &mut LLOBBCHJGDO| { &mut m.CFHHNMMMIHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HJPEJBLBDIK::HJPEJBLBDIK>(
            "LMBDAABGIPC",
            |m: &LLOBBCHJGDO| { &m.LMBDAABGIPC },
            |m: &mut LLOBBCHJGDO| { &mut m.LMBDAABGIPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AOGLMOLIAPN",
            |m: &LLOBBCHJGDO| { &m.AOGLMOLIAPN },
            |m: &mut LLOBBCHJGDO| { &mut m.AOGLMOLIAPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EBOIOIKFLFJ::EBOIOIKFLFJ>(
            "OLGAOIBGCHH",
            |m: &LLOBBCHJGDO| { &m.OLGAOIBGCHH },
            |m: &mut LLOBBCHJGDO| { &mut m.OLGAOIBGCHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HELNGDNKCLD",
            |m: &LLOBBCHJGDO| { &m.HELNGDNKCLD },
            |m: &mut LLOBBCHJGDO| { &mut m.HELNGDNKCLD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNAAPJKOHDF",
            |m: &LLOBBCHJGDO| { &m.NNAAPJKOHDF },
            |m: &mut LLOBBCHJGDO| { &mut m.NNAAPJKOHDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FIOECJMCKAM",
            |m: &LLOBBCHJGDO| { &m.FIOECJMCKAM },
            |m: &mut LLOBBCHJGDO| { &mut m.FIOECJMCKAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CKJPAJFAKAB",
            |m: &LLOBBCHJGDO| { &m.CKJPAJFAKAB },
            |m: &mut LLOBBCHJGDO| { &mut m.CKJPAJFAKAB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LLOBBCHJGDO>(
            "LLOBBCHJGDO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LLOBBCHJGDO {
    const NAME: &'static str = "LLOBBCHJGDO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                4272 => {
                    self.OJCMGOILFCF = is.read_int32()?;
                },
                24 => {
                    self.BILOIOMNADC = is.read_uint32()?;
                },
                8 => {
                    self.FKLMFFJFHNE = is.read_bool()?;
                },
                56 => {
                    self.GGHLKCMNEPE = is.read_uint32()?;
                },
                112 => {
                    self.HPBLJJBFCBN = is.read_uint32()?;
                },
                12042 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OMCLDMOBEBC)?;
                },
                96 => {
                    self.NCGJIDNGIBE = is.read_uint32()?;
                },
                40 => {
                    self.KEFHKELPBHC = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.DAIBNMGPBFC = is.read_uint32()?;
                },
                104 => {
                    self.CFHHNMMMIHM = is.read_uint32()?;
                },
                11698 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LMBDAABGIPC)?;
                },
                14248 => {
                    self.AOGLMOLIAPN = is.read_bool()?;
                },
                13562 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OLGAOIBGCHH)?;
                },
                9522 => {
                    is.read_repeated_packed_uint32_into(&mut self.HELNGDNKCLD)?;
                },
                9520 => {
                    self.HELNGDNKCLD.push(is.read_uint32()?);
                },
                88 => {
                    self.NNAAPJKOHDF = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.FIOECJMCKAM = is.read_uint32()?;
                },
                120 => {
                    self.CKJPAJFAKAB = is.read_uint32()?;
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
        if self.OJCMGOILFCF != 0 {
            my_size += ::protobuf::rt::int32_size(534, self.OJCMGOILFCF);
        }
        if self.BILOIOMNADC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BILOIOMNADC);
        }
        if self.FKLMFFJFHNE != false {
            my_size += 1 + 1;
        }
        if self.GGHLKCMNEPE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.GGHLKCMNEPE);
        }
        if self.HPBLJJBFCBN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HPBLJJBFCBN);
        }
        if let Some(v) = self.OMCLDMOBEBC.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NCGJIDNGIBE != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.NCGJIDNGIBE);
        }
        if self.KEFHKELPBHC != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED) {
            my_size += ::protobuf::rt::int32_size(5, self.KEFHKELPBHC.value());
        }
        if self.DAIBNMGPBFC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.DAIBNMGPBFC);
        }
        if self.CFHHNMMMIHM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CFHHNMMMIHM);
        }
        if let Some(v) = self.LMBDAABGIPC.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AOGLMOLIAPN != false {
            my_size += 2 + 1;
        }
        if let Some(v) = self.OLGAOIBGCHH.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.HELNGDNKCLD {
            my_size += ::protobuf::rt::uint32_size(1190, *value);
        };
        if self.NNAAPJKOHDF != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE) {
            my_size += ::protobuf::rt::int32_size(11, self.NNAAPJKOHDF.value());
        }
        if self.FIOECJMCKAM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.FIOECJMCKAM);
        }
        if self.CKJPAJFAKAB != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CKJPAJFAKAB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OJCMGOILFCF != 0 {
            os.write_int32(534, self.OJCMGOILFCF)?;
        }
        if self.BILOIOMNADC != 0 {
            os.write_uint32(3, self.BILOIOMNADC)?;
        }
        if self.FKLMFFJFHNE != false {
            os.write_bool(1, self.FKLMFFJFHNE)?;
        }
        if self.GGHLKCMNEPE != 0 {
            os.write_uint32(7, self.GGHLKCMNEPE)?;
        }
        if self.HPBLJJBFCBN != 0 {
            os.write_uint32(14, self.HPBLJJBFCBN)?;
        }
        if let Some(v) = self.OMCLDMOBEBC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1505, v, os)?;
        }
        if self.NCGJIDNGIBE != 0 {
            os.write_uint32(12, self.NCGJIDNGIBE)?;
        }
        if self.KEFHKELPBHC != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.KEFHKELPBHC))?;
        }
        if self.DAIBNMGPBFC != 0 {
            os.write_uint32(6, self.DAIBNMGPBFC)?;
        }
        if self.CFHHNMMMIHM != 0 {
            os.write_uint32(13, self.CFHHNMMMIHM)?;
        }
        if let Some(v) = self.LMBDAABGIPC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1462, v, os)?;
        }
        if self.AOGLMOLIAPN != false {
            os.write_bool(1781, self.AOGLMOLIAPN)?;
        }
        if let Some(v) = self.OLGAOIBGCHH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1695, v, os)?;
        }
        for v in &self.HELNGDNKCLD {
            os.write_uint32(1190, *v)?;
        };
        if self.NNAAPJKOHDF != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.NNAAPJKOHDF))?;
        }
        if self.FIOECJMCKAM != 0 {
            os.write_uint32(10, self.FIOECJMCKAM)?;
        }
        if self.CKJPAJFAKAB != 0 {
            os.write_uint32(15, self.CKJPAJFAKAB)?;
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

    fn new() -> LLOBBCHJGDO {
        LLOBBCHJGDO::new()
    }

    fn clear(&mut self) {
        self.OJCMGOILFCF = 0;
        self.BILOIOMNADC = 0;
        self.FKLMFFJFHNE = false;
        self.GGHLKCMNEPE = 0;
        self.HPBLJJBFCBN = 0;
        self.OMCLDMOBEBC.clear();
        self.NCGJIDNGIBE = 0;
        self.KEFHKELPBHC = ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED);
        self.DAIBNMGPBFC = 0;
        self.CFHHNMMMIHM = 0;
        self.LMBDAABGIPC.clear();
        self.AOGLMOLIAPN = false;
        self.OLGAOIBGCHH.clear();
        self.HELNGDNKCLD.clear();
        self.NNAAPJKOHDF = ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE);
        self.FIOECJMCKAM = 0;
        self.CKJPAJFAKAB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LLOBBCHJGDO {
        static instance: LLOBBCHJGDO = LLOBBCHJGDO {
            OJCMGOILFCF: 0,
            BILOIOMNADC: 0,
            FKLMFFJFHNE: false,
            GGHLKCMNEPE: 0,
            HPBLJJBFCBN: 0,
            OMCLDMOBEBC: ::protobuf::MessageField::none(),
            NCGJIDNGIBE: 0,
            KEFHKELPBHC: ::protobuf::EnumOrUnknown::from_i32(0),
            DAIBNMGPBFC: 0,
            CFHHNMMMIHM: 0,
            LMBDAABGIPC: ::protobuf::MessageField::none(),
            AOGLMOLIAPN: false,
            OLGAOIBGCHH: ::protobuf::MessageField::none(),
            HELNGDNKCLD: ::std::vec::Vec::new(),
            NNAAPJKOHDF: ::protobuf::EnumOrUnknown::from_i32(0),
            FIOECJMCKAM: 0,
            CKJPAJFAKAB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LLOBBCHJGDO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LLOBBCHJGDO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LLOBBCHJGDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LLOBBCHJGDO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LLOBBCHJGDO.proto\x1a\x1aChessRogueDiceStatus.proto\x1a\x18ChessRo\
    gueDiceType.proto\x1a\x11EBOIOIKFLFJ.proto\x1a\x11HJPEJBLBDIK.proto\x1a\
    \x11PPAGBCJIBNL.proto\"\xab\x05\n\x0bLLOBBCHJGDO\x12!\n\x0bOJCMGOILFCF\
    \x18\x96\x04\x20\x01(\x05R\x0bOJCMGOILFCF\x12\x20\n\x0bBILOIOMNADC\x18\
    \x03\x20\x01(\rR\x0bBILOIOMNADC\x12\x20\n\x0bFKLMFFJFHNE\x18\x01\x20\x01\
    (\x08R\x0bFKLMFFJFHNE\x12\x20\n\x0bGGHLKCMNEPE\x18\x07\x20\x01(\rR\x0bGG\
    HLKCMNEPE\x12\x20\n\x0bHPBLJJBFCBN\x18\x0e\x20\x01(\rR\x0bHPBLJJBFCBN\
    \x12/\n\x0bOMCLDMOBEBC\x18\xe1\x0b\x20\x01(\x0b2\x0c.PPAGBCJIBNLR\x0bOMC\
    LDMOBEBC\x12\x20\n\x0bNCGJIDNGIBE\x18\x0c\x20\x01(\rR\x0bNCGJIDNGIBE\x12\
    5\n\x0bKEFHKELPBHC\x18\x05\x20\x01(\x0e2\x13.ChessRogueDiceTypeR\x0bKEFH\
    KELPBHC\x12\x20\n\x0bDAIBNMGPBFC\x18\x06\x20\x01(\rR\x0bDAIBNMGPBFC\x12\
    \x20\n\x0bCFHHNMMMIHM\x18\r\x20\x01(\rR\x0bCFHHNMMMIHM\x12/\n\x0bLMBDAAB\
    GIPC\x18\xb6\x0b\x20\x01(\x0b2\x0c.HJPEJBLBDIKR\x0bLMBDAABGIPC\x12!\n\
    \x0bAOGLMOLIAPN\x18\xf5\r\x20\x01(\x08R\x0bAOGLMOLIAPN\x12/\n\x0bOLGAOIB\
    GCHH\x18\x9f\r\x20\x01(\x0b2\x0c.EBOIOIKFLFJR\x0bOLGAOIBGCHH\x12!\n\x0bH\
    ELNGDNKCLD\x18\xa6\t\x20\x03(\rR\x0bHELNGDNKCLD\x127\n\x0bNNAAPJKOHDF\
    \x18\x0b\x20\x01(\x0e2\x15.ChessRogueDiceStatusR\x0bNNAAPJKOHDF\x12\x20\
    \n\x0bFIOECJMCKAM\x18\n\x20\x01(\rR\x0bFIOECJMCKAM\x12\x20\n\x0bCKJPAJFA\
    KAB\x18\x0f\x20\x01(\rR\x0bCKJPAJFAKABb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::ChessRogueDiceStatus::file_descriptor().clone());
            deps.push(super::ChessRogueDiceType::file_descriptor().clone());
            deps.push(super::EBOIOIKFLFJ::file_descriptor().clone());
            deps.push(super::HJPEJBLBDIK::file_descriptor().clone());
            deps.push(super::PPAGBCJIBNL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LLOBBCHJGDO::generated_message_descriptor_data());
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