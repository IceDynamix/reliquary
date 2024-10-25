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

//! Generated file from `GOFLJKMHPMP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GOFLJKMHPMP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GOFLJKMHPMP {
    // message fields
    // @@protoc_insertion_point(field:GOFLJKMHPMP.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.LAJFCJALNMH)
    pub LAJFCJALNMH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.NLNKBGFOMKN)
    pub NLNKBGFOMKN: u32,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.EDNPJBHGKCK)
    pub EDNPJBHGKCK: f64,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.APFDBCJLPKP)
    pub APFDBCJLPKP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.LIIFHHAEPMJ)
    pub LIIFHHAEPMJ: i32,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.IPEDNEAFKNA)
    pub IPEDNEAFKNA: f64,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.BCKBOHJPJDI)
    pub BCKBOHJPJDI: u32,
    // @@protoc_insertion_point(field:GOFLJKMHPMP.BLEINMLIFDD)
    pub BLEINMLIFDD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GOFLJKMHPMP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GOFLJKMHPMP {
    fn default() -> &'a GOFLJKMHPMP {
        <GOFLJKMHPMP as ::protobuf::Message>::default_instance()
    }
}

impl GOFLJKMHPMP {
    pub fn new() -> GOFLJKMHPMP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &GOFLJKMHPMP| { &m.avatar_id },
            |m: &mut GOFLJKMHPMP| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LAJFCJALNMH",
            |m: &GOFLJKMHPMP| { &m.LAJFCJALNMH },
            |m: &mut GOFLJKMHPMP| { &mut m.LAJFCJALNMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLNKBGFOMKN",
            |m: &GOFLJKMHPMP| { &m.NLNKBGFOMKN },
            |m: &mut GOFLJKMHPMP| { &mut m.NLNKBGFOMKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EDNPJBHGKCK",
            |m: &GOFLJKMHPMP| { &m.EDNPJBHGKCK },
            |m: &mut GOFLJKMHPMP| { &mut m.EDNPJBHGKCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "APFDBCJLPKP",
            |m: &GOFLJKMHPMP| { &m.APFDBCJLPKP },
            |m: &mut GOFLJKMHPMP| { &mut m.APFDBCJLPKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIIFHHAEPMJ",
            |m: &GOFLJKMHPMP| { &m.LIIFHHAEPMJ },
            |m: &mut GOFLJKMHPMP| { &mut m.LIIFHHAEPMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPEDNEAFKNA",
            |m: &GOFLJKMHPMP| { &m.IPEDNEAFKNA },
            |m: &mut GOFLJKMHPMP| { &mut m.IPEDNEAFKNA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCKBOHJPJDI",
            |m: &GOFLJKMHPMP| { &m.BCKBOHJPJDI },
            |m: &mut GOFLJKMHPMP| { &mut m.BCKBOHJPJDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLEINMLIFDD",
            |m: &GOFLJKMHPMP| { &m.BLEINMLIFDD },
            |m: &mut GOFLJKMHPMP| { &mut m.BLEINMLIFDD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GOFLJKMHPMP>(
            "GOFLJKMHPMP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GOFLJKMHPMP {
    const NAME: &'static str = "GOFLJKMHPMP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.LAJFCJALNMH)?;
                },
                16 => {
                    self.LAJFCJALNMH.push(is.read_uint32()?);
                },
                24 => {
                    self.NLNKBGFOMKN = is.read_uint32()?;
                },
                33 => {
                    self.EDNPJBHGKCK = is.read_double()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.APFDBCJLPKP)?;
                },
                40 => {
                    self.APFDBCJLPKP.push(is.read_uint32()?);
                },
                48 => {
                    self.LIIFHHAEPMJ = is.read_int32()?;
                },
                57 => {
                    self.IPEDNEAFKNA = is.read_double()?;
                },
                64 => {
                    self.BCKBOHJPJDI = is.read_uint32()?;
                },
                72 => {
                    self.BLEINMLIFDD = is.read_uint32()?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        for value in &self.LAJFCJALNMH {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.NLNKBGFOMKN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NLNKBGFOMKN);
        }
        if self.EDNPJBHGKCK != 0. {
            my_size += 1 + 8;
        }
        for value in &self.APFDBCJLPKP {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.LIIFHHAEPMJ != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.LIIFHHAEPMJ);
        }
        if self.IPEDNEAFKNA != 0. {
            my_size += 1 + 8;
        }
        if self.BCKBOHJPJDI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.BCKBOHJPJDI);
        }
        if self.BLEINMLIFDD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BLEINMLIFDD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        for v in &self.LAJFCJALNMH {
            os.write_uint32(2, *v)?;
        };
        if self.NLNKBGFOMKN != 0 {
            os.write_uint32(3, self.NLNKBGFOMKN)?;
        }
        if self.EDNPJBHGKCK != 0. {
            os.write_double(4, self.EDNPJBHGKCK)?;
        }
        for v in &self.APFDBCJLPKP {
            os.write_uint32(5, *v)?;
        };
        if self.LIIFHHAEPMJ != 0 {
            os.write_int32(6, self.LIIFHHAEPMJ)?;
        }
        if self.IPEDNEAFKNA != 0. {
            os.write_double(7, self.IPEDNEAFKNA)?;
        }
        if self.BCKBOHJPJDI != 0 {
            os.write_uint32(8, self.BCKBOHJPJDI)?;
        }
        if self.BLEINMLIFDD != 0 {
            os.write_uint32(9, self.BLEINMLIFDD)?;
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

    fn new() -> GOFLJKMHPMP {
        GOFLJKMHPMP::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.LAJFCJALNMH.clear();
        self.NLNKBGFOMKN = 0;
        self.EDNPJBHGKCK = 0.;
        self.APFDBCJLPKP.clear();
        self.LIIFHHAEPMJ = 0;
        self.IPEDNEAFKNA = 0.;
        self.BCKBOHJPJDI = 0;
        self.BLEINMLIFDD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GOFLJKMHPMP {
        static instance: GOFLJKMHPMP = GOFLJKMHPMP {
            avatar_id: 0,
            LAJFCJALNMH: ::std::vec::Vec::new(),
            NLNKBGFOMKN: 0,
            EDNPJBHGKCK: 0.,
            APFDBCJLPKP: ::std::vec::Vec::new(),
            LIIFHHAEPMJ: 0,
            IPEDNEAFKNA: 0.,
            BCKBOHJPJDI: 0,
            BLEINMLIFDD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GOFLJKMHPMP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GOFLJKMHPMP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GOFLJKMHPMP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GOFLJKMHPMP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GOFLJKMHPMP.proto\"\xba\x02\n\x0bGOFLJKMHPMP\x12\x1b\n\tavatar_id\
    \x18\x01\x20\x01(\rR\x08avatarId\x12\x20\n\x0bLAJFCJALNMH\x18\x02\x20\
    \x03(\rR\x0bLAJFCJALNMH\x12\x20\n\x0bNLNKBGFOMKN\x18\x03\x20\x01(\rR\x0b\
    NLNKBGFOMKN\x12\x20\n\x0bEDNPJBHGKCK\x18\x04\x20\x01(\x01R\x0bEDNPJBHGKC\
    K\x12\x20\n\x0bAPFDBCJLPKP\x18\x05\x20\x03(\rR\x0bAPFDBCJLPKP\x12\x20\n\
    \x0bLIIFHHAEPMJ\x18\x06\x20\x01(\x05R\x0bLIIFHHAEPMJ\x12\x20\n\x0bIPEDNE\
    AFKNA\x18\x07\x20\x01(\x01R\x0bIPEDNEAFKNA\x12\x20\n\x0bBCKBOHJPJDI\x18\
    \x08\x20\x01(\rR\x0bBCKBOHJPJDI\x12\x20\n\x0bBLEINMLIFDD\x18\t\x20\x01(\
    \rR\x0bBLEINMLIFDDb\x06proto3\
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
            messages.push(GOFLJKMHPMP::generated_message_descriptor_data());
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