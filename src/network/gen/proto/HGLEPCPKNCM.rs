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

//! Generated file from `HGLEPCPKNCM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HGLEPCPKNCM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HGLEPCPKNCM {
    // message fields
    // @@protoc_insertion_point(field:HGLEPCPKNCM.NABCKMDFNOC)
    pub NABCKMDFNOC: u32,
    // @@protoc_insertion_point(field:HGLEPCPKNCM.MPNJPFDCBDG)
    pub MPNJPFDCBDG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HGLEPCPKNCM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HGLEPCPKNCM {
    fn default() -> &'a HGLEPCPKNCM {
        <HGLEPCPKNCM as ::protobuf::Message>::default_instance()
    }
}

impl HGLEPCPKNCM {
    pub fn new() -> HGLEPCPKNCM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NABCKMDFNOC",
            |m: &HGLEPCPKNCM| { &m.NABCKMDFNOC },
            |m: &mut HGLEPCPKNCM| { &mut m.NABCKMDFNOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPNJPFDCBDG",
            |m: &HGLEPCPKNCM| { &m.MPNJPFDCBDG },
            |m: &mut HGLEPCPKNCM| { &mut m.MPNJPFDCBDG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HGLEPCPKNCM>(
            "HGLEPCPKNCM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HGLEPCPKNCM {
    const NAME: &'static str = "HGLEPCPKNCM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.NABCKMDFNOC = is.read_uint32()?;
                },
                40 => {
                    self.MPNJPFDCBDG = is.read_uint32()?;
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
        if self.NABCKMDFNOC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NABCKMDFNOC);
        }
        if self.MPNJPFDCBDG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.MPNJPFDCBDG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NABCKMDFNOC != 0 {
            os.write_uint32(9, self.NABCKMDFNOC)?;
        }
        if self.MPNJPFDCBDG != 0 {
            os.write_uint32(5, self.MPNJPFDCBDG)?;
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

    fn new() -> HGLEPCPKNCM {
        HGLEPCPKNCM::new()
    }

    fn clear(&mut self) {
        self.NABCKMDFNOC = 0;
        self.MPNJPFDCBDG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HGLEPCPKNCM {
        static instance: HGLEPCPKNCM = HGLEPCPKNCM {
            NABCKMDFNOC: 0,
            MPNJPFDCBDG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HGLEPCPKNCM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HGLEPCPKNCM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HGLEPCPKNCM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HGLEPCPKNCM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HGLEPCPKNCM.proto\"Q\n\x0bHGLEPCPKNCM\x12\x20\n\x0bNABCKMDFNOC\x18\
    \t\x20\x01(\rR\x0bNABCKMDFNOC\x12\x20\n\x0bMPNJPFDCBDG\x18\x05\x20\x01(\
    \rR\x0bMPNJPFDCBDGb\x06proto3\
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
            messages.push(HGLEPCPKNCM::generated_message_descriptor_data());
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
