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

//! Generated file from `LMFMKPNJKJG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LMFMKPNJKJG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LMFMKPNJKJG {
    // message fields
    // @@protoc_insertion_point(field:LMFMKPNJKJG.PIKDGNGDCCA)
    pub PIKDGNGDCCA: u32,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.LLIOAIDDBCJ)
    pub LLIOAIDDBCJ: u32,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.PPAEPICFALB)
    pub PPAEPICFALB: bool,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.KFGAADCAFOE)
    pub KFGAADCAFOE: u32,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.HMPPFGCIFJK)
    pub HMPPFGCIFJK: u32,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.KOOKKCNHOPJ)
    pub KOOKKCNHOPJ: bool,
    // @@protoc_insertion_point(field:LMFMKPNJKJG.HBEDLLEAILC)
    pub HBEDLLEAILC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LMFMKPNJKJG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LMFMKPNJKJG {
    fn default() -> &'a LMFMKPNJKJG {
        <LMFMKPNJKJG as ::protobuf::Message>::default_instance()
    }
}

impl LMFMKPNJKJG {
    pub fn new() -> LMFMKPNJKJG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIKDGNGDCCA",
            |m: &LMFMKPNJKJG| { &m.PIKDGNGDCCA },
            |m: &mut LMFMKPNJKJG| { &mut m.PIKDGNGDCCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLIOAIDDBCJ",
            |m: &LMFMKPNJKJG| { &m.LLIOAIDDBCJ },
            |m: &mut LMFMKPNJKJG| { &mut m.LLIOAIDDBCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPAEPICFALB",
            |m: &LMFMKPNJKJG| { &m.PPAEPICFALB },
            |m: &mut LMFMKPNJKJG| { &mut m.PPAEPICFALB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFGAADCAFOE",
            |m: &LMFMKPNJKJG| { &m.KFGAADCAFOE },
            |m: &mut LMFMKPNJKJG| { &mut m.KFGAADCAFOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMPPFGCIFJK",
            |m: &LMFMKPNJKJG| { &m.HMPPFGCIFJK },
            |m: &mut LMFMKPNJKJG| { &mut m.HMPPFGCIFJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOOKKCNHOPJ",
            |m: &LMFMKPNJKJG| { &m.KOOKKCNHOPJ },
            |m: &mut LMFMKPNJKJG| { &mut m.KOOKKCNHOPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBEDLLEAILC",
            |m: &LMFMKPNJKJG| { &m.HBEDLLEAILC },
            |m: &mut LMFMKPNJKJG| { &mut m.HBEDLLEAILC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LMFMKPNJKJG>(
            "LMFMKPNJKJG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LMFMKPNJKJG {
    const NAME: &'static str = "LMFMKPNJKJG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.PIKDGNGDCCA = is.read_uint32()?;
                },
                96 => {
                    self.LLIOAIDDBCJ = is.read_uint32()?;
                },
                40 => {
                    self.PPAEPICFALB = is.read_bool()?;
                },
                120 => {
                    self.KFGAADCAFOE = is.read_uint32()?;
                },
                48 => {
                    self.HMPPFGCIFJK = is.read_uint32()?;
                },
                8 => {
                    self.KOOKKCNHOPJ = is.read_bool()?;
                },
                72 => {
                    self.HBEDLLEAILC = is.read_uint32()?;
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
        if self.PIKDGNGDCCA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PIKDGNGDCCA);
        }
        if self.LLIOAIDDBCJ != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LLIOAIDDBCJ);
        }
        if self.PPAEPICFALB != false {
            my_size += 1 + 1;
        }
        if self.KFGAADCAFOE != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.KFGAADCAFOE);
        }
        if self.HMPPFGCIFJK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.HMPPFGCIFJK);
        }
        if self.KOOKKCNHOPJ != false {
            my_size += 1 + 1;
        }
        if self.HBEDLLEAILC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.HBEDLLEAILC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PIKDGNGDCCA != 0 {
            os.write_uint32(2, self.PIKDGNGDCCA)?;
        }
        if self.LLIOAIDDBCJ != 0 {
            os.write_uint32(12, self.LLIOAIDDBCJ)?;
        }
        if self.PPAEPICFALB != false {
            os.write_bool(5, self.PPAEPICFALB)?;
        }
        if self.KFGAADCAFOE != 0 {
            os.write_uint32(15, self.KFGAADCAFOE)?;
        }
        if self.HMPPFGCIFJK != 0 {
            os.write_uint32(6, self.HMPPFGCIFJK)?;
        }
        if self.KOOKKCNHOPJ != false {
            os.write_bool(1, self.KOOKKCNHOPJ)?;
        }
        if self.HBEDLLEAILC != 0 {
            os.write_uint32(9, self.HBEDLLEAILC)?;
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

    fn new() -> LMFMKPNJKJG {
        LMFMKPNJKJG::new()
    }

    fn clear(&mut self) {
        self.PIKDGNGDCCA = 0;
        self.LLIOAIDDBCJ = 0;
        self.PPAEPICFALB = false;
        self.KFGAADCAFOE = 0;
        self.HMPPFGCIFJK = 0;
        self.KOOKKCNHOPJ = false;
        self.HBEDLLEAILC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LMFMKPNJKJG {
        static instance: LMFMKPNJKJG = LMFMKPNJKJG {
            PIKDGNGDCCA: 0,
            LLIOAIDDBCJ: 0,
            PPAEPICFALB: false,
            KFGAADCAFOE: 0,
            HMPPFGCIFJK: 0,
            KOOKKCNHOPJ: false,
            HBEDLLEAILC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LMFMKPNJKJG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LMFMKPNJKJG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LMFMKPNJKJG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LMFMKPNJKJG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LMFMKPNJKJG.proto\"\xfb\x01\n\x0bLMFMKPNJKJG\x12\x20\n\x0bPIKDGNGD\
    CCA\x18\x02\x20\x01(\rR\x0bPIKDGNGDCCA\x12\x20\n\x0bLLIOAIDDBCJ\x18\x0c\
    \x20\x01(\rR\x0bLLIOAIDDBCJ\x12\x20\n\x0bPPAEPICFALB\x18\x05\x20\x01(\
    \x08R\x0bPPAEPICFALB\x12\x20\n\x0bKFGAADCAFOE\x18\x0f\x20\x01(\rR\x0bKFG\
    AADCAFOE\x12\x20\n\x0bHMPPFGCIFJK\x18\x06\x20\x01(\rR\x0bHMPPFGCIFJK\x12\
    \x20\n\x0bKOOKKCNHOPJ\x18\x01\x20\x01(\x08R\x0bKOOKKCNHOPJ\x12\x20\n\x0b\
    HBEDLLEAILC\x18\t\x20\x01(\rR\x0bHBEDLLEAILCb\x06proto3\
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
            messages.push(LMFMKPNJKJG::generated_message_descriptor_data());
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