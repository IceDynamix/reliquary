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

//! Generated file from `OLKFPHPBPDL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OLKFPHPBPDL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OLKFPHPBPDL {
    // message fields
    // @@protoc_insertion_point(field:OLKFPHPBPDL.HDMDAODJACB)
    pub HDMDAODJACB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OLKFPHPBPDL.KNBDPFEIDNM)
    pub KNBDPFEIDNM: bool,
    // @@protoc_insertion_point(field:OLKFPHPBPDL.DBAHFEFGLMD)
    pub DBAHFEFGLMD: u32,
    // @@protoc_insertion_point(field:OLKFPHPBPDL.GMOPLJJGBPO)
    pub GMOPLJJGBPO: bool,
    // @@protoc_insertion_point(field:OLKFPHPBPDL.BIIFELFEGNK)
    pub BIIFELFEGNK: bool,
    // @@protoc_insertion_point(field:OLKFPHPBPDL.KGJHJMDCAOC)
    pub KGJHJMDCAOC: bool,
    // special fields
    // @@protoc_insertion_point(special_field:OLKFPHPBPDL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OLKFPHPBPDL {
    fn default() -> &'a OLKFPHPBPDL {
        <OLKFPHPBPDL as ::protobuf::Message>::default_instance()
    }
}

impl OLKFPHPBPDL {
    pub fn new() -> OLKFPHPBPDL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HDMDAODJACB",
            |m: &OLKFPHPBPDL| { &m.HDMDAODJACB },
            |m: &mut OLKFPHPBPDL| { &mut m.HDMDAODJACB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KNBDPFEIDNM",
            |m: &OLKFPHPBPDL| { &m.KNBDPFEIDNM },
            |m: &mut OLKFPHPBPDL| { &mut m.KNBDPFEIDNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBAHFEFGLMD",
            |m: &OLKFPHPBPDL| { &m.DBAHFEFGLMD },
            |m: &mut OLKFPHPBPDL| { &mut m.DBAHFEFGLMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMOPLJJGBPO",
            |m: &OLKFPHPBPDL| { &m.GMOPLJJGBPO },
            |m: &mut OLKFPHPBPDL| { &mut m.GMOPLJJGBPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIIFELFEGNK",
            |m: &OLKFPHPBPDL| { &m.BIIFELFEGNK },
            |m: &mut OLKFPHPBPDL| { &mut m.BIIFELFEGNK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGJHJMDCAOC",
            |m: &OLKFPHPBPDL| { &m.KGJHJMDCAOC },
            |m: &mut OLKFPHPBPDL| { &mut m.KGJHJMDCAOC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OLKFPHPBPDL>(
            "OLKFPHPBPDL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OLKFPHPBPDL {
    const NAME: &'static str = "OLKFPHPBPDL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.HDMDAODJACB)?;
                },
                8 => {
                    self.HDMDAODJACB.push(is.read_uint32()?);
                },
                64 => {
                    self.KNBDPFEIDNM = is.read_bool()?;
                },
                56 => {
                    self.DBAHFEFGLMD = is.read_uint32()?;
                },
                16 => {
                    self.GMOPLJJGBPO = is.read_bool()?;
                },
                112 => {
                    self.BIIFELFEGNK = is.read_bool()?;
                },
                104 => {
                    self.KGJHJMDCAOC = is.read_bool()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.HDMDAODJACB);
        if self.KNBDPFEIDNM != false {
            my_size += 1 + 1;
        }
        if self.DBAHFEFGLMD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DBAHFEFGLMD);
        }
        if self.GMOPLJJGBPO != false {
            my_size += 1 + 1;
        }
        if self.BIIFELFEGNK != false {
            my_size += 1 + 1;
        }
        if self.KGJHJMDCAOC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(1, &self.HDMDAODJACB)?;
        if self.KNBDPFEIDNM != false {
            os.write_bool(8, self.KNBDPFEIDNM)?;
        }
        if self.DBAHFEFGLMD != 0 {
            os.write_uint32(7, self.DBAHFEFGLMD)?;
        }
        if self.GMOPLJJGBPO != false {
            os.write_bool(2, self.GMOPLJJGBPO)?;
        }
        if self.BIIFELFEGNK != false {
            os.write_bool(14, self.BIIFELFEGNK)?;
        }
        if self.KGJHJMDCAOC != false {
            os.write_bool(13, self.KGJHJMDCAOC)?;
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

    fn new() -> OLKFPHPBPDL {
        OLKFPHPBPDL::new()
    }

    fn clear(&mut self) {
        self.HDMDAODJACB.clear();
        self.KNBDPFEIDNM = false;
        self.DBAHFEFGLMD = 0;
        self.GMOPLJJGBPO = false;
        self.BIIFELFEGNK = false;
        self.KGJHJMDCAOC = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OLKFPHPBPDL {
        static instance: OLKFPHPBPDL = OLKFPHPBPDL {
            HDMDAODJACB: ::std::vec::Vec::new(),
            KNBDPFEIDNM: false,
            DBAHFEFGLMD: 0,
            GMOPLJJGBPO: false,
            BIIFELFEGNK: false,
            KGJHJMDCAOC: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OLKFPHPBPDL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OLKFPHPBPDL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OLKFPHPBPDL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OLKFPHPBPDL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OLKFPHPBPDL.proto\"\xd9\x01\n\x0bOLKFPHPBPDL\x12\x20\n\x0bHDMDAODJ\
    ACB\x18\x01\x20\x03(\rR\x0bHDMDAODJACB\x12\x20\n\x0bKNBDPFEIDNM\x18\x08\
    \x20\x01(\x08R\x0bKNBDPFEIDNM\x12\x20\n\x0bDBAHFEFGLMD\x18\x07\x20\x01(\
    \rR\x0bDBAHFEFGLMD\x12\x20\n\x0bGMOPLJJGBPO\x18\x02\x20\x01(\x08R\x0bGMO\
    PLJJGBPO\x12\x20\n\x0bBIIFELFEGNK\x18\x0e\x20\x01(\x08R\x0bBIIFELFEGNK\
    \x12\x20\n\x0bKGJHJMDCAOC\x18\r\x20\x01(\x08R\x0bKGJHJMDCAOCb\x06proto3\
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
            messages.push(OLKFPHPBPDL::generated_message_descriptor_data());
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
