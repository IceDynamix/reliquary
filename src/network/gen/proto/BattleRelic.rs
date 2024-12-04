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

//! Generated file from `BattleRelic.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattleRelic)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattleRelic {
    // message fields
    // @@protoc_insertion_point(field:BattleRelic.id)
    pub id: u32,
    // @@protoc_insertion_point(field:BattleRelic.level)
    pub level: u32,
    // @@protoc_insertion_point(field:BattleRelic.main_affix_id)
    pub main_affix_id: u32,
    // @@protoc_insertion_point(field:BattleRelic.sub_affix_list)
    pub sub_affix_list: ::std::vec::Vec<super::RelicAffix::RelicAffix>,
    // @@protoc_insertion_point(field:BattleRelic.unique_id)
    pub unique_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BattleRelic.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattleRelic {
    fn default() -> &'a BattleRelic {
        <BattleRelic as ::protobuf::Message>::default_instance()
    }
}

impl BattleRelic {
    pub fn new() -> BattleRelic {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &BattleRelic| { &m.id },
            |m: &mut BattleRelic| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &BattleRelic| { &m.level },
            |m: &mut BattleRelic| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_affix_id",
            |m: &BattleRelic| { &m.main_affix_id },
            |m: &mut BattleRelic| { &mut m.main_affix_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sub_affix_list",
            |m: &BattleRelic| { &m.sub_affix_list },
            |m: &mut BattleRelic| { &mut m.sub_affix_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &BattleRelic| { &m.unique_id },
            |m: &mut BattleRelic| { &mut m.unique_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattleRelic>(
            "BattleRelic",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattleRelic {
    const NAME: &'static str = "BattleRelic";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.main_affix_id = is.read_uint32()?;
                },
                34 => {
                    self.sub_affix_list.push(is.read_message()?);
                },
                40 => {
                    self.unique_id = is.read_uint32()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.main_affix_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.main_affix_id);
        }
        for value in &self.sub_affix_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.unique_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.main_affix_id != 0 {
            os.write_uint32(3, self.main_affix_id)?;
        }
        for v in &self.sub_affix_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.unique_id != 0 {
            os.write_uint32(5, self.unique_id)?;
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

    fn new() -> BattleRelic {
        BattleRelic::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.level = 0;
        self.main_affix_id = 0;
        self.sub_affix_list.clear();
        self.unique_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattleRelic {
        static instance: BattleRelic = BattleRelic {
            id: 0,
            level: 0,
            main_affix_id: 0,
            sub_affix_list: ::std::vec::Vec::new(),
            unique_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattleRelic {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattleRelic").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattleRelic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattleRelic {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BattleRelic.proto\x1a\x10RelicAffix.proto\"\xa7\x01\n\x0bBattleRel\
    ic\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x14\n\x05level\x18\x02\
    \x20\x01(\rR\x05level\x12\"\n\rmain_affix_id\x18\x03\x20\x01(\rR\x0bmain\
    AffixId\x121\n\x0esub_affix_list\x18\x04\x20\x03(\x0b2\x0b.RelicAffixR\
    \x0csubAffixList\x12\x1b\n\tunique_id\x18\x05\x20\x01(\rR\x08uniqueIdB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::RelicAffix::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattleRelic::generated_message_descriptor_data());
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