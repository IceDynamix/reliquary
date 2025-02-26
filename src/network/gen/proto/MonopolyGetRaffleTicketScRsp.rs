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

//! Generated file from `MonopolyGetRaffleTicketScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MonopolyGetRaffleTicketScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MonopolyGetRaffleTicketScRsp {
    // message fields
    // @@protoc_insertion_point(field:MonopolyGetRaffleTicketScRsp.HHJPBLEKAPN)
    pub HHJPBLEKAPN: u32,
    // @@protoc_insertion_point(field:MonopolyGetRaffleTicketScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:MonopolyGetRaffleTicketScRsp.BLMEDLNOKEI)
    pub BLMEDLNOKEI: ::std::vec::Vec<super::LAILNIGFPOO::LAILNIGFPOO>,
    // special fields
    // @@protoc_insertion_point(special_field:MonopolyGetRaffleTicketScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MonopolyGetRaffleTicketScRsp {
    fn default() -> &'a MonopolyGetRaffleTicketScRsp {
        <MonopolyGetRaffleTicketScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MonopolyGetRaffleTicketScRsp {
    pub fn new() -> MonopolyGetRaffleTicketScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHJPBLEKAPN",
            |m: &MonopolyGetRaffleTicketScRsp| { &m.HHJPBLEKAPN },
            |m: &mut MonopolyGetRaffleTicketScRsp| { &mut m.HHJPBLEKAPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MonopolyGetRaffleTicketScRsp| { &m.retcode },
            |m: &mut MonopolyGetRaffleTicketScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BLMEDLNOKEI",
            |m: &MonopolyGetRaffleTicketScRsp| { &m.BLMEDLNOKEI },
            |m: &mut MonopolyGetRaffleTicketScRsp| { &mut m.BLMEDLNOKEI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MonopolyGetRaffleTicketScRsp>(
            "MonopolyGetRaffleTicketScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MonopolyGetRaffleTicketScRsp {
    const NAME: &'static str = "MonopolyGetRaffleTicketScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.HHJPBLEKAPN = is.read_uint32()?;
                },
                24 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    self.BLMEDLNOKEI.push(is.read_message()?);
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
        if self.HHJPBLEKAPN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.HHJPBLEKAPN);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        for value in &self.BLMEDLNOKEI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HHJPBLEKAPN != 0 {
            os.write_uint32(8, self.HHJPBLEKAPN)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
        }
        for v in &self.BLMEDLNOKEI {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> MonopolyGetRaffleTicketScRsp {
        MonopolyGetRaffleTicketScRsp::new()
    }

    fn clear(&mut self) {
        self.HHJPBLEKAPN = 0;
        self.retcode = 0;
        self.BLMEDLNOKEI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MonopolyGetRaffleTicketScRsp {
        static instance: MonopolyGetRaffleTicketScRsp = MonopolyGetRaffleTicketScRsp {
            HHJPBLEKAPN: 0,
            retcode: 0,
            BLMEDLNOKEI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MonopolyGetRaffleTicketScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MonopolyGetRaffleTicketScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MonopolyGetRaffleTicketScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonopolyGetRaffleTicketScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"MonopolyGetRaffleTicketScRsp.proto\x1a\x11LAILNIGFPOO.proto\"\x8a\
    \x01\n\x1cMonopolyGetRaffleTicketScRsp\x12\x20\n\x0bHHJPBLEKAPN\x18\x08\
    \x20\x01(\rR\x0bHHJPBLEKAPN\x12\x18\n\x07retcode\x18\x03\x20\x01(\rR\x07\
    retcode\x12.\n\x0bBLMEDLNOKEI\x18\x0f\x20\x03(\x0b2\x0c.LAILNIGFPOOR\x0b\
    BLMEDLNOKEIb\x06proto3\
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
            deps.push(super::LAILNIGFPOO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MonopolyGetRaffleTicketScRsp::generated_message_descriptor_data());
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
