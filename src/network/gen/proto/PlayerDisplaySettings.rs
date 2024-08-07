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

//! Generated file from `PlayerDisplaySettings.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerDisplaySettings)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerDisplaySettings {
    // message fields
    // @@protoc_insertion_point(field:PlayerDisplaySettings.KEACGBKOFKF)
    pub KEACGBKOFKF: bool,
    // @@protoc_insertion_point(field:PlayerDisplaySettings.DAFHJJEDMOF)
    pub DAFHJJEDMOF: bool,
    // @@protoc_insertion_point(field:PlayerDisplaySettings.KDAKDMCGFND)
    pub KDAKDMCGFND: bool,
    // @@protoc_insertion_point(field:PlayerDisplaySettings.BPELFJGIJID)
    pub BPELFJGIJID: bool,
    // @@protoc_insertion_point(field:PlayerDisplaySettings.DKLJGCEHPJL)
    pub DKLJGCEHPJL: bool,
    // @@protoc_insertion_point(field:PlayerDisplaySettings.EPMCKMCDIGB)
    pub EPMCKMCDIGB: ::protobuf::EnumOrUnknown<super::DisplayRecordType::DisplayRecordType>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerDisplaySettings.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerDisplaySettings {
    fn default() -> &'a PlayerDisplaySettings {
        <PlayerDisplaySettings as ::protobuf::Message>::default_instance()
    }
}

impl PlayerDisplaySettings {
    pub fn new() -> PlayerDisplaySettings {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KEACGBKOFKF",
            |m: &PlayerDisplaySettings| { &m.KEACGBKOFKF },
            |m: &mut PlayerDisplaySettings| { &mut m.KEACGBKOFKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAFHJJEDMOF",
            |m: &PlayerDisplaySettings| { &m.DAFHJJEDMOF },
            |m: &mut PlayerDisplaySettings| { &mut m.DAFHJJEDMOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDAKDMCGFND",
            |m: &PlayerDisplaySettings| { &m.KDAKDMCGFND },
            |m: &mut PlayerDisplaySettings| { &mut m.KDAKDMCGFND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPELFJGIJID",
            |m: &PlayerDisplaySettings| { &m.BPELFJGIJID },
            |m: &mut PlayerDisplaySettings| { &mut m.BPELFJGIJID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKLJGCEHPJL",
            |m: &PlayerDisplaySettings| { &m.DKLJGCEHPJL },
            |m: &mut PlayerDisplaySettings| { &mut m.DKLJGCEHPJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPMCKMCDIGB",
            |m: &PlayerDisplaySettings| { &m.EPMCKMCDIGB },
            |m: &mut PlayerDisplaySettings| { &mut m.EPMCKMCDIGB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerDisplaySettings>(
            "PlayerDisplaySettings",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerDisplaySettings {
    const NAME: &'static str = "PlayerDisplaySettings";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.KEACGBKOFKF = is.read_bool()?;
                },
                64 => {
                    self.DAFHJJEDMOF = is.read_bool()?;
                },
                88 => {
                    self.KDAKDMCGFND = is.read_bool()?;
                },
                56 => {
                    self.BPELFJGIJID = is.read_bool()?;
                },
                40 => {
                    self.DKLJGCEHPJL = is.read_bool()?;
                },
                120 => {
                    self.EPMCKMCDIGB = is.read_enum_or_unknown()?;
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
        if self.KEACGBKOFKF != false {
            my_size += 1 + 1;
        }
        if self.DAFHJJEDMOF != false {
            my_size += 1 + 1;
        }
        if self.KDAKDMCGFND != false {
            my_size += 1 + 1;
        }
        if self.BPELFJGIJID != false {
            my_size += 1 + 1;
        }
        if self.DKLJGCEHPJL != false {
            my_size += 1 + 1;
        }
        if self.EPMCKMCDIGB != ::protobuf::EnumOrUnknown::new(super::DisplayRecordType::DisplayRecordType::BATTLE_RECORD_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.EPMCKMCDIGB.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KEACGBKOFKF != false {
            os.write_bool(13, self.KEACGBKOFKF)?;
        }
        if self.DAFHJJEDMOF != false {
            os.write_bool(8, self.DAFHJJEDMOF)?;
        }
        if self.KDAKDMCGFND != false {
            os.write_bool(11, self.KDAKDMCGFND)?;
        }
        if self.BPELFJGIJID != false {
            os.write_bool(7, self.BPELFJGIJID)?;
        }
        if self.DKLJGCEHPJL != false {
            os.write_bool(5, self.DKLJGCEHPJL)?;
        }
        if self.EPMCKMCDIGB != ::protobuf::EnumOrUnknown::new(super::DisplayRecordType::DisplayRecordType::BATTLE_RECORD_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.EPMCKMCDIGB))?;
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

    fn new() -> PlayerDisplaySettings {
        PlayerDisplaySettings::new()
    }

    fn clear(&mut self) {
        self.KEACGBKOFKF = false;
        self.DAFHJJEDMOF = false;
        self.KDAKDMCGFND = false;
        self.BPELFJGIJID = false;
        self.DKLJGCEHPJL = false;
        self.EPMCKMCDIGB = ::protobuf::EnumOrUnknown::new(super::DisplayRecordType::DisplayRecordType::BATTLE_RECORD_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerDisplaySettings {
        static instance: PlayerDisplaySettings = PlayerDisplaySettings {
            KEACGBKOFKF: false,
            DAFHJJEDMOF: false,
            KDAKDMCGFND: false,
            BPELFJGIJID: false,
            DKLJGCEHPJL: false,
            EPMCKMCDIGB: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerDisplaySettings {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerDisplaySettings").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerDisplaySettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerDisplaySettings {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bPlayerDisplaySettings.proto\x1a\x17DisplayRecordType.proto\"\xf7\
    \x01\n\x15PlayerDisplaySettings\x12\x20\n\x0bKEACGBKOFKF\x18\r\x20\x01(\
    \x08R\x0bKEACGBKOFKF\x12\x20\n\x0bDAFHJJEDMOF\x18\x08\x20\x01(\x08R\x0bD\
    AFHJJEDMOF\x12\x20\n\x0bKDAKDMCGFND\x18\x0b\x20\x01(\x08R\x0bKDAKDMCGFND\
    \x12\x20\n\x0bBPELFJGIJID\x18\x07\x20\x01(\x08R\x0bBPELFJGIJID\x12\x20\n\
    \x0bDKLJGCEHPJL\x18\x05\x20\x01(\x08R\x0bDKLJGCEHPJL\x124\n\x0bEPMCKMCDI\
    GB\x18\x0f\x20\x01(\x0e2\x12.DisplayRecordTypeR\x0bEPMCKMCDIGBB\x15\n\
    \x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::DisplayRecordType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerDisplaySettings::generated_message_descriptor_data());
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
