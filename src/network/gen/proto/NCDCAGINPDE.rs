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

//! Generated file from `NCDCAGINPDE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NCDCAGINPDE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NCDCAGINPDE {
    // message fields
    // @@protoc_insertion_point(field:NCDCAGINPDE.KAJCDNEHIGP)
    pub KAJCDNEHIGP: ::std::vec::Vec<super::MFBBCCJHIOG::MFBBCCJHIOG>,
    // @@protoc_insertion_point(field:NCDCAGINPDE.MLFLLKMIADE)
    pub MLFLLKMIADE: u32,
    // @@protoc_insertion_point(field:NCDCAGINPDE.MJGLHJCKHMO)
    pub MJGLHJCKHMO: u32,
    // @@protoc_insertion_point(field:NCDCAGINPDE.NNBHLDDNLDE)
    pub NNBHLDDNLDE: ::std::vec::Vec<super::HHOKBPHNFNE::HHOKBPHNFNE>,
    // @@protoc_insertion_point(field:NCDCAGINPDE.CFJKBJHNIJM)
    pub CFJKBJHNIJM: u32,
    // @@protoc_insertion_point(field:NCDCAGINPDE.AKENJBAHIOM)
    pub AKENJBAHIOM: u32,
    // @@protoc_insertion_point(field:NCDCAGINPDE.KCFEECACMOD)
    pub KCFEECACMOD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NCDCAGINPDE.CDJFDJIAING)
    pub CDJFDJIAING: u32,
    // @@protoc_insertion_point(field:NCDCAGINPDE.JDMFGECPDPJ)
    pub JDMFGECPDPJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NCDCAGINPDE.HIMCAOKDNMP)
    pub HIMCAOKDNMP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NCDCAGINPDE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NCDCAGINPDE {
    fn default() -> &'a NCDCAGINPDE {
        <NCDCAGINPDE as ::protobuf::Message>::default_instance()
    }
}

impl NCDCAGINPDE {
    pub fn new() -> NCDCAGINPDE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KAJCDNEHIGP",
            |m: &NCDCAGINPDE| { &m.KAJCDNEHIGP },
            |m: &mut NCDCAGINPDE| { &mut m.KAJCDNEHIGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MLFLLKMIADE",
            |m: &NCDCAGINPDE| { &m.MLFLLKMIADE },
            |m: &mut NCDCAGINPDE| { &mut m.MLFLLKMIADE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MJGLHJCKHMO",
            |m: &NCDCAGINPDE| { &m.MJGLHJCKHMO },
            |m: &mut NCDCAGINPDE| { &mut m.MJGLHJCKHMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NNBHLDDNLDE",
            |m: &NCDCAGINPDE| { &m.NNBHLDDNLDE },
            |m: &mut NCDCAGINPDE| { &mut m.NNBHLDDNLDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFJKBJHNIJM",
            |m: &NCDCAGINPDE| { &m.CFJKBJHNIJM },
            |m: &mut NCDCAGINPDE| { &mut m.CFJKBJHNIJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKENJBAHIOM",
            |m: &NCDCAGINPDE| { &m.AKENJBAHIOM },
            |m: &mut NCDCAGINPDE| { &mut m.AKENJBAHIOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KCFEECACMOD",
            |m: &NCDCAGINPDE| { &m.KCFEECACMOD },
            |m: &mut NCDCAGINPDE| { &mut m.KCFEECACMOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CDJFDJIAING",
            |m: &NCDCAGINPDE| { &m.CDJFDJIAING },
            |m: &mut NCDCAGINPDE| { &mut m.CDJFDJIAING },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JDMFGECPDPJ",
            |m: &NCDCAGINPDE| { &m.JDMFGECPDPJ },
            |m: &mut NCDCAGINPDE| { &mut m.JDMFGECPDPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIMCAOKDNMP",
            |m: &NCDCAGINPDE| { &m.HIMCAOKDNMP },
            |m: &mut NCDCAGINPDE| { &mut m.HIMCAOKDNMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NCDCAGINPDE>(
            "NCDCAGINPDE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NCDCAGINPDE {
    const NAME: &'static str = "NCDCAGINPDE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.KAJCDNEHIGP.push(is.read_message()?);
                },
                16 => {
                    self.MLFLLKMIADE = is.read_uint32()?;
                },
                40 => {
                    self.MJGLHJCKHMO = is.read_uint32()?;
                },
                66 => {
                    self.NNBHLDDNLDE.push(is.read_message()?);
                },
                112 => {
                    self.CFJKBJHNIJM = is.read_uint32()?;
                },
                48 => {
                    self.AKENJBAHIOM = is.read_uint32()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.KCFEECACMOD)?;
                },
                80 => {
                    self.KCFEECACMOD.push(is.read_uint32()?);
                },
                8 => {
                    self.CDJFDJIAING = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.JDMFGECPDPJ)?;
                },
                120 => {
                    self.JDMFGECPDPJ.push(is.read_uint32()?);
                },
                104 => {
                    self.HIMCAOKDNMP = is.read_uint32()?;
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
        for value in &self.KAJCDNEHIGP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MLFLLKMIADE != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.MLFLLKMIADE);
        }
        if self.MJGLHJCKHMO != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.MJGLHJCKHMO);
        }
        for value in &self.NNBHLDDNLDE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CFJKBJHNIJM != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.CFJKBJHNIJM);
        }
        if self.AKENJBAHIOM != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.AKENJBAHIOM);
        }
        for value in &self.KCFEECACMOD {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.CDJFDJIAING != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CDJFDJIAING);
        }
        for value in &self.JDMFGECPDPJ {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.HIMCAOKDNMP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.HIMCAOKDNMP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KAJCDNEHIGP {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.MLFLLKMIADE != 0 {
            os.write_uint32(2, self.MLFLLKMIADE)?;
        }
        if self.MJGLHJCKHMO != 0 {
            os.write_uint32(5, self.MJGLHJCKHMO)?;
        }
        for v in &self.NNBHLDDNLDE {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.CFJKBJHNIJM != 0 {
            os.write_uint32(14, self.CFJKBJHNIJM)?;
        }
        if self.AKENJBAHIOM != 0 {
            os.write_uint32(6, self.AKENJBAHIOM)?;
        }
        for v in &self.KCFEECACMOD {
            os.write_uint32(10, *v)?;
        };
        if self.CDJFDJIAING != 0 {
            os.write_uint32(1, self.CDJFDJIAING)?;
        }
        for v in &self.JDMFGECPDPJ {
            os.write_uint32(15, *v)?;
        };
        if self.HIMCAOKDNMP != 0 {
            os.write_uint32(13, self.HIMCAOKDNMP)?;
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

    fn new() -> NCDCAGINPDE {
        NCDCAGINPDE::new()
    }

    fn clear(&mut self) {
        self.KAJCDNEHIGP.clear();
        self.MLFLLKMIADE = 0;
        self.MJGLHJCKHMO = 0;
        self.NNBHLDDNLDE.clear();
        self.CFJKBJHNIJM = 0;
        self.AKENJBAHIOM = 0;
        self.KCFEECACMOD.clear();
        self.CDJFDJIAING = 0;
        self.JDMFGECPDPJ.clear();
        self.HIMCAOKDNMP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NCDCAGINPDE {
        static instance: NCDCAGINPDE = NCDCAGINPDE {
            KAJCDNEHIGP: ::std::vec::Vec::new(),
            MLFLLKMIADE: 0,
            MJGLHJCKHMO: 0,
            NNBHLDDNLDE: ::std::vec::Vec::new(),
            CFJKBJHNIJM: 0,
            AKENJBAHIOM: 0,
            KCFEECACMOD: ::std::vec::Vec::new(),
            CDJFDJIAING: 0,
            JDMFGECPDPJ: ::std::vec::Vec::new(),
            HIMCAOKDNMP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NCDCAGINPDE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NCDCAGINPDE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NCDCAGINPDE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NCDCAGINPDE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NCDCAGINPDE.proto\x1a\x11HHOKBPHNFNE.proto\x1a\x11MFBBCCJHIOG.prot\
    o\"\xfd\x02\n\x0bNCDCAGINPDE\x12.\n\x0bKAJCDNEHIGP\x18\x0c\x20\x03(\x0b2\
    \x0c.MFBBCCJHIOGR\x0bKAJCDNEHIGP\x12\x20\n\x0bMLFLLKMIADE\x18\x02\x20\
    \x01(\rR\x0bMLFLLKMIADE\x12\x20\n\x0bMJGLHJCKHMO\x18\x05\x20\x01(\rR\x0b\
    MJGLHJCKHMO\x12.\n\x0bNNBHLDDNLDE\x18\x08\x20\x03(\x0b2\x0c.HHOKBPHNFNER\
    \x0bNNBHLDDNLDE\x12\x20\n\x0bCFJKBJHNIJM\x18\x0e\x20\x01(\rR\x0bCFJKBJHN\
    IJM\x12\x20\n\x0bAKENJBAHIOM\x18\x06\x20\x01(\rR\x0bAKENJBAHIOM\x12\x20\
    \n\x0bKCFEECACMOD\x18\n\x20\x03(\rR\x0bKCFEECACMOD\x12\x20\n\x0bCDJFDJIA\
    ING\x18\x01\x20\x01(\rR\x0bCDJFDJIAING\x12\x20\n\x0bJDMFGECPDPJ\x18\x0f\
    \x20\x03(\rR\x0bJDMFGECPDPJ\x12\x20\n\x0bHIMCAOKDNMP\x18\r\x20\x01(\rR\
    \x0bHIMCAOKDNMPb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::HHOKBPHNFNE::file_descriptor().clone());
            deps.push(super::MFBBCCJHIOG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NCDCAGINPDE::generated_message_descriptor_data());
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