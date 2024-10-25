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

//! Generated file from `ChangeScriptEmotionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChangeScriptEmotionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChangeScriptEmotionScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChangeScriptEmotionScRsp.FDDOKIIMPBF)
    pub FDDOKIIMPBF: ::protobuf::EnumOrUnknown<super::FOHOHPLGEFJ::FOHOHPLGEFJ>,
    // @@protoc_insertion_point(field:ChangeScriptEmotionScRsp.LKGNGHCPJAG)
    pub LKGNGHCPJAG: u32,
    // @@protoc_insertion_point(field:ChangeScriptEmotionScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChangeScriptEmotionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChangeScriptEmotionScRsp {
    fn default() -> &'a ChangeScriptEmotionScRsp {
        <ChangeScriptEmotionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChangeScriptEmotionScRsp {
    pub fn new() -> ChangeScriptEmotionScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDDOKIIMPBF",
            |m: &ChangeScriptEmotionScRsp| { &m.FDDOKIIMPBF },
            |m: &mut ChangeScriptEmotionScRsp| { &mut m.FDDOKIIMPBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKGNGHCPJAG",
            |m: &ChangeScriptEmotionScRsp| { &m.LKGNGHCPJAG },
            |m: &mut ChangeScriptEmotionScRsp| { &mut m.LKGNGHCPJAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChangeScriptEmotionScRsp| { &m.retcode },
            |m: &mut ChangeScriptEmotionScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChangeScriptEmotionScRsp>(
            "ChangeScriptEmotionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChangeScriptEmotionScRsp {
    const NAME: &'static str = "ChangeScriptEmotionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.FDDOKIIMPBF = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.LKGNGHCPJAG = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
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
        if self.FDDOKIIMPBF != ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE) {
            my_size += ::protobuf::rt::int32_size(14, self.FDDOKIIMPBF.value());
        }
        if self.LKGNGHCPJAG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LKGNGHCPJAG);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FDDOKIIMPBF != ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.FDDOKIIMPBF))?;
        }
        if self.LKGNGHCPJAG != 0 {
            os.write_uint32(7, self.LKGNGHCPJAG)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
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

    fn new() -> ChangeScriptEmotionScRsp {
        ChangeScriptEmotionScRsp::new()
    }

    fn clear(&mut self) {
        self.FDDOKIIMPBF = ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE);
        self.LKGNGHCPJAG = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChangeScriptEmotionScRsp {
        static instance: ChangeScriptEmotionScRsp = ChangeScriptEmotionScRsp {
            FDDOKIIMPBF: ::protobuf::EnumOrUnknown::from_i32(0),
            LKGNGHCPJAG: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChangeScriptEmotionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChangeScriptEmotionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChangeScriptEmotionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeScriptEmotionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eChangeScriptEmotionScRsp.proto\x1a\x11FOHOHPLGEFJ.proto\"\x86\x01\
    \n\x18ChangeScriptEmotionScRsp\x12.\n\x0bFDDOKIIMPBF\x18\x0e\x20\x01(\
    \x0e2\x0c.FOHOHPLGEFJR\x0bFDDOKIIMPBF\x12\x20\n\x0bLKGNGHCPJAG\x18\x07\
    \x20\x01(\rR\x0bLKGNGHCPJAG\x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\x07\
    retcodeb\x06proto3\
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
            deps.push(super::FOHOHPLGEFJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChangeScriptEmotionScRsp::generated_message_descriptor_data());
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
