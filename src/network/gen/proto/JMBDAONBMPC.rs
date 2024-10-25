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

//! Generated file from `JMBDAONBMPC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JMBDAONBMPC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JMBDAONBMPC {
    // message fields
    // @@protoc_insertion_point(field:JMBDAONBMPC.NCFAJPAMBGD)
    pub NCFAJPAMBGD: u32,
    // @@protoc_insertion_point(field:JMBDAONBMPC.ACLJKNHDFFL)
    pub ACLJKNHDFFL: u32,
    // @@protoc_insertion_point(field:JMBDAONBMPC.OJJLBIPFMAP)
    pub OJJLBIPFMAP: u32,
    // @@protoc_insertion_point(field:JMBDAONBMPC.BBFJCEGOIJN)
    pub BBFJCEGOIJN: bool,
    // @@protoc_insertion_point(field:JMBDAONBMPC.DHLPHKJBMHB)
    pub DHLPHKJBMHB: u32,
    // @@protoc_insertion_point(field:JMBDAONBMPC.AFKFMBNFCGE)
    pub AFKFMBNFCGE: bool,
    // @@protoc_insertion_point(field:JMBDAONBMPC.CNIJGAPOPAH)
    pub CNIJGAPOPAH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JMBDAONBMPC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JMBDAONBMPC {
    fn default() -> &'a JMBDAONBMPC {
        <JMBDAONBMPC as ::protobuf::Message>::default_instance()
    }
}

impl JMBDAONBMPC {
    pub fn new() -> JMBDAONBMPC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCFAJPAMBGD",
            |m: &JMBDAONBMPC| { &m.NCFAJPAMBGD },
            |m: &mut JMBDAONBMPC| { &mut m.NCFAJPAMBGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACLJKNHDFFL",
            |m: &JMBDAONBMPC| { &m.ACLJKNHDFFL },
            |m: &mut JMBDAONBMPC| { &mut m.ACLJKNHDFFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJJLBIPFMAP",
            |m: &JMBDAONBMPC| { &m.OJJLBIPFMAP },
            |m: &mut JMBDAONBMPC| { &mut m.OJJLBIPFMAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBFJCEGOIJN",
            |m: &JMBDAONBMPC| { &m.BBFJCEGOIJN },
            |m: &mut JMBDAONBMPC| { &mut m.BBFJCEGOIJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHLPHKJBMHB",
            |m: &JMBDAONBMPC| { &m.DHLPHKJBMHB },
            |m: &mut JMBDAONBMPC| { &mut m.DHLPHKJBMHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFKFMBNFCGE",
            |m: &JMBDAONBMPC| { &m.AFKFMBNFCGE },
            |m: &mut JMBDAONBMPC| { &mut m.AFKFMBNFCGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNIJGAPOPAH",
            |m: &JMBDAONBMPC| { &m.CNIJGAPOPAH },
            |m: &mut JMBDAONBMPC| { &mut m.CNIJGAPOPAH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JMBDAONBMPC>(
            "JMBDAONBMPC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JMBDAONBMPC {
    const NAME: &'static str = "JMBDAONBMPC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.NCFAJPAMBGD = is.read_uint32()?;
                },
                88 => {
                    self.ACLJKNHDFFL = is.read_uint32()?;
                },
                64 => {
                    self.OJJLBIPFMAP = is.read_uint32()?;
                },
                24 => {
                    self.BBFJCEGOIJN = is.read_bool()?;
                },
                72 => {
                    self.DHLPHKJBMHB = is.read_uint32()?;
                },
                120 => {
                    self.AFKFMBNFCGE = is.read_bool()?;
                },
                32 => {
                    self.CNIJGAPOPAH = is.read_uint32()?;
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
        if self.NCFAJPAMBGD != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NCFAJPAMBGD);
        }
        if self.ACLJKNHDFFL != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.ACLJKNHDFFL);
        }
        if self.OJJLBIPFMAP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OJJLBIPFMAP);
        }
        if self.BBFJCEGOIJN != false {
            my_size += 1 + 1;
        }
        if self.DHLPHKJBMHB != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DHLPHKJBMHB);
        }
        if self.AFKFMBNFCGE != false {
            my_size += 1 + 1;
        }
        if self.CNIJGAPOPAH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.CNIJGAPOPAH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NCFAJPAMBGD != 0 {
            os.write_uint32(6, self.NCFAJPAMBGD)?;
        }
        if self.ACLJKNHDFFL != 0 {
            os.write_uint32(11, self.ACLJKNHDFFL)?;
        }
        if self.OJJLBIPFMAP != 0 {
            os.write_uint32(8, self.OJJLBIPFMAP)?;
        }
        if self.BBFJCEGOIJN != false {
            os.write_bool(3, self.BBFJCEGOIJN)?;
        }
        if self.DHLPHKJBMHB != 0 {
            os.write_uint32(9, self.DHLPHKJBMHB)?;
        }
        if self.AFKFMBNFCGE != false {
            os.write_bool(15, self.AFKFMBNFCGE)?;
        }
        if self.CNIJGAPOPAH != 0 {
            os.write_uint32(4, self.CNIJGAPOPAH)?;
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

    fn new() -> JMBDAONBMPC {
        JMBDAONBMPC::new()
    }

    fn clear(&mut self) {
        self.NCFAJPAMBGD = 0;
        self.ACLJKNHDFFL = 0;
        self.OJJLBIPFMAP = 0;
        self.BBFJCEGOIJN = false;
        self.DHLPHKJBMHB = 0;
        self.AFKFMBNFCGE = false;
        self.CNIJGAPOPAH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JMBDAONBMPC {
        static instance: JMBDAONBMPC = JMBDAONBMPC {
            NCFAJPAMBGD: 0,
            ACLJKNHDFFL: 0,
            OJJLBIPFMAP: 0,
            BBFJCEGOIJN: false,
            DHLPHKJBMHB: 0,
            AFKFMBNFCGE: false,
            CNIJGAPOPAH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JMBDAONBMPC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JMBDAONBMPC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JMBDAONBMPC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JMBDAONBMPC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JMBDAONBMPC.proto\"\xfb\x01\n\x0bJMBDAONBMPC\x12\x20\n\x0bNCFAJPAM\
    BGD\x18\x06\x20\x01(\rR\x0bNCFAJPAMBGD\x12\x20\n\x0bACLJKNHDFFL\x18\x0b\
    \x20\x01(\rR\x0bACLJKNHDFFL\x12\x20\n\x0bOJJLBIPFMAP\x18\x08\x20\x01(\rR\
    \x0bOJJLBIPFMAP\x12\x20\n\x0bBBFJCEGOIJN\x18\x03\x20\x01(\x08R\x0bBBFJCE\
    GOIJN\x12\x20\n\x0bDHLPHKJBMHB\x18\t\x20\x01(\rR\x0bDHLPHKJBMHB\x12\x20\
    \n\x0bAFKFMBNFCGE\x18\x0f\x20\x01(\x08R\x0bAFKFMBNFCGE\x12\x20\n\x0bCNIJ\
    GAPOPAH\x18\x04\x20\x01(\rR\x0bCNIJGAPOPAHb\x06proto3\
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
            messages.push(JMBDAONBMPC::generated_message_descriptor_data());
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
