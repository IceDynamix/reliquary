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

//! Generated file from `JHPKNHHNAPP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:JHPKNHHNAPP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JHPKNHHNAPP {
    // message fields
    // @@protoc_insertion_point(field:JHPKNHHNAPP.JBLECMAPFDC)
    pub JBLECMAPFDC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JHPKNHHNAPP.ELINMPKBEFL)
    pub ELINMPKBEFL: u32,
    // @@protoc_insertion_point(field:JHPKNHHNAPP.LBGDLHKEEKC)
    pub LBGDLHKEEKC: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:JHPKNHHNAPP.KBJFONAGBHK)
    pub KBJFONAGBHK: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:JHPKNHHNAPP.AHMDOBICECA)
    pub AHMDOBICECA: ::std::vec::Vec<super::FightGeneralScNotify::FightGeneralScNotify>,
    // special fields
    // @@protoc_insertion_point(special_field:JHPKNHHNAPP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JHPKNHHNAPP {
    fn default() -> &'a JHPKNHHNAPP {
        <JHPKNHHNAPP as ::protobuf::Message>::default_instance()
    }
}

impl JHPKNHHNAPP {
    pub fn new() -> JHPKNHHNAPP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JBLECMAPFDC",
            |m: &JHPKNHHNAPP| { &m.JBLECMAPFDC },
            |m: &mut JHPKNHHNAPP| { &mut m.JBLECMAPFDC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELINMPKBEFL",
            |m: &JHPKNHHNAPP| { &m.ELINMPKBEFL },
            |m: &mut JHPKNHHNAPP| { &mut m.ELINMPKBEFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBGDLHKEEKC",
            |m: &JHPKNHHNAPP| { &m.LBGDLHKEEKC },
            |m: &mut JHPKNHHNAPP| { &mut m.LBGDLHKEEKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBJFONAGBHK",
            |m: &JHPKNHHNAPP| { &m.KBJFONAGBHK },
            |m: &mut JHPKNHHNAPP| { &mut m.KBJFONAGBHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AHMDOBICECA",
            |m: &JHPKNHHNAPP| { &m.AHMDOBICECA },
            |m: &mut JHPKNHHNAPP| { &mut m.AHMDOBICECA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JHPKNHHNAPP>(
            "JHPKNHHNAPP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JHPKNHHNAPP {
    const NAME: &'static str = "JHPKNHHNAPP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.JBLECMAPFDC)?;
                },
                8 => {
                    self.JBLECMAPFDC.push(is.read_uint32()?);
                },
                64 => {
                    self.ELINMPKBEFL = is.read_uint32()?;
                },
                122 => {
                    self.LBGDLHKEEKC = is.read_bytes()?;
                },
                114 => {
                    self.KBJFONAGBHK = is.read_bytes()?;
                },
                34 => {
                    self.AHMDOBICECA.push(is.read_message()?);
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.JBLECMAPFDC);
        if self.ELINMPKBEFL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ELINMPKBEFL);
        }
        if !self.LBGDLHKEEKC.is_empty() {
            my_size += ::protobuf::rt::bytes_size(15, &self.LBGDLHKEEKC);
        }
        if !self.KBJFONAGBHK.is_empty() {
            my_size += ::protobuf::rt::bytes_size(14, &self.KBJFONAGBHK);
        }
        for value in &self.AHMDOBICECA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(1, &self.JBLECMAPFDC)?;
        if self.ELINMPKBEFL != 0 {
            os.write_uint32(8, self.ELINMPKBEFL)?;
        }
        if !self.LBGDLHKEEKC.is_empty() {
            os.write_bytes(15, &self.LBGDLHKEEKC)?;
        }
        if !self.KBJFONAGBHK.is_empty() {
            os.write_bytes(14, &self.KBJFONAGBHK)?;
        }
        for v in &self.AHMDOBICECA {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> JHPKNHHNAPP {
        JHPKNHHNAPP::new()
    }

    fn clear(&mut self) {
        self.JBLECMAPFDC.clear();
        self.ELINMPKBEFL = 0;
        self.LBGDLHKEEKC.clear();
        self.KBJFONAGBHK.clear();
        self.AHMDOBICECA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JHPKNHHNAPP {
        static instance: JHPKNHHNAPP = JHPKNHHNAPP {
            JBLECMAPFDC: ::std::vec::Vec::new(),
            ELINMPKBEFL: 0,
            LBGDLHKEEKC: ::std::vec::Vec::new(),
            KBJFONAGBHK: ::std::vec::Vec::new(),
            AHMDOBICECA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JHPKNHHNAPP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JHPKNHHNAPP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JHPKNHHNAPP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JHPKNHHNAPP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JHPKNHHNAPP.proto\x1a\x1aFightGeneralScNotify.proto\"\xce\x01\n\
    \x0bJHPKNHHNAPP\x12\x20\n\x0bJBLECMAPFDC\x18\x01\x20\x03(\rR\x0bJBLECMAP\
    FDC\x12\x20\n\x0bELINMPKBEFL\x18\x08\x20\x01(\rR\x0bELINMPKBEFL\x12\x20\
    \n\x0bLBGDLHKEEKC\x18\x0f\x20\x01(\x0cR\x0bLBGDLHKEEKC\x12\x20\n\x0bKBJF\
    ONAGBHK\x18\x0e\x20\x01(\x0cR\x0bKBJFONAGBHK\x127\n\x0bAHMDOBICECA\x18\
    \x04\x20\x03(\x0b2\x15.FightGeneralScNotifyR\x0bAHMDOBICECAb\x06proto3\
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
            deps.push(super::FightGeneralScNotify::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JHPKNHHNAPP::generated_message_descriptor_data());
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
