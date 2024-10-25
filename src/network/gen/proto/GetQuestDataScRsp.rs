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

//! Generated file from `GetQuestDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetQuestDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetQuestDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetQuestDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetQuestDataScRsp.total_achievement_exp)
    pub total_achievement_exp: u32,
    // @@protoc_insertion_point(field:GetQuestDataScRsp.quest_list)
    pub quest_list: ::std::vec::Vec<super::Quest::Quest>,
    // special fields
    // @@protoc_insertion_point(special_field:GetQuestDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetQuestDataScRsp {
    fn default() -> &'a GetQuestDataScRsp {
        <GetQuestDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetQuestDataScRsp {
    pub fn new() -> GetQuestDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetQuestDataScRsp| { &m.retcode },
            |m: &mut GetQuestDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_achievement_exp",
            |m: &GetQuestDataScRsp| { &m.total_achievement_exp },
            |m: &mut GetQuestDataScRsp| { &mut m.total_achievement_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "quest_list",
            |m: &GetQuestDataScRsp| { &m.quest_list },
            |m: &mut GetQuestDataScRsp| { &mut m.quest_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetQuestDataScRsp>(
            "GetQuestDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetQuestDataScRsp {
    const NAME: &'static str = "GetQuestDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.retcode = is.read_uint32()?;
                },
                56 => {
                    self.total_achievement_exp = is.read_uint32()?;
                },
                106 => {
                    self.quest_list.push(is.read_message()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        if self.total_achievement_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.total_achievement_exp);
        }
        for value in &self.quest_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
        }
        if self.total_achievement_exp != 0 {
            os.write_uint32(7, self.total_achievement_exp)?;
        }
        for v in &self.quest_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> GetQuestDataScRsp {
        GetQuestDataScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.total_achievement_exp = 0;
        self.quest_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetQuestDataScRsp {
        static instance: GetQuestDataScRsp = GetQuestDataScRsp {
            retcode: 0,
            total_achievement_exp: 0,
            quest_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetQuestDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetQuestDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetQuestDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetQuestDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetQuestDataScRsp.proto\x1a\x0bQuest.proto\"\x88\x01\n\x11GetQuest\
    DataScRsp\x12\x18\n\x07retcode\x18\x0f\x20\x01(\rR\x07retcode\x122\n\x15\
    total_achievement_exp\x18\x07\x20\x01(\rR\x13totalAchievementExp\x12%\n\
    \nquest_list\x18\r\x20\x03(\x0b2\x06.QuestR\tquestListb\x06proto3\
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
            deps.push(super::Quest::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetQuestDataScRsp::generated_message_descriptor_data());
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
