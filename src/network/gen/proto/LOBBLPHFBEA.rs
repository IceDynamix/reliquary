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

//! Generated file from `LOBBLPHFBEA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LOBBLPHFBEA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LOBBLPHFBEA {
    // message fields
    // @@protoc_insertion_point(field:LOBBLPHFBEA.IBEONHKHFKK)
    pub IBEONHKHFKK: ::protobuf::EnumOrUnknown<super::EBDILHJHNGA::EBDILHJHNGA>,
    // message oneof groups
    pub PCCOGDPNHHI: ::std::option::Option<lobblphfbea::PCCOGDPNHHI>,
    // special fields
    // @@protoc_insertion_point(special_field:LOBBLPHFBEA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LOBBLPHFBEA {
    fn default() -> &'a LOBBLPHFBEA {
        <LOBBLPHFBEA as ::protobuf::Message>::default_instance()
    }
}

impl LOBBLPHFBEA {
    pub fn new() -> LOBBLPHFBEA {
        ::std::default::Default::default()
    }

    // .BNKEFBKDJKP JJNLKBFGICM = 164;

    pub fn JJNLKBFGICM(&self) -> &super::BNKEFBKDJKP::BNKEFBKDJKP {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(ref v)) => v,
            _ => <super::BNKEFBKDJKP::BNKEFBKDJKP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JJNLKBFGICM(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_JJNLKBFGICM(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JJNLKBFGICM(&mut self, v: super::BNKEFBKDJKP::BNKEFBKDJKP) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JJNLKBFGICM(&mut self) -> &mut super::BNKEFBKDJKP::BNKEFBKDJKP {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(super::BNKEFBKDJKP::BNKEFBKDJKP::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JJNLKBFGICM(&mut self) -> super::BNKEFBKDJKP::BNKEFBKDJKP {
        if self.has_JJNLKBFGICM() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BNKEFBKDJKP::BNKEFBKDJKP::new()
        }
    }

    // .CKDHFNDDEAF PPOFFAFFBDE = 463;

    pub fn PPOFFAFFBDE(&self) -> &super::CKDHFNDDEAF::CKDHFNDDEAF {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(ref v)) => v,
            _ => <super::CKDHFNDDEAF::CKDHFNDDEAF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PPOFFAFFBDE(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_PPOFFAFFBDE(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PPOFFAFFBDE(&mut self, v: super::CKDHFNDDEAF::CKDHFNDDEAF) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PPOFFAFFBDE(&mut self) -> &mut super::CKDHFNDDEAF::CKDHFNDDEAF {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(super::CKDHFNDDEAF::CKDHFNDDEAF::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PPOFFAFFBDE(&mut self) -> super::CKDHFNDDEAF::CKDHFNDDEAF {
        if self.has_PPOFFAFFBDE() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CKDHFNDDEAF::CKDHFNDDEAF::new()
        }
    }

    // .KKBBOHLOGHN MHPAKBAFJPB = 140;

    pub fn MHPAKBAFJPB(&self) -> &super::KKBBOHLOGHN::KKBBOHLOGHN {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(ref v)) => v,
            _ => <super::KKBBOHLOGHN::KKBBOHLOGHN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MHPAKBAFJPB(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_MHPAKBAFJPB(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MHPAKBAFJPB(&mut self, v: super::KKBBOHLOGHN::KKBBOHLOGHN) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MHPAKBAFJPB(&mut self) -> &mut super::KKBBOHLOGHN::KKBBOHLOGHN {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(super::KKBBOHLOGHN::KKBBOHLOGHN::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MHPAKBAFJPB(&mut self) -> super::KKBBOHLOGHN::KKBBOHLOGHN {
        if self.has_MHPAKBAFJPB() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KKBBOHLOGHN::KKBBOHLOGHN::new()
        }
    }

    // .DPOKFMOALKE KDMHKMKKPIN = 1135;

    pub fn KDMHKMKKPIN(&self) -> &super::DPOKFMOALKE::DPOKFMOALKE {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(ref v)) => v,
            _ => <super::DPOKFMOALKE::DPOKFMOALKE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KDMHKMKKPIN(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_KDMHKMKKPIN(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KDMHKMKKPIN(&mut self, v: super::DPOKFMOALKE::DPOKFMOALKE) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KDMHKMKKPIN(&mut self) -> &mut super::DPOKFMOALKE::DPOKFMOALKE {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(super::DPOKFMOALKE::DPOKFMOALKE::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KDMHKMKKPIN(&mut self) -> super::DPOKFMOALKE::DPOKFMOALKE {
        if self.has_KDMHKMKKPIN() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DPOKFMOALKE::DPOKFMOALKE::new()
        }
    }

    // .GFPCBHIHDNN JOHKNHLBPOP = 784;

    pub fn JOHKNHLBPOP(&self) -> &super::GFPCBHIHDNN::GFPCBHIHDNN {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(ref v)) => v,
            _ => <super::GFPCBHIHDNN::GFPCBHIHDNN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JOHKNHLBPOP(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_JOHKNHLBPOP(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JOHKNHLBPOP(&mut self, v: super::GFPCBHIHDNN::GFPCBHIHDNN) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JOHKNHLBPOP(&mut self) -> &mut super::GFPCBHIHDNN::GFPCBHIHDNN {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(super::GFPCBHIHDNN::GFPCBHIHDNN::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JOHKNHLBPOP(&mut self) -> super::GFPCBHIHDNN::GFPCBHIHDNN {
        if self.has_JOHKNHLBPOP() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GFPCBHIHDNN::GFPCBHIHDNN::new()
        }
    }

    // .FOHPEOLHHME IFIIODEJCBH = 1913;

    pub fn IFIIODEJCBH(&self) -> &super::FOHPEOLHHME::FOHPEOLHHME {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(ref v)) => v,
            _ => <super::FOHPEOLHHME::FOHPEOLHHME as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IFIIODEJCBH(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_IFIIODEJCBH(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IFIIODEJCBH(&mut self, v: super::FOHPEOLHHME::FOHPEOLHHME) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IFIIODEJCBH(&mut self) -> &mut super::FOHPEOLHHME::FOHPEOLHHME {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(super::FOHPEOLHHME::FOHPEOLHHME::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IFIIODEJCBH(&mut self) -> super::FOHPEOLHHME::FOHPEOLHHME {
        if self.has_IFIIODEJCBH() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FOHPEOLHHME::FOHPEOLHHME::new()
        }
    }

    // .GDHBAIHFKBG JKEMHLOPCAO = 44;

    pub fn JKEMHLOPCAO(&self) -> &super::GDHBAIHFKBG::GDHBAIHFKBG {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(ref v)) => v,
            _ => <super::GDHBAIHFKBG::GDHBAIHFKBG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JKEMHLOPCAO(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_JKEMHLOPCAO(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JKEMHLOPCAO(&mut self, v: super::GDHBAIHFKBG::GDHBAIHFKBG) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JKEMHLOPCAO(&mut self) -> &mut super::GDHBAIHFKBG::GDHBAIHFKBG {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(super::GDHBAIHFKBG::GDHBAIHFKBG::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JKEMHLOPCAO(&mut self) -> super::GDHBAIHFKBG::GDHBAIHFKBG {
        if self.has_JKEMHLOPCAO() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GDHBAIHFKBG::GDHBAIHFKBG::new()
        }
    }

    // .FNBMAMMBKJJ AJOONNAEHME = 1535;

    pub fn AJOONNAEHME(&self) -> &super::FNBMAMMBKJJ::FNBMAMMBKJJ {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(ref v)) => v,
            _ => <super::FNBMAMMBKJJ::FNBMAMMBKJJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AJOONNAEHME(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_AJOONNAEHME(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AJOONNAEHME(&mut self, v: super::FNBMAMMBKJJ::FNBMAMMBKJJ) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AJOONNAEHME(&mut self) -> &mut super::FNBMAMMBKJJ::FNBMAMMBKJJ {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(super::FNBMAMMBKJJ::FNBMAMMBKJJ::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AJOONNAEHME(&mut self) -> super::FNBMAMMBKJJ::FNBMAMMBKJJ {
        if self.has_AJOONNAEHME() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FNBMAMMBKJJ::FNBMAMMBKJJ::new()
        }
    }

    // .NJOPBMFEEPJ KOKELOCCEPP = 492;

    pub fn KOKELOCCEPP(&self) -> &super::NJOPBMFEEPJ::NJOPBMFEEPJ {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(ref v)) => v,
            _ => <super::NJOPBMFEEPJ::NJOPBMFEEPJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KOKELOCCEPP(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_KOKELOCCEPP(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KOKELOCCEPP(&mut self, v: super::NJOPBMFEEPJ::NJOPBMFEEPJ) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KOKELOCCEPP(&mut self) -> &mut super::NJOPBMFEEPJ::NJOPBMFEEPJ {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(super::NJOPBMFEEPJ::NJOPBMFEEPJ::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KOKELOCCEPP(&mut self) -> super::NJOPBMFEEPJ::NJOPBMFEEPJ {
        if self.has_KOKELOCCEPP() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NJOPBMFEEPJ::NJOPBMFEEPJ::new()
        }
    }

    // .HOMBELLNGMA JJFEABIGBNC = 337;

    pub fn JJFEABIGBNC(&self) -> &super::HOMBELLNGMA::HOMBELLNGMA {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(ref v)) => v,
            _ => <super::HOMBELLNGMA::HOMBELLNGMA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JJFEABIGBNC(&mut self) {
        self.PCCOGDPNHHI = ::std::option::Option::None;
    }

    pub fn has_JJFEABIGBNC(&self) -> bool {
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JJFEABIGBNC(&mut self, v: super::HOMBELLNGMA::HOMBELLNGMA) {
        self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JJFEABIGBNC(&mut self) -> &mut super::HOMBELLNGMA::HOMBELLNGMA {
        if let ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(_)) = self.PCCOGDPNHHI {
        } else {
            self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(super::HOMBELLNGMA::HOMBELLNGMA::new()));
        }
        match self.PCCOGDPNHHI {
            ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JJFEABIGBNC(&mut self) -> super::HOMBELLNGMA::HOMBELLNGMA {
        if self.has_JJFEABIGBNC() {
            match self.PCCOGDPNHHI.take() {
                ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HOMBELLNGMA::HOMBELLNGMA::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBEONHKHFKK",
            |m: &LOBBLPHFBEA| { &m.IBEONHKHFKK },
            |m: &mut LOBBLPHFBEA| { &mut m.IBEONHKHFKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BNKEFBKDJKP::BNKEFBKDJKP>(
            "JJNLKBFGICM",
            LOBBLPHFBEA::has_JJNLKBFGICM,
            LOBBLPHFBEA::JJNLKBFGICM,
            LOBBLPHFBEA::mut_JJNLKBFGICM,
            LOBBLPHFBEA::set_JJNLKBFGICM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CKDHFNDDEAF::CKDHFNDDEAF>(
            "PPOFFAFFBDE",
            LOBBLPHFBEA::has_PPOFFAFFBDE,
            LOBBLPHFBEA::PPOFFAFFBDE,
            LOBBLPHFBEA::mut_PPOFFAFFBDE,
            LOBBLPHFBEA::set_PPOFFAFFBDE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KKBBOHLOGHN::KKBBOHLOGHN>(
            "MHPAKBAFJPB",
            LOBBLPHFBEA::has_MHPAKBAFJPB,
            LOBBLPHFBEA::MHPAKBAFJPB,
            LOBBLPHFBEA::mut_MHPAKBAFJPB,
            LOBBLPHFBEA::set_MHPAKBAFJPB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DPOKFMOALKE::DPOKFMOALKE>(
            "KDMHKMKKPIN",
            LOBBLPHFBEA::has_KDMHKMKKPIN,
            LOBBLPHFBEA::KDMHKMKKPIN,
            LOBBLPHFBEA::mut_KDMHKMKKPIN,
            LOBBLPHFBEA::set_KDMHKMKKPIN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GFPCBHIHDNN::GFPCBHIHDNN>(
            "JOHKNHLBPOP",
            LOBBLPHFBEA::has_JOHKNHLBPOP,
            LOBBLPHFBEA::JOHKNHLBPOP,
            LOBBLPHFBEA::mut_JOHKNHLBPOP,
            LOBBLPHFBEA::set_JOHKNHLBPOP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FOHPEOLHHME::FOHPEOLHHME>(
            "IFIIODEJCBH",
            LOBBLPHFBEA::has_IFIIODEJCBH,
            LOBBLPHFBEA::IFIIODEJCBH,
            LOBBLPHFBEA::mut_IFIIODEJCBH,
            LOBBLPHFBEA::set_IFIIODEJCBH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GDHBAIHFKBG::GDHBAIHFKBG>(
            "JKEMHLOPCAO",
            LOBBLPHFBEA::has_JKEMHLOPCAO,
            LOBBLPHFBEA::JKEMHLOPCAO,
            LOBBLPHFBEA::mut_JKEMHLOPCAO,
            LOBBLPHFBEA::set_JKEMHLOPCAO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FNBMAMMBKJJ::FNBMAMMBKJJ>(
            "AJOONNAEHME",
            LOBBLPHFBEA::has_AJOONNAEHME,
            LOBBLPHFBEA::AJOONNAEHME,
            LOBBLPHFBEA::mut_AJOONNAEHME,
            LOBBLPHFBEA::set_AJOONNAEHME,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NJOPBMFEEPJ::NJOPBMFEEPJ>(
            "KOKELOCCEPP",
            LOBBLPHFBEA::has_KOKELOCCEPP,
            LOBBLPHFBEA::KOKELOCCEPP,
            LOBBLPHFBEA::mut_KOKELOCCEPP,
            LOBBLPHFBEA::set_KOKELOCCEPP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HOMBELLNGMA::HOMBELLNGMA>(
            "JJFEABIGBNC",
            LOBBLPHFBEA::has_JJFEABIGBNC,
            LOBBLPHFBEA::JJFEABIGBNC,
            LOBBLPHFBEA::mut_JJFEABIGBNC,
            LOBBLPHFBEA::set_JJFEABIGBNC,
        ));
        oneofs.push(lobblphfbea::PCCOGDPNHHI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LOBBLPHFBEA>(
            "LOBBLPHFBEA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LOBBLPHFBEA {
    const NAME: &'static str = "LOBBLPHFBEA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.IBEONHKHFKK = is.read_enum_or_unknown()?;
                },
                1314 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(is.read_message()?));
                },
                3706 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(is.read_message()?));
                },
                1122 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(is.read_message()?));
                },
                9082 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(is.read_message()?));
                },
                6274 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(is.read_message()?));
                },
                15306 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(is.read_message()?));
                },
                354 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(is.read_message()?));
                },
                12282 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(is.read_message()?));
                },
                3938 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(is.read_message()?));
                },
                2698 => {
                    self.PCCOGDPNHHI = ::std::option::Option::Some(lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(is.read_message()?));
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
        if self.IBEONHKHFKK != ::protobuf::EnumOrUnknown::new(super::EBDILHJHNGA::EBDILHJHNGA::kTrainPartySrcNone) {
            my_size += ::protobuf::rt::int32_size(2, self.IBEONHKHFKK.value());
        }
        if let ::std::option::Option::Some(ref v) = self.PCCOGDPNHHI {
            match v {
                &lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(ref v) => {
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
        if self.IBEONHKHFKK != ::protobuf::EnumOrUnknown::new(super::EBDILHJHNGA::EBDILHJHNGA::kTrainPartySrcNone) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.IBEONHKHFKK))?;
        }
        if let ::std::option::Option::Some(ref v) = self.PCCOGDPNHHI {
            match v {
                &lobblphfbea::PCCOGDPNHHI::JJNLKBFGICM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(164, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::PPOFFAFFBDE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(463, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::MHPAKBAFJPB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(140, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::KDMHKMKKPIN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1135, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::JOHKNHLBPOP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(784, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::IFIIODEJCBH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1913, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::JKEMHLOPCAO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(44, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::AJOONNAEHME(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1535, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::KOKELOCCEPP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(492, v, os)?;
                },
                &lobblphfbea::PCCOGDPNHHI::JJFEABIGBNC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(337, v, os)?;
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

    fn new() -> LOBBLPHFBEA {
        LOBBLPHFBEA::new()
    }

    fn clear(&mut self) {
        self.IBEONHKHFKK = ::protobuf::EnumOrUnknown::new(super::EBDILHJHNGA::EBDILHJHNGA::kTrainPartySrcNone);
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.PCCOGDPNHHI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LOBBLPHFBEA {
        static instance: LOBBLPHFBEA = LOBBLPHFBEA {
            IBEONHKHFKK: ::protobuf::EnumOrUnknown::from_i32(0),
            PCCOGDPNHHI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LOBBLPHFBEA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LOBBLPHFBEA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LOBBLPHFBEA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LOBBLPHFBEA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LOBBLPHFBEA`
pub mod lobblphfbea {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:LOBBLPHFBEA.PCCOGDPNHHI)
    pub enum PCCOGDPNHHI {
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.JJNLKBFGICM)
        JJNLKBFGICM(super::super::BNKEFBKDJKP::BNKEFBKDJKP),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.PPOFFAFFBDE)
        PPOFFAFFBDE(super::super::CKDHFNDDEAF::CKDHFNDDEAF),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.MHPAKBAFJPB)
        MHPAKBAFJPB(super::super::KKBBOHLOGHN::KKBBOHLOGHN),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.KDMHKMKKPIN)
        KDMHKMKKPIN(super::super::DPOKFMOALKE::DPOKFMOALKE),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.JOHKNHLBPOP)
        JOHKNHLBPOP(super::super::GFPCBHIHDNN::GFPCBHIHDNN),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.IFIIODEJCBH)
        IFIIODEJCBH(super::super::FOHPEOLHHME::FOHPEOLHHME),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.JKEMHLOPCAO)
        JKEMHLOPCAO(super::super::GDHBAIHFKBG::GDHBAIHFKBG),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.AJOONNAEHME)
        AJOONNAEHME(super::super::FNBMAMMBKJJ::FNBMAMMBKJJ),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.KOKELOCCEPP)
        KOKELOCCEPP(super::super::NJOPBMFEEPJ::NJOPBMFEEPJ),
        // @@protoc_insertion_point(oneof_field:LOBBLPHFBEA.JJFEABIGBNC)
        JJFEABIGBNC(super::super::HOMBELLNGMA::HOMBELLNGMA),
    }

    impl ::protobuf::Oneof for PCCOGDPNHHI {
    }

    impl ::protobuf::OneofFull for PCCOGDPNHHI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LOBBLPHFBEA as ::protobuf::MessageFull>::descriptor().oneof_by_name("PCCOGDPNHHI").unwrap()).clone()
        }
    }

    impl PCCOGDPNHHI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<PCCOGDPNHHI>("PCCOGDPNHHI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LOBBLPHFBEA.proto\x1a\x11BNKEFBKDJKP.proto\x1a\x11CKDHFNDDEAF.prot\
    o\x1a\x11DPOKFMOALKE.proto\x1a\x11EBDILHJHNGA.proto\x1a\x11FNBMAMMBKJJ.p\
    roto\x1a\x11FOHPEOLHHME.proto\x1a\x11GDHBAIHFKBG.proto\x1a\x11GFPCBHIHDN\
    N.proto\x1a\x11HOMBELLNGMA.proto\x1a\x11KKBBOHLOGHN.proto\x1a\x11NJOPBMF\
    EEPJ.proto\"\xc9\x04\n\x0bLOBBLPHFBEA\x12.\n\x0bIBEONHKHFKK\x18\x02\x20\
    \x01(\x0e2\x0c.EBDILHJHNGAR\x0bIBEONHKHFKK\x121\n\x0bJJNLKBFGICM\x18\xa4\
    \x01\x20\x01(\x0b2\x0c.BNKEFBKDJKPH\0R\x0bJJNLKBFGICM\x121\n\x0bPPOFFAFF\
    BDE\x18\xcf\x03\x20\x01(\x0b2\x0c.CKDHFNDDEAFH\0R\x0bPPOFFAFFBDE\x121\n\
    \x0bMHPAKBAFJPB\x18\x8c\x01\x20\x01(\x0b2\x0c.KKBBOHLOGHNH\0R\x0bMHPAKBA\
    FJPB\x121\n\x0bKDMHKMKKPIN\x18\xef\x08\x20\x01(\x0b2\x0c.DPOKFMOALKEH\0R\
    \x0bKDMHKMKKPIN\x121\n\x0bJOHKNHLBPOP\x18\x90\x06\x20\x01(\x0b2\x0c.GFPC\
    BHIHDNNH\0R\x0bJOHKNHLBPOP\x121\n\x0bIFIIODEJCBH\x18\xf9\x0e\x20\x01(\
    \x0b2\x0c.FOHPEOLHHMEH\0R\x0bIFIIODEJCBH\x120\n\x0bJKEMHLOPCAO\x18,\x20\
    \x01(\x0b2\x0c.GDHBAIHFKBGH\0R\x0bJKEMHLOPCAO\x121\n\x0bAJOONNAEHME\x18\
    \xff\x0b\x20\x01(\x0b2\x0c.FNBMAMMBKJJH\0R\x0bAJOONNAEHME\x121\n\x0bKOKE\
    LOCCEPP\x18\xec\x03\x20\x01(\x0b2\x0c.NJOPBMFEEPJH\0R\x0bKOKELOCCEPP\x12\
    1\n\x0bJJFEABIGBNC\x18\xd1\x02\x20\x01(\x0b2\x0c.HOMBELLNGMAH\0R\x0bJJFE\
    ABIGBNCB\r\n\x0bPCCOGDPNHHIb\x06proto3\
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
            deps.push(super::BNKEFBKDJKP::file_descriptor().clone());
            deps.push(super::CKDHFNDDEAF::file_descriptor().clone());
            deps.push(super::DPOKFMOALKE::file_descriptor().clone());
            deps.push(super::EBDILHJHNGA::file_descriptor().clone());
            deps.push(super::FNBMAMMBKJJ::file_descriptor().clone());
            deps.push(super::FOHPEOLHHME::file_descriptor().clone());
            deps.push(super::GDHBAIHFKBG::file_descriptor().clone());
            deps.push(super::GFPCBHIHDNN::file_descriptor().clone());
            deps.push(super::HOMBELLNGMA::file_descriptor().clone());
            deps.push(super::KKBBOHLOGHN::file_descriptor().clone());
            deps.push(super::NJOPBMFEEPJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LOBBLPHFBEA::generated_message_descriptor_data());
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