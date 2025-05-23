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

//! Generated file from `PLPNLIBMNIO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PLPNLIBMNIO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PLPNLIBMNIO {
    // message fields
    // @@protoc_insertion_point(field:PLPNLIBMNIO.phase)
    pub phase: ::std::string::String,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.DBDCNAFOGLF)
    pub DBDCNAFOGLF: f32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.HDALBIANCMF)
    pub HDALBIANCMF: f32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.ADJBBABEHAH)
    pub ADJBBABEHAH: u32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.PJBIAEJECAE)
    pub PJBIAEJECAE: u32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.KPNACGHJALJ)
    pub KPNACGHJALJ: u32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.FPJADBGOHKM)
    pub FPJADBGOHKM: u32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.CJEJOFAMDCD)
    pub CJEJOFAMDCD: u32,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.BGJCEDEAHGM)
    pub BGJCEDEAHGM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PLPNLIBMNIO.AAGJCJIOFPA)
    pub AAGJCJIOFPA: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PLPNLIBMNIO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PLPNLIBMNIO {
    fn default() -> &'a PLPNLIBMNIO {
        <PLPNLIBMNIO as ::protobuf::Message>::default_instance()
    }
}

impl PLPNLIBMNIO {
    pub fn new() -> PLPNLIBMNIO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "phase",
            |m: &PLPNLIBMNIO| { &m.phase },
            |m: &mut PLPNLIBMNIO| { &mut m.phase },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBDCNAFOGLF",
            |m: &PLPNLIBMNIO| { &m.DBDCNAFOGLF },
            |m: &mut PLPNLIBMNIO| { &mut m.DBDCNAFOGLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDALBIANCMF",
            |m: &PLPNLIBMNIO| { &m.HDALBIANCMF },
            |m: &mut PLPNLIBMNIO| { &mut m.HDALBIANCMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADJBBABEHAH",
            |m: &PLPNLIBMNIO| { &m.ADJBBABEHAH },
            |m: &mut PLPNLIBMNIO| { &mut m.ADJBBABEHAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJBIAEJECAE",
            |m: &PLPNLIBMNIO| { &m.PJBIAEJECAE },
            |m: &mut PLPNLIBMNIO| { &mut m.PJBIAEJECAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPNACGHJALJ",
            |m: &PLPNLIBMNIO| { &m.KPNACGHJALJ },
            |m: &mut PLPNLIBMNIO| { &mut m.KPNACGHJALJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPJADBGOHKM",
            |m: &PLPNLIBMNIO| { &m.FPJADBGOHKM },
            |m: &mut PLPNLIBMNIO| { &mut m.FPJADBGOHKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CJEJOFAMDCD",
            |m: &PLPNLIBMNIO| { &m.CJEJOFAMDCD },
            |m: &mut PLPNLIBMNIO| { &mut m.CJEJOFAMDCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BGJCEDEAHGM",
            |m: &PLPNLIBMNIO| { &m.BGJCEDEAHGM },
            |m: &mut PLPNLIBMNIO| { &mut m.BGJCEDEAHGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AAGJCJIOFPA",
            |m: &PLPNLIBMNIO| { &m.AAGJCJIOFPA },
            |m: &mut PLPNLIBMNIO| { &mut m.AAGJCJIOFPA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PLPNLIBMNIO>(
            "PLPNLIBMNIO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PLPNLIBMNIO {
    const NAME: &'static str = "PLPNLIBMNIO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.phase = is.read_string()?;
                },
                21 => {
                    self.DBDCNAFOGLF = is.read_float()?;
                },
                29 => {
                    self.HDALBIANCMF = is.read_float()?;
                },
                32 => {
                    self.ADJBBABEHAH = is.read_uint32()?;
                },
                40 => {
                    self.PJBIAEJECAE = is.read_uint32()?;
                },
                48 => {
                    self.KPNACGHJALJ = is.read_uint32()?;
                },
                56 => {
                    self.FPJADBGOHKM = is.read_uint32()?;
                },
                64 => {
                    self.CJEJOFAMDCD = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.BGJCEDEAHGM)?;
                },
                72 => {
                    self.BGJCEDEAHGM.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.AAGJCJIOFPA)?;
                },
                80 => {
                    self.AAGJCJIOFPA.push(is.read_uint32()?);
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
        if !self.phase.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.phase);
        }
        if self.DBDCNAFOGLF != 0. {
            my_size += 1 + 4;
        }
        if self.HDALBIANCMF != 0. {
            my_size += 1 + 4;
        }
        if self.ADJBBABEHAH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ADJBBABEHAH);
        }
        if self.PJBIAEJECAE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.PJBIAEJECAE);
        }
        if self.KPNACGHJALJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KPNACGHJALJ);
        }
        if self.FPJADBGOHKM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FPJADBGOHKM);
        }
        if self.CJEJOFAMDCD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CJEJOFAMDCD);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.BGJCEDEAHGM);
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.AAGJCJIOFPA);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.phase.is_empty() {
            os.write_string(1, &self.phase)?;
        }
        if self.DBDCNAFOGLF != 0. {
            os.write_float(2, self.DBDCNAFOGLF)?;
        }
        if self.HDALBIANCMF != 0. {
            os.write_float(3, self.HDALBIANCMF)?;
        }
        if self.ADJBBABEHAH != 0 {
            os.write_uint32(4, self.ADJBBABEHAH)?;
        }
        if self.PJBIAEJECAE != 0 {
            os.write_uint32(5, self.PJBIAEJECAE)?;
        }
        if self.KPNACGHJALJ != 0 {
            os.write_uint32(6, self.KPNACGHJALJ)?;
        }
        if self.FPJADBGOHKM != 0 {
            os.write_uint32(7, self.FPJADBGOHKM)?;
        }
        if self.CJEJOFAMDCD != 0 {
            os.write_uint32(8, self.CJEJOFAMDCD)?;
        }
        os.write_repeated_packed_uint32(9, &self.BGJCEDEAHGM)?;
        os.write_repeated_packed_uint32(10, &self.AAGJCJIOFPA)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PLPNLIBMNIO {
        PLPNLIBMNIO::new()
    }

    fn clear(&mut self) {
        self.phase.clear();
        self.DBDCNAFOGLF = 0.;
        self.HDALBIANCMF = 0.;
        self.ADJBBABEHAH = 0;
        self.PJBIAEJECAE = 0;
        self.KPNACGHJALJ = 0;
        self.FPJADBGOHKM = 0;
        self.CJEJOFAMDCD = 0;
        self.BGJCEDEAHGM.clear();
        self.AAGJCJIOFPA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PLPNLIBMNIO {
        static instance: PLPNLIBMNIO = PLPNLIBMNIO {
            phase: ::std::string::String::new(),
            DBDCNAFOGLF: 0.,
            HDALBIANCMF: 0.,
            ADJBBABEHAH: 0,
            PJBIAEJECAE: 0,
            KPNACGHJALJ: 0,
            FPJADBGOHKM: 0,
            CJEJOFAMDCD: 0,
            BGJCEDEAHGM: ::std::vec::Vec::new(),
            AAGJCJIOFPA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PLPNLIBMNIO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PLPNLIBMNIO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PLPNLIBMNIO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PLPNLIBMNIO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PLPNLIBMNIO.proto\"\xd5\x02\n\x0bPLPNLIBMNIO\x12\x14\n\x05phase\
    \x18\x01\x20\x01(\tR\x05phase\x12\x20\n\x0bDBDCNAFOGLF\x18\x02\x20\x01(\
    \x02R\x0bDBDCNAFOGLF\x12\x20\n\x0bHDALBIANCMF\x18\x03\x20\x01(\x02R\x0bH\
    DALBIANCMF\x12\x20\n\x0bADJBBABEHAH\x18\x04\x20\x01(\rR\x0bADJBBABEHAH\
    \x12\x20\n\x0bPJBIAEJECAE\x18\x05\x20\x01(\rR\x0bPJBIAEJECAE\x12\x20\n\
    \x0bKPNACGHJALJ\x18\x06\x20\x01(\rR\x0bKPNACGHJALJ\x12\x20\n\x0bFPJADBGO\
    HKM\x18\x07\x20\x01(\rR\x0bFPJADBGOHKM\x12\x20\n\x0bCJEJOFAMDCD\x18\x08\
    \x20\x01(\rR\x0bCJEJOFAMDCD\x12\x20\n\x0bBGJCEDEAHGM\x18\t\x20\x03(\rR\
    \x0bBGJCEDEAHGM\x12\x20\n\x0bAAGJCJIOFPA\x18\n\x20\x03(\rR\x0bAAGJCJIOFP\
    Ab\x06proto3\
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
            messages.push(PLPNLIBMNIO::generated_message_descriptor_data());
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
