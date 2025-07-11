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

//! Generated file from `ChessRogueEnterNextLayerScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChessRogueEnterNextLayerScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueEnterNextLayerScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.DBDGAHBLGBB)
    pub DBDGAHBLGBB: ::protobuf::MessageField<super::OJLEEFJELAP::OJLEEFJELAP>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.stage_info)
    pub stage_info: ::protobuf::MessageField<super::ChessRogueInfo::ChessRogueInfo>,
    // @@protoc_insertion_point(field:ChessRogueEnterNextLayerScRsp.rogue_game_info)
    pub rogue_game_info: ::protobuf::MessageField<super::ChessRogueGameInfo::ChessRogueGameInfo>,
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
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OJLEEFJELAP::OJLEEFJELAP>(
            "DBDGAHBLGBB",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.DBDGAHBLGBB },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.DBDGAHBLGBB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.retcode },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueInfo::ChessRogueInfo>(
            "stage_info",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.stage_info },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.stage_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueGameInfo::ChessRogueGameInfo>(
            "rogue_game_info",
            |m: &ChessRogueEnterNextLayerScRsp| { &m.rogue_game_info },
            |m: &mut ChessRogueEnterNextLayerScRsp| { &mut m.rogue_game_info },
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
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DBDGAHBLGBB)?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stage_info)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_game_info)?;
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
        if let Some(v) = self.DBDGAHBLGBB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if let Some(v) = self.stage_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_game_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.DBDGAHBLGBB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if let Some(v) = self.stage_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.rogue_game_info.as_ref() {
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

    fn new() -> ChessRogueEnterNextLayerScRsp {
        ChessRogueEnterNextLayerScRsp::new()
    }

    fn clear(&mut self) {
        self.DBDGAHBLGBB.clear();
        self.retcode = 0;
        self.stage_info.clear();
        self.rogue_game_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueEnterNextLayerScRsp {
        static instance: ChessRogueEnterNextLayerScRsp = ChessRogueEnterNextLayerScRsp {
            DBDGAHBLGBB: ::protobuf::MessageField::none(),
            retcode: 0,
            stage_info: ::protobuf::MessageField::none(),
            rogue_game_info: ::protobuf::MessageField::none(),
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
    \n#ChessRogueEnterNextLayerScRsp.proto\x1a\x18ChessRogueGameInfo.proto\
    \x1a\x14ChessRogueInfo.proto\x1a\x11OJLEEFJELAP.proto\"\xd6\x01\n\x1dChe\
    ssRogueEnterNextLayerScRsp\x12.\n\x0bDBDGAHBLGBB\x18\x0e\x20\x01(\x0b2\
    \x0c.OJLEEFJELAPR\x0bDBDGAHBLGBB\x12\x18\n\x07retcode\x18\x0c\x20\x01(\r\
    R\x07retcode\x12.\n\nstage_info\x18\x05\x20\x01(\x0b2\x0f.ChessRogueInfo\
    R\tstageInfo\x12;\n\x0frogue_game_info\x18\x07\x20\x01(\x0b2\x13.ChessRo\
    gueGameInfoR\rrogueGameInfob\x06proto3\
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
            deps.push(super::ChessRogueGameInfo::file_descriptor().clone());
            deps.push(super::ChessRogueInfo::file_descriptor().clone());
            deps.push(super::OJLEEFJELAP::file_descriptor().clone());
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
