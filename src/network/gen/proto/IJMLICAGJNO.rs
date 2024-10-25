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

//! Generated file from `IJMLICAGJNO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IJMLICAGJNO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IJMLICAGJNO {
    // message fields
    // @@protoc_insertion_point(field:IJMLICAGJNO.NDGHLIFGHKK)
    pub NDGHLIFGHKK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IJMLICAGJNO.LMEEBKDOBKI)
    pub LMEEBKDOBKI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IJMLICAGJNO.LMIJCPOICHI)
    pub LMIJCPOICHI: u32,
    // @@protoc_insertion_point(field:IJMLICAGJNO.DGFPMKGGFAG)
    pub DGFPMKGGFAG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IJMLICAGJNO.AKLCENCMMLF)
    pub AKLCENCMMLF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IJMLICAGJNO.PEHIDLKDCCJ)
    pub PEHIDLKDCCJ: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:IJMLICAGJNO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IJMLICAGJNO {
    fn default() -> &'a IJMLICAGJNO {
        <IJMLICAGJNO as ::protobuf::Message>::default_instance()
    }
}

impl IJMLICAGJNO {
    pub fn new() -> IJMLICAGJNO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NDGHLIFGHKK",
            |m: &IJMLICAGJNO| { &m.NDGHLIFGHKK },
            |m: &mut IJMLICAGJNO| { &mut m.NDGHLIFGHKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMEEBKDOBKI",
            |m: &IJMLICAGJNO| { &m.LMEEBKDOBKI },
            |m: &mut IJMLICAGJNO| { &mut m.LMEEBKDOBKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMIJCPOICHI",
            |m: &IJMLICAGJNO| { &m.LMIJCPOICHI },
            |m: &mut IJMLICAGJNO| { &mut m.LMIJCPOICHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DGFPMKGGFAG",
            |m: &IJMLICAGJNO| { &m.DGFPMKGGFAG },
            |m: &mut IJMLICAGJNO| { &mut m.DGFPMKGGFAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AKLCENCMMLF",
            |m: &IJMLICAGJNO| { &m.AKLCENCMMLF },
            |m: &mut IJMLICAGJNO| { &mut m.AKLCENCMMLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PEHIDLKDCCJ",
            |m: &IJMLICAGJNO| { &m.PEHIDLKDCCJ },
            |m: &mut IJMLICAGJNO| { &mut m.PEHIDLKDCCJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IJMLICAGJNO>(
            "IJMLICAGJNO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IJMLICAGJNO {
    const NAME: &'static str = "IJMLICAGJNO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.NDGHLIFGHKK)?;
                },
                24 => {
                    self.NDGHLIFGHKK.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.LMEEBKDOBKI)?;
                },
                56 => {
                    self.LMEEBKDOBKI.push(is.read_uint32()?);
                },
                112 => {
                    self.LMIJCPOICHI = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.DGFPMKGGFAG)?;
                },
                64 => {
                    self.DGFPMKGGFAG.push(is.read_uint32()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.AKLCENCMMLF)?;
                },
                40 => {
                    self.AKLCENCMMLF.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.PEHIDLKDCCJ)?;
                },
                80 => {
                    self.PEHIDLKDCCJ.push(is.read_uint32()?);
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
        for value in &self.NDGHLIFGHKK {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.LMEEBKDOBKI {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        if self.LMIJCPOICHI != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LMIJCPOICHI);
        }
        for value in &self.DGFPMKGGFAG {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.AKLCENCMMLF {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.PEHIDLKDCCJ {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NDGHLIFGHKK {
            os.write_uint32(3, *v)?;
        };
        for v in &self.LMEEBKDOBKI {
            os.write_uint32(7, *v)?;
        };
        if self.LMIJCPOICHI != 0 {
            os.write_uint32(14, self.LMIJCPOICHI)?;
        }
        for v in &self.DGFPMKGGFAG {
            os.write_uint32(8, *v)?;
        };
        for v in &self.AKLCENCMMLF {
            os.write_uint32(5, *v)?;
        };
        for v in &self.PEHIDLKDCCJ {
            os.write_uint32(10, *v)?;
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

    fn new() -> IJMLICAGJNO {
        IJMLICAGJNO::new()
    }

    fn clear(&mut self) {
        self.NDGHLIFGHKK.clear();
        self.LMEEBKDOBKI.clear();
        self.LMIJCPOICHI = 0;
        self.DGFPMKGGFAG.clear();
        self.AKLCENCMMLF.clear();
        self.PEHIDLKDCCJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IJMLICAGJNO {
        static instance: IJMLICAGJNO = IJMLICAGJNO {
            NDGHLIFGHKK: ::std::vec::Vec::new(),
            LMEEBKDOBKI: ::std::vec::Vec::new(),
            LMIJCPOICHI: 0,
            DGFPMKGGFAG: ::std::vec::Vec::new(),
            AKLCENCMMLF: ::std::vec::Vec::new(),
            PEHIDLKDCCJ: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IJMLICAGJNO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IJMLICAGJNO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IJMLICAGJNO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IJMLICAGJNO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IJMLICAGJNO.proto\"\xd9\x01\n\x0bIJMLICAGJNO\x12\x20\n\x0bNDGHLIFG\
    HKK\x18\x03\x20\x03(\rR\x0bNDGHLIFGHKK\x12\x20\n\x0bLMEEBKDOBKI\x18\x07\
    \x20\x03(\rR\x0bLMEEBKDOBKI\x12\x20\n\x0bLMIJCPOICHI\x18\x0e\x20\x01(\rR\
    \x0bLMIJCPOICHI\x12\x20\n\x0bDGFPMKGGFAG\x18\x08\x20\x03(\rR\x0bDGFPMKGG\
    FAG\x12\x20\n\x0bAKLCENCMMLF\x18\x05\x20\x03(\rR\x0bAKLCENCMMLF\x12\x20\
    \n\x0bPEHIDLKDCCJ\x18\n\x20\x03(\rR\x0bPEHIDLKDCCJb\x06proto3\
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
            messages.push(IJMLICAGJNO::generated_message_descriptor_data());
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
