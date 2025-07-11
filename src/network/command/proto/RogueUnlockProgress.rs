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

//! Generated file from `RogueUnlockProgress.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueUnlockProgress)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueUnlockProgress {
    // message fields
    // @@protoc_insertion_point(field:RogueUnlockProgress.progress)
    pub progress: u32,
    // @@protoc_insertion_point(field:RogueUnlockProgress.unlock_id)
    pub unlock_id: u32,
    // @@protoc_insertion_point(field:RogueUnlockProgress.finish)
    pub finish: bool,
    // special fields
    // @@protoc_insertion_point(special_field:RogueUnlockProgress.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueUnlockProgress {
    fn default() -> &'a RogueUnlockProgress {
        <RogueUnlockProgress as ::protobuf::Message>::default_instance()
    }
}

impl RogueUnlockProgress {
    pub fn new() -> RogueUnlockProgress {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "progress",
            |m: &RogueUnlockProgress| { &m.progress },
            |m: &mut RogueUnlockProgress| { &mut m.progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unlock_id",
            |m: &RogueUnlockProgress| { &m.unlock_id },
            |m: &mut RogueUnlockProgress| { &mut m.unlock_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finish",
            |m: &RogueUnlockProgress| { &m.finish },
            |m: &mut RogueUnlockProgress| { &mut m.finish },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueUnlockProgress>(
            "RogueUnlockProgress",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueUnlockProgress {
    const NAME: &'static str = "RogueUnlockProgress";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.progress = is.read_uint32()?;
                },
                24 => {
                    self.unlock_id = is.read_uint32()?;
                },
                56 => {
                    self.finish = is.read_bool()?;
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
        if self.progress != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.progress);
        }
        if self.unlock_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.unlock_id);
        }
        if self.finish != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.progress != 0 {
            os.write_uint32(12, self.progress)?;
        }
        if self.unlock_id != 0 {
            os.write_uint32(3, self.unlock_id)?;
        }
        if self.finish != false {
            os.write_bool(7, self.finish)?;
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

    fn new() -> RogueUnlockProgress {
        RogueUnlockProgress::new()
    }

    fn clear(&mut self) {
        self.progress = 0;
        self.unlock_id = 0;
        self.finish = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueUnlockProgress {
        static instance: RogueUnlockProgress = RogueUnlockProgress {
            progress: 0,
            unlock_id: 0,
            finish: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueUnlockProgress {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueUnlockProgress").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueUnlockProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueUnlockProgress {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19RogueUnlockProgress.proto\"f\n\x13RogueUnlockProgress\x12\x1a\n\
    \x08progress\x18\x0c\x20\x01(\rR\x08progress\x12\x1b\n\tunlock_id\x18\
    \x03\x20\x01(\rR\x08unlockId\x12\x16\n\x06finish\x18\x07\x20\x01(\x08R\
    \x06finishb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueUnlockProgress::generated_message_descriptor_data());
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
