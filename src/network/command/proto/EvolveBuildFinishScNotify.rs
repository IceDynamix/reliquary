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

//! Generated file from `EvolveBuildFinishScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EvolveBuildFinishScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EvolveBuildFinishScNotify {
    // message fields
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.coin)
    pub coin: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.cur_period_type)
    pub cur_period_type: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.battle_result_type)
    pub battle_result_type: ::protobuf::EnumOrUnknown<super::DLHCMCNIHII::DLHCMCNIHII>,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.is_lose)
    pub is_lose: bool,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.wave)
    pub wave: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.score)
    pub score: u32,
    // @@protoc_insertion_point(field:EvolveBuildFinishScNotify.level_info)
    pub level_info: ::protobuf::MessageField<super::EvolveBuildLevelInfo::EvolveBuildLevelInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:EvolveBuildFinishScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EvolveBuildFinishScNotify {
    fn default() -> &'a EvolveBuildFinishScNotify {
        <EvolveBuildFinishScNotify as ::protobuf::Message>::default_instance()
    }
}

impl EvolveBuildFinishScNotify {
    pub fn new() -> EvolveBuildFinishScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "coin",
            |m: &EvolveBuildFinishScNotify| { &m.coin },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.coin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_period_type",
            |m: &EvolveBuildFinishScNotify| { &m.cur_period_type },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.cur_period_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &EvolveBuildFinishScNotify| { &m.exp },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_result_type",
            |m: &EvolveBuildFinishScNotify| { &m.battle_result_type },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.battle_result_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_lose",
            |m: &EvolveBuildFinishScNotify| { &m.is_lose },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.is_lose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &EvolveBuildFinishScNotify| { &m.level_id },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave",
            |m: &EvolveBuildFinishScNotify| { &m.wave },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.wave },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &EvolveBuildFinishScNotify| { &m.score },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EvolveBuildLevelInfo::EvolveBuildLevelInfo>(
            "level_info",
            |m: &EvolveBuildFinishScNotify| { &m.level_info },
            |m: &mut EvolveBuildFinishScNotify| { &mut m.level_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EvolveBuildFinishScNotify>(
            "EvolveBuildFinishScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EvolveBuildFinishScNotify {
    const NAME: &'static str = "EvolveBuildFinishScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.coin = is.read_uint32()?;
                },
                120 => {
                    self.cur_period_type = is.read_uint32()?;
                },
                40 => {
                    self.exp = is.read_uint32()?;
                },
                24 => {
                    self.battle_result_type = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.is_lose = is.read_bool()?;
                },
                72 => {
                    self.level_id = is.read_uint32()?;
                },
                32 => {
                    self.wave = is.read_uint32()?;
                },
                16 => {
                    self.score = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.level_info)?;
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
        if self.coin != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.coin);
        }
        if self.cur_period_type != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.cur_period_type);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.exp);
        }
        if self.battle_result_type != ::protobuf::EnumOrUnknown::new(super::DLHCMCNIHII::DLHCMCNIHII::EVOLVE_BATTLE_RESULT_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.battle_result_type.value());
        }
        if self.is_lose != false {
            my_size += 1 + 1;
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.level_id);
        }
        if self.wave != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.wave);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.score);
        }
        if let Some(v) = self.level_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.coin != 0 {
            os.write_uint32(6, self.coin)?;
        }
        if self.cur_period_type != 0 {
            os.write_uint32(15, self.cur_period_type)?;
        }
        if self.exp != 0 {
            os.write_uint32(5, self.exp)?;
        }
        if self.battle_result_type != ::protobuf::EnumOrUnknown::new(super::DLHCMCNIHII::DLHCMCNIHII::EVOLVE_BATTLE_RESULT_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.battle_result_type))?;
        }
        if self.is_lose != false {
            os.write_bool(7, self.is_lose)?;
        }
        if self.level_id != 0 {
            os.write_uint32(9, self.level_id)?;
        }
        if self.wave != 0 {
            os.write_uint32(4, self.wave)?;
        }
        if self.score != 0 {
            os.write_uint32(2, self.score)?;
        }
        if let Some(v) = self.level_info.as_ref() {
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

    fn new() -> EvolveBuildFinishScNotify {
        EvolveBuildFinishScNotify::new()
    }

    fn clear(&mut self) {
        self.coin = 0;
        self.cur_period_type = 0;
        self.exp = 0;
        self.battle_result_type = ::protobuf::EnumOrUnknown::new(super::DLHCMCNIHII::DLHCMCNIHII::EVOLVE_BATTLE_RESULT_NONE);
        self.is_lose = false;
        self.level_id = 0;
        self.wave = 0;
        self.score = 0;
        self.level_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EvolveBuildFinishScNotify {
        static instance: EvolveBuildFinishScNotify = EvolveBuildFinishScNotify {
            coin: 0,
            cur_period_type: 0,
            exp: 0,
            battle_result_type: ::protobuf::EnumOrUnknown::from_i32(0),
            is_lose: false,
            level_id: 0,
            wave: 0,
            score: 0,
            level_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EvolveBuildFinishScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EvolveBuildFinishScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EvolveBuildFinishScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvolveBuildFinishScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fEvolveBuildFinishScNotify.proto\x1a\x11DLHCMCNIHII.proto\x1a\x1aEv\
    olveBuildLevelInfo.proto\"\xb9\x02\n\x19EvolveBuildFinishScNotify\x12\
    \x12\n\x04coin\x18\x06\x20\x01(\rR\x04coin\x12&\n\x0fcur_period_type\x18\
    \x0f\x20\x01(\rR\rcurPeriodType\x12\x10\n\x03exp\x18\x05\x20\x01(\rR\x03\
    exp\x12:\n\x12battle_result_type\x18\x03\x20\x01(\x0e2\x0c.DLHCMCNIHIIR\
    \x10battleResultType\x12\x17\n\x07is_lose\x18\x07\x20\x01(\x08R\x06isLos\
    e\x12\x19\n\x08level_id\x18\t\x20\x01(\rR\x07levelId\x12\x12\n\x04wave\
    \x18\x04\x20\x01(\rR\x04wave\x12\x14\n\x05score\x18\x02\x20\x01(\rR\x05s\
    core\x124\n\nlevel_info\x18\x0e\x20\x01(\x0b2\x15.EvolveBuildLevelInfoR\
    \tlevelInfob\x06proto3\
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
            deps.push(super::DLHCMCNIHII::file_descriptor().clone());
            deps.push(super::EvolveBuildLevelInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EvolveBuildFinishScNotify::generated_message_descriptor_data());
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
