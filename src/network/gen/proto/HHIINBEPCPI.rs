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

//! Generated file from `HHIINBEPCPI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HHIINBEPCPI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HHIINBEPCPI {
    // message fields
    // @@protoc_insertion_point(field:HHIINBEPCPI.IKGOMIBBENF)
    pub IKGOMIBBENF: u32,
    // @@protoc_insertion_point(field:HHIINBEPCPI.OPKEENHPKEO)
    pub OPKEENHPKEO: u32,
    // @@protoc_insertion_point(field:HHIINBEPCPI.OMGHAHGGGFA)
    pub OMGHAHGGGFA: u32,
    // @@protoc_insertion_point(field:HHIINBEPCPI.CNLLDMCOGBP)
    pub CNLLDMCOGBP: u32,
    // @@protoc_insertion_point(field:HHIINBEPCPI.JAODOPDGOGL)
    pub JAODOPDGOGL: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:HHIINBEPCPI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HHIINBEPCPI {
    fn default() -> &'a HHIINBEPCPI {
        <HHIINBEPCPI as ::protobuf::Message>::default_instance()
    }
}

impl HHIINBEPCPI {
    pub fn new() -> HHIINBEPCPI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKGOMIBBENF",
            |m: &HHIINBEPCPI| { &m.IKGOMIBBENF },
            |m: &mut HHIINBEPCPI| { &mut m.IKGOMIBBENF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPKEENHPKEO",
            |m: &HHIINBEPCPI| { &m.OPKEENHPKEO },
            |m: &mut HHIINBEPCPI| { &mut m.OPKEENHPKEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMGHAHGGGFA",
            |m: &HHIINBEPCPI| { &m.OMGHAHGGGFA },
            |m: &mut HHIINBEPCPI| { &mut m.OMGHAHGGGFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNLLDMCOGBP",
            |m: &HHIINBEPCPI| { &m.CNLLDMCOGBP },
            |m: &mut HHIINBEPCPI| { &mut m.CNLLDMCOGBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JAODOPDGOGL",
            |m: &HHIINBEPCPI| { &m.JAODOPDGOGL },
            |m: &mut HHIINBEPCPI| { &mut m.JAODOPDGOGL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HHIINBEPCPI>(
            "HHIINBEPCPI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HHIINBEPCPI {
    const NAME: &'static str = "HHIINBEPCPI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.IKGOMIBBENF = is.read_uint32()?;
                },
                64 => {
                    self.OPKEENHPKEO = is.read_uint32()?;
                },
                40 => {
                    self.OMGHAHGGGFA = is.read_uint32()?;
                },
                120 => {
                    self.CNLLDMCOGBP = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.JAODOPDGOGL)?;
                },
                8 => {
                    self.JAODOPDGOGL.push(is.read_uint32()?);
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
        if self.IKGOMIBBENF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.IKGOMIBBENF);
        }
        if self.OPKEENHPKEO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OPKEENHPKEO);
        }
        if self.OMGHAHGGGFA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.OMGHAHGGGFA);
        }
        if self.CNLLDMCOGBP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CNLLDMCOGBP);
        }
        for value in &self.JAODOPDGOGL {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IKGOMIBBENF != 0 {
            os.write_uint32(10, self.IKGOMIBBENF)?;
        }
        if self.OPKEENHPKEO != 0 {
            os.write_uint32(8, self.OPKEENHPKEO)?;
        }
        if self.OMGHAHGGGFA != 0 {
            os.write_uint32(5, self.OMGHAHGGGFA)?;
        }
        if self.CNLLDMCOGBP != 0 {
            os.write_uint32(15, self.CNLLDMCOGBP)?;
        }
        for v in &self.JAODOPDGOGL {
            os.write_uint32(1, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> HHIINBEPCPI {
        HHIINBEPCPI::new()
    }

    fn clear(&mut self) {
        self.IKGOMIBBENF = 0;
        self.OPKEENHPKEO = 0;
        self.OMGHAHGGGFA = 0;
        self.CNLLDMCOGBP = 0;
        self.JAODOPDGOGL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HHIINBEPCPI {
        static instance: HHIINBEPCPI = HHIINBEPCPI {
            IKGOMIBBENF: 0,
            OPKEENHPKEO: 0,
            OMGHAHGGGFA: 0,
            CNLLDMCOGBP: 0,
            JAODOPDGOGL: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HHIINBEPCPI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HHIINBEPCPI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HHIINBEPCPI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HHIINBEPCPI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HHIINBEPCPI.proto\"\xb7\x01\n\x0bHHIINBEPCPI\x12\x20\n\x0bIKGOMIBB\
    ENF\x18\n\x20\x01(\rR\x0bIKGOMIBBENF\x12\x20\n\x0bOPKEENHPKEO\x18\x08\
    \x20\x01(\rR\x0bOPKEENHPKEO\x12\x20\n\x0bOMGHAHGGGFA\x18\x05\x20\x01(\rR\
    \x0bOMGHAHGGGFA\x12\x20\n\x0bCNLLDMCOGBP\x18\x0f\x20\x01(\rR\x0bCNLLDMCO\
    GBP\x12\x20\n\x0bJAODOPDGOGL\x18\x01\x20\x03(\rR\x0bJAODOPDGOGLb\x06prot\
    o3\
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
            messages.push(HHIINBEPCPI::generated_message_descriptor_data());
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
