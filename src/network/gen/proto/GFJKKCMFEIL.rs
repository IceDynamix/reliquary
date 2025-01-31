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

//! Generated file from `GFJKKCMFEIL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GFJKKCMFEIL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GFJKKCMFEIL {
    // message fields
    // @@protoc_insertion_point(field:GFJKKCMFEIL.FKDOKCKIJAH)
    pub FKDOKCKIJAH: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.PNPNLLCCDKH)
    pub PNPNLLCCDKH: bool,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.KDACFDJGFCM)
    pub KDACFDJGFCM: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.HFGPLBJFBPK)
    pub HFGPLBJFBPK: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.LBGCGFAKEGO)
    pub LBGCGFAKEGO: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.GAAHDPKDGII)
    pub GAAHDPKDGII: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.JNCEFJMGKGC)
    pub JNCEFJMGKGC: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.AAJPPIEGEHG)
    pub AAJPPIEGEHG: u32,
    // @@protoc_insertion_point(field:GFJKKCMFEIL.BFDOLDACBOL)
    pub BFDOLDACBOL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GFJKKCMFEIL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GFJKKCMFEIL {
    fn default() -> &'a GFJKKCMFEIL {
        <GFJKKCMFEIL as ::protobuf::Message>::default_instance()
    }
}

impl GFJKKCMFEIL {
    pub fn new() -> GFJKKCMFEIL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKDOKCKIJAH",
            |m: &GFJKKCMFEIL| { &m.FKDOKCKIJAH },
            |m: &mut GFJKKCMFEIL| { &mut m.FKDOKCKIJAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNPNLLCCDKH",
            |m: &GFJKKCMFEIL| { &m.PNPNLLCCDKH },
            |m: &mut GFJKKCMFEIL| { &mut m.PNPNLLCCDKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDACFDJGFCM",
            |m: &GFJKKCMFEIL| { &m.KDACFDJGFCM },
            |m: &mut GFJKKCMFEIL| { &mut m.KDACFDJGFCM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFGPLBJFBPK",
            |m: &GFJKKCMFEIL| { &m.HFGPLBJFBPK },
            |m: &mut GFJKKCMFEIL| { &mut m.HFGPLBJFBPK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBGCGFAKEGO",
            |m: &GFJKKCMFEIL| { &m.LBGCGFAKEGO },
            |m: &mut GFJKKCMFEIL| { &mut m.LBGCGFAKEGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GAAHDPKDGII",
            |m: &GFJKKCMFEIL| { &m.GAAHDPKDGII },
            |m: &mut GFJKKCMFEIL| { &mut m.GAAHDPKDGII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNCEFJMGKGC",
            |m: &GFJKKCMFEIL| { &m.JNCEFJMGKGC },
            |m: &mut GFJKKCMFEIL| { &mut m.JNCEFJMGKGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAJPPIEGEHG",
            |m: &GFJKKCMFEIL| { &m.AAJPPIEGEHG },
            |m: &mut GFJKKCMFEIL| { &mut m.AAJPPIEGEHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFDOLDACBOL",
            |m: &GFJKKCMFEIL| { &m.BFDOLDACBOL },
            |m: &mut GFJKKCMFEIL| { &mut m.BFDOLDACBOL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GFJKKCMFEIL>(
            "GFJKKCMFEIL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GFJKKCMFEIL {
    const NAME: &'static str = "GFJKKCMFEIL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.FKDOKCKIJAH = is.read_uint32()?;
                },
                56 => {
                    self.PNPNLLCCDKH = is.read_bool()?;
                },
                72 => {
                    self.KDACFDJGFCM = is.read_uint32()?;
                },
                88 => {
                    self.HFGPLBJFBPK = is.read_uint32()?;
                },
                24 => {
                    self.LBGCGFAKEGO = is.read_uint32()?;
                },
                64 => {
                    self.GAAHDPKDGII = is.read_uint32()?;
                },
                112 => {
                    self.JNCEFJMGKGC = is.read_uint32()?;
                },
                120 => {
                    self.AAJPPIEGEHG = is.read_uint32()?;
                },
                80 => {
                    self.BFDOLDACBOL = is.read_uint32()?;
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
        if self.FKDOKCKIJAH != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FKDOKCKIJAH);
        }
        if self.PNPNLLCCDKH != false {
            my_size += 1 + 1;
        }
        if self.KDACFDJGFCM != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.KDACFDJGFCM);
        }
        if self.HFGPLBJFBPK != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HFGPLBJFBPK);
        }
        if self.LBGCGFAKEGO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LBGCGFAKEGO);
        }
        if self.GAAHDPKDGII != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GAAHDPKDGII);
        }
        if self.JNCEFJMGKGC != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.JNCEFJMGKGC);
        }
        if self.AAJPPIEGEHG != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.AAJPPIEGEHG);
        }
        if self.BFDOLDACBOL != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.BFDOLDACBOL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FKDOKCKIJAH != 0 {
            os.write_uint32(2, self.FKDOKCKIJAH)?;
        }
        if self.PNPNLLCCDKH != false {
            os.write_bool(7, self.PNPNLLCCDKH)?;
        }
        if self.KDACFDJGFCM != 0 {
            os.write_uint32(9, self.KDACFDJGFCM)?;
        }
        if self.HFGPLBJFBPK != 0 {
            os.write_uint32(11, self.HFGPLBJFBPK)?;
        }
        if self.LBGCGFAKEGO != 0 {
            os.write_uint32(3, self.LBGCGFAKEGO)?;
        }
        if self.GAAHDPKDGII != 0 {
            os.write_uint32(8, self.GAAHDPKDGII)?;
        }
        if self.JNCEFJMGKGC != 0 {
            os.write_uint32(14, self.JNCEFJMGKGC)?;
        }
        if self.AAJPPIEGEHG != 0 {
            os.write_uint32(15, self.AAJPPIEGEHG)?;
        }
        if self.BFDOLDACBOL != 0 {
            os.write_uint32(10, self.BFDOLDACBOL)?;
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

    fn new() -> GFJKKCMFEIL {
        GFJKKCMFEIL::new()
    }

    fn clear(&mut self) {
        self.FKDOKCKIJAH = 0;
        self.PNPNLLCCDKH = false;
        self.KDACFDJGFCM = 0;
        self.HFGPLBJFBPK = 0;
        self.LBGCGFAKEGO = 0;
        self.GAAHDPKDGII = 0;
        self.JNCEFJMGKGC = 0;
        self.AAJPPIEGEHG = 0;
        self.BFDOLDACBOL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GFJKKCMFEIL {
        static instance: GFJKKCMFEIL = GFJKKCMFEIL {
            FKDOKCKIJAH: 0,
            PNPNLLCCDKH: false,
            KDACFDJGFCM: 0,
            HFGPLBJFBPK: 0,
            LBGCGFAKEGO: 0,
            GAAHDPKDGII: 0,
            JNCEFJMGKGC: 0,
            AAJPPIEGEHG: 0,
            BFDOLDACBOL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GFJKKCMFEIL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GFJKKCMFEIL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GFJKKCMFEIL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GFJKKCMFEIL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GFJKKCMFEIL.proto\"\xbf\x02\n\x0bGFJKKCMFEIL\x12\x20\n\x0bFKDOKCKI\
    JAH\x18\x02\x20\x01(\rR\x0bFKDOKCKIJAH\x12\x20\n\x0bPNPNLLCCDKH\x18\x07\
    \x20\x01(\x08R\x0bPNPNLLCCDKH\x12\x20\n\x0bKDACFDJGFCM\x18\t\x20\x01(\rR\
    \x0bKDACFDJGFCM\x12\x20\n\x0bHFGPLBJFBPK\x18\x0b\x20\x01(\rR\x0bHFGPLBJF\
    BPK\x12\x20\n\x0bLBGCGFAKEGO\x18\x03\x20\x01(\rR\x0bLBGCGFAKEGO\x12\x20\
    \n\x0bGAAHDPKDGII\x18\x08\x20\x01(\rR\x0bGAAHDPKDGII\x12\x20\n\x0bJNCEFJ\
    MGKGC\x18\x0e\x20\x01(\rR\x0bJNCEFJMGKGC\x12\x20\n\x0bAAJPPIEGEHG\x18\
    \x0f\x20\x01(\rR\x0bAAJPPIEGEHG\x12\x20\n\x0bBFDOLDACBOL\x18\n\x20\x01(\
    \rR\x0bBFDOLDACBOLb\x06proto3\
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
            messages.push(GFJKKCMFEIL::generated_message_descriptor_data());
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
