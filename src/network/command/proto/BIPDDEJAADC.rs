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

//! Generated file from `BIPDDEJAADC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BIPDDEJAADC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BIPDDEJAADC {
    // message fields
    // @@protoc_insertion_point(field:BIPDDEJAADC.rogue_current_game_info)
    pub rogue_current_game_info: ::std::vec::Vec<super::RogueGameInfo::RogueGameInfo>,
    // @@protoc_insertion_point(field:BIPDDEJAADC.rogue_sub_mode)
    pub rogue_sub_mode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BIPDDEJAADC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BIPDDEJAADC {
    fn default() -> &'a BIPDDEJAADC {
        <BIPDDEJAADC as ::protobuf::Message>::default_instance()
    }
}

impl BIPDDEJAADC {
    pub fn new() -> BIPDDEJAADC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rogue_current_game_info",
            |m: &BIPDDEJAADC| { &m.rogue_current_game_info },
            |m: &mut BIPDDEJAADC| { &mut m.rogue_current_game_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_sub_mode",
            |m: &BIPDDEJAADC| { &m.rogue_sub_mode },
            |m: &mut BIPDDEJAADC| { &mut m.rogue_sub_mode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BIPDDEJAADC>(
            "BIPDDEJAADC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BIPDDEJAADC {
    const NAME: &'static str = "BIPDDEJAADC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.rogue_current_game_info.push(is.read_message()?);
                },
                120 => {
                    self.rogue_sub_mode = is.read_uint32()?;
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
        for value in &self.rogue_current_game_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rogue_sub_mode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.rogue_sub_mode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.rogue_current_game_info {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.rogue_sub_mode != 0 {
            os.write_uint32(15, self.rogue_sub_mode)?;
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

    fn new() -> BIPDDEJAADC {
        BIPDDEJAADC::new()
    }

    fn clear(&mut self) {
        self.rogue_current_game_info.clear();
        self.rogue_sub_mode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BIPDDEJAADC {
        static instance: BIPDDEJAADC = BIPDDEJAADC {
            rogue_current_game_info: ::std::vec::Vec::new(),
            rogue_sub_mode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BIPDDEJAADC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BIPDDEJAADC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BIPDDEJAADC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BIPDDEJAADC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BIPDDEJAADC.proto\x1a\x13RogueGameInfo.proto\"z\n\x0bBIPDDEJAADC\
    \x12E\n\x17rogue_current_game_info\x18\x0c\x20\x03(\x0b2\x0e.RogueGameIn\
    foR\x14rogueCurrentGameInfo\x12$\n\x0erogue_sub_mode\x18\x0f\x20\x01(\rR\
    \x0crogueSubModeb\x06proto3\
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
            deps.push(super::RogueGameInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BIPDDEJAADC::generated_message_descriptor_data());
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
