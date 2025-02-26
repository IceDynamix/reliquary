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

//! Generated file from `GameplayCounterUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GameplayCounterUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GameplayCounterUpdateScNotify {
    // message fields
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.ALIFPIHNMEK)
    pub ALIFPIHNMEK: ::protobuf::EnumOrUnknown<super::CNPPAAMMFFD::CNPPAAMMFFD>,
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.BLOGJDCKAHM)
    pub BLOGJDCKAHM: u32,
    // @@protoc_insertion_point(field:GameplayCounterUpdateScNotify.NMGLNHPANAH)
    pub NMGLNHPANAH: u32,
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
            "ALIFPIHNMEK",
            |m: &GameplayCounterUpdateScNotify| { &m.ALIFPIHNMEK },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.ALIFPIHNMEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLOGJDCKAHM",
            |m: &GameplayCounterUpdateScNotify| { &m.BLOGJDCKAHM },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.BLOGJDCKAHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMGLNHPANAH",
            |m: &GameplayCounterUpdateScNotify| { &m.NMGLNHPANAH },
            |m: &mut GameplayCounterUpdateScNotify| { &mut m.NMGLNHPANAH },
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
                88 => {
                    self.ALIFPIHNMEK = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.BLOGJDCKAHM = is.read_uint32()?;
                },
                64 => {
                    self.NMGLNHPANAH = is.read_uint32()?;
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
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::CNPPAAMMFFD::CNPPAAMMFFD::GAMEPLAY_COUNTER_UPDATE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.ALIFPIHNMEK.value());
        }
        if self.BLOGJDCKAHM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BLOGJDCKAHM);
        }
        if self.NMGLNHPANAH != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.NMGLNHPANAH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::CNPPAAMMFFD::CNPPAAMMFFD::GAMEPLAY_COUNTER_UPDATE_REASON_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.ALIFPIHNMEK))?;
        }
        if self.BLOGJDCKAHM != 0 {
            os.write_uint32(4, self.BLOGJDCKAHM)?;
        }
        if self.NMGLNHPANAH != 0 {
            os.write_uint32(8, self.NMGLNHPANAH)?;
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
        self.ALIFPIHNMEK = ::protobuf::EnumOrUnknown::new(super::CNPPAAMMFFD::CNPPAAMMFFD::GAMEPLAY_COUNTER_UPDATE_REASON_NONE);
        self.BLOGJDCKAHM = 0;
        self.NMGLNHPANAH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GameplayCounterUpdateScNotify {
        static instance: GameplayCounterUpdateScNotify = GameplayCounterUpdateScNotify {
            ALIFPIHNMEK: ::protobuf::EnumOrUnknown::from_i32(0),
            BLOGJDCKAHM: 0,
            NMGLNHPANAH: 0,
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
    \n#GameplayCounterUpdateScNotify.proto\x1a\x11CNPPAAMMFFD.proto\"\x93\
    \x01\n\x1dGameplayCounterUpdateScNotify\x12.\n\x0bALIFPIHNMEK\x18\x0b\
    \x20\x01(\x0e2\x0c.CNPPAAMMFFDR\x0bALIFPIHNMEK\x12\x20\n\x0bBLOGJDCKAHM\
    \x18\x04\x20\x01(\rR\x0bBLOGJDCKAHM\x12\x20\n\x0bNMGLNHPANAH\x18\x08\x20\
    \x01(\rR\x0bNMGLNHPANAHb\x06proto3\
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
            deps.push(super::CNPPAAMMFFD::file_descriptor().clone());
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
