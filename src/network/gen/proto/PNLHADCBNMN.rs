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

//! Generated file from `PNLHADCBNMN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PNLHADCBNMN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PNLHADCBNMN {
    // message fields
    // @@protoc_insertion_point(field:PNLHADCBNMN.MOPIHCLMGMG)
    pub MOPIHCLMGMG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PNLHADCBNMN.GOECAHJKKIP)
    pub GOECAHJKKIP: u32,
    // @@protoc_insertion_point(field:PNLHADCBNMN.IIBOBOIKPMG)
    pub IIBOBOIKPMG: u32,
    // @@protoc_insertion_point(field:PNLHADCBNMN.FHEDFMPMHKL)
    pub FHEDFMPMHKL: u32,
    // @@protoc_insertion_point(field:PNLHADCBNMN.EDNJJFLAAKB)
    pub EDNJJFLAAKB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PNLHADCBNMN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PNLHADCBNMN {
    fn default() -> &'a PNLHADCBNMN {
        <PNLHADCBNMN as ::protobuf::Message>::default_instance()
    }
}

impl PNLHADCBNMN {
    pub fn new() -> PNLHADCBNMN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MOPIHCLMGMG",
            |m: &PNLHADCBNMN| { &m.MOPIHCLMGMG },
            |m: &mut PNLHADCBNMN| { &mut m.MOPIHCLMGMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOECAHJKKIP",
            |m: &PNLHADCBNMN| { &m.GOECAHJKKIP },
            |m: &mut PNLHADCBNMN| { &mut m.GOECAHJKKIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIBOBOIKPMG",
            |m: &PNLHADCBNMN| { &m.IIBOBOIKPMG },
            |m: &mut PNLHADCBNMN| { &mut m.IIBOBOIKPMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHEDFMPMHKL",
            |m: &PNLHADCBNMN| { &m.FHEDFMPMHKL },
            |m: &mut PNLHADCBNMN| { &mut m.FHEDFMPMHKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EDNJJFLAAKB",
            |m: &PNLHADCBNMN| { &m.EDNJJFLAAKB },
            |m: &mut PNLHADCBNMN| { &mut m.EDNJJFLAAKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PNLHADCBNMN>(
            "PNLHADCBNMN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PNLHADCBNMN {
    const NAME: &'static str = "PNLHADCBNMN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.MOPIHCLMGMG)?;
                },
                72 => {
                    self.MOPIHCLMGMG.push(is.read_uint32()?);
                },
                80 => {
                    self.GOECAHJKKIP = is.read_uint32()?;
                },
                120 => {
                    self.IIBOBOIKPMG = is.read_uint32()?;
                },
                32 => {
                    self.FHEDFMPMHKL = is.read_uint32()?;
                },
                64 => {
                    self.EDNJJFLAAKB = is.read_uint32()?;
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
        for value in &self.MOPIHCLMGMG {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if self.GOECAHJKKIP != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.GOECAHJKKIP);
        }
        if self.IIBOBOIKPMG != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.IIBOBOIKPMG);
        }
        if self.FHEDFMPMHKL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FHEDFMPMHKL);
        }
        if self.EDNJJFLAAKB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.EDNJJFLAAKB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MOPIHCLMGMG {
            os.write_uint32(9, *v)?;
        };
        if self.GOECAHJKKIP != 0 {
            os.write_uint32(10, self.GOECAHJKKIP)?;
        }
        if self.IIBOBOIKPMG != 0 {
            os.write_uint32(15, self.IIBOBOIKPMG)?;
        }
        if self.FHEDFMPMHKL != 0 {
            os.write_uint32(4, self.FHEDFMPMHKL)?;
        }
        if self.EDNJJFLAAKB != 0 {
            os.write_uint32(8, self.EDNJJFLAAKB)?;
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

    fn new() -> PNLHADCBNMN {
        PNLHADCBNMN::new()
    }

    fn clear(&mut self) {
        self.MOPIHCLMGMG.clear();
        self.GOECAHJKKIP = 0;
        self.IIBOBOIKPMG = 0;
        self.FHEDFMPMHKL = 0;
        self.EDNJJFLAAKB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PNLHADCBNMN {
        static instance: PNLHADCBNMN = PNLHADCBNMN {
            MOPIHCLMGMG: ::std::vec::Vec::new(),
            GOECAHJKKIP: 0,
            IIBOBOIKPMG: 0,
            FHEDFMPMHKL: 0,
            EDNJJFLAAKB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PNLHADCBNMN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PNLHADCBNMN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PNLHADCBNMN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PNLHADCBNMN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PNLHADCBNMN.proto\"\xb7\x01\n\x0bPNLHADCBNMN\x12\x20\n\x0bMOPIHCLM\
    GMG\x18\t\x20\x03(\rR\x0bMOPIHCLMGMG\x12\x20\n\x0bGOECAHJKKIP\x18\n\x20\
    \x01(\rR\x0bGOECAHJKKIP\x12\x20\n\x0bIIBOBOIKPMG\x18\x0f\x20\x01(\rR\x0b\
    IIBOBOIKPMG\x12\x20\n\x0bFHEDFMPMHKL\x18\x04\x20\x01(\rR\x0bFHEDFMPMHKL\
    \x12\x20\n\x0bEDNJJFLAAKB\x18\x08\x20\x01(\rR\x0bEDNJJFLAAKBb\x06proto3\
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
            messages.push(PNLHADCBNMN::generated_message_descriptor_data());
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