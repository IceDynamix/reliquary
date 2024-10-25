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

//! Generated file from `OIKHAJADEMG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OIKHAJADEMG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OIKHAJADEMG {
    // message fields
    // @@protoc_insertion_point(field:OIKHAJADEMG.DNDLFECFMNI)
    pub DNDLFECFMNI: u32,
    // @@protoc_insertion_point(field:OIKHAJADEMG.INOGEEHKJMO)
    pub INOGEEHKJMO: u32,
    // @@protoc_insertion_point(field:OIKHAJADEMG.PLKJNGEPIAH)
    pub PLKJNGEPIAH: u32,
    // @@protoc_insertion_point(field:OIKHAJADEMG.IGBJOAKIOFL)
    pub IGBJOAKIOFL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OIKHAJADEMG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OIKHAJADEMG {
    fn default() -> &'a OIKHAJADEMG {
        <OIKHAJADEMG as ::protobuf::Message>::default_instance()
    }
}

impl OIKHAJADEMG {
    pub fn new() -> OIKHAJADEMG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNDLFECFMNI",
            |m: &OIKHAJADEMG| { &m.DNDLFECFMNI },
            |m: &mut OIKHAJADEMG| { &mut m.DNDLFECFMNI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "INOGEEHKJMO",
            |m: &OIKHAJADEMG| { &m.INOGEEHKJMO },
            |m: &mut OIKHAJADEMG| { &mut m.INOGEEHKJMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLKJNGEPIAH",
            |m: &OIKHAJADEMG| { &m.PLKJNGEPIAH },
            |m: &mut OIKHAJADEMG| { &mut m.PLKJNGEPIAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGBJOAKIOFL",
            |m: &OIKHAJADEMG| { &m.IGBJOAKIOFL },
            |m: &mut OIKHAJADEMG| { &mut m.IGBJOAKIOFL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OIKHAJADEMG>(
            "OIKHAJADEMG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OIKHAJADEMG {
    const NAME: &'static str = "OIKHAJADEMG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.DNDLFECFMNI = is.read_uint32()?;
                },
                8 => {
                    self.INOGEEHKJMO = is.read_uint32()?;
                },
                40 => {
                    self.PLKJNGEPIAH = is.read_uint32()?;
                },
                56 => {
                    self.IGBJOAKIOFL = is.read_uint32()?;
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
        if self.DNDLFECFMNI != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DNDLFECFMNI);
        }
        if self.INOGEEHKJMO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.INOGEEHKJMO);
        }
        if self.PLKJNGEPIAH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.PLKJNGEPIAH);
        }
        if self.IGBJOAKIOFL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IGBJOAKIOFL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DNDLFECFMNI != 0 {
            os.write_uint32(4, self.DNDLFECFMNI)?;
        }
        if self.INOGEEHKJMO != 0 {
            os.write_uint32(1, self.INOGEEHKJMO)?;
        }
        if self.PLKJNGEPIAH != 0 {
            os.write_uint32(5, self.PLKJNGEPIAH)?;
        }
        if self.IGBJOAKIOFL != 0 {
            os.write_uint32(7, self.IGBJOAKIOFL)?;
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

    fn new() -> OIKHAJADEMG {
        OIKHAJADEMG::new()
    }

    fn clear(&mut self) {
        self.DNDLFECFMNI = 0;
        self.INOGEEHKJMO = 0;
        self.PLKJNGEPIAH = 0;
        self.IGBJOAKIOFL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OIKHAJADEMG {
        static instance: OIKHAJADEMG = OIKHAJADEMG {
            DNDLFECFMNI: 0,
            INOGEEHKJMO: 0,
            PLKJNGEPIAH: 0,
            IGBJOAKIOFL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OIKHAJADEMG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OIKHAJADEMG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OIKHAJADEMG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OIKHAJADEMG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OIKHAJADEMG.proto\"\x95\x01\n\x0bOIKHAJADEMG\x12\x20\n\x0bDNDLFECF\
    MNI\x18\x04\x20\x01(\rR\x0bDNDLFECFMNI\x12\x20\n\x0bINOGEEHKJMO\x18\x01\
    \x20\x01(\rR\x0bINOGEEHKJMO\x12\x20\n\x0bPLKJNGEPIAH\x18\x05\x20\x01(\rR\
    \x0bPLKJNGEPIAH\x12\x20\n\x0bIGBJOAKIOFL\x18\x07\x20\x01(\rR\x0bIGBJOAKI\
    OFLb\x06proto3\
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
            messages.push(OIKHAJADEMG::generated_message_descriptor_data());
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
