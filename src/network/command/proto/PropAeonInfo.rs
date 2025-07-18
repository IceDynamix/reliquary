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

//! Generated file from `PropAeonInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PropAeonInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PropAeonInfo {
    // message fields
    // @@protoc_insertion_point(field:PropAeonInfo.add_exp)
    pub add_exp: u32,
    // @@protoc_insertion_point(field:PropAeonInfo.dialogue_group_id)
    pub dialogue_group_id: u32,
    // @@protoc_insertion_point(field:PropAeonInfo.aeon_id)
    pub aeon_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PropAeonInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PropAeonInfo {
    fn default() -> &'a PropAeonInfo {
        <PropAeonInfo as ::protobuf::Message>::default_instance()
    }
}

impl PropAeonInfo {
    pub fn new() -> PropAeonInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "add_exp",
            |m: &PropAeonInfo| { &m.add_exp },
            |m: &mut PropAeonInfo| { &mut m.add_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dialogue_group_id",
            |m: &PropAeonInfo| { &m.dialogue_group_id },
            |m: &mut PropAeonInfo| { &mut m.dialogue_group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "aeon_id",
            |m: &PropAeonInfo| { &m.aeon_id },
            |m: &mut PropAeonInfo| { &mut m.aeon_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PropAeonInfo>(
            "PropAeonInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PropAeonInfo {
    const NAME: &'static str = "PropAeonInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.add_exp = is.read_uint32()?;
                },
                96 => {
                    self.dialogue_group_id = is.read_uint32()?;
                },
                16 => {
                    self.aeon_id = is.read_uint32()?;
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
        if self.add_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.add_exp);
        }
        if self.dialogue_group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.dialogue_group_id);
        }
        if self.aeon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.aeon_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.add_exp != 0 {
            os.write_uint32(8, self.add_exp)?;
        }
        if self.dialogue_group_id != 0 {
            os.write_uint32(12, self.dialogue_group_id)?;
        }
        if self.aeon_id != 0 {
            os.write_uint32(2, self.aeon_id)?;
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

    fn new() -> PropAeonInfo {
        PropAeonInfo::new()
    }

    fn clear(&mut self) {
        self.add_exp = 0;
        self.dialogue_group_id = 0;
        self.aeon_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PropAeonInfo {
        static instance: PropAeonInfo = PropAeonInfo {
            add_exp: 0,
            dialogue_group_id: 0,
            aeon_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PropAeonInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PropAeonInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PropAeonInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropAeonInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12PropAeonInfo.proto\"l\n\x0cPropAeonInfo\x12\x17\n\x07add_exp\x18\
    \x08\x20\x01(\rR\x06addExp\x12*\n\x11dialogue_group_id\x18\x0c\x20\x01(\
    \rR\x0fdialogueGroupId\x12\x17\n\x07aeon_id\x18\x02\x20\x01(\rR\x06aeonI\
    db\x06proto3\
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
            messages.push(PropAeonInfo::generated_message_descriptor_data());
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
