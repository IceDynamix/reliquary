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

//! Generated file from `BattleStatistics.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattleStatistics)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattleStatistics {
    // message fields
    // @@protoc_insertion_point(field:BattleStatistics.total_battle_turns)
    pub total_battle_turns: u32,
    // @@protoc_insertion_point(field:BattleStatistics.total_auto_turns)
    pub total_auto_turns: u32,
    // @@protoc_insertion_point(field:BattleStatistics.avatar_id_list)
    pub avatar_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BattleStatistics.ultra_cnt)
    pub ultra_cnt: u32,
    // @@protoc_insertion_point(field:BattleStatistics.total_delay_cumulate)
    pub total_delay_cumulate: f64,
    // @@protoc_insertion_point(field:BattleStatistics.cost_time)
    pub cost_time: f64,
    // @@protoc_insertion_point(field:BattleStatistics.battle_avatar_list)
    pub battle_avatar_list: ::std::vec::Vec<super::AvatarBattleInfo::AvatarBattleInfo>,
    // @@protoc_insertion_point(field:BattleStatistics.monster_list)
    pub monster_list: ::std::vec::Vec<super::MonsterBattleInfo::MonsterBattleInfo>,
    // @@protoc_insertion_point(field:BattleStatistics.round_cnt)
    pub round_cnt: u32,
    // @@protoc_insertion_point(field:BattleStatistics.cocoon_dead_wave)
    pub cocoon_dead_wave: u32,
    // @@protoc_insertion_point(field:BattleStatistics.avatar_battle_turns)
    pub avatar_battle_turns: u32,
    // @@protoc_insertion_point(field:BattleStatistics.monster_battle_turns)
    pub monster_battle_turns: u32,
    // @@protoc_insertion_point(field:BattleStatistics.custom_values)
    pub custom_values: ::std::collections::HashMap<::std::string::String, f32>,
    // @@protoc_insertion_point(field:BattleStatistics.challenge_score)
    pub challenge_score: u32,
    // @@protoc_insertion_point(field:BattleStatistics.PJOECEPBPOJ)
    pub PJOECEPBPOJ: ::std::vec::Vec<super::BattleEventBattleInfo::BattleEventBattleInfo>,
    // @@protoc_insertion_point(field:BattleStatistics.end_reason)
    pub end_reason: ::protobuf::EnumOrUnknown<super::BattleEndReason::BattleEndReason>,
    // @@protoc_insertion_point(field:BattleStatistics.DENNDAGNJNN)
    pub DENNDAGNJNN: ::std::vec::Vec<i32>,
    // @@protoc_insertion_point(field:BattleStatistics.battle_target_info)
    pub battle_target_info: ::std::collections::HashMap<u32, super::BattleTargetList::BattleTargetList>,
    // @@protoc_insertion_point(field:BattleStatistics.DBHGJCODLBK)
    pub DBHGJCODLBK: bool,
    // @@protoc_insertion_point(field:BattleStatistics.DAHDDICCOGD)
    pub DAHDDICCOGD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BattleStatistics.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattleStatistics {
    fn default() -> &'a BattleStatistics {
        <BattleStatistics as ::protobuf::Message>::default_instance()
    }
}

impl BattleStatistics {
    pub fn new() -> BattleStatistics {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(20);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_battle_turns",
            |m: &BattleStatistics| { &m.total_battle_turns },
            |m: &mut BattleStatistics| { &mut m.total_battle_turns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_auto_turns",
            |m: &BattleStatistics| { &m.total_auto_turns },
            |m: &mut BattleStatistics| { &mut m.total_auto_turns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_id_list",
            |m: &BattleStatistics| { &m.avatar_id_list },
            |m: &mut BattleStatistics| { &mut m.avatar_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ultra_cnt",
            |m: &BattleStatistics| { &m.ultra_cnt },
            |m: &mut BattleStatistics| { &mut m.ultra_cnt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_delay_cumulate",
            |m: &BattleStatistics| { &m.total_delay_cumulate },
            |m: &mut BattleStatistics| { &mut m.total_delay_cumulate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cost_time",
            |m: &BattleStatistics| { &m.cost_time },
            |m: &mut BattleStatistics| { &mut m.cost_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "battle_avatar_list",
            |m: &BattleStatistics| { &m.battle_avatar_list },
            |m: &mut BattleStatistics| { &mut m.battle_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "monster_list",
            |m: &BattleStatistics| { &m.monster_list },
            |m: &mut BattleStatistics| { &mut m.monster_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "round_cnt",
            |m: &BattleStatistics| { &m.round_cnt },
            |m: &mut BattleStatistics| { &mut m.round_cnt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cocoon_dead_wave",
            |m: &BattleStatistics| { &m.cocoon_dead_wave },
            |m: &mut BattleStatistics| { &mut m.cocoon_dead_wave },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_battle_turns",
            |m: &BattleStatistics| { &m.avatar_battle_turns },
            |m: &mut BattleStatistics| { &mut m.avatar_battle_turns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_battle_turns",
            |m: &BattleStatistics| { &m.monster_battle_turns },
            |m: &mut BattleStatistics| { &mut m.monster_battle_turns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "custom_values",
            |m: &BattleStatistics| { &m.custom_values },
            |m: &mut BattleStatistics| { &mut m.custom_values },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_score",
            |m: &BattleStatistics| { &m.challenge_score },
            |m: &mut BattleStatistics| { &mut m.challenge_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PJOECEPBPOJ",
            |m: &BattleStatistics| { &m.PJOECEPBPOJ },
            |m: &mut BattleStatistics| { &mut m.PJOECEPBPOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_reason",
            |m: &BattleStatistics| { &m.end_reason },
            |m: &mut BattleStatistics| { &mut m.end_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DENNDAGNJNN",
            |m: &BattleStatistics| { &m.DENNDAGNJNN },
            |m: &mut BattleStatistics| { &mut m.DENNDAGNJNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "battle_target_info",
            |m: &BattleStatistics| { &m.battle_target_info },
            |m: &mut BattleStatistics| { &mut m.battle_target_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBHGJCODLBK",
            |m: &BattleStatistics| { &m.DBHGJCODLBK },
            |m: &mut BattleStatistics| { &mut m.DBHGJCODLBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAHDDICCOGD",
            |m: &BattleStatistics| { &m.DAHDDICCOGD },
            |m: &mut BattleStatistics| { &mut m.DAHDDICCOGD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattleStatistics>(
            "BattleStatistics",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattleStatistics {
    const NAME: &'static str = "BattleStatistics";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.total_battle_turns = is.read_uint32()?;
                },
                16 => {
                    self.total_auto_turns = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_id_list)?;
                },
                24 => {
                    self.avatar_id_list.push(is.read_uint32()?);
                },
                32 => {
                    self.ultra_cnt = is.read_uint32()?;
                },
                41 => {
                    self.total_delay_cumulate = is.read_double()?;
                },
                49 => {
                    self.cost_time = is.read_double()?;
                },
                58 => {
                    self.battle_avatar_list.push(is.read_message()?);
                },
                66 => {
                    self.monster_list.push(is.read_message()?);
                },
                72 => {
                    self.round_cnt = is.read_uint32()?;
                },
                80 => {
                    self.cocoon_dead_wave = is.read_uint32()?;
                },
                88 => {
                    self.avatar_battle_turns = is.read_uint32()?;
                },
                96 => {
                    self.monster_battle_turns = is.read_uint32()?;
                },
                106 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            21 => value = is.read_float()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.custom_values.insert(key, value);
                },
                112 => {
                    self.challenge_score = is.read_uint32()?;
                },
                130 => {
                    self.PJOECEPBPOJ.push(is.read_message()?);
                },
                152 => {
                    self.end_reason = is.read_enum_or_unknown()?;
                },
                178 => {
                    is.read_repeated_packed_int32_into(&mut self.DENNDAGNJNN)?;
                },
                176 => {
                    self.DENNDAGNJNN.push(is.read_int32()?);
                },
                226 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.battle_target_info.insert(key, value);
                },
                256 => {
                    self.DBHGJCODLBK = is.read_bool()?;
                },
                280 => {
                    self.DAHDDICCOGD = is.read_uint32()?;
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
        if self.total_battle_turns != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.total_battle_turns);
        }
        if self.total_auto_turns != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.total_auto_turns);
        }
        for value in &self.avatar_id_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.ultra_cnt != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ultra_cnt);
        }
        if self.total_delay_cumulate != 0. {
            my_size += 1 + 8;
        }
        if self.cost_time != 0. {
            my_size += 1 + 8;
        }
        for value in &self.battle_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.monster_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.round_cnt != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.round_cnt);
        }
        if self.cocoon_dead_wave != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.cocoon_dead_wave);
        }
        if self.avatar_battle_turns != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.avatar_battle_turns);
        }
        if self.monster_battle_turns != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.monster_battle_turns);
        }
        for (k, v) in &self.custom_values {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.challenge_score != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.challenge_score);
        }
        for value in &self.PJOECEPBPOJ {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::BattleEndReason::BattleEndReason::BATTLE_END_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(19, self.end_reason.value());
        }
        for value in &self.DENNDAGNJNN {
            my_size += ::protobuf::rt::int32_size(22, *value);
        };
        for (k, v) in &self.battle_target_info {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.DBHGJCODLBK != false {
            my_size += 2 + 1;
        }
        if self.DAHDDICCOGD != 0 {
            my_size += ::protobuf::rt::uint32_size(35, self.DAHDDICCOGD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.total_battle_turns != 0 {
            os.write_uint32(1, self.total_battle_turns)?;
        }
        if self.total_auto_turns != 0 {
            os.write_uint32(2, self.total_auto_turns)?;
        }
        for v in &self.avatar_id_list {
            os.write_uint32(3, *v)?;
        };
        if self.ultra_cnt != 0 {
            os.write_uint32(4, self.ultra_cnt)?;
        }
        if self.total_delay_cumulate != 0. {
            os.write_double(5, self.total_delay_cumulate)?;
        }
        if self.cost_time != 0. {
            os.write_double(6, self.cost_time)?;
        }
        for v in &self.battle_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        for v in &self.monster_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.round_cnt != 0 {
            os.write_uint32(9, self.round_cnt)?;
        }
        if self.cocoon_dead_wave != 0 {
            os.write_uint32(10, self.cocoon_dead_wave)?;
        }
        if self.avatar_battle_turns != 0 {
            os.write_uint32(11, self.avatar_battle_turns)?;
        }
        if self.monster_battle_turns != 0 {
            os.write_uint32(12, self.monster_battle_turns)?;
        }
        for (k, v) in &self.custom_values {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            os.write_raw_varint32(106)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_float(2, *v)?;
        };
        if self.challenge_score != 0 {
            os.write_uint32(14, self.challenge_score)?;
        }
        for v in &self.PJOECEPBPOJ {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        };
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::BattleEndReason::BattleEndReason::BATTLE_END_REASON_NONE) {
            os.write_enum(19, ::protobuf::EnumOrUnknown::value(&self.end_reason))?;
        }
        for v in &self.DENNDAGNJNN {
            os.write_int32(22, *v)?;
        };
        for (k, v) in &self.battle_target_info {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(226)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.DBHGJCODLBK != false {
            os.write_bool(32, self.DBHGJCODLBK)?;
        }
        if self.DAHDDICCOGD != 0 {
            os.write_uint32(35, self.DAHDDICCOGD)?;
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

    fn new() -> BattleStatistics {
        BattleStatistics::new()
    }

    fn clear(&mut self) {
        self.total_battle_turns = 0;
        self.total_auto_turns = 0;
        self.avatar_id_list.clear();
        self.ultra_cnt = 0;
        self.total_delay_cumulate = 0.;
        self.cost_time = 0.;
        self.battle_avatar_list.clear();
        self.monster_list.clear();
        self.round_cnt = 0;
        self.cocoon_dead_wave = 0;
        self.avatar_battle_turns = 0;
        self.monster_battle_turns = 0;
        self.custom_values.clear();
        self.challenge_score = 0;
        self.PJOECEPBPOJ.clear();
        self.end_reason = ::protobuf::EnumOrUnknown::new(super::BattleEndReason::BattleEndReason::BATTLE_END_REASON_NONE);
        self.DENNDAGNJNN.clear();
        self.battle_target_info.clear();
        self.DBHGJCODLBK = false;
        self.DAHDDICCOGD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattleStatistics {
        static instance: ::protobuf::rt::Lazy<BattleStatistics> = ::protobuf::rt::Lazy::new();
        instance.get(BattleStatistics::new)
    }
}

impl ::protobuf::MessageFull for BattleStatistics {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattleStatistics").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattleStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattleStatistics {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16BattleStatistics.proto\x1a\x16AvatarBattleInfo.proto\x1a\x15Battle\
    EndReason.proto\x1a\x16BattleTargetList.proto\x1a\x1bBattleEventBattleIn\
    fo.proto\x1a\x17MonsterBattleInfo.proto\"\xd1\x08\n\x10BattleStatistics\
    \x12,\n\x12total_battle_turns\x18\x01\x20\x01(\rR\x10totalBattleTurns\
    \x12(\n\x10total_auto_turns\x18\x02\x20\x01(\rR\x0etotalAutoTurns\x12$\n\
    \x0eavatar_id_list\x18\x03\x20\x03(\rR\x0cavatarIdList\x12\x1b\n\tultra_\
    cnt\x18\x04\x20\x01(\rR\x08ultraCnt\x120\n\x14total_delay_cumulate\x18\
    \x05\x20\x01(\x01R\x12totalDelayCumulate\x12\x1b\n\tcost_time\x18\x06\
    \x20\x01(\x01R\x08costTime\x12?\n\x12battle_avatar_list\x18\x07\x20\x03(\
    \x0b2\x11.AvatarBattleInfoR\x10battleAvatarList\x125\n\x0cmonster_list\
    \x18\x08\x20\x03(\x0b2\x12.MonsterBattleInfoR\x0bmonsterList\x12\x1b\n\t\
    round_cnt\x18\t\x20\x01(\rR\x08roundCnt\x12(\n\x10cocoon_dead_wave\x18\n\
    \x20\x01(\rR\x0ecocoonDeadWave\x12.\n\x13avatar_battle_turns\x18\x0b\x20\
    \x01(\rR\x11avatarBattleTurns\x120\n\x14monster_battle_turns\x18\x0c\x20\
    \x01(\rR\x12monsterBattleTurns\x12H\n\rcustom_values\x18\r\x20\x03(\x0b2\
    #.BattleStatistics.CustomValuesEntryR\x0ccustomValues\x12'\n\x0fchalleng\
    e_score\x18\x0e\x20\x01(\rR\x0echallengeScore\x128\n\x0bPJOECEPBPOJ\x18\
    \x10\x20\x03(\x0b2\x16.BattleEventBattleInfoR\x0bPJOECEPBPOJ\x12/\n\nend\
    _reason\x18\x13\x20\x01(\x0e2\x10.BattleEndReasonR\tendReason\x12\x20\n\
    \x0bDENNDAGNJNN\x18\x16\x20\x03(\x05R\x0bDENNDAGNJNN\x12U\n\x12battle_ta\
    rget_info\x18\x1c\x20\x03(\x0b2'.BattleStatistics.BattleTargetInfoEntryR\
    \x10battleTargetInfo\x12\x20\n\x0bDBHGJCODLBK\x18\x20\x20\x01(\x08R\x0bD\
    BHGJCODLBK\x12\x20\n\x0bDAHDDICCOGD\x18#\x20\x01(\rR\x0bDAHDDICCOGD\x1a?\
    \n\x11CustomValuesEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x02R\x05value:\x028\x01\x1aV\n\x15Battl\
    eTargetInfoEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12'\n\x05v\
    alue\x18\x02\x20\x01(\x0b2\x11.BattleTargetListR\x05value:\x028\x01B\x15\
    \n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::AvatarBattleInfo::file_descriptor().clone());
            deps.push(super::BattleEndReason::file_descriptor().clone());
            deps.push(super::BattleTargetList::file_descriptor().clone());
            deps.push(super::BattleEventBattleInfo::file_descriptor().clone());
            deps.push(super::MonsterBattleInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattleStatistics::generated_message_descriptor_data());
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
