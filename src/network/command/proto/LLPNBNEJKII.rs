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

//! Generated file from `LLPNBNEJKII.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LLPNBNEJKII)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LLPNBNEJKII {
    // message fields
    // @@protoc_insertion_point(field:LLPNBNEJKII.lineup)
    pub lineup: ::protobuf::MessageField<super::LineupInfo::LineupInfo>,
    // @@protoc_insertion_point(field:LLPNBNEJKII.KEILFHHNLIH)
    pub KEILFHHNLIH: ::protobuf::MessageField<super::RogueMapRotateInfo::RogueMapRotateInfo>,
    // @@protoc_insertion_point(field:LLPNBNEJKII.scene)
    pub scene: ::protobuf::MessageField<super::SceneInfo::SceneInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:LLPNBNEJKII.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LLPNBNEJKII {
    fn default() -> &'a LLPNBNEJKII {
        <LLPNBNEJKII as ::protobuf::Message>::default_instance()
    }
}

impl LLPNBNEJKII {
    pub fn new() -> LLPNBNEJKII {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LineupInfo::LineupInfo>(
            "lineup",
            |m: &LLPNBNEJKII| { &m.lineup },
            |m: &mut LLPNBNEJKII| { &mut m.lineup },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueMapRotateInfo::RogueMapRotateInfo>(
            "KEILFHHNLIH",
            |m: &LLPNBNEJKII| { &m.KEILFHHNLIH },
            |m: &mut LLPNBNEJKII| { &mut m.KEILFHHNLIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneInfo::SceneInfo>(
            "scene",
            |m: &LLPNBNEJKII| { &m.scene },
            |m: &mut LLPNBNEJKII| { &mut m.scene },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LLPNBNEJKII>(
            "LLPNBNEJKII",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LLPNBNEJKII {
    const NAME: &'static str = "LLPNBNEJKII";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.lineup)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KEILFHHNLIH)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scene)?;
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
        if let Some(v) = self.lineup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KEILFHHNLIH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.scene.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.lineup.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.KEILFHHNLIH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.scene.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> LLPNBNEJKII {
        LLPNBNEJKII::new()
    }

    fn clear(&mut self) {
        self.lineup.clear();
        self.KEILFHHNLIH.clear();
        self.scene.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LLPNBNEJKII {
        static instance: LLPNBNEJKII = LLPNBNEJKII {
            lineup: ::protobuf::MessageField::none(),
            KEILFHHNLIH: ::protobuf::MessageField::none(),
            scene: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LLPNBNEJKII {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LLPNBNEJKII").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LLPNBNEJKII {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LLPNBNEJKII {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LLPNBNEJKII.proto\x1a\x10LineupInfo.proto\x1a\x18RogueMapRotateInf\
    o.proto\x1a\x0fSceneInfo.proto\"\x8b\x01\n\x0bLLPNBNEJKII\x12#\n\x06line\
    up\x18\r\x20\x01(\x0b2\x0b.LineupInfoR\x06lineup\x125\n\x0bKEILFHHNLIH\
    \x18\x05\x20\x01(\x0b2\x13.RogueMapRotateInfoR\x0bKEILFHHNLIH\x12\x20\n\
    \x05scene\x18\x0e\x20\x01(\x0b2\n.SceneInfoR\x05sceneb\x06proto3\
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
            deps.push(super::LineupInfo::file_descriptor().clone());
            deps.push(super::RogueMapRotateInfo::file_descriptor().clone());
            deps.push(super::SceneInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LLPNBNEJKII::generated_message_descriptor_data());
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
