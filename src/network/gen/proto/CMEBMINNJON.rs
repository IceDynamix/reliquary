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

//! Generated file from `CMEBMINNJON.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CMEBMINNJON)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CMEBMINNJON {
    // message fields
    // @@protoc_insertion_point(field:CMEBMINNJON.HIAGEINLAHL)
    pub HIAGEINLAHL: u32,
    // message oneof groups
    pub FGNOIDAFAJL: ::std::option::Option<cmebminnjon::FGNOIDAFAJL>,
    // special fields
    // @@protoc_insertion_point(special_field:CMEBMINNJON.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CMEBMINNJON {
    fn default() -> &'a CMEBMINNJON {
        <CMEBMINNJON as ::protobuf::Message>::default_instance()
    }
}

impl CMEBMINNJON {
    pub fn new() -> CMEBMINNJON {
        ::std::default::Default::default()
    }

    // .DCMFOFEBFAK FBBEHFHPOPD = 9;

    pub fn FBBEHFHPOPD(&self) -> &super::DCMFOFEBFAK::DCMFOFEBFAK {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(ref v)) => v,
            _ => <super::DCMFOFEBFAK::DCMFOFEBFAK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FBBEHFHPOPD(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_FBBEHFHPOPD(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FBBEHFHPOPD(&mut self, v: super::DCMFOFEBFAK::DCMFOFEBFAK) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FBBEHFHPOPD(&mut self) -> &mut super::DCMFOFEBFAK::DCMFOFEBFAK {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(super::DCMFOFEBFAK::DCMFOFEBFAK::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FBBEHFHPOPD(&mut self) -> super::DCMFOFEBFAK::DCMFOFEBFAK {
        if self.has_FBBEHFHPOPD() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DCMFOFEBFAK::DCMFOFEBFAK::new()
        }
    }

    // .DGPMABEJOMJ NGBMDKDKBGG = 13;

    pub fn NGBMDKDKBGG(&self) -> &super::DGPMABEJOMJ::DGPMABEJOMJ {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(ref v)) => v,
            _ => <super::DGPMABEJOMJ::DGPMABEJOMJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NGBMDKDKBGG(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_NGBMDKDKBGG(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NGBMDKDKBGG(&mut self, v: super::DGPMABEJOMJ::DGPMABEJOMJ) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NGBMDKDKBGG(&mut self) -> &mut super::DGPMABEJOMJ::DGPMABEJOMJ {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(super::DGPMABEJOMJ::DGPMABEJOMJ::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NGBMDKDKBGG(&mut self) -> super::DGPMABEJOMJ::DGPMABEJOMJ {
        if self.has_NGBMDKDKBGG() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DGPMABEJOMJ::DGPMABEJOMJ::new()
        }
    }

    // .ELBENNKHNLC IMFFNDDABLB = 8;

    pub fn IMFFNDDABLB(&self) -> &super::ELBENNKHNLC::ELBENNKHNLC {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(ref v)) => v,
            _ => <super::ELBENNKHNLC::ELBENNKHNLC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IMFFNDDABLB(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_IMFFNDDABLB(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IMFFNDDABLB(&mut self, v: super::ELBENNKHNLC::ELBENNKHNLC) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IMFFNDDABLB(&mut self) -> &mut super::ELBENNKHNLC::ELBENNKHNLC {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(super::ELBENNKHNLC::ELBENNKHNLC::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IMFFNDDABLB(&mut self) -> super::ELBENNKHNLC::ELBENNKHNLC {
        if self.has_IMFFNDDABLB() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ELBENNKHNLC::ELBENNKHNLC::new()
        }
    }

    // .IDELAMMCOII FCFHJBPPBLO = 10;

    pub fn FCFHJBPPBLO(&self) -> &super::IDELAMMCOII::IDELAMMCOII {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(ref v)) => v,
            _ => <super::IDELAMMCOII::IDELAMMCOII as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FCFHJBPPBLO(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_FCFHJBPPBLO(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FCFHJBPPBLO(&mut self, v: super::IDELAMMCOII::IDELAMMCOII) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FCFHJBPPBLO(&mut self) -> &mut super::IDELAMMCOII::IDELAMMCOII {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(super::IDELAMMCOII::IDELAMMCOII::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FCFHJBPPBLO(&mut self) -> super::IDELAMMCOII::IDELAMMCOII {
        if self.has_FCFHJBPPBLO() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IDELAMMCOII::IDELAMMCOII::new()
        }
    }

    // .DKPPCLLGIMC FLJFEHGOBDB = 6;

    pub fn FLJFEHGOBDB(&self) -> &super::DKPPCLLGIMC::DKPPCLLGIMC {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(ref v)) => v,
            _ => <super::DKPPCLLGIMC::DKPPCLLGIMC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FLJFEHGOBDB(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_FLJFEHGOBDB(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FLJFEHGOBDB(&mut self, v: super::DKPPCLLGIMC::DKPPCLLGIMC) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FLJFEHGOBDB(&mut self) -> &mut super::DKPPCLLGIMC::DKPPCLLGIMC {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(super::DKPPCLLGIMC::DKPPCLLGIMC::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FLJFEHGOBDB(&mut self) -> super::DKPPCLLGIMC::DKPPCLLGIMC {
        if self.has_FLJFEHGOBDB() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DKPPCLLGIMC::DKPPCLLGIMC::new()
        }
    }

    // .FAFOJMLECPG MCBGNPDEOBN = 7;

    pub fn MCBGNPDEOBN(&self) -> &super::FAFOJMLECPG::FAFOJMLECPG {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(ref v)) => v,
            _ => <super::FAFOJMLECPG::FAFOJMLECPG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MCBGNPDEOBN(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_MCBGNPDEOBN(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MCBGNPDEOBN(&mut self, v: super::FAFOJMLECPG::FAFOJMLECPG) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MCBGNPDEOBN(&mut self) -> &mut super::FAFOJMLECPG::FAFOJMLECPG {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(super::FAFOJMLECPG::FAFOJMLECPG::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MCBGNPDEOBN(&mut self) -> super::FAFOJMLECPG::FAFOJMLECPG {
        if self.has_MCBGNPDEOBN() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FAFOJMLECPG::FAFOJMLECPG::new()
        }
    }

    // .CMNFMIHMGID AGPJDEJALFH = 15;

    pub fn AGPJDEJALFH(&self) -> &super::CMNFMIHMGID::CMNFMIHMGID {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(ref v)) => v,
            _ => <super::CMNFMIHMGID::CMNFMIHMGID as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AGPJDEJALFH(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_AGPJDEJALFH(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AGPJDEJALFH(&mut self, v: super::CMNFMIHMGID::CMNFMIHMGID) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AGPJDEJALFH(&mut self) -> &mut super::CMNFMIHMGID::CMNFMIHMGID {
        if let ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(_)) = self.FGNOIDAFAJL {
        } else {
            self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(super::CMNFMIHMGID::CMNFMIHMGID::new()));
        }
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AGPJDEJALFH(&mut self) -> super::CMNFMIHMGID::CMNFMIHMGID {
        if self.has_AGPJDEJALFH() {
            match self.FGNOIDAFAJL.take() {
                ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CMNFMIHMGID::CMNFMIHMGID::new()
        }
    }

    // bool AEKEHEMKPID = 12;

    pub fn AEKEHEMKPID(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(v)) => v,
            _ => false,
        }
    }

    pub fn clear_AEKEHEMKPID(&mut self) {
        self.FGNOIDAFAJL = ::std::option::Option::None;
    }

    pub fn has_AEKEHEMKPID(&self) -> bool {
        match self.FGNOIDAFAJL {
            ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AEKEHEMKPID(&mut self, v: bool) {
        self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIAGEINLAHL",
            |m: &CMEBMINNJON| { &m.HIAGEINLAHL },
            |m: &mut CMEBMINNJON| { &mut m.HIAGEINLAHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DCMFOFEBFAK::DCMFOFEBFAK>(
            "FBBEHFHPOPD",
            CMEBMINNJON::has_FBBEHFHPOPD,
            CMEBMINNJON::FBBEHFHPOPD,
            CMEBMINNJON::mut_FBBEHFHPOPD,
            CMEBMINNJON::set_FBBEHFHPOPD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DGPMABEJOMJ::DGPMABEJOMJ>(
            "NGBMDKDKBGG",
            CMEBMINNJON::has_NGBMDKDKBGG,
            CMEBMINNJON::NGBMDKDKBGG,
            CMEBMINNJON::mut_NGBMDKDKBGG,
            CMEBMINNJON::set_NGBMDKDKBGG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ELBENNKHNLC::ELBENNKHNLC>(
            "IMFFNDDABLB",
            CMEBMINNJON::has_IMFFNDDABLB,
            CMEBMINNJON::IMFFNDDABLB,
            CMEBMINNJON::mut_IMFFNDDABLB,
            CMEBMINNJON::set_IMFFNDDABLB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IDELAMMCOII::IDELAMMCOII>(
            "FCFHJBPPBLO",
            CMEBMINNJON::has_FCFHJBPPBLO,
            CMEBMINNJON::FCFHJBPPBLO,
            CMEBMINNJON::mut_FCFHJBPPBLO,
            CMEBMINNJON::set_FCFHJBPPBLO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DKPPCLLGIMC::DKPPCLLGIMC>(
            "FLJFEHGOBDB",
            CMEBMINNJON::has_FLJFEHGOBDB,
            CMEBMINNJON::FLJFEHGOBDB,
            CMEBMINNJON::mut_FLJFEHGOBDB,
            CMEBMINNJON::set_FLJFEHGOBDB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FAFOJMLECPG::FAFOJMLECPG>(
            "MCBGNPDEOBN",
            CMEBMINNJON::has_MCBGNPDEOBN,
            CMEBMINNJON::MCBGNPDEOBN,
            CMEBMINNJON::mut_MCBGNPDEOBN,
            CMEBMINNJON::set_MCBGNPDEOBN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CMNFMIHMGID::CMNFMIHMGID>(
            "AGPJDEJALFH",
            CMEBMINNJON::has_AGPJDEJALFH,
            CMEBMINNJON::AGPJDEJALFH,
            CMEBMINNJON::mut_AGPJDEJALFH,
            CMEBMINNJON::set_AGPJDEJALFH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "AEKEHEMKPID",
            CMEBMINNJON::has_AEKEHEMKPID,
            CMEBMINNJON::AEKEHEMKPID,
            CMEBMINNJON::set_AEKEHEMKPID,
        ));
        oneofs.push(cmebminnjon::FGNOIDAFAJL::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CMEBMINNJON>(
            "CMEBMINNJON",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CMEBMINNJON {
    const NAME: &'static str = "CMEBMINNJON";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.HIAGEINLAHL = is.read_uint32()?;
                },
                74 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(is.read_message()?));
                },
                106 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(is.read_message()?));
                },
                66 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(is.read_message()?));
                },
                82 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(is.read_message()?));
                },
                50 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(is.read_message()?));
                },
                58 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(is.read_message()?));
                },
                122 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(is.read_message()?));
                },
                96 => {
                    self.FGNOIDAFAJL = ::std::option::Option::Some(cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(is.read_bool()?));
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
        if self.HIAGEINLAHL != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.HIAGEINLAHL);
        }
        if let ::std::option::Option::Some(ref v) = self.FGNOIDAFAJL {
            match v {
                &cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(v) => {
                    my_size += 1 + 1;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HIAGEINLAHL != 0 {
            os.write_uint32(5, self.HIAGEINLAHL)?;
        }
        if let ::std::option::Option::Some(ref v) = self.FGNOIDAFAJL {
            match v {
                &cmebminnjon::FGNOIDAFAJL::FBBEHFHPOPD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::NGBMDKDKBGG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::IMFFNDDABLB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::FCFHJBPPBLO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::FLJFEHGOBDB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::MCBGNPDEOBN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::AGPJDEJALFH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &cmebminnjon::FGNOIDAFAJL::AEKEHEMKPID(v) => {
                    os.write_bool(12, v)?;
                },
            };
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

    fn new() -> CMEBMINNJON {
        CMEBMINNJON::new()
    }

    fn clear(&mut self) {
        self.HIAGEINLAHL = 0;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.FGNOIDAFAJL = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CMEBMINNJON {
        static instance: CMEBMINNJON = CMEBMINNJON {
            HIAGEINLAHL: 0,
            FGNOIDAFAJL: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CMEBMINNJON {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CMEBMINNJON").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CMEBMINNJON {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMEBMINNJON {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CMEBMINNJON`
pub mod cmebminnjon {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CMEBMINNJON.FGNOIDAFAJL)
    pub enum FGNOIDAFAJL {
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.FBBEHFHPOPD)
        FBBEHFHPOPD(super::super::DCMFOFEBFAK::DCMFOFEBFAK),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.NGBMDKDKBGG)
        NGBMDKDKBGG(super::super::DGPMABEJOMJ::DGPMABEJOMJ),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.IMFFNDDABLB)
        IMFFNDDABLB(super::super::ELBENNKHNLC::ELBENNKHNLC),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.FCFHJBPPBLO)
        FCFHJBPPBLO(super::super::IDELAMMCOII::IDELAMMCOII),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.FLJFEHGOBDB)
        FLJFEHGOBDB(super::super::DKPPCLLGIMC::DKPPCLLGIMC),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.MCBGNPDEOBN)
        MCBGNPDEOBN(super::super::FAFOJMLECPG::FAFOJMLECPG),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.AGPJDEJALFH)
        AGPJDEJALFH(super::super::CMNFMIHMGID::CMNFMIHMGID),
        // @@protoc_insertion_point(oneof_field:CMEBMINNJON.AEKEHEMKPID)
        AEKEHEMKPID(bool),
    }

    impl ::protobuf::Oneof for FGNOIDAFAJL {
    }

    impl ::protobuf::OneofFull for FGNOIDAFAJL {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CMEBMINNJON as ::protobuf::MessageFull>::descriptor().oneof_by_name("FGNOIDAFAJL").unwrap()).clone()
        }
    }

    impl FGNOIDAFAJL {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<FGNOIDAFAJL>("FGNOIDAFAJL")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CMEBMINNJON.proto\x1a\x11CMNFMIHMGID.proto\x1a\x11DCMFOFEBFAK.prot\
    o\x1a\x11DGPMABEJOMJ.proto\x1a\x11DKPPCLLGIMC.proto\x1a\x11ELBENNKHNLC.p\
    roto\x1a\x11FAFOJMLECPG.proto\x1a\x11IDELAMMCOII.proto\"\xc0\x03\n\x0bCM\
    EBMINNJON\x12\x20\n\x0bHIAGEINLAHL\x18\x05\x20\x01(\rR\x0bHIAGEINLAHL\
    \x120\n\x0bFBBEHFHPOPD\x18\t\x20\x01(\x0b2\x0c.DCMFOFEBFAKH\0R\x0bFBBEHF\
    HPOPD\x120\n\x0bNGBMDKDKBGG\x18\r\x20\x01(\x0b2\x0c.DGPMABEJOMJH\0R\x0bN\
    GBMDKDKBGG\x120\n\x0bIMFFNDDABLB\x18\x08\x20\x01(\x0b2\x0c.ELBENNKHNLCH\
    \0R\x0bIMFFNDDABLB\x120\n\x0bFCFHJBPPBLO\x18\n\x20\x01(\x0b2\x0c.IDELAMM\
    COIIH\0R\x0bFCFHJBPPBLO\x120\n\x0bFLJFEHGOBDB\x18\x06\x20\x01(\x0b2\x0c.\
    DKPPCLLGIMCH\0R\x0bFLJFEHGOBDB\x120\n\x0bMCBGNPDEOBN\x18\x07\x20\x01(\
    \x0b2\x0c.FAFOJMLECPGH\0R\x0bMCBGNPDEOBN\x120\n\x0bAGPJDEJALFH\x18\x0f\
    \x20\x01(\x0b2\x0c.CMNFMIHMGIDH\0R\x0bAGPJDEJALFH\x12\"\n\x0bAEKEHEMKPID\
    \x18\x0c\x20\x01(\x08H\0R\x0bAEKEHEMKPIDB\r\n\x0bFGNOIDAFAJLb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(7);
            deps.push(super::CMNFMIHMGID::file_descriptor().clone());
            deps.push(super::DCMFOFEBFAK::file_descriptor().clone());
            deps.push(super::DGPMABEJOMJ::file_descriptor().clone());
            deps.push(super::DKPPCLLGIMC::file_descriptor().clone());
            deps.push(super::ELBENNKHNLC::file_descriptor().clone());
            deps.push(super::FAFOJMLECPG::file_descriptor().clone());
            deps.push(super::IDELAMMCOII::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CMEBMINNJON::generated_message_descriptor_data());
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
