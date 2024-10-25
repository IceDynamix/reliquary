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

//! Generated file from `KMMHJDNCBAB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KMMHJDNCBAB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KMMHJDNCBAB {
    // message oneof groups
    pub NBMKIGLFBJM: ::std::option::Option<kmmhjdncbab::NBMKIGLFBJM>,
    // special fields
    // @@protoc_insertion_point(special_field:KMMHJDNCBAB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KMMHJDNCBAB {
    fn default() -> &'a KMMHJDNCBAB {
        <KMMHJDNCBAB as ::protobuf::Message>::default_instance()
    }
}

impl KMMHJDNCBAB {
    pub fn new() -> KMMHJDNCBAB {
        ::std::default::Default::default()
    }

    // .PFCCJJJLPIC ENNABLPIAEE = 7;

    pub fn ENNABLPIAEE(&self) -> &super::PFCCJJJLPIC::PFCCJJJLPIC {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(ref v)) => v,
            _ => <super::PFCCJJJLPIC::PFCCJJJLPIC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ENNABLPIAEE(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_ENNABLPIAEE(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ENNABLPIAEE(&mut self, v: super::PFCCJJJLPIC::PFCCJJJLPIC) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ENNABLPIAEE(&mut self) -> &mut super::PFCCJJJLPIC::PFCCJJJLPIC {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(super::PFCCJJJLPIC::PFCCJJJLPIC::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ENNABLPIAEE(&mut self) -> super::PFCCJJJLPIC::PFCCJJJLPIC {
        if self.has_ENNABLPIAEE() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PFCCJJJLPIC::PFCCJJJLPIC::new()
        }
    }

    // .JJPPOOADFOO EKDHJFNNNNB = 3;

    pub fn EKDHJFNNNNB(&self) -> &super::JJPPOOADFOO::JJPPOOADFOO {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(ref v)) => v,
            _ => <super::JJPPOOADFOO::JJPPOOADFOO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_EKDHJFNNNNB(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_EKDHJFNNNNB(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_EKDHJFNNNNB(&mut self, v: super::JJPPOOADFOO::JJPPOOADFOO) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_EKDHJFNNNNB(&mut self) -> &mut super::JJPPOOADFOO::JJPPOOADFOO {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(super::JJPPOOADFOO::JJPPOOADFOO::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_EKDHJFNNNNB(&mut self) -> super::JJPPOOADFOO::JJPPOOADFOO {
        if self.has_EKDHJFNNNNB() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JJPPOOADFOO::JJPPOOADFOO::new()
        }
    }

    // .OOMFCADDDPB BCLCJEJHGAE = 9;

    pub fn BCLCJEJHGAE(&self) -> &super::OOMFCADDDPB::OOMFCADDDPB {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(ref v)) => v,
            _ => <super::OOMFCADDDPB::OOMFCADDDPB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BCLCJEJHGAE(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_BCLCJEJHGAE(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BCLCJEJHGAE(&mut self, v: super::OOMFCADDDPB::OOMFCADDDPB) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BCLCJEJHGAE(&mut self) -> &mut super::OOMFCADDDPB::OOMFCADDDPB {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(super::OOMFCADDDPB::OOMFCADDDPB::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BCLCJEJHGAE(&mut self) -> super::OOMFCADDDPB::OOMFCADDDPB {
        if self.has_BCLCJEJHGAE() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::OOMFCADDDPB::OOMFCADDDPB::new()
        }
    }

    // .KMOOALCLOLN MACIMGBEBDD = 8;

    pub fn MACIMGBEBDD(&self) -> &super::KMOOALCLOLN::KMOOALCLOLN {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(ref v)) => v,
            _ => <super::KMOOALCLOLN::KMOOALCLOLN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MACIMGBEBDD(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_MACIMGBEBDD(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MACIMGBEBDD(&mut self, v: super::KMOOALCLOLN::KMOOALCLOLN) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MACIMGBEBDD(&mut self) -> &mut super::KMOOALCLOLN::KMOOALCLOLN {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(super::KMOOALCLOLN::KMOOALCLOLN::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MACIMGBEBDD(&mut self) -> super::KMOOALCLOLN::KMOOALCLOLN {
        if self.has_MACIMGBEBDD() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KMOOALCLOLN::KMOOALCLOLN::new()
        }
    }

    // .KMFIKFJIEMO CBBDBJGJAMO = 10;

    pub fn CBBDBJGJAMO(&self) -> &super::KMFIKFJIEMO::KMFIKFJIEMO {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(ref v)) => v,
            _ => <super::KMFIKFJIEMO::KMFIKFJIEMO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_CBBDBJGJAMO(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_CBBDBJGJAMO(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_CBBDBJGJAMO(&mut self, v: super::KMFIKFJIEMO::KMFIKFJIEMO) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_CBBDBJGJAMO(&mut self) -> &mut super::KMFIKFJIEMO::KMFIKFJIEMO {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(super::KMFIKFJIEMO::KMFIKFJIEMO::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_CBBDBJGJAMO(&mut self) -> super::KMFIKFJIEMO::KMFIKFJIEMO {
        if self.has_CBBDBJGJAMO() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KMFIKFJIEMO::KMFIKFJIEMO::new()
        }
    }

    // .MIMLAKLPCPL FKOEDJJOJGB = 12;

    pub fn FKOEDJJOJGB(&self) -> &super::MIMLAKLPCPL::MIMLAKLPCPL {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(ref v)) => v,
            _ => <super::MIMLAKLPCPL::MIMLAKLPCPL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FKOEDJJOJGB(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_FKOEDJJOJGB(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FKOEDJJOJGB(&mut self, v: super::MIMLAKLPCPL::MIMLAKLPCPL) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FKOEDJJOJGB(&mut self) -> &mut super::MIMLAKLPCPL::MIMLAKLPCPL {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(super::MIMLAKLPCPL::MIMLAKLPCPL::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FKOEDJJOJGB(&mut self) -> super::MIMLAKLPCPL::MIMLAKLPCPL {
        if self.has_FKOEDJJOJGB() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MIMLAKLPCPL::MIMLAKLPCPL::new()
        }
    }

    // .LAGHMBLEMJJ GJOAKDGLLLL = 13;

    pub fn GJOAKDGLLLL(&self) -> &super::LAGHMBLEMJJ::LAGHMBLEMJJ {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(ref v)) => v,
            _ => <super::LAGHMBLEMJJ::LAGHMBLEMJJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GJOAKDGLLLL(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_GJOAKDGLLLL(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GJOAKDGLLLL(&mut self, v: super::LAGHMBLEMJJ::LAGHMBLEMJJ) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GJOAKDGLLLL(&mut self) -> &mut super::LAGHMBLEMJJ::LAGHMBLEMJJ {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(super::LAGHMBLEMJJ::LAGHMBLEMJJ::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GJOAKDGLLLL(&mut self) -> super::LAGHMBLEMJJ::LAGHMBLEMJJ {
        if self.has_GJOAKDGLLLL() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LAGHMBLEMJJ::LAGHMBLEMJJ::new()
        }
    }

    // .DAOPAFHNNAH CEPLFBBJIGK = 15;

    pub fn CEPLFBBJIGK(&self) -> &super::DAOPAFHNNAH::DAOPAFHNNAH {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(ref v)) => v,
            _ => <super::DAOPAFHNNAH::DAOPAFHNNAH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_CEPLFBBJIGK(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_CEPLFBBJIGK(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_CEPLFBBJIGK(&mut self, v: super::DAOPAFHNNAH::DAOPAFHNNAH) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_CEPLFBBJIGK(&mut self) -> &mut super::DAOPAFHNNAH::DAOPAFHNNAH {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(super::DAOPAFHNNAH::DAOPAFHNNAH::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_CEPLFBBJIGK(&mut self) -> super::DAOPAFHNNAH::DAOPAFHNNAH {
        if self.has_CEPLFBBJIGK() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DAOPAFHNNAH::DAOPAFHNNAH::new()
        }
    }

    // .FHEJDFGDPMO MOJKOEDKGAO = 4;

    pub fn MOJKOEDKGAO(&self) -> &super::FHEJDFGDPMO::FHEJDFGDPMO {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(ref v)) => v,
            _ => <super::FHEJDFGDPMO::FHEJDFGDPMO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MOJKOEDKGAO(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_MOJKOEDKGAO(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MOJKOEDKGAO(&mut self, v: super::FHEJDFGDPMO::FHEJDFGDPMO) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MOJKOEDKGAO(&mut self) -> &mut super::FHEJDFGDPMO::FHEJDFGDPMO {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(super::FHEJDFGDPMO::FHEJDFGDPMO::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MOJKOEDKGAO(&mut self) -> super::FHEJDFGDPMO::FHEJDFGDPMO {
        if self.has_MOJKOEDKGAO() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FHEJDFGDPMO::FHEJDFGDPMO::new()
        }
    }

    // .LEPMIKONNPH DCPPMNOKCOC = 14;

    pub fn DCPPMNOKCOC(&self) -> &super::LEPMIKONNPH::LEPMIKONNPH {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(ref v)) => v,
            _ => <super::LEPMIKONNPH::LEPMIKONNPH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DCPPMNOKCOC(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
    }

    pub fn has_DCPPMNOKCOC(&self) -> bool {
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DCPPMNOKCOC(&mut self, v: super::LEPMIKONNPH::LEPMIKONNPH) {
        self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DCPPMNOKCOC(&mut self) -> &mut super::LEPMIKONNPH::LEPMIKONNPH {
        if let ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(_)) = self.NBMKIGLFBJM {
        } else {
            self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(super::LEPMIKONNPH::LEPMIKONNPH::new()));
        }
        match self.NBMKIGLFBJM {
            ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DCPPMNOKCOC(&mut self) -> super::LEPMIKONNPH::LEPMIKONNPH {
        if self.has_DCPPMNOKCOC() {
            match self.NBMKIGLFBJM.take() {
                ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LEPMIKONNPH::LEPMIKONNPH::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PFCCJJJLPIC::PFCCJJJLPIC>(
            "ENNABLPIAEE",
            KMMHJDNCBAB::has_ENNABLPIAEE,
            KMMHJDNCBAB::ENNABLPIAEE,
            KMMHJDNCBAB::mut_ENNABLPIAEE,
            KMMHJDNCBAB::set_ENNABLPIAEE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JJPPOOADFOO::JJPPOOADFOO>(
            "EKDHJFNNNNB",
            KMMHJDNCBAB::has_EKDHJFNNNNB,
            KMMHJDNCBAB::EKDHJFNNNNB,
            KMMHJDNCBAB::mut_EKDHJFNNNNB,
            KMMHJDNCBAB::set_EKDHJFNNNNB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::OOMFCADDDPB::OOMFCADDDPB>(
            "BCLCJEJHGAE",
            KMMHJDNCBAB::has_BCLCJEJHGAE,
            KMMHJDNCBAB::BCLCJEJHGAE,
            KMMHJDNCBAB::mut_BCLCJEJHGAE,
            KMMHJDNCBAB::set_BCLCJEJHGAE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KMOOALCLOLN::KMOOALCLOLN>(
            "MACIMGBEBDD",
            KMMHJDNCBAB::has_MACIMGBEBDD,
            KMMHJDNCBAB::MACIMGBEBDD,
            KMMHJDNCBAB::mut_MACIMGBEBDD,
            KMMHJDNCBAB::set_MACIMGBEBDD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KMFIKFJIEMO::KMFIKFJIEMO>(
            "CBBDBJGJAMO",
            KMMHJDNCBAB::has_CBBDBJGJAMO,
            KMMHJDNCBAB::CBBDBJGJAMO,
            KMMHJDNCBAB::mut_CBBDBJGJAMO,
            KMMHJDNCBAB::set_CBBDBJGJAMO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MIMLAKLPCPL::MIMLAKLPCPL>(
            "FKOEDJJOJGB",
            KMMHJDNCBAB::has_FKOEDJJOJGB,
            KMMHJDNCBAB::FKOEDJJOJGB,
            KMMHJDNCBAB::mut_FKOEDJJOJGB,
            KMMHJDNCBAB::set_FKOEDJJOJGB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LAGHMBLEMJJ::LAGHMBLEMJJ>(
            "GJOAKDGLLLL",
            KMMHJDNCBAB::has_GJOAKDGLLLL,
            KMMHJDNCBAB::GJOAKDGLLLL,
            KMMHJDNCBAB::mut_GJOAKDGLLLL,
            KMMHJDNCBAB::set_GJOAKDGLLLL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DAOPAFHNNAH::DAOPAFHNNAH>(
            "CEPLFBBJIGK",
            KMMHJDNCBAB::has_CEPLFBBJIGK,
            KMMHJDNCBAB::CEPLFBBJIGK,
            KMMHJDNCBAB::mut_CEPLFBBJIGK,
            KMMHJDNCBAB::set_CEPLFBBJIGK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FHEJDFGDPMO::FHEJDFGDPMO>(
            "MOJKOEDKGAO",
            KMMHJDNCBAB::has_MOJKOEDKGAO,
            KMMHJDNCBAB::MOJKOEDKGAO,
            KMMHJDNCBAB::mut_MOJKOEDKGAO,
            KMMHJDNCBAB::set_MOJKOEDKGAO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LEPMIKONNPH::LEPMIKONNPH>(
            "DCPPMNOKCOC",
            KMMHJDNCBAB::has_DCPPMNOKCOC,
            KMMHJDNCBAB::DCPPMNOKCOC,
            KMMHJDNCBAB::mut_DCPPMNOKCOC,
            KMMHJDNCBAB::set_DCPPMNOKCOC,
        ));
        oneofs.push(kmmhjdncbab::NBMKIGLFBJM::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KMMHJDNCBAB>(
            "KMMHJDNCBAB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KMMHJDNCBAB {
    const NAME: &'static str = "KMMHJDNCBAB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(is.read_message()?));
                },
                26 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(is.read_message()?));
                },
                74 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(is.read_message()?));
                },
                66 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(is.read_message()?));
                },
                82 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(is.read_message()?));
                },
                98 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(is.read_message()?));
                },
                106 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(is.read_message()?));
                },
                122 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(is.read_message()?));
                },
                34 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(is.read_message()?));
                },
                114 => {
                    self.NBMKIGLFBJM = ::std::option::Option::Some(kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.NBMKIGLFBJM {
            match v {
                &kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.NBMKIGLFBJM {
            match v {
                &kmmhjdncbab::NBMKIGLFBJM::ENNABLPIAEE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::EKDHJFNNNNB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::BCLCJEJHGAE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::MACIMGBEBDD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::CBBDBJGJAMO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::FKOEDJJOJGB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::GJOAKDGLLLL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::CEPLFBBJIGK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::MOJKOEDKGAO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &kmmhjdncbab::NBMKIGLFBJM::DCPPMNOKCOC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> KMMHJDNCBAB {
        KMMHJDNCBAB::new()
    }

    fn clear(&mut self) {
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.NBMKIGLFBJM = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KMMHJDNCBAB {
        static instance: KMMHJDNCBAB = KMMHJDNCBAB {
            NBMKIGLFBJM: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KMMHJDNCBAB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KMMHJDNCBAB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KMMHJDNCBAB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KMMHJDNCBAB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `KMMHJDNCBAB`
pub mod kmmhjdncbab {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:KMMHJDNCBAB.NBMKIGLFBJM)
    pub enum NBMKIGLFBJM {
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.ENNABLPIAEE)
        ENNABLPIAEE(super::super::PFCCJJJLPIC::PFCCJJJLPIC),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.EKDHJFNNNNB)
        EKDHJFNNNNB(super::super::JJPPOOADFOO::JJPPOOADFOO),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.BCLCJEJHGAE)
        BCLCJEJHGAE(super::super::OOMFCADDDPB::OOMFCADDDPB),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.MACIMGBEBDD)
        MACIMGBEBDD(super::super::KMOOALCLOLN::KMOOALCLOLN),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.CBBDBJGJAMO)
        CBBDBJGJAMO(super::super::KMFIKFJIEMO::KMFIKFJIEMO),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.FKOEDJJOJGB)
        FKOEDJJOJGB(super::super::MIMLAKLPCPL::MIMLAKLPCPL),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.GJOAKDGLLLL)
        GJOAKDGLLLL(super::super::LAGHMBLEMJJ::LAGHMBLEMJJ),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.CEPLFBBJIGK)
        CEPLFBBJIGK(super::super::DAOPAFHNNAH::DAOPAFHNNAH),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.MOJKOEDKGAO)
        MOJKOEDKGAO(super::super::FHEJDFGDPMO::FHEJDFGDPMO),
        // @@protoc_insertion_point(oneof_field:KMMHJDNCBAB.DCPPMNOKCOC)
        DCPPMNOKCOC(super::super::LEPMIKONNPH::LEPMIKONNPH),
    }

    impl ::protobuf::Oneof for NBMKIGLFBJM {
    }

    impl ::protobuf::OneofFull for NBMKIGLFBJM {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::KMMHJDNCBAB as ::protobuf::MessageFull>::descriptor().oneof_by_name("NBMKIGLFBJM").unwrap()).clone()
        }
    }

    impl NBMKIGLFBJM {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NBMKIGLFBJM>("NBMKIGLFBJM")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KMMHJDNCBAB.proto\x1a\x11DAOPAFHNNAH.proto\x1a\x11FHEJDFGDPMO.prot\
    o\x1a\x11JJPPOOADFOO.proto\x1a\x11KMFIKFJIEMO.proto\x1a\x11KMOOALCLOLN.p\
    roto\x1a\x11LAGHMBLEMJJ.proto\x1a\x11LEPMIKONNPH.proto\x1a\x11MIMLAKLPCP\
    L.proto\x1a\x11OOMFCADDDPB.proto\x1a\x11PFCCJJJLPIC.proto\"\x90\x04\n\
    \x0bKMMHJDNCBAB\x120\n\x0bENNABLPIAEE\x18\x07\x20\x01(\x0b2\x0c.PFCCJJJL\
    PICH\0R\x0bENNABLPIAEE\x120\n\x0bEKDHJFNNNNB\x18\x03\x20\x01(\x0b2\x0c.J\
    JPPOOADFOOH\0R\x0bEKDHJFNNNNB\x120\n\x0bBCLCJEJHGAE\x18\t\x20\x01(\x0b2\
    \x0c.OOMFCADDDPBH\0R\x0bBCLCJEJHGAE\x120\n\x0bMACIMGBEBDD\x18\x08\x20\
    \x01(\x0b2\x0c.KMOOALCLOLNH\0R\x0bMACIMGBEBDD\x120\n\x0bCBBDBJGJAMO\x18\
    \n\x20\x01(\x0b2\x0c.KMFIKFJIEMOH\0R\x0bCBBDBJGJAMO\x120\n\x0bFKOEDJJOJG\
    B\x18\x0c\x20\x01(\x0b2\x0c.MIMLAKLPCPLH\0R\x0bFKOEDJJOJGB\x120\n\x0bGJO\
    AKDGLLLL\x18\r\x20\x01(\x0b2\x0c.LAGHMBLEMJJH\0R\x0bGJOAKDGLLLL\x120\n\
    \x0bCEPLFBBJIGK\x18\x0f\x20\x01(\x0b2\x0c.DAOPAFHNNAHH\0R\x0bCEPLFBBJIGK\
    \x120\n\x0bMOJKOEDKGAO\x18\x04\x20\x01(\x0b2\x0c.FHEJDFGDPMOH\0R\x0bMOJK\
    OEDKGAO\x120\n\x0bDCPPMNOKCOC\x18\x0e\x20\x01(\x0b2\x0c.LEPMIKONNPHH\0R\
    \x0bDCPPMNOKCOCB\r\n\x0bNBMKIGLFBJMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(10);
            deps.push(super::DAOPAFHNNAH::file_descriptor().clone());
            deps.push(super::FHEJDFGDPMO::file_descriptor().clone());
            deps.push(super::JJPPOOADFOO::file_descriptor().clone());
            deps.push(super::KMFIKFJIEMO::file_descriptor().clone());
            deps.push(super::KMOOALCLOLN::file_descriptor().clone());
            deps.push(super::LAGHMBLEMJJ::file_descriptor().clone());
            deps.push(super::LEPMIKONNPH::file_descriptor().clone());
            deps.push(super::MIMLAKLPCPL::file_descriptor().clone());
            deps.push(super::OOMFCADDDPB::file_descriptor().clone());
            deps.push(super::PFCCJJJLPIC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KMMHJDNCBAB::generated_message_descriptor_data());
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
