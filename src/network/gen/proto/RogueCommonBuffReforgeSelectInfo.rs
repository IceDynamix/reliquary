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

//! Generated file from `RogueCommonBuffReforgeSelectInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueCommonBuffReforgeSelectInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueCommonBuffReforgeSelectInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueCommonBuffReforgeSelectInfo.select_hint_id)
    pub select_hint_id: u32,
    // @@protoc_insertion_point(field:RogueCommonBuffReforgeSelectInfo.select_buffs)
    pub select_buffs: ::std::vec::Vec<super::RogueCommonBuff::RogueCommonBuff>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueCommonBuffReforgeSelectInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueCommonBuffReforgeSelectInfo {
    fn default() -> &'a RogueCommonBuffReforgeSelectInfo {
        <RogueCommonBuffReforgeSelectInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueCommonBuffReforgeSelectInfo {
    pub fn new() -> RogueCommonBuffReforgeSelectInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "select_hint_id",
            |m: &RogueCommonBuffReforgeSelectInfo| { &m.select_hint_id },
            |m: &mut RogueCommonBuffReforgeSelectInfo| { &mut m.select_hint_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "select_buffs",
            |m: &RogueCommonBuffReforgeSelectInfo| { &m.select_buffs },
            |m: &mut RogueCommonBuffReforgeSelectInfo| { &mut m.select_buffs },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueCommonBuffReforgeSelectInfo>(
            "RogueCommonBuffReforgeSelectInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueCommonBuffReforgeSelectInfo {
    const NAME: &'static str = "RogueCommonBuffReforgeSelectInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.select_hint_id = is.read_uint32()?;
                },
                10 => {
                    self.select_buffs.push(is.read_message()?);
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
        if self.select_hint_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.select_hint_id);
        }
        for value in &self.select_buffs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.select_hint_id != 0 {
            os.write_uint32(12, self.select_hint_id)?;
        }
        for v in &self.select_buffs {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RogueCommonBuffReforgeSelectInfo {
        RogueCommonBuffReforgeSelectInfo::new()
    }

    fn clear(&mut self) {
        self.select_hint_id = 0;
        self.select_buffs.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueCommonBuffReforgeSelectInfo {
        static instance: RogueCommonBuffReforgeSelectInfo = RogueCommonBuffReforgeSelectInfo {
            select_hint_id: 0,
            select_buffs: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueCommonBuffReforgeSelectInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueCommonBuffReforgeSelectInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueCommonBuffReforgeSelectInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueCommonBuffReforgeSelectInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&RogueCommonBuffReforgeSelectInfo.proto\x1a\x15RogueCommonBuff.proto\"\
    }\n\x20RogueCommonBuffReforgeSelectInfo\x12$\n\x0eselect_hint_id\x18\x0c\
    \x20\x01(\rR\x0cselectHintId\x123\n\x0cselect_buffs\x18\x01\x20\x03(\x0b\
    2\x10.RogueCommonBuffR\x0bselectBuffsB\x15\n\x13emu.lunarcore.protob\x06\
    proto3\
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
            deps.push(super::RogueCommonBuff::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueCommonBuffReforgeSelectInfo::generated_message_descriptor_data());
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