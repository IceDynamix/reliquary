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

//! Generated file from `KHCOKINAOFI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KHCOKINAOFI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KHCOKINAOFI {
    // message fields
    // @@protoc_insertion_point(field:KHCOKINAOFI.FKMOJLILEDA)
    pub FKMOJLILEDA: u32,
    // @@protoc_insertion_point(field:KHCOKINAOFI.JOLOKJLDABC)
    pub JOLOKJLDABC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KHCOKINAOFI.PBALFFCFPJD)
    pub PBALFFCFPJD: u32,
    // @@protoc_insertion_point(field:KHCOKINAOFI.CMGGJGKHEOB)
    pub CMGGJGKHEOB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KHCOKINAOFI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KHCOKINAOFI {
    fn default() -> &'a KHCOKINAOFI {
        <KHCOKINAOFI as ::protobuf::Message>::default_instance()
    }
}

impl KHCOKINAOFI {
    pub fn new() -> KHCOKINAOFI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKMOJLILEDA",
            |m: &KHCOKINAOFI| { &m.FKMOJLILEDA },
            |m: &mut KHCOKINAOFI| { &mut m.FKMOJLILEDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JOLOKJLDABC",
            |m: &KHCOKINAOFI| { &m.JOLOKJLDABC },
            |m: &mut KHCOKINAOFI| { &mut m.JOLOKJLDABC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBALFFCFPJD",
            |m: &KHCOKINAOFI| { &m.PBALFFCFPJD },
            |m: &mut KHCOKINAOFI| { &mut m.PBALFFCFPJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMGGJGKHEOB",
            |m: &KHCOKINAOFI| { &m.CMGGJGKHEOB },
            |m: &mut KHCOKINAOFI| { &mut m.CMGGJGKHEOB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KHCOKINAOFI>(
            "KHCOKINAOFI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KHCOKINAOFI {
    const NAME: &'static str = "KHCOKINAOFI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.FKMOJLILEDA = is.read_uint32()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.JOLOKJLDABC)?;
                },
                48 => {
                    self.JOLOKJLDABC.push(is.read_uint32()?);
                },
                8 => {
                    self.PBALFFCFPJD = is.read_uint32()?;
                },
                56 => {
                    self.CMGGJGKHEOB = is.read_uint32()?;
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
        if self.FKMOJLILEDA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FKMOJLILEDA);
        }
        for value in &self.JOLOKJLDABC {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        if self.PBALFFCFPJD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PBALFFCFPJD);
        }
        if self.CMGGJGKHEOB != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.CMGGJGKHEOB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FKMOJLILEDA != 0 {
            os.write_uint32(2, self.FKMOJLILEDA)?;
        }
        for v in &self.JOLOKJLDABC {
            os.write_uint32(6, *v)?;
        };
        if self.PBALFFCFPJD != 0 {
            os.write_uint32(1, self.PBALFFCFPJD)?;
        }
        if self.CMGGJGKHEOB != 0 {
            os.write_uint32(7, self.CMGGJGKHEOB)?;
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

    fn new() -> KHCOKINAOFI {
        KHCOKINAOFI::new()
    }

    fn clear(&mut self) {
        self.FKMOJLILEDA = 0;
        self.JOLOKJLDABC.clear();
        self.PBALFFCFPJD = 0;
        self.CMGGJGKHEOB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KHCOKINAOFI {
        static instance: KHCOKINAOFI = KHCOKINAOFI {
            FKMOJLILEDA: 0,
            JOLOKJLDABC: ::std::vec::Vec::new(),
            PBALFFCFPJD: 0,
            CMGGJGKHEOB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KHCOKINAOFI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KHCOKINAOFI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KHCOKINAOFI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KHCOKINAOFI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KHCOKINAOFI.proto\"\x95\x01\n\x0bKHCOKINAOFI\x12\x20\n\x0bFKMOJLIL\
    EDA\x18\x02\x20\x01(\rR\x0bFKMOJLILEDA\x12\x20\n\x0bJOLOKJLDABC\x18\x06\
    \x20\x03(\rR\x0bJOLOKJLDABC\x12\x20\n\x0bPBALFFCFPJD\x18\x01\x20\x01(\rR\
    \x0bPBALFFCFPJD\x12\x20\n\x0bCMGGJGKHEOB\x18\x07\x20\x01(\rR\x0bCMGGJGKH\
    EOBb\x06proto3\
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
            messages.push(KHCOKINAOFI::generated_message_descriptor_data());
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