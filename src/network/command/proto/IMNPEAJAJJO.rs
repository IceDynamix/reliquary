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

//! Generated file from `IMNPEAJAJJO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IMNPEAJAJJO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IMNPEAJAJJO {
    // message fields
    // @@protoc_insertion_point(field:IMNPEAJAJJO.IKMNAMKJAFA)
    pub IKMNAMKJAFA: u32,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.OBLHBOEOLAF)
    pub OBLHBOEOLAF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.FGOMIPLMEIC)
    pub FGOMIPLMEIC: u32,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.EMLLECGEPCK)
    pub EMLLECGEPCK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.FFMDBDEHHEG)
    pub FFMDBDEHHEG: u32,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.ILMOOKBJHHC)
    pub ILMOOKBJHHC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.FJKGKAEKBKJ)
    pub FJKGKAEKBKJ: bool,
    // @@protoc_insertion_point(field:IMNPEAJAJJO.FFHEEIDBHEA)
    pub FFHEEIDBHEA: bool,
    // special fields
    // @@protoc_insertion_point(special_field:IMNPEAJAJJO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IMNPEAJAJJO {
    fn default() -> &'a IMNPEAJAJJO {
        <IMNPEAJAJJO as ::protobuf::Message>::default_instance()
    }
}

impl IMNPEAJAJJO {
    pub fn new() -> IMNPEAJAJJO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKMNAMKJAFA",
            |m: &IMNPEAJAJJO| { &m.IKMNAMKJAFA },
            |m: &mut IMNPEAJAJJO| { &mut m.IKMNAMKJAFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OBLHBOEOLAF",
            |m: &IMNPEAJAJJO| { &m.OBLHBOEOLAF },
            |m: &mut IMNPEAJAJJO| { &mut m.OBLHBOEOLAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGOMIPLMEIC",
            |m: &IMNPEAJAJJO| { &m.FGOMIPLMEIC },
            |m: &mut IMNPEAJAJJO| { &mut m.FGOMIPLMEIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMLLECGEPCK",
            |m: &IMNPEAJAJJO| { &m.EMLLECGEPCK },
            |m: &mut IMNPEAJAJJO| { &mut m.EMLLECGEPCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFMDBDEHHEG",
            |m: &IMNPEAJAJJO| { &m.FFMDBDEHHEG },
            |m: &mut IMNPEAJAJJO| { &mut m.FFMDBDEHHEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ILMOOKBJHHC",
            |m: &IMNPEAJAJJO| { &m.ILMOOKBJHHC },
            |m: &mut IMNPEAJAJJO| { &mut m.ILMOOKBJHHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJKGKAEKBKJ",
            |m: &IMNPEAJAJJO| { &m.FJKGKAEKBKJ },
            |m: &mut IMNPEAJAJJO| { &mut m.FJKGKAEKBKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFHEEIDBHEA",
            |m: &IMNPEAJAJJO| { &m.FFHEEIDBHEA },
            |m: &mut IMNPEAJAJJO| { &mut m.FFHEEIDBHEA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IMNPEAJAJJO>(
            "IMNPEAJAJJO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IMNPEAJAJJO {
    const NAME: &'static str = "IMNPEAJAJJO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.IKMNAMKJAFA = is.read_uint32()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.OBLHBOEOLAF)?;
                },
                48 => {
                    self.OBLHBOEOLAF.push(is.read_uint32()?);
                },
                8 => {
                    self.FGOMIPLMEIC = is.read_uint32()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.EMLLECGEPCK)?;
                },
                40 => {
                    self.EMLLECGEPCK.push(is.read_uint32()?);
                },
                56 => {
                    self.FFMDBDEHHEG = is.read_uint32()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.ILMOOKBJHHC)?;
                },
                96 => {
                    self.ILMOOKBJHHC.push(is.read_uint32()?);
                },
                16 => {
                    self.FJKGKAEKBKJ = is.read_bool()?;
                },
                64 => {
                    self.FFHEEIDBHEA = is.read_bool()?;
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
        if self.IKMNAMKJAFA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.IKMNAMKJAFA);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(6, &self.OBLHBOEOLAF);
        if self.FGOMIPLMEIC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FGOMIPLMEIC);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(5, &self.EMLLECGEPCK);
        if self.FFMDBDEHHEG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FFMDBDEHHEG);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.ILMOOKBJHHC);
        if self.FJKGKAEKBKJ != false {
            my_size += 1 + 1;
        }
        if self.FFHEEIDBHEA != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IKMNAMKJAFA != 0 {
            os.write_uint32(15, self.IKMNAMKJAFA)?;
        }
        os.write_repeated_packed_uint32(6, &self.OBLHBOEOLAF)?;
        if self.FGOMIPLMEIC != 0 {
            os.write_uint32(1, self.FGOMIPLMEIC)?;
        }
        os.write_repeated_packed_uint32(5, &self.EMLLECGEPCK)?;
        if self.FFMDBDEHHEG != 0 {
            os.write_uint32(7, self.FFMDBDEHHEG)?;
        }
        os.write_repeated_packed_uint32(12, &self.ILMOOKBJHHC)?;
        if self.FJKGKAEKBKJ != false {
            os.write_bool(2, self.FJKGKAEKBKJ)?;
        }
        if self.FFHEEIDBHEA != false {
            os.write_bool(8, self.FFHEEIDBHEA)?;
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

    fn new() -> IMNPEAJAJJO {
        IMNPEAJAJJO::new()
    }

    fn clear(&mut self) {
        self.IKMNAMKJAFA = 0;
        self.OBLHBOEOLAF.clear();
        self.FGOMIPLMEIC = 0;
        self.EMLLECGEPCK.clear();
        self.FFMDBDEHHEG = 0;
        self.ILMOOKBJHHC.clear();
        self.FJKGKAEKBKJ = false;
        self.FFHEEIDBHEA = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IMNPEAJAJJO {
        static instance: IMNPEAJAJJO = IMNPEAJAJJO {
            IKMNAMKJAFA: 0,
            OBLHBOEOLAF: ::std::vec::Vec::new(),
            FGOMIPLMEIC: 0,
            EMLLECGEPCK: ::std::vec::Vec::new(),
            FFMDBDEHHEG: 0,
            ILMOOKBJHHC: ::std::vec::Vec::new(),
            FJKGKAEKBKJ: false,
            FFHEEIDBHEA: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IMNPEAJAJJO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IMNPEAJAJJO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IMNPEAJAJJO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IMNPEAJAJJO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IMNPEAJAJJO.proto\"\x9d\x02\n\x0bIMNPEAJAJJO\x12\x20\n\x0bIKMNAMKJ\
    AFA\x18\x0f\x20\x01(\rR\x0bIKMNAMKJAFA\x12\x20\n\x0bOBLHBOEOLAF\x18\x06\
    \x20\x03(\rR\x0bOBLHBOEOLAF\x12\x20\n\x0bFGOMIPLMEIC\x18\x01\x20\x01(\rR\
    \x0bFGOMIPLMEIC\x12\x20\n\x0bEMLLECGEPCK\x18\x05\x20\x03(\rR\x0bEMLLECGE\
    PCK\x12\x20\n\x0bFFMDBDEHHEG\x18\x07\x20\x01(\rR\x0bFFMDBDEHHEG\x12\x20\
    \n\x0bILMOOKBJHHC\x18\x0c\x20\x03(\rR\x0bILMOOKBJHHC\x12\x20\n\x0bFJKGKA\
    EKBKJ\x18\x02\x20\x01(\x08R\x0bFJKGKAEKBKJ\x12\x20\n\x0bFFHEEIDBHEA\x18\
    \x08\x20\x01(\x08R\x0bFFHEEIDBHEAb\x06proto3\
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
            messages.push(IMNPEAJAJJO::generated_message_descriptor_data());
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
