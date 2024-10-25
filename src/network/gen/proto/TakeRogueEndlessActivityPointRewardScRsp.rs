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

//! Generated file from `TakeRogueEndlessActivityPointRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeRogueEndlessActivityPointRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeRogueEndlessActivityPointRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.APGBCNNPHMB)
    pub APGBCNNPHMB: u32,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.LPDNAMLHGNJ)
    pub LPDNAMLHGNJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.MGCIACFKBFM)
    pub MGCIACFKBFM: bool,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityPointRewardScRsp.LHFDCBAFJGK)
    pub LHFDCBAFJGK: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:TakeRogueEndlessActivityPointRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeRogueEndlessActivityPointRewardScRsp {
    fn default() -> &'a TakeRogueEndlessActivityPointRewardScRsp {
        <TakeRogueEndlessActivityPointRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeRogueEndlessActivityPointRewardScRsp {
    pub fn new() -> TakeRogueEndlessActivityPointRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APGBCNNPHMB",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.APGBCNNPHMB },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.APGBCNNPHMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "LPDNAMLHGNJ",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.LPDNAMLHGNJ },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.LPDNAMLHGNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.retcode },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.level },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGCIACFKBFM",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.MGCIACFKBFM },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.MGCIACFKBFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LHFDCBAFJGK",
            |m: &TakeRogueEndlessActivityPointRewardScRsp| { &m.LHFDCBAFJGK },
            |m: &mut TakeRogueEndlessActivityPointRewardScRsp| { &mut m.LHFDCBAFJGK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeRogueEndlessActivityPointRewardScRsp>(
            "TakeRogueEndlessActivityPointRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeRogueEndlessActivityPointRewardScRsp {
    const NAME: &'static str = "TakeRogueEndlessActivityPointRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.APGBCNNPHMB = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LPDNAMLHGNJ)?;
                },
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                64 => {
                    self.level = is.read_uint32()?;
                },
                80 => {
                    self.MGCIACFKBFM = is.read_bool()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.LHFDCBAFJGK)?;
                },
                56 => {
                    self.LHFDCBAFJGK.push(is.read_uint32()?);
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
        if self.APGBCNNPHMB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.APGBCNNPHMB);
        }
        if let Some(v) = self.LPDNAMLHGNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.level);
        }
        if self.MGCIACFKBFM != false {
            my_size += 1 + 1;
        }
        for value in &self.LHFDCBAFJGK {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.APGBCNNPHMB != 0 {
            os.write_uint32(12, self.APGBCNNPHMB)?;
        }
        if let Some(v) = self.LPDNAMLHGNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if self.level != 0 {
            os.write_uint32(8, self.level)?;
        }
        if self.MGCIACFKBFM != false {
            os.write_bool(10, self.MGCIACFKBFM)?;
        }
        for v in &self.LHFDCBAFJGK {
            os.write_uint32(7, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> TakeRogueEndlessActivityPointRewardScRsp {
        TakeRogueEndlessActivityPointRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.APGBCNNPHMB = 0;
        self.LPDNAMLHGNJ.clear();
        self.retcode = 0;
        self.level = 0;
        self.MGCIACFKBFM = false;
        self.LHFDCBAFJGK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeRogueEndlessActivityPointRewardScRsp {
        static instance: TakeRogueEndlessActivityPointRewardScRsp = TakeRogueEndlessActivityPointRewardScRsp {
            APGBCNNPHMB: 0,
            LPDNAMLHGNJ: ::protobuf::MessageField::none(),
            retcode: 0,
            level: 0,
            MGCIACFKBFM: false,
            LHFDCBAFJGK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeRogueEndlessActivityPointRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeRogueEndlessActivityPointRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeRogueEndlessActivityPointRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeRogueEndlessActivityPointRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.TakeRogueEndlessActivityPointRewardScRsp.proto\x1a\x0eItemList.proto\
    \"\xed\x01\n(TakeRogueEndlessActivityPointRewardScRsp\x12\x20\n\x0bAPGBC\
    NNPHMB\x18\x0c\x20\x01(\rR\x0bAPGBCNNPHMB\x12+\n\x0bLPDNAMLHGNJ\x18\t\
    \x20\x01(\x0b2\t.ItemListR\x0bLPDNAMLHGNJ\x12\x18\n\x07retcode\x18\x02\
    \x20\x01(\rR\x07retcode\x12\x14\n\x05level\x18\x08\x20\x01(\rR\x05level\
    \x12\x20\n\x0bMGCIACFKBFM\x18\n\x20\x01(\x08R\x0bMGCIACFKBFM\x12\x20\n\
    \x0bLHFDCBAFJGK\x18\x07\x20\x03(\rR\x0bLHFDCBAFJGKb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeRogueEndlessActivityPointRewardScRsp::generated_message_descriptor_data());
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
