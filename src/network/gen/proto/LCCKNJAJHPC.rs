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

//! Generated file from `LCCKNJAJHPC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LCCKNJAJHPC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LCCKNJAJHPC {
    // message fields
    // @@protoc_insertion_point(field:LCCKNJAJHPC.PCJNNCJLPDA)
    pub PCJNNCJLPDA: u32,
    // @@protoc_insertion_point(field:LCCKNJAJHPC.OFCKKGGNEFK)
    pub OFCKKGGNEFK: u32,
    // @@protoc_insertion_point(field:LCCKNJAJHPC.CFNNMEMPIKE)
    pub CFNNMEMPIKE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LCCKNJAJHPC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LCCKNJAJHPC {
    fn default() -> &'a LCCKNJAJHPC {
        <LCCKNJAJHPC as ::protobuf::Message>::default_instance()
    }
}

impl LCCKNJAJHPC {
    pub fn new() -> LCCKNJAJHPC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCJNNCJLPDA",
            |m: &LCCKNJAJHPC| { &m.PCJNNCJLPDA },
            |m: &mut LCCKNJAJHPC| { &mut m.PCJNNCJLPDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFCKKGGNEFK",
            |m: &LCCKNJAJHPC| { &m.OFCKKGGNEFK },
            |m: &mut LCCKNJAJHPC| { &mut m.OFCKKGGNEFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNNMEMPIKE",
            |m: &LCCKNJAJHPC| { &m.CFNNMEMPIKE },
            |m: &mut LCCKNJAJHPC| { &mut m.CFNNMEMPIKE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LCCKNJAJHPC>(
            "LCCKNJAJHPC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LCCKNJAJHPC {
    const NAME: &'static str = "LCCKNJAJHPC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.PCJNNCJLPDA = is.read_uint32()?;
                },
                72 => {
                    self.OFCKKGGNEFK = is.read_uint32()?;
                },
                80 => {
                    self.CFNNMEMPIKE = is.read_uint32()?;
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
        if self.PCJNNCJLPDA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PCJNNCJLPDA);
        }
        if self.OFCKKGGNEFK != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.OFCKKGGNEFK);
        }
        if self.CFNNMEMPIKE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.CFNNMEMPIKE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PCJNNCJLPDA != 0 {
            os.write_uint32(8, self.PCJNNCJLPDA)?;
        }
        if self.OFCKKGGNEFK != 0 {
            os.write_uint32(9, self.OFCKKGGNEFK)?;
        }
        if self.CFNNMEMPIKE != 0 {
            os.write_uint32(10, self.CFNNMEMPIKE)?;
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

    fn new() -> LCCKNJAJHPC {
        LCCKNJAJHPC::new()
    }

    fn clear(&mut self) {
        self.PCJNNCJLPDA = 0;
        self.OFCKKGGNEFK = 0;
        self.CFNNMEMPIKE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LCCKNJAJHPC {
        static instance: LCCKNJAJHPC = LCCKNJAJHPC {
            PCJNNCJLPDA: 0,
            OFCKKGGNEFK: 0,
            CFNNMEMPIKE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LCCKNJAJHPC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LCCKNJAJHPC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LCCKNJAJHPC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LCCKNJAJHPC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LCCKNJAJHPC.proto\"s\n\x0bLCCKNJAJHPC\x12\x20\n\x0bPCJNNCJLPDA\x18\
    \x08\x20\x01(\rR\x0bPCJNNCJLPDA\x12\x20\n\x0bOFCKKGGNEFK\x18\t\x20\x01(\
    \rR\x0bOFCKKGGNEFK\x12\x20\n\x0bCFNNMEMPIKE\x18\n\x20\x01(\rR\x0bCFNNMEM\
    PIKEb\x06proto3\
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
            messages.push(LCCKNJAJHPC::generated_message_descriptor_data());
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
