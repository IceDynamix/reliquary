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

//! Generated file from `IJKJJDHLKLB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IJKJJDHLKLB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IJKJJDHLKLB {
    // message fields
    // @@protoc_insertion_point(field:IJKJJDHLKLB.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:IJKJJDHLKLB.IHLEAMDIKKN)
    pub IHLEAMDIKKN: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // special fields
    // @@protoc_insertion_point(special_field:IJKJJDHLKLB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IJKJJDHLKLB {
    fn default() -> &'a IJKJJDHLKLB {
        <IJKJJDHLKLB as ::protobuf::Message>::default_instance()
    }
}

impl IJKJJDHLKLB {
    pub fn new() -> IJKJJDHLKLB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &IJKJJDHLKLB| { &m.avatar_id },
            |m: &mut IJKJJDHLKLB| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHLEAMDIKKN",
            |m: &IJKJJDHLKLB| { &m.IHLEAMDIKKN },
            |m: &mut IJKJJDHLKLB| { &mut m.IHLEAMDIKKN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IJKJJDHLKLB>(
            "IJKJJDHLKLB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IJKJJDHLKLB {
    const NAME: &'static str = "IJKJJDHLKLB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.avatar_id = is.read_uint32()?;
                },
                80 => {
                    self.IHLEAMDIKKN = is.read_enum_or_unknown()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.avatar_id);
        }
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.IHLEAMDIKKN.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(13, self.avatar_id)?;
        }
        if self.IHLEAMDIKKN != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.IHLEAMDIKKN))?;
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

    fn new() -> IJKJJDHLKLB {
        IJKJJDHLKLB::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.IHLEAMDIKKN = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IJKJJDHLKLB {
        static instance: IJKJJDHLKLB = IJKJJDHLKLB {
            avatar_id: 0,
            IHLEAMDIKKN: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IJKJJDHLKLB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IJKJJDHLKLB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IJKJJDHLKLB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IJKJJDHLKLB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IJKJJDHLKLB.proto\x1a\x10AvatarType.proto\"Y\n\x0bIJKJJDHLKLB\x12\
    \x1b\n\tavatar_id\x18\r\x20\x01(\rR\x08avatarId\x12-\n\x0bIHLEAMDIKKN\
    \x18\n\x20\x01(\x0e2\x0b.AvatarTypeR\x0bIHLEAMDIKKNb\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IJKJJDHLKLB::generated_message_descriptor_data());
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
