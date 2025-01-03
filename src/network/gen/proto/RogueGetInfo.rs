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

//! Generated file from `RogueGetInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueGetInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueGetInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueGetInfo.rogue_virtual_item_info)
    pub rogue_virtual_item_info: ::protobuf::MessageField<super::RogueGetVirtualItemInfo::RogueGetVirtualItemInfo>,
    // @@protoc_insertion_point(field:RogueGetInfo.rogue_season_info)
    pub rogue_season_info: ::protobuf::MessageField<super::RogueSeasonInfo::RogueSeasonInfo>,
    // @@protoc_insertion_point(field:RogueGetInfo.rogue_area_info)
    pub rogue_area_info: ::protobuf::MessageField<super::RogueAreaInfo::RogueAreaInfo>,
    // @@protoc_insertion_point(field:RogueGetInfo.rogue_aeon_info)
    pub rogue_aeon_info: ::protobuf::MessageField<super::RogueAeonInfo::RogueAeonInfo>,
    // @@protoc_insertion_point(field:RogueGetInfo.rogue_score_reward_info)
    pub rogue_score_reward_info: ::protobuf::MessageField<super::RogueScoreRewardInfo::RogueScoreRewardInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueGetInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueGetInfo {
    fn default() -> &'a RogueGetInfo {
        <RogueGetInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueGetInfo {
    pub fn new() -> RogueGetInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueGetVirtualItemInfo::RogueGetVirtualItemInfo>(
            "rogue_virtual_item_info",
            |m: &RogueGetInfo| { &m.rogue_virtual_item_info },
            |m: &mut RogueGetInfo| { &mut m.rogue_virtual_item_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueSeasonInfo::RogueSeasonInfo>(
            "rogue_season_info",
            |m: &RogueGetInfo| { &m.rogue_season_info },
            |m: &mut RogueGetInfo| { &mut m.rogue_season_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueAreaInfo::RogueAreaInfo>(
            "rogue_area_info",
            |m: &RogueGetInfo| { &m.rogue_area_info },
            |m: &mut RogueGetInfo| { &mut m.rogue_area_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueAeonInfo::RogueAeonInfo>(
            "rogue_aeon_info",
            |m: &RogueGetInfo| { &m.rogue_aeon_info },
            |m: &mut RogueGetInfo| { &mut m.rogue_aeon_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueScoreRewardInfo::RogueScoreRewardInfo>(
            "rogue_score_reward_info",
            |m: &RogueGetInfo| { &m.rogue_score_reward_info },
            |m: &mut RogueGetInfo| { &mut m.rogue_score_reward_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueGetInfo>(
            "RogueGetInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueGetInfo {
    const NAME: &'static str = "RogueGetInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_virtual_item_info)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_season_info)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_area_info)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_aeon_info)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_score_reward_info)?;
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
        if let Some(v) = self.rogue_virtual_item_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_season_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_area_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_aeon_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_score_reward_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.rogue_virtual_item_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.rogue_season_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.rogue_area_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.rogue_aeon_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.rogue_score_reward_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> RogueGetInfo {
        RogueGetInfo::new()
    }

    fn clear(&mut self) {
        self.rogue_virtual_item_info.clear();
        self.rogue_season_info.clear();
        self.rogue_area_info.clear();
        self.rogue_aeon_info.clear();
        self.rogue_score_reward_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueGetInfo {
        static instance: RogueGetInfo = RogueGetInfo {
            rogue_virtual_item_info: ::protobuf::MessageField::none(),
            rogue_season_info: ::protobuf::MessageField::none(),
            rogue_area_info: ::protobuf::MessageField::none(),
            rogue_aeon_info: ::protobuf::MessageField::none(),
            rogue_score_reward_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueGetInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueGetInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueGetInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueGetInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12RogueGetInfo.proto\x1a\x15RogueSeasonInfo.proto\x1a\x1dRogueGetVir\
    tualItemInfo.proto\x1a\x13RogueAreaInfo.proto\x1a\x13RogueAeonInfo.proto\
    \x1a\x1aRogueScoreRewardInfo.proto\"\xdb\x02\n\x0cRogueGetInfo\x12O\n\
    \x17rogue_virtual_item_info\x18\x0f\x20\x01(\x0b2\x18.RogueGetVirtualIte\
    mInfoR\x14rogueVirtualItemInfo\x12<\n\x11rogue_season_info\x18\t\x20\x01\
    (\x0b2\x10.RogueSeasonInfoR\x0frogueSeasonInfo\x126\n\x0frogue_area_info\
    \x18\x0b\x20\x01(\x0b2\x0e.RogueAreaInfoR\rrogueAreaInfo\x126\n\x0frogue\
    _aeon_info\x18\n\x20\x01(\x0b2\x0e.RogueAeonInfoR\rrogueAeonInfo\x12L\n\
    \x17rogue_score_reward_info\x18\x03\x20\x01(\x0b2\x15.RogueScoreRewardIn\
    foR\x14rogueScoreRewardInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::RogueSeasonInfo::file_descriptor().clone());
            deps.push(super::RogueGetVirtualItemInfo::file_descriptor().clone());
            deps.push(super::RogueAreaInfo::file_descriptor().clone());
            deps.push(super::RogueAeonInfo::file_descriptor().clone());
            deps.push(super::RogueScoreRewardInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueGetInfo::generated_message_descriptor_data());
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
