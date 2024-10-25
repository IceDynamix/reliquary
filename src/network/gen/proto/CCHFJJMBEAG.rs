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

//! Generated file from `CCHFJJMBEAG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CCHFJJMBEAG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCHFJJMBEAG {
    // message fields
    // @@protoc_insertion_point(field:CCHFJJMBEAG.id)
    pub id: u32,
    // @@protoc_insertion_point(field:CCHFJJMBEAG.OIGPKLBIBAC)
    pub OIGPKLBIBAC: ::std::vec::Vec<super::AMMOKDCDKOO::AMMOKDCDKOO>,
    // @@protoc_insertion_point(field:CCHFJJMBEAG.status)
    pub status: ::protobuf::EnumOrUnknown<super::MissionStatus::MissionStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:CCHFJJMBEAG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCHFJJMBEAG {
    fn default() -> &'a CCHFJJMBEAG {
        <CCHFJJMBEAG as ::protobuf::Message>::default_instance()
    }
}

impl CCHFJJMBEAG {
    pub fn new() -> CCHFJJMBEAG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &CCHFJJMBEAG| { &m.id },
            |m: &mut CCHFJJMBEAG| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OIGPKLBIBAC",
            |m: &CCHFJJMBEAG| { &m.OIGPKLBIBAC },
            |m: &mut CCHFJJMBEAG| { &mut m.OIGPKLBIBAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &CCHFJJMBEAG| { &m.status },
            |m: &mut CCHFJJMBEAG| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CCHFJJMBEAG>(
            "CCHFJJMBEAG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CCHFJJMBEAG {
    const NAME: &'static str = "CCHFJJMBEAG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.id = is.read_uint32()?;
                },
                10 => {
                    self.OIGPKLBIBAC.push(is.read_message()?);
                },
                120 => {
                    self.status = is.read_enum_or_unknown()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.id);
        }
        for value in &self.OIGPKLBIBAC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::MissionStatus::MissionStatus::MISSION_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(14, self.id)?;
        }
        for v in &self.OIGPKLBIBAC {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::MissionStatus::MissionStatus::MISSION_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.status))?;
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

    fn new() -> CCHFJJMBEAG {
        CCHFJJMBEAG::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.OIGPKLBIBAC.clear();
        self.status = ::protobuf::EnumOrUnknown::new(super::MissionStatus::MissionStatus::MISSION_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCHFJJMBEAG {
        static instance: CCHFJJMBEAG = CCHFJJMBEAG {
            id: 0,
            OIGPKLBIBAC: ::std::vec::Vec::new(),
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CCHFJJMBEAG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CCHFJJMBEAG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CCHFJJMBEAG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCHFJJMBEAG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CCHFJJMBEAG.proto\x1a\x11AMMOKDCDKOO.proto\x1a\x13MissionStatus.pr\
    oto\"u\n\x0bCCHFJJMBEAG\x12\x0e\n\x02id\x18\x0e\x20\x01(\rR\x02id\x12.\n\
    \x0bOIGPKLBIBAC\x18\x01\x20\x03(\x0b2\x0c.AMMOKDCDKOOR\x0bOIGPKLBIBAC\
    \x12&\n\x06status\x18\x0f\x20\x01(\x0e2\x0e.MissionStatusR\x06statusb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::AMMOKDCDKOO::file_descriptor().clone());
            deps.push(super::MissionStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CCHFJJMBEAG::generated_message_descriptor_data());
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
