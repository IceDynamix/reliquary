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

//! Generated file from `ChessRogueEnterNextLayerScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueEnterNextLayerScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueEnterNextLayerScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.CPJGLAJDNAC)
    pub CPJGLAJDNAC: ::protobuf::MessageField<super::IKHALMEKJNA::IKHALMEKJNA>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.OCGLFPNJGAB)
    pub OCGLFPNJGAB: ::protobuf::MessageField<super::NOMDNDHHELA::NOMDNDHHELA>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.POCDNJBLNAN)
    pub POCDNJBLNAN: ::protobuf::MessageField<super::DIEDGIEDKHM::DIEDGIEDKHM>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.MGBBKHHAHEH)
    pub MGBBKHHAHEH: ::protobuf::MessageField<super::ANHJCGDPCAM::ANHJCGDPCAM>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueEnterNextLayerScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueEnterNextLayerScRsp {
    fn default() -> &'a ChessRogueEnterNextLayerScRsp {
        <ChessRogueEnterNextLayerScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueEnterNextLayerScRsp {
    pub fn new() -> ChessRogueEnterNextLayerScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IKHALMEKJNA::IKHALMEKJNA>(
            "CPJGLAJDNAC",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.CPJGLAJDNAC },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.CPJGLAJDNAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NOMDNDHHELA::NOMDNDHHELA>(
            "OCGLFPNJGAB",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.OCGLFPNJGAB },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.OCGLFPNJGAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DIEDGIEDKHM::DIEDGIEDKHM>(
            "POCDNJBLNAN",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.POCDNJBLNAN },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.POCDNJBLNAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ANHJCGDPCAM::ANHJCGDPCAM>(
            "MGBBKHHAHEH",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.MGBBKHHAHEH },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.MGBBKHHAHEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.retcode },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueEnterNextLayerScRsp>(
            "ChessRogueEnterNextLayerScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueEnterNextLayerScRsp {
    const NAME: &'static str = "ChessRogueEnterNextLayerScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPJGLAJDNAC)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OCGLFPNJGAB)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.POCDNJBLNAN)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MGBBKHHAHEH)?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
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
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OCGLFPNJGAB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.POCDNJBLNAN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MGBBKHHAHEH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.OCGLFPNJGAB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.POCDNJBLNAN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.MGBBKHHAHEH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
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

    fn new() -> ChessRogueEnterNextLayerScRsp {
        ChessRogueEnterNextLayerScRsp::new()
    }

    fn clear(&mut self) {
        self.CPJGLAJDNAC.clear();
        self.OCGLFPNJGAB.clear();
        self.POCDNJBLNAN.clear();
        self.MGBBKHHAHEH.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueEnterNextLayerScRsp {
        static instance: ChessRogueEnterNextLayerScRsp = ChessRogueEnterNextLayerScRsp {
            CPJGLAJDNAC: ::protobuf::MessageField::none(),
            OCGLFPNJGAB: ::protobuf::MessageField::none(),
            POCDNJBLNAN: ::protobuf::MessageField::none(),
            MGBBKHHAHEH: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueEnterNextLayerScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueEnterNextLayerScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueEnterNextLayerScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueEnterNextLayerScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#ChessRogueEnterNextLayerScRsp.proto\x1a\x11ANHJCGDPCAM.proto\x1a\x11D\
    IEDGIEDKHM.proto\x1a\x11IKHALMEKJNA.proto\x1a\x11NOMDNDHHELA.proto\"\xf9\
    \x01\n\x1dChessRogueEnterNextLayerScRsp\x12.\n\x0bCPJGLAJDNAC\x18\x07\
    \x20\x01(\x0b2\x0c.IKHALMEKJNAR\x0bCPJGLAJDNAC\x12.\n\x0bOCGLFPNJGAB\x18\
    \x06\x20\x01(\x0b2\x0c.NOMDNDHHELAR\x0bOCGLFPNJGAB\x12.\n\x0bPOCDNJBLNAN\
    \x18\x02\x20\x01(\x0b2\x0c.DIEDGIEDKHMR\x0bPOCDNJBLNAN\x12.\n\x0bMGBBKHH\
    AHEH\x18\x0b\x20\x01(\x0b2\x0c.ANHJCGDPCAMR\x0bMGBBKHHAHEH\x12\x18\n\x07\
    retcode\x18\x04\x20\x01(\rR\x07retcodeb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::ANHJCGDPCAM::file_descriptor().clone());
            deps.push(super::DIEDGIEDKHM::file_descriptor().clone());
            deps.push(super::IKHALMEKJNA::file_descriptor().clone());
            deps.push(super::NOMDNDHHELA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueEnterNextLayerScRsp::generated_message_descriptor_data());
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
