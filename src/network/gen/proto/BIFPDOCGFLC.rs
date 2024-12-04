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

//! Generated file from `BIFPDOCGFLC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BIFPDOCGFLC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BIFPDOCGFLC {
    // message fields
    // @@protoc_insertion_point(field:BIFPDOCGFLC.NHCHLEKIOLI)
    pub NHCHLEKIOLI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.LBBDDEKIKII)
    pub LBBDDEKIKII: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.LMOJJANGNAI)
    pub LMOJJANGNAI: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.EFGNGPJKOAA)
    pub EFGNGPJKOAA: ::protobuf::MessageField<super::FOCKFLEPFCF::FOCKFLEPFCF>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.NJNFGBJGOLJ)
    pub NJNFGBJGOLJ: ::protobuf::MessageField<super::HHEJBAAIMFE::HHEJBAAIMFE>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.OGOHDNIHBGI)
    pub OGOHDNIHBGI: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.MHMJMECKNNF)
    pub MHMJMECKNNF: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.HAKCDMKJHFG)
    pub HAKCDMKJHFG: bool,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.DEKGEGFGLED)
    pub DEKGEGFGLED: bool,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.NIPEACDADDD)
    pub NIPEACDADDD: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.LOIOGPDKFHL)
    pub LOIOGPDKFHL: ::protobuf::MessageField<super::NMKHMIOOPJN::NMKHMIOOPJN>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.LPELHHCDBIM)
    pub LPELHHCDBIM: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.GIKPKGEGOBO)
    pub GIKPKGEGOBO: u32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.FFPKLDIMFED)
    pub FFPKLDIMFED: ::protobuf::EnumOrUnknown<super::ChessRogueDiceStatus::ChessRogueDiceStatus>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.EHKMBFKMIPJ)
    pub EHKMBFKMIPJ: i32,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.CEJHPGPJACE)
    pub CEJHPGPJACE: ::protobuf::EnumOrUnknown<super::ChessRogueDiceType::ChessRogueDiceType>,
    // @@protoc_insertion_point(field:BIFPDOCGFLC.BIBFJDMPJHM)
    pub BIBFJDMPJHM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BIFPDOCGFLC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BIFPDOCGFLC {
    fn default() -> &'a BIFPDOCGFLC {
        <BIFPDOCGFLC as ::protobuf::Message>::default_instance()
    }
}

impl BIFPDOCGFLC {
    pub fn new() -> BIFPDOCGFLC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(17);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NHCHLEKIOLI",
            |m: &BIFPDOCGFLC| { &m.NHCHLEKIOLI },
            |m: &mut BIFPDOCGFLC| { &mut m.NHCHLEKIOLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBBDDEKIKII",
            |m: &BIFPDOCGFLC| { &m.LBBDDEKIKII },
            |m: &mut BIFPDOCGFLC| { &mut m.LBBDDEKIKII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMOJJANGNAI",
            |m: &BIFPDOCGFLC| { &m.LMOJJANGNAI },
            |m: &mut BIFPDOCGFLC| { &mut m.LMOJJANGNAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FOCKFLEPFCF::FOCKFLEPFCF>(
            "EFGNGPJKOAA",
            |m: &BIFPDOCGFLC| { &m.EFGNGPJKOAA },
            |m: &mut BIFPDOCGFLC| { &mut m.EFGNGPJKOAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HHEJBAAIMFE::HHEJBAAIMFE>(
            "NJNFGBJGOLJ",
            |m: &BIFPDOCGFLC| { &m.NJNFGBJGOLJ },
            |m: &mut BIFPDOCGFLC| { &mut m.NJNFGBJGOLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGOHDNIHBGI",
            |m: &BIFPDOCGFLC| { &m.OGOHDNIHBGI },
            |m: &mut BIFPDOCGFLC| { &mut m.OGOHDNIHBGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MHMJMECKNNF",
            |m: &BIFPDOCGFLC| { &m.MHMJMECKNNF },
            |m: &mut BIFPDOCGFLC| { &mut m.MHMJMECKNNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HAKCDMKJHFG",
            |m: &BIFPDOCGFLC| { &m.HAKCDMKJHFG },
            |m: &mut BIFPDOCGFLC| { &mut m.HAKCDMKJHFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEKGEGFGLED",
            |m: &BIFPDOCGFLC| { &m.DEKGEGFGLED },
            |m: &mut BIFPDOCGFLC| { &mut m.DEKGEGFGLED },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIPEACDADDD",
            |m: &BIFPDOCGFLC| { &m.NIPEACDADDD },
            |m: &mut BIFPDOCGFLC| { &mut m.NIPEACDADDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NMKHMIOOPJN::NMKHMIOOPJN>(
            "LOIOGPDKFHL",
            |m: &BIFPDOCGFLC| { &m.LOIOGPDKFHL },
            |m: &mut BIFPDOCGFLC| { &mut m.LOIOGPDKFHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LPELHHCDBIM",
            |m: &BIFPDOCGFLC| { &m.LPELHHCDBIM },
            |m: &mut BIFPDOCGFLC| { &mut m.LPELHHCDBIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIKPKGEGOBO",
            |m: &BIFPDOCGFLC| { &m.GIKPKGEGOBO },
            |m: &mut BIFPDOCGFLC| { &mut m.GIKPKGEGOBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFPKLDIMFED",
            |m: &BIFPDOCGFLC| { &m.FFPKLDIMFED },
            |m: &mut BIFPDOCGFLC| { &mut m.FFPKLDIMFED },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EHKMBFKMIPJ",
            |m: &BIFPDOCGFLC| { &m.EHKMBFKMIPJ },
            |m: &mut BIFPDOCGFLC| { &mut m.EHKMBFKMIPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CEJHPGPJACE",
            |m: &BIFPDOCGFLC| { &m.CEJHPGPJACE },
            |m: &mut BIFPDOCGFLC| { &mut m.CEJHPGPJACE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIBFJDMPJHM",
            |m: &BIFPDOCGFLC| { &m.BIBFJDMPJHM },
            |m: &mut BIFPDOCGFLC| { &mut m.BIBFJDMPJHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BIFPDOCGFLC>(
            "BIFPDOCGFLC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BIFPDOCGFLC {
    const NAME: &'static str = "BIFPDOCGFLC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                11538 => {
                    is.read_repeated_packed_uint32_into(&mut self.NHCHLEKIOLI)?;
                },
                11536 => {
                    self.NHCHLEKIOLI.push(is.read_uint32()?);
                },
                72 => {
                    self.LBBDDEKIKII = is.read_uint32()?;
                },
                96 => {
                    self.LMOJJANGNAI = is.read_uint32()?;
                },
                1586 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EFGNGPJKOAA)?;
                },
                3338 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NJNFGBJGOLJ)?;
                },
                88 => {
                    self.OGOHDNIHBGI = is.read_uint32()?;
                },
                112 => {
                    self.MHMJMECKNNF = is.read_uint32()?;
                },
                8 => {
                    self.HAKCDMKJHFG = is.read_bool()?;
                },
                1776 => {
                    self.DEKGEGFGLED = is.read_bool()?;
                },
                32 => {
                    self.NIPEACDADDD = is.read_uint32()?;
                },
                4306 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LOIOGPDKFHL)?;
                },
                120 => {
                    self.LPELHHCDBIM = is.read_uint32()?;
                },
                24 => {
                    self.GIKPKGEGOBO = is.read_uint32()?;
                },
                16 => {
                    self.FFPKLDIMFED = is.read_enum_or_unknown()?;
                },
                12208 => {
                    self.EHKMBFKMIPJ = is.read_int32()?;
                },
                104 => {
                    self.CEJHPGPJACE = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.BIBFJDMPJHM = is.read_uint32()?;
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
        for value in &self.NHCHLEKIOLI {
            my_size += ::protobuf::rt::uint32_size(1442, *value);
        };
        if self.LBBDDEKIKII != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.LBBDDEKIKII);
        }
        if self.LMOJJANGNAI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LMOJJANGNAI);
        }
        if let Some(v) = self.EFGNGPJKOAA.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NJNFGBJGOLJ.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OGOHDNIHBGI != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.OGOHDNIHBGI);
        }
        if self.MHMJMECKNNF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.MHMJMECKNNF);
        }
        if self.HAKCDMKJHFG != false {
            my_size += 1 + 1;
        }
        if self.DEKGEGFGLED != false {
            my_size += 2 + 1;
        }
        if self.NIPEACDADDD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NIPEACDADDD);
        }
        if let Some(v) = self.LOIOGPDKFHL.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LPELHHCDBIM != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LPELHHCDBIM);
        }
        if self.GIKPKGEGOBO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.GIKPKGEGOBO);
        }
        if self.FFPKLDIMFED != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE) {
            my_size += ::protobuf::rt::int32_size(2, self.FFPKLDIMFED.value());
        }
        if self.EHKMBFKMIPJ != 0 {
            my_size += ::protobuf::rt::int32_size(1526, self.EHKMBFKMIPJ);
        }
        if self.CEJHPGPJACE != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED) {
            my_size += ::protobuf::rt::int32_size(13, self.CEJHPGPJACE.value());
        }
        if self.BIBFJDMPJHM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.BIBFJDMPJHM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NHCHLEKIOLI {
            os.write_uint32(1442, *v)?;
        };
        if self.LBBDDEKIKII != 0 {
            os.write_uint32(9, self.LBBDDEKIKII)?;
        }
        if self.LMOJJANGNAI != 0 {
            os.write_uint32(12, self.LMOJJANGNAI)?;
        }
        if let Some(v) = self.EFGNGPJKOAA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(198, v, os)?;
        }
        if let Some(v) = self.NJNFGBJGOLJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(417, v, os)?;
        }
        if self.OGOHDNIHBGI != 0 {
            os.write_uint32(11, self.OGOHDNIHBGI)?;
        }
        if self.MHMJMECKNNF != 0 {
            os.write_uint32(14, self.MHMJMECKNNF)?;
        }
        if self.HAKCDMKJHFG != false {
            os.write_bool(1, self.HAKCDMKJHFG)?;
        }
        if self.DEKGEGFGLED != false {
            os.write_bool(222, self.DEKGEGFGLED)?;
        }
        if self.NIPEACDADDD != 0 {
            os.write_uint32(4, self.NIPEACDADDD)?;
        }
        if let Some(v) = self.LOIOGPDKFHL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(538, v, os)?;
        }
        if self.LPELHHCDBIM != 0 {
            os.write_uint32(15, self.LPELHHCDBIM)?;
        }
        if self.GIKPKGEGOBO != 0 {
            os.write_uint32(3, self.GIKPKGEGOBO)?;
        }
        if self.FFPKLDIMFED != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.FFPKLDIMFED))?;
        }
        if self.EHKMBFKMIPJ != 0 {
            os.write_int32(1526, self.EHKMBFKMIPJ)?;
        }
        if self.CEJHPGPJACE != ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.CEJHPGPJACE))?;
        }
        if self.BIBFJDMPJHM != 0 {
            os.write_uint32(7, self.BIBFJDMPJHM)?;
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

    fn new() -> BIFPDOCGFLC {
        BIFPDOCGFLC::new()
    }

    fn clear(&mut self) {
        self.NHCHLEKIOLI.clear();
        self.LBBDDEKIKII = 0;
        self.LMOJJANGNAI = 0;
        self.EFGNGPJKOAA.clear();
        self.NJNFGBJGOLJ.clear();
        self.OGOHDNIHBGI = 0;
        self.MHMJMECKNNF = 0;
        self.HAKCDMKJHFG = false;
        self.DEKGEGFGLED = false;
        self.NIPEACDADDD = 0;
        self.LOIOGPDKFHL.clear();
        self.LPELHHCDBIM = 0;
        self.GIKPKGEGOBO = 0;
        self.FFPKLDIMFED = ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceStatus::ChessRogueDiceStatus::CHESS_ROGUE_DICE_IDLE);
        self.EHKMBFKMIPJ = 0;
        self.CEJHPGPJACE = ::protobuf::EnumOrUnknown::new(super::ChessRogueDiceType::ChessRogueDiceType::CHESS_ROGUE_DICE_FIXED);
        self.BIBFJDMPJHM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BIFPDOCGFLC {
        static instance: BIFPDOCGFLC = BIFPDOCGFLC {
            NHCHLEKIOLI: ::std::vec::Vec::new(),
            LBBDDEKIKII: 0,
            LMOJJANGNAI: 0,
            EFGNGPJKOAA: ::protobuf::MessageField::none(),
            NJNFGBJGOLJ: ::protobuf::MessageField::none(),
            OGOHDNIHBGI: 0,
            MHMJMECKNNF: 0,
            HAKCDMKJHFG: false,
            DEKGEGFGLED: false,
            NIPEACDADDD: 0,
            LOIOGPDKFHL: ::protobuf::MessageField::none(),
            LPELHHCDBIM: 0,
            GIKPKGEGOBO: 0,
            FFPKLDIMFED: ::protobuf::EnumOrUnknown::from_i32(0),
            EHKMBFKMIPJ: 0,
            CEJHPGPJACE: ::protobuf::EnumOrUnknown::from_i32(0),
            BIBFJDMPJHM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BIFPDOCGFLC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BIFPDOCGFLC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BIFPDOCGFLC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BIFPDOCGFLC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BIFPDOCGFLC.proto\x1a\x1aChessRogueDiceStatus.proto\x1a\x18ChessRo\
    gueDiceType.proto\x1a\x11FOCKFLEPFCF.proto\x1a\x11HHEJBAAIMFE.proto\x1a\
    \x11NMKHMIOOPJN.proto\"\xab\x05\n\x0bBIFPDOCGFLC\x12!\n\x0bNHCHLEKIOLI\
    \x18\xa2\x0b\x20\x03(\rR\x0bNHCHLEKIOLI\x12\x20\n\x0bLBBDDEKIKII\x18\t\
    \x20\x01(\rR\x0bLBBDDEKIKII\x12\x20\n\x0bLMOJJANGNAI\x18\x0c\x20\x01(\rR\
    \x0bLMOJJANGNAI\x12/\n\x0bEFGNGPJKOAA\x18\xc6\x01\x20\x01(\x0b2\x0c.FOCK\
    FLEPFCFR\x0bEFGNGPJKOAA\x12/\n\x0bNJNFGBJGOLJ\x18\xa1\x03\x20\x01(\x0b2\
    \x0c.HHEJBAAIMFER\x0bNJNFGBJGOLJ\x12\x20\n\x0bOGOHDNIHBGI\x18\x0b\x20\
    \x01(\rR\x0bOGOHDNIHBGI\x12\x20\n\x0bMHMJMECKNNF\x18\x0e\x20\x01(\rR\x0b\
    MHMJMECKNNF\x12\x20\n\x0bHAKCDMKJHFG\x18\x01\x20\x01(\x08R\x0bHAKCDMKJHF\
    G\x12!\n\x0bDEKGEGFGLED\x18\xde\x01\x20\x01(\x08R\x0bDEKGEGFGLED\x12\x20\
    \n\x0bNIPEACDADDD\x18\x04\x20\x01(\rR\x0bNIPEACDADDD\x12/\n\x0bLOIOGPDKF\
    HL\x18\x9a\x04\x20\x01(\x0b2\x0c.NMKHMIOOPJNR\x0bLOIOGPDKFHL\x12\x20\n\
    \x0bLPELHHCDBIM\x18\x0f\x20\x01(\rR\x0bLPELHHCDBIM\x12\x20\n\x0bGIKPKGEG\
    OBO\x18\x03\x20\x01(\rR\x0bGIKPKGEGOBO\x127\n\x0bFFPKLDIMFED\x18\x02\x20\
    \x01(\x0e2\x15.ChessRogueDiceStatusR\x0bFFPKLDIMFED\x12!\n\x0bEHKMBFKMIP\
    J\x18\xf6\x0b\x20\x01(\x05R\x0bEHKMBFKMIPJ\x125\n\x0bCEJHPGPJACE\x18\r\
    \x20\x01(\x0e2\x13.ChessRogueDiceTypeR\x0bCEJHPGPJACE\x12\x20\n\x0bBIBFJ\
    DMPJHM\x18\x07\x20\x01(\rR\x0bBIBFJDMPJHMb\x06proto3\
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
            deps.push(super::FOCKFLEPFCF::file_descriptor().clone());
            deps.push(super::HHEJBAAIMFE::file_descriptor().clone());
            deps.push(super::NMKHMIOOPJN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BIFPDOCGFLC::generated_message_descriptor_data());
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