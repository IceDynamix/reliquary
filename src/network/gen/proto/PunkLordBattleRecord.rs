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

//! Generated file from `PunkLordBattleRecord.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PunkLordBattleRecord)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PunkLordBattleRecord {
    // message fields
    // @@protoc_insertion_point(field:PunkLordBattleRecord.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.damage_hp)
    pub damage_hp: u32,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.is_final_hit)
    pub is_final_hit: bool,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.over_kill_damage_hp)
    pub over_kill_damage_hp: u32,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.battle_replay_key)
    pub battle_replay_key: ::std::string::String,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::PunkLordBattleAvatar::PunkLordBattleAvatar>,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.assist_score)
    pub assist_score: u32,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.damage_score)
    pub damage_score: u32,
    // @@protoc_insertion_point(field:PunkLordBattleRecord.final_hit_score)
    pub final_hit_score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PunkLordBattleRecord.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PunkLordBattleRecord {
    fn default() -> &'a PunkLordBattleRecord {
        <PunkLordBattleRecord as ::protobuf::Message>::default_instance()
    }
}

impl PunkLordBattleRecord {
    pub fn new() -> PunkLordBattleRecord {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &PunkLordBattleRecord| { &m.uid },
            |m: &mut PunkLordBattleRecord| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "damage_hp",
            |m: &PunkLordBattleRecord| { &m.damage_hp },
            |m: &mut PunkLordBattleRecord| { &mut m.damage_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_final_hit",
            |m: &PunkLordBattleRecord| { &m.is_final_hit },
            |m: &mut PunkLordBattleRecord| { &mut m.is_final_hit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "over_kill_damage_hp",
            |m: &PunkLordBattleRecord| { &m.over_kill_damage_hp },
            |m: &mut PunkLordBattleRecord| { &mut m.over_kill_damage_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_replay_key",
            |m: &PunkLordBattleRecord| { &m.battle_replay_key },
            |m: &mut PunkLordBattleRecord| { &mut m.battle_replay_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &PunkLordBattleRecord| { &m.avatar_list },
            |m: &mut PunkLordBattleRecord| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "assist_score",
            |m: &PunkLordBattleRecord| { &m.assist_score },
            |m: &mut PunkLordBattleRecord| { &mut m.assist_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "damage_score",
            |m: &PunkLordBattleRecord| { &m.damage_score },
            |m: &mut PunkLordBattleRecord| { &mut m.damage_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "final_hit_score",
            |m: &PunkLordBattleRecord| { &m.final_hit_score },
            |m: &mut PunkLordBattleRecord| { &mut m.final_hit_score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PunkLordBattleRecord>(
            "PunkLordBattleRecord",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PunkLordBattleRecord {
    const NAME: &'static str = "PunkLordBattleRecord";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.uid = is.read_uint32()?;
                },
                16 => {
                    self.damage_hp = is.read_uint32()?;
                },
                24 => {
                    self.is_final_hit = is.read_bool()?;
                },
                32 => {
                    self.over_kill_damage_hp = is.read_uint32()?;
                },
                42 => {
                    self.battle_replay_key = is.read_string()?;
                },
                50 => {
                    self.avatar_list.push(is.read_message()?);
                },
                56 => {
                    self.assist_score = is.read_uint32()?;
                },
                64 => {
                    self.damage_score = is.read_uint32()?;
                },
                72 => {
                    self.final_hit_score = is.read_uint32()?;
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.uid);
        }
        if self.damage_hp != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.damage_hp);
        }
        if self.is_final_hit != false {
            my_size += 1 + 1;
        }
        if self.over_kill_damage_hp != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.over_kill_damage_hp);
        }
        if !self.battle_replay_key.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.battle_replay_key);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.assist_score != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.assist_score);
        }
        if self.damage_score != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.damage_score);
        }
        if self.final_hit_score != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.final_hit_score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(1, self.uid)?;
        }
        if self.damage_hp != 0 {
            os.write_uint32(2, self.damage_hp)?;
        }
        if self.is_final_hit != false {
            os.write_bool(3, self.is_final_hit)?;
        }
        if self.over_kill_damage_hp != 0 {
            os.write_uint32(4, self.over_kill_damage_hp)?;
        }
        if !self.battle_replay_key.is_empty() {
            os.write_string(5, &self.battle_replay_key)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.assist_score != 0 {
            os.write_uint32(7, self.assist_score)?;
        }
        if self.damage_score != 0 {
            os.write_uint32(8, self.damage_score)?;
        }
        if self.final_hit_score != 0 {
            os.write_uint32(9, self.final_hit_score)?;
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

    fn new() -> PunkLordBattleRecord {
        PunkLordBattleRecord::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.damage_hp = 0;
        self.is_final_hit = false;
        self.over_kill_damage_hp = 0;
        self.battle_replay_key.clear();
        self.avatar_list.clear();
        self.assist_score = 0;
        self.damage_score = 0;
        self.final_hit_score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PunkLordBattleRecord {
        static instance: PunkLordBattleRecord = PunkLordBattleRecord {
            uid: 0,
            damage_hp: 0,
            is_final_hit: false,
            over_kill_damage_hp: 0,
            battle_replay_key: ::std::string::String::new(),
            avatar_list: ::std::vec::Vec::new(),
            assist_score: 0,
            damage_score: 0,
            final_hit_score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PunkLordBattleRecord {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PunkLordBattleRecord").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PunkLordBattleRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PunkLordBattleRecord {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPunkLordBattleRecord.proto\x1a\x1aPunkLordBattleAvatar.proto\"\xe8\
    \x02\n\x14PunkLordBattleRecord\x12\x10\n\x03uid\x18\x01\x20\x01(\rR\x03u\
    id\x12\x1b\n\tdamage_hp\x18\x02\x20\x01(\rR\x08damageHp\x12\x20\n\x0cis_\
    final_hit\x18\x03\x20\x01(\x08R\nisFinalHit\x12-\n\x13over_kill_damage_h\
    p\x18\x04\x20\x01(\rR\x10overKillDamageHp\x12*\n\x11battle_replay_key\
    \x18\x05\x20\x01(\tR\x0fbattleReplayKey\x126\n\x0bavatar_list\x18\x06\
    \x20\x03(\x0b2\x15.PunkLordBattleAvatarR\navatarList\x12!\n\x0cassist_sc\
    ore\x18\x07\x20\x01(\rR\x0bassistScore\x12!\n\x0cdamage_score\x18\x08\
    \x20\x01(\rR\x0bdamageScore\x12&\n\x0ffinal_hit_score\x18\t\x20\x01(\rR\
    \rfinalHitScoreb\x06proto3\
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
            deps.push(super::PunkLordBattleAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PunkLordBattleRecord::generated_message_descriptor_data());
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
