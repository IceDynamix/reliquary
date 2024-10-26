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

//! Generated file from `MAGBCAHDHCP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MAGBCAHDHCP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MAGBCAHDHCP {
    // message fields
    // @@protoc_insertion_point(field:MAGBCAHDHCP.PPMKDMINBNH)
    pub PPMKDMINBNH: u32,
    // @@protoc_insertion_point(field:MAGBCAHDHCP.CKLHHOLMBOO)
    pub CKLHHOLMBOO: ::std::vec::Vec<super::FMGCOGMJKCM::FMGCOGMJKCM>,
    // @@protoc_insertion_point(field:MAGBCAHDHCP.level)
    pub level: u32,
    // @@protoc_insertion_point(field:MAGBCAHDHCP.CMIPCBOJJIC)
    pub CMIPCBOJJIC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MAGBCAHDHCP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MAGBCAHDHCP {
    fn default() -> &'a MAGBCAHDHCP {
        <MAGBCAHDHCP as ::protobuf::Message>::default_instance()
    }
}

impl MAGBCAHDHCP {
    pub fn new() -> MAGBCAHDHCP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPMKDMINBNH",
            |m: &MAGBCAHDHCP| { &m.PPMKDMINBNH },
            |m: &mut MAGBCAHDHCP| { &mut m.PPMKDMINBNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CKLHHOLMBOO",
            |m: &MAGBCAHDHCP| { &m.CKLHHOLMBOO },
            |m: &mut MAGBCAHDHCP| { &mut m.CKLHHOLMBOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &MAGBCAHDHCP| { &m.level },
            |m: &mut MAGBCAHDHCP| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMIPCBOJJIC",
            |m: &MAGBCAHDHCP| { &m.CMIPCBOJJIC },
            |m: &mut MAGBCAHDHCP| { &mut m.CMIPCBOJJIC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MAGBCAHDHCP>(
            "MAGBCAHDHCP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MAGBCAHDHCP {
    const NAME: &'static str = "MAGBCAHDHCP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.PPMKDMINBNH = is.read_uint32()?;
                },
                90 => {
                    self.CKLHHOLMBOO.push(is.read_message()?);
                },
                56 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.CMIPCBOJJIC = is.read_uint32()?;
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
        if self.PPMKDMINBNH != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PPMKDMINBNH);
        }
        for value in &self.CKLHHOLMBOO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.level);
        }
        if self.CMIPCBOJJIC != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.CMIPCBOJJIC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PPMKDMINBNH != 0 {
            os.write_uint32(14, self.PPMKDMINBNH)?;
        }
        for v in &self.CKLHHOLMBOO {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.level != 0 {
            os.write_uint32(7, self.level)?;
        }
        if self.CMIPCBOJJIC != 0 {
            os.write_uint32(12, self.CMIPCBOJJIC)?;
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

    fn new() -> MAGBCAHDHCP {
        MAGBCAHDHCP::new()
    }

    fn clear(&mut self) {
        self.PPMKDMINBNH = 0;
        self.CKLHHOLMBOO.clear();
        self.level = 0;
        self.CMIPCBOJJIC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MAGBCAHDHCP {
        static instance: MAGBCAHDHCP = MAGBCAHDHCP {
            PPMKDMINBNH: 0,
            CKLHHOLMBOO: ::std::vec::Vec::new(),
            level: 0,
            CMIPCBOJJIC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MAGBCAHDHCP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MAGBCAHDHCP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MAGBCAHDHCP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MAGBCAHDHCP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MAGBCAHDHCP.proto\x1a\x11FMGCOGMJKCM.proto\"\x97\x01\n\x0bMAGBCAHD\
    HCP\x12\x20\n\x0bPPMKDMINBNH\x18\x0e\x20\x01(\rR\x0bPPMKDMINBNH\x12.\n\
    \x0bCKLHHOLMBOO\x18\x0b\x20\x03(\x0b2\x0c.FMGCOGMJKCMR\x0bCKLHHOLMBOO\
    \x12\x14\n\x05level\x18\x07\x20\x01(\rR\x05level\x12\x20\n\x0bCMIPCBOJJI\
    C\x18\x0c\x20\x01(\rR\x0bCMIPCBOJJICb\x06proto3\
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
            deps.push(super::FMGCOGMJKCM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MAGBCAHDHCP::generated_message_descriptor_data());
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
