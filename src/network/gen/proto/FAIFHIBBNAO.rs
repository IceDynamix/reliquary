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

//! Generated file from `FAIFHIBBNAO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FAIFHIBBNAO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FAIFHIBBNAO {
    // message oneof groups
    pub ELPNOOCAFBE: ::std::option::Option<faifhibbnao::ELPNOOCAFBE>,
    // special fields
    // @@protoc_insertion_point(special_field:FAIFHIBBNAO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FAIFHIBBNAO {
    fn default() -> &'a FAIFHIBBNAO {
        <FAIFHIBBNAO as ::protobuf::Message>::default_instance()
    }
}

impl FAIFHIBBNAO {
    pub fn new() -> FAIFHIBBNAO {
        ::std::default::Default::default()
    }

    // .CNHFMAFDIFF OIIIHMDGMJO = 5;

    pub fn OIIIHMDGMJO(&self) -> &super::CNHFMAFDIFF::CNHFMAFDIFF {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(ref v)) => v,
            _ => <super::CNHFMAFDIFF::CNHFMAFDIFF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OIIIHMDGMJO(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_OIIIHMDGMJO(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OIIIHMDGMJO(&mut self, v: super::CNHFMAFDIFF::CNHFMAFDIFF) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OIIIHMDGMJO(&mut self) -> &mut super::CNHFMAFDIFF::CNHFMAFDIFF {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(super::CNHFMAFDIFF::CNHFMAFDIFF::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OIIIHMDGMJO(&mut self) -> super::CNHFMAFDIFF::CNHFMAFDIFF {
        if self.has_OIIIHMDGMJO() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CNHFMAFDIFF::CNHFMAFDIFF::new()
        }
    }

    // .OAIALNNOCHL LJAOJFFEADO = 10;

    pub fn LJAOJFFEADO(&self) -> &super::OAIALNNOCHL::OAIALNNOCHL {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(ref v)) => v,
            _ => <super::OAIALNNOCHL::OAIALNNOCHL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LJAOJFFEADO(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_LJAOJFFEADO(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LJAOJFFEADO(&mut self, v: super::OAIALNNOCHL::OAIALNNOCHL) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LJAOJFFEADO(&mut self) -> &mut super::OAIALNNOCHL::OAIALNNOCHL {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(super::OAIALNNOCHL::OAIALNNOCHL::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LJAOJFFEADO(&mut self) -> super::OAIALNNOCHL::OAIALNNOCHL {
        if self.has_LJAOJFFEADO() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::OAIALNNOCHL::OAIALNNOCHL::new()
        }
    }

    // .GCFFCCNLEMH KIKHALDIMFD = 9;

    pub fn KIKHALDIMFD(&self) -> &super::GCFFCCNLEMH::GCFFCCNLEMH {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(ref v)) => v,
            _ => <super::GCFFCCNLEMH::GCFFCCNLEMH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KIKHALDIMFD(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_KIKHALDIMFD(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KIKHALDIMFD(&mut self, v: super::GCFFCCNLEMH::GCFFCCNLEMH) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KIKHALDIMFD(&mut self) -> &mut super::GCFFCCNLEMH::GCFFCCNLEMH {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(super::GCFFCCNLEMH::GCFFCCNLEMH::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KIKHALDIMFD(&mut self) -> super::GCFFCCNLEMH::GCFFCCNLEMH {
        if self.has_KIKHALDIMFD() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GCFFCCNLEMH::GCFFCCNLEMH::new()
        }
    }

    // .MJCNJHBMCLM KCDBLDHFADE = 3;

    pub fn KCDBLDHFADE(&self) -> &super::MJCNJHBMCLM::MJCNJHBMCLM {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(ref v)) => v,
            _ => <super::MJCNJHBMCLM::MJCNJHBMCLM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KCDBLDHFADE(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_KCDBLDHFADE(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KCDBLDHFADE(&mut self, v: super::MJCNJHBMCLM::MJCNJHBMCLM) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KCDBLDHFADE(&mut self) -> &mut super::MJCNJHBMCLM::MJCNJHBMCLM {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(super::MJCNJHBMCLM::MJCNJHBMCLM::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KCDBLDHFADE(&mut self) -> super::MJCNJHBMCLM::MJCNJHBMCLM {
        if self.has_KCDBLDHFADE() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::MJCNJHBMCLM::MJCNJHBMCLM::new()
        }
    }

    // .NDDBKIHEOPD EHEGLHKHMNP = 2;

    pub fn EHEGLHKHMNP(&self) -> &super::NDDBKIHEOPD::NDDBKIHEOPD {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(ref v)) => v,
            _ => <super::NDDBKIHEOPD::NDDBKIHEOPD as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_EHEGLHKHMNP(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_EHEGLHKHMNP(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_EHEGLHKHMNP(&mut self, v: super::NDDBKIHEOPD::NDDBKIHEOPD) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_EHEGLHKHMNP(&mut self) -> &mut super::NDDBKIHEOPD::NDDBKIHEOPD {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(super::NDDBKIHEOPD::NDDBKIHEOPD::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_EHEGLHKHMNP(&mut self) -> super::NDDBKIHEOPD::NDDBKIHEOPD {
        if self.has_EHEGLHKHMNP() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NDDBKIHEOPD::NDDBKIHEOPD::new()
        }
    }

    // .IHEMHBKFEOB LOEMPIHEMEE = 4;

    pub fn LOEMPIHEMEE(&self) -> &super::IHEMHBKFEOB::IHEMHBKFEOB {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(ref v)) => v,
            _ => <super::IHEMHBKFEOB::IHEMHBKFEOB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LOEMPIHEMEE(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_LOEMPIHEMEE(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LOEMPIHEMEE(&mut self, v: super::IHEMHBKFEOB::IHEMHBKFEOB) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LOEMPIHEMEE(&mut self) -> &mut super::IHEMHBKFEOB::IHEMHBKFEOB {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(super::IHEMHBKFEOB::IHEMHBKFEOB::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LOEMPIHEMEE(&mut self) -> super::IHEMHBKFEOB::IHEMHBKFEOB {
        if self.has_LOEMPIHEMEE() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IHEMHBKFEOB::IHEMHBKFEOB::new()
        }
    }

    // .FMKPLCOGLPF FBBMPMJIELF = 13;

    pub fn FBBMPMJIELF(&self) -> &super::FMKPLCOGLPF::FMKPLCOGLPF {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(ref v)) => v,
            _ => <super::FMKPLCOGLPF::FMKPLCOGLPF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FBBMPMJIELF(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_FBBMPMJIELF(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FBBMPMJIELF(&mut self, v: super::FMKPLCOGLPF::FMKPLCOGLPF) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FBBMPMJIELF(&mut self) -> &mut super::FMKPLCOGLPF::FMKPLCOGLPF {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(super::FMKPLCOGLPF::FMKPLCOGLPF::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FBBMPMJIELF(&mut self) -> super::FMKPLCOGLPF::FMKPLCOGLPF {
        if self.has_FBBMPMJIELF() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FMKPLCOGLPF::FMKPLCOGLPF::new()
        }
    }

    // .ENBCCINOJNE NHIGIPHKGEA = 6;

    pub fn NHIGIPHKGEA(&self) -> &super::ENBCCINOJNE::ENBCCINOJNE {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(ref v)) => v,
            _ => <super::ENBCCINOJNE::ENBCCINOJNE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NHIGIPHKGEA(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_NHIGIPHKGEA(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NHIGIPHKGEA(&mut self, v: super::ENBCCINOJNE::ENBCCINOJNE) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NHIGIPHKGEA(&mut self) -> &mut super::ENBCCINOJNE::ENBCCINOJNE {
        if let ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(super::ENBCCINOJNE::ENBCCINOJNE::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NHIGIPHKGEA(&mut self) -> super::ENBCCINOJNE::ENBCCINOJNE {
        if self.has_NHIGIPHKGEA() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ENBCCINOJNE::ENBCCINOJNE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CNHFMAFDIFF::CNHFMAFDIFF>(
            "OIIIHMDGMJO",
            FAIFHIBBNAO::has_OIIIHMDGMJO,
            FAIFHIBBNAO::OIIIHMDGMJO,
            FAIFHIBBNAO::mut_OIIIHMDGMJO,
            FAIFHIBBNAO::set_OIIIHMDGMJO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::OAIALNNOCHL::OAIALNNOCHL>(
            "LJAOJFFEADO",
            FAIFHIBBNAO::has_LJAOJFFEADO,
            FAIFHIBBNAO::LJAOJFFEADO,
            FAIFHIBBNAO::mut_LJAOJFFEADO,
            FAIFHIBBNAO::set_LJAOJFFEADO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GCFFCCNLEMH::GCFFCCNLEMH>(
            "KIKHALDIMFD",
            FAIFHIBBNAO::has_KIKHALDIMFD,
            FAIFHIBBNAO::KIKHALDIMFD,
            FAIFHIBBNAO::mut_KIKHALDIMFD,
            FAIFHIBBNAO::set_KIKHALDIMFD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::MJCNJHBMCLM::MJCNJHBMCLM>(
            "KCDBLDHFADE",
            FAIFHIBBNAO::has_KCDBLDHFADE,
            FAIFHIBBNAO::KCDBLDHFADE,
            FAIFHIBBNAO::mut_KCDBLDHFADE,
            FAIFHIBBNAO::set_KCDBLDHFADE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NDDBKIHEOPD::NDDBKIHEOPD>(
            "EHEGLHKHMNP",
            FAIFHIBBNAO::has_EHEGLHKHMNP,
            FAIFHIBBNAO::EHEGLHKHMNP,
            FAIFHIBBNAO::mut_EHEGLHKHMNP,
            FAIFHIBBNAO::set_EHEGLHKHMNP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IHEMHBKFEOB::IHEMHBKFEOB>(
            "LOEMPIHEMEE",
            FAIFHIBBNAO::has_LOEMPIHEMEE,
            FAIFHIBBNAO::LOEMPIHEMEE,
            FAIFHIBBNAO::mut_LOEMPIHEMEE,
            FAIFHIBBNAO::set_LOEMPIHEMEE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FMKPLCOGLPF::FMKPLCOGLPF>(
            "FBBMPMJIELF",
            FAIFHIBBNAO::has_FBBMPMJIELF,
            FAIFHIBBNAO::FBBMPMJIELF,
            FAIFHIBBNAO::mut_FBBMPMJIELF,
            FAIFHIBBNAO::set_FBBMPMJIELF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ENBCCINOJNE::ENBCCINOJNE>(
            "NHIGIPHKGEA",
            FAIFHIBBNAO::has_NHIGIPHKGEA,
            FAIFHIBBNAO::NHIGIPHKGEA,
            FAIFHIBBNAO::mut_NHIGIPHKGEA,
            FAIFHIBBNAO::set_NHIGIPHKGEA,
        ));
        oneofs.push(faifhibbnao::ELPNOOCAFBE::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FAIFHIBBNAO>(
            "FAIFHIBBNAO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FAIFHIBBNAO {
    const NAME: &'static str = "FAIFHIBBNAO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(is.read_message()?));
                },
                82 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(is.read_message()?));
                },
                74 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(is.read_message()?));
                },
                26 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(is.read_message()?));
                },
                18 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(is.read_message()?));
                },
                34 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(is.read_message()?));
                },
                106 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(is.read_message()?));
                },
                50 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.ELPNOOCAFBE {
            match v {
                &faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.ELPNOOCAFBE {
            match v {
                &faifhibbnao::ELPNOOCAFBE::OIIIHMDGMJO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::LJAOJFFEADO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::KIKHALDIMFD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::KCDBLDHFADE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::EHEGLHKHMNP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::LOEMPIHEMEE(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::FBBMPMJIELF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &faifhibbnao::ELPNOOCAFBE::NHIGIPHKGEA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> FAIFHIBBNAO {
        FAIFHIBBNAO::new()
    }

    fn clear(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FAIFHIBBNAO {
        static instance: FAIFHIBBNAO = FAIFHIBBNAO {
            ELPNOOCAFBE: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FAIFHIBBNAO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FAIFHIBBNAO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FAIFHIBBNAO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FAIFHIBBNAO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FAIFHIBBNAO`
pub mod faifhibbnao {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FAIFHIBBNAO.ELPNOOCAFBE)
    pub enum ELPNOOCAFBE {
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.OIIIHMDGMJO)
        OIIIHMDGMJO(super::super::CNHFMAFDIFF::CNHFMAFDIFF),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.LJAOJFFEADO)
        LJAOJFFEADO(super::super::OAIALNNOCHL::OAIALNNOCHL),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.KIKHALDIMFD)
        KIKHALDIMFD(super::super::GCFFCCNLEMH::GCFFCCNLEMH),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.KCDBLDHFADE)
        KCDBLDHFADE(super::super::MJCNJHBMCLM::MJCNJHBMCLM),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.EHEGLHKHMNP)
        EHEGLHKHMNP(super::super::NDDBKIHEOPD::NDDBKIHEOPD),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.LOEMPIHEMEE)
        LOEMPIHEMEE(super::super::IHEMHBKFEOB::IHEMHBKFEOB),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.FBBMPMJIELF)
        FBBMPMJIELF(super::super::FMKPLCOGLPF::FMKPLCOGLPF),
        // @@protoc_insertion_point(oneof_field:FAIFHIBBNAO.NHIGIPHKGEA)
        NHIGIPHKGEA(super::super::ENBCCINOJNE::ENBCCINOJNE),
    }

    impl ::protobuf::Oneof for ELPNOOCAFBE {
    }

    impl ::protobuf::OneofFull for ELPNOOCAFBE {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FAIFHIBBNAO as ::protobuf::MessageFull>::descriptor().oneof_by_name("ELPNOOCAFBE").unwrap()).clone()
        }
    }

    impl ELPNOOCAFBE {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<ELPNOOCAFBE>("ELPNOOCAFBE")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FAIFHIBBNAO.proto\x1a\x11CNHFMAFDIFF.proto\x1a\x11ENBCCINOJNE.prot\
    o\x1a\x11FMKPLCOGLPF.proto\x1a\x11GCFFCCNLEMH.proto\x1a\x11IHEMHBKFEOB.p\
    roto\x1a\x11MJCNJHBMCLM.proto\x1a\x11NDDBKIHEOPD.proto\x1a\x11OAIALNNOCH\
    L.proto\"\xac\x03\n\x0bFAIFHIBBNAO\x120\n\x0bOIIIHMDGMJO\x18\x05\x20\x01\
    (\x0b2\x0c.CNHFMAFDIFFH\0R\x0bOIIIHMDGMJO\x120\n\x0bLJAOJFFEADO\x18\n\
    \x20\x01(\x0b2\x0c.OAIALNNOCHLH\0R\x0bLJAOJFFEADO\x120\n\x0bKIKHALDIMFD\
    \x18\t\x20\x01(\x0b2\x0c.GCFFCCNLEMHH\0R\x0bKIKHALDIMFD\x120\n\x0bKCDBLD\
    HFADE\x18\x03\x20\x01(\x0b2\x0c.MJCNJHBMCLMH\0R\x0bKCDBLDHFADE\x120\n\
    \x0bEHEGLHKHMNP\x18\x02\x20\x01(\x0b2\x0c.NDDBKIHEOPDH\0R\x0bEHEGLHKHMNP\
    \x120\n\x0bLOEMPIHEMEE\x18\x04\x20\x01(\x0b2\x0c.IHEMHBKFEOBH\0R\x0bLOEM\
    PIHEMEE\x120\n\x0bFBBMPMJIELF\x18\r\x20\x01(\x0b2\x0c.FMKPLCOGLPFH\0R\
    \x0bFBBMPMJIELF\x120\n\x0bNHIGIPHKGEA\x18\x06\x20\x01(\x0b2\x0c.ENBCCINO\
    JNEH\0R\x0bNHIGIPHKGEAB\r\n\x0bELPNOOCAFBEb\x06proto3\
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
            deps.push(super::CNHFMAFDIFF::file_descriptor().clone());
            deps.push(super::ENBCCINOJNE::file_descriptor().clone());
            deps.push(super::FMKPLCOGLPF::file_descriptor().clone());
            deps.push(super::GCFFCCNLEMH::file_descriptor().clone());
            deps.push(super::IHEMHBKFEOB::file_descriptor().clone());
            deps.push(super::MJCNJHBMCLM::file_descriptor().clone());
            deps.push(super::NDDBKIHEOPD::file_descriptor().clone());
            deps.push(super::OAIALNNOCHL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FAIFHIBBNAO::generated_message_descriptor_data());
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
