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

//! Generated file from `LobbyInviteScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LobbyInviteScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LobbyInviteScNotify {
    // message fields
    // @@protoc_insertion_point(field:LobbyInviteScNotify.MGAIAJAKPEI)
    pub MGAIAJAKPEI: ::protobuf::EnumOrUnknown<super::FightGameMode::FightGameMode>,
    // @@protoc_insertion_point(field:LobbyInviteScNotify.JFODJKAADCL)
    pub JFODJKAADCL: u32,
    // @@protoc_insertion_point(field:LobbyInviteScNotify.PIJKIPIDACA)
    pub PIJKIPIDACA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LobbyInviteScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LobbyInviteScNotify {
    fn default() -> &'a LobbyInviteScNotify {
        <LobbyInviteScNotify as ::protobuf::Message>::default_instance()
    }
}

impl LobbyInviteScNotify {
    pub fn new() -> LobbyInviteScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGAIAJAKPEI",
            |m: &LobbyInviteScNotify| { &m.MGAIAJAKPEI },
            |m: &mut LobbyInviteScNotify| { &mut m.MGAIAJAKPEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFODJKAADCL",
            |m: &LobbyInviteScNotify| { &m.JFODJKAADCL },
            |m: &mut LobbyInviteScNotify| { &mut m.JFODJKAADCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIJKIPIDACA",
            |m: &LobbyInviteScNotify| { &m.PIJKIPIDACA },
            |m: &mut LobbyInviteScNotify| { &mut m.PIJKIPIDACA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LobbyInviteScNotify>(
            "LobbyInviteScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LobbyInviteScNotify {
    const NAME: &'static str = "LobbyInviteScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.MGAIAJAKPEI = is.read_enum_or_unknown()?;
                },
                24 => {
                    self.JFODJKAADCL = is.read_uint32()?;
                },
                40 => {
                    self.PIJKIPIDACA = is.read_uint32()?;
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
        if self.MGAIAJAKPEI != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.MGAIAJAKPEI.value());
        }
        if self.JFODJKAADCL != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JFODJKAADCL);
        }
        if self.PIJKIPIDACA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.PIJKIPIDACA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MGAIAJAKPEI != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.MGAIAJAKPEI))?;
        }
        if self.JFODJKAADCL != 0 {
            os.write_uint32(3, self.JFODJKAADCL)?;
        }
        if self.PIJKIPIDACA != 0 {
            os.write_uint32(5, self.PIJKIPIDACA)?;
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

    fn new() -> LobbyInviteScNotify {
        LobbyInviteScNotify::new()
    }

    fn clear(&mut self) {
        self.MGAIAJAKPEI = ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE);
        self.JFODJKAADCL = 0;
        self.PIJKIPIDACA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LobbyInviteScNotify {
        static instance: LobbyInviteScNotify = LobbyInviteScNotify {
            MGAIAJAKPEI: ::protobuf::EnumOrUnknown::from_i32(0),
            JFODJKAADCL: 0,
            PIJKIPIDACA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LobbyInviteScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LobbyInviteScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LobbyInviteScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LobbyInviteScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19LobbyInviteScNotify.proto\x1a\x13FightGameMode.proto\"\x8b\x01\n\
    \x13LobbyInviteScNotify\x120\n\x0bMGAIAJAKPEI\x18\x0b\x20\x01(\x0e2\x0e.\
    FightGameModeR\x0bMGAIAJAKPEI\x12\x20\n\x0bJFODJKAADCL\x18\x03\x20\x01(\
    \rR\x0bJFODJKAADCL\x12\x20\n\x0bPIJKIPIDACA\x18\x05\x20\x01(\rR\x0bPIJKI\
    PIDACAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::FightGameMode::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LobbyInviteScNotify::generated_message_descriptor_data());
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