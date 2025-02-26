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

//! Generated file from `OHDNCHFGFMA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OHDNCHFGFMA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OHDNCHFGFMA {
    // message fields
    // @@protoc_insertion_point(field:OHDNCHFGFMA.BLFAANHJPAD)
    pub BLFAANHJPAD: u32,
    // @@protoc_insertion_point(field:OHDNCHFGFMA.BJFBGLBJBNN)
    pub BJFBGLBJBNN: u32,
    // @@protoc_insertion_point(field:OHDNCHFGFMA.LCLMHEGDGGB)
    pub LCLMHEGDGGB: u32,
    // @@protoc_insertion_point(field:OHDNCHFGFMA.DKKLLMOHGFD)
    pub DKKLLMOHGFD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OHDNCHFGFMA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OHDNCHFGFMA {
    fn default() -> &'a OHDNCHFGFMA {
        <OHDNCHFGFMA as ::protobuf::Message>::default_instance()
    }
}

impl OHDNCHFGFMA {
    pub fn new() -> OHDNCHFGFMA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLFAANHJPAD",
            |m: &OHDNCHFGFMA| { &m.BLFAANHJPAD },
            |m: &mut OHDNCHFGFMA| { &mut m.BLFAANHJPAD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJFBGLBJBNN",
            |m: &OHDNCHFGFMA| { &m.BJFBGLBJBNN },
            |m: &mut OHDNCHFGFMA| { &mut m.BJFBGLBJBNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCLMHEGDGGB",
            |m: &OHDNCHFGFMA| { &m.LCLMHEGDGGB },
            |m: &mut OHDNCHFGFMA| { &mut m.LCLMHEGDGGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKKLLMOHGFD",
            |m: &OHDNCHFGFMA| { &m.DKKLLMOHGFD },
            |m: &mut OHDNCHFGFMA| { &mut m.DKKLLMOHGFD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OHDNCHFGFMA>(
            "OHDNCHFGFMA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OHDNCHFGFMA {
    const NAME: &'static str = "OHDNCHFGFMA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.BLFAANHJPAD = is.read_uint32()?;
                },
                120 => {
                    self.BJFBGLBJBNN = is.read_uint32()?;
                },
                80 => {
                    self.LCLMHEGDGGB = is.read_uint32()?;
                },
                56 => {
                    self.DKKLLMOHGFD = is.read_uint32()?;
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
        if self.BLFAANHJPAD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.BLFAANHJPAD);
        }
        if self.BJFBGLBJBNN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BJFBGLBJBNN);
        }
        if self.LCLMHEGDGGB != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.LCLMHEGDGGB);
        }
        if self.DKKLLMOHGFD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DKKLLMOHGFD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BLFAANHJPAD != 0 {
            os.write_uint32(1, self.BLFAANHJPAD)?;
        }
        if self.BJFBGLBJBNN != 0 {
            os.write_uint32(15, self.BJFBGLBJBNN)?;
        }
        if self.LCLMHEGDGGB != 0 {
            os.write_uint32(10, self.LCLMHEGDGGB)?;
        }
        if self.DKKLLMOHGFD != 0 {
            os.write_uint32(7, self.DKKLLMOHGFD)?;
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

    fn new() -> OHDNCHFGFMA {
        OHDNCHFGFMA::new()
    }

    fn clear(&mut self) {
        self.BLFAANHJPAD = 0;
        self.BJFBGLBJBNN = 0;
        self.LCLMHEGDGGB = 0;
        self.DKKLLMOHGFD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OHDNCHFGFMA {
        static instance: OHDNCHFGFMA = OHDNCHFGFMA {
            BLFAANHJPAD: 0,
            BJFBGLBJBNN: 0,
            LCLMHEGDGGB: 0,
            DKKLLMOHGFD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OHDNCHFGFMA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OHDNCHFGFMA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OHDNCHFGFMA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OHDNCHFGFMA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OHDNCHFGFMA.proto\"\x95\x01\n\x0bOHDNCHFGFMA\x12\x20\n\x0bBLFAANHJ\
    PAD\x18\x01\x20\x01(\rR\x0bBLFAANHJPAD\x12\x20\n\x0bBJFBGLBJBNN\x18\x0f\
    \x20\x01(\rR\x0bBJFBGLBJBNN\x12\x20\n\x0bLCLMHEGDGGB\x18\n\x20\x01(\rR\
    \x0bLCLMHEGDGGB\x12\x20\n\x0bDKKLLMOHGFD\x18\x07\x20\x01(\rR\x0bDKKLLMOH\
    GFDb\x06proto3\
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
            messages.push(OHDNCHFGFMA::generated_message_descriptor_data());
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
