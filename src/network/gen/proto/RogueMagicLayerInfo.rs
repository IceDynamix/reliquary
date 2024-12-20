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

//! Generated file from `RogueMagicLayerInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueMagicLayerInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMagicLayerInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueMagicLayerInfo.status)
    pub status: ::protobuf::EnumOrUnknown<super::RogueMagicLayerStatus::RogueMagicLayerStatus>,
    // @@protoc_insertion_point(field:RogueMagicLayerInfo.layer_id)
    pub layer_id: u32,
    // @@protoc_insertion_point(field:RogueMagicLayerInfo.ext_room_list)
    pub ext_room_list: ::std::vec::Vec<super::RogueMagicRoomInfo::RogueMagicRoomInfo>,
    // @@protoc_insertion_point(field:RogueMagicLayerInfo.level_index)
    pub level_index: u32,
    // @@protoc_insertion_point(field:RogueMagicLayerInfo.cur_room_index)
    pub cur_room_index: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMagicLayerInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMagicLayerInfo {
    fn default() -> &'a RogueMagicLayerInfo {
        <RogueMagicLayerInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueMagicLayerInfo {
    pub fn new() -> RogueMagicLayerInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &RogueMagicLayerInfo| { &m.status },
            |m: &mut RogueMagicLayerInfo| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "layer_id",
            |m: &RogueMagicLayerInfo| { &m.layer_id },
            |m: &mut RogueMagicLayerInfo| { &mut m.layer_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ext_room_list",
            |m: &RogueMagicLayerInfo| { &m.ext_room_list },
            |m: &mut RogueMagicLayerInfo| { &mut m.ext_room_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_index",
            |m: &RogueMagicLayerInfo| { &m.level_index },
            |m: &mut RogueMagicLayerInfo| { &mut m.level_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_room_index",
            |m: &RogueMagicLayerInfo| { &m.cur_room_index },
            |m: &mut RogueMagicLayerInfo| { &mut m.cur_room_index },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMagicLayerInfo>(
            "RogueMagicLayerInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMagicLayerInfo {
    const NAME: &'static str = "RogueMagicLayerInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.layer_id = is.read_uint32()?;
                },
                26 => {
                    self.ext_room_list.push(is.read_message()?);
                },
                32 => {
                    self.level_index = is.read_uint32()?;
                },
                8 => {
                    self.cur_room_index = is.read_uint32()?;
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
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueMagicLayerStatus::RogueMagicLayerStatus::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.status.value());
        }
        if self.layer_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.layer_id);
        }
        for value in &self.ext_room_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level_index != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.level_index);
        }
        if self.cur_room_index != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.cur_room_index);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueMagicLayerStatus::RogueMagicLayerStatus::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.layer_id != 0 {
            os.write_uint32(7, self.layer_id)?;
        }
        for v in &self.ext_room_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.level_index != 0 {
            os.write_uint32(4, self.level_index)?;
        }
        if self.cur_room_index != 0 {
            os.write_uint32(1, self.cur_room_index)?;
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

    fn new() -> RogueMagicLayerInfo {
        RogueMagicLayerInfo::new()
    }

    fn clear(&mut self) {
        self.status = ::protobuf::EnumOrUnknown::new(super::RogueMagicLayerStatus::RogueMagicLayerStatus::ROGUE_MAGIC_LAYER_STATUS_NONE);
        self.layer_id = 0;
        self.ext_room_list.clear();
        self.level_index = 0;
        self.cur_room_index = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMagicLayerInfo {
        static instance: RogueMagicLayerInfo = RogueMagicLayerInfo {
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            layer_id: 0,
            ext_room_list: ::std::vec::Vec::new(),
            level_index: 0,
            cur_room_index: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueMagicLayerInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMagicLayerInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMagicLayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMagicLayerInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19RogueMagicLayerInfo.proto\x1a\x1bRogueMagicLayerStatus.proto\x1a\
    \x18RogueMagicRoomInfo.proto\"\xe0\x01\n\x13RogueMagicLayerInfo\x12.\n\
    \x06status\x18\r\x20\x01(\x0e2\x16.RogueMagicLayerStatusR\x06status\x12\
    \x19\n\x08layer_id\x18\x07\x20\x01(\rR\x07layerId\x127\n\rext_room_list\
    \x18\x03\x20\x03(\x0b2\x13.RogueMagicRoomInfoR\x0bextRoomList\x12\x1f\n\
    \x0blevel_index\x18\x04\x20\x01(\rR\nlevelIndex\x12$\n\x0ecur_room_index\
    \x18\x01\x20\x01(\rR\x0ccurRoomIndexB\x15\n\x13emu.lunarcore.protob\x06p\
    roto3\
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
            deps.push(super::RogueMagicLayerStatus::file_descriptor().clone());
            deps.push(super::RogueMagicRoomInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueMagicLayerInfo::generated_message_descriptor_data());
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
