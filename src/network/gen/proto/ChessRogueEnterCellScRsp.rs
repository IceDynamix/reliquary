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

//! Generated file from `ChessRogueEnterCellScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueEnterCellScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueEnterCellScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueEnterCellScRsp.ROGUE_DEBUG_MESSAGE_TYPE_INFO)
    pub ROGUE_DEBUG_MESSAGE_TYPE_INFO: ::protobuf::MessageField<super::NLGOGJBLJFE::NLGOGJBLJFE>,
    // @@protoc_insertion_point(field:ChessRogueEnterCellScRsp.KMIPNOOKBMB)
    pub KMIPNOOKBMB: u32,
    // @@protoc_insertion_point(field:ChessRogueEnterCellScRsp.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::MOHDEOFNBNK::MOHDEOFNBNK>,
    // @@protoc_insertion_point(field:ChessRogueEnterCellScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChessRogueEnterCellScRsp.JMGAILLNCEL)
    pub JMGAILLNCEL: ::protobuf::MessageField<super::KFDJPCGIBEH::KFDJPCGIBEH>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueEnterCellScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueEnterCellScRsp {
    fn default() -> &'a ChessRogueEnterCellScRsp {
        <ChessRogueEnterCellScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueEnterCellScRsp {
    pub fn new() -> ChessRogueEnterCellScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NLGOGJBLJFE::NLGOGJBLJFE>(
            "ROGUE_DEBUG_MESSAGE_TYPE_INFO",
            |m: &ChessRogueEnterCellScRsp| { &m.ROGUE_DEBUG_MESSAGE_TYPE_INFO },
            |m: &mut ChessRogueEnterCellScRsp| { &mut m.ROGUE_DEBUG_MESSAGE_TYPE_INFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KMIPNOOKBMB",
            |m: &ChessRogueEnterCellScRsp| { &m.KMIPNOOKBMB },
            |m: &mut ChessRogueEnterCellScRsp| { &mut m.KMIPNOOKBMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MOHDEOFNBNK::MOHDEOFNBNK>(
            "HLDLDAPNILF",
            |m: &ChessRogueEnterCellScRsp| { &m.HLDLDAPNILF },
            |m: &mut ChessRogueEnterCellScRsp| { &mut m.HLDLDAPNILF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueEnterCellScRsp| { &m.retcode },
            |m: &mut ChessRogueEnterCellScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KFDJPCGIBEH::KFDJPCGIBEH>(
            "JMGAILLNCEL",
            |m: &ChessRogueEnterCellScRsp| { &m.JMGAILLNCEL },
            |m: &mut ChessRogueEnterCellScRsp| { &mut m.JMGAILLNCEL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueEnterCellScRsp>(
            "ChessRogueEnterCellScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueEnterCellScRsp {
    const NAME: &'static str = "ChessRogueEnterCellScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ROGUE_DEBUG_MESSAGE_TYPE_INFO)?;
                },
                48 => {
                    self.KMIPNOOKBMB = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JMGAILLNCEL)?;
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
        if let Some(v) = self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KMIPNOOKBMB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KMIPNOOKBMB);
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if let Some(v) = self.JMGAILLNCEL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.KMIPNOOKBMB != 0 {
            os.write_uint32(6, self.KMIPNOOKBMB)?;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if let Some(v) = self.JMGAILLNCEL.as_ref() {
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

    fn new() -> ChessRogueEnterCellScRsp {
        ChessRogueEnterCellScRsp::new()
    }

    fn clear(&mut self) {
        self.ROGUE_DEBUG_MESSAGE_TYPE_INFO.clear();
        self.KMIPNOOKBMB = 0;
        self.HLDLDAPNILF.clear();
        self.retcode = 0;
        self.JMGAILLNCEL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueEnterCellScRsp {
        static instance: ChessRogueEnterCellScRsp = ChessRogueEnterCellScRsp {
            ROGUE_DEBUG_MESSAGE_TYPE_INFO: ::protobuf::MessageField::none(),
            KMIPNOOKBMB: 0,
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            retcode: 0,
            JMGAILLNCEL: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueEnterCellScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueEnterCellScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueEnterCellScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueEnterCellScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eChessRogueEnterCellScRsp.proto\x1a\x11KFDJPCGIBEH.proto\x1a\x11MOH\
    DEOFNBNK.proto\x1a\x11NLGOGJBLJFE.proto\"\x86\x02\n\x18ChessRogueEnterCe\
    llScRsp\x12N\n\x1dROGUE_DEBUG_MESSAGE_TYPE_INFO\x18\x0f\x20\x01(\x0b2\
    \x0c.NLGOGJBLJFER\x19ROGUEDEBUGMESSAGETYPEINFO\x12\x20\n\x0bKMIPNOOKBMB\
    \x18\x06\x20\x01(\rR\x0bKMIPNOOKBMB\x12.\n\x0bHLDLDAPNILF\x18\x02\x20\
    \x01(\x0b2\x0c.MOHDEOFNBNKR\x0bHLDLDAPNILF\x12\x18\n\x07retcode\x18\x04\
    \x20\x01(\rR\x07retcode\x12.\n\x0bJMGAILLNCEL\x18\n\x20\x01(\x0b2\x0c.KF\
    DJPCGIBEHR\x0bJMGAILLNCELb\x06proto3\
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
            deps.push(super::KFDJPCGIBEH::file_descriptor().clone());
            deps.push(super::MOHDEOFNBNK::file_descriptor().clone());
            deps.push(super::NLGOGJBLJFE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueEnterCellScRsp::generated_message_descriptor_data());
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
