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

//! Generated file from `KICCJEHBDCP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KICCJEHBDCP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KICCJEHBDCP {
    // message fields
    // @@protoc_insertion_point(field:KICCJEHBDCP.HEBOKLEIDMO)
    pub HEBOKLEIDMO: u32,
    // @@protoc_insertion_point(field:KICCJEHBDCP.LPCKFAOPOLH)
    pub LPCKFAOPOLH: u32,
    // @@protoc_insertion_point(field:KICCJEHBDCP.MBMJDACCDNF)
    pub MBMJDACCDNF: f64,
    // @@protoc_insertion_point(field:KICCJEHBDCP.IJOLEKJFLGF)
    pub IJOLEKJFLGF: u32,
    // @@protoc_insertion_point(field:KICCJEHBDCP.BEPEKKHIFKE)
    pub BEPEKKHIFKE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KICCJEHBDCP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KICCJEHBDCP {
    fn default() -> &'a KICCJEHBDCP {
        <KICCJEHBDCP as ::protobuf::Message>::default_instance()
    }
}

impl KICCJEHBDCP {
    pub fn new() -> KICCJEHBDCP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEBOKLEIDMO",
            |m: &KICCJEHBDCP| { &m.HEBOKLEIDMO },
            |m: &mut KICCJEHBDCP| { &mut m.HEBOKLEIDMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LPCKFAOPOLH",
            |m: &KICCJEHBDCP| { &m.LPCKFAOPOLH },
            |m: &mut KICCJEHBDCP| { &mut m.LPCKFAOPOLH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MBMJDACCDNF",
            |m: &KICCJEHBDCP| { &m.MBMJDACCDNF },
            |m: &mut KICCJEHBDCP| { &mut m.MBMJDACCDNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJOLEKJFLGF",
            |m: &KICCJEHBDCP| { &m.IJOLEKJFLGF },
            |m: &mut KICCJEHBDCP| { &mut m.IJOLEKJFLGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEPEKKHIFKE",
            |m: &KICCJEHBDCP| { &m.BEPEKKHIFKE },
            |m: &mut KICCJEHBDCP| { &mut m.BEPEKKHIFKE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KICCJEHBDCP>(
            "KICCJEHBDCP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KICCJEHBDCP {
    const NAME: &'static str = "KICCJEHBDCP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.HEBOKLEIDMO = is.read_uint32()?;
                },
                16 => {
                    self.LPCKFAOPOLH = is.read_uint32()?;
                },
                25 => {
                    self.MBMJDACCDNF = is.read_double()?;
                },
                32 => {
                    self.IJOLEKJFLGF = is.read_uint32()?;
                },
                40 => {
                    self.BEPEKKHIFKE = is.read_uint32()?;
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
        if self.HEBOKLEIDMO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.HEBOKLEIDMO);
        }
        if self.LPCKFAOPOLH != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LPCKFAOPOLH);
        }
        if self.MBMJDACCDNF != 0. {
            my_size += 1 + 8;
        }
        if self.IJOLEKJFLGF != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IJOLEKJFLGF);
        }
        if self.BEPEKKHIFKE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BEPEKKHIFKE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HEBOKLEIDMO != 0 {
            os.write_uint32(1, self.HEBOKLEIDMO)?;
        }
        if self.LPCKFAOPOLH != 0 {
            os.write_uint32(2, self.LPCKFAOPOLH)?;
        }
        if self.MBMJDACCDNF != 0. {
            os.write_double(3, self.MBMJDACCDNF)?;
        }
        if self.IJOLEKJFLGF != 0 {
            os.write_uint32(4, self.IJOLEKJFLGF)?;
        }
        if self.BEPEKKHIFKE != 0 {
            os.write_uint32(5, self.BEPEKKHIFKE)?;
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

    fn new() -> KICCJEHBDCP {
        KICCJEHBDCP::new()
    }

    fn clear(&mut self) {
        self.HEBOKLEIDMO = 0;
        self.LPCKFAOPOLH = 0;
        self.MBMJDACCDNF = 0.;
        self.IJOLEKJFLGF = 0;
        self.BEPEKKHIFKE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KICCJEHBDCP {
        static instance: KICCJEHBDCP = KICCJEHBDCP {
            HEBOKLEIDMO: 0,
            LPCKFAOPOLH: 0,
            MBMJDACCDNF: 0.,
            IJOLEKJFLGF: 0,
            BEPEKKHIFKE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KICCJEHBDCP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KICCJEHBDCP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KICCJEHBDCP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KICCJEHBDCP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KICCJEHBDCP.proto\"\xb7\x01\n\x0bKICCJEHBDCP\x12\x20\n\x0bHEBOKLEI\
    DMO\x18\x01\x20\x01(\rR\x0bHEBOKLEIDMO\x12\x20\n\x0bLPCKFAOPOLH\x18\x02\
    \x20\x01(\rR\x0bLPCKFAOPOLH\x12\x20\n\x0bMBMJDACCDNF\x18\x03\x20\x01(\
    \x01R\x0bMBMJDACCDNF\x12\x20\n\x0bIJOLEKJFLGF\x18\x04\x20\x01(\rR\x0bIJO\
    LEKJFLGF\x12\x20\n\x0bBEPEKKHIFKE\x18\x05\x20\x01(\rR\x0bBEPEKKHIFKEb\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KICCJEHBDCP::generated_message_descriptor_data());
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