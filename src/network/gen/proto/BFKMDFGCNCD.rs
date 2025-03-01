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

//! Generated file from `BFKMDFGCNCD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BFKMDFGCNCD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BFKMDFGCNCD {
    // message fields
    // @@protoc_insertion_point(field:BFKMDFGCNCD.OGCKDLKCABG)
    pub OGCKDLKCABG: i64,
    // @@protoc_insertion_point(field:BFKMDFGCNCD.AHANJLEHCGA)
    pub AHANJLEHCGA: u32,
    // @@protoc_insertion_point(field:BFKMDFGCNCD.PJBIPPDMCHE)
    pub PJBIPPDMCHE: i64,
    // special fields
    // @@protoc_insertion_point(special_field:BFKMDFGCNCD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BFKMDFGCNCD {
    fn default() -> &'a BFKMDFGCNCD {
        <BFKMDFGCNCD as ::protobuf::Message>::default_instance()
    }
}

impl BFKMDFGCNCD {
    pub fn new() -> BFKMDFGCNCD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGCKDLKCABG",
            |m: &BFKMDFGCNCD| { &m.OGCKDLKCABG },
            |m: &mut BFKMDFGCNCD| { &mut m.OGCKDLKCABG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AHANJLEHCGA",
            |m: &BFKMDFGCNCD| { &m.AHANJLEHCGA },
            |m: &mut BFKMDFGCNCD| { &mut m.AHANJLEHCGA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJBIPPDMCHE",
            |m: &BFKMDFGCNCD| { &m.PJBIPPDMCHE },
            |m: &mut BFKMDFGCNCD| { &mut m.PJBIPPDMCHE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BFKMDFGCNCD>(
            "BFKMDFGCNCD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BFKMDFGCNCD {
    const NAME: &'static str = "BFKMDFGCNCD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.OGCKDLKCABG = is.read_int64()?;
                },
                80 => {
                    self.AHANJLEHCGA = is.read_uint32()?;
                },
                120 => {
                    self.PJBIPPDMCHE = is.read_int64()?;
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
        if self.OGCKDLKCABG != 0 {
            my_size += ::protobuf::rt::int64_size(9, self.OGCKDLKCABG);
        }
        if self.AHANJLEHCGA != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.AHANJLEHCGA);
        }
        if self.PJBIPPDMCHE != 0 {
            my_size += ::protobuf::rt::int64_size(15, self.PJBIPPDMCHE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OGCKDLKCABG != 0 {
            os.write_int64(9, self.OGCKDLKCABG)?;
        }
        if self.AHANJLEHCGA != 0 {
            os.write_uint32(10, self.AHANJLEHCGA)?;
        }
        if self.PJBIPPDMCHE != 0 {
            os.write_int64(15, self.PJBIPPDMCHE)?;
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

    fn new() -> BFKMDFGCNCD {
        BFKMDFGCNCD::new()
    }

    fn clear(&mut self) {
        self.OGCKDLKCABG = 0;
        self.AHANJLEHCGA = 0;
        self.PJBIPPDMCHE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BFKMDFGCNCD {
        static instance: BFKMDFGCNCD = BFKMDFGCNCD {
            OGCKDLKCABG: 0,
            AHANJLEHCGA: 0,
            PJBIPPDMCHE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BFKMDFGCNCD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BFKMDFGCNCD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BFKMDFGCNCD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BFKMDFGCNCD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BFKMDFGCNCD.proto\"s\n\x0bBFKMDFGCNCD\x12\x20\n\x0bOGCKDLKCABG\x18\
    \t\x20\x01(\x03R\x0bOGCKDLKCABG\x12\x20\n\x0bAHANJLEHCGA\x18\n\x20\x01(\
    \rR\x0bAHANJLEHCGA\x12\x20\n\x0bPJBIPPDMCHE\x18\x0f\x20\x01(\x03R\x0bPJB\
    IPPDMCHEb\x06proto3\
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
            messages.push(BFKMDFGCNCD::generated_message_descriptor_data());
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
