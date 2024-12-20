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

//! Generated file from `IJAIFMPFJDN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IJAIFMPFJDN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IJAIFMPFJDN {
    // message fields
    // @@protoc_insertion_point(field:IJAIFMPFJDN.GFKIHHOPJDG)
    pub GFKIHHOPJDG: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.IJGMJLMJABD)
    pub IJGMJLMJABD: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.PLONDJNPGIB)
    pub PLONDJNPGIB: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.OGEBFJLGGKE)
    pub OGEBFJLGGKE: ::std::vec::Vec<super::DBMIEMCPMGC::DBMIEMCPMGC>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.DCEDFKHEKIE)
    pub DCEDFKHEKIE: ::std::vec::Vec<super::IJBEDOLECAF::IJBEDOLECAF>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.LBEHAGJCLHJ)
    pub LBEHAGJCLHJ: ::std::vec::Vec<super::SceneEntityInfo::SceneEntityInfo>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.HCEJEBKDKHJ)
    pub HCEJEBKDKHJ: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.HBCAFGHLIOP)
    pub HBCAFGHLIOP: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.KBBLJPLPLBC)
    pub KBBLJPLPLBC: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.DKDFJNFHNMC)
    pub DKDFJNFHNMC: ::std::collections::HashMap<u32, super::INILDOJLMPA::INILDOJLMPA>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.LOADEBCDKEA)
    pub LOADEBCDKEA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.DBLNKHDMFNI)
    pub DBLNKHDMFNI: ::std::collections::HashMap<::std::string::String, i32>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.HPDCDPEIAHP)
    pub HPDCDPEIAHP: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.PDMLCLAJBMG)
    pub PDMLCLAJBMG: ::protobuf::MessageField<super::IMBEPFMFNDL::IMBEPFMFNDL>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.MPIAPMBCMFF)
    pub MPIAPMBCMFF: ::std::vec::Vec<super::LKNKAFKJLDG::LKNKAFKJLDG>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.BBLGKACGAOB)
    pub BBLGKACGAOB: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.PMCIJKIINJL)
    pub PMCIJKIINJL: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.KOCMDLFBKEI)
    pub KOCMDLFBKEI: u32,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.ICGHEHCDBNC)
    pub ICGHEHCDBNC: ::std::vec::Vec<super::NMCLGEKJLMO::NMCLGEKJLMO>,
    // @@protoc_insertion_point(field:IJAIFMPFJDN.NBHLMJLKEBE)
    pub NBHLMJLKEBE: ::std::vec::Vec<super::FGKNFACMFJK::FGKNFACMFJK>,
    // special fields
    // @@protoc_insertion_point(special_field:IJAIFMPFJDN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IJAIFMPFJDN {
    fn default() -> &'a IJAIFMPFJDN {
        <IJAIFMPFJDN as ::protobuf::Message>::default_instance()
    }
}

impl IJAIFMPFJDN {
    pub fn new() -> IJAIFMPFJDN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(20);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFKIHHOPJDG",
            |m: &IJAIFMPFJDN| { &m.GFKIHHOPJDG },
            |m: &mut IJAIFMPFJDN| { &mut m.GFKIHHOPJDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJGMJLMJABD",
            |m: &IJAIFMPFJDN| { &m.IJGMJLMJABD },
            |m: &mut IJAIFMPFJDN| { &mut m.IJGMJLMJABD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLONDJNPGIB",
            |m: &IJAIFMPFJDN| { &m.PLONDJNPGIB },
            |m: &mut IJAIFMPFJDN| { &mut m.PLONDJNPGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OGEBFJLGGKE",
            |m: &IJAIFMPFJDN| { &m.OGEBFJLGGKE },
            |m: &mut IJAIFMPFJDN| { &mut m.OGEBFJLGGKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DCEDFKHEKIE",
            |m: &IJAIFMPFJDN| { &m.DCEDFKHEKIE },
            |m: &mut IJAIFMPFJDN| { &mut m.DCEDFKHEKIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LBEHAGJCLHJ",
            |m: &IJAIFMPFJDN| { &m.LBEHAGJCLHJ },
            |m: &mut IJAIFMPFJDN| { &mut m.LBEHAGJCLHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCEJEBKDKHJ",
            |m: &IJAIFMPFJDN| { &m.HCEJEBKDKHJ },
            |m: &mut IJAIFMPFJDN| { &mut m.HCEJEBKDKHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBCAFGHLIOP",
            |m: &IJAIFMPFJDN| { &m.HBCAFGHLIOP },
            |m: &mut IJAIFMPFJDN| { &mut m.HBCAFGHLIOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBBLJPLPLBC",
            |m: &IJAIFMPFJDN| { &m.KBBLJPLPLBC },
            |m: &mut IJAIFMPFJDN| { &mut m.KBBLJPLPLBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "DKDFJNFHNMC",
            |m: &IJAIFMPFJDN| { &m.DKDFJNFHNMC },
            |m: &mut IJAIFMPFJDN| { &mut m.DKDFJNFHNMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LOADEBCDKEA",
            |m: &IJAIFMPFJDN| { &m.LOADEBCDKEA },
            |m: &mut IJAIFMPFJDN| { &mut m.LOADEBCDKEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "DBLNKHDMFNI",
            |m: &IJAIFMPFJDN| { &m.DBLNKHDMFNI },
            |m: &mut IJAIFMPFJDN| { &mut m.DBLNKHDMFNI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPDCDPEIAHP",
            |m: &IJAIFMPFJDN| { &m.HPDCDPEIAHP },
            |m: &mut IJAIFMPFJDN| { &mut m.HPDCDPEIAHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IMBEPFMFNDL::IMBEPFMFNDL>(
            "PDMLCLAJBMG",
            |m: &IJAIFMPFJDN| { &m.PDMLCLAJBMG },
            |m: &mut IJAIFMPFJDN| { &mut m.PDMLCLAJBMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MPIAPMBCMFF",
            |m: &IJAIFMPFJDN| { &m.MPIAPMBCMFF },
            |m: &mut IJAIFMPFJDN| { &mut m.MPIAPMBCMFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBLGKACGAOB",
            |m: &IJAIFMPFJDN| { &m.BBLGKACGAOB },
            |m: &mut IJAIFMPFJDN| { &mut m.BBLGKACGAOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMCIJKIINJL",
            |m: &IJAIFMPFJDN| { &m.PMCIJKIINJL },
            |m: &mut IJAIFMPFJDN| { &mut m.PMCIJKIINJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOCMDLFBKEI",
            |m: &IJAIFMPFJDN| { &m.KOCMDLFBKEI },
            |m: &mut IJAIFMPFJDN| { &mut m.KOCMDLFBKEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ICGHEHCDBNC",
            |m: &IJAIFMPFJDN| { &m.ICGHEHCDBNC },
            |m: &mut IJAIFMPFJDN| { &mut m.ICGHEHCDBNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NBHLMJLKEBE",
            |m: &IJAIFMPFJDN| { &m.NBHLMJLKEBE },
            |m: &mut IJAIFMPFJDN| { &mut m.NBHLMJLKEBE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IJAIFMPFJDN>(
            "IJAIFMPFJDN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IJAIFMPFJDN {
    const NAME: &'static str = "IJAIFMPFJDN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                14304 => {
                    self.GFKIHHOPJDG = is.read_uint32()?;
                },
                64 => {
                    self.IJGMJLMJABD = is.read_uint32()?;
                },
                16 => {
                    self.PLONDJNPGIB = is.read_uint32()?;
                },
                26 => {
                    self.OGEBFJLGGKE.push(is.read_message()?);
                },
                42 => {
                    self.DCEDFKHEKIE.push(is.read_message()?);
                },
                74 => {
                    self.LBEHAGJCLHJ.push(is.read_message()?);
                },
                32 => {
                    self.HCEJEBKDKHJ = is.read_uint32()?;
                },
                1616 => {
                    self.HBCAFGHLIOP = is.read_uint32()?;
                },
                56 => {
                    self.KBBLJPLPLBC = is.read_uint32()?;
                },
                90 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.DKDFJNFHNMC.insert(key, value);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.LOADEBCDKEA)?;
                },
                8 => {
                    self.LOADEBCDKEA.push(is.read_uint32()?);
                },
                5498 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            16 => value = is.read_int32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.DBLNKHDMFNI.insert(key, value);
                },
                112 => {
                    self.HPDCDPEIAHP = is.read_uint32()?;
                },
                6466 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PDMLCLAJBMG)?;
                },
                106 => {
                    self.MPIAPMBCMFF.push(is.read_message()?);
                },
                96 => {
                    self.BBLGKACGAOB = is.read_uint32()?;
                },
                14216 => {
                    self.PMCIJKIINJL = is.read_uint32()?;
                },
                120 => {
                    self.KOCMDLFBKEI = is.read_uint32()?;
                },
                1186 => {
                    self.ICGHEHCDBNC.push(is.read_message()?);
                },
                4290 => {
                    self.NBHLMJLKEBE.push(is.read_message()?);
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
        if self.GFKIHHOPJDG != 0 {
            my_size += ::protobuf::rt::uint32_size(1788, self.GFKIHHOPJDG);
        }
        if self.IJGMJLMJABD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IJGMJLMJABD);
        }
        if self.PLONDJNPGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PLONDJNPGIB);
        }
        for value in &self.OGEBFJLGGKE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.DCEDFKHEKIE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.LBEHAGJCLHJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.HCEJEBKDKHJ != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.HCEJEBKDKHJ);
        }
        if self.HBCAFGHLIOP != 0 {
            my_size += ::protobuf::rt::uint32_size(202, self.HBCAFGHLIOP);
        }
        if self.KBBLJPLPLBC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.KBBLJPLPLBC);
        }
        for (k, v) in &self.DKDFJNFHNMC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.LOADEBCDKEA {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for (k, v) in &self.DBLNKHDMFNI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.HPDCDPEIAHP != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HPDCDPEIAHP);
        }
        if let Some(v) = self.PDMLCLAJBMG.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.MPIAPMBCMFF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BBLGKACGAOB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.BBLGKACGAOB);
        }
        if self.PMCIJKIINJL != 0 {
            my_size += ::protobuf::rt::uint32_size(1777, self.PMCIJKIINJL);
        }
        if self.KOCMDLFBKEI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.KOCMDLFBKEI);
        }
        for value in &self.ICGHEHCDBNC {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.NBHLMJLKEBE {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GFKIHHOPJDG != 0 {
            os.write_uint32(1788, self.GFKIHHOPJDG)?;
        }
        if self.IJGMJLMJABD != 0 {
            os.write_uint32(8, self.IJGMJLMJABD)?;
        }
        if self.PLONDJNPGIB != 0 {
            os.write_uint32(2, self.PLONDJNPGIB)?;
        }
        for v in &self.OGEBFJLGGKE {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.DCEDFKHEKIE {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.LBEHAGJCLHJ {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.HCEJEBKDKHJ != 0 {
            os.write_uint32(4, self.HCEJEBKDKHJ)?;
        }
        if self.HBCAFGHLIOP != 0 {
            os.write_uint32(202, self.HBCAFGHLIOP)?;
        }
        if self.KBBLJPLPLBC != 0 {
            os.write_uint32(7, self.KBBLJPLPLBC)?;
        }
        for (k, v) in &self.DKDFJNFHNMC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(90)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.LOADEBCDKEA {
            os.write_uint32(1, *v)?;
        };
        for (k, v) in &self.DBLNKHDMFNI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(5498)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_int32(2, *v)?;
        };
        if self.HPDCDPEIAHP != 0 {
            os.write_uint32(14, self.HPDCDPEIAHP)?;
        }
        if let Some(v) = self.PDMLCLAJBMG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(808, v, os)?;
        }
        for v in &self.MPIAPMBCMFF {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.BBLGKACGAOB != 0 {
            os.write_uint32(12, self.BBLGKACGAOB)?;
        }
        if self.PMCIJKIINJL != 0 {
            os.write_uint32(1777, self.PMCIJKIINJL)?;
        }
        if self.KOCMDLFBKEI != 0 {
            os.write_uint32(15, self.KOCMDLFBKEI)?;
        }
        for v in &self.ICGHEHCDBNC {
            ::protobuf::rt::write_message_field_with_cached_size(148, v, os)?;
        };
        for v in &self.NBHLMJLKEBE {
            ::protobuf::rt::write_message_field_with_cached_size(536, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> IJAIFMPFJDN {
        IJAIFMPFJDN::new()
    }

    fn clear(&mut self) {
        self.GFKIHHOPJDG = 0;
        self.IJGMJLMJABD = 0;
        self.PLONDJNPGIB = 0;
        self.OGEBFJLGGKE.clear();
        self.DCEDFKHEKIE.clear();
        self.LBEHAGJCLHJ.clear();
        self.HCEJEBKDKHJ = 0;
        self.HBCAFGHLIOP = 0;
        self.KBBLJPLPLBC = 0;
        self.DKDFJNFHNMC.clear();
        self.LOADEBCDKEA.clear();
        self.DBLNKHDMFNI.clear();
        self.HPDCDPEIAHP = 0;
        self.PDMLCLAJBMG.clear();
        self.MPIAPMBCMFF.clear();
        self.BBLGKACGAOB = 0;
        self.PMCIJKIINJL = 0;
        self.KOCMDLFBKEI = 0;
        self.ICGHEHCDBNC.clear();
        self.NBHLMJLKEBE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IJAIFMPFJDN {
        static instance: ::protobuf::rt::Lazy<IJAIFMPFJDN> = ::protobuf::rt::Lazy::new();
        instance.get(IJAIFMPFJDN::new)
    }
}

impl ::protobuf::MessageFull for IJAIFMPFJDN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IJAIFMPFJDN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IJAIFMPFJDN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IJAIFMPFJDN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IJAIFMPFJDN.proto\x1a\x11DBMIEMCPMGC.proto\x1a\x11FGKNFACMFJK.prot\
    o\x1a\x11IJBEDOLECAF.proto\x1a\x11IMBEPFMFNDL.proto\x1a\x11INILDOJLMPA.p\
    roto\x1a\x11LKNKAFKJLDG.proto\x1a\x11NMCLGEKJLMO.proto\x1a\x15SceneEntit\
    yInfo.proto\"\xee\x07\n\x0bIJAIFMPFJDN\x12!\n\x0bGFKIHHOPJDG\x18\xfc\r\
    \x20\x01(\rR\x0bGFKIHHOPJDG\x12\x20\n\x0bIJGMJLMJABD\x18\x08\x20\x01(\rR\
    \x0bIJGMJLMJABD\x12\x20\n\x0bPLONDJNPGIB\x18\x02\x20\x01(\rR\x0bPLONDJNP\
    GIB\x12.\n\x0bOGEBFJLGGKE\x18\x03\x20\x03(\x0b2\x0c.DBMIEMCPMGCR\x0bOGEB\
    FJLGGKE\x12.\n\x0bDCEDFKHEKIE\x18\x05\x20\x03(\x0b2\x0c.IJBEDOLECAFR\x0b\
    DCEDFKHEKIE\x122\n\x0bLBEHAGJCLHJ\x18\t\x20\x03(\x0b2\x10.SceneEntityInf\
    oR\x0bLBEHAGJCLHJ\x12\x20\n\x0bHCEJEBKDKHJ\x18\x04\x20\x01(\rR\x0bHCEJEB\
    KDKHJ\x12!\n\x0bHBCAFGHLIOP\x18\xca\x01\x20\x01(\rR\x0bHBCAFGHLIOP\x12\
    \x20\n\x0bKBBLJPLPLBC\x18\x07\x20\x01(\rR\x0bKBBLJPLPLBC\x12?\n\x0bDKDFJ\
    NFHNMC\x18\x0b\x20\x03(\x0b2\x1d.IJAIFMPFJDN.DKDFJNFHNMCEntryR\x0bDKDFJN\
    FHNMC\x12\x20\n\x0bLOADEBCDKEA\x18\x01\x20\x03(\rR\x0bLOADEBCDKEA\x12@\n\
    \x0bDBLNKHDMFNI\x18\xaf\x05\x20\x03(\x0b2\x1d.IJAIFMPFJDN.DBLNKHDMFNIEnt\
    ryR\x0bDBLNKHDMFNI\x12\x20\n\x0bHPDCDPEIAHP\x18\x0e\x20\x01(\rR\x0bHPDCD\
    PEIAHP\x12/\n\x0bPDMLCLAJBMG\x18\xa8\x06\x20\x01(\x0b2\x0c.IMBEPFMFNDLR\
    \x0bPDMLCLAJBMG\x12.\n\x0bMPIAPMBCMFF\x18\r\x20\x03(\x0b2\x0c.LKNKAFKJLD\
    GR\x0bMPIAPMBCMFF\x12\x20\n\x0bBBLGKACGAOB\x18\x0c\x20\x01(\rR\x0bBBLGKA\
    CGAOB\x12!\n\x0bPMCIJKIINJL\x18\xf1\r\x20\x01(\rR\x0bPMCIJKIINJL\x12\x20\
    \n\x0bKOCMDLFBKEI\x18\x0f\x20\x01(\rR\x0bKOCMDLFBKEI\x12/\n\x0bICGHEHCDB\
    NC\x18\x94\x01\x20\x03(\x0b2\x0c.NMCLGEKJLMOR\x0bICGHEHCDBNC\x12/\n\x0bN\
    BHLMJLKEBE\x18\x98\x04\x20\x03(\x0b2\x0c.FGKNFACMFJKR\x0bNBHLMJLKEBE\x1a\
    L\n\x10DKDFJNFHNMCEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\
    \"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.INILDOJLMPAR\x05value:\x028\x01\
    \x1a>\n\x10DBLNKHDMFNIEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\x05R\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::DBMIEMCPMGC::file_descriptor().clone());
            deps.push(super::FGKNFACMFJK::file_descriptor().clone());
            deps.push(super::IJBEDOLECAF::file_descriptor().clone());
            deps.push(super::IMBEPFMFNDL::file_descriptor().clone());
            deps.push(super::INILDOJLMPA::file_descriptor().clone());
            deps.push(super::LKNKAFKJLDG::file_descriptor().clone());
            deps.push(super::NMCLGEKJLMO::file_descriptor().clone());
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IJAIFMPFJDN::generated_message_descriptor_data());
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
