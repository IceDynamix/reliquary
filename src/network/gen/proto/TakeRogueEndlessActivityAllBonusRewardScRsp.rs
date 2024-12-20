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

//! Generated file from `TakeRogueEndlessActivityAllBonusRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeRogueEndlessActivityAllBonusRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeRogueEndlessActivityAllBonusRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityAllBonusRewardScRsp.FEIBKNDNJAH)
    pub FEIBKNDNJAH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityAllBonusRewardScRsp.ELPMNKHEPKJ)
    pub ELPMNKHEPKJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityAllBonusRewardScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:TakeRogueEndlessActivityAllBonusRewardScRsp.FEJEDBDGPAF)
    pub FEJEDBDGPAF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeRogueEndlessActivityAllBonusRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeRogueEndlessActivityAllBonusRewardScRsp {
    fn default() -> &'a TakeRogueEndlessActivityAllBonusRewardScRsp {
        <TakeRogueEndlessActivityAllBonusRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeRogueEndlessActivityAllBonusRewardScRsp {
    pub fn new() -> TakeRogueEndlessActivityAllBonusRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FEIBKNDNJAH",
            |m: &TakeRogueEndlessActivityAllBonusRewardScRsp| { &m.FEIBKNDNJAH },
            |m: &mut TakeRogueEndlessActivityAllBonusRewardScRsp| { &mut m.FEIBKNDNJAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "ELPMNKHEPKJ",
            |m: &TakeRogueEndlessActivityAllBonusRewardScRsp| { &m.ELPMNKHEPKJ },
            |m: &mut TakeRogueEndlessActivityAllBonusRewardScRsp| { &mut m.ELPMNKHEPKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &TakeRogueEndlessActivityAllBonusRewardScRsp| { &m.ADADHIHDHJC },
            |m: &mut TakeRogueEndlessActivityAllBonusRewardScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FEJEDBDGPAF",
            |m: &TakeRogueEndlessActivityAllBonusRewardScRsp| { &m.FEJEDBDGPAF },
            |m: &mut TakeRogueEndlessActivityAllBonusRewardScRsp| { &mut m.FEJEDBDGPAF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeRogueEndlessActivityAllBonusRewardScRsp>(
            "TakeRogueEndlessActivityAllBonusRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeRogueEndlessActivityAllBonusRewardScRsp {
    const NAME: &'static str = "TakeRogueEndlessActivityAllBonusRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.FEIBKNDNJAH)?;
                },
                8 => {
                    self.FEIBKNDNJAH.push(is.read_uint32()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ELPMNKHEPKJ)?;
                },
                16 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                112 => {
                    self.FEJEDBDGPAF = is.read_uint32()?;
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
        for value in &self.FEIBKNDNJAH {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ADADHIHDHJC);
        }
        if self.FEJEDBDGPAF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.FEJEDBDGPAF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.FEIBKNDNJAH {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(2, self.ADADHIHDHJC)?;
        }
        if self.FEJEDBDGPAF != 0 {
            os.write_uint32(14, self.FEJEDBDGPAF)?;
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

    fn new() -> TakeRogueEndlessActivityAllBonusRewardScRsp {
        TakeRogueEndlessActivityAllBonusRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.FEIBKNDNJAH.clear();
        self.ELPMNKHEPKJ.clear();
        self.ADADHIHDHJC = 0;
        self.FEJEDBDGPAF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeRogueEndlessActivityAllBonusRewardScRsp {
        static instance: TakeRogueEndlessActivityAllBonusRewardScRsp = TakeRogueEndlessActivityAllBonusRewardScRsp {
            FEIBKNDNJAH: ::std::vec::Vec::new(),
            ELPMNKHEPKJ: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            FEJEDBDGPAF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeRogueEndlessActivityAllBonusRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeRogueEndlessActivityAllBonusRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeRogueEndlessActivityAllBonusRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeRogueEndlessActivityAllBonusRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1TakeRogueEndlessActivityAllBonusRewardScRsp.proto\x1a\x0eItemList.pro\
    to\"\xc0\x01\n+TakeRogueEndlessActivityAllBonusRewardScRsp\x12\x20\n\x0b\
    FEIBKNDNJAH\x18\x01\x20\x03(\rR\x0bFEIBKNDNJAH\x12+\n\x0bELPMNKHEPKJ\x18\
    \x0b\x20\x01(\x0b2\t.ItemListR\x0bELPMNKHEPKJ\x12\x20\n\x0bADADHIHDHJC\
    \x18\x02\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bFEJEDBDGPAF\x18\x0e\x20\
    \x01(\rR\x0bFEJEDBDGPAFb\x06proto3\
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
            messages.push(TakeRogueEndlessActivityAllBonusRewardScRsp::generated_message_descriptor_data());
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
