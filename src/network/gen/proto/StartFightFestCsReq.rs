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

//! Generated file from `StartFightFestCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartFightFestCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartFightFestCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartFightFestCsReq.slot)
    pub slot: ::protobuf::EnumOrUnknown<super::FightFestType::FightFestType>,
    // @@protoc_insertion_point(field:StartFightFestCsReq.DAAIBKIKBEJ)
    pub DAAIBKIKBEJ: u32,
    // @@protoc_insertion_point(field:StartFightFestCsReq.JJIPJBEAJIH)
    pub JJIPJBEAJIH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:StartFightFestCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::BNBMFMMBIAO::BNBMFMMBIAO>,
    // @@protoc_insertion_point(field:StartFightFestCsReq.id)
    pub id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartFightFestCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartFightFestCsReq {
    fn default() -> &'a StartFightFestCsReq {
        <StartFightFestCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartFightFestCsReq {
    pub fn new() -> StartFightFestCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &StartFightFestCsReq| { &m.slot },
            |m: &mut StartFightFestCsReq| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAAIBKIKBEJ",
            |m: &StartFightFestCsReq| { &m.DAAIBKIKBEJ },
            |m: &mut StartFightFestCsReq| { &mut m.DAAIBKIKBEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JJIPJBEAJIH",
            |m: &StartFightFestCsReq| { &m.JJIPJBEAJIH },
            |m: &mut StartFightFestCsReq| { &mut m.JJIPJBEAJIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &StartFightFestCsReq| { &m.avatar_list },
            |m: &mut StartFightFestCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &StartFightFestCsReq| { &m.id },
            |m: &mut StartFightFestCsReq| { &mut m.id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartFightFestCsReq>(
            "StartFightFestCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartFightFestCsReq {
    const NAME: &'static str = "StartFightFestCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.slot = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.DAAIBKIKBEJ = is.read_uint32()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.JJIPJBEAJIH)?;
                },
                96 => {
                    self.JJIPJBEAJIH.push(is.read_uint32()?);
                },
                90 => {
                    self.avatar_list.push(is.read_message()?);
                },
                56 => {
                    self.id = is.read_uint32()?;
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
        if self.slot != ::protobuf::EnumOrUnknown::new(super::FightFestType::FightFestType::FIGHT_FEST_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.slot.value());
        }
        if self.DAAIBKIKBEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.DAAIBKIKBEJ);
        }
        for value in &self.JJIPJBEAJIH {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.slot != ::protobuf::EnumOrUnknown::new(super::FightFestType::FightFestType::FIGHT_FEST_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.slot))?;
        }
        if self.DAAIBKIKBEJ != 0 {
            os.write_uint32(13, self.DAAIBKIKBEJ)?;
        }
        for v in &self.JJIPJBEAJIH {
            os.write_uint32(12, *v)?;
        };
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.id != 0 {
            os.write_uint32(7, self.id)?;
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

    fn new() -> StartFightFestCsReq {
        StartFightFestCsReq::new()
    }

    fn clear(&mut self) {
        self.slot = ::protobuf::EnumOrUnknown::new(super::FightFestType::FightFestType::FIGHT_FEST_TYPE_NONE);
        self.DAAIBKIKBEJ = 0;
        self.JJIPJBEAJIH.clear();
        self.avatar_list.clear();
        self.id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartFightFestCsReq {
        static instance: StartFightFestCsReq = StartFightFestCsReq {
            slot: ::protobuf::EnumOrUnknown::from_i32(0),
            DAAIBKIKBEJ: 0,
            JJIPJBEAJIH: ::std::vec::Vec::new(),
            avatar_list: ::std::vec::Vec::new(),
            id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartFightFestCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartFightFestCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartFightFestCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartFightFestCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19StartFightFestCsReq.proto\x1a\x11BNBMFMMBIAO.proto\x1a\x13FightFes\
    tType.proto\"\xbc\x01\n\x13StartFightFestCsReq\x12\"\n\x04slot\x18\x0f\
    \x20\x01(\x0e2\x0e.FightFestTypeR\x04slot\x12\x20\n\x0bDAAIBKIKBEJ\x18\r\
    \x20\x01(\rR\x0bDAAIBKIKBEJ\x12\x20\n\x0bJJIPJBEAJIH\x18\x0c\x20\x03(\rR\
    \x0bJJIPJBEAJIH\x12-\n\x0bavatar_list\x18\x0b\x20\x03(\x0b2\x0c.BNBMFMMB\
    IAOR\navatarList\x12\x0e\n\x02id\x18\x07\x20\x01(\rR\x02idb\x06proto3\
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
            deps.push(super::BNBMFMMBIAO::file_descriptor().clone());
            deps.push(super::FightFestType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartFightFestCsReq::generated_message_descriptor_data());
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
