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

//! Generated file from `ClockParkHandleWaitOperationCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ClockParkHandleWaitOperationCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClockParkHandleWaitOperationCsReq {
    // message fields
    // @@protoc_insertion_point(field:ClockParkHandleWaitOperationCsReq.CLKEOEHPLNG)
    pub CLKEOEHPLNG: u32,
    // @@protoc_insertion_point(field:ClockParkHandleWaitOperationCsReq.AHIDJBJGGPP)
    pub AHIDJBJGGPP: u32,
    // message oneof groups
    pub DEJMPMFHIOC: ::std::option::Option<clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC>,
    // special fields
    // @@protoc_insertion_point(special_field:ClockParkHandleWaitOperationCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClockParkHandleWaitOperationCsReq {
    fn default() -> &'a ClockParkHandleWaitOperationCsReq {
        <ClockParkHandleWaitOperationCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ClockParkHandleWaitOperationCsReq {
    pub fn new() -> ClockParkHandleWaitOperationCsReq {
        ::std::default::Default::default()
    }

    // .OBNONMHMECK JFBCKCLPAKO = 5;

    pub fn JFBCKCLPAKO(&self) -> &super::OBNONMHMECK::OBNONMHMECK {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(ref v)) => v,
            _ => <super::OBNONMHMECK::OBNONMHMECK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JFBCKCLPAKO(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_JFBCKCLPAKO(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JFBCKCLPAKO(&mut self, v: super::OBNONMHMECK::OBNONMHMECK) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JFBCKCLPAKO(&mut self) -> &mut super::OBNONMHMECK::OBNONMHMECK {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(super::OBNONMHMECK::OBNONMHMECK::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JFBCKCLPAKO(&mut self) -> super::OBNONMHMECK::OBNONMHMECK {
        if self.has_JFBCKCLPAKO() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::OBNONMHMECK::OBNONMHMECK::new()
        }
    }

    // .FFOMIBNCFKI IPIKFLCEFLA = 9;

    pub fn IPIKFLCEFLA(&self) -> &super::FFOMIBNCFKI::FFOMIBNCFKI {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(ref v)) => v,
            _ => <super::FFOMIBNCFKI::FFOMIBNCFKI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IPIKFLCEFLA(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_IPIKFLCEFLA(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IPIKFLCEFLA(&mut self, v: super::FFOMIBNCFKI::FFOMIBNCFKI) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IPIKFLCEFLA(&mut self) -> &mut super::FFOMIBNCFKI::FFOMIBNCFKI {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(super::FFOMIBNCFKI::FFOMIBNCFKI::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IPIKFLCEFLA(&mut self) -> super::FFOMIBNCFKI::FFOMIBNCFKI {
        if self.has_IPIKFLCEFLA() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FFOMIBNCFKI::FFOMIBNCFKI::new()
        }
    }

    // .AJEHAMDABNA MEGNBBFILNL = 7;

    pub fn MEGNBBFILNL(&self) -> &super::AJEHAMDABNA::AJEHAMDABNA {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(ref v)) => v,
            _ => <super::AJEHAMDABNA::AJEHAMDABNA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MEGNBBFILNL(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_MEGNBBFILNL(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MEGNBBFILNL(&mut self, v: super::AJEHAMDABNA::AJEHAMDABNA) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MEGNBBFILNL(&mut self) -> &mut super::AJEHAMDABNA::AJEHAMDABNA {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(super::AJEHAMDABNA::AJEHAMDABNA::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MEGNBBFILNL(&mut self) -> super::AJEHAMDABNA::AJEHAMDABNA {
        if self.has_MEGNBBFILNL() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AJEHAMDABNA::AJEHAMDABNA::new()
        }
    }

    // .AMGHDCABJMJ ABKKDHAPCHN = 1;

    pub fn ABKKDHAPCHN(&self) -> &super::AMGHDCABJMJ::AMGHDCABJMJ {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(ref v)) => v,
            _ => <super::AMGHDCABJMJ::AMGHDCABJMJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ABKKDHAPCHN(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_ABKKDHAPCHN(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ABKKDHAPCHN(&mut self, v: super::AMGHDCABJMJ::AMGHDCABJMJ) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ABKKDHAPCHN(&mut self) -> &mut super::AMGHDCABJMJ::AMGHDCABJMJ {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(super::AMGHDCABJMJ::AMGHDCABJMJ::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ABKKDHAPCHN(&mut self) -> super::AMGHDCABJMJ::AMGHDCABJMJ {
        if self.has_ABKKDHAPCHN() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::AMGHDCABJMJ::AMGHDCABJMJ::new()
        }
    }

    // .INNNICFOLII LIHJMEINGIK = 6;

    pub fn LIHJMEINGIK(&self) -> &super::INNNICFOLII::INNNICFOLII {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(ref v)) => v,
            _ => <super::INNNICFOLII::INNNICFOLII as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LIHJMEINGIK(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_LIHJMEINGIK(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LIHJMEINGIK(&mut self, v: super::INNNICFOLII::INNNICFOLII) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LIHJMEINGIK(&mut self) -> &mut super::INNNICFOLII::INNNICFOLII {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(super::INNNICFOLII::INNNICFOLII::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LIHJMEINGIK(&mut self) -> super::INNNICFOLII::INNNICFOLII {
        if self.has_LIHJMEINGIK() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::INNNICFOLII::INNNICFOLII::new()
        }
    }

    // .CEOONFLONDJ LMGGLCNCDHF = 12;

    pub fn LMGGLCNCDHF(&self) -> &super::CEOONFLONDJ::CEOONFLONDJ {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(ref v)) => v,
            _ => <super::CEOONFLONDJ::CEOONFLONDJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LMGGLCNCDHF(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_LMGGLCNCDHF(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LMGGLCNCDHF(&mut self, v: super::CEOONFLONDJ::CEOONFLONDJ) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LMGGLCNCDHF(&mut self) -> &mut super::CEOONFLONDJ::CEOONFLONDJ {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(super::CEOONFLONDJ::CEOONFLONDJ::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LMGGLCNCDHF(&mut self) -> super::CEOONFLONDJ::CEOONFLONDJ {
        if self.has_LMGGLCNCDHF() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CEOONFLONDJ::CEOONFLONDJ::new()
        }
    }

    // .HELNOIHMDHA FNGFOAEPFJN = 2;

    pub fn FNGFOAEPFJN(&self) -> &super::HELNOIHMDHA::HELNOIHMDHA {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(ref v)) => v,
            _ => <super::HELNOIHMDHA::HELNOIHMDHA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FNGFOAEPFJN(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_FNGFOAEPFJN(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FNGFOAEPFJN(&mut self, v: super::HELNOIHMDHA::HELNOIHMDHA) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FNGFOAEPFJN(&mut self) -> &mut super::HELNOIHMDHA::HELNOIHMDHA {
        if let ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(_)) = self.DEJMPMFHIOC {
        } else {
            self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(super::HELNOIHMDHA::HELNOIHMDHA::new()));
        }
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FNGFOAEPFJN(&mut self) -> super::HELNOIHMDHA::HELNOIHMDHA {
        if self.has_FNGFOAEPFJN() {
            match self.DEJMPMFHIOC.take() {
                ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HELNOIHMDHA::HELNOIHMDHA::new()
        }
    }

    // uint32 OBPFBLNBFKI = 14;

    pub fn OBPFBLNBFKI(&self) -> u32 {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_OBPFBLNBFKI(&mut self) {
        self.DEJMPMFHIOC = ::std::option::Option::None;
    }

    pub fn has_OBPFBLNBFKI(&self) -> bool {
        match self.DEJMPMFHIOC {
            ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OBPFBLNBFKI(&mut self, v: u32) {
        self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLKEOEHPLNG",
            |m: &ClockParkHandleWaitOperationCsReq| { &m.CLKEOEHPLNG },
            |m: &mut ClockParkHandleWaitOperationCsReq| { &mut m.CLKEOEHPLNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AHIDJBJGGPP",
            |m: &ClockParkHandleWaitOperationCsReq| { &m.AHIDJBJGGPP },
            |m: &mut ClockParkHandleWaitOperationCsReq| { &mut m.AHIDJBJGGPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::OBNONMHMECK::OBNONMHMECK>(
            "JFBCKCLPAKO",
            ClockParkHandleWaitOperationCsReq::has_JFBCKCLPAKO,
            ClockParkHandleWaitOperationCsReq::JFBCKCLPAKO,
            ClockParkHandleWaitOperationCsReq::mut_JFBCKCLPAKO,
            ClockParkHandleWaitOperationCsReq::set_JFBCKCLPAKO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FFOMIBNCFKI::FFOMIBNCFKI>(
            "IPIKFLCEFLA",
            ClockParkHandleWaitOperationCsReq::has_IPIKFLCEFLA,
            ClockParkHandleWaitOperationCsReq::IPIKFLCEFLA,
            ClockParkHandleWaitOperationCsReq::mut_IPIKFLCEFLA,
            ClockParkHandleWaitOperationCsReq::set_IPIKFLCEFLA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AJEHAMDABNA::AJEHAMDABNA>(
            "MEGNBBFILNL",
            ClockParkHandleWaitOperationCsReq::has_MEGNBBFILNL,
            ClockParkHandleWaitOperationCsReq::MEGNBBFILNL,
            ClockParkHandleWaitOperationCsReq::mut_MEGNBBFILNL,
            ClockParkHandleWaitOperationCsReq::set_MEGNBBFILNL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::AMGHDCABJMJ::AMGHDCABJMJ>(
            "ABKKDHAPCHN",
            ClockParkHandleWaitOperationCsReq::has_ABKKDHAPCHN,
            ClockParkHandleWaitOperationCsReq::ABKKDHAPCHN,
            ClockParkHandleWaitOperationCsReq::mut_ABKKDHAPCHN,
            ClockParkHandleWaitOperationCsReq::set_ABKKDHAPCHN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::INNNICFOLII::INNNICFOLII>(
            "LIHJMEINGIK",
            ClockParkHandleWaitOperationCsReq::has_LIHJMEINGIK,
            ClockParkHandleWaitOperationCsReq::LIHJMEINGIK,
            ClockParkHandleWaitOperationCsReq::mut_LIHJMEINGIK,
            ClockParkHandleWaitOperationCsReq::set_LIHJMEINGIK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CEOONFLONDJ::CEOONFLONDJ>(
            "LMGGLCNCDHF",
            ClockParkHandleWaitOperationCsReq::has_LMGGLCNCDHF,
            ClockParkHandleWaitOperationCsReq::LMGGLCNCDHF,
            ClockParkHandleWaitOperationCsReq::mut_LMGGLCNCDHF,
            ClockParkHandleWaitOperationCsReq::set_LMGGLCNCDHF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HELNOIHMDHA::HELNOIHMDHA>(
            "FNGFOAEPFJN",
            ClockParkHandleWaitOperationCsReq::has_FNGFOAEPFJN,
            ClockParkHandleWaitOperationCsReq::FNGFOAEPFJN,
            ClockParkHandleWaitOperationCsReq::mut_FNGFOAEPFJN,
            ClockParkHandleWaitOperationCsReq::set_FNGFOAEPFJN,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "OBPFBLNBFKI",
            ClockParkHandleWaitOperationCsReq::has_OBPFBLNBFKI,
            ClockParkHandleWaitOperationCsReq::OBPFBLNBFKI,
            ClockParkHandleWaitOperationCsReq::set_OBPFBLNBFKI,
        ));
        oneofs.push(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClockParkHandleWaitOperationCsReq>(
            "ClockParkHandleWaitOperationCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClockParkHandleWaitOperationCsReq {
    const NAME: &'static str = "ClockParkHandleWaitOperationCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.CLKEOEHPLNG = is.read_uint32()?;
                },
                88 => {
                    self.AHIDJBJGGPP = is.read_uint32()?;
                },
                42 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(is.read_message()?));
                },
                74 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(is.read_message()?));
                },
                58 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(is.read_message()?));
                },
                10 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(is.read_message()?));
                },
                50 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(is.read_message()?));
                },
                98 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(is.read_message()?));
                },
                18 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(is.read_message()?));
                },
                112 => {
                    self.DEJMPMFHIOC = ::std::option::Option::Some(clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(is.read_uint32()?));
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
        if self.CLKEOEHPLNG != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.CLKEOEHPLNG);
        }
        if self.AHIDJBJGGPP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.AHIDJBJGGPP);
        }
        if let ::std::option::Option::Some(ref v) = self.DEJMPMFHIOC {
            match v {
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(v) => {
                    my_size += ::protobuf::rt::uint32_size(14, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CLKEOEHPLNG != 0 {
            os.write_uint32(4, self.CLKEOEHPLNG)?;
        }
        if self.AHIDJBJGGPP != 0 {
            os.write_uint32(11, self.AHIDJBJGGPP)?;
        }
        if let ::std::option::Option::Some(ref v) = self.DEJMPMFHIOC {
            match v {
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::JFBCKCLPAKO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::IPIKFLCEFLA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::MEGNBBFILNL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::ABKKDHAPCHN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LIHJMEINGIK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::LMGGLCNCDHF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::FNGFOAEPFJN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &clock_park_handle_wait_operation_cs_req::DEJMPMFHIOC::OBPFBLNBFKI(v) => {
                    os.write_uint32(14, v)?;
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

    fn new() -> ClockParkHandleWaitOperationCsReq {
        ClockParkHandleWaitOperationCsReq::new()
    }

    fn clear(&mut self) {
        self.CLKEOEHPLNG = 0;
        self.AHIDJBJGGPP = 0;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.DEJMPMFHIOC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClockParkHandleWaitOperationCsReq {
        static instance: ClockParkHandleWaitOperationCsReq = ClockParkHandleWaitOperationCsReq {
            CLKEOEHPLNG: 0,
            AHIDJBJGGPP: 0,
            DEJMPMFHIOC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClockParkHandleWaitOperationCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClockParkHandleWaitOperationCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClockParkHandleWaitOperationCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClockParkHandleWaitOperationCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ClockParkHandleWaitOperationCsReq`
pub mod clock_park_handle_wait_operation_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ClockParkHandleWaitOperationCsReq.DEJMPMFHIOC)
    pub enum DEJMPMFHIOC {
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.JFBCKCLPAKO)
        JFBCKCLPAKO(super::super::OBNONMHMECK::OBNONMHMECK),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.IPIKFLCEFLA)
        IPIKFLCEFLA(super::super::FFOMIBNCFKI::FFOMIBNCFKI),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.MEGNBBFILNL)
        MEGNBBFILNL(super::super::AJEHAMDABNA::AJEHAMDABNA),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.ABKKDHAPCHN)
        ABKKDHAPCHN(super::super::AMGHDCABJMJ::AMGHDCABJMJ),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.LIHJMEINGIK)
        LIHJMEINGIK(super::super::INNNICFOLII::INNNICFOLII),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.LMGGLCNCDHF)
        LMGGLCNCDHF(super::super::CEOONFLONDJ::CEOONFLONDJ),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.FNGFOAEPFJN)
        FNGFOAEPFJN(super::super::HELNOIHMDHA::HELNOIHMDHA),
        // @@protoc_insertion_point(oneof_field:ClockParkHandleWaitOperationCsReq.OBPFBLNBFKI)
        OBPFBLNBFKI(u32),
    }

    impl ::protobuf::Oneof for DEJMPMFHIOC {
    }

    impl ::protobuf::OneofFull for DEJMPMFHIOC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ClockParkHandleWaitOperationCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("DEJMPMFHIOC").unwrap()).clone()
        }
    }

    impl DEJMPMFHIOC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DEJMPMFHIOC>("DEJMPMFHIOC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'ClockParkHandleWaitOperationCsReq.proto\x1a\x11AJEHAMDABNA.proto\x1a\
    \x11AMGHDCABJMJ.proto\x1a\x11CEOONFLONDJ.proto\x1a\x11FFOMIBNCFKI.proto\
    \x1a\x11HELNOIHMDHA.proto\x1a\x11INNNICFOLII.proto\x1a\x11OBNONMHMECK.pr\
    oto\"\xf8\x03\n!ClockParkHandleWaitOperationCsReq\x12\x20\n\x0bCLKEOEHPL\
    NG\x18\x04\x20\x01(\rR\x0bCLKEOEHPLNG\x12\x20\n\x0bAHIDJBJGGPP\x18\x0b\
    \x20\x01(\rR\x0bAHIDJBJGGPP\x120\n\x0bJFBCKCLPAKO\x18\x05\x20\x01(\x0b2\
    \x0c.OBNONMHMECKH\0R\x0bJFBCKCLPAKO\x120\n\x0bIPIKFLCEFLA\x18\t\x20\x01(\
    \x0b2\x0c.FFOMIBNCFKIH\0R\x0bIPIKFLCEFLA\x120\n\x0bMEGNBBFILNL\x18\x07\
    \x20\x01(\x0b2\x0c.AJEHAMDABNAH\0R\x0bMEGNBBFILNL\x120\n\x0bABKKDHAPCHN\
    \x18\x01\x20\x01(\x0b2\x0c.AMGHDCABJMJH\0R\x0bABKKDHAPCHN\x120\n\x0bLIHJ\
    MEINGIK\x18\x06\x20\x01(\x0b2\x0c.INNNICFOLIIH\0R\x0bLIHJMEINGIK\x120\n\
    \x0bLMGGLCNCDHF\x18\x0c\x20\x01(\x0b2\x0c.CEOONFLONDJH\0R\x0bLMGGLCNCDHF\
    \x120\n\x0bFNGFOAEPFJN\x18\x02\x20\x01(\x0b2\x0c.HELNOIHMDHAH\0R\x0bFNGF\
    OAEPFJN\x12\"\n\x0bOBPFBLNBFKI\x18\x0e\x20\x01(\rH\0R\x0bOBPFBLNBFKIB\r\
    \n\x0bDEJMPMFHIOCb\x06proto3\
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
            deps.push(super::AJEHAMDABNA::file_descriptor().clone());
            deps.push(super::AMGHDCABJMJ::file_descriptor().clone());
            deps.push(super::CEOONFLONDJ::file_descriptor().clone());
            deps.push(super::FFOMIBNCFKI::file_descriptor().clone());
            deps.push(super::HELNOIHMDHA::file_descriptor().clone());
            deps.push(super::INNNICFOLII::file_descriptor().clone());
            deps.push(super::OBNONMHMECK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClockParkHandleWaitOperationCsReq::generated_message_descriptor_data());
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
