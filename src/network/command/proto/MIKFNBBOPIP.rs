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

//! Generated file from `MIKFNBBOPIP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MIKFNBBOPIP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MIKFNBBOPIP {
    // message fields
    // @@protoc_insertion_point(field:MIKFNBBOPIP.MDLNDGIJNML)
    pub MDLNDGIJNML: ::std::string::String,
    // @@protoc_insertion_point(field:MIKFNBBOPIP.rogue_magic_battle_const)
    pub rogue_magic_battle_const: u32,
    // @@protoc_insertion_point(field:MIKFNBBOPIP.ILLODGAEFAE)
    pub ILLODGAEFAE: u32,
    // @@protoc_insertion_point(field:MIKFNBBOPIP.pending_action)
    pub pending_action: ::protobuf::MessageField<super::RogueCommonPendingAction::RogueCommonPendingAction>,
    // @@protoc_insertion_point(field:MIKFNBBOPIP.AGEBAMBKKBC)
    pub AGEBAMBKKBC: u32,
    // @@protoc_insertion_point(field:MIKFNBBOPIP.rogue_sub_mode)
    pub rogue_sub_mode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MIKFNBBOPIP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MIKFNBBOPIP {
    fn default() -> &'a MIKFNBBOPIP {
        <MIKFNBBOPIP as ::protobuf::Message>::default_instance()
    }
}

impl MIKFNBBOPIP {
    pub fn new() -> MIKFNBBOPIP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDLNDGIJNML",
            |m: &MIKFNBBOPIP| { &m.MDLNDGIJNML },
            |m: &mut MIKFNBBOPIP| { &mut m.MDLNDGIJNML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_magic_battle_const",
            |m: &MIKFNBBOPIP| { &m.rogue_magic_battle_const },
            |m: &mut MIKFNBBOPIP| { &mut m.rogue_magic_battle_const },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILLODGAEFAE",
            |m: &MIKFNBBOPIP| { &m.ILLODGAEFAE },
            |m: &mut MIKFNBBOPIP| { &mut m.ILLODGAEFAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueCommonPendingAction::RogueCommonPendingAction>(
            "pending_action",
            |m: &MIKFNBBOPIP| { &m.pending_action },
            |m: &mut MIKFNBBOPIP| { &mut m.pending_action },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGEBAMBKKBC",
            |m: &MIKFNBBOPIP| { &m.AGEBAMBKKBC },
            |m: &mut MIKFNBBOPIP| { &mut m.AGEBAMBKKBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rogue_sub_mode",
            |m: &MIKFNBBOPIP| { &m.rogue_sub_mode },
            |m: &mut MIKFNBBOPIP| { &mut m.rogue_sub_mode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MIKFNBBOPIP>(
            "MIKFNBBOPIP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MIKFNBBOPIP {
    const NAME: &'static str = "MIKFNBBOPIP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.MDLNDGIJNML = is.read_string()?;
                },
                8 => {
                    self.rogue_magic_battle_const = is.read_uint32()?;
                },
                24 => {
                    self.ILLODGAEFAE = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pending_action)?;
                },
                16 => {
                    self.AGEBAMBKKBC = is.read_uint32()?;
                },
                48 => {
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
        if !self.MDLNDGIJNML.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.MDLNDGIJNML);
        }
        if self.rogue_magic_battle_const != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.rogue_magic_battle_const);
        }
        if self.ILLODGAEFAE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.ILLODGAEFAE);
        }
        if let Some(v) = self.pending_action.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AGEBAMBKKBC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.AGEBAMBKKBC);
        }
        if self.rogue_sub_mode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.rogue_sub_mode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.MDLNDGIJNML.is_empty() {
            os.write_string(7, &self.MDLNDGIJNML)?;
        }
        if self.rogue_magic_battle_const != 0 {
            os.write_uint32(1, self.rogue_magic_battle_const)?;
        }
        if self.ILLODGAEFAE != 0 {
            os.write_uint32(3, self.ILLODGAEFAE)?;
        }
        if let Some(v) = self.pending_action.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.AGEBAMBKKBC != 0 {
            os.write_uint32(2, self.AGEBAMBKKBC)?;
        }
        if self.rogue_sub_mode != 0 {
            os.write_uint32(6, self.rogue_sub_mode)?;
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

    fn new() -> MIKFNBBOPIP {
        MIKFNBBOPIP::new()
    }

    fn clear(&mut self) {
        self.MDLNDGIJNML.clear();
        self.rogue_magic_battle_const = 0;
        self.ILLODGAEFAE = 0;
        self.pending_action.clear();
        self.AGEBAMBKKBC = 0;
        self.rogue_sub_mode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MIKFNBBOPIP {
        static instance: MIKFNBBOPIP = MIKFNBBOPIP {
            MDLNDGIJNML: ::std::string::String::new(),
            rogue_magic_battle_const: 0,
            ILLODGAEFAE: 0,
            pending_action: ::protobuf::MessageField::none(),
            AGEBAMBKKBC: 0,
            rogue_sub_mode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MIKFNBBOPIP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MIKFNBBOPIP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MIKFNBBOPIP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MIKFNBBOPIP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MIKFNBBOPIP.proto\x1a\x1eRogueCommonPendingAction.proto\"\x94\x02\
    \n\x0bMIKFNBBOPIP\x12\x20\n\x0bMDLNDGIJNML\x18\x07\x20\x01(\tR\x0bMDLNDG\
    IJNML\x127\n\x18rogue_magic_battle_const\x18\x01\x20\x01(\rR\x15rogueMag\
    icBattleConst\x12\x20\n\x0bILLODGAEFAE\x18\x03\x20\x01(\rR\x0bILLODGAEFA\
    E\x12@\n\x0epending_action\x18\x0b\x20\x01(\x0b2\x19.RogueCommonPendingA\
    ctionR\rpendingAction\x12\x20\n\x0bAGEBAMBKKBC\x18\x02\x20\x01(\rR\x0bAG\
    EBAMBKKBC\x12$\n\x0erogue_sub_mode\x18\x06\x20\x01(\rR\x0crogueSubModeb\
    \x06proto3\
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
            deps.push(super::RogueCommonPendingAction::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MIKFNBBOPIP::generated_message_descriptor_data());
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
