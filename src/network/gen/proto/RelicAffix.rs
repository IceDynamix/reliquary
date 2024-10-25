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

//! Generated file from `RelicAffix.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RelicAffix)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RelicAffix {
    // message fields
    // @@protoc_insertion_point(field:RelicAffix.affix_id)
    pub affix_id: u32,
    // @@protoc_insertion_point(field:RelicAffix.cnt)
    pub cnt: u32,
    // @@protoc_insertion_point(field:RelicAffix.step)
    pub step: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RelicAffix.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RelicAffix {
    fn default() -> &'a RelicAffix {
        <RelicAffix as ::protobuf::Message>::default_instance()
    }
}

impl RelicAffix {
    pub fn new() -> RelicAffix {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "affix_id",
            |m: &RelicAffix| { &m.affix_id },
            |m: &mut RelicAffix| { &mut m.affix_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cnt",
            |m: &RelicAffix| { &m.cnt },
            |m: &mut RelicAffix| { &mut m.cnt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &RelicAffix| { &m.step },
            |m: &mut RelicAffix| { &mut m.step },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RelicAffix>(
            "RelicAffix",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RelicAffix {
    const NAME: &'static str = "RelicAffix";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.affix_id = is.read_uint32()?;
                },
                16 => {
                    self.cnt = is.read_uint32()?;
                },
                24 => {
                    self.step = is.read_uint32()?;
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
        if self.affix_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.affix_id);
        }
        if self.cnt != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.cnt);
        }
        if self.step != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.step);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.affix_id != 0 {
            os.write_uint32(1, self.affix_id)?;
        }
        if self.cnt != 0 {
            os.write_uint32(2, self.cnt)?;
        }
        if self.step != 0 {
            os.write_uint32(3, self.step)?;
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

    fn new() -> RelicAffix {
        RelicAffix::new()
    }

    fn clear(&mut self) {
        self.affix_id = 0;
        self.cnt = 0;
        self.step = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RelicAffix {
        static instance: RelicAffix = RelicAffix {
            affix_id: 0,
            cnt: 0,
            step: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RelicAffix {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RelicAffix").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RelicAffix {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RelicAffix {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10RelicAffix.proto\"M\n\nRelicAffix\x12\x19\n\x08affix_id\x18\x01\
    \x20\x01(\rR\x07affixId\x12\x10\n\x03cnt\x18\x02\x20\x01(\rR\x03cnt\x12\
    \x12\n\x04step\x18\x03\x20\x01(\rR\x04stepb\x06proto3\
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
            messages.push(RelicAffix::generated_message_descriptor_data());
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
