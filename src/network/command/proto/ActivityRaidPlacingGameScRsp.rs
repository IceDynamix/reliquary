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

//! Generated file from `ActivityRaidPlacingGameScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ActivityRaidPlacingGameScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ActivityRaidPlacingGameScRsp {
    // message fields
    // @@protoc_insertion_point(field:ActivityRaidPlacingGameScRsp.retcode)
    pub retcode: u32,
    // message oneof groups
    pub IPCMEJKOEDA: ::std::option::Option<activity_raid_placing_game_sc_rsp::IPCMEJKOEDA>,
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

    // uint32 CJEMMDPICLJ = 4;

    pub fn CJEMMDPICLJ(&self) -> u32 {
        match self.IPCMEJKOEDA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_CJEMMDPICLJ(&mut self) {
        self.IPCMEJKOEDA = ::std::option::Option::None;
    }

    pub fn has_CJEMMDPICLJ(&self) -> bool {
        match self.IPCMEJKOEDA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_CJEMMDPICLJ(&mut self, v: u32) {
        self.IPCMEJKOEDA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(v))
    }

    // uint32 FJDABPPANDC = 14;

    pub fn FJDABPPANDC(&self) -> u32 {
        match self.IPCMEJKOEDA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_FJDABPPANDC(&mut self) {
        self.IPCMEJKOEDA = ::std::option::Option::None;
    }

    pub fn has_FJDABPPANDC(&self) -> bool {
        match self.IPCMEJKOEDA {
            ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FJDABPPANDC(&mut self, v: u32) {
        self.IPCMEJKOEDA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(v))
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
            "CJEMMDPICLJ",
            ActivityRaidPlacingGameScRsp::has_CJEMMDPICLJ,
            ActivityRaidPlacingGameScRsp::CJEMMDPICLJ,
            ActivityRaidPlacingGameScRsp::set_CJEMMDPICLJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "FJDABPPANDC",
            ActivityRaidPlacingGameScRsp::has_FJDABPPANDC,
            ActivityRaidPlacingGameScRsp::FJDABPPANDC,
            ActivityRaidPlacingGameScRsp::set_FJDABPPANDC,
        ));
        oneofs.push(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::generated_oneof_descriptor_data());
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
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                32 => {
                    self.IPCMEJKOEDA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(is.read_uint32()?));
                },
                112 => {
                    self.IPCMEJKOEDA = ::std::option::Option::Some(activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(is.read_uint32()?));
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
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if let ::std::option::Option::Some(ref v) = self.IPCMEJKOEDA {
            match v {
                &activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(v) => {
                    my_size += ::protobuf::rt::uint32_size(4, v);
                },
                &activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(v) => {
                    my_size += ::protobuf::rt::uint32_size(14, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if let ::std::option::Option::Some(ref v) = self.IPCMEJKOEDA {
            match v {
                &activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::CJEMMDPICLJ(v) => {
                    os.write_uint32(4, v)?;
                },
                &activity_raid_placing_game_sc_rsp::IPCMEJKOEDA::FJDABPPANDC(v) => {
                    os.write_uint32(14, v)?;
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
        self.IPCMEJKOEDA = ::std::option::Option::None;
        self.IPCMEJKOEDA = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ActivityRaidPlacingGameScRsp {
        static instance: ActivityRaidPlacingGameScRsp = ActivityRaidPlacingGameScRsp {
            retcode: 0,
            IPCMEJKOEDA: ::std::option::Option::None,
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
    // @@protoc_insertion_point(oneof:ActivityRaidPlacingGameScRsp.IPCMEJKOEDA)
    pub enum IPCMEJKOEDA {
        // @@protoc_insertion_point(oneof_field:ActivityRaidPlacingGameScRsp.CJEMMDPICLJ)
        CJEMMDPICLJ(u32),
        // @@protoc_insertion_point(oneof_field:ActivityRaidPlacingGameScRsp.FJDABPPANDC)
        FJDABPPANDC(u32),
    }

    impl ::protobuf::Oneof for IPCMEJKOEDA {
    }

    impl ::protobuf::OneofFull for IPCMEJKOEDA {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ActivityRaidPlacingGameScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("IPCMEJKOEDA").unwrap()).clone()
        }
    }

    impl IPCMEJKOEDA {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<IPCMEJKOEDA>("IPCMEJKOEDA")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"ActivityRaidPlacingGameScRsp.proto\"\x8f\x01\n\x1cActivityRaidPlacin\
    gGameScRsp\x12\x18\n\x07retcode\x18\x02\x20\x01(\rR\x07retcode\x12\"\n\
    \x0bCJEMMDPICLJ\x18\x04\x20\x01(\rH\0R\x0bCJEMMDPICLJ\x12\"\n\x0bFJDABPP\
    ANDC\x18\x0e\x20\x01(\rH\0R\x0bFJDABPPANDCB\r\n\x0bIPCMEJKOEDAb\x06proto\
    3\
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
