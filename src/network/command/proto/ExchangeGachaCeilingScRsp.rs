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

//! Generated file from `ExchangeGachaCeilingScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ExchangeGachaCeilingScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExchangeGachaCeilingScRsp {
    // message fields
    // @@protoc_insertion_point(field:ExchangeGachaCeilingScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ExchangeGachaCeilingScRsp.transfer_item_list)
    pub transfer_item_list: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:ExchangeGachaCeilingScRsp.gacha_ceiling)
    pub gacha_ceiling: ::protobuf::MessageField<super::GachaCeiling::GachaCeiling>,
    // @@protoc_insertion_point(field:ExchangeGachaCeilingScRsp.gacha_type)
    pub gacha_type: u32,
    // @@protoc_insertion_point(field:ExchangeGachaCeilingScRsp.avatar_id)
    pub avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ExchangeGachaCeilingScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExchangeGachaCeilingScRsp {
    fn default() -> &'a ExchangeGachaCeilingScRsp {
        <ExchangeGachaCeilingScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ExchangeGachaCeilingScRsp {
    pub fn new() -> ExchangeGachaCeilingScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ExchangeGachaCeilingScRsp| { &m.retcode },
            |m: &mut ExchangeGachaCeilingScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "transfer_item_list",
            |m: &ExchangeGachaCeilingScRsp| { &m.transfer_item_list },
            |m: &mut ExchangeGachaCeilingScRsp| { &mut m.transfer_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GachaCeiling::GachaCeiling>(
            "gacha_ceiling",
            |m: &ExchangeGachaCeilingScRsp| { &m.gacha_ceiling },
            |m: &mut ExchangeGachaCeilingScRsp| { &mut m.gacha_ceiling },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_type",
            |m: &ExchangeGachaCeilingScRsp| { &m.gacha_type },
            |m: &mut ExchangeGachaCeilingScRsp| { &mut m.gacha_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &ExchangeGachaCeilingScRsp| { &m.avatar_id },
            |m: &mut ExchangeGachaCeilingScRsp| { &mut m.avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExchangeGachaCeilingScRsp>(
            "ExchangeGachaCeilingScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExchangeGachaCeilingScRsp {
    const NAME: &'static str = "ExchangeGachaCeilingScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.transfer_item_list)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gacha_ceiling)?;
                },
                120 => {
                    self.gacha_type = is.read_uint32()?;
                },
                80 => {
                    self.avatar_id = is.read_uint32()?;
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
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        if let Some(v) = self.transfer_item_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.gacha_ceiling.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.gacha_type != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.gacha_type);
        }
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        if let Some(v) = self.transfer_item_list.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.gacha_ceiling.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.gacha_type != 0 {
            os.write_uint32(15, self.gacha_type)?;
        }
        if self.avatar_id != 0 {
            os.write_uint32(10, self.avatar_id)?;
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

    fn new() -> ExchangeGachaCeilingScRsp {
        ExchangeGachaCeilingScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.transfer_item_list.clear();
        self.gacha_ceiling.clear();
        self.gacha_type = 0;
        self.avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExchangeGachaCeilingScRsp {
        static instance: ExchangeGachaCeilingScRsp = ExchangeGachaCeilingScRsp {
            retcode: 0,
            transfer_item_list: ::protobuf::MessageField::none(),
            gacha_ceiling: ::protobuf::MessageField::none(),
            gacha_type: 0,
            avatar_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExchangeGachaCeilingScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExchangeGachaCeilingScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExchangeGachaCeilingScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExchangeGachaCeilingScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fExchangeGachaCeilingScRsp.proto\x1a\x12GachaCeiling.proto\x1a\x0eI\
    temList.proto\"\xde\x01\n\x19ExchangeGachaCeilingScRsp\x12\x18\n\x07retc\
    ode\x18\x0b\x20\x01(\rR\x07retcode\x127\n\x12transfer_item_list\x18\x08\
    \x20\x01(\x0b2\t.ItemListR\x10transferItemList\x122\n\rgacha_ceiling\x18\
    \x0e\x20\x01(\x0b2\r.GachaCeilingR\x0cgachaCeiling\x12\x1d\n\ngacha_type\
    \x18\x0f\x20\x01(\rR\tgachaType\x12\x1b\n\tavatar_id\x18\n\x20\x01(\rR\
    \x08avatarIdb\x06proto3\
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
            deps.push(super::GachaCeiling::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ExchangeGachaCeilingScRsp::generated_message_descriptor_data());
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
