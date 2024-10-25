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

//! Generated file from `JHPFGKKNOGF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JHPFGKKNOGF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JHPFGKKNOGF {
    // message fields
    // @@protoc_insertion_point(field:JHPFGKKNOGF.NGLHMFAKDKG)
    pub NGLHMFAKDKG: ::protobuf::EnumOrUnknown<super::LECFJDJMEEA::LECFJDJMEEA>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.ICGBJKDBHOE)
    pub ICGBJKDBHOE: bool,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.BNMNFEKMCNM)
    pub BNMNFEKMCNM: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.OEPFGHBAJOO)
    pub OEPFGHBAJOO: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.MKCLAPDPKDD)
    pub MKCLAPDPKDD: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.AOKHJJKMBJL)
    pub AOKHJJKMBJL: ::protobuf::MessageField<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.HBLHPOLEDGA)
    pub HBLHPOLEDGA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.MLMELOKMGBH)
    pub MLMELOKMGBH: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.HLCDOADEIJI)
    pub HLCDOADEIJI: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.ABKOHABEMMF)
    pub ABKOHABEMMF: ::protobuf::MessageField<super::ENPCFKIMJAH::ENPCFKIMJAH>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.FGGJEGFMMAO)
    pub FGGJEGFMMAO: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.JNNLBPGGEDM)
    pub JNNLBPGGEDM: ::protobuf::MessageField<super::JFHEFPLCCCJ::JFHEFPLCCCJ>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.OAKCHBDNEIC)
    pub OAKCHBDNEIC: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.GEFPJKGJLEO)
    pub GEFPJKGJLEO: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.CKDFADABICO)
    pub CKDFADABICO: ::protobuf::MessageField<super::DDBKAILCBBP::DDBKAILCBBP>,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.FILMAOEBILH)
    pub FILMAOEBILH: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.HMGHMJLLHKC)
    pub HMGHMJLLHKC: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.GAFCODOPAMF)
    pub GAFCODOPAMF: u32,
    // @@protoc_insertion_point(field:JHPFGKKNOGF.FAAMBPABPHG)
    pub FAAMBPABPHG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JHPFGKKNOGF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JHPFGKKNOGF {
    fn default() -> &'a JHPFGKKNOGF {
        <JHPFGKKNOGF as ::protobuf::Message>::default_instance()
    }
}

impl JHPFGKKNOGF {
    pub fn new() -> JHPFGKKNOGF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(19);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NGLHMFAKDKG",
            |m: &JHPFGKKNOGF| { &m.NGLHMFAKDKG },
            |m: &mut JHPFGKKNOGF| { &mut m.NGLHMFAKDKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICGBJKDBHOE",
            |m: &JHPFGKKNOGF| { &m.ICGBJKDBHOE },
            |m: &mut JHPFGKKNOGF| { &mut m.ICGBJKDBHOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNMNFEKMCNM",
            |m: &JHPFGKKNOGF| { &m.BNMNFEKMCNM },
            |m: &mut JHPFGKKNOGF| { &mut m.BNMNFEKMCNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "OEPFGHBAJOO",
            |m: &JHPFGKKNOGF| { &m.OEPFGHBAJOO },
            |m: &mut JHPFGKKNOGF| { &mut m.OEPFGHBAJOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKCLAPDPKDD",
            |m: &JHPFGKKNOGF| { &m.MKCLAPDPKDD },
            |m: &mut JHPFGKKNOGF| { &mut m.MKCLAPDPKDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJPJJEIJLLP::FJPJJEIJLLP>(
            "AOKHJJKMBJL",
            |m: &JHPFGKKNOGF| { &m.AOKHJJKMBJL },
            |m: &mut JHPFGKKNOGF| { &mut m.AOKHJJKMBJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HBLHPOLEDGA",
            |m: &JHPFGKKNOGF| { &m.HBLHPOLEDGA },
            |m: &mut JHPFGKKNOGF| { &mut m.HBLHPOLEDGA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MLMELOKMGBH",
            |m: &JHPFGKKNOGF| { &m.MLMELOKMGBH },
            |m: &mut JHPFGKKNOGF| { &mut m.MLMELOKMGBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLCDOADEIJI",
            |m: &JHPFGKKNOGF| { &m.HLCDOADEIJI },
            |m: &mut JHPFGKKNOGF| { &mut m.HLCDOADEIJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ENPCFKIMJAH::ENPCFKIMJAH>(
            "ABKOHABEMMF",
            |m: &JHPFGKKNOGF| { &m.ABKOHABEMMF },
            |m: &mut JHPFGKKNOGF| { &mut m.ABKOHABEMMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGGJEGFMMAO",
            |m: &JHPFGKKNOGF| { &m.FGGJEGFMMAO },
            |m: &mut JHPFGKKNOGF| { &mut m.FGGJEGFMMAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JFHEFPLCCCJ::JFHEFPLCCCJ>(
            "JNNLBPGGEDM",
            |m: &JHPFGKKNOGF| { &m.JNNLBPGGEDM },
            |m: &mut JHPFGKKNOGF| { &mut m.JNNLBPGGEDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OAKCHBDNEIC",
            |m: &JHPFGKKNOGF| { &m.OAKCHBDNEIC },
            |m: &mut JHPFGKKNOGF| { &mut m.OAKCHBDNEIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEFPJKGJLEO",
            |m: &JHPFGKKNOGF| { &m.GEFPJKGJLEO },
            |m: &mut JHPFGKKNOGF| { &mut m.GEFPJKGJLEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DDBKAILCBBP::DDBKAILCBBP>(
            "CKDFADABICO",
            |m: &JHPFGKKNOGF| { &m.CKDFADABICO },
            |m: &mut JHPFGKKNOGF| { &mut m.CKDFADABICO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILMAOEBILH",
            |m: &JHPFGKKNOGF| { &m.FILMAOEBILH },
            |m: &mut JHPFGKKNOGF| { &mut m.FILMAOEBILH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMGHMJLLHKC",
            |m: &JHPFGKKNOGF| { &m.HMGHMJLLHKC },
            |m: &mut JHPFGKKNOGF| { &mut m.HMGHMJLLHKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GAFCODOPAMF",
            |m: &JHPFGKKNOGF| { &m.GAFCODOPAMF },
            |m: &mut JHPFGKKNOGF| { &mut m.GAFCODOPAMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FAAMBPABPHG",
            |m: &JHPFGKKNOGF| { &m.FAAMBPABPHG },
            |m: &mut JHPFGKKNOGF| { &mut m.FAAMBPABPHG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JHPFGKKNOGF>(
            "JHPFGKKNOGF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JHPFGKKNOGF {
    const NAME: &'static str = "JHPFGKKNOGF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.NGLHMFAKDKG = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.ICGBJKDBHOE = is.read_bool()?;
                },
                9752 => {
                    self.BNMNFEKMCNM = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OEPFGHBAJOO)?;
                },
                16 => {
                    self.MKCLAPDPKDD = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AOKHJJKMBJL)?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.HBLHPOLEDGA)?;
                },
                80 => {
                    self.HBLHPOLEDGA.push(is.read_uint32()?);
                },
                24 => {
                    self.MLMELOKMGBH = is.read_uint32()?;
                },
                32 => {
                    self.HLCDOADEIJI = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ABKOHABEMMF)?;
                },
                72 => {
                    self.FGGJEGFMMAO = is.read_uint32()?;
                },
                370 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JNNLBPGGEDM)?;
                },
                56 => {
                    self.OAKCHBDNEIC = is.read_uint32()?;
                },
                12368 => {
                    self.GEFPJKGJLEO = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CKDFADABICO)?;
                },
                1536 => {
                    self.FILMAOEBILH = is.read_uint32()?;
                },
                11144 => {
                    self.HMGHMJLLHKC = is.read_uint32()?;
                },
                96 => {
                    self.GAFCODOPAMF = is.read_uint32()?;
                },
                10552 => {
                    self.FAAMBPABPHG = is.read_uint32()?;
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
        if self.NGLHMFAKDKG != ::protobuf::EnumOrUnknown::new(super::LECFJDJMEEA::LECFJDJMEEA::CHESS_ROGUE_ACCOUNT_BY_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.NGLHMFAKDKG.value());
        }
        if self.ICGBJKDBHOE != false {
            my_size += 1 + 1;
        }
        if self.BNMNFEKMCNM != 0 {
            my_size += ::protobuf::rt::uint32_size(1219, self.BNMNFEKMCNM);
        }
        if let Some(v) = self.OEPFGHBAJOO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MKCLAPDPKDD != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.MKCLAPDPKDD);
        }
        if let Some(v) = self.AOKHJJKMBJL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.HBLHPOLEDGA {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.MLMELOKMGBH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MLMELOKMGBH);
        }
        if self.HLCDOADEIJI != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.HLCDOADEIJI);
        }
        if let Some(v) = self.ABKOHABEMMF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.FGGJEGFMMAO != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FGGJEGFMMAO);
        }
        if let Some(v) = self.JNNLBPGGEDM.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OAKCHBDNEIC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.OAKCHBDNEIC);
        }
        if self.GEFPJKGJLEO != 0 {
            my_size += ::protobuf::rt::uint32_size(1546, self.GEFPJKGJLEO);
        }
        if let Some(v) = self.CKDFADABICO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.FILMAOEBILH != 0 {
            my_size += ::protobuf::rt::uint32_size(192, self.FILMAOEBILH);
        }
        if self.HMGHMJLLHKC != 0 {
            my_size += ::protobuf::rt::uint32_size(1393, self.HMGHMJLLHKC);
        }
        if self.GAFCODOPAMF != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.GAFCODOPAMF);
        }
        if self.FAAMBPABPHG != 0 {
            my_size += ::protobuf::rt::uint32_size(1319, self.FAAMBPABPHG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NGLHMFAKDKG != ::protobuf::EnumOrUnknown::new(super::LECFJDJMEEA::LECFJDJMEEA::CHESS_ROGUE_ACCOUNT_BY_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.NGLHMFAKDKG))?;
        }
        if self.ICGBJKDBHOE != false {
            os.write_bool(1, self.ICGBJKDBHOE)?;
        }
        if self.BNMNFEKMCNM != 0 {
            os.write_uint32(1219, self.BNMNFEKMCNM)?;
        }
        if let Some(v) = self.OEPFGHBAJOO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.MKCLAPDPKDD != 0 {
            os.write_uint32(2, self.MKCLAPDPKDD)?;
        }
        if let Some(v) = self.AOKHJJKMBJL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        for v in &self.HBLHPOLEDGA {
            os.write_uint32(10, *v)?;
        };
        if self.MLMELOKMGBH != 0 {
            os.write_uint32(3, self.MLMELOKMGBH)?;
        }
        if self.HLCDOADEIJI != 0 {
            os.write_uint32(4, self.HLCDOADEIJI)?;
        }
        if let Some(v) = self.ABKOHABEMMF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.FGGJEGFMMAO != 0 {
            os.write_uint32(9, self.FGGJEGFMMAO)?;
        }
        if let Some(v) = self.JNNLBPGGEDM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(46, v, os)?;
        }
        if self.OAKCHBDNEIC != 0 {
            os.write_uint32(7, self.OAKCHBDNEIC)?;
        }
        if self.GEFPJKGJLEO != 0 {
            os.write_uint32(1546, self.GEFPJKGJLEO)?;
        }
        if let Some(v) = self.CKDFADABICO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.FILMAOEBILH != 0 {
            os.write_uint32(192, self.FILMAOEBILH)?;
        }
        if self.HMGHMJLLHKC != 0 {
            os.write_uint32(1393, self.HMGHMJLLHKC)?;
        }
        if self.GAFCODOPAMF != 0 {
            os.write_uint32(12, self.GAFCODOPAMF)?;
        }
        if self.FAAMBPABPHG != 0 {
            os.write_uint32(1319, self.FAAMBPABPHG)?;
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

    fn new() -> JHPFGKKNOGF {
        JHPFGKKNOGF::new()
    }

    fn clear(&mut self) {
        self.NGLHMFAKDKG = ::protobuf::EnumOrUnknown::new(super::LECFJDJMEEA::LECFJDJMEEA::CHESS_ROGUE_ACCOUNT_BY_NONE);
        self.ICGBJKDBHOE = false;
        self.BNMNFEKMCNM = 0;
        self.OEPFGHBAJOO.clear();
        self.MKCLAPDPKDD = 0;
        self.AOKHJJKMBJL.clear();
        self.HBLHPOLEDGA.clear();
        self.MLMELOKMGBH = 0;
        self.HLCDOADEIJI = 0;
        self.ABKOHABEMMF.clear();
        self.FGGJEGFMMAO = 0;
        self.JNNLBPGGEDM.clear();
        self.OAKCHBDNEIC = 0;
        self.GEFPJKGJLEO = 0;
        self.CKDFADABICO.clear();
        self.FILMAOEBILH = 0;
        self.HMGHMJLLHKC = 0;
        self.GAFCODOPAMF = 0;
        self.FAAMBPABPHG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JHPFGKKNOGF {
        static instance: JHPFGKKNOGF = JHPFGKKNOGF {
            NGLHMFAKDKG: ::protobuf::EnumOrUnknown::from_i32(0),
            ICGBJKDBHOE: false,
            BNMNFEKMCNM: 0,
            OEPFGHBAJOO: ::protobuf::MessageField::none(),
            MKCLAPDPKDD: 0,
            AOKHJJKMBJL: ::protobuf::MessageField::none(),
            HBLHPOLEDGA: ::std::vec::Vec::new(),
            MLMELOKMGBH: 0,
            HLCDOADEIJI: 0,
            ABKOHABEMMF: ::protobuf::MessageField::none(),
            FGGJEGFMMAO: 0,
            JNNLBPGGEDM: ::protobuf::MessageField::none(),
            OAKCHBDNEIC: 0,
            GEFPJKGJLEO: 0,
            CKDFADABICO: ::protobuf::MessageField::none(),
            FILMAOEBILH: 0,
            HMGHMJLLHKC: 0,
            GAFCODOPAMF: 0,
            FAAMBPABPHG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JHPFGKKNOGF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JHPFGKKNOGF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JHPFGKKNOGF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JHPFGKKNOGF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JHPFGKKNOGF.proto\x1a\x11DDBKAILCBBP.proto\x1a\x11ENPCFKIMJAH.prot\
    o\x1a\x11FJPJJEIJLLP.proto\x1a\x0eItemList.proto\x1a\x11JFHEFPLCCCJ.prot\
    o\x1a\x11LECFJDJMEEA.proto\"\xe9\x05\n\x0bJHPFGKKNOGF\x12.\n\x0bNGLHMFAK\
    DKG\x18\x0b\x20\x01(\x0e2\x0c.LECFJDJMEEAR\x0bNGLHMFAKDKG\x12\x20\n\x0bI\
    CGBJKDBHOE\x18\x01\x20\x01(\x08R\x0bICGBJKDBHOE\x12!\n\x0bBNMNFEKMCNM\
    \x18\xc3\t\x20\x01(\rR\x0bBNMNFEKMCNM\x12+\n\x0bOEPFGHBAJOO\x18\x08\x20\
    \x01(\x0b2\t.ItemListR\x0bOEPFGHBAJOO\x12\x20\n\x0bMKCLAPDPKDD\x18\x02\
    \x20\x01(\rR\x0bMKCLAPDPKDD\x12.\n\x0bAOKHJJKMBJL\x18\x05\x20\x01(\x0b2\
    \x0c.FJPJJEIJLLPR\x0bAOKHJJKMBJL\x12\x20\n\x0bHBLHPOLEDGA\x18\n\x20\x03(\
    \rR\x0bHBLHPOLEDGA\x12\x20\n\x0bMLMELOKMGBH\x18\x03\x20\x01(\rR\x0bMLMEL\
    OKMGBH\x12\x20\n\x0bHLCDOADEIJI\x18\x04\x20\x01(\rR\x0bHLCDOADEIJI\x12.\
    \n\x0bABKOHABEMMF\x18\r\x20\x01(\x0b2\x0c.ENPCFKIMJAHR\x0bABKOHABEMMF\
    \x12\x20\n\x0bFGGJEGFMMAO\x18\t\x20\x01(\rR\x0bFGGJEGFMMAO\x12.\n\x0bJNN\
    LBPGGEDM\x18.\x20\x01(\x0b2\x0c.JFHEFPLCCCJR\x0bJNNLBPGGEDM\x12\x20\n\
    \x0bOAKCHBDNEIC\x18\x07\x20\x01(\rR\x0bOAKCHBDNEIC\x12!\n\x0bGEFPJKGJLEO\
    \x18\x8a\x0c\x20\x01(\rR\x0bGEFPJKGJLEO\x12.\n\x0bCKDFADABICO\x18\x0f\
    \x20\x01(\x0b2\x0c.DDBKAILCBBPR\x0bCKDFADABICO\x12!\n\x0bFILMAOEBILH\x18\
    \xc0\x01\x20\x01(\rR\x0bFILMAOEBILH\x12!\n\x0bHMGHMJLLHKC\x18\xf1\n\x20\
    \x01(\rR\x0bHMGHMJLLHKC\x12\x20\n\x0bGAFCODOPAMF\x18\x0c\x20\x01(\rR\x0b\
    GAFCODOPAMF\x12!\n\x0bFAAMBPABPHG\x18\xa7\n\x20\x01(\rR\x0bFAAMBPABPHGb\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::DDBKAILCBBP::file_descriptor().clone());
            deps.push(super::ENPCFKIMJAH::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::JFHEFPLCCCJ::file_descriptor().clone());
            deps.push(super::LECFJDJMEEA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JHPFGKKNOGF::generated_message_descriptor_data());
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