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

//! Generated file from `IGJENCIKLOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IGJENCIKLOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IGJENCIKLOF {
    // message fields
    // @@protoc_insertion_point(field:IGJENCIKLOF.IHGMPJNNMKI)
    pub IHGMPJNNMKI: ::std::vec::Vec<super::MKEELPFDCLM::MKEELPFDCLM>,
    // @@protoc_insertion_point(field:IGJENCIKLOF.CLPLEFHHAFB)
    pub CLPLEFHHAFB: ::std::vec::Vec<super::MKEELPFDCLM::MKEELPFDCLM>,
    // @@protoc_insertion_point(field:IGJENCIKLOF.BILBOHBDBPN)
    pub BILBOHBDBPN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IGJENCIKLOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IGJENCIKLOF {
    fn default() -> &'a IGJENCIKLOF {
        <IGJENCIKLOF as ::protobuf::Message>::default_instance()
    }
}

impl IGJENCIKLOF {
    pub fn new() -> IGJENCIKLOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IHGMPJNNMKI",
            |m: &IGJENCIKLOF| { &m.IHGMPJNNMKI },
            |m: &mut IGJENCIKLOF| { &mut m.IHGMPJNNMKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CLPLEFHHAFB",
            |m: &IGJENCIKLOF| { &m.CLPLEFHHAFB },
            |m: &mut IGJENCIKLOF| { &mut m.CLPLEFHHAFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BILBOHBDBPN",
            |m: &IGJENCIKLOF| { &m.BILBOHBDBPN },
            |m: &mut IGJENCIKLOF| { &mut m.BILBOHBDBPN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IGJENCIKLOF>(
            "IGJENCIKLOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IGJENCIKLOF {
    const NAME: &'static str = "IGJENCIKLOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.IHGMPJNNMKI.push(is.read_message()?);
                },
                74 => {
                    self.CLPLEFHHAFB.push(is.read_message()?);
                },
                56 => {
                    self.BILBOHBDBPN = is.read_uint32()?;
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
        for value in &self.IHGMPJNNMKI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CLPLEFHHAFB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BILBOHBDBPN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.BILBOHBDBPN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.IHGMPJNNMKI {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.CLPLEFHHAFB {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.BILBOHBDBPN != 0 {
            os.write_uint32(7, self.BILBOHBDBPN)?;
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

    fn new() -> IGJENCIKLOF {
        IGJENCIKLOF::new()
    }

    fn clear(&mut self) {
        self.IHGMPJNNMKI.clear();
        self.CLPLEFHHAFB.clear();
        self.BILBOHBDBPN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IGJENCIKLOF {
        static instance: IGJENCIKLOF = IGJENCIKLOF {
            IHGMPJNNMKI: ::std::vec::Vec::new(),
            CLPLEFHHAFB: ::std::vec::Vec::new(),
            BILBOHBDBPN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IGJENCIKLOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IGJENCIKLOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IGJENCIKLOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IGJENCIKLOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IGJENCIKLOF.proto\x1a\x11MKEELPFDCLM.proto\"\x8f\x01\n\x0bIGJENCIK\
    LOF\x12.\n\x0bIHGMPJNNMKI\x18\x0e\x20\x03(\x0b2\x0c.MKEELPFDCLMR\x0bIHGM\
    PJNNMKI\x12.\n\x0bCLPLEFHHAFB\x18\t\x20\x03(\x0b2\x0c.MKEELPFDCLMR\x0bCL\
    PLEFHHAFB\x12\x20\n\x0bBILBOHBDBPN\x18\x07\x20\x01(\rR\x0bBILBOHBDBPNb\
    \x06proto3\
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
            deps.push(super::MKEELPFDCLM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IGJENCIKLOF::generated_message_descriptor_data());
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
