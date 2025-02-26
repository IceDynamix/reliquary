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

//! Generated file from `CHDONIGOKNM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CHDONIGOKNM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHDONIGOKNM {
    // message fields
    // @@protoc_insertion_point(field:CHDONIGOKNM.HEOCPAKCELM)
    pub HEOCPAKCELM: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.BGHKBMFHMOJ)
    pub BGHKBMFHMOJ: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.LOOLLAGMNLH)
    pub LOOLLAGMNLH: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.HECJOOOBAHC)
    pub HECJOOOBAHC: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.FKEAAIPKPAA)
    pub FKEAAIPKPAA: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.ICLEENHIPOH)
    pub ICLEENHIPOH: u32,
    // @@protoc_insertion_point(field:CHDONIGOKNM.AGLEFMECOMC)
    pub AGLEFMECOMC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CHDONIGOKNM.ODBONKCMDMP)
    pub ODBONKCMDMP: ::std::vec::Vec<super::PLPNLIBMNIO::PLPNLIBMNIO>,
    // special fields
    // @@protoc_insertion_point(special_field:CHDONIGOKNM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHDONIGOKNM {
    fn default() -> &'a CHDONIGOKNM {
        <CHDONIGOKNM as ::protobuf::Message>::default_instance()
    }
}

impl CHDONIGOKNM {
    pub fn new() -> CHDONIGOKNM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEOCPAKCELM",
            |m: &CHDONIGOKNM| { &m.HEOCPAKCELM },
            |m: &mut CHDONIGOKNM| { &mut m.HEOCPAKCELM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGHKBMFHMOJ",
            |m: &CHDONIGOKNM| { &m.BGHKBMFHMOJ },
            |m: &mut CHDONIGOKNM| { &mut m.BGHKBMFHMOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOOLLAGMNLH",
            |m: &CHDONIGOKNM| { &m.LOOLLAGMNLH },
            |m: &mut CHDONIGOKNM| { &mut m.LOOLLAGMNLH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HECJOOOBAHC",
            |m: &CHDONIGOKNM| { &m.HECJOOOBAHC },
            |m: &mut CHDONIGOKNM| { &mut m.HECJOOOBAHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKEAAIPKPAA",
            |m: &CHDONIGOKNM| { &m.FKEAAIPKPAA },
            |m: &mut CHDONIGOKNM| { &mut m.FKEAAIPKPAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICLEENHIPOH",
            |m: &CHDONIGOKNM| { &m.ICLEENHIPOH },
            |m: &mut CHDONIGOKNM| { &mut m.ICLEENHIPOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AGLEFMECOMC",
            |m: &CHDONIGOKNM| { &m.AGLEFMECOMC },
            |m: &mut CHDONIGOKNM| { &mut m.AGLEFMECOMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ODBONKCMDMP",
            |m: &CHDONIGOKNM| { &m.ODBONKCMDMP },
            |m: &mut CHDONIGOKNM| { &mut m.ODBONKCMDMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHDONIGOKNM>(
            "CHDONIGOKNM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHDONIGOKNM {
    const NAME: &'static str = "CHDONIGOKNM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.HEOCPAKCELM = is.read_uint32()?;
                },
                16 => {
                    self.BGHKBMFHMOJ = is.read_uint32()?;
                },
                24 => {
                    self.LOOLLAGMNLH = is.read_uint32()?;
                },
                32 => {
                    self.HECJOOOBAHC = is.read_uint32()?;
                },
                40 => {
                    self.FKEAAIPKPAA = is.read_uint32()?;
                },
                48 => {
                    self.ICLEENHIPOH = is.read_uint32()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.AGLEFMECOMC)?;
                },
                56 => {
                    self.AGLEFMECOMC.push(is.read_uint32()?);
                },
                66 => {
                    self.ODBONKCMDMP.push(is.read_message()?);
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
        if self.HEOCPAKCELM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.HEOCPAKCELM);
        }
        if self.BGHKBMFHMOJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.BGHKBMFHMOJ);
        }
        if self.LOOLLAGMNLH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LOOLLAGMNLH);
        }
        if self.HECJOOOBAHC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.HECJOOOBAHC);
        }
        if self.FKEAAIPKPAA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.FKEAAIPKPAA);
        }
        if self.ICLEENHIPOH != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ICLEENHIPOH);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(7, &self.AGLEFMECOMC);
        for value in &self.ODBONKCMDMP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HEOCPAKCELM != 0 {
            os.write_uint32(1, self.HEOCPAKCELM)?;
        }
        if self.BGHKBMFHMOJ != 0 {
            os.write_uint32(2, self.BGHKBMFHMOJ)?;
        }
        if self.LOOLLAGMNLH != 0 {
            os.write_uint32(3, self.LOOLLAGMNLH)?;
        }
        if self.HECJOOOBAHC != 0 {
            os.write_uint32(4, self.HECJOOOBAHC)?;
        }
        if self.FKEAAIPKPAA != 0 {
            os.write_uint32(5, self.FKEAAIPKPAA)?;
        }
        if self.ICLEENHIPOH != 0 {
            os.write_uint32(6, self.ICLEENHIPOH)?;
        }
        os.write_repeated_packed_uint32(7, &self.AGLEFMECOMC)?;
        for v in &self.ODBONKCMDMP {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> CHDONIGOKNM {
        CHDONIGOKNM::new()
    }

    fn clear(&mut self) {
        self.HEOCPAKCELM = 0;
        self.BGHKBMFHMOJ = 0;
        self.LOOLLAGMNLH = 0;
        self.HECJOOOBAHC = 0;
        self.FKEAAIPKPAA = 0;
        self.ICLEENHIPOH = 0;
        self.AGLEFMECOMC.clear();
        self.ODBONKCMDMP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHDONIGOKNM {
        static instance: CHDONIGOKNM = CHDONIGOKNM {
            HEOCPAKCELM: 0,
            BGHKBMFHMOJ: 0,
            LOOLLAGMNLH: 0,
            HECJOOOBAHC: 0,
            FKEAAIPKPAA: 0,
            ICLEENHIPOH: 0,
            AGLEFMECOMC: ::std::vec::Vec::new(),
            ODBONKCMDMP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CHDONIGOKNM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHDONIGOKNM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHDONIGOKNM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHDONIGOKNM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CHDONIGOKNM.proto\x1a\x11PLPNLIBMNIO.proto\"\xab\x02\n\x0bCHDONIGO\
    KNM\x12\x20\n\x0bHEOCPAKCELM\x18\x01\x20\x01(\rR\x0bHEOCPAKCELM\x12\x20\
    \n\x0bBGHKBMFHMOJ\x18\x02\x20\x01(\rR\x0bBGHKBMFHMOJ\x12\x20\n\x0bLOOLLA\
    GMNLH\x18\x03\x20\x01(\rR\x0bLOOLLAGMNLH\x12\x20\n\x0bHECJOOOBAHC\x18\
    \x04\x20\x01(\rR\x0bHECJOOOBAHC\x12\x20\n\x0bFKEAAIPKPAA\x18\x05\x20\x01\
    (\rR\x0bFKEAAIPKPAA\x12\x20\n\x0bICLEENHIPOH\x18\x06\x20\x01(\rR\x0bICLE\
    ENHIPOH\x12\x20\n\x0bAGLEFMECOMC\x18\x07\x20\x03(\rR\x0bAGLEFMECOMC\x12.\
    \n\x0bODBONKCMDMP\x18\x08\x20\x03(\x0b2\x0c.PLPNLIBMNIOR\x0bODBONKCMDMPb\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::PLPNLIBMNIO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CHDONIGOKNM::generated_message_descriptor_data());
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
