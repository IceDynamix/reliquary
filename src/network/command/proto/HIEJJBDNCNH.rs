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

//! Generated file from `HIEJJBDNCNH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HIEJJBDNCNH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HIEJJBDNCNH {
    // message fields
    // @@protoc_insertion_point(field:HIEJJBDNCNH.JFPCPDCFLMD)
    pub JFPCPDCFLMD: ::protobuf::MessageField<super::OBIHNGMNKEK::OBIHNGMNKEK>,
    // @@protoc_insertion_point(field:HIEJJBDNCNH.FGMGPLJCKPC)
    pub FGMGPLJCKPC: ::protobuf::EnumOrUnknown<super::BattleRecordType::BattleRecordType>,
    // @@protoc_insertion_point(field:HIEJJBDNCNH.EBOOMGDGNEP)
    pub EBOOMGDGNEP: ::protobuf::MessageField<super::IHKGNJDNALJ::IHKGNJDNALJ>,
    // special fields
    // @@protoc_insertion_point(special_field:HIEJJBDNCNH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HIEJJBDNCNH {
    fn default() -> &'a HIEJJBDNCNH {
        <HIEJJBDNCNH as ::protobuf::Message>::default_instance()
    }
}

impl HIEJJBDNCNH {
    pub fn new() -> HIEJJBDNCNH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OBIHNGMNKEK::OBIHNGMNKEK>(
            "JFPCPDCFLMD",
            |m: &HIEJJBDNCNH| { &m.JFPCPDCFLMD },
            |m: &mut HIEJJBDNCNH| { &mut m.JFPCPDCFLMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGMGPLJCKPC",
            |m: &HIEJJBDNCNH| { &m.FGMGPLJCKPC },
            |m: &mut HIEJJBDNCNH| { &mut m.FGMGPLJCKPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IHKGNJDNALJ::IHKGNJDNALJ>(
            "EBOOMGDGNEP",
            |m: &HIEJJBDNCNH| { &m.EBOOMGDGNEP },
            |m: &mut HIEJJBDNCNH| { &mut m.EBOOMGDGNEP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HIEJJBDNCNH>(
            "HIEJJBDNCNH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HIEJJBDNCNH {
    const NAME: &'static str = "HIEJJBDNCNH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JFPCPDCFLMD)?;
                },
                56 => {
                    self.FGMGPLJCKPC = is.read_enum_or_unknown()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EBOOMGDGNEP)?;
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
        if let Some(v) = self.JFPCPDCFLMD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.FGMGPLJCKPC != ::protobuf::EnumOrUnknown::new(super::BattleRecordType::BattleRecordType::BATTLE_RECORD_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.FGMGPLJCKPC.value());
        }
        if let Some(v) = self.EBOOMGDGNEP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.JFPCPDCFLMD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.FGMGPLJCKPC != ::protobuf::EnumOrUnknown::new(super::BattleRecordType::BattleRecordType::BATTLE_RECORD_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.FGMGPLJCKPC))?;
        }
        if let Some(v) = self.EBOOMGDGNEP.as_ref() {
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

    fn new() -> HIEJJBDNCNH {
        HIEJJBDNCNH::new()
    }

    fn clear(&mut self) {
        self.JFPCPDCFLMD.clear();
        self.FGMGPLJCKPC = ::protobuf::EnumOrUnknown::new(super::BattleRecordType::BattleRecordType::BATTLE_RECORD_NONE);
        self.EBOOMGDGNEP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HIEJJBDNCNH {
        static instance: HIEJJBDNCNH = HIEJJBDNCNH {
            JFPCPDCFLMD: ::protobuf::MessageField::none(),
            FGMGPLJCKPC: ::protobuf::EnumOrUnknown::from_i32(0),
            EBOOMGDGNEP: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HIEJJBDNCNH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HIEJJBDNCNH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HIEJJBDNCNH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HIEJJBDNCNH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HIEJJBDNCNH.proto\x1a\x16BattleRecordType.proto\x1a\x11IHKGNJDNALJ\
    .proto\x1a\x11OBIHNGMNKEK.proto\"\xa2\x01\n\x0bHIEJJBDNCNH\x12.\n\x0bJFP\
    CPDCFLMD\x18\x04\x20\x01(\x0b2\x0c.OBIHNGMNKEKR\x0bJFPCPDCFLMD\x123\n\
    \x0bFGMGPLJCKPC\x18\x07\x20\x01(\x0e2\x11.BattleRecordTypeR\x0bFGMGPLJCK\
    PC\x12.\n\x0bEBOOMGDGNEP\x18\n\x20\x01(\x0b2\x0c.IHKGNJDNALJR\x0bEBOOMGD\
    GNEPb\x06proto3\
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
            deps.push(super::BattleRecordType::file_descriptor().clone());
            deps.push(super::IHKGNJDNALJ::file_descriptor().clone());
            deps.push(super::OBIHNGMNKEK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HIEJJBDNCNH::generated_message_descriptor_data());
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
