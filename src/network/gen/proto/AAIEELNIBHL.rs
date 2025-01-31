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

//! Generated file from `AAIEELNIBHL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AAIEELNIBHL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AAIEELNIBHL {
    // message fields
    // @@protoc_insertion_point(field:AAIEELNIBHL.level)
    pub level: ::protobuf::MessageField<super::IHKCKOAOMMA::IHKCKOAOMMA>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.OKBKBPJPDOB)
    pub OKBKBPJPDOB: ::protobuf::MessageField<super::PDHGDLPCLGK::PDHGDLPCLGK>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.EPAFPEJNMFB)
    pub EPAFPEJNMFB: ::protobuf::MessageField<super::FOBCIMDNKPI::FOBCIMDNKPI>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.IIBAHAEAEEB)
    pub IIBAHAEAEEB: ::protobuf::MessageField<super::BFEJLAJIPNG::BFEJLAJIPNG>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF)
    pub ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF: ::protobuf::MessageField<super::EAAMMPJFKIB::EAAMMPJFKIB>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.GNKGCONLODG)
    pub GNKGCONLODG: ::protobuf::MessageField<super::FANHHMLOGPI::FANHHMLOGPI>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.BPGELDJGKGJ)
    pub BPGELDJGKGJ: ::protobuf::MessageField<super::GFMELEMCJNA::GFMELEMCJNA>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.EEKLHGHIOGB)
    pub EEKLHGHIOGB: ::protobuf::MessageField<super::OMJCICFMBEH::OMJCICFMBEH>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.FFFDOIGNDFN)
    pub FFFDOIGNDFN: ::protobuf::MessageField<super::ODHFHOOEDPJ::ODHFHOOEDPJ>,
    // @@protoc_insertion_point(field:AAIEELNIBHL.MPFMPMLGOGE)
    pub MPFMPMLGOGE: ::protobuf::MessageField<super::MJJAIIIDPPD::MJJAIIIDPPD>,
    // special fields
    // @@protoc_insertion_point(special_field:AAIEELNIBHL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AAIEELNIBHL {
    fn default() -> &'a AAIEELNIBHL {
        <AAIEELNIBHL as ::protobuf::Message>::default_instance()
    }
}

impl AAIEELNIBHL {
    pub fn new() -> AAIEELNIBHL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IHKCKOAOMMA::IHKCKOAOMMA>(
            "level",
            |m: &AAIEELNIBHL| { &m.level },
            |m: &mut AAIEELNIBHL| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PDHGDLPCLGK::PDHGDLPCLGK>(
            "OKBKBPJPDOB",
            |m: &AAIEELNIBHL| { &m.OKBKBPJPDOB },
            |m: &mut AAIEELNIBHL| { &mut m.OKBKBPJPDOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FOBCIMDNKPI::FOBCIMDNKPI>(
            "EPAFPEJNMFB",
            |m: &AAIEELNIBHL| { &m.EPAFPEJNMFB },
            |m: &mut AAIEELNIBHL| { &mut m.EPAFPEJNMFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BFEJLAJIPNG::BFEJLAJIPNG>(
            "IIBAHAEAEEB",
            |m: &AAIEELNIBHL| { &m.IIBAHAEAEEB },
            |m: &mut AAIEELNIBHL| { &mut m.IIBAHAEAEEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EAAMMPJFKIB::EAAMMPJFKIB>(
            "ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF",
            |m: &AAIEELNIBHL| { &m.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF },
            |m: &mut AAIEELNIBHL| { &mut m.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FANHHMLOGPI::FANHHMLOGPI>(
            "GNKGCONLODG",
            |m: &AAIEELNIBHL| { &m.GNKGCONLODG },
            |m: &mut AAIEELNIBHL| { &mut m.GNKGCONLODG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GFMELEMCJNA::GFMELEMCJNA>(
            "BPGELDJGKGJ",
            |m: &AAIEELNIBHL| { &m.BPGELDJGKGJ },
            |m: &mut AAIEELNIBHL| { &mut m.BPGELDJGKGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OMJCICFMBEH::OMJCICFMBEH>(
            "EEKLHGHIOGB",
            |m: &AAIEELNIBHL| { &m.EEKLHGHIOGB },
            |m: &mut AAIEELNIBHL| { &mut m.EEKLHGHIOGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ODHFHOOEDPJ::ODHFHOOEDPJ>(
            "FFFDOIGNDFN",
            |m: &AAIEELNIBHL| { &m.FFFDOIGNDFN },
            |m: &mut AAIEELNIBHL| { &mut m.FFFDOIGNDFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MJJAIIIDPPD::MJJAIIIDPPD>(
            "MPFMPMLGOGE",
            |m: &AAIEELNIBHL| { &m.MPFMPMLGOGE },
            |m: &mut AAIEELNIBHL| { &mut m.MPFMPMLGOGE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AAIEELNIBHL>(
            "AAIEELNIBHL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AAIEELNIBHL {
    const NAME: &'static str = "AAIEELNIBHL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.level)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OKBKBPJPDOB)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EPAFPEJNMFB)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IIBAHAEAEEB)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GNKGCONLODG)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BPGELDJGKGJ)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EEKLHGHIOGB)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FFFDOIGNDFN)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MPFMPMLGOGE)?;
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
        if let Some(v) = self.OKBKBPJPDOB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EPAFPEJNMFB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GNKGCONLODG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BPGELDJGKGJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EEKLHGHIOGB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FFFDOIGNDFN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MPFMPMLGOGE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.level.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.OKBKBPJPDOB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.EPAFPEJNMFB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.IIBAHAEAEEB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.GNKGCONLODG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.BPGELDJGKGJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.EEKLHGHIOGB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.FFFDOIGNDFN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.MPFMPMLGOGE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> AAIEELNIBHL {
        AAIEELNIBHL::new()
    }

    fn clear(&mut self) {
        self.level.clear();
        self.OKBKBPJPDOB.clear();
        self.EPAFPEJNMFB.clear();
        self.IIBAHAEAEEB.clear();
        self.ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF.clear();
        self.GNKGCONLODG.clear();
        self.BPGELDJGKGJ.clear();
        self.EEKLHGHIOGB.clear();
        self.FFFDOIGNDFN.clear();
        self.MPFMPMLGOGE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AAIEELNIBHL {
        static instance: AAIEELNIBHL = AAIEELNIBHL {
            level: ::protobuf::MessageField::none(),
            OKBKBPJPDOB: ::protobuf::MessageField::none(),
            EPAFPEJNMFB: ::protobuf::MessageField::none(),
            IIBAHAEAEEB: ::protobuf::MessageField::none(),
            ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BUFF: ::protobuf::MessageField::none(),
            GNKGCONLODG: ::protobuf::MessageField::none(),
            BPGELDJGKGJ: ::protobuf::MessageField::none(),
            EEKLHGHIOGB: ::protobuf::MessageField::none(),
            FFFDOIGNDFN: ::protobuf::MessageField::none(),
            MPFMPMLGOGE: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AAIEELNIBHL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AAIEELNIBHL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AAIEELNIBHL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AAIEELNIBHL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AAIEELNIBHL.proto\x1a\x11BFEJLAJIPNG.proto\x1a\x11EAAMMPJFKIB.prot\
    o\x1a\x11FANHHMLOGPI.proto\x1a\x11FOBCIMDNKPI.proto\x1a\x11GFMELEMCJNA.p\
    roto\x1a\x11IHKCKOAOMMA.proto\x1a\x11MJJAIIIDPPD.proto\x1a\x11ODHFHOOEDP\
    J.proto\x1a\x11OMJCICFMBEH.proto\x1a\x11PDHGDLPCLGK.proto\"\x9b\x04\n\
    \x0bAAIEELNIBHL\x12\"\n\x05level\x18\x01\x20\x01(\x0b2\x0c.IHKCKOAOMMAR\
    \x05level\x12.\n\x0bOKBKBPJPDOB\x18\x04\x20\x01(\x0b2\x0c.PDHGDLPCLGKR\
    \x0bOKBKBPJPDOB\x12.\n\x0bEPAFPEJNMFB\x18\x08\x20\x01(\x0b2\x0c.FOBCIMDN\
    KPIR\x0bEPAFPEJNMFB\x12.\n\x0bIIBAHAEAEEB\x18\x02\x20\x01(\x0b2\x0c.BFEJ\
    LAJIPNGR\x0bIIBAHAEAEEB\x12h\n+ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_BU\
    FF\x18\x0f\x20\x01(\x0b2\x0c.EAAMMPJFKIBR%ROGUECOMMONACTIONRESULTSOURCET\
    YPEBUFF\x12.\n\x0bGNKGCONLODG\x18\t\x20\x01(\x0b2\x0c.FANHHMLOGPIR\x0bGN\
    KGCONLODG\x12.\n\x0bBPGELDJGKGJ\x18\x0e\x20\x01(\x0b2\x0c.GFMELEMCJNAR\
    \x0bBPGELDJGKGJ\x12.\n\x0bEEKLHGHIOGB\x18\n\x20\x01(\x0b2\x0c.OMJCICFMBE\
    HR\x0bEEKLHGHIOGB\x12.\n\x0bFFFDOIGNDFN\x18\x03\x20\x01(\x0b2\x0c.ODHFHO\
    OEDPJR\x0bFFFDOIGNDFN\x12.\n\x0bMPFMPMLGOGE\x18\x07\x20\x01(\x0b2\x0c.MJ\
    JAIIIDPPDR\x0bMPFMPMLGOGEb\x06proto3\
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
            deps.push(super::BFEJLAJIPNG::file_descriptor().clone());
            deps.push(super::EAAMMPJFKIB::file_descriptor().clone());
            deps.push(super::FANHHMLOGPI::file_descriptor().clone());
            deps.push(super::FOBCIMDNKPI::file_descriptor().clone());
            deps.push(super::GFMELEMCJNA::file_descriptor().clone());
            deps.push(super::IHKCKOAOMMA::file_descriptor().clone());
            deps.push(super::MJJAIIIDPPD::file_descriptor().clone());
            deps.push(super::ODHFHOOEDPJ::file_descriptor().clone());
            deps.push(super::OMJCICFMBEH::file_descriptor().clone());
            deps.push(super::PDHGDLPCLGK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AAIEELNIBHL::generated_message_descriptor_data());
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
