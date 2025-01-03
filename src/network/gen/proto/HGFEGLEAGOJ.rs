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

//! Generated file from `HGFEGLEAGOJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HGFEGLEAGOJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HGFEGLEAGOJ {
    // message fields
    // @@protoc_insertion_point(field:HGFEGLEAGOJ.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:HGFEGLEAGOJ.OCHJFCHENBE)
    pub OCHJFCHENBE: u32,
    // @@protoc_insertion_point(field:HGFEGLEAGOJ.PHLBECLKBHH)
    pub PHLBECLKBHH: u32,
    // @@protoc_insertion_point(field:HGFEGLEAGOJ.BAIPBKIPOOM)
    pub BAIPBKIPOOM: u32,
    // @@protoc_insertion_point(field:HGFEGLEAGOJ.AILDDAKLBKK)
    pub AILDDAKLBKK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HGFEGLEAGOJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HGFEGLEAGOJ {
    fn default() -> &'a HGFEGLEAGOJ {
        <HGFEGLEAGOJ as ::protobuf::Message>::default_instance()
    }
}

impl HGFEGLEAGOJ {
    pub fn new() -> HGFEGLEAGOJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &HGFEGLEAGOJ| { &m.JKOCJIMAGBN },
            |m: &mut HGFEGLEAGOJ| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OCHJFCHENBE",
            |m: &HGFEGLEAGOJ| { &m.OCHJFCHENBE },
            |m: &mut HGFEGLEAGOJ| { &mut m.OCHJFCHENBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHLBECLKBHH",
            |m: &HGFEGLEAGOJ| { &m.PHLBECLKBHH },
            |m: &mut HGFEGLEAGOJ| { &mut m.PHLBECLKBHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BAIPBKIPOOM",
            |m: &HGFEGLEAGOJ| { &m.BAIPBKIPOOM },
            |m: &mut HGFEGLEAGOJ| { &mut m.BAIPBKIPOOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AILDDAKLBKK",
            |m: &HGFEGLEAGOJ| { &m.AILDDAKLBKK },
            |m: &mut HGFEGLEAGOJ| { &mut m.AILDDAKLBKK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HGFEGLEAGOJ>(
            "HGFEGLEAGOJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HGFEGLEAGOJ {
    const NAME: &'static str = "HGFEGLEAGOJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                48 => {
                    self.OCHJFCHENBE = is.read_uint32()?;
                },
                24 => {
                    self.PHLBECLKBHH = is.read_uint32()?;
                },
                88 => {
                    self.BAIPBKIPOOM = is.read_uint32()?;
                },
                32 => {
                    self.AILDDAKLBKK = is.read_uint32()?;
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
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JKOCJIMAGBN);
        }
        if self.OCHJFCHENBE != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.OCHJFCHENBE);
        }
        if self.PHLBECLKBHH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.PHLBECLKBHH);
        }
        if self.BAIPBKIPOOM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BAIPBKIPOOM);
        }
        if self.AILDDAKLBKK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.AILDDAKLBKK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(14, self.JKOCJIMAGBN)?;
        }
        if self.OCHJFCHENBE != 0 {
            os.write_uint32(6, self.OCHJFCHENBE)?;
        }
        if self.PHLBECLKBHH != 0 {
            os.write_uint32(3, self.PHLBECLKBHH)?;
        }
        if self.BAIPBKIPOOM != 0 {
            os.write_uint32(11, self.BAIPBKIPOOM)?;
        }
        if self.AILDDAKLBKK != 0 {
            os.write_uint32(4, self.AILDDAKLBKK)?;
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

    fn new() -> HGFEGLEAGOJ {
        HGFEGLEAGOJ::new()
    }

    fn clear(&mut self) {
        self.JKOCJIMAGBN = 0;
        self.OCHJFCHENBE = 0;
        self.PHLBECLKBHH = 0;
        self.BAIPBKIPOOM = 0;
        self.AILDDAKLBKK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HGFEGLEAGOJ {
        static instance: HGFEGLEAGOJ = HGFEGLEAGOJ {
            JKOCJIMAGBN: 0,
            OCHJFCHENBE: 0,
            PHLBECLKBHH: 0,
            BAIPBKIPOOM: 0,
            AILDDAKLBKK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HGFEGLEAGOJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HGFEGLEAGOJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HGFEGLEAGOJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HGFEGLEAGOJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HGFEGLEAGOJ.proto\"\xb7\x01\n\x0bHGFEGLEAGOJ\x12\x20\n\x0bJKOCJIMA\
    GBN\x18\x0e\x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\n\x0bOCHJFCHENBE\x18\x06\
    \x20\x01(\rR\x0bOCHJFCHENBE\x12\x20\n\x0bPHLBECLKBHH\x18\x03\x20\x01(\rR\
    \x0bPHLBECLKBHH\x12\x20\n\x0bBAIPBKIPOOM\x18\x0b\x20\x01(\rR\x0bBAIPBKIP\
    OOM\x12\x20\n\x0bAILDDAKLBKK\x18\x04\x20\x01(\rR\x0bAILDDAKLBKKb\x06prot\
    o3\
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
            messages.push(HGFEGLEAGOJ::generated_message_descriptor_data());
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
