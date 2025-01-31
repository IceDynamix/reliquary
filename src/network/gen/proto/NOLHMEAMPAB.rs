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

//! Generated file from `NOLHMEAMPAB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NOLHMEAMPAB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NOLHMEAMPAB {
    // message fields
    // @@protoc_insertion_point(field:NOLHMEAMPAB.AAECKEHLNEN)
    pub AAECKEHLNEN: ::std::vec::Vec<super::HIDIBJJAOJN::HIDIBJJAOJN>,
    // @@protoc_insertion_point(field:NOLHMEAMPAB.JPBPMBBDLIC)
    pub JPBPMBBDLIC: u32,
    // @@protoc_insertion_point(field:NOLHMEAMPAB.MBAIPCCHDCI)
    pub MBAIPCCHDCI: bool,
    // special fields
    // @@protoc_insertion_point(special_field:NOLHMEAMPAB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NOLHMEAMPAB {
    fn default() -> &'a NOLHMEAMPAB {
        <NOLHMEAMPAB as ::protobuf::Message>::default_instance()
    }
}

impl NOLHMEAMPAB {
    pub fn new() -> NOLHMEAMPAB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AAECKEHLNEN",
            |m: &NOLHMEAMPAB| { &m.AAECKEHLNEN },
            |m: &mut NOLHMEAMPAB| { &mut m.AAECKEHLNEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JPBPMBBDLIC",
            |m: &NOLHMEAMPAB| { &m.JPBPMBBDLIC },
            |m: &mut NOLHMEAMPAB| { &mut m.JPBPMBBDLIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MBAIPCCHDCI",
            |m: &NOLHMEAMPAB| { &m.MBAIPCCHDCI },
            |m: &mut NOLHMEAMPAB| { &mut m.MBAIPCCHDCI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NOLHMEAMPAB>(
            "NOLHMEAMPAB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NOLHMEAMPAB {
    const NAME: &'static str = "NOLHMEAMPAB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.AAECKEHLNEN.push(is.read_message()?);
                },
                120 => {
                    self.JPBPMBBDLIC = is.read_uint32()?;
                },
                96 => {
                    self.MBAIPCCHDCI = is.read_bool()?;
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
        for value in &self.AAECKEHLNEN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.JPBPMBBDLIC != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.JPBPMBBDLIC);
        }
        if self.MBAIPCCHDCI != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.AAECKEHLNEN {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.JPBPMBBDLIC != 0 {
            os.write_uint32(15, self.JPBPMBBDLIC)?;
        }
        if self.MBAIPCCHDCI != false {
            os.write_bool(12, self.MBAIPCCHDCI)?;
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

    fn new() -> NOLHMEAMPAB {
        NOLHMEAMPAB::new()
    }

    fn clear(&mut self) {
        self.AAECKEHLNEN.clear();
        self.JPBPMBBDLIC = 0;
        self.MBAIPCCHDCI = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NOLHMEAMPAB {
        static instance: NOLHMEAMPAB = NOLHMEAMPAB {
            AAECKEHLNEN: ::std::vec::Vec::new(),
            JPBPMBBDLIC: 0,
            MBAIPCCHDCI: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NOLHMEAMPAB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NOLHMEAMPAB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NOLHMEAMPAB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NOLHMEAMPAB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NOLHMEAMPAB.proto\x1a\x11HIDIBJJAOJN.proto\"\x81\x01\n\x0bNOLHMEAM\
    PAB\x12.\n\x0bAAECKEHLNEN\x18\t\x20\x03(\x0b2\x0c.HIDIBJJAOJNR\x0bAAECKE\
    HLNEN\x12\x20\n\x0bJPBPMBBDLIC\x18\x0f\x20\x01(\rR\x0bJPBPMBBDLIC\x12\
    \x20\n\x0bMBAIPCCHDCI\x18\x0c\x20\x01(\x08R\x0bMBAIPCCHDCIb\x06proto3\
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
            deps.push(super::HIDIBJJAOJN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NOLHMEAMPAB::generated_message_descriptor_data());
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
