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

//! Generated file from `ActivityRaidPlacingGameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ActivityRaidPlacingGameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ActivityRaidPlacingGameScRsp {
    // message fields
    // @@protoc_insertion_point(field:ActivityRaidPlacingGameScRsp.retcode)
    pub retcode: u32,
    // message oneof groups
    pub IJBOGOGIBKA: ::std::option::Option<activity_raid_placing_game_sc_rsp::IJBOGOGIBKA>,
    // special fields
    // @@protoc_insertion_point(special_field:ActivityRaidPlacingGameScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ActivityRaidPlacingGameScRsp {
    fn default() -> &'a ActivityRaidPlacingGameScRsp {
        <ActivityRaidPlacingGameScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ActivityRaidPlacingGameScRsp {
    pub fn new() -> ActivityRaidPlacingGameScRsp {
        ::std::default::Default::default()
    }

    // uint32 BFBCBCADGOJ = 13;

    pub fn BFBCBCADGOJ(&self) -> u32 {
        match self.IJBOGOGIBKA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_BFBCBCADGOJ(&mut self) {
        self.IJBOGOGIBKA = ::std::option::Option::None;
    }

    pub fn has_BFBCBCADGOJ(&self) -> bool {
        match self.IJBOGOGIBKA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BFBCBCADGOJ(&mut self, v: u32) {
        self.IJBOGOGIBKA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(v))
    }

    // uint32 MHJBFLBMGMH = 2;

    pub fn MHJBFLBMGMH(&self) -> u32 {
        match self.IJBOGOGIBKA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_MHJBFLBMGMH(&mut self) {
        self.IJBOGOGIBKA = ::std::option::Option::None;
    }

    pub fn has_MHJBFLBMGMH(&self) -> bool {
        match self.IJBOGOGIBKA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MHJBFLBMGMH(&mut self, v: u32) {
        self.IJBOGOGIBKA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ActivityRaidPlacingGameScRsp| { &m.retcode },
            |m: &mut ActivityRaidPlacingGameScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "BFBCBCADGOJ",
            ActivityRaidPlacingGameScRsp::has_BFBCBCADGOJ,
            ActivityRaidPlacingGameScRsp::BFBCBCADGOJ,
            ActivityRaidPlacingGameScRsp::set_BFBCBCADGOJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "MHJBFLBMGMH",
            ActivityRaidPlacingGameScRsp::has_MHJBFLBMGMH,
            ActivityRaidPlacingGameScRsp::MHJBFLBMGMH,
            ActivityRaidPlacingGameScRsp::set_MHJBFLBMGMH,
        ));
        oneofs.push(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ActivityRaidPlacingGameScRsp>(
            "ActivityRaidPlacingGameScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ActivityRaidPlacingGameScRsp {
    const NAME: &'static str = "ActivityRaidPlacingGameScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                104 => {
                    self.IJBOGOGIBKA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(is.read_uint32()?));
                },
                16 => {
                    self.IJBOGOGIBKA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(is.read_uint32()?));
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
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        if let ::std::option::Option::Some(ref v) = self.IJBOGOGIBKA {
            match v {
                &activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(v) => {
                    my_size += ::protobuf::rt::uint32_size(13, v);
                },
                &activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(v) => {
                    my_size += ::protobuf::rt::uint32_size(2, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        if let ::std::option::Option::Some(ref v) = self.IJBOGOGIBKA {
            match v {
                &activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::BFBCBCADGOJ(v) => {
                    os.write_uint32(13, v)?;
                },
                &activity_raid_placing_game_sc_rsp::IJBOGOGIBKA::MHJBFLBMGMH(v) => {
                    os.write_uint32(2, v)?;
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

    fn new() -> ActivityRaidPlacingGameScRsp {
        ActivityRaidPlacingGameScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.IJBOGOGIBKA = ::std::option::Option::None;
        self.IJBOGOGIBKA = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ActivityRaidPlacingGameScRsp {
        static instance: ActivityRaidPlacingGameScRsp = ActivityRaidPlacingGameScRsp {
            retcode: 0,
            IJBOGOGIBKA: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ActivityRaidPlacingGameScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ActivityRaidPlacingGameScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ActivityRaidPlacingGameScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActivityRaidPlacingGameScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ActivityRaidPlacingGameScRsp`
pub mod activity_raid_placing_game_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ActivityRaidPlacingGameScRsp.IJBOGOGIBKA)
    pub enum IJBOGOGIBKA {
        // @@protoc_insertion_point(oneof_field:ActivityRaidPlacingGameScRsp.BFBCBCADGOJ)
        BFBCBCADGOJ(u32),
        // @@protoc_insertion_point(oneof_field:ActivityRaidPlacingGameScRsp.MHJBFLBMGMH)
        MHJBFLBMGMH(u32),
    }

    impl ::protobuf::Oneof for IJBOGOGIBKA {
    }

    impl ::protobuf::OneofFull for IJBOGOGIBKA {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ActivityRaidPlacingGameScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("IJBOGOGIBKA").unwrap()).clone()
        }
    }

    impl IJBOGOGIBKA {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<IJBOGOGIBKA>("IJBOGOGIBKA")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"ActivityRaidPlacingGameScRsp.proto\"\x8f\x01\n\x1cActivityRaidPlacin\
    gGameScRsp\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07retcode\x12\"\n\
    \x0bBFBCBCADGOJ\x18\r\x20\x01(\rH\0R\x0bBFBCBCADGOJ\x12\"\n\x0bMHJBFLBMG\
    MH\x18\x02\x20\x01(\rH\0R\x0bMHJBFLBMGMHB\r\n\x0bIJBOGOGIBKAb\x06proto3\
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
            messages.push(ActivityRaidPlacingGameScRsp::generated_message_descriptor_data());
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
