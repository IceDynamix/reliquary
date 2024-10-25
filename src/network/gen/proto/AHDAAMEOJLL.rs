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

//! Generated file from `AHDAAMEOJLL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AHDAAMEOJLL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AHDAAMEOJLL {
    // message fields
    // @@protoc_insertion_point(field:AHDAAMEOJLL.LCOAGNJBCKG)
    pub LCOAGNJBCKG: bool,
    // @@protoc_insertion_point(field:AHDAAMEOJLL.GGFKEIJJLCE)
    pub GGFKEIJJLCE: bool,
    // @@protoc_insertion_point(field:AHDAAMEOJLL.LKGNGHCPJAG)
    pub LKGNGHCPJAG: u32,
    // @@protoc_insertion_point(field:AHDAAMEOJLL.step)
    pub step: ::protobuf::EnumOrUnknown<super::FFOPGMAMLMF::FFOPGMAMLMF>,
    // @@protoc_insertion_point(field:AHDAAMEOJLL.NHMFGEIBPFH)
    pub NHMFGEIBPFH: ::protobuf::EnumOrUnknown<super::FOHOHPLGEFJ::FOHOHPLGEFJ>,
    // special fields
    // @@protoc_insertion_point(special_field:AHDAAMEOJLL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AHDAAMEOJLL {
    fn default() -> &'a AHDAAMEOJLL {
        <AHDAAMEOJLL as ::protobuf::Message>::default_instance()
    }
}

impl AHDAAMEOJLL {
    pub fn new() -> AHDAAMEOJLL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCOAGNJBCKG",
            |m: &AHDAAMEOJLL| { &m.LCOAGNJBCKG },
            |m: &mut AHDAAMEOJLL| { &mut m.LCOAGNJBCKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGFKEIJJLCE",
            |m: &AHDAAMEOJLL| { &m.GGFKEIJJLCE },
            |m: &mut AHDAAMEOJLL| { &mut m.GGFKEIJJLCE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKGNGHCPJAG",
            |m: &AHDAAMEOJLL| { &m.LKGNGHCPJAG },
            |m: &mut AHDAAMEOJLL| { &mut m.LKGNGHCPJAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &AHDAAMEOJLL| { &m.step },
            |m: &mut AHDAAMEOJLL| { &mut m.step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHMFGEIBPFH",
            |m: &AHDAAMEOJLL| { &m.NHMFGEIBPFH },
            |m: &mut AHDAAMEOJLL| { &mut m.NHMFGEIBPFH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AHDAAMEOJLL>(
            "AHDAAMEOJLL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AHDAAMEOJLL {
    const NAME: &'static str = "AHDAAMEOJLL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.LCOAGNJBCKG = is.read_bool()?;
                },
                120 => {
                    self.GGFKEIJJLCE = is.read_bool()?;
                },
                56 => {
                    self.LKGNGHCPJAG = is.read_uint32()?;
                },
                80 => {
                    self.step = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.NHMFGEIBPFH = is.read_enum_or_unknown()?;
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
        if self.LCOAGNJBCKG != false {
            my_size += 1 + 1;
        }
        if self.GGFKEIJJLCE != false {
            my_size += 1 + 1;
        }
        if self.LKGNGHCPJAG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LKGNGHCPJAG);
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::FFOPGMAMLMF::FFOPGMAMLMF::HEART_DIAL_STEP_TYPE_MISSING) {
            my_size += ::protobuf::rt::int32_size(10, self.step.value());
        }
        if self.NHMFGEIBPFH != ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE) {
            my_size += ::protobuf::rt::int32_size(14, self.NHMFGEIBPFH.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LCOAGNJBCKG != false {
            os.write_bool(4, self.LCOAGNJBCKG)?;
        }
        if self.GGFKEIJJLCE != false {
            os.write_bool(15, self.GGFKEIJJLCE)?;
        }
        if self.LKGNGHCPJAG != 0 {
            os.write_uint32(7, self.LKGNGHCPJAG)?;
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::FFOPGMAMLMF::FFOPGMAMLMF::HEART_DIAL_STEP_TYPE_MISSING) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.step))?;
        }
        if self.NHMFGEIBPFH != ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.NHMFGEIBPFH))?;
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

    fn new() -> AHDAAMEOJLL {
        AHDAAMEOJLL::new()
    }

    fn clear(&mut self) {
        self.LCOAGNJBCKG = false;
        self.GGFKEIJJLCE = false;
        self.LKGNGHCPJAG = 0;
        self.step = ::protobuf::EnumOrUnknown::new(super::FFOPGMAMLMF::FFOPGMAMLMF::HEART_DIAL_STEP_TYPE_MISSING);
        self.NHMFGEIBPFH = ::protobuf::EnumOrUnknown::new(super::FOHOHPLGEFJ::FOHOHPLGEFJ::HEART_DIAL_EMOTION_TYPE_PEACE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AHDAAMEOJLL {
        static instance: AHDAAMEOJLL = AHDAAMEOJLL {
            LCOAGNJBCKG: false,
            GGFKEIJJLCE: false,
            LKGNGHCPJAG: 0,
            step: ::protobuf::EnumOrUnknown::from_i32(0),
            NHMFGEIBPFH: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AHDAAMEOJLL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AHDAAMEOJLL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AHDAAMEOJLL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AHDAAMEOJLL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AHDAAMEOJLL.proto\x1a\x11FFOPGMAMLMF.proto\x1a\x11FOHOHPLGEFJ.prot\
    o\"\xc5\x01\n\x0bAHDAAMEOJLL\x12\x20\n\x0bLCOAGNJBCKG\x18\x04\x20\x01(\
    \x08R\x0bLCOAGNJBCKG\x12\x20\n\x0bGGFKEIJJLCE\x18\x0f\x20\x01(\x08R\x0bG\
    GFKEIJJLCE\x12\x20\n\x0bLKGNGHCPJAG\x18\x07\x20\x01(\rR\x0bLKGNGHCPJAG\
    \x12\x20\n\x04step\x18\n\x20\x01(\x0e2\x0c.FFOPGMAMLMFR\x04step\x12.\n\
    \x0bNHMFGEIBPFH\x18\x0e\x20\x01(\x0e2\x0c.FOHOHPLGEFJR\x0bNHMFGEIBPFHb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::FFOPGMAMLMF::file_descriptor().clone());
            deps.push(super::FOHOHPLGEFJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AHDAAMEOJLL::generated_message_descriptor_data());
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
