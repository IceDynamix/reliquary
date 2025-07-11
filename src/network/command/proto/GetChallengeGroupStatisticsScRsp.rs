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

//! Generated file from `GetChallengeGroupStatisticsScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetChallengeGroupStatisticsScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChallengeGroupStatisticsScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.group_id)
    pub group_id: u32,
    // message oneof groups
    pub LCDJBAAKIHB: ::std::option::Option<get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB>,
    // special fields
    // @@protoc_insertion_point(special_field:GetChallengeGroupStatisticsScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChallengeGroupStatisticsScRsp {
    fn default() -> &'a GetChallengeGroupStatisticsScRsp {
        <GetChallengeGroupStatisticsScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChallengeGroupStatisticsScRsp {
    pub fn new() -> GetChallengeGroupStatisticsScRsp {
        ::std::default::Default::default()
    }

    // .ChallengeStatistics challenge_default = 12;

    pub fn challenge_default(&self) -> &super::ChallengeStatistics::ChallengeStatistics {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(ref v)) => v,
            _ => <super::ChallengeStatistics::ChallengeStatistics as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_challenge_default(&mut self) {
        self.LCDJBAAKIHB = ::std::option::Option::None;
    }

    pub fn has_challenge_default(&self) -> bool {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_challenge_default(&mut self, v: super::ChallengeStatistics::ChallengeStatistics) {
        self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(v))
    }

    // Mutable pointer to the field.
    pub fn mut_challenge_default(&mut self) -> &mut super::ChallengeStatistics::ChallengeStatistics {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(_)) = self.LCDJBAAKIHB {
        } else {
            self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(super::ChallengeStatistics::ChallengeStatistics::new()));
        }
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_challenge_default(&mut self) -> super::ChallengeStatistics::ChallengeStatistics {
        if self.has_challenge_default() {
            match self.LCDJBAAKIHB.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ChallengeStatistics::ChallengeStatistics::new()
        }
    }

    // .ChallengeStoryStatistics challenge_story = 1;

    pub fn challenge_story(&self) -> &super::ChallengeStoryStatistics::ChallengeStoryStatistics {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(ref v)) => v,
            _ => <super::ChallengeStoryStatistics::ChallengeStoryStatistics as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_challenge_story(&mut self) {
        self.LCDJBAAKIHB = ::std::option::Option::None;
    }

    pub fn has_challenge_story(&self) -> bool {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_challenge_story(&mut self, v: super::ChallengeStoryStatistics::ChallengeStoryStatistics) {
        self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(v))
    }

    // Mutable pointer to the field.
    pub fn mut_challenge_story(&mut self) -> &mut super::ChallengeStoryStatistics::ChallengeStoryStatistics {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(_)) = self.LCDJBAAKIHB {
        } else {
            self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(super::ChallengeStoryStatistics::ChallengeStoryStatistics::new()));
        }
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_challenge_story(&mut self) -> super::ChallengeStoryStatistics::ChallengeStoryStatistics {
        if self.has_challenge_story() {
            match self.LCDJBAAKIHB.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ChallengeStoryStatistics::ChallengeStoryStatistics::new()
        }
    }

    // .ChallengeBossStatistics challenge_boss = 10;

    pub fn challenge_boss(&self) -> &super::ChallengeBossStatistics::ChallengeBossStatistics {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(ref v)) => v,
            _ => <super::ChallengeBossStatistics::ChallengeBossStatistics as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_challenge_boss(&mut self) {
        self.LCDJBAAKIHB = ::std::option::Option::None;
    }

    pub fn has_challenge_boss(&self) -> bool {
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_challenge_boss(&mut self, v: super::ChallengeBossStatistics::ChallengeBossStatistics) {
        self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(v))
    }

    // Mutable pointer to the field.
    pub fn mut_challenge_boss(&mut self) -> &mut super::ChallengeBossStatistics::ChallengeBossStatistics {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(_)) = self.LCDJBAAKIHB {
        } else {
            self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(super::ChallengeBossStatistics::ChallengeBossStatistics::new()));
        }
        match self.LCDJBAAKIHB {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_challenge_boss(&mut self) -> super::ChallengeBossStatistics::ChallengeBossStatistics {
        if self.has_challenge_boss() {
            match self.LCDJBAAKIHB.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ChallengeBossStatistics::ChallengeBossStatistics::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.retcode },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.group_id },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ChallengeStatistics::ChallengeStatistics>(
            "challenge_default",
            GetChallengeGroupStatisticsScRsp::has_challenge_default,
            GetChallengeGroupStatisticsScRsp::challenge_default,
            GetChallengeGroupStatisticsScRsp::mut_challenge_default,
            GetChallengeGroupStatisticsScRsp::set_challenge_default,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ChallengeStoryStatistics::ChallengeStoryStatistics>(
            "challenge_story",
            GetChallengeGroupStatisticsScRsp::has_challenge_story,
            GetChallengeGroupStatisticsScRsp::challenge_story,
            GetChallengeGroupStatisticsScRsp::mut_challenge_story,
            GetChallengeGroupStatisticsScRsp::set_challenge_story,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ChallengeBossStatistics::ChallengeBossStatistics>(
            "challenge_boss",
            GetChallengeGroupStatisticsScRsp::has_challenge_boss,
            GetChallengeGroupStatisticsScRsp::challenge_boss,
            GetChallengeGroupStatisticsScRsp::mut_challenge_boss,
            GetChallengeGroupStatisticsScRsp::set_challenge_boss,
        ));
        oneofs.push(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChallengeGroupStatisticsScRsp>(
            "GetChallengeGroupStatisticsScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChallengeGroupStatisticsScRsp {
    const NAME: &'static str = "GetChallengeGroupStatisticsScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                40 => {
                    self.group_id = is.read_uint32()?;
                },
                98 => {
                    self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(is.read_message()?));
                },
                10 => {
                    self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(is.read_message()?));
                },
                82 => {
                    self.LCDJBAAKIHB = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(is.read_message()?));
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
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.group_id);
        }
        if let ::std::option::Option::Some(ref v) = self.LCDJBAAKIHB {
            match v {
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        if self.group_id != 0 {
            os.write_uint32(5, self.group_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.LCDJBAAKIHB {
            match v {
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeDefault(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeStory(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::LCDJBAAKIHB::ChallengeBoss(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
            };
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

    fn new() -> GetChallengeGroupStatisticsScRsp {
        GetChallengeGroupStatisticsScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.group_id = 0;
        self.LCDJBAAKIHB = ::std::option::Option::None;
        self.LCDJBAAKIHB = ::std::option::Option::None;
        self.LCDJBAAKIHB = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChallengeGroupStatisticsScRsp {
        static instance: GetChallengeGroupStatisticsScRsp = GetChallengeGroupStatisticsScRsp {
            retcode: 0,
            group_id: 0,
            LCDJBAAKIHB: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChallengeGroupStatisticsScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChallengeGroupStatisticsScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChallengeGroupStatisticsScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChallengeGroupStatisticsScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `GetChallengeGroupStatisticsScRsp`
pub mod get_challenge_group_statistics_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:GetChallengeGroupStatisticsScRsp.LCDJBAAKIHB)
    pub enum LCDJBAAKIHB {
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.challenge_default)
        ChallengeDefault(super::super::ChallengeStatistics::ChallengeStatistics),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.challenge_story)
        ChallengeStory(super::super::ChallengeStoryStatistics::ChallengeStoryStatistics),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.challenge_boss)
        ChallengeBoss(super::super::ChallengeBossStatistics::ChallengeBossStatistics),
    }

    impl ::protobuf::Oneof for LCDJBAAKIHB {
    }

    impl ::protobuf::OneofFull for LCDJBAAKIHB {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::GetChallengeGroupStatisticsScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("LCDJBAAKIHB").unwrap()).clone()
        }
    }

    impl LCDJBAAKIHB {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<LCDJBAAKIHB>("LCDJBAAKIHB")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&GetChallengeGroupStatisticsScRsp.proto\x1a\x1dChallengeBossStatistics\
    .proto\x1a\x19ChallengeStatistics.proto\x1a\x1eChallengeStoryStatistics.\
    proto\"\xb4\x02\n\x20GetChallengeGroupStatisticsScRsp\x12\x18\n\x07retco\
    de\x18\t\x20\x01(\rR\x07retcode\x12\x19\n\x08group_id\x18\x05\x20\x01(\r\
    R\x07groupId\x12C\n\x11challenge_default\x18\x0c\x20\x01(\x0b2\x14.Chall\
    engeStatisticsH\0R\x10challengeDefault\x12D\n\x0fchallenge_story\x18\x01\
    \x20\x01(\x0b2\x19.ChallengeStoryStatisticsH\0R\x0echallengeStory\x12A\n\
    \x0echallenge_boss\x18\n\x20\x01(\x0b2\x18.ChallengeBossStatisticsH\0R\r\
    challengeBossB\r\n\x0bLCDJBAAKIHBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::ChallengeBossStatistics::file_descriptor().clone());
            deps.push(super::ChallengeStatistics::file_descriptor().clone());
            deps.push(super::ChallengeStoryStatistics::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChallengeGroupStatisticsScRsp::generated_message_descriptor_data());
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
