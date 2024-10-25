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

//! Generated file from `OFLMBKLKONH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OFLMBKLKONH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OFLMBKLKONH {
    // message fields
    // @@protoc_insertion_point(field:OFLMBKLKONH.EHDFJGODBBD)
    pub EHDFJGODBBD: u32,
    // @@protoc_insertion_point(field:OFLMBKLKONH.FHHHADKHHEF)
    pub FHHHADKHHEF: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // @@protoc_insertion_point(field:OFLMBKLKONH.LENIAKPNNHF)
    pub LENIAKPNNHF: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // @@protoc_insertion_point(field:OFLMBKLKONH.DNEOAMOBPGM)
    pub DNEOAMOBPGM: u32,
    // @@protoc_insertion_point(field:OFLMBKLKONH.JFODJKAADCL)
    pub JFODJKAADCL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OFLMBKLKONH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OFLMBKLKONH {
    fn default() -> &'a OFLMBKLKONH {
        <OFLMBKLKONH as ::protobuf::Message>::default_instance()
    }
}

impl OFLMBKLKONH {
    pub fn new() -> OFLMBKLKONH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EHDFJGODBBD",
            |m: &OFLMBKLKONH| { &m.EHDFJGODBBD },
            |m: &mut OFLMBKLKONH| { &mut m.EHDFJGODBBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHHHADKHHEF",
            |m: &OFLMBKLKONH| { &m.FHHHADKHHEF },
            |m: &mut OFLMBKLKONH| { &mut m.FHHHADKHHEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LENIAKPNNHF",
            |m: &OFLMBKLKONH| { &m.LENIAKPNNHF },
            |m: &mut OFLMBKLKONH| { &mut m.LENIAKPNNHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNEOAMOBPGM",
            |m: &OFLMBKLKONH| { &m.DNEOAMOBPGM },
            |m: &mut OFLMBKLKONH| { &mut m.DNEOAMOBPGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFODJKAADCL",
            |m: &OFLMBKLKONH| { &m.JFODJKAADCL },
            |m: &mut OFLMBKLKONH| { &mut m.JFODJKAADCL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OFLMBKLKONH>(
            "OFLMBKLKONH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OFLMBKLKONH {
    const NAME: &'static str = "OFLMBKLKONH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.EHDFJGODBBD = is.read_uint32()?;
                },
                120 => {
                    self.FHHHADKHHEF = is.read_enum_or_unknown()?;
                },
                96 => {
                    self.LENIAKPNNHF = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.DNEOAMOBPGM = is.read_uint32()?;
                },
                24 => {
                    self.JFODJKAADCL = is.read_uint32()?;
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
        if self.EHDFJGODBBD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.EHDFJGODBBD);
        }
        if self.FHHHADKHHEF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.FHHHADKHHEF.value());
        }
        if self.LENIAKPNNHF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.LENIAKPNNHF.value());
        }
        if self.DNEOAMOBPGM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DNEOAMOBPGM);
        }
        if self.JFODJKAADCL != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JFODJKAADCL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EHDFJGODBBD != 0 {
            os.write_uint32(14, self.EHDFJGODBBD)?;
        }
        if self.FHHHADKHHEF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.FHHHADKHHEF))?;
        }
        if self.LENIAKPNNHF != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.LENIAKPNNHF))?;
        }
        if self.DNEOAMOBPGM != 0 {
            os.write_uint32(8, self.DNEOAMOBPGM)?;
        }
        if self.JFODJKAADCL != 0 {
            os.write_uint32(3, self.JFODJKAADCL)?;
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

    fn new() -> OFLMBKLKONH {
        OFLMBKLKONH::new()
    }

    fn clear(&mut self) {
        self.EHDFJGODBBD = 0;
        self.FHHHADKHHEF = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.LENIAKPNNHF = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.DNEOAMOBPGM = 0;
        self.JFODJKAADCL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OFLMBKLKONH {
        static instance: OFLMBKLKONH = OFLMBKLKONH {
            EHDFJGODBBD: 0,
            FHHHADKHHEF: ::protobuf::EnumOrUnknown::from_i32(0),
            LENIAKPNNHF: ::protobuf::EnumOrUnknown::from_i32(0),
            DNEOAMOBPGM: 0,
            JFODJKAADCL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OFLMBKLKONH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OFLMBKLKONH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OFLMBKLKONH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OFLMBKLKONH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OFLMBKLKONH.proto\x1a\x15RogueRoomStatus.proto\"\xdb\x01\n\x0bOFLM\
    BKLKONH\x12\x20\n\x0bEHDFJGODBBD\x18\x0e\x20\x01(\rR\x0bEHDFJGODBBD\x122\
    \n\x0bFHHHADKHHEF\x18\x0f\x20\x01(\x0e2\x10.RogueRoomStatusR\x0bFHHHADKH\
    HEF\x122\n\x0bLENIAKPNNHF\x18\x0c\x20\x01(\x0e2\x10.RogueRoomStatusR\x0b\
    LENIAKPNNHF\x12\x20\n\x0bDNEOAMOBPGM\x18\x08\x20\x01(\rR\x0bDNEOAMOBPGM\
    \x12\x20\n\x0bJFODJKAADCL\x18\x03\x20\x01(\rR\x0bJFODJKAADCLb\x06proto3\
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
            deps.push(super::RogueRoomStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OFLMBKLKONH::generated_message_descriptor_data());
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
