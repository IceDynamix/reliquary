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

//! Generated file from `ADGNFGFBCPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ADGNFGFBCPE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ADGNFGFBCPE {
    // message fields
    // @@protoc_insertion_point(field:ADGNFGFBCPE.ALJLKAOELDP)
    pub ALJLKAOELDP: u32,
    // @@protoc_insertion_point(field:ADGNFGFBCPE.BLOEFNLOLLJ)
    pub BLOEFNLOLLJ: u32,
    // @@protoc_insertion_point(field:ADGNFGFBCPE.CINMLCKBHIM)
    pub CINMLCKBHIM: bool,
    // @@protoc_insertion_point(field:ADGNFGFBCPE.PCLMNBILAPH)
    pub PCLMNBILAPH: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:ADGNFGFBCPE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ADGNFGFBCPE {
    fn default() -> &'a ADGNFGFBCPE {
        <ADGNFGFBCPE as ::protobuf::Message>::default_instance()
    }
}

impl ADGNFGFBCPE {
    pub fn new() -> ADGNFGFBCPE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALJLKAOELDP",
            |m: &ADGNFGFBCPE| { &m.ALJLKAOELDP },
            |m: &mut ADGNFGFBCPE| { &mut m.ALJLKAOELDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLOEFNLOLLJ",
            |m: &ADGNFGFBCPE| { &m.BLOEFNLOLLJ },
            |m: &mut ADGNFGFBCPE| { &mut m.BLOEFNLOLLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CINMLCKBHIM",
            |m: &ADGNFGFBCPE| { &m.CINMLCKBHIM },
            |m: &mut ADGNFGFBCPE| { &mut m.CINMLCKBHIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCLMNBILAPH",
            |m: &ADGNFGFBCPE| { &m.PCLMNBILAPH },
            |m: &mut ADGNFGFBCPE| { &mut m.PCLMNBILAPH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ADGNFGFBCPE>(
            "ADGNFGFBCPE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ADGNFGFBCPE {
    const NAME: &'static str = "ADGNFGFBCPE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.ALJLKAOELDP = is.read_uint32()?;
                },
                72 => {
                    self.BLOEFNLOLLJ = is.read_uint32()?;
                },
                24 => {
                    self.CINMLCKBHIM = is.read_bool()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.PCLMNBILAPH)?;
                },
                8 => {
                    self.PCLMNBILAPH.push(is.read_uint32()?);
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
        if self.ALJLKAOELDP != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ALJLKAOELDP);
        }
        if self.BLOEFNLOLLJ != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BLOEFNLOLLJ);
        }
        if self.CINMLCKBHIM != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.PCLMNBILAPH);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ALJLKAOELDP != 0 {
            os.write_uint32(4, self.ALJLKAOELDP)?;
        }
        if self.BLOEFNLOLLJ != 0 {
            os.write_uint32(9, self.BLOEFNLOLLJ)?;
        }
        if self.CINMLCKBHIM != false {
            os.write_bool(3, self.CINMLCKBHIM)?;
        }
        os.write_repeated_packed_uint32(1, &self.PCLMNBILAPH)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ADGNFGFBCPE {
        ADGNFGFBCPE::new()
    }

    fn clear(&mut self) {
        self.ALJLKAOELDP = 0;
        self.BLOEFNLOLLJ = 0;
        self.CINMLCKBHIM = false;
        self.PCLMNBILAPH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ADGNFGFBCPE {
        static instance: ADGNFGFBCPE = ADGNFGFBCPE {
            ALJLKAOELDP: 0,
            BLOEFNLOLLJ: 0,
            CINMLCKBHIM: false,
            PCLMNBILAPH: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ADGNFGFBCPE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ADGNFGFBCPE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ADGNFGFBCPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ADGNFGFBCPE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ADGNFGFBCPE.proto\"\x95\x01\n\x0bADGNFGFBCPE\x12\x20\n\x0bALJLKAOE\
    LDP\x18\x04\x20\x01(\rR\x0bALJLKAOELDP\x12\x20\n\x0bBLOEFNLOLLJ\x18\t\
    \x20\x01(\rR\x0bBLOEFNLOLLJ\x12\x20\n\x0bCINMLCKBHIM\x18\x03\x20\x01(\
    \x08R\x0bCINMLCKBHIM\x12\x20\n\x0bPCLMNBILAPH\x18\x01\x20\x03(\rR\x0bPCL\
    MNBILAPHb\x06proto3\
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
            messages.push(ADGNFGFBCPE::generated_message_descriptor_data());
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
