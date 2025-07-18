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

//! Generated file from `RogueTournFinishInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueTournFinishInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournFinishInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueTournFinishInfo.rogue__lineup_info)
    pub rogue__lineup_info: ::protobuf::MessageField<super::LineupInfo::LineupInfo>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.CJCOJAMLEEL)
    pub CJCOJAMLEEL: ::protobuf::MessageField<super::NKPKIAAMODG::NKPKIAAMODG>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.KGCIAIAFIBE)
    pub KGCIAIAFIBE: ::protobuf::MessageField<super::GPNJMEHNDMN::GPNJMEHNDMN>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.CLKHPONDDDO)
    pub CLKHPONDDDO: ::protobuf::MessageField<super::KCLCHJMNPGL::KCLCHJMNPGL>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.PFOEPFPHFNJ)
    pub PFOEPFPHFNJ: ::protobuf::MessageField<super::FBHNFJCNHML::FBHNFJCNHML>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.BHMHLPCHKLG)
    pub BHMHLPCHKLG: ::protobuf::MessageField<super::CFCDHLPOOGC::CFCDHLPOOGC>,
    // @@protoc_insertion_point(field:RogueTournFinishInfo.GCGLNKFDKKN)
    pub GCGLNKFDKKN: ::protobuf::MessageField<super::NNIJCDKHPKL::NNIJCDKHPKL>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournFinishInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournFinishInfo {
    fn default() -> &'a RogueTournFinishInfo {
        <RogueTournFinishInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournFinishInfo {
    pub fn new() -> RogueTournFinishInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LineupInfo::LineupInfo>(
            "rogue__lineup_info",
            |m: &RogueTournFinishInfo| { &m.rogue__lineup_info },
            |m: &mut RogueTournFinishInfo| { &mut m.rogue__lineup_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NKPKIAAMODG::NKPKIAAMODG>(
            "CJCOJAMLEEL",
            |m: &RogueTournFinishInfo| { &m.CJCOJAMLEEL },
            |m: &mut RogueTournFinishInfo| { &mut m.CJCOJAMLEEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GPNJMEHNDMN::GPNJMEHNDMN>(
            "KGCIAIAFIBE",
            |m: &RogueTournFinishInfo| { &m.KGCIAIAFIBE },
            |m: &mut RogueTournFinishInfo| { &mut m.KGCIAIAFIBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KCLCHJMNPGL::KCLCHJMNPGL>(
            "CLKHPONDDDO",
            |m: &RogueTournFinishInfo| { &m.CLKHPONDDDO },
            |m: &mut RogueTournFinishInfo| { &mut m.CLKHPONDDDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FBHNFJCNHML::FBHNFJCNHML>(
            "PFOEPFPHFNJ",
            |m: &RogueTournFinishInfo| { &m.PFOEPFPHFNJ },
            |m: &mut RogueTournFinishInfo| { &mut m.PFOEPFPHFNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CFCDHLPOOGC::CFCDHLPOOGC>(
            "BHMHLPCHKLG",
            |m: &RogueTournFinishInfo| { &m.BHMHLPCHKLG },
            |m: &mut RogueTournFinishInfo| { &mut m.BHMHLPCHKLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NNIJCDKHPKL::NNIJCDKHPKL>(
            "GCGLNKFDKKN",
            |m: &RogueTournFinishInfo| { &m.GCGLNKFDKKN },
            |m: &mut RogueTournFinishInfo| { &mut m.GCGLNKFDKKN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournFinishInfo>(
            "RogueTournFinishInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournFinishInfo {
    const NAME: &'static str = "RogueTournFinishInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue__lineup_info)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CJCOJAMLEEL)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KGCIAIAFIBE)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CLKHPONDDDO)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PFOEPFPHFNJ)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BHMHLPCHKLG)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GCGLNKFDKKN)?;
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
        if let Some(v) = self.rogue__lineup_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CJCOJAMLEEL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KGCIAIAFIBE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CLKHPONDDDO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PFOEPFPHFNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GCGLNKFDKKN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.rogue__lineup_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.CJCOJAMLEEL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.KGCIAIAFIBE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.CLKHPONDDDO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.PFOEPFPHFNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.BHMHLPCHKLG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.GCGLNKFDKKN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
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

    fn new() -> RogueTournFinishInfo {
        RogueTournFinishInfo::new()
    }

    fn clear(&mut self) {
        self.rogue__lineup_info.clear();
        self.CJCOJAMLEEL.clear();
        self.KGCIAIAFIBE.clear();
        self.CLKHPONDDDO.clear();
        self.PFOEPFPHFNJ.clear();
        self.BHMHLPCHKLG.clear();
        self.GCGLNKFDKKN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournFinishInfo {
        static instance: RogueTournFinishInfo = RogueTournFinishInfo {
            rogue__lineup_info: ::protobuf::MessageField::none(),
            CJCOJAMLEEL: ::protobuf::MessageField::none(),
            KGCIAIAFIBE: ::protobuf::MessageField::none(),
            CLKHPONDDDO: ::protobuf::MessageField::none(),
            PFOEPFPHFNJ: ::protobuf::MessageField::none(),
            BHMHLPCHKLG: ::protobuf::MessageField::none(),
            GCGLNKFDKKN: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournFinishInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournFinishInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournFinishInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournFinishInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aRogueTournFinishInfo.proto\x1a\x11CFCDHLPOOGC.proto\x1a\x11FBHNFJC\
    NHML.proto\x1a\x11GPNJMEHNDMN.proto\x1a\x11KCLCHJMNPGL.proto\x1a\x10Line\
    upInfo.proto\x1a\x11NKPKIAAMODG.proto\x1a\x11NNIJCDKHPKL.proto\"\xf0\x02\
    \n\x14RogueTournFinishInfo\x128\n\x12rogue__lineup_info\x18\x06\x20\x01(\
    \x0b2\x0b.LineupInfoR\x0frogueLineupInfo\x12.\n\x0bCJCOJAMLEEL\x18\x02\
    \x20\x01(\x0b2\x0c.NKPKIAAMODGR\x0bCJCOJAMLEEL\x12.\n\x0bKGCIAIAFIBE\x18\
    \x0c\x20\x01(\x0b2\x0c.GPNJMEHNDMNR\x0bKGCIAIAFIBE\x12.\n\x0bCLKHPONDDDO\
    \x18\x0e\x20\x01(\x0b2\x0c.KCLCHJMNPGLR\x0bCLKHPONDDDO\x12.\n\x0bPFOEPFP\
    HFNJ\x18\x05\x20\x01(\x0b2\x0c.FBHNFJCNHMLR\x0bPFOEPFPHFNJ\x12.\n\x0bBHM\
    HLPCHKLG\x18\r\x20\x01(\x0b2\x0c.CFCDHLPOOGCR\x0bBHMHLPCHKLG\x12.\n\x0bG\
    CGLNKFDKKN\x18\n\x20\x01(\x0b2\x0c.NNIJCDKHPKLR\x0bGCGLNKFDKKNb\x06proto\
    3\
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
            deps.push(super::CFCDHLPOOGC::file_descriptor().clone());
            deps.push(super::FBHNFJCNHML::file_descriptor().clone());
            deps.push(super::GPNJMEHNDMN::file_descriptor().clone());
            deps.push(super::KCLCHJMNPGL::file_descriptor().clone());
            deps.push(super::LineupInfo::file_descriptor().clone());
            deps.push(super::NKPKIAAMODG::file_descriptor().clone());
            deps.push(super::NNIJCDKHPKL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueTournFinishInfo::generated_message_descriptor_data());
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
