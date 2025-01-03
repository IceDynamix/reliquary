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

//! Generated file from `ENNGGPFCFDA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ENNGGPFCFDA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ENNGGPFCFDA {
    // message fields
    // @@protoc_insertion_point(field:ENNGGPFCFDA.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<super::PHCBNLNIFMI::PHCBNLNIFMI>,
    // @@protoc_insertion_point(field:ENNGGPFCFDA.KHOPBJOOJNC)
    pub KHOPBJOOJNC: ::std::vec::Vec<super::CDDBLMIAIKF::CDDBLMIAIKF>,
    // @@protoc_insertion_point(field:ENNGGPFCFDA.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<super::KIGHHJDHGOA::KIGHHJDHGOA>,
    // special fields
    // @@protoc_insertion_point(special_field:ENNGGPFCFDA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ENNGGPFCFDA {
    fn default() -> &'a ENNGGPFCFDA {
        <ENNGGPFCFDA as ::protobuf::Message>::default_instance()
    }
}

impl ENNGGPFCFDA {
    pub fn new() -> ENNGGPFCFDA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &ENNGGPFCFDA| { &m.EMALNMLGANJ },
            |m: &mut ENNGGPFCFDA| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KHOPBJOOJNC",
            |m: &ENNGGPFCFDA| { &m.KHOPBJOOJNC },
            |m: &mut ENNGGPFCFDA| { &mut m.KHOPBJOOJNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &ENNGGPFCFDA| { &m.EBDDNGHLIGH },
            |m: &mut ENNGGPFCFDA| { &mut m.EBDDNGHLIGH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ENNGGPFCFDA>(
            "ENNGGPFCFDA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ENNGGPFCFDA {
    const NAME: &'static str = "ENNGGPFCFDA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.EMALNMLGANJ.push(is.read_message()?);
                },
                18 => {
                    self.KHOPBJOOJNC.push(is.read_message()?);
                },
                26 => {
                    self.EBDDNGHLIGH.push(is.read_message()?);
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
        for value in &self.EMALNMLGANJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.KHOPBJOOJNC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.EBDDNGHLIGH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EMALNMLGANJ {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.KHOPBJOOJNC {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.EBDDNGHLIGH {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> ENNGGPFCFDA {
        ENNGGPFCFDA::new()
    }

    fn clear(&mut self) {
        self.EMALNMLGANJ.clear();
        self.KHOPBJOOJNC.clear();
        self.EBDDNGHLIGH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ENNGGPFCFDA {
        static instance: ENNGGPFCFDA = ENNGGPFCFDA {
            EMALNMLGANJ: ::std::vec::Vec::new(),
            KHOPBJOOJNC: ::std::vec::Vec::new(),
            EBDDNGHLIGH: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ENNGGPFCFDA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ENNGGPFCFDA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ENNGGPFCFDA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ENNGGPFCFDA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ENNGGPFCFDA.proto\x1a\x11CDDBLMIAIKF.proto\x1a\x11KIGHHJDHGOA.prot\
    o\x1a\x11PHCBNLNIFMI.proto\"\x9d\x01\n\x0bENNGGPFCFDA\x12.\n\x0bEMALNMLG\
    ANJ\x18\x01\x20\x03(\x0b2\x0c.PHCBNLNIFMIR\x0bEMALNMLGANJ\x12.\n\x0bKHOP\
    BJOOJNC\x18\x02\x20\x03(\x0b2\x0c.CDDBLMIAIKFR\x0bKHOPBJOOJNC\x12.\n\x0b\
    EBDDNGHLIGH\x18\x03\x20\x03(\x0b2\x0c.KIGHHJDHGOAR\x0bEBDDNGHLIGHb\x06pr\
    oto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CDDBLMIAIKF::file_descriptor().clone());
            deps.push(super::KIGHHJDHGOA::file_descriptor().clone());
            deps.push(super::PHCBNLNIFMI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ENNGGPFCFDA::generated_message_descriptor_data());
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
