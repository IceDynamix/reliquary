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

//! Generated file from `ChessRogueCell.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChessRogueCell)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueCell {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueCell.is_unlocked)
    pub is_unlocked: bool,
    // @@protoc_insertion_point(field:ChessRogueCell.unlock)
    pub unlock: bool,
    // @@protoc_insertion_point(field:ChessRogueCell.stage_info)
    pub stage_info: ::protobuf::MessageField<super::CellAdvanceInfo::CellAdvanceInfo>,
    // @@protoc_insertion_point(field:ChessRogueCell.block_type)
    pub block_type: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.id)
    pub id: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.room_id)
    pub room_id: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.pos_y)
    pub pos_y: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.pos_x)
    pub pos_x: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.special_type)
    pub special_type: ::protobuf::EnumOrUnknown<super::ChessRogueCellSpecialType::ChessRogueCellSpecialType>,
    // @@protoc_insertion_point(field:ChessRogueCell.mark_type)
    pub mark_type: u32,
    // @@protoc_insertion_point(field:ChessRogueCell.cell_status)
    pub cell_status: ::protobuf::EnumOrUnknown<super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueCell.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueCell {
    fn default() -> &'a ChessRogueCell {
        <ChessRogueCell as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueCell {
    pub fn new() -> ChessRogueCell {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_unlocked",
            |m: &ChessRogueCell| { &m.is_unlocked },
            |m: &mut ChessRogueCell| { &mut m.is_unlocked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unlock",
            |m: &ChessRogueCell| { &m.unlock },
            |m: &mut ChessRogueCell| { &mut m.unlock },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CellAdvanceInfo::CellAdvanceInfo>(
            "stage_info",
            |m: &ChessRogueCell| { &m.stage_info },
            |m: &mut ChessRogueCell| { &mut m.stage_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "block_type",
            |m: &ChessRogueCell| { &m.block_type },
            |m: &mut ChessRogueCell| { &mut m.block_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &ChessRogueCell| { &m.id },
            |m: &mut ChessRogueCell| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "room_id",
            |m: &ChessRogueCell| { &m.room_id },
            |m: &mut ChessRogueCell| { &mut m.room_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pos_y",
            |m: &ChessRogueCell| { &m.pos_y },
            |m: &mut ChessRogueCell| { &mut m.pos_y },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pos_x",
            |m: &ChessRogueCell| { &m.pos_x },
            |m: &mut ChessRogueCell| { &mut m.pos_x },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "special_type",
            |m: &ChessRogueCell| { &m.special_type },
            |m: &mut ChessRogueCell| { &mut m.special_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mark_type",
            |m: &ChessRogueCell| { &m.mark_type },
            |m: &mut ChessRogueCell| { &mut m.mark_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cell_status",
            |m: &ChessRogueCell| { &m.cell_status },
            |m: &mut ChessRogueCell| { &mut m.cell_status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueCell>(
            "ChessRogueCell",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueCell {
    const NAME: &'static str = "ChessRogueCell";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.is_unlocked = is.read_bool()?;
                },
                120 => {
                    self.unlock = is.read_bool()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stage_info)?;
                },
                24 => {
                    self.block_type = is.read_uint32()?;
                },
                80 => {
                    self.id = is.read_uint32()?;
                },
                96 => {
                    self.room_id = is.read_uint32()?;
                },
                16 => {
                    self.pos_y = is.read_uint32()?;
                },
                104 => {
                    self.pos_x = is.read_uint32()?;
                },
                88 => {
                    self.special_type = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.mark_type = is.read_uint32()?;
                },
                48 => {
                    self.cell_status = is.read_enum_or_unknown()?;
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
        if self.unlock != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.stage_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.block_type != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.block_type);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.id);
        }
        if self.room_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.room_id);
        }
        if self.pos_y != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.pos_y);
        }
        if self.pos_x != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.pos_x);
        }
        if self.special_type != ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.special_type.value());
        }
        if self.mark_type != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.mark_type);
        }
        if self.cell_status != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            my_size += ::protobuf::rt::int32_size(6, self.cell_status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_unlocked != false {
            os.write_bool(8, self.is_unlocked)?;
        }
        if self.unlock != false {
            os.write_bool(15, self.unlock)?;
        }
        if let Some(v) = self.stage_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.block_type != 0 {
            os.write_uint32(3, self.block_type)?;
        }
        if self.id != 0 {
            os.write_uint32(10, self.id)?;
        }
        if self.room_id != 0 {
            os.write_uint32(12, self.room_id)?;
        }
        if self.pos_y != 0 {
            os.write_uint32(2, self.pos_y)?;
        }
        if self.pos_x != 0 {
            os.write_uint32(13, self.pos_x)?;
        }
        if self.special_type != ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.special_type))?;
        }
        if self.mark_type != 0 {
            os.write_uint32(7, self.mark_type)?;
        }
        if self.cell_status != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.cell_status))?;
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

    fn new() -> ChessRogueCell {
        ChessRogueCell::new()
    }

    fn clear(&mut self) {
        self.is_unlocked = false;
        self.unlock = false;
        self.stage_info.clear();
        self.block_type = 0;
        self.id = 0;
        self.room_id = 0;
        self.pos_y = 0;
        self.pos_x = 0;
        self.special_type = ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE);
        self.mark_type = 0;
        self.cell_status = ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueCell {
        static instance: ChessRogueCell = ChessRogueCell {
            is_unlocked: false,
            unlock: false,
            stage_info: ::protobuf::MessageField::none(),
            block_type: 0,
            id: 0,
            room_id: 0,
            pos_y: 0,
            pos_x: 0,
            special_type: ::protobuf::EnumOrUnknown::from_i32(0),
            mark_type: 0,
            cell_status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueCell {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueCell").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueCell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueCell {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14ChessRogueCell.proto\x1a\x15CellAdvanceInfo.proto\x1a\x1fChessRogu\
    eBoardCellStatus.proto\x1a\x1fChessRogueCellSpecialType.proto\"\x85\x03\
    \n\x0eChessRogueCell\x12\x1f\n\x0bis_unlocked\x18\x08\x20\x01(\x08R\nisU\
    nlocked\x12\x16\n\x06unlock\x18\x0f\x20\x01(\x08R\x06unlock\x12/\n\nstag\
    e_info\x18\x0e\x20\x01(\x0b2\x10.CellAdvanceInfoR\tstageInfo\x12\x1d\n\n\
    block_type\x18\x03\x20\x01(\rR\tblockType\x12\x0e\n\x02id\x18\n\x20\x01(\
    \rR\x02id\x12\x17\n\x07room_id\x18\x0c\x20\x01(\rR\x06roomId\x12\x13\n\
    \x05pos_y\x18\x02\x20\x01(\rR\x04posY\x12\x13\n\x05pos_x\x18\r\x20\x01(\
    \rR\x04posX\x12=\n\x0cspecial_type\x18\x0b\x20\x01(\x0e2\x1a.ChessRogueC\
    ellSpecialTypeR\x0bspecialType\x12\x1b\n\tmark_type\x18\x07\x20\x01(\rR\
    \x08markType\x12;\n\x0bcell_status\x18\x06\x20\x01(\x0e2\x1a.ChessRogueB\
    oardCellStatusR\ncellStatusb\x06proto3\
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
            deps.push(super::CellAdvanceInfo::file_descriptor().clone());
            deps.push(super::ChessRogueBoardCellStatus::file_descriptor().clone());
            deps.push(super::ChessRogueCellSpecialType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueCell::generated_message_descriptor_data());
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
