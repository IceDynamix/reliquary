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

//! Generated file from `JoinLineupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:JoinLineupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JoinLineupCsReq {
    // message fields
    // @@protoc_insertion_point(field:JoinLineupCsReq.index)
    pub index: u32,
    // @@protoc_insertion_point(field:JoinLineupCsReq.avatar_type)
    pub avatar_type: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:JoinLineupCsReq.slot)
    pub slot: u32,
    // @@protoc_insertion_point(field:JoinLineupCsReq.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:JoinLineupCsReq.plane_id)
    pub plane_id: u32,
    // @@protoc_insertion_point(field:JoinLineupCsReq.is_virtual)
    pub is_virtual: bool,
    // @@protoc_insertion_point(field:JoinLineupCsReq.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // special fields
    // @@protoc_insertion_point(special_field:JoinLineupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JoinLineupCsReq {
    fn default() -> &'a JoinLineupCsReq {
        <JoinLineupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl JoinLineupCsReq {
    pub fn new() -> JoinLineupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &JoinLineupCsReq| { &m.index },
            |m: &mut JoinLineupCsReq| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_type",
            |m: &JoinLineupCsReq| { &m.avatar_type },
            |m: &mut JoinLineupCsReq| { &mut m.avatar_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &JoinLineupCsReq| { &m.slot },
            |m: &mut JoinLineupCsReq| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &JoinLineupCsReq| { &m.base_avatar_id },
            |m: &mut JoinLineupCsReq| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &JoinLineupCsReq| { &m.plane_id },
            |m: &mut JoinLineupCsReq| { &mut m.plane_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_virtual",
            |m: &JoinLineupCsReq| { &m.is_virtual },
            |m: &mut JoinLineupCsReq| { &mut m.is_virtual },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &JoinLineupCsReq| { &m.extra_lineup_type },
            |m: &mut JoinLineupCsReq| { &mut m.extra_lineup_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JoinLineupCsReq>(
            "JoinLineupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JoinLineupCsReq {
    const NAME: &'static str = "JoinLineupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.index = is.read_uint32()?;
                },
                104 => {
                    self.avatar_type = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.slot = is.read_uint32()?;
                },
                40 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                48 => {
                    self.plane_id = is.read_uint32()?;
                },
                32 => {
                    self.is_virtual = is.read_bool()?;
                },
                120 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
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
        if self.index != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.index);
        }
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.avatar_type.value());
        }
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.slot);
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.base_avatar_id);
        }
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.plane_id);
        }
        if self.is_virtual != false {
            my_size += 1 + 1;
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.extra_lineup_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.index != 0 {
            os.write_uint32(7, self.index)?;
        }
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.avatar_type))?;
        }
        if self.slot != 0 {
            os.write_uint32(1, self.slot)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(5, self.base_avatar_id)?;
        }
        if self.plane_id != 0 {
            os.write_uint32(6, self.plane_id)?;
        }
        if self.is_virtual != false {
            os.write_bool(4, self.is_virtual)?;
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
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

    fn new() -> JoinLineupCsReq {
        JoinLineupCsReq::new()
    }

    fn clear(&mut self) {
        self.index = 0;
        self.avatar_type = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.slot = 0;
        self.base_avatar_id = 0;
        self.plane_id = 0;
        self.is_virtual = false;
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JoinLineupCsReq {
        static instance: JoinLineupCsReq = JoinLineupCsReq {
            index: 0,
            avatar_type: ::protobuf::EnumOrUnknown::from_i32(0),
            slot: 0,
            base_avatar_id: 0,
            plane_id: 0,
            is_virtual: false,
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JoinLineupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JoinLineupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JoinLineupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JoinLineupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15JoinLineupCsReq.proto\x1a\x10AvatarType.proto\x1a\x15ExtraLineupTy\
    pe.proto\"\x87\x02\n\x0fJoinLineupCsReq\x12\x14\n\x05index\x18\x07\x20\
    \x01(\rR\x05index\x12,\n\x0bavatar_type\x18\r\x20\x01(\x0e2\x0b.AvatarTy\
    peR\navatarType\x12\x12\n\x04slot\x18\x01\x20\x01(\rR\x04slot\x12$\n\x0e\
    base_avatar_id\x18\x05\x20\x01(\rR\x0cbaseAvatarId\x12\x19\n\x08plane_id\
    \x18\x06\x20\x01(\rR\x07planeId\x12\x1d\n\nis_virtual\x18\x04\x20\x01(\
    \x08R\tisVirtual\x12<\n\x11extra_lineup_type\x18\x0f\x20\x01(\x0e2\x10.E\
    xtraLineupTypeR\x0fextraLineupTypeb\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JoinLineupCsReq::generated_message_descriptor_data());
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
