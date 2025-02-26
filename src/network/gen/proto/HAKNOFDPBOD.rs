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

//! Generated file from `HAKNOFDPBOD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HAKNOFDPBOD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HAKNOFDPBOD {
    // message fields
    // @@protoc_insertion_point(field:HAKNOFDPBOD.ACDOPCBMPNL)
    pub ACDOPCBMPNL: u32,
    // @@protoc_insertion_point(field:HAKNOFDPBOD.FPOGIALMCIP)
    pub FPOGIALMCIP: u32,
    // @@protoc_insertion_point(field:HAKNOFDPBOD.MNCIHJHGNMJ)
    pub MNCIHJHGNMJ: u32,
    // message oneof groups
    pub NGIKDJMNGBG: ::std::option::Option<haknofdpbod::NGIKDJMNGBG>,
    // special fields
    // @@protoc_insertion_point(special_field:HAKNOFDPBOD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HAKNOFDPBOD {
    fn default() -> &'a HAKNOFDPBOD {
        <HAKNOFDPBOD as ::protobuf::Message>::default_instance()
    }
}

impl HAKNOFDPBOD {
    pub fn new() -> HAKNOFDPBOD {
        ::std::default::Default::default()
    }

    // .GJKIAPIPGAN EJCOLGNJGDC = 15;

    pub fn EJCOLGNJGDC(&self) -> &super::GJKIAPIPGAN::GJKIAPIPGAN {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(ref v)) => v,
            _ => <super::GJKIAPIPGAN::GJKIAPIPGAN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_EJCOLGNJGDC(&mut self) {
        self.NGIKDJMNGBG = ::std::option::Option::None;
    }

    pub fn has_EJCOLGNJGDC(&self) -> bool {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_EJCOLGNJGDC(&mut self, v: super::GJKIAPIPGAN::GJKIAPIPGAN) {
        self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_EJCOLGNJGDC(&mut self) -> &mut super::GJKIAPIPGAN::GJKIAPIPGAN {
        if let ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(_)) = self.NGIKDJMNGBG {
        } else {
            self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(super::GJKIAPIPGAN::GJKIAPIPGAN::new()));
        }
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_EJCOLGNJGDC(&mut self) -> super::GJKIAPIPGAN::GJKIAPIPGAN {
        if self.has_EJCOLGNJGDC() {
            match self.NGIKDJMNGBG.take() {
                ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GJKIAPIPGAN::GJKIAPIPGAN::new()
        }
    }

    // .EIMOBGLLEFO LKANIPLNKGC = 9;

    pub fn LKANIPLNKGC(&self) -> &super::EIMOBGLLEFO::EIMOBGLLEFO {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(ref v)) => v,
            _ => <super::EIMOBGLLEFO::EIMOBGLLEFO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LKANIPLNKGC(&mut self) {
        self.NGIKDJMNGBG = ::std::option::Option::None;
    }

    pub fn has_LKANIPLNKGC(&self) -> bool {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LKANIPLNKGC(&mut self, v: super::EIMOBGLLEFO::EIMOBGLLEFO) {
        self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LKANIPLNKGC(&mut self) -> &mut super::EIMOBGLLEFO::EIMOBGLLEFO {
        if let ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(_)) = self.NGIKDJMNGBG {
        } else {
            self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(super::EIMOBGLLEFO::EIMOBGLLEFO::new()));
        }
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LKANIPLNKGC(&mut self) -> super::EIMOBGLLEFO::EIMOBGLLEFO {
        if self.has_LKANIPLNKGC() {
            match self.NGIKDJMNGBG.take() {
                ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EIMOBGLLEFO::EIMOBGLLEFO::new()
        }
    }

    // .EOPFMPAOOJE BDEMPAKHGMJ = 10;

    pub fn BDEMPAKHGMJ(&self) -> &super::EOPFMPAOOJE::EOPFMPAOOJE {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(ref v)) => v,
            _ => <super::EOPFMPAOOJE::EOPFMPAOOJE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BDEMPAKHGMJ(&mut self) {
        self.NGIKDJMNGBG = ::std::option::Option::None;
    }

    pub fn has_BDEMPAKHGMJ(&self) -> bool {
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BDEMPAKHGMJ(&mut self, v: super::EOPFMPAOOJE::EOPFMPAOOJE) {
        self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BDEMPAKHGMJ(&mut self) -> &mut super::EOPFMPAOOJE::EOPFMPAOOJE {
        if let ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(_)) = self.NGIKDJMNGBG {
        } else {
            self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(super::EOPFMPAOOJE::EOPFMPAOOJE::new()));
        }
        match self.NGIKDJMNGBG {
            ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BDEMPAKHGMJ(&mut self) -> super::EOPFMPAOOJE::EOPFMPAOOJE {
        if self.has_BDEMPAKHGMJ() {
            match self.NGIKDJMNGBG.take() {
                ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EOPFMPAOOJE::EOPFMPAOOJE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACDOPCBMPNL",
            |m: &HAKNOFDPBOD| { &m.ACDOPCBMPNL },
            |m: &mut HAKNOFDPBOD| { &mut m.ACDOPCBMPNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPOGIALMCIP",
            |m: &HAKNOFDPBOD| { &m.FPOGIALMCIP },
            |m: &mut HAKNOFDPBOD| { &mut m.FPOGIALMCIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNCIHJHGNMJ",
            |m: &HAKNOFDPBOD| { &m.MNCIHJHGNMJ },
            |m: &mut HAKNOFDPBOD| { &mut m.MNCIHJHGNMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GJKIAPIPGAN::GJKIAPIPGAN>(
            "EJCOLGNJGDC",
            HAKNOFDPBOD::has_EJCOLGNJGDC,
            HAKNOFDPBOD::EJCOLGNJGDC,
            HAKNOFDPBOD::mut_EJCOLGNJGDC,
            HAKNOFDPBOD::set_EJCOLGNJGDC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EIMOBGLLEFO::EIMOBGLLEFO>(
            "LKANIPLNKGC",
            HAKNOFDPBOD::has_LKANIPLNKGC,
            HAKNOFDPBOD::LKANIPLNKGC,
            HAKNOFDPBOD::mut_LKANIPLNKGC,
            HAKNOFDPBOD::set_LKANIPLNKGC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EOPFMPAOOJE::EOPFMPAOOJE>(
            "BDEMPAKHGMJ",
            HAKNOFDPBOD::has_BDEMPAKHGMJ,
            HAKNOFDPBOD::BDEMPAKHGMJ,
            HAKNOFDPBOD::mut_BDEMPAKHGMJ,
            HAKNOFDPBOD::set_BDEMPAKHGMJ,
        ));
        oneofs.push(haknofdpbod::NGIKDJMNGBG::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HAKNOFDPBOD>(
            "HAKNOFDPBOD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HAKNOFDPBOD {
    const NAME: &'static str = "HAKNOFDPBOD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.ACDOPCBMPNL = is.read_uint32()?;
                },
                96 => {
                    self.FPOGIALMCIP = is.read_uint32()?;
                },
                8 => {
                    self.MNCIHJHGNMJ = is.read_uint32()?;
                },
                122 => {
                    self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(is.read_message()?));
                },
                74 => {
                    self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(is.read_message()?));
                },
                82 => {
                    self.NGIKDJMNGBG = ::std::option::Option::Some(haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(is.read_message()?));
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
        if self.ACDOPCBMPNL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ACDOPCBMPNL);
        }
        if self.FPOGIALMCIP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FPOGIALMCIP);
        }
        if self.MNCIHJHGNMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MNCIHJHGNMJ);
        }
        if let ::std::option::Option::Some(ref v) = self.NGIKDJMNGBG {
            match v {
                &haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(ref v) => {
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
        if self.ACDOPCBMPNL != 0 {
            os.write_uint32(6, self.ACDOPCBMPNL)?;
        }
        if self.FPOGIALMCIP != 0 {
            os.write_uint32(12, self.FPOGIALMCIP)?;
        }
        if self.MNCIHJHGNMJ != 0 {
            os.write_uint32(1, self.MNCIHJHGNMJ)?;
        }
        if let ::std::option::Option::Some(ref v) = self.NGIKDJMNGBG {
            match v {
                &haknofdpbod::NGIKDJMNGBG::EJCOLGNJGDC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &haknofdpbod::NGIKDJMNGBG::LKANIPLNKGC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &haknofdpbod::NGIKDJMNGBG::BDEMPAKHGMJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
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

    fn new() -> HAKNOFDPBOD {
        HAKNOFDPBOD::new()
    }

    fn clear(&mut self) {
        self.ACDOPCBMPNL = 0;
        self.FPOGIALMCIP = 0;
        self.MNCIHJHGNMJ = 0;
        self.NGIKDJMNGBG = ::std::option::Option::None;
        self.NGIKDJMNGBG = ::std::option::Option::None;
        self.NGIKDJMNGBG = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HAKNOFDPBOD {
        static instance: HAKNOFDPBOD = HAKNOFDPBOD {
            ACDOPCBMPNL: 0,
            FPOGIALMCIP: 0,
            MNCIHJHGNMJ: 0,
            NGIKDJMNGBG: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HAKNOFDPBOD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HAKNOFDPBOD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HAKNOFDPBOD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HAKNOFDPBOD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HAKNOFDPBOD`
pub mod haknofdpbod {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:HAKNOFDPBOD.NGIKDJMNGBG)
    pub enum NGIKDJMNGBG {
        // @@protoc_insertion_point(oneof_field:HAKNOFDPBOD.EJCOLGNJGDC)
        EJCOLGNJGDC(super::super::GJKIAPIPGAN::GJKIAPIPGAN),
        // @@protoc_insertion_point(oneof_field:HAKNOFDPBOD.LKANIPLNKGC)
        LKANIPLNKGC(super::super::EIMOBGLLEFO::EIMOBGLLEFO),
        // @@protoc_insertion_point(oneof_field:HAKNOFDPBOD.BDEMPAKHGMJ)
        BDEMPAKHGMJ(super::super::EOPFMPAOOJE::EOPFMPAOOJE),
    }

    impl ::protobuf::Oneof for NGIKDJMNGBG {
    }

    impl ::protobuf::OneofFull for NGIKDJMNGBG {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HAKNOFDPBOD as ::protobuf::MessageFull>::descriptor().oneof_by_name("NGIKDJMNGBG").unwrap()).clone()
        }
    }

    impl NGIKDJMNGBG {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NGIKDJMNGBG>("NGIKDJMNGBG")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HAKNOFDPBOD.proto\x1a\x11EIMOBGLLEFO.proto\x1a\x11EOPFMPAOOJE.prot\
    o\x1a\x11GJKIAPIPGAN.proto\"\x98\x02\n\x0bHAKNOFDPBOD\x12\x20\n\x0bACDOP\
    CBMPNL\x18\x06\x20\x01(\rR\x0bACDOPCBMPNL\x12\x20\n\x0bFPOGIALMCIP\x18\
    \x0c\x20\x01(\rR\x0bFPOGIALMCIP\x12\x20\n\x0bMNCIHJHGNMJ\x18\x01\x20\x01\
    (\rR\x0bMNCIHJHGNMJ\x120\n\x0bEJCOLGNJGDC\x18\x0f\x20\x01(\x0b2\x0c.GJKI\
    APIPGANH\0R\x0bEJCOLGNJGDC\x120\n\x0bLKANIPLNKGC\x18\t\x20\x01(\x0b2\x0c\
    .EIMOBGLLEFOH\0R\x0bLKANIPLNKGC\x120\n\x0bBDEMPAKHGMJ\x18\n\x20\x01(\x0b\
    2\x0c.EOPFMPAOOJEH\0R\x0bBDEMPAKHGMJB\r\n\x0bNGIKDJMNGBGb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EIMOBGLLEFO::file_descriptor().clone());
            deps.push(super::EOPFMPAOOJE::file_descriptor().clone());
            deps.push(super::GJKIAPIPGAN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HAKNOFDPBOD::generated_message_descriptor_data());
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
