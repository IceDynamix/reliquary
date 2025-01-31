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

//! Generated file from `PlayerKickOutScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerKickOutScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerKickOutScNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerKickOutScNotify.KJLNBIBOLBJ)
    pub KJLNBIBOLBJ: ::protobuf::EnumOrUnknown<super::AMJMENEBFKG::AMJMENEBFKG>,
    // @@protoc_insertion_point(field:PlayerKickOutScNotify.MFJPFLGKHDI)
    pub MFJPFLGKHDI: ::protobuf::MessageField<super::BlackInfo::BlackInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerKickOutScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerKickOutScNotify {
    fn default() -> &'a PlayerKickOutScNotify {
        <PlayerKickOutScNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerKickOutScNotify {
    pub fn new() -> PlayerKickOutScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJLNBIBOLBJ",
            |m: &PlayerKickOutScNotify| { &m.KJLNBIBOLBJ },
            |m: &mut PlayerKickOutScNotify| { &mut m.KJLNBIBOLBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BlackInfo::BlackInfo>(
            "MFJPFLGKHDI",
            |m: &PlayerKickOutScNotify| { &m.MFJPFLGKHDI },
            |m: &mut PlayerKickOutScNotify| { &mut m.MFJPFLGKHDI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerKickOutScNotify>(
            "PlayerKickOutScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerKickOutScNotify {
    const NAME: &'static str = "PlayerKickOutScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.KJLNBIBOLBJ = is.read_enum_or_unknown()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MFJPFLGKHDI)?;
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
        if self.KJLNBIBOLBJ != ::protobuf::EnumOrUnknown::new(super::AMJMENEBFKG::AMJMENEBFKG::KICK_SQUEEZED) {
            my_size += ::protobuf::rt::int32_size(9, self.KJLNBIBOLBJ.value());
        }
        if let Some(v) = self.MFJPFLGKHDI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KJLNBIBOLBJ != ::protobuf::EnumOrUnknown::new(super::AMJMENEBFKG::AMJMENEBFKG::KICK_SQUEEZED) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.KJLNBIBOLBJ))?;
        }
        if let Some(v) = self.MFJPFLGKHDI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> PlayerKickOutScNotify {
        PlayerKickOutScNotify::new()
    }

    fn clear(&mut self) {
        self.KJLNBIBOLBJ = ::protobuf::EnumOrUnknown::new(super::AMJMENEBFKG::AMJMENEBFKG::KICK_SQUEEZED);
        self.MFJPFLGKHDI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerKickOutScNotify {
        static instance: PlayerKickOutScNotify = PlayerKickOutScNotify {
            KJLNBIBOLBJ: ::protobuf::EnumOrUnknown::from_i32(0),
            MFJPFLGKHDI: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerKickOutScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerKickOutScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerKickOutScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerKickOutScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bPlayerKickOutScNotify.proto\x1a\x11AMJMENEBFKG.proto\x1a\x0fBlackI\
    nfo.proto\"u\n\x15PlayerKickOutScNotify\x12.\n\x0bKJLNBIBOLBJ\x18\t\x20\
    \x01(\x0e2\x0c.AMJMENEBFKGR\x0bKJLNBIBOLBJ\x12,\n\x0bMFJPFLGKHDI\x18\x02\
    \x20\x01(\x0b2\n.BlackInfoR\x0bMFJPFLGKHDIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AMJMENEBFKG::file_descriptor().clone());
            deps.push(super::BlackInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerKickOutScNotify::generated_message_descriptor_data());
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
