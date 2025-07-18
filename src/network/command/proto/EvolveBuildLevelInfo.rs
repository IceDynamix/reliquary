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

//! Generated file from `EvolveBuildLevelInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EvolveBuildLevelInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EvolveBuildLevelInfo {
    // message fields
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.cur_game_exp)
    pub cur_game_exp: u32,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.battle_target_list)
    pub battle_target_list: ::std::vec::Vec<super::BattleTarget::BattleTarget>,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.season)
    pub season: ::protobuf::EnumOrUnknown<super::KLNIPNJCNMJ::KLNIPNJCNMJ>,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.round_cnt)
    pub round_cnt: u32,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.battle_info)
    pub battle_info: ::protobuf::MessageField<super::EvolveBuildBattleInfo::EvolveBuildBattleInfo>,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.period_id_list)
    pub period_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EvolveBuildLevelInfo.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::EvolveBuildAvatar::EvolveBuildAvatar>,
    // special fields
    // @@protoc_insertion_point(special_field:EvolveBuildLevelInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EvolveBuildLevelInfo {
    fn default() -> &'a EvolveBuildLevelInfo {
        <EvolveBuildLevelInfo as ::protobuf::Message>::default_instance()
    }
}

impl EvolveBuildLevelInfo {
    pub fn new() -> EvolveBuildLevelInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_game_exp",
            |m: &EvolveBuildLevelInfo| { &m.cur_game_exp },
            |m: &mut EvolveBuildLevelInfo| { &mut m.cur_game_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "battle_target_list",
            |m: &EvolveBuildLevelInfo| { &m.battle_target_list },
            |m: &mut EvolveBuildLevelInfo| { &mut m.battle_target_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "season",
            |m: &EvolveBuildLevelInfo| { &m.season },
            |m: &mut EvolveBuildLevelInfo| { &mut m.season },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "round_cnt",
            |m: &EvolveBuildLevelInfo| { &m.round_cnt },
            |m: &mut EvolveBuildLevelInfo| { &mut m.round_cnt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EvolveBuildBattleInfo::EvolveBuildBattleInfo>(
            "battle_info",
            |m: &EvolveBuildLevelInfo| { &m.battle_info },
            |m: &mut EvolveBuildLevelInfo| { &mut m.battle_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "period_id_list",
            |m: &EvolveBuildLevelInfo| { &m.period_id_list },
            |m: &mut EvolveBuildLevelInfo| { &mut m.period_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EvolveBuildLevelInfo| { &m.avatar_list },
            |m: &mut EvolveBuildLevelInfo| { &mut m.avatar_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EvolveBuildLevelInfo>(
            "EvolveBuildLevelInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EvolveBuildLevelInfo {
    const NAME: &'static str = "EvolveBuildLevelInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.cur_game_exp = is.read_uint32()?;
                },
                74 => {
                    self.battle_target_list.push(is.read_message()?);
                },
                104 => {
                    self.season = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.round_cnt = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.battle_info)?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.period_id_list)?;
                },
                32 => {
                    self.period_id_list.push(is.read_uint32()?);
                },
                42 => {
                    self.avatar_list.push(is.read_message()?);
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
        if self.cur_game_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.cur_game_exp);
        }
        for value in &self.battle_target_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.season != ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.season.value());
        }
        if self.round_cnt != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.round_cnt);
        }
        if let Some(v) = self.battle_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.period_id_list);
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.cur_game_exp != 0 {
            os.write_uint32(2, self.cur_game_exp)?;
        }
        for v in &self.battle_target_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.season != ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.season))?;
        }
        if self.round_cnt != 0 {
            os.write_uint32(8, self.round_cnt)?;
        }
        if let Some(v) = self.battle_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        os.write_repeated_packed_uint32(4, &self.period_id_list)?;
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> EvolveBuildLevelInfo {
        EvolveBuildLevelInfo::new()
    }

    fn clear(&mut self) {
        self.cur_game_exp = 0;
        self.battle_target_list.clear();
        self.season = ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE);
        self.round_cnt = 0;
        self.battle_info.clear();
        self.period_id_list.clear();
        self.avatar_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EvolveBuildLevelInfo {
        static instance: EvolveBuildLevelInfo = EvolveBuildLevelInfo {
            cur_game_exp: 0,
            battle_target_list: ::std::vec::Vec::new(),
            season: ::protobuf::EnumOrUnknown::from_i32(0),
            round_cnt: 0,
            battle_info: ::protobuf::MessageField::none(),
            period_id_list: ::std::vec::Vec::new(),
            avatar_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EvolveBuildLevelInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EvolveBuildLevelInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EvolveBuildLevelInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvolveBuildLevelInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aEvolveBuildLevelInfo.proto\x1a\x12BattleTarget.proto\x1a\x17Evolve\
    BuildAvatar.proto\x1a\x1bEvolveBuildBattleInfo.proto\x1a\x11KLNIPNJCNMJ.\
    proto\"\xcc\x02\n\x14EvolveBuildLevelInfo\x12\x20\n\x0ccur_game_exp\x18\
    \x02\x20\x01(\rR\ncurGameExp\x12;\n\x12battle_target_list\x18\t\x20\x03(\
    \x0b2\r.BattleTargetR\x10battleTargetList\x12$\n\x06season\x18\r\x20\x01\
    (\x0e2\x0c.KLNIPNJCNMJR\x06season\x12\x1b\n\tround_cnt\x18\x08\x20\x01(\
    \rR\x08roundCnt\x127\n\x0bbattle_info\x18\x0b\x20\x01(\x0b2\x16.EvolveBu\
    ildBattleInfoR\nbattleInfo\x12$\n\x0eperiod_id_list\x18\x04\x20\x03(\rR\
    \x0cperiodIdList\x123\n\x0bavatar_list\x18\x05\x20\x03(\x0b2\x12.EvolveB\
    uildAvatarR\navatarListb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::BattleTarget::file_descriptor().clone());
            deps.push(super::EvolveBuildAvatar::file_descriptor().clone());
            deps.push(super::EvolveBuildBattleInfo::file_descriptor().clone());
            deps.push(super::KLNIPNJCNMJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EvolveBuildLevelInfo::generated_message_descriptor_data());
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
