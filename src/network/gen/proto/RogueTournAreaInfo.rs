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

//! Generated file from `RogueTournAreaInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueTournAreaInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournAreaInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueTournAreaInfo.is_unlocked)
    pub is_unlocked: bool,
    // @@protoc_insertion_point(field:RogueTournAreaInfo.area_id)
    pub area_id: u32,
    // @@protoc_insertion_point(field:RogueTournAreaInfo.unlocked_tourn_difficulty_list)
    pub unlocked_tourn_difficulty_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueTournAreaInfo.is_taken_reward)
    pub is_taken_reward: bool,
    // @@protoc_insertion_point(field:RogueTournAreaInfo.is_ext_finish)
    pub is_ext_finish: bool,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournAreaInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournAreaInfo {
    fn default() -> &'a RogueTournAreaInfo {
        <RogueTournAreaInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournAreaInfo {
    pub fn new() -> RogueTournAreaInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlocked",
            |m: &RogueTournAreaInfo| { &m.is_unlocked },
            |m: &mut RogueTournAreaInfo| { &mut m.is_unlocked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_id",
            |m: &RogueTournAreaInfo| { &m.area_id },
            |m: &mut RogueTournAreaInfo| { &mut m.area_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_tourn_difficulty_list",
            |m: &RogueTournAreaInfo| { &m.unlocked_tourn_difficulty_list },
            |m: &mut RogueTournAreaInfo| { &mut m.unlocked_tourn_difficulty_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_taken_reward",
            |m: &RogueTournAreaInfo| { &m.is_taken_reward },
            |m: &mut RogueTournAreaInfo| { &mut m.is_taken_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_ext_finish",
            |m: &RogueTournAreaInfo| { &m.is_ext_finish },
            |m: &mut RogueTournAreaInfo| { &mut m.is_ext_finish },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournAreaInfo>(
            "RogueTournAreaInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournAreaInfo {
    const NAME: &'static str = "RogueTournAreaInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.is_unlocked = is.read_bool()?;
                },
                16 => {
                    self.area_id = is.read_uint32()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlocked_tourn_difficulty_list)?;
                },
                48 => {
                    self.unlocked_tourn_difficulty_list.push(is.read_uint32()?);
                },
                64 => {
                    self.is_taken_reward = is.read_bool()?;
                },
                104 => {
                    self.is_ext_finish = is.read_bool()?;
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
        if self.is_unlocked != false {
            my_size += 1 + 1;
        }
        if self.area_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.area_id);
        }
        for value in &self.unlocked_tourn_difficulty_list {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        if self.is_taken_reward != false {
            my_size += 1 + 1;
        }
        if self.is_ext_finish != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_unlocked != false {
            os.write_bool(5, self.is_unlocked)?;
        }
        if self.area_id != 0 {
            os.write_uint32(2, self.area_id)?;
        }
        for v in &self.unlocked_tourn_difficulty_list {
            os.write_uint32(6, *v)?;
        };
        if self.is_taken_reward != false {
            os.write_bool(8, self.is_taken_reward)?;
        }
        if self.is_ext_finish != false {
            os.write_bool(13, self.is_ext_finish)?;
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

    fn new() -> RogueTournAreaInfo {
        RogueTournAreaInfo::new()
    }

    fn clear(&mut self) {
        self.is_unlocked = false;
        self.area_id = 0;
        self.unlocked_tourn_difficulty_list.clear();
        self.is_taken_reward = false;
        self.is_ext_finish = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournAreaInfo {
        static instance: RogueTournAreaInfo = RogueTournAreaInfo {
            is_unlocked: false,
            area_id: 0,
            unlocked_tourn_difficulty_list: ::std::vec::Vec::new(),
            is_taken_reward: false,
            is_ext_finish: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournAreaInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournAreaInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournAreaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournAreaInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18RogueTournAreaInfo.proto\"\xdf\x01\n\x12RogueTournAreaInfo\x12\x1f\
    \n\x0bis_unlocked\x18\x05\x20\x01(\x08R\nisUnlocked\x12\x17\n\x07area_id\
    \x18\x02\x20\x01(\rR\x06areaId\x12C\n\x1eunlocked_tourn_difficulty_list\
    \x18\x06\x20\x03(\rR\x1bunlockedTournDifficultyList\x12&\n\x0fis_taken_r\
    eward\x18\x08\x20\x01(\x08R\risTakenReward\x12\"\n\ris_ext_finish\x18\r\
    \x20\x01(\x08R\x0bisExtFinishB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(RogueTournAreaInfo::generated_message_descriptor_data());
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
