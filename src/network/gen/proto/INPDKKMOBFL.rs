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

//! Generated file from `INPDKKMOBFL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:INPDKKMOBFL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct INPDKKMOBFL {
    // message fields
    // @@protoc_insertion_point(field:INPDKKMOBFL.IBBAMPCADEC)
    pub IBBAMPCADEC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:INPDKKMOBFL.GLIACPPAFGC)
    pub GLIACPPAFGC: u32,
    // @@protoc_insertion_point(field:INPDKKMOBFL.FFKNMAONGIB)
    pub FFKNMAONGIB: u32,
    // @@protoc_insertion_point(field:INPDKKMOBFL.KGBLECELPNE)
    pub KGBLECELPNE: i64,
    // special fields
    // @@protoc_insertion_point(special_field:INPDKKMOBFL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a INPDKKMOBFL {
    fn default() -> &'a INPDKKMOBFL {
        <INPDKKMOBFL as ::protobuf::Message>::default_instance()
    }
}

impl INPDKKMOBFL {
    pub fn new() -> INPDKKMOBFL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IBBAMPCADEC",
            |m: &INPDKKMOBFL| { &m.IBBAMPCADEC },
            |m: &mut INPDKKMOBFL| { &mut m.IBBAMPCADEC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GLIACPPAFGC",
            |m: &INPDKKMOBFL| { &m.GLIACPPAFGC },
            |m: &mut INPDKKMOBFL| { &mut m.GLIACPPAFGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFKNMAONGIB",
            |m: &INPDKKMOBFL| { &m.FFKNMAONGIB },
            |m: &mut INPDKKMOBFL| { &mut m.FFKNMAONGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGBLECELPNE",
            |m: &INPDKKMOBFL| { &m.KGBLECELPNE },
            |m: &mut INPDKKMOBFL| { &mut m.KGBLECELPNE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<INPDKKMOBFL>(
            "INPDKKMOBFL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for INPDKKMOBFL {
    const NAME: &'static str = "INPDKKMOBFL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.IBBAMPCADEC)?;
                },
                40 => {
                    self.IBBAMPCADEC.push(is.read_uint32()?);
                },
                64 => {
                    self.GLIACPPAFGC = is.read_uint32()?;
                },
                48 => {
                    self.FFKNMAONGIB = is.read_uint32()?;
                },
                32 => {
                    self.KGBLECELPNE = is.read_int64()?;
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
        for value in &self.IBBAMPCADEC {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.GLIACPPAFGC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GLIACPPAFGC);
        }
        if self.FFKNMAONGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FFKNMAONGIB);
        }
        if self.KGBLECELPNE != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.KGBLECELPNE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.IBBAMPCADEC {
            os.write_uint32(5, *v)?;
        };
        if self.GLIACPPAFGC != 0 {
            os.write_uint32(8, self.GLIACPPAFGC)?;
        }
        if self.FFKNMAONGIB != 0 {
            os.write_uint32(6, self.FFKNMAONGIB)?;
        }
        if self.KGBLECELPNE != 0 {
            os.write_int64(4, self.KGBLECELPNE)?;
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

    fn new() -> INPDKKMOBFL {
        INPDKKMOBFL::new()
    }

    fn clear(&mut self) {
        self.IBBAMPCADEC.clear();
        self.GLIACPPAFGC = 0;
        self.FFKNMAONGIB = 0;
        self.KGBLECELPNE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static INPDKKMOBFL {
        static instance: INPDKKMOBFL = INPDKKMOBFL {
            IBBAMPCADEC: ::std::vec::Vec::new(),
            GLIACPPAFGC: 0,
            FFKNMAONGIB: 0,
            KGBLECELPNE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for INPDKKMOBFL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("INPDKKMOBFL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for INPDKKMOBFL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INPDKKMOBFL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11INPDKKMOBFL.proto\"\x95\x01\n\x0bINPDKKMOBFL\x12\x20\n\x0bIBBAMPCA\
    DEC\x18\x05\x20\x03(\rR\x0bIBBAMPCADEC\x12\x20\n\x0bGLIACPPAFGC\x18\x08\
    \x20\x01(\rR\x0bGLIACPPAFGC\x12\x20\n\x0bFFKNMAONGIB\x18\x06\x20\x01(\rR\
    \x0bFFKNMAONGIB\x12\x20\n\x0bKGBLECELPNE\x18\x04\x20\x01(\x03R\x0bKGBLEC\
    ELPNEb\x06proto3\
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
            messages.push(INPDKKMOBFL::generated_message_descriptor_data());
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