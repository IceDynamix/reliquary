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

//! Generated file from `CLGPAKIJFGP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CLGPAKIJFGP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CLGPAKIJFGP {
    // message fields
    // @@protoc_insertion_point(field:CLGPAKIJFGP.GALKEHLDDJJ)
    pub GALKEHLDDJJ: ::std::vec::Vec<super::OEOOBAEEDKA::OEOOBAEEDKA>,
    // @@protoc_insertion_point(field:CLGPAKIJFGP.CNCKAAMNDFM)
    pub CNCKAAMNDFM: u32,
    // @@protoc_insertion_point(field:CLGPAKIJFGP.status)
    pub status: ::protobuf::EnumOrUnknown<super::RogueTournLevelStatus::RogueTournLevelStatus>,
    // @@protoc_insertion_point(field:CLGPAKIJFGP.DGDDHBLKMLI)
    pub DGDDHBLKMLI: ::protobuf::EnumOrUnknown<super::BGCHPBPOLLK::BGCHPBPOLLK>,
    // special fields
    // @@protoc_insertion_point(special_field:CLGPAKIJFGP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CLGPAKIJFGP {
    fn default() -> &'a CLGPAKIJFGP {
        <CLGPAKIJFGP as ::protobuf::Message>::default_instance()
    }
}

impl CLGPAKIJFGP {
    pub fn new() -> CLGPAKIJFGP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GALKEHLDDJJ",
            |m: &CLGPAKIJFGP| { &m.GALKEHLDDJJ },
            |m: &mut CLGPAKIJFGP| { &mut m.GALKEHLDDJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNCKAAMNDFM",
            |m: &CLGPAKIJFGP| { &m.CNCKAAMNDFM },
            |m: &mut CLGPAKIJFGP| { &mut m.CNCKAAMNDFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &CLGPAKIJFGP| { &m.status },
            |m: &mut CLGPAKIJFGP| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGDDHBLKMLI",
            |m: &CLGPAKIJFGP| { &m.DGDDHBLKMLI },
            |m: &mut CLGPAKIJFGP| { &mut m.DGDDHBLKMLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CLGPAKIJFGP>(
            "CLGPAKIJFGP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CLGPAKIJFGP {
    const NAME: &'static str = "CLGPAKIJFGP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.GALKEHLDDJJ.push(is.read_message()?);
                },
                64 => {
                    self.CNCKAAMNDFM = is.read_uint32()?;
                },
                48 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.DGDDHBLKMLI = is.read_enum_or_unknown()?;
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
        for value in &self.GALKEHLDDJJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CNCKAAMNDFM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CNCKAAMNDFM);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.status.value());
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::BGCHPBPOLLK::BGCHPBPOLLK::ROGUE_TOURN_SETTLE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.DGDDHBLKMLI.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GALKEHLDDJJ {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.CNCKAAMNDFM != 0 {
            os.write_uint32(8, self.CNCKAAMNDFM)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::BGCHPBPOLLK::BGCHPBPOLLK::ROGUE_TOURN_SETTLE_REASON_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.DGDDHBLKMLI))?;
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

    fn new() -> CLGPAKIJFGP {
        CLGPAKIJFGP::new()
    }

    fn clear(&mut self) {
        self.GALKEHLDDJJ.clear();
        self.CNCKAAMNDFM = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::RogueTournLevelStatus::RogueTournLevelStatus::ROGUE_TOURN_LEVEL_STATUS_NONE);
        self.DGDDHBLKMLI = ::protobuf::EnumOrUnknown::new(super::BGCHPBPOLLK::BGCHPBPOLLK::ROGUE_TOURN_SETTLE_REASON_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CLGPAKIJFGP {
        static instance: CLGPAKIJFGP = CLGPAKIJFGP {
            GALKEHLDDJJ: ::std::vec::Vec::new(),
            CNCKAAMNDFM: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            DGDDHBLKMLI: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CLGPAKIJFGP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CLGPAKIJFGP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CLGPAKIJFGP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CLGPAKIJFGP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CLGPAKIJFGP.proto\x1a\x11BGCHPBPOLLK.proto\x1a\x11OEOOBAEEDKA.prot\
    o\x1a\x1bRogueTournLevelStatus.proto\"\xbf\x01\n\x0bCLGPAKIJFGP\x12.\n\
    \x0bGALKEHLDDJJ\x18\r\x20\x03(\x0b2\x0c.OEOOBAEEDKAR\x0bGALKEHLDDJJ\x12\
    \x20\n\x0bCNCKAAMNDFM\x18\x08\x20\x01(\rR\x0bCNCKAAMNDFM\x12.\n\x06statu\
    s\x18\x06\x20\x01(\x0e2\x16.RogueTournLevelStatusR\x06status\x12.\n\x0bD\
    GDDHBLKMLI\x18\x01\x20\x01(\x0e2\x0c.BGCHPBPOLLKR\x0bDGDDHBLKMLIb\x06pro\
    to3\
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
            deps.push(super::BGCHPBPOLLK::file_descriptor().clone());
            deps.push(super::OEOOBAEEDKA::file_descriptor().clone());
            deps.push(super::RogueTournLevelStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CLGPAKIJFGP::generated_message_descriptor_data());
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
