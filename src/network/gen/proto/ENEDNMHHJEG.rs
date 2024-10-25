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

//! Generated file from `ENEDNMHHJEG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ENEDNMHHJEG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ENEDNMHHJEG {
    // message fields
    // @@protoc_insertion_point(field:ENEDNMHHJEG.IIBMDAJNHLA)
    pub IIBMDAJNHLA: i64,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.MMKMCHDADFD)
    pub MMKMCHDADFD: i64,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.HNPCPDJPGLL)
    pub HNPCPDJPGLL: u32,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.AOALMNPJPCG)
    pub AOALMNPJPCG: ::protobuf::MessageField<super::IPFLIHMFLPC::IPFLIHMFLPC>,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.LHMBNMIOKHB)
    pub LHMBNMIOKHB: u32,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.NPBKDNFAECA)
    pub NPBKDNFAECA: u32,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.OIOBBBDKPFE)
    pub OIOBBBDKPFE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.IIHHOHFLKMO)
    pub IIHHOHFLKMO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.ADPFEPDLNCB)
    pub ADPFEPDLNCB: ::std::string::String,
    // @@protoc_insertion_point(field:ENEDNMHHJEG.NBLLOMGDDPO)
    pub NBLLOMGDDPO: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ENEDNMHHJEG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ENEDNMHHJEG {
    fn default() -> &'a ENEDNMHHJEG {
        <ENEDNMHHJEG as ::protobuf::Message>::default_instance()
    }
}

impl ENEDNMHHJEG {
    pub fn new() -> ENEDNMHHJEG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIBMDAJNHLA",
            |m: &ENEDNMHHJEG| { &m.IIBMDAJNHLA },
            |m: &mut ENEDNMHHJEG| { &mut m.IIBMDAJNHLA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMKMCHDADFD",
            |m: &ENEDNMHHJEG| { &m.MMKMCHDADFD },
            |m: &mut ENEDNMHHJEG| { &mut m.MMKMCHDADFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNPCPDJPGLL",
            |m: &ENEDNMHHJEG| { &m.HNPCPDJPGLL },
            |m: &mut ENEDNMHHJEG| { &mut m.HNPCPDJPGLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IPFLIHMFLPC::IPFLIHMFLPC>(
            "AOALMNPJPCG",
            |m: &ENEDNMHHJEG| { &m.AOALMNPJPCG },
            |m: &mut ENEDNMHHJEG| { &mut m.AOALMNPJPCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHMBNMIOKHB",
            |m: &ENEDNMHHJEG| { &m.LHMBNMIOKHB },
            |m: &mut ENEDNMHHJEG| { &mut m.LHMBNMIOKHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPBKDNFAECA",
            |m: &ENEDNMHHJEG| { &m.NPBKDNFAECA },
            |m: &mut ENEDNMHHJEG| { &mut m.NPBKDNFAECA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OIOBBBDKPFE",
            |m: &ENEDNMHHJEG| { &m.OIOBBBDKPFE },
            |m: &mut ENEDNMHHJEG| { &mut m.OIOBBBDKPFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IIHHOHFLKMO",
            |m: &ENEDNMHHJEG| { &m.IIHHOHFLKMO },
            |m: &mut ENEDNMHHJEG| { &mut m.IIHHOHFLKMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADPFEPDLNCB",
            |m: &ENEDNMHHJEG| { &m.ADPFEPDLNCB },
            |m: &mut ENEDNMHHJEG| { &mut m.ADPFEPDLNCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLLOMGDDPO",
            |m: &ENEDNMHHJEG| { &m.NBLLOMGDDPO },
            |m: &mut ENEDNMHHJEG| { &mut m.NBLLOMGDDPO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ENEDNMHHJEG>(
            "ENEDNMHHJEG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ENEDNMHHJEG {
    const NAME: &'static str = "ENEDNMHHJEG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.IIBMDAJNHLA = is.read_int64()?;
                },
                16 => {
                    self.MMKMCHDADFD = is.read_int64()?;
                },
                88 => {
                    self.HNPCPDJPGLL = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AOALMNPJPCG)?;
                },
                96 => {
                    self.LHMBNMIOKHB = is.read_uint32()?;
                },
                104 => {
                    self.NPBKDNFAECA = is.read_uint32()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.OIOBBBDKPFE)?;
                },
                48 => {
                    self.OIOBBBDKPFE.push(is.read_uint32()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.IIHHOHFLKMO)?;
                },
                64 => {
                    self.IIHHOHFLKMO.push(is.read_uint32()?);
                },
                58 => {
                    self.ADPFEPDLNCB = is.read_string()?;
                },
                34 => {
                    self.NBLLOMGDDPO = is.read_string()?;
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
        if self.IIBMDAJNHLA != 0 {
            my_size += ::protobuf::rt::int64_size(15, self.IIBMDAJNHLA);
        }
        if self.MMKMCHDADFD != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.MMKMCHDADFD);
        }
        if self.HNPCPDJPGLL != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HNPCPDJPGLL);
        }
        if let Some(v) = self.AOALMNPJPCG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LHMBNMIOKHB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LHMBNMIOKHB);
        }
        if self.NPBKDNFAECA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.NPBKDNFAECA);
        }
        for value in &self.OIOBBBDKPFE {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        for value in &self.IIHHOHFLKMO {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if !self.ADPFEPDLNCB.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.ADPFEPDLNCB);
        }
        if !self.NBLLOMGDDPO.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.NBLLOMGDDPO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IIBMDAJNHLA != 0 {
            os.write_int64(15, self.IIBMDAJNHLA)?;
        }
        if self.MMKMCHDADFD != 0 {
            os.write_int64(2, self.MMKMCHDADFD)?;
        }
        if self.HNPCPDJPGLL != 0 {
            os.write_uint32(11, self.HNPCPDJPGLL)?;
        }
        if let Some(v) = self.AOALMNPJPCG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.LHMBNMIOKHB != 0 {
            os.write_uint32(12, self.LHMBNMIOKHB)?;
        }
        if self.NPBKDNFAECA != 0 {
            os.write_uint32(13, self.NPBKDNFAECA)?;
        }
        for v in &self.OIOBBBDKPFE {
            os.write_uint32(6, *v)?;
        };
        for v in &self.IIHHOHFLKMO {
            os.write_uint32(8, *v)?;
        };
        if !self.ADPFEPDLNCB.is_empty() {
            os.write_string(7, &self.ADPFEPDLNCB)?;
        }
        if !self.NBLLOMGDDPO.is_empty() {
            os.write_string(4, &self.NBLLOMGDDPO)?;
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

    fn new() -> ENEDNMHHJEG {
        ENEDNMHHJEG::new()
    }

    fn clear(&mut self) {
        self.IIBMDAJNHLA = 0;
        self.MMKMCHDADFD = 0;
        self.HNPCPDJPGLL = 0;
        self.AOALMNPJPCG.clear();
        self.LHMBNMIOKHB = 0;
        self.NPBKDNFAECA = 0;
        self.OIOBBBDKPFE.clear();
        self.IIHHOHFLKMO.clear();
        self.ADPFEPDLNCB.clear();
        self.NBLLOMGDDPO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ENEDNMHHJEG {
        static instance: ENEDNMHHJEG = ENEDNMHHJEG {
            IIBMDAJNHLA: 0,
            MMKMCHDADFD: 0,
            HNPCPDJPGLL: 0,
            AOALMNPJPCG: ::protobuf::MessageField::none(),
            LHMBNMIOKHB: 0,
            NPBKDNFAECA: 0,
            OIOBBBDKPFE: ::std::vec::Vec::new(),
            IIHHOHFLKMO: ::std::vec::Vec::new(),
            ADPFEPDLNCB: ::std::string::String::new(),
            NBLLOMGDDPO: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ENEDNMHHJEG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ENEDNMHHJEG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ENEDNMHHJEG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ENEDNMHHJEG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ENEDNMHHJEG.proto\x1a\x11IPFLIHMFLPC.proto\"\xef\x02\n\x0bENEDNMHH\
    JEG\x12\x20\n\x0bIIBMDAJNHLA\x18\x0f\x20\x01(\x03R\x0bIIBMDAJNHLA\x12\
    \x20\n\x0bMMKMCHDADFD\x18\x02\x20\x01(\x03R\x0bMMKMCHDADFD\x12\x20\n\x0b\
    HNPCPDJPGLL\x18\x0b\x20\x01(\rR\x0bHNPCPDJPGLL\x12.\n\x0bAOALMNPJPCG\x18\
    \t\x20\x01(\x0b2\x0c.IPFLIHMFLPCR\x0bAOALMNPJPCG\x12\x20\n\x0bLHMBNMIOKH\
    B\x18\x0c\x20\x01(\rR\x0bLHMBNMIOKHB\x12\x20\n\x0bNPBKDNFAECA\x18\r\x20\
    \x01(\rR\x0bNPBKDNFAECA\x12\x20\n\x0bOIOBBBDKPFE\x18\x06\x20\x03(\rR\x0b\
    OIOBBBDKPFE\x12\x20\n\x0bIIHHOHFLKMO\x18\x08\x20\x03(\rR\x0bIIHHOHFLKMO\
    \x12\x20\n\x0bADPFEPDLNCB\x18\x07\x20\x01(\tR\x0bADPFEPDLNCB\x12\x20\n\
    \x0bNBLLOMGDDPO\x18\x04\x20\x01(\tR\x0bNBLLOMGDDPOb\x06proto3\
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
            deps.push(super::IPFLIHMFLPC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ENEDNMHHJEG::generated_message_descriptor_data());
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
