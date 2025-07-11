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

//! Generated file from `ILJJBGIFDPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ILJJBGIFDPE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ILJJBGIFDPE {
    // message fields
    // @@protoc_insertion_point(field:ILJJBGIFDPE.EAJPDPCDJPK)
    pub EAJPDPCDJPK: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.OEBAFBIGMBC)
    pub OEBAFBIGMBC: ::std::vec::Vec<super::NPAIINEKEFB::NPAIINEKEFB>,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.NIJMJBMCFJF)
    pub NIJMJBMCFJF: u32,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.BJELCLBGALF)
    pub BJELCLBGALF: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.FOFHIEIICPB)
    pub FOFHIEIICPB: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.NLAOMPDENKK)
    pub NLAOMPDENKK: u32,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.HDKAFNKHALA)
    pub HDKAFNKHALA: u32,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.CCIGDJCGAMD)
    pub CCIGDJCGAMD: u32,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.PEHINGJKGCB)
    pub PEHINGJKGCB: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:ILJJBGIFDPE.MEMBICNIFLI)
    pub MEMBICNIFLI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ILJJBGIFDPE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ILJJBGIFDPE {
    fn default() -> &'a ILJJBGIFDPE {
        <ILJJBGIFDPE as ::protobuf::Message>::default_instance()
    }
}

impl ILJJBGIFDPE {
    pub fn new() -> ILJJBGIFDPE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "EAJPDPCDJPK",
            |m: &ILJJBGIFDPE| { &m.EAJPDPCDJPK },
            |m: &mut ILJJBGIFDPE| { &mut m.EAJPDPCDJPK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OEBAFBIGMBC",
            |m: &ILJJBGIFDPE| { &m.OEBAFBIGMBC },
            |m: &mut ILJJBGIFDPE| { &mut m.OEBAFBIGMBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIJMJBMCFJF",
            |m: &ILJJBGIFDPE| { &m.NIJMJBMCFJF },
            |m: &mut ILJJBGIFDPE| { &mut m.NIJMJBMCFJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "BJELCLBGALF",
            |m: &ILJJBGIFDPE| { &m.BJELCLBGALF },
            |m: &mut ILJJBGIFDPE| { &mut m.BJELCLBGALF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "FOFHIEIICPB",
            |m: &ILJJBGIFDPE| { &m.FOFHIEIICPB },
            |m: &mut ILJJBGIFDPE| { &mut m.FOFHIEIICPB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLAOMPDENKK",
            |m: &ILJJBGIFDPE| { &m.NLAOMPDENKK },
            |m: &mut ILJJBGIFDPE| { &mut m.NLAOMPDENKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDKAFNKHALA",
            |m: &ILJJBGIFDPE| { &m.HDKAFNKHALA },
            |m: &mut ILJJBGIFDPE| { &mut m.HDKAFNKHALA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCIGDJCGAMD",
            |m: &ILJJBGIFDPE| { &m.CCIGDJCGAMD },
            |m: &mut ILJJBGIFDPE| { &mut m.CCIGDJCGAMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "PEHINGJKGCB",
            |m: &ILJJBGIFDPE| { &m.PEHINGJKGCB },
            |m: &mut ILJJBGIFDPE| { &mut m.PEHINGJKGCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MEMBICNIFLI",
            |m: &ILJJBGIFDPE| { &m.MEMBICNIFLI },
            |m: &mut ILJJBGIFDPE| { &mut m.MEMBICNIFLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ILJJBGIFDPE>(
            "ILJJBGIFDPE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ILJJBGIFDPE {
    const NAME: &'static str = "ILJJBGIFDPE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EAJPDPCDJPK)?;
                },
                106 => {
                    self.OEBAFBIGMBC.push(is.read_message()?);
                },
                64 => {
                    self.NIJMJBMCFJF = is.read_uint32()?;
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.BJELCLBGALF.insert(key, value);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FOFHIEIICPB)?;
                },
                40 => {
                    self.NLAOMPDENKK = is.read_uint32()?;
                },
                24 => {
                    self.HDKAFNKHALA = is.read_uint32()?;
                },
                8 => {
                    self.CCIGDJCGAMD = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PEHINGJKGCB)?;
                },
                48 => {
                    self.MEMBICNIFLI = is.read_uint32()?;
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
        if let Some(v) = self.EAJPDPCDJPK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.OEBAFBIGMBC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NIJMJBMCFJF != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.NIJMJBMCFJF);
        }
        for (k, v) in &self.BJELCLBGALF {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let Some(v) = self.FOFHIEIICPB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NLAOMPDENKK != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NLAOMPDENKK);
        }
        if self.HDKAFNKHALA != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HDKAFNKHALA);
        }
        if self.CCIGDJCGAMD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CCIGDJCGAMD);
        }
        if let Some(v) = self.PEHINGJKGCB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MEMBICNIFLI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.MEMBICNIFLI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.EAJPDPCDJPK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        for v in &self.OEBAFBIGMBC {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.NIJMJBMCFJF != 0 {
            os.write_uint32(8, self.NIJMJBMCFJF)?;
        }
        for (k, v) in &self.BJELCLBGALF {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.FOFHIEIICPB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.NLAOMPDENKK != 0 {
            os.write_uint32(5, self.NLAOMPDENKK)?;
        }
        if self.HDKAFNKHALA != 0 {
            os.write_uint32(3, self.HDKAFNKHALA)?;
        }
        if self.CCIGDJCGAMD != 0 {
            os.write_uint32(1, self.CCIGDJCGAMD)?;
        }
        if let Some(v) = self.PEHINGJKGCB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.MEMBICNIFLI != 0 {
            os.write_uint32(6, self.MEMBICNIFLI)?;
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

    fn new() -> ILJJBGIFDPE {
        ILJJBGIFDPE::new()
    }

    fn clear(&mut self) {
        self.EAJPDPCDJPK.clear();
        self.OEBAFBIGMBC.clear();
        self.NIJMJBMCFJF = 0;
        self.BJELCLBGALF.clear();
        self.FOFHIEIICPB.clear();
        self.NLAOMPDENKK = 0;
        self.HDKAFNKHALA = 0;
        self.CCIGDJCGAMD = 0;
        self.PEHINGJKGCB.clear();
        self.MEMBICNIFLI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ILJJBGIFDPE {
        static instance: ::protobuf::rt::Lazy<ILJJBGIFDPE> = ::protobuf::rt::Lazy::new();
        instance.get(ILJJBGIFDPE::new)
    }
}

impl ::protobuf::MessageFull for ILJJBGIFDPE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ILJJBGIFDPE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ILJJBGIFDPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ILJJBGIFDPE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ILJJBGIFDPE.proto\x1a\x11IIKNGNHDMFI.proto\x1a\x11NPAIINEKEFB.prot\
    o\"\xf8\x03\n\x0bILJJBGIFDPE\x12.\n\x0bEAJPDPCDJPK\x18\x0e\x20\x01(\x0b2\
    \x0c.IIKNGNHDMFIR\x0bEAJPDPCDJPK\x12.\n\x0bOEBAFBIGMBC\x18\r\x20\x03(\
    \x0b2\x0c.NPAIINEKEFBR\x0bOEBAFBIGMBC\x12\x20\n\x0bNIJMJBMCFJF\x18\x08\
    \x20\x01(\rR\x0bNIJMJBMCFJF\x12?\n\x0bBJELCLBGALF\x18\x02\x20\x03(\x0b2\
    \x1d.ILJJBGIFDPE.BJELCLBGALFEntryR\x0bBJELCLBGALF\x12.\n\x0bFOFHIEIICPB\
    \x18\n\x20\x01(\x0b2\x0c.IIKNGNHDMFIR\x0bFOFHIEIICPB\x12\x20\n\x0bNLAOMP\
    DENKK\x18\x05\x20\x01(\rR\x0bNLAOMPDENKK\x12\x20\n\x0bHDKAFNKHALA\x18\
    \x03\x20\x01(\rR\x0bHDKAFNKHALA\x12\x20\n\x0bCCIGDJCGAMD\x18\x01\x20\x01\
    (\rR\x0bCCIGDJCGAMD\x12.\n\x0bPEHINGJKGCB\x18\t\x20\x01(\x0b2\x0c.IIKNGN\
    HDMFIR\x0bPEHINGJKGCB\x12\x20\n\x0bMEMBICNIFLI\x18\x06\x20\x01(\rR\x0bME\
    MBICNIFLI\x1a>\n\x10BJELCLBGALFEntry\x12\x10\n\x03key\x18\x01\x20\x01(\r\
    R\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01b\x06p\
    roto3\
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
            deps.push(super::IIKNGNHDMFI::file_descriptor().clone());
            deps.push(super::NPAIINEKEFB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ILJJBGIFDPE::generated_message_descriptor_data());
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
