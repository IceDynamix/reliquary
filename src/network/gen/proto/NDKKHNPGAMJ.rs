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

//! Generated file from `NDKKHNPGAMJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NDKKHNPGAMJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NDKKHNPGAMJ {
    // message fields
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.BNGDKIBJHMD)
    pub BNGDKIBJHMD: ::std::string::String,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.CENCAKDHHHA)
    pub CENCAKDHHHA: ::std::string::String,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.JOCJAFILDGH)
    pub JOCJAFILDGH: bool,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.IIBMDAJNHLA)
    pub IIBMDAJNHLA: i64,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.JCLHPKIEDIA)
    pub JCLHPKIEDIA: u32,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.MMKMCHDADFD)
    pub MMKMCHDADFD: i64,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.FJPEFPBFALC)
    pub FJPEFPBFALC: u32,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.ODBEELGCENI)
    pub ODBEELGCENI: u32,
    // @@protoc_insertion_point(field:NDKKHNPGAMJ.NFOIIBJPFFG)
    pub NFOIIBJPFFG: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:NDKKHNPGAMJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NDKKHNPGAMJ {
    fn default() -> &'a NDKKHNPGAMJ {
        <NDKKHNPGAMJ as ::protobuf::Message>::default_instance()
    }
}

impl NDKKHNPGAMJ {
    pub fn new() -> NDKKHNPGAMJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNGDKIBJHMD",
            |m: &NDKKHNPGAMJ| { &m.BNGDKIBJHMD },
            |m: &mut NDKKHNPGAMJ| { &mut m.BNGDKIBJHMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CENCAKDHHHA",
            |m: &NDKKHNPGAMJ| { &m.CENCAKDHHHA },
            |m: &mut NDKKHNPGAMJ| { &mut m.CENCAKDHHHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JOCJAFILDGH",
            |m: &NDKKHNPGAMJ| { &m.JOCJAFILDGH },
            |m: &mut NDKKHNPGAMJ| { &mut m.JOCJAFILDGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIBMDAJNHLA",
            |m: &NDKKHNPGAMJ| { &m.IIBMDAJNHLA },
            |m: &mut NDKKHNPGAMJ| { &mut m.IIBMDAJNHLA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCLHPKIEDIA",
            |m: &NDKKHNPGAMJ| { &m.JCLHPKIEDIA },
            |m: &mut NDKKHNPGAMJ| { &mut m.JCLHPKIEDIA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMKMCHDADFD",
            |m: &NDKKHNPGAMJ| { &m.MMKMCHDADFD },
            |m: &mut NDKKHNPGAMJ| { &mut m.MMKMCHDADFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJPEFPBFALC",
            |m: &NDKKHNPGAMJ| { &m.FJPEFPBFALC },
            |m: &mut NDKKHNPGAMJ| { &mut m.FJPEFPBFALC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODBEELGCENI",
            |m: &NDKKHNPGAMJ| { &m.ODBEELGCENI },
            |m: &mut NDKKHNPGAMJ| { &mut m.ODBEELGCENI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFOIIBJPFFG",
            |m: &NDKKHNPGAMJ| { &m.NFOIIBJPFFG },
            |m: &mut NDKKHNPGAMJ| { &mut m.NFOIIBJPFFG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NDKKHNPGAMJ>(
            "NDKKHNPGAMJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NDKKHNPGAMJ {
    const NAME: &'static str = "NDKKHNPGAMJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.BNGDKIBJHMD = is.read_string()?;
                },
                42 => {
                    self.CENCAKDHHHA = is.read_string()?;
                },
                32 => {
                    self.JOCJAFILDGH = is.read_bool()?;
                },
                88 => {
                    self.IIBMDAJNHLA = is.read_int64()?;
                },
                112 => {
                    self.JCLHPKIEDIA = is.read_uint32()?;
                },
                56 => {
                    self.MMKMCHDADFD = is.read_int64()?;
                },
                72 => {
                    self.FJPEFPBFALC = is.read_uint32()?;
                },
                8 => {
                    self.ODBEELGCENI = is.read_uint32()?;
                },
                66 => {
                    self.NFOIIBJPFFG = is.read_string()?;
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
        if !self.BNGDKIBJHMD.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.BNGDKIBJHMD);
        }
        if !self.CENCAKDHHHA.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.CENCAKDHHHA);
        }
        if self.JOCJAFILDGH != false {
            my_size += 1 + 1;
        }
        if self.IIBMDAJNHLA != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.IIBMDAJNHLA);
        }
        if self.JCLHPKIEDIA != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JCLHPKIEDIA);
        }
        if self.MMKMCHDADFD != 0 {
            my_size += ::protobuf::rt::int64_size(7, self.MMKMCHDADFD);
        }
        if self.FJPEFPBFALC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FJPEFPBFALC);
        }
        if self.ODBEELGCENI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ODBEELGCENI);
        }
        if !self.NFOIIBJPFFG.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.NFOIIBJPFFG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.BNGDKIBJHMD.is_empty() {
            os.write_string(15, &self.BNGDKIBJHMD)?;
        }
        if !self.CENCAKDHHHA.is_empty() {
            os.write_string(5, &self.CENCAKDHHHA)?;
        }
        if self.JOCJAFILDGH != false {
            os.write_bool(4, self.JOCJAFILDGH)?;
        }
        if self.IIBMDAJNHLA != 0 {
            os.write_int64(11, self.IIBMDAJNHLA)?;
        }
        if self.JCLHPKIEDIA != 0 {
            os.write_uint32(14, self.JCLHPKIEDIA)?;
        }
        if self.MMKMCHDADFD != 0 {
            os.write_int64(7, self.MMKMCHDADFD)?;
        }
        if self.FJPEFPBFALC != 0 {
            os.write_uint32(9, self.FJPEFPBFALC)?;
        }
        if self.ODBEELGCENI != 0 {
            os.write_uint32(1, self.ODBEELGCENI)?;
        }
        if !self.NFOIIBJPFFG.is_empty() {
            os.write_string(8, &self.NFOIIBJPFFG)?;
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

    fn new() -> NDKKHNPGAMJ {
        NDKKHNPGAMJ::new()
    }

    fn clear(&mut self) {
        self.BNGDKIBJHMD.clear();
        self.CENCAKDHHHA.clear();
        self.JOCJAFILDGH = false;
        self.IIBMDAJNHLA = 0;
        self.JCLHPKIEDIA = 0;
        self.MMKMCHDADFD = 0;
        self.FJPEFPBFALC = 0;
        self.ODBEELGCENI = 0;
        self.NFOIIBJPFFG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NDKKHNPGAMJ {
        static instance: NDKKHNPGAMJ = NDKKHNPGAMJ {
            BNGDKIBJHMD: ::std::string::String::new(),
            CENCAKDHHHA: ::std::string::String::new(),
            JOCJAFILDGH: false,
            IIBMDAJNHLA: 0,
            JCLHPKIEDIA: 0,
            MMKMCHDADFD: 0,
            FJPEFPBFALC: 0,
            ODBEELGCENI: 0,
            NFOIIBJPFFG: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NDKKHNPGAMJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NDKKHNPGAMJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NDKKHNPGAMJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NDKKHNPGAMJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NDKKHNPGAMJ.proto\"\xbf\x02\n\x0bNDKKHNPGAMJ\x12\x20\n\x0bBNGDKIBJ\
    HMD\x18\x0f\x20\x01(\tR\x0bBNGDKIBJHMD\x12\x20\n\x0bCENCAKDHHHA\x18\x05\
    \x20\x01(\tR\x0bCENCAKDHHHA\x12\x20\n\x0bJOCJAFILDGH\x18\x04\x20\x01(\
    \x08R\x0bJOCJAFILDGH\x12\x20\n\x0bIIBMDAJNHLA\x18\x0b\x20\x01(\x03R\x0bI\
    IBMDAJNHLA\x12\x20\n\x0bJCLHPKIEDIA\x18\x0e\x20\x01(\rR\x0bJCLHPKIEDIA\
    \x12\x20\n\x0bMMKMCHDADFD\x18\x07\x20\x01(\x03R\x0bMMKMCHDADFD\x12\x20\n\
    \x0bFJPEFPBFALC\x18\t\x20\x01(\rR\x0bFJPEFPBFALC\x12\x20\n\x0bODBEELGCEN\
    I\x18\x01\x20\x01(\rR\x0bODBEELGCENI\x12\x20\n\x0bNFOIIBJPFFG\x18\x08\
    \x20\x01(\tR\x0bNFOIIBJPFFGb\x06proto3\
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
            messages.push(NDKKHNPGAMJ::generated_message_descriptor_data());
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
