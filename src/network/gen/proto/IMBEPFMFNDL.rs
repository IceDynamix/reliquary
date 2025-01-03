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

//! Generated file from `IMBEPFMFNDL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IMBEPFMFNDL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IMBEPFMFNDL {
    // message fields
    // @@protoc_insertion_point(field:IMBEPFMFNDL.KAIDMEGMCDA)
    pub KAIDMEGMCDA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IMBEPFMFNDL.FAGNBDALNJO)
    pub FAGNBDALNJO: ::std::vec::Vec<super::FINBBOKJBPL::FINBBOKJBPL>,
    // @@protoc_insertion_point(field:IMBEPFMFNDL.PFDBLEONMLM)
    pub PFDBLEONMLM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IMBEPFMFNDL.NGOHFJKLDAB)
    pub NGOHFJKLDAB: ::std::vec::Vec<super::HLPMMJEGFMI::HLPMMJEGFMI>,
    // @@protoc_insertion_point(field:IMBEPFMFNDL.HDOEGCKJEHP)
    pub HDOEGCKJEHP: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:IMBEPFMFNDL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IMBEPFMFNDL {
    fn default() -> &'a IMBEPFMFNDL {
        <IMBEPFMFNDL as ::protobuf::Message>::default_instance()
    }
}

impl IMBEPFMFNDL {
    pub fn new() -> IMBEPFMFNDL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KAIDMEGMCDA",
            |m: &IMBEPFMFNDL| { &m.KAIDMEGMCDA },
            |m: &mut IMBEPFMFNDL| { &mut m.KAIDMEGMCDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FAGNBDALNJO",
            |m: &IMBEPFMFNDL| { &m.FAGNBDALNJO },
            |m: &mut IMBEPFMFNDL| { &mut m.FAGNBDALNJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PFDBLEONMLM",
            |m: &IMBEPFMFNDL| { &m.PFDBLEONMLM },
            |m: &mut IMBEPFMFNDL| { &mut m.PFDBLEONMLM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGOHFJKLDAB",
            |m: &IMBEPFMFNDL| { &m.NGOHFJKLDAB },
            |m: &mut IMBEPFMFNDL| { &mut m.NGOHFJKLDAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HDOEGCKJEHP",
            |m: &IMBEPFMFNDL| { &m.HDOEGCKJEHP },
            |m: &mut IMBEPFMFNDL| { &mut m.HDOEGCKJEHP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IMBEPFMFNDL>(
            "IMBEPFMFNDL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IMBEPFMFNDL {
    const NAME: &'static str = "IMBEPFMFNDL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.KAIDMEGMCDA)?;
                },
                88 => {
                    self.KAIDMEGMCDA.push(is.read_uint32()?);
                },
                50 => {
                    self.FAGNBDALNJO.push(is.read_message()?);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.PFDBLEONMLM)?;
                },
                8 => {
                    self.PFDBLEONMLM.push(is.read_uint32()?);
                },
                42 => {
                    self.NGOHFJKLDAB.push(is.read_message()?);
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.HDOEGCKJEHP)?;
                },
                112 => {
                    self.HDOEGCKJEHP.push(is.read_uint32()?);
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
        for value in &self.KAIDMEGMCDA {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.FAGNBDALNJO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PFDBLEONMLM {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for value in &self.NGOHFJKLDAB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.HDOEGCKJEHP {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KAIDMEGMCDA {
            os.write_uint32(11, *v)?;
        };
        for v in &self.FAGNBDALNJO {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        for v in &self.PFDBLEONMLM {
            os.write_uint32(1, *v)?;
        };
        for v in &self.NGOHFJKLDAB {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.HDOEGCKJEHP {
            os.write_uint32(14, *v)?;
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

    fn new() -> IMBEPFMFNDL {
        IMBEPFMFNDL::new()
    }

    fn clear(&mut self) {
        self.KAIDMEGMCDA.clear();
        self.FAGNBDALNJO.clear();
        self.PFDBLEONMLM.clear();
        self.NGOHFJKLDAB.clear();
        self.HDOEGCKJEHP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IMBEPFMFNDL {
        static instance: IMBEPFMFNDL = IMBEPFMFNDL {
            KAIDMEGMCDA: ::std::vec::Vec::new(),
            FAGNBDALNJO: ::std::vec::Vec::new(),
            PFDBLEONMLM: ::std::vec::Vec::new(),
            NGOHFJKLDAB: ::std::vec::Vec::new(),
            HDOEGCKJEHP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IMBEPFMFNDL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IMBEPFMFNDL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IMBEPFMFNDL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IMBEPFMFNDL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IMBEPFMFNDL.proto\x1a\x11FINBBOKJBPL.proto\x1a\x11HLPMMJEGFMI.prot\
    o\"\xd3\x01\n\x0bIMBEPFMFNDL\x12\x20\n\x0bKAIDMEGMCDA\x18\x0b\x20\x03(\r\
    R\x0bKAIDMEGMCDA\x12.\n\x0bFAGNBDALNJO\x18\x06\x20\x03(\x0b2\x0c.FINBBOK\
    JBPLR\x0bFAGNBDALNJO\x12\x20\n\x0bPFDBLEONMLM\x18\x01\x20\x03(\rR\x0bPFD\
    BLEONMLM\x12.\n\x0bNGOHFJKLDAB\x18\x05\x20\x03(\x0b2\x0c.HLPMMJEGFMIR\
    \x0bNGOHFJKLDAB\x12\x20\n\x0bHDOEGCKJEHP\x18\x0e\x20\x03(\rR\x0bHDOEGCKJ\
    EHPb\x06proto3\
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
            deps.push(super::FINBBOKJBPL::file_descriptor().clone());
            deps.push(super::HLPMMJEGFMI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IMBEPFMFNDL::generated_message_descriptor_data());
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
