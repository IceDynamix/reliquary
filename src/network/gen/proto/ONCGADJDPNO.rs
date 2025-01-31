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

//! Generated file from `ONCGADJDPNO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ONCGADJDPNO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ONCGADJDPNO {
    // message fields
    // @@protoc_insertion_point(field:ONCGADJDPNO.JDHAIBLDFIH)
    pub JDHAIBLDFIH: u32,
    // @@protoc_insertion_point(field:ONCGADJDPNO.PPAEDKGDKDA)
    pub PPAEDKGDKDA: u32,
    // @@protoc_insertion_point(field:ONCGADJDPNO.OKKDBKAGOAO)
    pub OKKDBKAGOAO: ::std::vec::Vec<super::CKLKPNBJBGN::CKLKPNBJBGN>,
    // @@protoc_insertion_point(field:ONCGADJDPNO.EKAPBEEHOMB)
    pub EKAPBEEHOMB: u32,
    // @@protoc_insertion_point(field:ONCGADJDPNO.NFELFACJOGF)
    pub NFELFACJOGF: ::std::vec::Vec<super::KHKLCKLEDAL::KHKLCKLEDAL>,
    // special fields
    // @@protoc_insertion_point(special_field:ONCGADJDPNO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ONCGADJDPNO {
    fn default() -> &'a ONCGADJDPNO {
        <ONCGADJDPNO as ::protobuf::Message>::default_instance()
    }
}

impl ONCGADJDPNO {
    pub fn new() -> ONCGADJDPNO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JDHAIBLDFIH",
            |m: &ONCGADJDPNO| { &m.JDHAIBLDFIH },
            |m: &mut ONCGADJDPNO| { &mut m.JDHAIBLDFIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPAEDKGDKDA",
            |m: &ONCGADJDPNO| { &m.PPAEDKGDKDA },
            |m: &mut ONCGADJDPNO| { &mut m.PPAEDKGDKDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OKKDBKAGOAO",
            |m: &ONCGADJDPNO| { &m.OKKDBKAGOAO },
            |m: &mut ONCGADJDPNO| { &mut m.OKKDBKAGOAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKAPBEEHOMB",
            |m: &ONCGADJDPNO| { &m.EKAPBEEHOMB },
            |m: &mut ONCGADJDPNO| { &mut m.EKAPBEEHOMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NFELFACJOGF",
            |m: &ONCGADJDPNO| { &m.NFELFACJOGF },
            |m: &mut ONCGADJDPNO| { &mut m.NFELFACJOGF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ONCGADJDPNO>(
            "ONCGADJDPNO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ONCGADJDPNO {
    const NAME: &'static str = "ONCGADJDPNO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.JDHAIBLDFIH = is.read_uint32()?;
                },
                16 => {
                    self.PPAEDKGDKDA = is.read_uint32()?;
                },
                26 => {
                    self.OKKDBKAGOAO.push(is.read_message()?);
                },
                32 => {
                    self.EKAPBEEHOMB = is.read_uint32()?;
                },
                42 => {
                    self.NFELFACJOGF.push(is.read_message()?);
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
        if self.JDHAIBLDFIH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.JDHAIBLDFIH);
        }
        if self.PPAEDKGDKDA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PPAEDKGDKDA);
        }
        for value in &self.OKKDBKAGOAO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.EKAPBEEHOMB != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.EKAPBEEHOMB);
        }
        for value in &self.NFELFACJOGF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JDHAIBLDFIH != 0 {
            os.write_uint32(1, self.JDHAIBLDFIH)?;
        }
        if self.PPAEDKGDKDA != 0 {
            os.write_uint32(2, self.PPAEDKGDKDA)?;
        }
        for v in &self.OKKDBKAGOAO {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.EKAPBEEHOMB != 0 {
            os.write_uint32(4, self.EKAPBEEHOMB)?;
        }
        for v in &self.NFELFACJOGF {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> ONCGADJDPNO {
        ONCGADJDPNO::new()
    }

    fn clear(&mut self) {
        self.JDHAIBLDFIH = 0;
        self.PPAEDKGDKDA = 0;
        self.OKKDBKAGOAO.clear();
        self.EKAPBEEHOMB = 0;
        self.NFELFACJOGF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ONCGADJDPNO {
        static instance: ONCGADJDPNO = ONCGADJDPNO {
            JDHAIBLDFIH: 0,
            PPAEDKGDKDA: 0,
            OKKDBKAGOAO: ::std::vec::Vec::new(),
            EKAPBEEHOMB: 0,
            NFELFACJOGF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ONCGADJDPNO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ONCGADJDPNO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ONCGADJDPNO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ONCGADJDPNO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ONCGADJDPNO.proto\x1a\x11CKLKPNBJBGN.proto\x1a\x11KHKLCKLEDAL.prot\
    o\"\xd3\x01\n\x0bONCGADJDPNO\x12\x20\n\x0bJDHAIBLDFIH\x18\x01\x20\x01(\r\
    R\x0bJDHAIBLDFIH\x12\x20\n\x0bPPAEDKGDKDA\x18\x02\x20\x01(\rR\x0bPPAEDKG\
    DKDA\x12.\n\x0bOKKDBKAGOAO\x18\x03\x20\x03(\x0b2\x0c.CKLKPNBJBGNR\x0bOKK\
    DBKAGOAO\x12\x20\n\x0bEKAPBEEHOMB\x18\x04\x20\x01(\rR\x0bEKAPBEEHOMB\x12\
    .\n\x0bNFELFACJOGF\x18\x05\x20\x03(\x0b2\x0c.KHKLCKLEDALR\x0bNFELFACJOGF\
    b\x06proto3\
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
            deps.push(super::CKLKPNBJBGN::file_descriptor().clone());
            deps.push(super::KHKLCKLEDAL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ONCGADJDPNO::generated_message_descriptor_data());
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
