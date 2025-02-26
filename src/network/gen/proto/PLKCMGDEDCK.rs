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

//! Generated file from `PLKCMGDEDCK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PLKCMGDEDCK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PLKCMGDEDCK {
    // message fields
    // @@protoc_insertion_point(field:PLKCMGDEDCK.IGCIIKMMDAI)
    pub IGCIIKMMDAI: ::protobuf::EnumOrUnknown<super::CBEJAJENOHJ::CBEJAJENOHJ>,
    // message oneof groups
    pub PPHBGCABELF: ::std::option::Option<plkcmgdedck::PPHBGCABELF>,
    // special fields
    // @@protoc_insertion_point(special_field:PLKCMGDEDCK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PLKCMGDEDCK {
    fn default() -> &'a PLKCMGDEDCK {
        <PLKCMGDEDCK as ::protobuf::Message>::default_instance()
    }
}

impl PLKCMGDEDCK {
    pub fn new() -> PLKCMGDEDCK {
        ::std::default::Default::default()
    }

    // .HBCINIKPAFI LNMMKFMEAJM = 1409;

    pub fn LNMMKFMEAJM(&self) -> &super::HBCINIKPAFI::HBCINIKPAFI {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(ref v)) => v,
            _ => <super::HBCINIKPAFI::HBCINIKPAFI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LNMMKFMEAJM(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_LNMMKFMEAJM(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LNMMKFMEAJM(&mut self, v: super::HBCINIKPAFI::HBCINIKPAFI) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LNMMKFMEAJM(&mut self) -> &mut super::HBCINIKPAFI::HBCINIKPAFI {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(super::HBCINIKPAFI::HBCINIKPAFI::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LNMMKFMEAJM(&mut self) -> super::HBCINIKPAFI::HBCINIKPAFI {
        if self.has_LNMMKFMEAJM() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HBCINIKPAFI::HBCINIKPAFI::new()
        }
    }

    // .IBOMHKHBAAO FPFDJNDNPIM = 1137;

    pub fn FPFDJNDNPIM(&self) -> &super::IBOMHKHBAAO::IBOMHKHBAAO {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(ref v)) => v,
            _ => <super::IBOMHKHBAAO::IBOMHKHBAAO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FPFDJNDNPIM(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_FPFDJNDNPIM(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FPFDJNDNPIM(&mut self, v: super::IBOMHKHBAAO::IBOMHKHBAAO) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FPFDJNDNPIM(&mut self) -> &mut super::IBOMHKHBAAO::IBOMHKHBAAO {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(super::IBOMHKHBAAO::IBOMHKHBAAO::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FPFDJNDNPIM(&mut self) -> super::IBOMHKHBAAO::IBOMHKHBAAO {
        if self.has_FPFDJNDNPIM() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IBOMHKHBAAO::IBOMHKHBAAO::new()
        }
    }

    // .ENJHDLHKINO NJKJNBDBOCA = 1942;

    pub fn NJKJNBDBOCA(&self) -> &super::ENJHDLHKINO::ENJHDLHKINO {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(ref v)) => v,
            _ => <super::ENJHDLHKINO::ENJHDLHKINO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NJKJNBDBOCA(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_NJKJNBDBOCA(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NJKJNBDBOCA(&mut self, v: super::ENJHDLHKINO::ENJHDLHKINO) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NJKJNBDBOCA(&mut self) -> &mut super::ENJHDLHKINO::ENJHDLHKINO {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(super::ENJHDLHKINO::ENJHDLHKINO::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NJKJNBDBOCA(&mut self) -> super::ENJHDLHKINO::ENJHDLHKINO {
        if self.has_NJKJNBDBOCA() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ENJHDLHKINO::ENJHDLHKINO::new()
        }
    }

    // .FNOGHGHPJPD LGJGBIGHONP = 636;

    pub fn LGJGBIGHONP(&self) -> &super::FNOGHGHPJPD::FNOGHGHPJPD {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(ref v)) => v,
            _ => <super::FNOGHGHPJPD::FNOGHGHPJPD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LGJGBIGHONP(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_LGJGBIGHONP(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LGJGBIGHONP(&mut self, v: super::FNOGHGHPJPD::FNOGHGHPJPD) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LGJGBIGHONP(&mut self) -> &mut super::FNOGHGHPJPD::FNOGHGHPJPD {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(super::FNOGHGHPJPD::FNOGHGHPJPD::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LGJGBIGHONP(&mut self) -> super::FNOGHGHPJPD::FNOGHGHPJPD {
        if self.has_LGJGBIGHONP() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FNOGHGHPJPD::FNOGHGHPJPD::new()
        }
    }

    // .HAKMEBIAJCF DGNKBNGIPKI = 1789;

    pub fn DGNKBNGIPKI(&self) -> &super::HAKMEBIAJCF::HAKMEBIAJCF {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(ref v)) => v,
            _ => <super::HAKMEBIAJCF::HAKMEBIAJCF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DGNKBNGIPKI(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_DGNKBNGIPKI(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DGNKBNGIPKI(&mut self, v: super::HAKMEBIAJCF::HAKMEBIAJCF) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DGNKBNGIPKI(&mut self) -> &mut super::HAKMEBIAJCF::HAKMEBIAJCF {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(super::HAKMEBIAJCF::HAKMEBIAJCF::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DGNKBNGIPKI(&mut self) -> super::HAKMEBIAJCF::HAKMEBIAJCF {
        if self.has_DGNKBNGIPKI() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HAKMEBIAJCF::HAKMEBIAJCF::new()
        }
    }

    // .JLDHCFGGEAO MKKPCHHNHCJ = 1413;

    pub fn MKKPCHHNHCJ(&self) -> &super::JLDHCFGGEAO::JLDHCFGGEAO {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(ref v)) => v,
            _ => <super::JLDHCFGGEAO::JLDHCFGGEAO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MKKPCHHNHCJ(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_MKKPCHHNHCJ(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MKKPCHHNHCJ(&mut self, v: super::JLDHCFGGEAO::JLDHCFGGEAO) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MKKPCHHNHCJ(&mut self) -> &mut super::JLDHCFGGEAO::JLDHCFGGEAO {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(super::JLDHCFGGEAO::JLDHCFGGEAO::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MKKPCHHNHCJ(&mut self) -> super::JLDHCFGGEAO::JLDHCFGGEAO {
        if self.has_MKKPCHHNHCJ() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JLDHCFGGEAO::JLDHCFGGEAO::new()
        }
    }

    // .HMOPIBLFCLN PKPJEFGGBOO = 1416;

    pub fn PKPJEFGGBOO(&self) -> &super::HMOPIBLFCLN::HMOPIBLFCLN {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(ref v)) => v,
            _ => <super::HMOPIBLFCLN::HMOPIBLFCLN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PKPJEFGGBOO(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_PKPJEFGGBOO(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PKPJEFGGBOO(&mut self, v: super::HMOPIBLFCLN::HMOPIBLFCLN) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PKPJEFGGBOO(&mut self) -> &mut super::HMOPIBLFCLN::HMOPIBLFCLN {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(super::HMOPIBLFCLN::HMOPIBLFCLN::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PKPJEFGGBOO(&mut self) -> super::HMOPIBLFCLN::HMOPIBLFCLN {
        if self.has_PKPJEFGGBOO() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HMOPIBLFCLN::HMOPIBLFCLN::new()
        }
    }

    // .PFGIAHAIDLM NHACNNJPALP = 55;

    pub fn NHACNNJPALP(&self) -> &super::PFGIAHAIDLM::PFGIAHAIDLM {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(ref v)) => v,
            _ => <super::PFGIAHAIDLM::PFGIAHAIDLM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NHACNNJPALP(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_NHACNNJPALP(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NHACNNJPALP(&mut self, v: super::PFGIAHAIDLM::PFGIAHAIDLM) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NHACNNJPALP(&mut self) -> &mut super::PFGIAHAIDLM::PFGIAHAIDLM {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(super::PFGIAHAIDLM::PFGIAHAIDLM::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NHACNNJPALP(&mut self) -> super::PFGIAHAIDLM::PFGIAHAIDLM {
        if self.has_NHACNNJPALP() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PFGIAHAIDLM::PFGIAHAIDLM::new()
        }
    }

    // .JHMNLCOBJCJ AHPDPOOLJLE = 1290;

    pub fn AHPDPOOLJLE(&self) -> &super::JHMNLCOBJCJ::JHMNLCOBJCJ {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(ref v)) => v,
            _ => <super::JHMNLCOBJCJ::JHMNLCOBJCJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AHPDPOOLJLE(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_AHPDPOOLJLE(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AHPDPOOLJLE(&mut self, v: super::JHMNLCOBJCJ::JHMNLCOBJCJ) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AHPDPOOLJLE(&mut self) -> &mut super::JHMNLCOBJCJ::JHMNLCOBJCJ {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(super::JHMNLCOBJCJ::JHMNLCOBJCJ::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AHPDPOOLJLE(&mut self) -> super::JHMNLCOBJCJ::JHMNLCOBJCJ {
        if self.has_AHPDPOOLJLE() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JHMNLCOBJCJ::JHMNLCOBJCJ::new()
        }
    }

    // .FKJLBFNIGGM AFMIEICDNEA = 769;

    pub fn AFMIEICDNEA(&self) -> &super::FKJLBFNIGGM::FKJLBFNIGGM {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(ref v)) => v,
            _ => <super::FKJLBFNIGGM::FKJLBFNIGGM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AFMIEICDNEA(&mut self) {
        self.PPHBGCABELF = ::std::option::Option::None;
    }

    pub fn has_AFMIEICDNEA(&self) -> bool {
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AFMIEICDNEA(&mut self, v: super::FKJLBFNIGGM::FKJLBFNIGGM) {
        self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AFMIEICDNEA(&mut self) -> &mut super::FKJLBFNIGGM::FKJLBFNIGGM {
        if let ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(_)) = self.PPHBGCABELF {
        } else {
            self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(super::FKJLBFNIGGM::FKJLBFNIGGM::new()));
        }
        match self.PPHBGCABELF {
            ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AFMIEICDNEA(&mut self) -> super::FKJLBFNIGGM::FKJLBFNIGGM {
        if self.has_AFMIEICDNEA() {
            match self.PPHBGCABELF.take() {
                ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FKJLBFNIGGM::FKJLBFNIGGM::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGCIIKMMDAI",
            |m: &PLKCMGDEDCK| { &m.IGCIIKMMDAI },
            |m: &mut PLKCMGDEDCK| { &mut m.IGCIIKMMDAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HBCINIKPAFI::HBCINIKPAFI>(
            "LNMMKFMEAJM",
            PLKCMGDEDCK::has_LNMMKFMEAJM,
            PLKCMGDEDCK::LNMMKFMEAJM,
            PLKCMGDEDCK::mut_LNMMKFMEAJM,
            PLKCMGDEDCK::set_LNMMKFMEAJM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IBOMHKHBAAO::IBOMHKHBAAO>(
            "FPFDJNDNPIM",
            PLKCMGDEDCK::has_FPFDJNDNPIM,
            PLKCMGDEDCK::FPFDJNDNPIM,
            PLKCMGDEDCK::mut_FPFDJNDNPIM,
            PLKCMGDEDCK::set_FPFDJNDNPIM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ENJHDLHKINO::ENJHDLHKINO>(
            "NJKJNBDBOCA",
            PLKCMGDEDCK::has_NJKJNBDBOCA,
            PLKCMGDEDCK::NJKJNBDBOCA,
            PLKCMGDEDCK::mut_NJKJNBDBOCA,
            PLKCMGDEDCK::set_NJKJNBDBOCA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FNOGHGHPJPD::FNOGHGHPJPD>(
            "LGJGBIGHONP",
            PLKCMGDEDCK::has_LGJGBIGHONP,
            PLKCMGDEDCK::LGJGBIGHONP,
            PLKCMGDEDCK::mut_LGJGBIGHONP,
            PLKCMGDEDCK::set_LGJGBIGHONP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HAKMEBIAJCF::HAKMEBIAJCF>(
            "DGNKBNGIPKI",
            PLKCMGDEDCK::has_DGNKBNGIPKI,
            PLKCMGDEDCK::DGNKBNGIPKI,
            PLKCMGDEDCK::mut_DGNKBNGIPKI,
            PLKCMGDEDCK::set_DGNKBNGIPKI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JLDHCFGGEAO::JLDHCFGGEAO>(
            "MKKPCHHNHCJ",
            PLKCMGDEDCK::has_MKKPCHHNHCJ,
            PLKCMGDEDCK::MKKPCHHNHCJ,
            PLKCMGDEDCK::mut_MKKPCHHNHCJ,
            PLKCMGDEDCK::set_MKKPCHHNHCJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HMOPIBLFCLN::HMOPIBLFCLN>(
            "PKPJEFGGBOO",
            PLKCMGDEDCK::has_PKPJEFGGBOO,
            PLKCMGDEDCK::PKPJEFGGBOO,
            PLKCMGDEDCK::mut_PKPJEFGGBOO,
            PLKCMGDEDCK::set_PKPJEFGGBOO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PFGIAHAIDLM::PFGIAHAIDLM>(
            "NHACNNJPALP",
            PLKCMGDEDCK::has_NHACNNJPALP,
            PLKCMGDEDCK::NHACNNJPALP,
            PLKCMGDEDCK::mut_NHACNNJPALP,
            PLKCMGDEDCK::set_NHACNNJPALP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JHMNLCOBJCJ::JHMNLCOBJCJ>(
            "AHPDPOOLJLE",
            PLKCMGDEDCK::has_AHPDPOOLJLE,
            PLKCMGDEDCK::AHPDPOOLJLE,
            PLKCMGDEDCK::mut_AHPDPOOLJLE,
            PLKCMGDEDCK::set_AHPDPOOLJLE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FKJLBFNIGGM::FKJLBFNIGGM>(
            "AFMIEICDNEA",
            PLKCMGDEDCK::has_AFMIEICDNEA,
            PLKCMGDEDCK::AFMIEICDNEA,
            PLKCMGDEDCK::mut_AFMIEICDNEA,
            PLKCMGDEDCK::set_AFMIEICDNEA,
        ));
        oneofs.push(plkcmgdedck::PPHBGCABELF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PLKCMGDEDCK>(
            "PLKCMGDEDCK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PLKCMGDEDCK {
    const NAME: &'static str = "PLKCMGDEDCK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.IGCIIKMMDAI = is.read_enum_or_unknown()?;
                },
                11274 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(is.read_message()?));
                },
                9098 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(is.read_message()?));
                },
                15538 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(is.read_message()?));
                },
                5090 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(is.read_message()?));
                },
                14314 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(is.read_message()?));
                },
                11306 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(is.read_message()?));
                },
                11330 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(is.read_message()?));
                },
                442 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::NHACNNJPALP(is.read_message()?));
                },
                10322 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(is.read_message()?));
                },
                6154 => {
                    self.PPHBGCABELF = ::std::option::Option::Some(plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(is.read_message()?));
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
        if self.IGCIIKMMDAI != ::protobuf::EnumOrUnknown::new(super::CBEJAJENOHJ::CBEJAJENOHJ::kTrainPartySrcNone) {
            my_size += ::protobuf::rt::int32_size(2, self.IGCIIKMMDAI.value());
        }
        if let ::std::option::Option::Some(ref v) = self.PPHBGCABELF {
            match v {
                &plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::NHACNNJPALP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IGCIIKMMDAI != ::protobuf::EnumOrUnknown::new(super::CBEJAJENOHJ::CBEJAJENOHJ::kTrainPartySrcNone) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.IGCIIKMMDAI))?;
        }
        if let ::std::option::Option::Some(ref v) = self.PPHBGCABELF {
            match v {
                &plkcmgdedck::PPHBGCABELF::LNMMKFMEAJM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1409, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::FPFDJNDNPIM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1137, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::NJKJNBDBOCA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1942, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::LGJGBIGHONP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(636, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::DGNKBNGIPKI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1789, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::MKKPCHHNHCJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1413, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::PKPJEFGGBOO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1416, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::NHACNNJPALP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(55, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::AHPDPOOLJLE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1290, v, os)?;
                },
                &plkcmgdedck::PPHBGCABELF::AFMIEICDNEA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(769, v, os)?;
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

    fn new() -> PLKCMGDEDCK {
        PLKCMGDEDCK::new()
    }

    fn clear(&mut self) {
        self.IGCIIKMMDAI = ::protobuf::EnumOrUnknown::new(super::CBEJAJENOHJ::CBEJAJENOHJ::kTrainPartySrcNone);
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.PPHBGCABELF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PLKCMGDEDCK {
        static instance: PLKCMGDEDCK = PLKCMGDEDCK {
            IGCIIKMMDAI: ::protobuf::EnumOrUnknown::from_i32(0),
            PPHBGCABELF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PLKCMGDEDCK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PLKCMGDEDCK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PLKCMGDEDCK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PLKCMGDEDCK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PLKCMGDEDCK`
pub mod plkcmgdedck {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PLKCMGDEDCK.PPHBGCABELF)
    pub enum PPHBGCABELF {
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.LNMMKFMEAJM)
        LNMMKFMEAJM(super::super::HBCINIKPAFI::HBCINIKPAFI),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.FPFDJNDNPIM)
        FPFDJNDNPIM(super::super::IBOMHKHBAAO::IBOMHKHBAAO),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.NJKJNBDBOCA)
        NJKJNBDBOCA(super::super::ENJHDLHKINO::ENJHDLHKINO),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.LGJGBIGHONP)
        LGJGBIGHONP(super::super::FNOGHGHPJPD::FNOGHGHPJPD),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.DGNKBNGIPKI)
        DGNKBNGIPKI(super::super::HAKMEBIAJCF::HAKMEBIAJCF),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.MKKPCHHNHCJ)
        MKKPCHHNHCJ(super::super::JLDHCFGGEAO::JLDHCFGGEAO),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.PKPJEFGGBOO)
        PKPJEFGGBOO(super::super::HMOPIBLFCLN::HMOPIBLFCLN),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.NHACNNJPALP)
        NHACNNJPALP(super::super::PFGIAHAIDLM::PFGIAHAIDLM),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.AHPDPOOLJLE)
        AHPDPOOLJLE(super::super::JHMNLCOBJCJ::JHMNLCOBJCJ),
        // @@protoc_insertion_point(oneof_field:PLKCMGDEDCK.AFMIEICDNEA)
        AFMIEICDNEA(super::super::FKJLBFNIGGM::FKJLBFNIGGM),
    }

    impl ::protobuf::Oneof for PPHBGCABELF {
    }

    impl ::protobuf::OneofFull for PPHBGCABELF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PLKCMGDEDCK as ::protobuf::MessageFull>::descriptor().oneof_by_name("PPHBGCABELF").unwrap()).clone()
        }
    }

    impl PPHBGCABELF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<PPHBGCABELF>("PPHBGCABELF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PLKCMGDEDCK.proto\x1a\x11CBEJAJENOHJ.proto\x1a\x11ENJHDLHKINO.prot\
    o\x1a\x11FKJLBFNIGGM.proto\x1a\x11FNOGHGHPJPD.proto\x1a\x11HAKMEBIAJCF.p\
    roto\x1a\x11HBCINIKPAFI.proto\x1a\x11HMOPIBLFCLN.proto\x1a\x11IBOMHKHBAA\
    O.proto\x1a\x11JHMNLCOBJCJ.proto\x1a\x11JLDHCFGGEAO.proto\x1a\x11PFGIAHA\
    IDLM.proto\"\xc9\x04\n\x0bPLKCMGDEDCK\x12.\n\x0bIGCIIKMMDAI\x18\x02\x20\
    \x01(\x0e2\x0c.CBEJAJENOHJR\x0bIGCIIKMMDAI\x121\n\x0bLNMMKFMEAJM\x18\x81\
    \x0b\x20\x01(\x0b2\x0c.HBCINIKPAFIH\0R\x0bLNMMKFMEAJM\x121\n\x0bFPFDJNDN\
    PIM\x18\xf1\x08\x20\x01(\x0b2\x0c.IBOMHKHBAAOH\0R\x0bFPFDJNDNPIM\x121\n\
    \x0bNJKJNBDBOCA\x18\x96\x0f\x20\x01(\x0b2\x0c.ENJHDLHKINOH\0R\x0bNJKJNBD\
    BOCA\x121\n\x0bLGJGBIGHONP\x18\xfc\x04\x20\x01(\x0b2\x0c.FNOGHGHPJPDH\0R\
    \x0bLGJGBIGHONP\x121\n\x0bDGNKBNGIPKI\x18\xfd\r\x20\x01(\x0b2\x0c.HAKMEB\
    IAJCFH\0R\x0bDGNKBNGIPKI\x121\n\x0bMKKPCHHNHCJ\x18\x85\x0b\x20\x01(\x0b2\
    \x0c.JLDHCFGGEAOH\0R\x0bMKKPCHHNHCJ\x121\n\x0bPKPJEFGGBOO\x18\x88\x0b\
    \x20\x01(\x0b2\x0c.HMOPIBLFCLNH\0R\x0bPKPJEFGGBOO\x120\n\x0bNHACNNJPALP\
    \x187\x20\x01(\x0b2\x0c.PFGIAHAIDLMH\0R\x0bNHACNNJPALP\x121\n\x0bAHPDPOO\
    LJLE\x18\x8a\n\x20\x01(\x0b2\x0c.JHMNLCOBJCJH\0R\x0bAHPDPOOLJLE\x121\n\
    \x0bAFMIEICDNEA\x18\x81\x06\x20\x01(\x0b2\x0c.FKJLBFNIGGMH\0R\x0bAFMIEIC\
    DNEAB\r\n\x0bPPHBGCABELFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::CBEJAJENOHJ::file_descriptor().clone());
            deps.push(super::ENJHDLHKINO::file_descriptor().clone());
            deps.push(super::FKJLBFNIGGM::file_descriptor().clone());
            deps.push(super::FNOGHGHPJPD::file_descriptor().clone());
            deps.push(super::HAKMEBIAJCF::file_descriptor().clone());
            deps.push(super::HBCINIKPAFI::file_descriptor().clone());
            deps.push(super::HMOPIBLFCLN::file_descriptor().clone());
            deps.push(super::IBOMHKHBAAO::file_descriptor().clone());
            deps.push(super::JHMNLCOBJCJ::file_descriptor().clone());
            deps.push(super::JLDHCFGGEAO::file_descriptor().clone());
            deps.push(super::PFGIAHAIDLM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PLKCMGDEDCK::generated_message_descriptor_data());
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
