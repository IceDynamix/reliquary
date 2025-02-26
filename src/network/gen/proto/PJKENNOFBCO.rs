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

//! Generated file from `PJKENNOFBCO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PJKENNOFBCO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PJKENNOFBCO {
    // message fields
    // @@protoc_insertion_point(field:PJKENNOFBCO.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // @@protoc_insertion_point(field:PJKENNOFBCO.KJDOLBOBKJF)
    pub KJDOLBOBKJF: u32,
    // @@protoc_insertion_point(field:PJKENNOFBCO.CPGHNDEKBFG)
    pub CPGHNDEKBFG: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PJKENNOFBCO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PJKENNOFBCO {
    fn default() -> &'a PJKENNOFBCO {
        <PJKENNOFBCO as ::protobuf::Message>::default_instance()
    }
}

impl PJKENNOFBCO {
    pub fn new() -> PJKENNOFBCO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &PJKENNOFBCO| { &m.FJNHDHOHBCL },
            |m: &mut PJKENNOFBCO| { &mut m.FJNHDHOHBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJDOLBOBKJF",
            |m: &PJKENNOFBCO| { &m.KJDOLBOBKJF },
            |m: &mut PJKENNOFBCO| { &mut m.KJDOLBOBKJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPGHNDEKBFG",
            |m: &PJKENNOFBCO| { &m.CPGHNDEKBFG },
            |m: &mut PJKENNOFBCO| { &mut m.CPGHNDEKBFG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PJKENNOFBCO>(
            "PJKENNOFBCO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PJKENNOFBCO {
    const NAME: &'static str = "PJKENNOFBCO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
                },
                40 => {
                    self.KJDOLBOBKJF = is.read_uint32()?;
                },
                80 => {
                    self.CPGHNDEKBFG = is.read_bool()?;
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
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FJNHDHOHBCL);
        }
        if self.KJDOLBOBKJF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KJDOLBOBKJF);
        }
        if self.CPGHNDEKBFG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(12, self.FJNHDHOHBCL)?;
        }
        if self.KJDOLBOBKJF != 0 {
            os.write_uint32(5, self.KJDOLBOBKJF)?;
        }
        if self.CPGHNDEKBFG != false {
            os.write_bool(10, self.CPGHNDEKBFG)?;
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

    fn new() -> PJKENNOFBCO {
        PJKENNOFBCO::new()
    }

    fn clear(&mut self) {
        self.FJNHDHOHBCL = 0;
        self.KJDOLBOBKJF = 0;
        self.CPGHNDEKBFG = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PJKENNOFBCO {
        static instance: PJKENNOFBCO = PJKENNOFBCO {
            FJNHDHOHBCL: 0,
            KJDOLBOBKJF: 0,
            CPGHNDEKBFG: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PJKENNOFBCO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PJKENNOFBCO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PJKENNOFBCO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PJKENNOFBCO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PJKENNOFBCO.proto\"s\n\x0bPJKENNOFBCO\x12\x20\n\x0bFJNHDHOHBCL\x18\
    \x0c\x20\x01(\rR\x0bFJNHDHOHBCL\x12\x20\n\x0bKJDOLBOBKJF\x18\x05\x20\x01\
    (\rR\x0bKJDOLBOBKJF\x12\x20\n\x0bCPGHNDEKBFG\x18\n\x20\x01(\x08R\x0bCPGH\
    NDEKBFGb\x06proto3\
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
            messages.push(PJKENNOFBCO::generated_message_descriptor_data());
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
