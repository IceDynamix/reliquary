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

//! Generated file from `CDFOIPHMOJP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CDFOIPHMOJP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDFOIPHMOJP {
    // message fields
    // @@protoc_insertion_point(field:CDFOIPHMOJP.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:CDFOIPHMOJP.APOHJEGEEND)
    pub APOHJEGEEND: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // special fields
    // @@protoc_insertion_point(special_field:CDFOIPHMOJP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDFOIPHMOJP {
    fn default() -> &'a CDFOIPHMOJP {
        <CDFOIPHMOJP as ::protobuf::Message>::default_instance()
    }
}

impl CDFOIPHMOJP {
    pub fn new() -> CDFOIPHMOJP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &CDFOIPHMOJP| { &m.avatar_id },
            |m: &mut CDFOIPHMOJP| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APOHJEGEEND",
            |m: &CDFOIPHMOJP| { &m.APOHJEGEEND },
            |m: &mut CDFOIPHMOJP| { &mut m.APOHJEGEEND },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDFOIPHMOJP>(
            "CDFOIPHMOJP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDFOIPHMOJP {
    const NAME: &'static str = "CDFOIPHMOJP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.avatar_id = is.read_uint32()?;
                },
                104 => {
                    self.APOHJEGEEND = is.read_enum_or_unknown()?;
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
            my_size += ::protobuf::rt::uint32_size(6, self.avatar_id);
        }
        if self.APOHJEGEEND != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.APOHJEGEEND.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(6, self.avatar_id)?;
        }
        if self.APOHJEGEEND != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.APOHJEGEEND))?;
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

    fn new() -> CDFOIPHMOJP {
        CDFOIPHMOJP::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.APOHJEGEEND = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDFOIPHMOJP {
        static instance: CDFOIPHMOJP = CDFOIPHMOJP {
            avatar_id: 0,
            APOHJEGEEND: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDFOIPHMOJP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDFOIPHMOJP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDFOIPHMOJP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDFOIPHMOJP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CDFOIPHMOJP.proto\x1a\x10AvatarType.proto\"Y\n\x0bCDFOIPHMOJP\x12\
    \x1b\n\tavatar_id\x18\x06\x20\x01(\rR\x08avatarId\x12-\n\x0bAPOHJEGEEND\
    \x18\r\x20\x01(\x0e2\x0b.AvatarTypeR\x0bAPOHJEGEENDb\x06proto3\
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
            messages.push(CDFOIPHMOJP::generated_message_descriptor_data());
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
