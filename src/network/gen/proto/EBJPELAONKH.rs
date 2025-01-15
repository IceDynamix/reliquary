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

//! Generated file from `EBJPELAONKH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EBJPELAONKH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EBJPELAONKH {
    // message fields
    // @@protoc_insertion_point(field:EBJPELAONKH.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.BKBENAEIADB)
    pub BKBENAEIADB: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.CCBAPDGIIIF)
    pub CCBAPDGIIIF: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.PIHKHGJEPLC)
    pub PIHKHGJEPLC: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.FEAIANLIKLJ)
    pub FEAIANLIKLJ: ::std::vec::Vec<super::LCLCENKCILA::LCLCENKCILA>,
    // @@protoc_insertion_point(field:EBJPELAONKH.MKLPNKMKHND)
    pub MKLPNKMKHND: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.EJEGEHBPLOJ)
    pub EJEGEHBPLOJ: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.ABDENBLKHCE)
    pub ABDENBLKHCE: ::std::vec::Vec<super::HNPOMHNOJDP::HNPOMHNOJDP>,
    // @@protoc_insertion_point(field:EBJPELAONKH.FCKMDEHBIMM)
    pub FCKMDEHBIMM: ::std::vec::Vec<super::HNPOMHNOJDP::HNPOMHNOJDP>,
    // @@protoc_insertion_point(field:EBJPELAONKH.MOPHNJLKPEK)
    pub MOPHNJLKPEK: ::std::vec::Vec<super::HNPOMHNOJDP::HNPOMHNOJDP>,
    // @@protoc_insertion_point(field:EBJPELAONKH.NEIFGPFAIPH)
    pub NEIFGPFAIPH: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.IOFAJFHFDKI)
    pub IOFAJFHFDKI: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.NPJJPHMOEIC)
    pub NPJJPHMOEIC: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.LHAMHBPHLIL)
    pub LHAMHBPHLIL: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.EPEDMNKEPCK)
    pub EPEDMNKEPCK: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.PICMIGFBKAH)
    pub PICMIGFBKAH: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.BCFNAHGMPDH)
    pub BCFNAHGMPDH: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.BCCMHAJKIOF)
    pub BCCMHAJKIOF: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.JOOBDAIMBJE)
    pub JOOBDAIMBJE: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.OAJACKJJCGP)
    pub OAJACKJJCGP: u32,
    // @@protoc_insertion_point(field:EBJPELAONKH.JKEHIMGBDAF)
    pub JKEHIMGBDAF: f64,
    // @@protoc_insertion_point(field:EBJPELAONKH.NHPGEOMCILN)
    pub NHPGEOMCILN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EBJPELAONKH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EBJPELAONKH {
    fn default() -> &'a EBJPELAONKH {
        <EBJPELAONKH as ::protobuf::Message>::default_instance()
    }
}

impl EBJPELAONKH {
    pub fn new() -> EBJPELAONKH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(22);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &EBJPELAONKH| { &m.avatar_id },
            |m: &mut EBJPELAONKH| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKBENAEIADB",
            |m: &EBJPELAONKH| { &m.BKBENAEIADB },
            |m: &mut EBJPELAONKH| { &mut m.BKBENAEIADB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCBAPDGIIIF",
            |m: &EBJPELAONKH| { &m.CCBAPDGIIIF },
            |m: &mut EBJPELAONKH| { &mut m.CCBAPDGIIIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIHKHGJEPLC",
            |m: &EBJPELAONKH| { &m.PIHKHGJEPLC },
            |m: &mut EBJPELAONKH| { &mut m.PIHKHGJEPLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FEAIANLIKLJ",
            |m: &EBJPELAONKH| { &m.FEAIANLIKLJ },
            |m: &mut EBJPELAONKH| { &mut m.FEAIANLIKLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKLPNKMKHND",
            |m: &EBJPELAONKH| { &m.MKLPNKMKHND },
            |m: &mut EBJPELAONKH| { &mut m.MKLPNKMKHND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EJEGEHBPLOJ",
            |m: &EBJPELAONKH| { &m.EJEGEHBPLOJ },
            |m: &mut EBJPELAONKH| { &mut m.EJEGEHBPLOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ABDENBLKHCE",
            |m: &EBJPELAONKH| { &m.ABDENBLKHCE },
            |m: &mut EBJPELAONKH| { &mut m.ABDENBLKHCE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FCKMDEHBIMM",
            |m: &EBJPELAONKH| { &m.FCKMDEHBIMM },
            |m: &mut EBJPELAONKH| { &mut m.FCKMDEHBIMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MOPHNJLKPEK",
            |m: &EBJPELAONKH| { &m.MOPHNJLKPEK },
            |m: &mut EBJPELAONKH| { &mut m.MOPHNJLKPEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEIFGPFAIPH",
            |m: &EBJPELAONKH| { &m.NEIFGPFAIPH },
            |m: &mut EBJPELAONKH| { &mut m.NEIFGPFAIPH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOFAJFHFDKI",
            |m: &EBJPELAONKH| { &m.IOFAJFHFDKI },
            |m: &mut EBJPELAONKH| { &mut m.IOFAJFHFDKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPJJPHMOEIC",
            |m: &EBJPELAONKH| { &m.NPJJPHMOEIC },
            |m: &mut EBJPELAONKH| { &mut m.NPJJPHMOEIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHAMHBPHLIL",
            |m: &EBJPELAONKH| { &m.LHAMHBPHLIL },
            |m: &mut EBJPELAONKH| { &mut m.LHAMHBPHLIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPEDMNKEPCK",
            |m: &EBJPELAONKH| { &m.EPEDMNKEPCK },
            |m: &mut EBJPELAONKH| { &mut m.EPEDMNKEPCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PICMIGFBKAH",
            |m: &EBJPELAONKH| { &m.PICMIGFBKAH },
            |m: &mut EBJPELAONKH| { &mut m.PICMIGFBKAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCFNAHGMPDH",
            |m: &EBJPELAONKH| { &m.BCFNAHGMPDH },
            |m: &mut EBJPELAONKH| { &mut m.BCFNAHGMPDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCCMHAJKIOF",
            |m: &EBJPELAONKH| { &m.BCCMHAJKIOF },
            |m: &mut EBJPELAONKH| { &mut m.BCCMHAJKIOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JOOBDAIMBJE",
            |m: &EBJPELAONKH| { &m.JOOBDAIMBJE },
            |m: &mut EBJPELAONKH| { &mut m.JOOBDAIMBJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OAJACKJJCGP",
            |m: &EBJPELAONKH| { &m.OAJACKJJCGP },
            |m: &mut EBJPELAONKH| { &mut m.OAJACKJJCGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKEHIMGBDAF",
            |m: &EBJPELAONKH| { &m.JKEHIMGBDAF },
            |m: &mut EBJPELAONKH| { &mut m.JKEHIMGBDAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHPGEOMCILN",
            |m: &EBJPELAONKH| { &m.NHPGEOMCILN },
            |m: &mut EBJPELAONKH| { &mut m.NHPGEOMCILN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EBJPELAONKH>(
            "EBJPELAONKH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EBJPELAONKH {
    const NAME: &'static str = "EBJPELAONKH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                16 => {
                    self.BKBENAEIADB = is.read_uint32()?;
                },
                24 => {
                    self.CCBAPDGIIIF = is.read_uint32()?;
                },
                33 => {
                    self.PIHKHGJEPLC = is.read_double()?;
                },
                42 => {
                    self.FEAIANLIKLJ.push(is.read_message()?);
                },
                49 => {
                    self.MKLPNKMKHND = is.read_double()?;
                },
                57 => {
                    self.EJEGEHBPLOJ = is.read_double()?;
                },
                66 => {
                    self.ABDENBLKHCE.push(is.read_message()?);
                },
                74 => {
                    self.FCKMDEHBIMM.push(is.read_message()?);
                },
                82 => {
                    self.MOPHNJLKPEK.push(is.read_message()?);
                },
                89 => {
                    self.NEIFGPFAIPH = is.read_double()?;
                },
                97 => {
                    self.IOFAJFHFDKI = is.read_double()?;
                },
                105 => {
                    self.NPJJPHMOEIC = is.read_double()?;
                },
                113 => {
                    self.LHAMHBPHLIL = is.read_double()?;
                },
                121 => {
                    self.EPEDMNKEPCK = is.read_double()?;
                },
                129 => {
                    self.PICMIGFBKAH = is.read_double()?;
                },
                136 => {
                    self.BCFNAHGMPDH = is.read_uint32()?;
                },
                144 => {
                    self.BCCMHAJKIOF = is.read_uint32()?;
                },
                152 => {
                    self.JOOBDAIMBJE = is.read_uint32()?;
                },
                160 => {
                    self.OAJACKJJCGP = is.read_uint32()?;
                },
                169 => {
                    self.JKEHIMGBDAF = is.read_double()?;
                },
                176 => {
                    self.NHPGEOMCILN = is.read_uint32()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        if self.BKBENAEIADB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.BKBENAEIADB);
        }
        if self.CCBAPDGIIIF != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.CCBAPDGIIIF);
        }
        if self.PIHKHGJEPLC != 0. {
            my_size += 1 + 8;
        }
        for value in &self.FEAIANLIKLJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MKLPNKMKHND != 0. {
            my_size += 1 + 8;
        }
        if self.EJEGEHBPLOJ != 0. {
            my_size += 1 + 8;
        }
        for value in &self.ABDENBLKHCE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.FCKMDEHBIMM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MOPHNJLKPEK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NEIFGPFAIPH != 0. {
            my_size += 1 + 8;
        }
        if self.IOFAJFHFDKI != 0. {
            my_size += 1 + 8;
        }
        if self.NPJJPHMOEIC != 0. {
            my_size += 1 + 8;
        }
        if self.LHAMHBPHLIL != 0. {
            my_size += 1 + 8;
        }
        if self.EPEDMNKEPCK != 0. {
            my_size += 1 + 8;
        }
        if self.PICMIGFBKAH != 0. {
            my_size += 2 + 8;
        }
        if self.BCFNAHGMPDH != 0 {
            my_size += ::protobuf::rt::uint32_size(17, self.BCFNAHGMPDH);
        }
        if self.BCCMHAJKIOF != 0 {
            my_size += ::protobuf::rt::uint32_size(18, self.BCCMHAJKIOF);
        }
        if self.JOOBDAIMBJE != 0 {
            my_size += ::protobuf::rt::uint32_size(19, self.JOOBDAIMBJE);
        }
        if self.OAJACKJJCGP != 0 {
            my_size += ::protobuf::rt::uint32_size(20, self.OAJACKJJCGP);
        }
        if self.JKEHIMGBDAF != 0. {
            my_size += 2 + 8;
        }
        if self.NHPGEOMCILN != 0 {
            my_size += ::protobuf::rt::uint32_size(22, self.NHPGEOMCILN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        if self.BKBENAEIADB != 0 {
            os.write_uint32(2, self.BKBENAEIADB)?;
        }
        if self.CCBAPDGIIIF != 0 {
            os.write_uint32(3, self.CCBAPDGIIIF)?;
        }
        if self.PIHKHGJEPLC != 0. {
            os.write_double(4, self.PIHKHGJEPLC)?;
        }
        for v in &self.FEAIANLIKLJ {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.MKLPNKMKHND != 0. {
            os.write_double(6, self.MKLPNKMKHND)?;
        }
        if self.EJEGEHBPLOJ != 0. {
            os.write_double(7, self.EJEGEHBPLOJ)?;
        }
        for v in &self.ABDENBLKHCE {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.FCKMDEHBIMM {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.MOPHNJLKPEK {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.NEIFGPFAIPH != 0. {
            os.write_double(11, self.NEIFGPFAIPH)?;
        }
        if self.IOFAJFHFDKI != 0. {
            os.write_double(12, self.IOFAJFHFDKI)?;
        }
        if self.NPJJPHMOEIC != 0. {
            os.write_double(13, self.NPJJPHMOEIC)?;
        }
        if self.LHAMHBPHLIL != 0. {
            os.write_double(14, self.LHAMHBPHLIL)?;
        }
        if self.EPEDMNKEPCK != 0. {
            os.write_double(15, self.EPEDMNKEPCK)?;
        }
        if self.PICMIGFBKAH != 0. {
            os.write_double(16, self.PICMIGFBKAH)?;
        }
        if self.BCFNAHGMPDH != 0 {
            os.write_uint32(17, self.BCFNAHGMPDH)?;
        }
        if self.BCCMHAJKIOF != 0 {
            os.write_uint32(18, self.BCCMHAJKIOF)?;
        }
        if self.JOOBDAIMBJE != 0 {
            os.write_uint32(19, self.JOOBDAIMBJE)?;
        }
        if self.OAJACKJJCGP != 0 {
            os.write_uint32(20, self.OAJACKJJCGP)?;
        }
        if self.JKEHIMGBDAF != 0. {
            os.write_double(21, self.JKEHIMGBDAF)?;
        }
        if self.NHPGEOMCILN != 0 {
            os.write_uint32(22, self.NHPGEOMCILN)?;
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

    fn new() -> EBJPELAONKH {
        EBJPELAONKH::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.BKBENAEIADB = 0;
        self.CCBAPDGIIIF = 0;
        self.PIHKHGJEPLC = 0.;
        self.FEAIANLIKLJ.clear();
        self.MKLPNKMKHND = 0.;
        self.EJEGEHBPLOJ = 0.;
        self.ABDENBLKHCE.clear();
        self.FCKMDEHBIMM.clear();
        self.MOPHNJLKPEK.clear();
        self.NEIFGPFAIPH = 0.;
        self.IOFAJFHFDKI = 0.;
        self.NPJJPHMOEIC = 0.;
        self.LHAMHBPHLIL = 0.;
        self.EPEDMNKEPCK = 0.;
        self.PICMIGFBKAH = 0.;
        self.BCFNAHGMPDH = 0;
        self.BCCMHAJKIOF = 0;
        self.JOOBDAIMBJE = 0;
        self.OAJACKJJCGP = 0;
        self.JKEHIMGBDAF = 0.;
        self.NHPGEOMCILN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EBJPELAONKH {
        static instance: EBJPELAONKH = EBJPELAONKH {
            avatar_id: 0,
            BKBENAEIADB: 0,
            CCBAPDGIIIF: 0,
            PIHKHGJEPLC: 0.,
            FEAIANLIKLJ: ::std::vec::Vec::new(),
            MKLPNKMKHND: 0.,
            EJEGEHBPLOJ: 0.,
            ABDENBLKHCE: ::std::vec::Vec::new(),
            FCKMDEHBIMM: ::std::vec::Vec::new(),
            MOPHNJLKPEK: ::std::vec::Vec::new(),
            NEIFGPFAIPH: 0.,
            IOFAJFHFDKI: 0.,
            NPJJPHMOEIC: 0.,
            LHAMHBPHLIL: 0.,
            EPEDMNKEPCK: 0.,
            PICMIGFBKAH: 0.,
            BCFNAHGMPDH: 0,
            BCCMHAJKIOF: 0,
            JOOBDAIMBJE: 0,
            OAJACKJJCGP: 0,
            JKEHIMGBDAF: 0.,
            NHPGEOMCILN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EBJPELAONKH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EBJPELAONKH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EBJPELAONKH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EBJPELAONKH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EBJPELAONKH.proto\x1a\x11HNPOMHNOJDP.proto\x1a\x11LCLCENKCILA.prot\
    o\"\xac\x06\n\x0bEBJPELAONKH\x12\x1b\n\tavatar_id\x18\x01\x20\x01(\rR\
    \x08avatarId\x12\x20\n\x0bBKBENAEIADB\x18\x02\x20\x01(\rR\x0bBKBENAEIADB\
    \x12\x20\n\x0bCCBAPDGIIIF\x18\x03\x20\x01(\rR\x0bCCBAPDGIIIF\x12\x20\n\
    \x0bPIHKHGJEPLC\x18\x04\x20\x01(\x01R\x0bPIHKHGJEPLC\x12.\n\x0bFEAIANLIK\
    LJ\x18\x05\x20\x03(\x0b2\x0c.LCLCENKCILAR\x0bFEAIANLIKLJ\x12\x20\n\x0bMK\
    LPNKMKHND\x18\x06\x20\x01(\x01R\x0bMKLPNKMKHND\x12\x20\n\x0bEJEGEHBPLOJ\
    \x18\x07\x20\x01(\x01R\x0bEJEGEHBPLOJ\x12.\n\x0bABDENBLKHCE\x18\x08\x20\
    \x03(\x0b2\x0c.HNPOMHNOJDPR\x0bABDENBLKHCE\x12.\n\x0bFCKMDEHBIMM\x18\t\
    \x20\x03(\x0b2\x0c.HNPOMHNOJDPR\x0bFCKMDEHBIMM\x12.\n\x0bMOPHNJLKPEK\x18\
    \n\x20\x03(\x0b2\x0c.HNPOMHNOJDPR\x0bMOPHNJLKPEK\x12\x20\n\x0bNEIFGPFAIP\
    H\x18\x0b\x20\x01(\x01R\x0bNEIFGPFAIPH\x12\x20\n\x0bIOFAJFHFDKI\x18\x0c\
    \x20\x01(\x01R\x0bIOFAJFHFDKI\x12\x20\n\x0bNPJJPHMOEIC\x18\r\x20\x01(\
    \x01R\x0bNPJJPHMOEIC\x12\x20\n\x0bLHAMHBPHLIL\x18\x0e\x20\x01(\x01R\x0bL\
    HAMHBPHLIL\x12\x20\n\x0bEPEDMNKEPCK\x18\x0f\x20\x01(\x01R\x0bEPEDMNKEPCK\
    \x12\x20\n\x0bPICMIGFBKAH\x18\x10\x20\x01(\x01R\x0bPICMIGFBKAH\x12\x20\n\
    \x0bBCFNAHGMPDH\x18\x11\x20\x01(\rR\x0bBCFNAHGMPDH\x12\x20\n\x0bBCCMHAJK\
    IOF\x18\x12\x20\x01(\rR\x0bBCCMHAJKIOF\x12\x20\n\x0bJOOBDAIMBJE\x18\x13\
    \x20\x01(\rR\x0bJOOBDAIMBJE\x12\x20\n\x0bOAJACKJJCGP\x18\x14\x20\x01(\rR\
    \x0bOAJACKJJCGP\x12\x20\n\x0bJKEHIMGBDAF\x18\x15\x20\x01(\x01R\x0bJKEHIM\
    GBDAF\x12\x20\n\x0bNHPGEOMCILN\x18\x16\x20\x01(\rR\x0bNHPGEOMCILNb\x06pr\
    oto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::HNPOMHNOJDP::file_descriptor().clone());
            deps.push(super::LCLCENKCILA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EBJPELAONKH::generated_message_descriptor_data());
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