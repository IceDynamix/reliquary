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

//! Generated file from `CNMLONGLBHB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CNMLONGLBHB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CNMLONGLBHB {
    // message oneof groups
    pub ICMIHDBCJEI: ::std::option::Option<cnmlonglbhb::ICMIHDBCJEI>,
    // special fields
    // @@protoc_insertion_point(special_field:CNMLONGLBHB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CNMLONGLBHB {
    fn default() -> &'a CNMLONGLBHB {
        <CNMLONGLBHB as ::protobuf::Message>::default_instance()
    }
}

impl CNMLONGLBHB {
    pub fn new() -> CNMLONGLBHB {
        ::std::default::Default::default()
    }

    // bool ODBLDNFKIAF = 3;

    pub fn ODBLDNFKIAF(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(v)) => v,
            _ => false,
        }
    }

    pub fn clear_ODBLDNFKIAF(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_ODBLDNFKIAF(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ODBLDNFKIAF(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(v))
    }

    // bool JLIPPMHELFF = 15;

    pub fn JLIPPMHELFF(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(v)) => v,
            _ => false,
        }
    }

    pub fn clear_JLIPPMHELFF(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_JLIPPMHELFF(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JLIPPMHELFF(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(v))
    }

    // bool MOMAEKCODKG = 6;

    pub fn MOMAEKCODKG(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(v)) => v,
            _ => false,
        }
    }

    pub fn clear_MOMAEKCODKG(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_MOMAEKCODKG(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MOMAEKCODKG(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(v))
    }

    // bool AAFJGNNAPFC = 10;

    pub fn AAFJGNNAPFC(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(v)) => v,
            _ => false,
        }
    }

    pub fn clear_AAFJGNNAPFC(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_AAFJGNNAPFC(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AAFJGNNAPFC(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(v))
    }

    // bool GIGFGKBKNJD = 12;

    pub fn GIGFGKBKNJD(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(v)) => v,
            _ => false,
        }
    }

    pub fn clear_GIGFGKBKNJD(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_GIGFGKBKNJD(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GIGFGKBKNJD(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(v))
    }

    // bool BJADALJFKAB = 14;

    pub fn BJADALJFKAB(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(v)) => v,
            _ => false,
        }
    }

    pub fn clear_BJADALJFKAB(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_BJADALJFKAB(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BJADALJFKAB(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(v))
    }

    // bool LEALOPKFPPE = 2;

    pub fn LEALOPKFPPE(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(v)) => v,
            _ => false,
        }
    }

    pub fn clear_LEALOPKFPPE(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_LEALOPKFPPE(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LEALOPKFPPE(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(v))
    }

    // bool DEMOBHBJNDI = 1;

    pub fn DEMOBHBJNDI(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(v)) => v,
            _ => false,
        }
    }

    pub fn clear_DEMOBHBJNDI(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_DEMOBHBJNDI(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DEMOBHBJNDI(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(v))
    }

    // bool AFDCJBKGNPJ = 8;

    pub fn AFDCJBKGNPJ(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(v)) => v,
            _ => false,
        }
    }

    pub fn clear_AFDCJBKGNPJ(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
    }

    pub fn has_AFDCJBKGNPJ(&self) -> bool {
        match self.ICMIHDBCJEI {
            ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_AFDCJBKGNPJ(&mut self, v: bool) {
        self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "ODBLDNFKIAF",
            CNMLONGLBHB::has_ODBLDNFKIAF,
            CNMLONGLBHB::ODBLDNFKIAF,
            CNMLONGLBHB::set_ODBLDNFKIAF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "JLIPPMHELFF",
            CNMLONGLBHB::has_JLIPPMHELFF,
            CNMLONGLBHB::JLIPPMHELFF,
            CNMLONGLBHB::set_JLIPPMHELFF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "MOMAEKCODKG",
            CNMLONGLBHB::has_MOMAEKCODKG,
            CNMLONGLBHB::MOMAEKCODKG,
            CNMLONGLBHB::set_MOMAEKCODKG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "AAFJGNNAPFC",
            CNMLONGLBHB::has_AAFJGNNAPFC,
            CNMLONGLBHB::AAFJGNNAPFC,
            CNMLONGLBHB::set_AAFJGNNAPFC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "GIGFGKBKNJD",
            CNMLONGLBHB::has_GIGFGKBKNJD,
            CNMLONGLBHB::GIGFGKBKNJD,
            CNMLONGLBHB::set_GIGFGKBKNJD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "BJADALJFKAB",
            CNMLONGLBHB::has_BJADALJFKAB,
            CNMLONGLBHB::BJADALJFKAB,
            CNMLONGLBHB::set_BJADALJFKAB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "LEALOPKFPPE",
            CNMLONGLBHB::has_LEALOPKFPPE,
            CNMLONGLBHB::LEALOPKFPPE,
            CNMLONGLBHB::set_LEALOPKFPPE,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "DEMOBHBJNDI",
            CNMLONGLBHB::has_DEMOBHBJNDI,
            CNMLONGLBHB::DEMOBHBJNDI,
            CNMLONGLBHB::set_DEMOBHBJNDI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "AFDCJBKGNPJ",
            CNMLONGLBHB::has_AFDCJBKGNPJ,
            CNMLONGLBHB::AFDCJBKGNPJ,
            CNMLONGLBHB::set_AFDCJBKGNPJ,
        ));
        oneofs.push(cnmlonglbhb::ICMIHDBCJEI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CNMLONGLBHB>(
            "CNMLONGLBHB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CNMLONGLBHB {
    const NAME: &'static str = "CNMLONGLBHB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(is.read_bool()?));
                },
                120 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(is.read_bool()?));
                },
                48 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(is.read_bool()?));
                },
                80 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(is.read_bool()?));
                },
                96 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(is.read_bool()?));
                },
                112 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(is.read_bool()?));
                },
                16 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(is.read_bool()?));
                },
                8 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(is.read_bool()?));
                },
                64 => {
                    self.ICMIHDBCJEI = ::std::option::Option::Some(cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(is.read_bool()?));
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
        if let ::std::option::Option::Some(ref v) = self.ICMIHDBCJEI {
            match v {
                &cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(v) => {
                    my_size += 1 + 1;
                },
                &cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(v) => {
                    my_size += 1 + 1;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.ICMIHDBCJEI {
            match v {
                &cnmlonglbhb::ICMIHDBCJEI::ODBLDNFKIAF(v) => {
                    os.write_bool(3, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::JLIPPMHELFF(v) => {
                    os.write_bool(15, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::MOMAEKCODKG(v) => {
                    os.write_bool(6, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::AAFJGNNAPFC(v) => {
                    os.write_bool(10, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::GIGFGKBKNJD(v) => {
                    os.write_bool(12, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::BJADALJFKAB(v) => {
                    os.write_bool(14, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::LEALOPKFPPE(v) => {
                    os.write_bool(2, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::DEMOBHBJNDI(v) => {
                    os.write_bool(1, v)?;
                },
                &cnmlonglbhb::ICMIHDBCJEI::AFDCJBKGNPJ(v) => {
                    os.write_bool(8, v)?;
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

    fn new() -> CNMLONGLBHB {
        CNMLONGLBHB::new()
    }

    fn clear(&mut self) {
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.ICMIHDBCJEI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CNMLONGLBHB {
        static instance: CNMLONGLBHB = CNMLONGLBHB {
            ICMIHDBCJEI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CNMLONGLBHB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CNMLONGLBHB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CNMLONGLBHB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNMLONGLBHB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CNMLONGLBHB`
pub mod cnmlonglbhb {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CNMLONGLBHB.ICMIHDBCJEI)
    pub enum ICMIHDBCJEI {
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.ODBLDNFKIAF)
        ODBLDNFKIAF(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.JLIPPMHELFF)
        JLIPPMHELFF(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.MOMAEKCODKG)
        MOMAEKCODKG(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.AAFJGNNAPFC)
        AAFJGNNAPFC(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.GIGFGKBKNJD)
        GIGFGKBKNJD(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.BJADALJFKAB)
        BJADALJFKAB(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.LEALOPKFPPE)
        LEALOPKFPPE(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.DEMOBHBJNDI)
        DEMOBHBJNDI(bool),
        // @@protoc_insertion_point(oneof_field:CNMLONGLBHB.AFDCJBKGNPJ)
        AFDCJBKGNPJ(bool),
    }

    impl ::protobuf::Oneof for ICMIHDBCJEI {
    }

    impl ::protobuf::OneofFull for ICMIHDBCJEI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CNMLONGLBHB as ::protobuf::MessageFull>::descriptor().oneof_by_name("ICMIHDBCJEI").unwrap()).clone()
        }
    }

    impl ICMIHDBCJEI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<ICMIHDBCJEI>("ICMIHDBCJEI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CNMLONGLBHB.proto\"\xe0\x02\n\x0bCNMLONGLBHB\x12\"\n\x0bODBLDNFKIA\
    F\x18\x03\x20\x01(\x08H\0R\x0bODBLDNFKIAF\x12\"\n\x0bJLIPPMHELFF\x18\x0f\
    \x20\x01(\x08H\0R\x0bJLIPPMHELFF\x12\"\n\x0bMOMAEKCODKG\x18\x06\x20\x01(\
    \x08H\0R\x0bMOMAEKCODKG\x12\"\n\x0bAAFJGNNAPFC\x18\n\x20\x01(\x08H\0R\
    \x0bAAFJGNNAPFC\x12\"\n\x0bGIGFGKBKNJD\x18\x0c\x20\x01(\x08H\0R\x0bGIGFG\
    KBKNJD\x12\"\n\x0bBJADALJFKAB\x18\x0e\x20\x01(\x08H\0R\x0bBJADALJFKAB\
    \x12\"\n\x0bLEALOPKFPPE\x18\x02\x20\x01(\x08H\0R\x0bLEALOPKFPPE\x12\"\n\
    \x0bDEMOBHBJNDI\x18\x01\x20\x01(\x08H\0R\x0bDEMOBHBJNDI\x12\"\n\x0bAFDCJ\
    BKGNPJ\x18\x08\x20\x01(\x08H\0R\x0bAFDCJBKGNPJB\r\n\x0bICMIHDBCJEIb\x06p\
    roto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CNMLONGLBHB::generated_message_descriptor_data());
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
