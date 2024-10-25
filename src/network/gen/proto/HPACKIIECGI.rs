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

//! Generated file from `HPACKIIECGI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HPACKIIECGI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HPACKIIECGI {
    // message oneof groups
    pub BJAKONLBDOG: ::std::option::Option<hpackiiecgi::BJAKONLBDOG>,
    // special fields
    // @@protoc_insertion_point(special_field:HPACKIIECGI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HPACKIIECGI {
    fn default() -> &'a HPACKIIECGI {
        <HPACKIIECGI as ::protobuf::Message>::default_instance()
    }
}

impl HPACKIIECGI {
    pub fn new() -> HPACKIIECGI {
        ::std::default::Default::default()
    }

    // .JJBCEMNDHDH JMDIKHHFHOL = 13;

    pub fn JMDIKHHFHOL(&self) -> &super::JJBCEMNDHDH::JJBCEMNDHDH {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(ref v)) => v,
            _ => <super::JJBCEMNDHDH::JJBCEMNDHDH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JMDIKHHFHOL(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_JMDIKHHFHOL(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JMDIKHHFHOL(&mut self, v: super::JJBCEMNDHDH::JJBCEMNDHDH) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JMDIKHHFHOL(&mut self) -> &mut super::JJBCEMNDHDH::JJBCEMNDHDH {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(super::JJBCEMNDHDH::JJBCEMNDHDH::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JMDIKHHFHOL(&mut self) -> super::JJBCEMNDHDH::JJBCEMNDHDH {
        if self.has_JMDIKHHFHOL() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JJBCEMNDHDH::JJBCEMNDHDH::new()
        }
    }

    // .HNNKHMPLHFL AHHOJCJBPFJ = 1;

    pub fn AHHOJCJBPFJ(&self) -> &super::HNNKHMPLHFL::HNNKHMPLHFL {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(ref v)) => v,
            _ => <super::HNNKHMPLHFL::HNNKHMPLHFL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_AHHOJCJBPFJ(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_AHHOJCJBPFJ(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AHHOJCJBPFJ(&mut self, v: super::HNNKHMPLHFL::HNNKHMPLHFL) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_AHHOJCJBPFJ(&mut self) -> &mut super::HNNKHMPLHFL::HNNKHMPLHFL {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(super::HNNKHMPLHFL::HNNKHMPLHFL::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_AHHOJCJBPFJ(&mut self) -> super::HNNKHMPLHFL::HNNKHMPLHFL {
        if self.has_AHHOJCJBPFJ() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HNNKHMPLHFL::HNNKHMPLHFL::new()
        }
    }

    // .GOAMENEAPNI ILJELJIFLHA = 5;

    pub fn ILJELJIFLHA(&self) -> &super::GOAMENEAPNI::GOAMENEAPNI {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(ref v)) => v,
            _ => <super::GOAMENEAPNI::GOAMENEAPNI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ILJELJIFLHA(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_ILJELJIFLHA(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ILJELJIFLHA(&mut self, v: super::GOAMENEAPNI::GOAMENEAPNI) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ILJELJIFLHA(&mut self) -> &mut super::GOAMENEAPNI::GOAMENEAPNI {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(super::GOAMENEAPNI::GOAMENEAPNI::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ILJELJIFLHA(&mut self) -> super::GOAMENEAPNI::GOAMENEAPNI {
        if self.has_ILJELJIFLHA() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GOAMENEAPNI::GOAMENEAPNI::new()
        }
    }

    // .JBBIBKOIEDB BHBMJNIHINK = 15;

    pub fn BHBMJNIHINK(&self) -> &super::JBBIBKOIEDB::JBBIBKOIEDB {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(ref v)) => v,
            _ => <super::JBBIBKOIEDB::JBBIBKOIEDB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BHBMJNIHINK(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_BHBMJNIHINK(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BHBMJNIHINK(&mut self, v: super::JBBIBKOIEDB::JBBIBKOIEDB) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BHBMJNIHINK(&mut self) -> &mut super::JBBIBKOIEDB::JBBIBKOIEDB {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(super::JBBIBKOIEDB::JBBIBKOIEDB::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BHBMJNIHINK(&mut self) -> super::JBBIBKOIEDB::JBBIBKOIEDB {
        if self.has_BHBMJNIHINK() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JBBIBKOIEDB::JBBIBKOIEDB::new()
        }
    }

    // .AMHGAANHDMN PBMJABBAGMM = 3;

    pub fn PBMJABBAGMM(&self) -> &super::AMHGAANHDMN::AMHGAANHDMN {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(ref v)) => v,
            _ => <super::AMHGAANHDMN::AMHGAANHDMN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PBMJABBAGMM(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_PBMJABBAGMM(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PBMJABBAGMM(&mut self, v: super::AMHGAANHDMN::AMHGAANHDMN) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PBMJABBAGMM(&mut self) -> &mut super::AMHGAANHDMN::AMHGAANHDMN {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(super::AMHGAANHDMN::AMHGAANHDMN::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PBMJABBAGMM(&mut self) -> super::AMHGAANHDMN::AMHGAANHDMN {
        if self.has_PBMJABBAGMM() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AMHGAANHDMN::AMHGAANHDMN::new()
        }
    }

    // .HMIDIIBGJLJ ENNMPOCKOEI = 8;

    pub fn ENNMPOCKOEI(&self) -> &super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(ref v)) => v,
            _ => <super::HMIDIIBGJLJ::HMIDIIBGJLJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ENNMPOCKOEI(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_ENNMPOCKOEI(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ENNMPOCKOEI(&mut self, v: super::HMIDIIBGJLJ::HMIDIIBGJLJ) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ENNMPOCKOEI(&mut self) -> &mut super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(super::HMIDIIBGJLJ::HMIDIIBGJLJ::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ENNMPOCKOEI(&mut self) -> super::HMIDIIBGJLJ::HMIDIIBGJLJ {
        if self.has_ENNMPOCKOEI() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HMIDIIBGJLJ::HMIDIIBGJLJ::new()
        }
    }

    // .JHEMAFBELBK OPINEILCLOJ = 9;

    pub fn OPINEILCLOJ(&self) -> &super::JHEMAFBELBK::JHEMAFBELBK {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(ref v)) => v,
            _ => <super::JHEMAFBELBK::JHEMAFBELBK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OPINEILCLOJ(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_OPINEILCLOJ(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OPINEILCLOJ(&mut self, v: super::JHEMAFBELBK::JHEMAFBELBK) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OPINEILCLOJ(&mut self) -> &mut super::JHEMAFBELBK::JHEMAFBELBK {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(super::JHEMAFBELBK::JHEMAFBELBK::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OPINEILCLOJ(&mut self) -> super::JHEMAFBELBK::JHEMAFBELBK {
        if self.has_OPINEILCLOJ() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JHEMAFBELBK::JHEMAFBELBK::new()
        }
    }

    // .JHHKEBFANLL HCJIDCPLOGD = 4;

    pub fn HCJIDCPLOGD(&self) -> &super::JHHKEBFANLL::JHHKEBFANLL {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(ref v)) => v,
            _ => <super::JHHKEBFANLL::JHHKEBFANLL as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_HCJIDCPLOGD(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
    }

    pub fn has_HCJIDCPLOGD(&self) -> bool {
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HCJIDCPLOGD(&mut self, v: super::JHHKEBFANLL::JHHKEBFANLL) {
        self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_HCJIDCPLOGD(&mut self) -> &mut super::JHHKEBFANLL::JHHKEBFANLL {
        if let ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(_)) = self.BJAKONLBDOG {
        } else {
            self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(super::JHHKEBFANLL::JHHKEBFANLL::new()));
        }
        match self.BJAKONLBDOG {
            ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_HCJIDCPLOGD(&mut self) -> super::JHHKEBFANLL::JHHKEBFANLL {
        if self.has_HCJIDCPLOGD() {
            match self.BJAKONLBDOG.take() {
                ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JHHKEBFANLL::JHHKEBFANLL::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JJBCEMNDHDH::JJBCEMNDHDH>(
            "JMDIKHHFHOL",
            HPACKIIECGI::has_JMDIKHHFHOL,
            HPACKIIECGI::JMDIKHHFHOL,
            HPACKIIECGI::mut_JMDIKHHFHOL,
            HPACKIIECGI::set_JMDIKHHFHOL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HNNKHMPLHFL::HNNKHMPLHFL>(
            "AHHOJCJBPFJ",
            HPACKIIECGI::has_AHHOJCJBPFJ,
            HPACKIIECGI::AHHOJCJBPFJ,
            HPACKIIECGI::mut_AHHOJCJBPFJ,
            HPACKIIECGI::set_AHHOJCJBPFJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GOAMENEAPNI::GOAMENEAPNI>(
            "ILJELJIFLHA",
            HPACKIIECGI::has_ILJELJIFLHA,
            HPACKIIECGI::ILJELJIFLHA,
            HPACKIIECGI::mut_ILJELJIFLHA,
            HPACKIIECGI::set_ILJELJIFLHA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JBBIBKOIEDB::JBBIBKOIEDB>(
            "BHBMJNIHINK",
            HPACKIIECGI::has_BHBMJNIHINK,
            HPACKIIECGI::BHBMJNIHINK,
            HPACKIIECGI::mut_BHBMJNIHINK,
            HPACKIIECGI::set_BHBMJNIHINK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AMHGAANHDMN::AMHGAANHDMN>(
            "PBMJABBAGMM",
            HPACKIIECGI::has_PBMJABBAGMM,
            HPACKIIECGI::PBMJABBAGMM,
            HPACKIIECGI::mut_PBMJABBAGMM,
            HPACKIIECGI::set_PBMJABBAGMM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HMIDIIBGJLJ::HMIDIIBGJLJ>(
            "ENNMPOCKOEI",
            HPACKIIECGI::has_ENNMPOCKOEI,
            HPACKIIECGI::ENNMPOCKOEI,
            HPACKIIECGI::mut_ENNMPOCKOEI,
            HPACKIIECGI::set_ENNMPOCKOEI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JHEMAFBELBK::JHEMAFBELBK>(
            "OPINEILCLOJ",
            HPACKIIECGI::has_OPINEILCLOJ,
            HPACKIIECGI::OPINEILCLOJ,
            HPACKIIECGI::mut_OPINEILCLOJ,
            HPACKIIECGI::set_OPINEILCLOJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JHHKEBFANLL::JHHKEBFANLL>(
            "HCJIDCPLOGD",
            HPACKIIECGI::has_HCJIDCPLOGD,
            HPACKIIECGI::HCJIDCPLOGD,
            HPACKIIECGI::mut_HCJIDCPLOGD,
            HPACKIIECGI::set_HCJIDCPLOGD,
        ));
        oneofs.push(hpackiiecgi::BJAKONLBDOG::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HPACKIIECGI>(
            "HPACKIIECGI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HPACKIIECGI {
    const NAME: &'static str = "HPACKIIECGI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(is.read_message()?));
                },
                10 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(is.read_message()?));
                },
                42 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(is.read_message()?));
                },
                122 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(is.read_message()?));
                },
                26 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(is.read_message()?));
                },
                66 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(is.read_message()?));
                },
                74 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(is.read_message()?));
                },
                34 => {
                    self.BJAKONLBDOG = ::std::option::Option::Some(hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.BJAKONLBDOG {
            match v {
                &hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.BJAKONLBDOG {
            match v {
                &hpackiiecgi::BJAKONLBDOG::JMDIKHHFHOL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::AHHOJCJBPFJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::ILJELJIFLHA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::BHBMJNIHINK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::PBMJABBAGMM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::ENNMPOCKOEI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::OPINEILCLOJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &hpackiiecgi::BJAKONLBDOG::HCJIDCPLOGD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> HPACKIIECGI {
        HPACKIIECGI::new()
    }

    fn clear(&mut self) {
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.BJAKONLBDOG = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HPACKIIECGI {
        static instance: HPACKIIECGI = HPACKIIECGI {
            BJAKONLBDOG: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HPACKIIECGI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HPACKIIECGI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HPACKIIECGI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HPACKIIECGI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HPACKIIECGI`
pub mod hpackiiecgi {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:HPACKIIECGI.BJAKONLBDOG)
    pub enum BJAKONLBDOG {
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.JMDIKHHFHOL)
        JMDIKHHFHOL(super::super::JJBCEMNDHDH::JJBCEMNDHDH),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.AHHOJCJBPFJ)
        AHHOJCJBPFJ(super::super::HNNKHMPLHFL::HNNKHMPLHFL),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.ILJELJIFLHA)
        ILJELJIFLHA(super::super::GOAMENEAPNI::GOAMENEAPNI),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.BHBMJNIHINK)
        BHBMJNIHINK(super::super::JBBIBKOIEDB::JBBIBKOIEDB),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.PBMJABBAGMM)
        PBMJABBAGMM(super::super::AMHGAANHDMN::AMHGAANHDMN),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.ENNMPOCKOEI)
        ENNMPOCKOEI(super::super::HMIDIIBGJLJ::HMIDIIBGJLJ),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.OPINEILCLOJ)
        OPINEILCLOJ(super::super::JHEMAFBELBK::JHEMAFBELBK),
        // @@protoc_insertion_point(oneof_field:HPACKIIECGI.HCJIDCPLOGD)
        HCJIDCPLOGD(super::super::JHHKEBFANLL::JHHKEBFANLL),
    }

    impl ::protobuf::Oneof for BJAKONLBDOG {
    }

    impl ::protobuf::OneofFull for BJAKONLBDOG {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HPACKIIECGI as ::protobuf::MessageFull>::descriptor().oneof_by_name("BJAKONLBDOG").unwrap()).clone()
        }
    }

    impl BJAKONLBDOG {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<BJAKONLBDOG>("BJAKONLBDOG")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HPACKIIECGI.proto\x1a\x11AMHGAANHDMN.proto\x1a\x11GOAMENEAPNI.prot\
    o\x1a\x11HMIDIIBGJLJ.proto\x1a\x11HNNKHMPLHFL.proto\x1a\x11JBBIBKOIEDB.p\
    roto\x1a\x11JHEMAFBELBK.proto\x1a\x11JHHKEBFANLL.proto\x1a\x11JJBCEMNDHD\
    H.proto\"\xac\x03\n\x0bHPACKIIECGI\x120\n\x0bJMDIKHHFHOL\x18\r\x20\x01(\
    \x0b2\x0c.JJBCEMNDHDHH\0R\x0bJMDIKHHFHOL\x120\n\x0bAHHOJCJBPFJ\x18\x01\
    \x20\x01(\x0b2\x0c.HNNKHMPLHFLH\0R\x0bAHHOJCJBPFJ\x120\n\x0bILJELJIFLHA\
    \x18\x05\x20\x01(\x0b2\x0c.GOAMENEAPNIH\0R\x0bILJELJIFLHA\x120\n\x0bBHBM\
    JNIHINK\x18\x0f\x20\x01(\x0b2\x0c.JBBIBKOIEDBH\0R\x0bBHBMJNIHINK\x120\n\
    \x0bPBMJABBAGMM\x18\x03\x20\x01(\x0b2\x0c.AMHGAANHDMNH\0R\x0bPBMJABBAGMM\
    \x120\n\x0bENNMPOCKOEI\x18\x08\x20\x01(\x0b2\x0c.HMIDIIBGJLJH\0R\x0bENNM\
    POCKOEI\x120\n\x0bOPINEILCLOJ\x18\t\x20\x01(\x0b2\x0c.JHEMAFBELBKH\0R\
    \x0bOPINEILCLOJ\x120\n\x0bHCJIDCPLOGD\x18\x04\x20\x01(\x0b2\x0c.JHHKEBFA\
    NLLH\0R\x0bHCJIDCPLOGDB\r\n\x0bBJAKONLBDOGb\x06proto3\
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
            deps.push(super::AMHGAANHDMN::file_descriptor().clone());
            deps.push(super::GOAMENEAPNI::file_descriptor().clone());
            deps.push(super::HMIDIIBGJLJ::file_descriptor().clone());
            deps.push(super::HNNKHMPLHFL::file_descriptor().clone());
            deps.push(super::JBBIBKOIEDB::file_descriptor().clone());
            deps.push(super::JHEMAFBELBK::file_descriptor().clone());
            deps.push(super::JHHKEBFANLL::file_descriptor().clone());
            deps.push(super::JJBCEMNDHDH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HPACKIIECGI::generated_message_descriptor_data());
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
