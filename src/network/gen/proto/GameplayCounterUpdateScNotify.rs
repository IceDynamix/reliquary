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

//! Generated file from `GameplayCounterUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GameplayCounterUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GameplayCounterUpdateScNotify {
    // message fields
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.KPPGEOMIKLJ)
    pub KPPGEOMIKLJ: u32,
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::FFBMLFLALLL::FFBMLFLALLL>,
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.PFHEGFOOLHE)
    pub PFHEGFOOLHE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GameplayCounterUpdateScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GameplayCounterUpdateScNotify {
    fn default() -> &'a GameplayCounterUpdateScNotify {
        <GameplayCounterUpdateScNotify as ::protobuf::Message>::default_instance()
    }
}

impl GameplayCounterUpdateScNotify {
    pub fn new() -> GameplayCounterUpdateScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPPGEOMIKLJ",
            |m: &GameplayCounterUpdateScNotify| { &m.KPPGEOMIKLJ },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.KPPGEOMIKLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &GameplayCounterUpdateScNotify| { &m.KGGHLADEKGP },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PFHEGFOOLHE",
            |m: &GameplayCounterUpdateScNotify| { &m.PFHEGFOOLHE },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.PFHEGFOOLHE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GameplayCounterUpdateScNotify>(
            "GameplayCounterUpdateScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GameplayCounterUpdateScNotify {
    const NAME: &'static str = "GameplayCounterUpdateScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.KPPGEOMIKLJ = is.read_uint32()?;
                },
                104 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.PFHEGFOOLHE = is.read_uint32()?;
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
        if self.KPPGEOMIKLJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.KPPGEOMIKLJ);
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::FFBMLFLALLL::FFBMLFLALLL::GAMEPLAY_COUNTER_UPDATE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.KGGHLADEKGP.value());
        }
        if self.PFHEGFOOLHE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.PFHEGFOOLHE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KPPGEOMIKLJ != 0 {
            os.write_uint32(14, self.KPPGEOMIKLJ)?;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::FFBMLFLALLL::FFBMLFLALLL::GAMEPLAY_COUNTER_UPDATE_REASON_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if self.PFHEGFOOLHE != 0 {
            os.write_uint32(5, self.PFHEGFOOLHE)?;
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

    fn new() -> GameplayCounterUpdateScNotify {
        GameplayCounterUpdateScNotify::new()
    }

    fn clear(&mut self) {
        self.KPPGEOMIKLJ = 0;
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::FFBMLFLALLL::FFBMLFLALLL::GAMEPLAY_COUNTER_UPDATE_REASON_NONE);
        self.PFHEGFOOLHE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GameplayCounterUpdateScNotify {
        static instance: GameplayCounterUpdateScNotify = GameplayCounterUpdateScNotify {
            KPPGEOMIKLJ: 0,
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            PFHEGFOOLHE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GameplayCounterUpdateScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GameplayCounterUpdateScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GameplayCounterUpdateScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GameplayCounterUpdateScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#GameplayCounterUpdateScNotify.proto\x1a\x11FFBMLFLALLL.proto\"\x93\
    \x01\n\x1dGameplayCounterUpdateScNotify\x12\x20\n\x0bKPPGEOMIKLJ\x18\x0e\
    \x20\x01(\rR\x0bKPPGEOMIKLJ\x12.\n\x0bKGGHLADEKGP\x18\r\x20\x01(\x0e2\
    \x0c.FFBMLFLALLLR\x0bKGGHLADEKGP\x12\x20\n\x0bPFHEGFOOLHE\x18\x05\x20\
    \x01(\rR\x0bPFHEGFOOLHEb\x06proto3\
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
            deps.push(super::FFBMLFLALLL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GameplayCounterUpdateScNotify::generated_message_descriptor_data());
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
