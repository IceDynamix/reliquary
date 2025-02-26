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

//! Generated file from `HBLDPBBKHPB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HBLDPBBKHPB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HBLDPBBKHPB {
    // message fields
    // @@protoc_insertion_point(field:HBLDPBBKHPB.KKKLNGHCFJN)
    pub KKKLNGHCFJN: u32,
    // @@protoc_insertion_point(field:HBLDPBBKHPB.DNAPMNEMHBO)
    pub DNAPMNEMHBO: u32,
    // @@protoc_insertion_point(field:HBLDPBBKHPB.IBJFAFABGBK)
    pub IBJFAFABGBK: u32,
    // @@protoc_insertion_point(field:HBLDPBBKHPB.CDAFGDBMAJN)
    pub CDAFGDBMAJN: ::std::vec::Vec<super::OKKGELPAHEF::OKKGELPAHEF>,
    // @@protoc_insertion_point(field:HBLDPBBKHPB.KFLNKEBBONK)
    pub KFLNKEBBONK: ::std::vec::Vec<super::ELCPGNINPIN::ELCPGNINPIN>,
    // @@protoc_insertion_point(field:HBLDPBBKHPB.JEINBMLFCBP)
    pub JEINBMLFCBP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HBLDPBBKHPB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HBLDPBBKHPB {
    fn default() -> &'a HBLDPBBKHPB {
        <HBLDPBBKHPB as ::protobuf::Message>::default_instance()
    }
}

impl HBLDPBBKHPB {
    pub fn new() -> HBLDPBBKHPB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKKLNGHCFJN",
            |m: &HBLDPBBKHPB| { &m.KKKLNGHCFJN },
            |m: &mut HBLDPBBKHPB| { &mut m.KKKLNGHCFJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNAPMNEMHBO",
            |m: &HBLDPBBKHPB| { &m.DNAPMNEMHBO },
            |m: &mut HBLDPBBKHPB| { &mut m.DNAPMNEMHBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBJFAFABGBK",
            |m: &HBLDPBBKHPB| { &m.IBJFAFABGBK },
            |m: &mut HBLDPBBKHPB| { &mut m.IBJFAFABGBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CDAFGDBMAJN",
            |m: &HBLDPBBKHPB| { &m.CDAFGDBMAJN },
            |m: &mut HBLDPBBKHPB| { &mut m.CDAFGDBMAJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KFLNKEBBONK",
            |m: &HBLDPBBKHPB| { &m.KFLNKEBBONK },
            |m: &mut HBLDPBBKHPB| { &mut m.KFLNKEBBONK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JEINBMLFCBP",
            |m: &HBLDPBBKHPB| { &m.JEINBMLFCBP },
            |m: &mut HBLDPBBKHPB| { &mut m.JEINBMLFCBP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HBLDPBBKHPB>(
            "HBLDPBBKHPB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HBLDPBBKHPB {
    const NAME: &'static str = "HBLDPBBKHPB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.KKKLNGHCFJN = is.read_uint32()?;
                },
                16 => {
                    self.DNAPMNEMHBO = is.read_uint32()?;
                },
                24 => {
                    self.IBJFAFABGBK = is.read_uint32()?;
                },
                34 => {
                    self.CDAFGDBMAJN.push(is.read_message()?);
                },
                42 => {
                    self.KFLNKEBBONK.push(is.read_message()?);
                },
                48 => {
                    self.JEINBMLFCBP = is.read_uint32()?;
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
        if self.KKKLNGHCFJN != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.KKKLNGHCFJN);
        }
        if self.DNAPMNEMHBO != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.DNAPMNEMHBO);
        }
        if self.IBJFAFABGBK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.IBJFAFABGBK);
        }
        for value in &self.CDAFGDBMAJN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.KFLNKEBBONK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.JEINBMLFCBP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.JEINBMLFCBP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KKKLNGHCFJN != 0 {
            os.write_uint32(1, self.KKKLNGHCFJN)?;
        }
        if self.DNAPMNEMHBO != 0 {
            os.write_uint32(2, self.DNAPMNEMHBO)?;
        }
        if self.IBJFAFABGBK != 0 {
            os.write_uint32(3, self.IBJFAFABGBK)?;
        }
        for v in &self.CDAFGDBMAJN {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.KFLNKEBBONK {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.JEINBMLFCBP != 0 {
            os.write_uint32(6, self.JEINBMLFCBP)?;
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

    fn new() -> HBLDPBBKHPB {
        HBLDPBBKHPB::new()
    }

    fn clear(&mut self) {
        self.KKKLNGHCFJN = 0;
        self.DNAPMNEMHBO = 0;
        self.IBJFAFABGBK = 0;
        self.CDAFGDBMAJN.clear();
        self.KFLNKEBBONK.clear();
        self.JEINBMLFCBP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HBLDPBBKHPB {
        static instance: HBLDPBBKHPB = HBLDPBBKHPB {
            KKKLNGHCFJN: 0,
            DNAPMNEMHBO: 0,
            IBJFAFABGBK: 0,
            CDAFGDBMAJN: ::std::vec::Vec::new(),
            KFLNKEBBONK: ::std::vec::Vec::new(),
            JEINBMLFCBP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HBLDPBBKHPB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HBLDPBBKHPB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HBLDPBBKHPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HBLDPBBKHPB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HBLDPBBKHPB.proto\x1a\x11ELCPGNINPIN.proto\x1a\x11OKKGELPAHEF.prot\
    o\"\xf5\x01\n\x0bHBLDPBBKHPB\x12\x20\n\x0bKKKLNGHCFJN\x18\x01\x20\x01(\r\
    R\x0bKKKLNGHCFJN\x12\x20\n\x0bDNAPMNEMHBO\x18\x02\x20\x01(\rR\x0bDNAPMNE\
    MHBO\x12\x20\n\x0bIBJFAFABGBK\x18\x03\x20\x01(\rR\x0bIBJFAFABGBK\x12.\n\
    \x0bCDAFGDBMAJN\x18\x04\x20\x03(\x0b2\x0c.OKKGELPAHEFR\x0bCDAFGDBMAJN\
    \x12.\n\x0bKFLNKEBBONK\x18\x05\x20\x03(\x0b2\x0c.ELCPGNINPINR\x0bKFLNKEB\
    BONK\x12\x20\n\x0bJEINBMLFCBP\x18\x06\x20\x01(\rR\x0bJEINBMLFCBPb\x06pro\
    to3\
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
            deps.push(super::ELCPGNINPIN::file_descriptor().clone());
            deps.push(super::OKKGELPAHEF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HBLDPBBKHPB::generated_message_descriptor_data());
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
