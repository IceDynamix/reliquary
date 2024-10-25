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

//! Generated file from `EFNNGHDBBHH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EFNNGHDBBHH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EFNNGHDBBHH {
    // message fields
    // @@protoc_insertion_point(field:EFNNGHDBBHH.NGHKJJPOLNJ)
    pub NGHKJJPOLNJ: ::std::vec::Vec<super::JGJOCLOCIBP::JGJOCLOCIBP>,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.OFNEIPIOMDP)
    pub OFNEIPIOMDP: ::std::string::String,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.LFDDAGOMEGB)
    pub LFDDAGOMEGB: ::std::string::String,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.LJOJGLFBHLP)
    pub LJOJGLFBHLP: ::std::vec::Vec<super::HJOKCPFFFAF::HJOKCPFFFAF>,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.OGPKNEFGNAN)
    pub OGPKNEFGNAN: u32,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.KPFMEBBLJCD)
    pub KPFMEBBLJCD: u32,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.DMBPCJBBCGE)
    pub DMBPCJBBCGE: u32,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.LKNNDKMAEPM)
    pub LKNNDKMAEPM: u32,
    // @@protoc_insertion_point(field:EFNNGHDBBHH.LIKMIJOKDPM)
    pub LIKMIJOKDPM: ::protobuf::MessageField<super::ANOAJMPBFII::ANOAJMPBFII>,
    // special fields
    // @@protoc_insertion_point(special_field:EFNNGHDBBHH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EFNNGHDBBHH {
    fn default() -> &'a EFNNGHDBBHH {
        <EFNNGHDBBHH as ::protobuf::Message>::default_instance()
    }
}

impl EFNNGHDBBHH {
    pub fn new() -> EFNNGHDBBHH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGHKJJPOLNJ",
            |m: &EFNNGHDBBHH| { &m.NGHKJJPOLNJ },
            |m: &mut EFNNGHDBBHH| { &mut m.NGHKJJPOLNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFNEIPIOMDP",
            |m: &EFNNGHDBBHH| { &m.OFNEIPIOMDP },
            |m: &mut EFNNGHDBBHH| { &mut m.OFNEIPIOMDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFDDAGOMEGB",
            |m: &EFNNGHDBBHH| { &m.LFDDAGOMEGB },
            |m: &mut EFNNGHDBBHH| { &mut m.LFDDAGOMEGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LJOJGLFBHLP",
            |m: &EFNNGHDBBHH| { &m.LJOJGLFBHLP },
            |m: &mut EFNNGHDBBHH| { &mut m.LJOJGLFBHLP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGPKNEFGNAN",
            |m: &EFNNGHDBBHH| { &m.OGPKNEFGNAN },
            |m: &mut EFNNGHDBBHH| { &mut m.OGPKNEFGNAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPFMEBBLJCD",
            |m: &EFNNGHDBBHH| { &m.KPFMEBBLJCD },
            |m: &mut EFNNGHDBBHH| { &mut m.KPFMEBBLJCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMBPCJBBCGE",
            |m: &EFNNGHDBBHH| { &m.DMBPCJBBCGE },
            |m: &mut EFNNGHDBBHH| { &mut m.DMBPCJBBCGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKNNDKMAEPM",
            |m: &EFNNGHDBBHH| { &m.LKNNDKMAEPM },
            |m: &mut EFNNGHDBBHH| { &mut m.LKNNDKMAEPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ANOAJMPBFII::ANOAJMPBFII>(
            "LIKMIJOKDPM",
            |m: &EFNNGHDBBHH| { &m.LIKMIJOKDPM },
            |m: &mut EFNNGHDBBHH| { &mut m.LIKMIJOKDPM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EFNNGHDBBHH>(
            "EFNNGHDBBHH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EFNNGHDBBHH {
    const NAME: &'static str = "EFNNGHDBBHH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.NGHKJJPOLNJ.push(is.read_message()?);
                },
                18 => {
                    self.OFNEIPIOMDP = is.read_string()?;
                },
                26 => {
                    self.LFDDAGOMEGB = is.read_string()?;
                },
                34 => {
                    self.LJOJGLFBHLP.push(is.read_message()?);
                },
                40 => {
                    self.OGPKNEFGNAN = is.read_uint32()?;
                },
                48 => {
                    self.KPFMEBBLJCD = is.read_uint32()?;
                },
                56 => {
                    self.DMBPCJBBCGE = is.read_uint32()?;
                },
                64 => {
                    self.LKNNDKMAEPM = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LIKMIJOKDPM)?;
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
        for value in &self.NGHKJJPOLNJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.OFNEIPIOMDP.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.OFNEIPIOMDP);
        }
        if !self.LFDDAGOMEGB.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.LFDDAGOMEGB);
        }
        for value in &self.LJOJGLFBHLP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.OGPKNEFGNAN != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.OGPKNEFGNAN);
        }
        if self.KPFMEBBLJCD != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KPFMEBBLJCD);
        }
        if self.DMBPCJBBCGE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DMBPCJBBCGE);
        }
        if self.LKNNDKMAEPM != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LKNNDKMAEPM);
        }
        if let Some(v) = self.LIKMIJOKDPM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NGHKJJPOLNJ {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if !self.OFNEIPIOMDP.is_empty() {
            os.write_string(2, &self.OFNEIPIOMDP)?;
        }
        if !self.LFDDAGOMEGB.is_empty() {
            os.write_string(3, &self.LFDDAGOMEGB)?;
        }
        for v in &self.LJOJGLFBHLP {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.OGPKNEFGNAN != 0 {
            os.write_uint32(5, self.OGPKNEFGNAN)?;
        }
        if self.KPFMEBBLJCD != 0 {
            os.write_uint32(6, self.KPFMEBBLJCD)?;
        }
        if self.DMBPCJBBCGE != 0 {
            os.write_uint32(7, self.DMBPCJBBCGE)?;
        }
        if self.LKNNDKMAEPM != 0 {
            os.write_uint32(8, self.LKNNDKMAEPM)?;
        }
        if let Some(v) = self.LIKMIJOKDPM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> EFNNGHDBBHH {
        EFNNGHDBBHH::new()
    }

    fn clear(&mut self) {
        self.NGHKJJPOLNJ.clear();
        self.OFNEIPIOMDP.clear();
        self.LFDDAGOMEGB.clear();
        self.LJOJGLFBHLP.clear();
        self.OGPKNEFGNAN = 0;
        self.KPFMEBBLJCD = 0;
        self.DMBPCJBBCGE = 0;
        self.LKNNDKMAEPM = 0;
        self.LIKMIJOKDPM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EFNNGHDBBHH {
        static instance: EFNNGHDBBHH = EFNNGHDBBHH {
            NGHKJJPOLNJ: ::std::vec::Vec::new(),
            OFNEIPIOMDP: ::std::string::String::new(),
            LFDDAGOMEGB: ::std::string::String::new(),
            LJOJGLFBHLP: ::std::vec::Vec::new(),
            OGPKNEFGNAN: 0,
            KPFMEBBLJCD: 0,
            DMBPCJBBCGE: 0,
            LKNNDKMAEPM: 0,
            LIKMIJOKDPM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EFNNGHDBBHH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EFNNGHDBBHH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EFNNGHDBBHH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EFNNGHDBBHH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EFNNGHDBBHH.proto\x1a\x11ANOAJMPBFII.proto\x1a\x11HJOKCPFFFAF.prot\
    o\x1a\x11JGJOCLOCIBP.proto\"\xe9\x02\n\x0bEFNNGHDBBHH\x12.\n\x0bNGHKJJPO\
    LNJ\x18\x01\x20\x03(\x0b2\x0c.JGJOCLOCIBPR\x0bNGHKJJPOLNJ\x12\x20\n\x0bO\
    FNEIPIOMDP\x18\x02\x20\x01(\tR\x0bOFNEIPIOMDP\x12\x20\n\x0bLFDDAGOMEGB\
    \x18\x03\x20\x01(\tR\x0bLFDDAGOMEGB\x12.\n\x0bLJOJGLFBHLP\x18\x04\x20\
    \x03(\x0b2\x0c.HJOKCPFFFAFR\x0bLJOJGLFBHLP\x12\x20\n\x0bOGPKNEFGNAN\x18\
    \x05\x20\x01(\rR\x0bOGPKNEFGNAN\x12\x20\n\x0bKPFMEBBLJCD\x18\x06\x20\x01\
    (\rR\x0bKPFMEBBLJCD\x12\x20\n\x0bDMBPCJBBCGE\x18\x07\x20\x01(\rR\x0bDMBP\
    CJBBCGE\x12\x20\n\x0bLKNNDKMAEPM\x18\x08\x20\x01(\rR\x0bLKNNDKMAEPM\x12.\
    \n\x0bLIKMIJOKDPM\x18\t\x20\x01(\x0b2\x0c.ANOAJMPBFIIR\x0bLIKMIJOKDPMb\
    \x06proto3\
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
            deps.push(super::ANOAJMPBFII::file_descriptor().clone());
            deps.push(super::HJOKCPFFFAF::file_descriptor().clone());
            deps.push(super::JGJOCLOCIBP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EFNNGHDBBHH::generated_message_descriptor_data());
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
