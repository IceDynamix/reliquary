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

//! Generated file from `PPJCDCOAFDK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PPJCDCOAFDK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PPJCDCOAFDK {
    // message fields
    // @@protoc_insertion_point(field:PPJCDCOAFDK.MPEHIBKEOBE)
    pub MPEHIBKEOBE: ::std::collections::HashMap<u32, super::KEGMIHDFPMM::KEGMIHDFPMM>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.IKPKIDEIDCP)
    pub IKPKIDEIDCP: ::std::vec::Vec<super::PJKENNOFBCO::PJKENNOFBCO>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.BOKNFCIBNBM)
    pub BOKNFCIBNBM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.PDONLOOBBCI)
    pub PDONLOOBBCI: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.PIOKKPDLEHF)
    pub PIOKKPDLEHF: ::std::collections::HashMap<::std::string::String, i32>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.MHKNNLMPING)
    pub MHKNNLMPING: ::std::vec::Vec<super::AFIFEGDEGHJ::AFIFEGDEGHJ>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.GFHGLFFHFBD)
    pub GFHGLFFHFBD: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.LJHIJCABHEP)
    pub LJHIJCABHEP: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.EMDHEKKOCMD)
    pub EMDHEKKOCMD: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.CKLNKNOJIFM)
    pub CKLNKNOJIFM: ::std::vec::Vec<super::SceneEntityInfo::SceneEntityInfo>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.KMCONOKHAHJ)
    pub KMCONOKHAHJ: ::protobuf::MessageField<super::PIMIJLOANEP::PIMIJLOANEP>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.DJBIBIJMEBH)
    pub DJBIBIJMEBH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.NMCNCKKMMOD)
    pub NMCNCKKMMOD: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.GKPNKMEEIBM)
    pub GKPNKMEEIBM: ::std::vec::Vec<super::MNKDDCMJCCP::MNKDDCMJCCP>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.PPMGKACBKFA)
    pub PPMGKACBKFA: ::std::vec::Vec<super::OJFHDBNLEBM::OJFHDBNLEBM>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.ICCLNHKDBBM)
    pub ICCLNHKDBBM: ::std::vec::Vec<super::GPGGAINLLIN::GPGGAINLLIN>,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.LOLCMPAOJBG)
    pub LOLCMPAOJBG: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.CCIIHMMJOEM)
    pub CCIIHMMJOEM: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.HLICGBNDLAC)
    pub HLICGBNDLAC: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.EKGLIFOIIEA)
    pub EKGLIFOIIEA: u32,
    // @@protoc_insertion_point(field:PPJCDCOAFDK.BDOKMBIBFJB)
    pub BDOKMBIBFJB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PPJCDCOAFDK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PPJCDCOAFDK {
    fn default() -> &'a PPJCDCOAFDK {
        <PPJCDCOAFDK as ::protobuf::Message>::default_instance()
    }
}

impl PPJCDCOAFDK {
    pub fn new() -> PPJCDCOAFDK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(21);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "MPEHIBKEOBE",
            |m: &PPJCDCOAFDK| { &m.MPEHIBKEOBE },
            |m: &mut PPJCDCOAFDK| { &mut m.MPEHIBKEOBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IKPKIDEIDCP",
            |m: &PPJCDCOAFDK| { &m.IKPKIDEIDCP },
            |m: &mut PPJCDCOAFDK| { &mut m.IKPKIDEIDCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BOKNFCIBNBM",
            |m: &PPJCDCOAFDK| { &m.BOKNFCIBNBM },
            |m: &mut PPJCDCOAFDK| { &mut m.BOKNFCIBNBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDONLOOBBCI",
            |m: &PPJCDCOAFDK| { &m.PDONLOOBBCI },
            |m: &mut PPJCDCOAFDK| { &mut m.PDONLOOBBCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "PIOKKPDLEHF",
            |m: &PPJCDCOAFDK| { &m.PIOKKPDLEHF },
            |m: &mut PPJCDCOAFDK| { &mut m.PIOKKPDLEHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MHKNNLMPING",
            |m: &PPJCDCOAFDK| { &m.MHKNNLMPING },
            |m: &mut PPJCDCOAFDK| { &mut m.MHKNNLMPING },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFHGLFFHFBD",
            |m: &PPJCDCOAFDK| { &m.GFHGLFFHFBD },
            |m: &mut PPJCDCOAFDK| { &mut m.GFHGLFFHFBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJHIJCABHEP",
            |m: &PPJCDCOAFDK| { &m.LJHIJCABHEP },
            |m: &mut PPJCDCOAFDK| { &mut m.LJHIJCABHEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EMDHEKKOCMD",
            |m: &PPJCDCOAFDK| { &m.EMDHEKKOCMD },
            |m: &mut PPJCDCOAFDK| { &mut m.EMDHEKKOCMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CKLNKNOJIFM",
            |m: &PPJCDCOAFDK| { &m.CKLNKNOJIFM },
            |m: &mut PPJCDCOAFDK| { &mut m.CKLNKNOJIFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PIMIJLOANEP::PIMIJLOANEP>(
            "KMCONOKHAHJ",
            |m: &PPJCDCOAFDK| { &m.KMCONOKHAHJ },
            |m: &mut PPJCDCOAFDK| { &mut m.KMCONOKHAHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DJBIBIJMEBH",
            |m: &PPJCDCOAFDK| { &m.DJBIBIJMEBH },
            |m: &mut PPJCDCOAFDK| { &mut m.DJBIBIJMEBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMCNCKKMMOD",
            |m: &PPJCDCOAFDK| { &m.NMCNCKKMMOD },
            |m: &mut PPJCDCOAFDK| { &mut m.NMCNCKKMMOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GKPNKMEEIBM",
            |m: &PPJCDCOAFDK| { &m.GKPNKMEEIBM },
            |m: &mut PPJCDCOAFDK| { &mut m.GKPNKMEEIBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PPMGKACBKFA",
            |m: &PPJCDCOAFDK| { &m.PPMGKACBKFA },
            |m: &mut PPJCDCOAFDK| { &mut m.PPMGKACBKFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ICCLNHKDBBM",
            |m: &PPJCDCOAFDK| { &m.ICCLNHKDBBM },
            |m: &mut PPJCDCOAFDK| { &mut m.ICCLNHKDBBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOLCMPAOJBG",
            |m: &PPJCDCOAFDK| { &m.LOLCMPAOJBG },
            |m: &mut PPJCDCOAFDK| { &mut m.LOLCMPAOJBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCIIHMMJOEM",
            |m: &PPJCDCOAFDK| { &m.CCIIHMMJOEM },
            |m: &mut PPJCDCOAFDK| { &mut m.CCIIHMMJOEM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLICGBNDLAC",
            |m: &PPJCDCOAFDK| { &m.HLICGBNDLAC },
            |m: &mut PPJCDCOAFDK| { &mut m.HLICGBNDLAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKGLIFOIIEA",
            |m: &PPJCDCOAFDK| { &m.EKGLIFOIIEA },
            |m: &mut PPJCDCOAFDK| { &mut m.EKGLIFOIIEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDOKMBIBFJB",
            |m: &PPJCDCOAFDK| { &m.BDOKMBIBFJB },
            |m: &mut PPJCDCOAFDK| { &mut m.BDOKMBIBFJB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PPJCDCOAFDK>(
            "PPJCDCOAFDK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PPJCDCOAFDK {
    const NAME: &'static str = "PPJCDCOAFDK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
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
                    self.MPEHIBKEOBE.insert(key, value);
                },
                7554 => {
                    self.IKPKIDEIDCP.push(is.read_message()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.BOKNFCIBNBM)?;
                },
                24 => {
                    self.BOKNFCIBNBM.push(is.read_uint32()?);
                },
                64 => {
                    self.PDONLOOBBCI = is.read_uint32()?;
                },
                7978 => {
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
                    self.PIOKKPDLEHF.insert(key, value);
                },
                42 => {
                    self.MHKNNLMPING.push(is.read_message()?);
                },
                488 => {
                    self.GFHGLFFHFBD = is.read_uint32()?;
                },
                112 => {
                    self.LJHIJCABHEP = is.read_uint32()?;
                },
                11736 => {
                    self.EMDHEKKOCMD = is.read_uint32()?;
                },
                82 => {
                    self.CKLNKNOJIFM.push(is.read_message()?);
                },
                5706 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KMCONOKHAHJ)?;
                },
                11970 => {
                    is.read_repeated_packed_uint32_into(&mut self.DJBIBIJMEBH)?;
                },
                11968 => {
                    self.DJBIBIJMEBH.push(is.read_uint32()?);
                },
                32 => {
                    self.NMCNCKKMMOD = is.read_uint32()?;
                },
                18 => {
                    self.GKPNKMEEIBM.push(is.read_message()?);
                },
                2626 => {
                    self.PPMGKACBKFA.push(is.read_message()?);
                },
                10 => {
                    self.ICCLNHKDBBM.push(is.read_message()?);
                },
                10848 => {
                    self.LOLCMPAOJBG = is.read_uint32()?;
                },
                96 => {
                    self.CCIIHMMJOEM = is.read_uint32()?;
                },
                48 => {
                    self.HLICGBNDLAC = is.read_uint32()?;
                },
                56 => {
                    self.EKGLIFOIIEA = is.read_uint32()?;
                },
                120 => {
                    self.BDOKMBIBFJB = is.read_uint32()?;
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
        for (k, v) in &self.MPEHIBKEOBE {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.IKPKIDEIDCP {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.BOKNFCIBNBM);
        if self.PDONLOOBBCI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PDONLOOBBCI);
        }
        for (k, v) in &self.PIOKKPDLEHF {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.MHKNNLMPING {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.GFHGLFFHFBD != 0 {
            my_size += ::protobuf::rt::uint32_size(61, self.GFHGLFFHFBD);
        }
        if self.LJHIJCABHEP != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LJHIJCABHEP);
        }
        if self.EMDHEKKOCMD != 0 {
            my_size += ::protobuf::rt::uint32_size(1467, self.EMDHEKKOCMD);
        }
        for value in &self.CKLNKNOJIFM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.KMCONOKHAHJ.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1496, &self.DJBIBIJMEBH);
        if self.NMCNCKKMMOD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NMCNCKKMMOD);
        }
        for value in &self.GKPNKMEEIBM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PPMGKACBKFA {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.ICCLNHKDBBM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LOLCMPAOJBG != 0 {
            my_size += ::protobuf::rt::uint32_size(1356, self.LOLCMPAOJBG);
        }
        if self.CCIIHMMJOEM != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.CCIIHMMJOEM);
        }
        if self.HLICGBNDLAC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.HLICGBNDLAC);
        }
        if self.EKGLIFOIIEA != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.EKGLIFOIIEA);
        }
        if self.BDOKMBIBFJB != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BDOKMBIBFJB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.MPEHIBKEOBE {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.IKPKIDEIDCP {
            ::protobuf::rt::write_message_field_with_cached_size(944, v, os)?;
        };
        os.write_repeated_packed_uint32(3, &self.BOKNFCIBNBM)?;
        if self.PDONLOOBBCI != 0 {
            os.write_uint32(8, self.PDONLOOBBCI)?;
        }
        for (k, v) in &self.PIOKKPDLEHF {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(7978)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_int32(2, *v)?;
        };
        for v in &self.MHKNNLMPING {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.GFHGLFFHFBD != 0 {
            os.write_uint32(61, self.GFHGLFFHFBD)?;
        }
        if self.LJHIJCABHEP != 0 {
            os.write_uint32(14, self.LJHIJCABHEP)?;
        }
        if self.EMDHEKKOCMD != 0 {
            os.write_uint32(1467, self.EMDHEKKOCMD)?;
        }
        for v in &self.CKLNKNOJIFM {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if let Some(v) = self.KMCONOKHAHJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(713, v, os)?;
        }
        os.write_repeated_packed_uint32(1496, &self.DJBIBIJMEBH)?;
        if self.NMCNCKKMMOD != 0 {
            os.write_uint32(4, self.NMCNCKKMMOD)?;
        }
        for v in &self.GKPNKMEEIBM {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.PPMGKACBKFA {
            ::protobuf::rt::write_message_field_with_cached_size(328, v, os)?;
        };
        for v in &self.ICCLNHKDBBM {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.LOLCMPAOJBG != 0 {
            os.write_uint32(1356, self.LOLCMPAOJBG)?;
        }
        if self.CCIIHMMJOEM != 0 {
            os.write_uint32(12, self.CCIIHMMJOEM)?;
        }
        if self.HLICGBNDLAC != 0 {
            os.write_uint32(6, self.HLICGBNDLAC)?;
        }
        if self.EKGLIFOIIEA != 0 {
            os.write_uint32(7, self.EKGLIFOIIEA)?;
        }
        if self.BDOKMBIBFJB != 0 {
            os.write_uint32(15, self.BDOKMBIBFJB)?;
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

    fn new() -> PPJCDCOAFDK {
        PPJCDCOAFDK::new()
    }

    fn clear(&mut self) {
        self.MPEHIBKEOBE.clear();
        self.IKPKIDEIDCP.clear();
        self.BOKNFCIBNBM.clear();
        self.PDONLOOBBCI = 0;
        self.PIOKKPDLEHF.clear();
        self.MHKNNLMPING.clear();
        self.GFHGLFFHFBD = 0;
        self.LJHIJCABHEP = 0;
        self.EMDHEKKOCMD = 0;
        self.CKLNKNOJIFM.clear();
        self.KMCONOKHAHJ.clear();
        self.DJBIBIJMEBH.clear();
        self.NMCNCKKMMOD = 0;
        self.GKPNKMEEIBM.clear();
        self.PPMGKACBKFA.clear();
        self.ICCLNHKDBBM.clear();
        self.LOLCMPAOJBG = 0;
        self.CCIIHMMJOEM = 0;
        self.HLICGBNDLAC = 0;
        self.EKGLIFOIIEA = 0;
        self.BDOKMBIBFJB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PPJCDCOAFDK {
        static instance: ::protobuf::rt::Lazy<PPJCDCOAFDK> = ::protobuf::rt::Lazy::new();
        instance.get(PPJCDCOAFDK::new)
    }
}

impl ::protobuf::MessageFull for PPJCDCOAFDK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PPJCDCOAFDK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PPJCDCOAFDK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PPJCDCOAFDK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PPJCDCOAFDK.proto\x1a\x11AFIFEGDEGHJ.proto\x1a\x11GPGGAINLLIN.prot\
    o\x1a\x11KEGMIHDFPMM.proto\x1a\x11MNKDDCMJCCP.proto\x1a\x11OJFHDBNLEBM.p\
    roto\x1a\x11PIMIJLOANEP.proto\x1a\x11PJKENNOFBCO.proto\x1a\x15SceneEntit\
    yInfo.proto\"\x90\x08\n\x0bPPJCDCOAFDK\x12?\n\x0bMPEHIBKEOBE\x18\t\x20\
    \x03(\x0b2\x1d.PPJCDCOAFDK.MPEHIBKEOBEEntryR\x0bMPEHIBKEOBE\x12/\n\x0bIK\
    PKIDEIDCP\x18\xb0\x07\x20\x03(\x0b2\x0c.PJKENNOFBCOR\x0bIKPKIDEIDCP\x12\
    \x20\n\x0bBOKNFCIBNBM\x18\x03\x20\x03(\rR\x0bBOKNFCIBNBM\x12\x20\n\x0bPD\
    ONLOOBBCI\x18\x08\x20\x01(\rR\x0bPDONLOOBBCI\x12@\n\x0bPIOKKPDLEHF\x18\
    \xe5\x07\x20\x03(\x0b2\x1d.PPJCDCOAFDK.PIOKKPDLEHFEntryR\x0bPIOKKPDLEHF\
    \x12.\n\x0bMHKNNLMPING\x18\x05\x20\x03(\x0b2\x0c.AFIFEGDEGHJR\x0bMHKNNLM\
    PING\x12\x20\n\x0bGFHGLFFHFBD\x18=\x20\x01(\rR\x0bGFHGLFFHFBD\x12\x20\n\
    \x0bLJHIJCABHEP\x18\x0e\x20\x01(\rR\x0bLJHIJCABHEP\x12!\n\x0bEMDHEKKOCMD\
    \x18\xbb\x0b\x20\x01(\rR\x0bEMDHEKKOCMD\x122\n\x0bCKLNKNOJIFM\x18\n\x20\
    \x03(\x0b2\x10.SceneEntityInfoR\x0bCKLNKNOJIFM\x12/\n\x0bKMCONOKHAHJ\x18\
    \xc9\x05\x20\x01(\x0b2\x0c.PIMIJLOANEPR\x0bKMCONOKHAHJ\x12!\n\x0bDJBIBIJ\
    MEBH\x18\xd8\x0b\x20\x03(\rR\x0bDJBIBIJMEBH\x12\x20\n\x0bNMCNCKKMMOD\x18\
    \x04\x20\x01(\rR\x0bNMCNCKKMMOD\x12.\n\x0bGKPNKMEEIBM\x18\x02\x20\x03(\
    \x0b2\x0c.MNKDDCMJCCPR\x0bGKPNKMEEIBM\x12/\n\x0bPPMGKACBKFA\x18\xc8\x02\
    \x20\x03(\x0b2\x0c.OJFHDBNLEBMR\x0bPPMGKACBKFA\x12.\n\x0bICCLNHKDBBM\x18\
    \x01\x20\x03(\x0b2\x0c.GPGGAINLLINR\x0bICCLNHKDBBM\x12!\n\x0bLOLCMPAOJBG\
    \x18\xcc\n\x20\x01(\rR\x0bLOLCMPAOJBG\x12\x20\n\x0bCCIIHMMJOEM\x18\x0c\
    \x20\x01(\rR\x0bCCIIHMMJOEM\x12\x20\n\x0bHLICGBNDLAC\x18\x06\x20\x01(\rR\
    \x0bHLICGBNDLAC\x12\x20\n\x0bEKGLIFOIIEA\x18\x07\x20\x01(\rR\x0bEKGLIFOI\
    IEA\x12\x20\n\x0bBDOKMBIBFJB\x18\x0f\x20\x01(\rR\x0bBDOKMBIBFJB\x1aL\n\
    \x10MPEHIBKEOBEEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\"\n\
    \x05value\x18\x02\x20\x01(\x0b2\x0c.KEGMIHDFPMMR\x05value:\x028\x01\x1a>\
    \n\x10PIOKKPDLEHFEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x05R\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::AFIFEGDEGHJ::file_descriptor().clone());
            deps.push(super::GPGGAINLLIN::file_descriptor().clone());
            deps.push(super::KEGMIHDFPMM::file_descriptor().clone());
            deps.push(super::MNKDDCMJCCP::file_descriptor().clone());
            deps.push(super::OJFHDBNLEBM::file_descriptor().clone());
            deps.push(super::PIMIJLOANEP::file_descriptor().clone());
            deps.push(super::PJKENNOFBCO::file_descriptor().clone());
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PPJCDCOAFDK::generated_message_descriptor_data());
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
