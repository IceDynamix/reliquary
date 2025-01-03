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

//! Generated file from `DJOMEMKCBNB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DJOMEMKCBNB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DJOMEMKCBNB {
    // message fields
    // @@protoc_insertion_point(field:DJOMEMKCBNB.LIJNMBCNOMK)
    pub LIJNMBCNOMK: u32,
    // @@protoc_insertion_point(field:DJOMEMKCBNB.NOFPFOIPOFD)
    pub NOFPFOIPOFD: bool,
    // @@protoc_insertion_point(field:DJOMEMKCBNB.HDMFHOPNPAI)
    pub HDMFHOPNPAI: ::protobuf::MessageField<super::IPDPIAGBMNO::IPDPIAGBMNO>,
    // @@protoc_insertion_point(field:DJOMEMKCBNB.PMMELAOHNGO)
    pub PMMELAOHNGO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DJOMEMKCBNB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DJOMEMKCBNB {
    fn default() -> &'a DJOMEMKCBNB {
        <DJOMEMKCBNB as ::protobuf::Message>::default_instance()
    }
}

impl DJOMEMKCBNB {
    pub fn new() -> DJOMEMKCBNB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIJNMBCNOMK",
            |m: &DJOMEMKCBNB| { &m.LIJNMBCNOMK },
            |m: &mut DJOMEMKCBNB| { &mut m.LIJNMBCNOMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOFPFOIPOFD",
            |m: &DJOMEMKCBNB| { &m.NOFPFOIPOFD },
            |m: &mut DJOMEMKCBNB| { &mut m.NOFPFOIPOFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IPDPIAGBMNO::IPDPIAGBMNO>(
            "HDMFHOPNPAI",
            |m: &DJOMEMKCBNB| { &m.HDMFHOPNPAI },
            |m: &mut DJOMEMKCBNB| { &mut m.HDMFHOPNPAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMMELAOHNGO",
            |m: &DJOMEMKCBNB| { &m.PMMELAOHNGO },
            |m: &mut DJOMEMKCBNB| { &mut m.PMMELAOHNGO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DJOMEMKCBNB>(
            "DJOMEMKCBNB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DJOMEMKCBNB {
    const NAME: &'static str = "DJOMEMKCBNB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.LIJNMBCNOMK = is.read_uint32()?;
                },
                32 => {
                    self.NOFPFOIPOFD = is.read_bool()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HDMFHOPNPAI)?;
                },
                72 => {
                    self.PMMELAOHNGO = is.read_uint32()?;
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
        if self.LIJNMBCNOMK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LIJNMBCNOMK);
        }
        if self.NOFPFOIPOFD != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.HDMFHOPNPAI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PMMELAOHNGO != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PMMELAOHNGO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LIJNMBCNOMK != 0 {
            os.write_uint32(7, self.LIJNMBCNOMK)?;
        }
        if self.NOFPFOIPOFD != false {
            os.write_bool(4, self.NOFPFOIPOFD)?;
        }
        if let Some(v) = self.HDMFHOPNPAI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.PMMELAOHNGO != 0 {
            os.write_uint32(9, self.PMMELAOHNGO)?;
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

    fn new() -> DJOMEMKCBNB {
        DJOMEMKCBNB::new()
    }

    fn clear(&mut self) {
        self.LIJNMBCNOMK = 0;
        self.NOFPFOIPOFD = false;
        self.HDMFHOPNPAI.clear();
        self.PMMELAOHNGO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DJOMEMKCBNB {
        static instance: DJOMEMKCBNB = DJOMEMKCBNB {
            LIJNMBCNOMK: 0,
            NOFPFOIPOFD: false,
            HDMFHOPNPAI: ::protobuf::MessageField::none(),
            PMMELAOHNGO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DJOMEMKCBNB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DJOMEMKCBNB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DJOMEMKCBNB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DJOMEMKCBNB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DJOMEMKCBNB.proto\x1a\x11IPDPIAGBMNO.proto\"\xa3\x01\n\x0bDJOMEMKC\
    BNB\x12\x20\n\x0bLIJNMBCNOMK\x18\x07\x20\x01(\rR\x0bLIJNMBCNOMK\x12\x20\
    \n\x0bNOFPFOIPOFD\x18\x04\x20\x01(\x08R\x0bNOFPFOIPOFD\x12.\n\x0bHDMFHOP\
    NPAI\x18\x08\x20\x01(\x0b2\x0c.IPDPIAGBMNOR\x0bHDMFHOPNPAI\x12\x20\n\x0b\
    PMMELAOHNGO\x18\t\x20\x01(\rR\x0bPMMELAOHNGOb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::IPDPIAGBMNO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DJOMEMKCBNB::generated_message_descriptor_data());
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
