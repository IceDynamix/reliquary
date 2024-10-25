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

//! Generated file from `FECMANIDNGM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FECMANIDNGM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FECMANIDNGM {
    // message fields
    // @@protoc_insertion_point(field:FECMANIDNGM.JFNJOJCMKMM)
    pub JFNJOJCMKMM: u32,
    // @@protoc_insertion_point(field:FECMANIDNGM.GNGGPEAABBA)
    pub GNGGPEAABBA: u32,
    // @@protoc_insertion_point(field:FECMANIDNGM.MMBOIAHGEJJ)
    pub MMBOIAHGEJJ: u32,
    // @@protoc_insertion_point(field:FECMANIDNGM.ECIOJNHACLI)
    pub ECIOJNHACLI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FECMANIDNGM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FECMANIDNGM {
    fn default() -> &'a FECMANIDNGM {
        <FECMANIDNGM as ::protobuf::Message>::default_instance()
    }
}

impl FECMANIDNGM {
    pub fn new() -> FECMANIDNGM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFNJOJCMKMM",
            |m: &FECMANIDNGM| { &m.JFNJOJCMKMM },
            |m: &mut FECMANIDNGM| { &mut m.JFNJOJCMKMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GNGGPEAABBA",
            |m: &FECMANIDNGM| { &m.GNGGPEAABBA },
            |m: &mut FECMANIDNGM| { &mut m.GNGGPEAABBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMBOIAHGEJJ",
            |m: &FECMANIDNGM| { &m.MMBOIAHGEJJ },
            |m: &mut FECMANIDNGM| { &mut m.MMBOIAHGEJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ECIOJNHACLI",
            |m: &FECMANIDNGM| { &m.ECIOJNHACLI },
            |m: &mut FECMANIDNGM| { &mut m.ECIOJNHACLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FECMANIDNGM>(
            "FECMANIDNGM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FECMANIDNGM {
    const NAME: &'static str = "FECMANIDNGM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.JFNJOJCMKMM = is.read_uint32()?;
                },
                104 => {
                    self.GNGGPEAABBA = is.read_uint32()?;
                },
                24 => {
                    self.MMBOIAHGEJJ = is.read_uint32()?;
                },
                8 => {
                    self.ECIOJNHACLI = is.read_uint32()?;
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
        if self.JFNJOJCMKMM != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.JFNJOJCMKMM);
        }
        if self.GNGGPEAABBA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.GNGGPEAABBA);
        }
        if self.MMBOIAHGEJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MMBOIAHGEJJ);
        }
        if self.ECIOJNHACLI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ECIOJNHACLI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JFNJOJCMKMM != 0 {
            os.write_uint32(6, self.JFNJOJCMKMM)?;
        }
        if self.GNGGPEAABBA != 0 {
            os.write_uint32(13, self.GNGGPEAABBA)?;
        }
        if self.MMBOIAHGEJJ != 0 {
            os.write_uint32(3, self.MMBOIAHGEJJ)?;
        }
        if self.ECIOJNHACLI != 0 {
            os.write_uint32(1, self.ECIOJNHACLI)?;
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

    fn new() -> FECMANIDNGM {
        FECMANIDNGM::new()
    }

    fn clear(&mut self) {
        self.JFNJOJCMKMM = 0;
        self.GNGGPEAABBA = 0;
        self.MMBOIAHGEJJ = 0;
        self.ECIOJNHACLI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FECMANIDNGM {
        static instance: FECMANIDNGM = FECMANIDNGM {
            JFNJOJCMKMM: 0,
            GNGGPEAABBA: 0,
            MMBOIAHGEJJ: 0,
            ECIOJNHACLI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FECMANIDNGM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FECMANIDNGM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FECMANIDNGM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FECMANIDNGM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FECMANIDNGM.proto\"\x95\x01\n\x0bFECMANIDNGM\x12\x20\n\x0bJFNJOJCM\
    KMM\x18\x06\x20\x01(\rR\x0bJFNJOJCMKMM\x12\x20\n\x0bGNGGPEAABBA\x18\r\
    \x20\x01(\rR\x0bGNGGPEAABBA\x12\x20\n\x0bMMBOIAHGEJJ\x18\x03\x20\x01(\rR\
    \x0bMMBOIAHGEJJ\x12\x20\n\x0bECIOJNHACLI\x18\x01\x20\x01(\rR\x0bECIOJNHA\
    CLIb\x06proto3\
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
            messages.push(FECMANIDNGM::generated_message_descriptor_data());
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
