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

//! Generated file from `LDPINJADACG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LDPINJADACG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LDPINJADACG {
    // message fields
    // @@protoc_insertion_point(field:LDPINJADACG.HDMFHOPNPAI)
    pub HDMFHOPNPAI: ::protobuf::MessageField<super::IPDPIAGBMNO::IPDPIAGBMNO>,
    // @@protoc_insertion_point(field:LDPINJADACG.NHKKBMBCDJE)
    pub NHKKBMBCDJE: u32,
    // @@protoc_insertion_point(field:LDPINJADACG.KBFKJCFGFKF)
    pub KBFKJCFGFKF: ::std::vec::Vec<super::JCLACFBCCAD::JCLACFBCCAD>,
    // @@protoc_insertion_point(field:LDPINJADACG.KMPCNKLGIAE)
    pub KMPCNKLGIAE: bool,
    // special fields
    // @@protoc_insertion_point(special_field:LDPINJADACG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LDPINJADACG {
    fn default() -> &'a LDPINJADACG {
        <LDPINJADACG as ::protobuf::Message>::default_instance()
    }
}

impl LDPINJADACG {
    pub fn new() -> LDPINJADACG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IPDPIAGBMNO::IPDPIAGBMNO>(
            "HDMFHOPNPAI",
            |m: &LDPINJADACG| { &m.HDMFHOPNPAI },
            |m: &mut LDPINJADACG| { &mut m.HDMFHOPNPAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHKKBMBCDJE",
            |m: &LDPINJADACG| { &m.NHKKBMBCDJE },
            |m: &mut LDPINJADACG| { &mut m.NHKKBMBCDJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KBFKJCFGFKF",
            |m: &LDPINJADACG| { &m.KBFKJCFGFKF },
            |m: &mut LDPINJADACG| { &mut m.KBFKJCFGFKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KMPCNKLGIAE",
            |m: &LDPINJADACG| { &m.KMPCNKLGIAE },
            |m: &mut LDPINJADACG| { &mut m.KMPCNKLGIAE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LDPINJADACG>(
            "LDPINJADACG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LDPINJADACG {
    const NAME: &'static str = "LDPINJADACG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HDMFHOPNPAI)?;
                },
                80 => {
                    self.NHKKBMBCDJE = is.read_uint32()?;
                },
                74 => {
                    self.KBFKJCFGFKF.push(is.read_message()?);
                },
                112 => {
                    self.KMPCNKLGIAE = is.read_bool()?;
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
        if let Some(v) = self.HDMFHOPNPAI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NHKKBMBCDJE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NHKKBMBCDJE);
        }
        for value in &self.KBFKJCFGFKF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KMPCNKLGIAE != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.HDMFHOPNPAI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.NHKKBMBCDJE != 0 {
            os.write_uint32(10, self.NHKKBMBCDJE)?;
        }
        for v in &self.KBFKJCFGFKF {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.KMPCNKLGIAE != false {
            os.write_bool(14, self.KMPCNKLGIAE)?;
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

    fn new() -> LDPINJADACG {
        LDPINJADACG::new()
    }

    fn clear(&mut self) {
        self.HDMFHOPNPAI.clear();
        self.NHKKBMBCDJE = 0;
        self.KBFKJCFGFKF.clear();
        self.KMPCNKLGIAE = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LDPINJADACG {
        static instance: LDPINJADACG = LDPINJADACG {
            HDMFHOPNPAI: ::protobuf::MessageField::none(),
            NHKKBMBCDJE: 0,
            KBFKJCFGFKF: ::std::vec::Vec::new(),
            KMPCNKLGIAE: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LDPINJADACG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LDPINJADACG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LDPINJADACG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LDPINJADACG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LDPINJADACG.proto\x1a\x11IPDPIAGBMNO.proto\x1a\x11JCLACFBCCAD.prot\
    o\"\xb1\x01\n\x0bLDPINJADACG\x12.\n\x0bHDMFHOPNPAI\x18\x08\x20\x01(\x0b2\
    \x0c.IPDPIAGBMNOR\x0bHDMFHOPNPAI\x12\x20\n\x0bNHKKBMBCDJE\x18\n\x20\x01(\
    \rR\x0bNHKKBMBCDJE\x12.\n\x0bKBFKJCFGFKF\x18\t\x20\x03(\x0b2\x0c.JCLACFB\
    CCADR\x0bKBFKJCFGFKF\x12\x20\n\x0bKMPCNKLGIAE\x18\x0e\x20\x01(\x08R\x0bK\
    MPCNKLGIAEb\x06proto3\
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
            deps.push(super::IPDPIAGBMNO::file_descriptor().clone());
            deps.push(super::JCLACFBCCAD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LDPINJADACG::generated_message_descriptor_data());
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