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

//! Generated file from `SetTurnFoodSwitchCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SetTurnFoodSwitchCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetTurnFoodSwitchCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetTurnFoodSwitchCsReq.JCAKHHKFDFN)
    pub JCAKHHKFDFN: ::protobuf::EnumOrUnknown<super::TurnFoodSwitch::TurnFoodSwitch>,
    // @@protoc_insertion_point(field:SetTurnFoodSwitchCsReq.BNDLHJHALMB)
    pub BNDLHJHALMB: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SetTurnFoodSwitchCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetTurnFoodSwitchCsReq {
    fn default() -> &'a SetTurnFoodSwitchCsReq {
        <SetTurnFoodSwitchCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetTurnFoodSwitchCsReq {
    pub fn new() -> SetTurnFoodSwitchCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCAKHHKFDFN",
            |m: &SetTurnFoodSwitchCsReq| { &m.JCAKHHKFDFN },
            |m: &mut SetTurnFoodSwitchCsReq| { &mut m.JCAKHHKFDFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNDLHJHALMB",
            |m: &SetTurnFoodSwitchCsReq| { &m.BNDLHJHALMB },
            |m: &mut SetTurnFoodSwitchCsReq| { &mut m.BNDLHJHALMB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetTurnFoodSwitchCsReq>(
            "SetTurnFoodSwitchCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetTurnFoodSwitchCsReq {
    const NAME: &'static str = "SetTurnFoodSwitchCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.JCAKHHKFDFN = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.BNDLHJHALMB = is.read_bool()?;
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
        if self.JCAKHHKFDFN != ::protobuf::EnumOrUnknown::new(super::TurnFoodSwitch::TurnFoodSwitch::TURN_FOOD_SWITCH_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.JCAKHHKFDFN.value());
        }
        if self.BNDLHJHALMB != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JCAKHHKFDFN != ::protobuf::EnumOrUnknown::new(super::TurnFoodSwitch::TurnFoodSwitch::TURN_FOOD_SWITCH_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.JCAKHHKFDFN))?;
        }
        if self.BNDLHJHALMB != false {
            os.write_bool(10, self.BNDLHJHALMB)?;
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

    fn new() -> SetTurnFoodSwitchCsReq {
        SetTurnFoodSwitchCsReq::new()
    }

    fn clear(&mut self) {
        self.JCAKHHKFDFN = ::protobuf::EnumOrUnknown::new(super::TurnFoodSwitch::TurnFoodSwitch::TURN_FOOD_SWITCH_NONE);
        self.BNDLHJHALMB = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetTurnFoodSwitchCsReq {
        static instance: SetTurnFoodSwitchCsReq = SetTurnFoodSwitchCsReq {
            JCAKHHKFDFN: ::protobuf::EnumOrUnknown::from_i32(0),
            BNDLHJHALMB: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetTurnFoodSwitchCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetTurnFoodSwitchCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetTurnFoodSwitchCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetTurnFoodSwitchCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cSetTurnFoodSwitchCsReq.proto\x1a\x14TurnFoodSwitch.proto\"m\n\x16S\
    etTurnFoodSwitchCsReq\x121\n\x0bJCAKHHKFDFN\x18\x07\x20\x01(\x0e2\x0f.Tu\
    rnFoodSwitchR\x0bJCAKHHKFDFN\x12\x20\n\x0bBNDLHJHALMB\x18\n\x20\x01(\x08\
    R\x0bBNDLHJHALMBb\x06proto3\
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
            deps.push(super::TurnFoodSwitch::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetTurnFoodSwitchCsReq::generated_message_descriptor_data());
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
