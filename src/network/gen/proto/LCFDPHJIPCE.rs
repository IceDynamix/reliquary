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

//! Generated file from `LCFDPHJIPCE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LCFDPHJIPCE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LCFDPHJIPCE {
    // message fields
    // @@protoc_insertion_point(field:LCFDPHJIPCE.PBCLKHACEAN)
    pub PBCLKHACEAN: u32,
    // @@protoc_insertion_point(field:LCFDPHJIPCE.OFAGGKBMPJN)
    pub OFAGGKBMPJN: u32,
    // @@protoc_insertion_point(field:LCFDPHJIPCE.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:LCFDPHJIPCE.IFCOBCLMEIA)
    pub IFCOBCLMEIA: u32,
    // @@protoc_insertion_point(field:LCFDPHJIPCE.HCEFGBHKLIH)
    pub HCEFGBHKLIH: u32,
    // @@protoc_insertion_point(field:LCFDPHJIPCE.CILFOCBCCNK)
    pub CILFOCBCCNK: ::std::vec::Vec<super::CDIIDOLJILK::CDIIDOLJILK>,
    // special fields
    // @@protoc_insertion_point(special_field:LCFDPHJIPCE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LCFDPHJIPCE {
    fn default() -> &'a LCFDPHJIPCE {
        <LCFDPHJIPCE as ::protobuf::Message>::default_instance()
    }
}

impl LCFDPHJIPCE {
    pub fn new() -> LCFDPHJIPCE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBCLKHACEAN",
            |m: &LCFDPHJIPCE| { &m.PBCLKHACEAN },
            |m: &mut LCFDPHJIPCE| { &mut m.PBCLKHACEAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFAGGKBMPJN",
            |m: &LCFDPHJIPCE| { &m.OFAGGKBMPJN },
            |m: &mut LCFDPHJIPCE| { &mut m.OFAGGKBMPJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &LCFDPHJIPCE| { &m.JKOCJIMAGBN },
            |m: &mut LCFDPHJIPCE| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFCOBCLMEIA",
            |m: &LCFDPHJIPCE| { &m.IFCOBCLMEIA },
            |m: &mut LCFDPHJIPCE| { &mut m.IFCOBCLMEIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCEFGBHKLIH",
            |m: &LCFDPHJIPCE| { &m.HCEFGBHKLIH },
            |m: &mut LCFDPHJIPCE| { &mut m.HCEFGBHKLIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CILFOCBCCNK",
            |m: &LCFDPHJIPCE| { &m.CILFOCBCCNK },
            |m: &mut LCFDPHJIPCE| { &mut m.CILFOCBCCNK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LCFDPHJIPCE>(
            "LCFDPHJIPCE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LCFDPHJIPCE {
    const NAME: &'static str = "LCFDPHJIPCE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.PBCLKHACEAN = is.read_uint32()?;
                },
                104 => {
                    self.OFAGGKBMPJN = is.read_uint32()?;
                },
                16 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                48 => {
                    self.IFCOBCLMEIA = is.read_uint32()?;
                },
                80 => {
                    self.HCEFGBHKLIH = is.read_uint32()?;
                },
                90 => {
                    self.CILFOCBCCNK.push(is.read_message()?);
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
        if self.PBCLKHACEAN != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PBCLKHACEAN);
        }
        if self.OFAGGKBMPJN != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OFAGGKBMPJN);
        }
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.JKOCJIMAGBN);
        }
        if self.IFCOBCLMEIA != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IFCOBCLMEIA);
        }
        if self.HCEFGBHKLIH != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.HCEFGBHKLIH);
        }
        for value in &self.CILFOCBCCNK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PBCLKHACEAN != 0 {
            os.write_uint32(1, self.PBCLKHACEAN)?;
        }
        if self.OFAGGKBMPJN != 0 {
            os.write_uint32(13, self.OFAGGKBMPJN)?;
        }
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(2, self.JKOCJIMAGBN)?;
        }
        if self.IFCOBCLMEIA != 0 {
            os.write_uint32(6, self.IFCOBCLMEIA)?;
        }
        if self.HCEFGBHKLIH != 0 {
            os.write_uint32(10, self.HCEFGBHKLIH)?;
        }
        for v in &self.CILFOCBCCNK {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> LCFDPHJIPCE {
        LCFDPHJIPCE::new()
    }

    fn clear(&mut self) {
        self.PBCLKHACEAN = 0;
        self.OFAGGKBMPJN = 0;
        self.JKOCJIMAGBN = 0;
        self.IFCOBCLMEIA = 0;
        self.HCEFGBHKLIH = 0;
        self.CILFOCBCCNK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LCFDPHJIPCE {
        static instance: LCFDPHJIPCE = LCFDPHJIPCE {
            PBCLKHACEAN: 0,
            OFAGGKBMPJN: 0,
            JKOCJIMAGBN: 0,
            IFCOBCLMEIA: 0,
            HCEFGBHKLIH: 0,
            CILFOCBCCNK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LCFDPHJIPCE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LCFDPHJIPCE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LCFDPHJIPCE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LCFDPHJIPCE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LCFDPHJIPCE.proto\x1a\x11CDIIDOLJILK.proto\"\xe7\x01\n\x0bLCFDPHJI\
    PCE\x12\x20\n\x0bPBCLKHACEAN\x18\x01\x20\x01(\rR\x0bPBCLKHACEAN\x12\x20\
    \n\x0bOFAGGKBMPJN\x18\r\x20\x01(\rR\x0bOFAGGKBMPJN\x12\x20\n\x0bJKOCJIMA\
    GBN\x18\x02\x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\n\x0bIFCOBCLMEIA\x18\x06\
    \x20\x01(\rR\x0bIFCOBCLMEIA\x12\x20\n\x0bHCEFGBHKLIH\x18\n\x20\x01(\rR\
    \x0bHCEFGBHKLIH\x12.\n\x0bCILFOCBCCNK\x18\x0b\x20\x03(\x0b2\x0c.CDIIDOLJ\
    ILKR\x0bCILFOCBCCNKb\x06proto3\
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
            deps.push(super::CDIIDOLJILK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LCFDPHJIPCE::generated_message_descriptor_data());
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