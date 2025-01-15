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

//! Generated file from `AGAOOLGMDOO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AGAOOLGMDOO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AGAOOLGMDOO {
    // message fields
    // @@protoc_insertion_point(field:AGAOOLGMDOO.level)
    pub level: ::protobuf::MessageField<super::BDMOKJBDCDD::BDMOKJBDCDD>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.EAOFCFHMKAG)
    pub EAOFCFHMKAG: ::protobuf::MessageField<super::KPOOJGLKGFK::KPOOJGLKGFK>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.FFFDOIGNDFN)
    pub FFFDOIGNDFN: ::protobuf::MessageField<super::MGEGFDMBKCB::MGEGFDMBKCB>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.BPGELDJGKGJ)
    pub BPGELDJGKGJ: ::protobuf::MessageField<super::GFMELEMCJNA::GFMELEMCJNA>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.LPIAOHMEJHI)
    pub LPIAOHMEJHI: ::protobuf::MessageField<super::HAHBDLFOBHB::HAHBDLFOBHB>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.MPFMPMLGOGE)
    pub MPFMPMLGOGE: ::protobuf::MessageField<super::MJJAIIIDPPD::MJJAIIIDPPD>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.IIBAHAEAEEB)
    pub IIBAHAEAEEB: ::protobuf::MessageField<super::BFEJLAJIPNG::BFEJLAJIPNG>,
    // @@protoc_insertion_point(field:AGAOOLGMDOO.MOBJJPMIAAB)
    pub MOBJJPMIAAB: ::protobuf::MessageField<super::BOEKJLOLAJE::BOEKJLOLAJE>,
    // special fields
    // @@protoc_insertion_point(special_field:AGAOOLGMDOO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AGAOOLGMDOO {
    fn default() -> &'a AGAOOLGMDOO {
        <AGAOOLGMDOO as ::protobuf::Message>::default_instance()
    }
}

impl AGAOOLGMDOO {
    pub fn new() -> AGAOOLGMDOO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BDMOKJBDCDD::BDMOKJBDCDD>(
            "level",
            |m: &AGAOOLGMDOO| { &m.level },
            |m: &mut AGAOOLGMDOO| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KPOOJGLKGFK::KPOOJGLKGFK>(
            "EAOFCFHMKAG",
            |m: &AGAOOLGMDOO| { &m.EAOFCFHMKAG },
            |m: &mut AGAOOLGMDOO| { &mut m.EAOFCFHMKAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MGEGFDMBKCB::MGEGFDMBKCB>(
            "FFFDOIGNDFN",
            |m: &AGAOOLGMDOO| { &m.FFFDOIGNDFN },
            |m: &mut AGAOOLGMDOO| { &mut m.FFFDOIGNDFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GFMELEMCJNA::GFMELEMCJNA>(
            "BPGELDJGKGJ",
            |m: &AGAOOLGMDOO| { &m.BPGELDJGKGJ },
            |m: &mut AGAOOLGMDOO| { &mut m.BPGELDJGKGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HAHBDLFOBHB::HAHBDLFOBHB>(
            "LPIAOHMEJHI",
            |m: &AGAOOLGMDOO| { &m.LPIAOHMEJHI },
            |m: &mut AGAOOLGMDOO| { &mut m.LPIAOHMEJHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MJJAIIIDPPD::MJJAIIIDPPD>(
            "MPFMPMLGOGE",
            |m: &AGAOOLGMDOO| { &m.MPFMPMLGOGE },
            |m: &mut AGAOOLGMDOO| { &mut m.MPFMPMLGOGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BFEJLAJIPNG::BFEJLAJIPNG>(
            "IIBAHAEAEEB",
            |m: &AGAOOLGMDOO| { &m.IIBAHAEAEEB },
            |m: &mut AGAOOLGMDOO| { &mut m.IIBAHAEAEEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BOEKJLOLAJE::BOEKJLOLAJE>(
            "MOBJJPMIAAB",
            |m: &AGAOOLGMDOO| { &m.MOBJJPMIAAB },
            |m: &mut AGAOOLGMDOO| { &mut m.MOBJJPMIAAB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AGAOOLGMDOO>(
            "AGAOOLGMDOO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AGAOOLGMDOO {
    const NAME: &'static str = "AGAOOLGMDOO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.level)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EAOFCFHMKAG)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FFFDOIGNDFN)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BPGELDJGKGJ)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LPIAOHMEJHI)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MPFMPMLGOGE)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IIBAHAEAEEB)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MOBJJPMIAAB)?;
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
        if let Some(v) = self.level.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EAOFCFHMKAG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FFFDOIGNDFN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BPGELDJGKGJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LPIAOHMEJHI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MPFMPMLGOGE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MOBJJPMIAAB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.level.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.EAOFCFHMKAG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.FFFDOIGNDFN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.BPGELDJGKGJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.LPIAOHMEJHI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.MPFMPMLGOGE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.MOBJJPMIAAB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> AGAOOLGMDOO {
        AGAOOLGMDOO::new()
    }

    fn clear(&mut self) {
        self.level.clear();
        self.EAOFCFHMKAG.clear();
        self.FFFDOIGNDFN.clear();
        self.BPGELDJGKGJ.clear();
        self.LPIAOHMEJHI.clear();
        self.MPFMPMLGOGE.clear();
        self.IIBAHAEAEEB.clear();
        self.MOBJJPMIAAB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AGAOOLGMDOO {
        static instance: AGAOOLGMDOO = AGAOOLGMDOO {
            level: ::protobuf::MessageField::none(),
            EAOFCFHMKAG: ::protobuf::MessageField::none(),
            FFFDOIGNDFN: ::protobuf::MessageField::none(),
            BPGELDJGKGJ: ::protobuf::MessageField::none(),
            LPIAOHMEJHI: ::protobuf::MessageField::none(),
            MPFMPMLGOGE: ::protobuf::MessageField::none(),
            IIBAHAEAEEB: ::protobuf::MessageField::none(),
            MOBJJPMIAAB: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AGAOOLGMDOO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AGAOOLGMDOO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AGAOOLGMDOO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AGAOOLGMDOO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AGAOOLGMDOO.proto\x1a\x11BDMOKJBDCDD.proto\x1a\x11BFEJLAJIPNG.prot\
    o\x1a\x11BOEKJLOLAJE.proto\x1a\x11GFMELEMCJNA.proto\x1a\x11HAHBDLFOBHB.p\
    roto\x1a\x11KPOOJGLKGFK.proto\x1a\x11MGEGFDMBKCB.proto\x1a\x11MJJAIIIDPP\
    D.proto\"\x81\x03\n\x0bAGAOOLGMDOO\x12\"\n\x05level\x18\x03\x20\x01(\x0b\
    2\x0c.BDMOKJBDCDDR\x05level\x12.\n\x0bEAOFCFHMKAG\x18\x07\x20\x01(\x0b2\
    \x0c.KPOOJGLKGFKR\x0bEAOFCFHMKAG\x12.\n\x0bFFFDOIGNDFN\x18\x08\x20\x01(\
    \x0b2\x0c.MGEGFDMBKCBR\x0bFFFDOIGNDFN\x12.\n\x0bBPGELDJGKGJ\x18\x0f\x20\
    \x01(\x0b2\x0c.GFMELEMCJNAR\x0bBPGELDJGKGJ\x12.\n\x0bLPIAOHMEJHI\x18\x02\
    \x20\x01(\x0b2\x0c.HAHBDLFOBHBR\x0bLPIAOHMEJHI\x12.\n\x0bMPFMPMLGOGE\x18\
    \x0b\x20\x01(\x0b2\x0c.MJJAIIIDPPDR\x0bMPFMPMLGOGE\x12.\n\x0bIIBAHAEAEEB\
    \x18\x0e\x20\x01(\x0b2\x0c.BFEJLAJIPNGR\x0bIIBAHAEAEEB\x12.\n\x0bMOBJJPM\
    IAAB\x18\x0c\x20\x01(\x0b2\x0c.BOEKJLOLAJER\x0bMOBJJPMIAABb\x06proto3\
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
            deps.push(super::BDMOKJBDCDD::file_descriptor().clone());
            deps.push(super::BFEJLAJIPNG::file_descriptor().clone());
            deps.push(super::BOEKJLOLAJE::file_descriptor().clone());
            deps.push(super::GFMELEMCJNA::file_descriptor().clone());
            deps.push(super::HAHBDLFOBHB::file_descriptor().clone());
            deps.push(super::KPOOJGLKGFK::file_descriptor().clone());
            deps.push(super::MGEGFDMBKCB::file_descriptor().clone());
            deps.push(super::MJJAIIIDPPD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AGAOOLGMDOO::generated_message_descriptor_data());
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