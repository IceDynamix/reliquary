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

//! Generated file from `OJOBBCKLNFK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OJOBBCKLNFK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OJOBBCKLNFK {
    // message fields
    // @@protoc_insertion_point(field:OJOBBCKLNFK.NAPBHFPEFAB)
    pub NAPBHFPEFAB: u32,
    // @@protoc_insertion_point(field:OJOBBCKLNFK.AJCHIGGPBAH)
    pub AJCHIGGPBAH: u32,
    // @@protoc_insertion_point(field:OJOBBCKLNFK.FGIMEDPPDHI)
    pub FGIMEDPPDHI: ::protobuf::EnumOrUnknown<super::OFNMFJICEOP::OFNMFJICEOP>,
    // @@protoc_insertion_point(field:OJOBBCKLNFK.BFAHIMCFHNM)
    pub BFAHIMCFHNM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OJOBBCKLNFK.IGBFOBIGLAH)
    pub IGBFOBIGLAH: u32,
    // @@protoc_insertion_point(field:OJOBBCKLNFK.PBAAPCAMCAB)
    pub PBAAPCAMCAB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OJOBBCKLNFK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OJOBBCKLNFK {
    fn default() -> &'a OJOBBCKLNFK {
        <OJOBBCKLNFK as ::protobuf::Message>::default_instance()
    }
}

impl OJOBBCKLNFK {
    pub fn new() -> OJOBBCKLNFK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAPBHFPEFAB",
            |m: &OJOBBCKLNFK| { &m.NAPBHFPEFAB },
            |m: &mut OJOBBCKLNFK| { &mut m.NAPBHFPEFAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJCHIGGPBAH",
            |m: &OJOBBCKLNFK| { &m.AJCHIGGPBAH },
            |m: &mut OJOBBCKLNFK| { &mut m.AJCHIGGPBAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGIMEDPPDHI",
            |m: &OJOBBCKLNFK| { &m.FGIMEDPPDHI },
            |m: &mut OJOBBCKLNFK| { &mut m.FGIMEDPPDHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BFAHIMCFHNM",
            |m: &OJOBBCKLNFK| { &m.BFAHIMCFHNM },
            |m: &mut OJOBBCKLNFK| { &mut m.BFAHIMCFHNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGBFOBIGLAH",
            |m: &OJOBBCKLNFK| { &m.IGBFOBIGLAH },
            |m: &mut OJOBBCKLNFK| { &mut m.IGBFOBIGLAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBAAPCAMCAB",
            |m: &OJOBBCKLNFK| { &m.PBAAPCAMCAB },
            |m: &mut OJOBBCKLNFK| { &mut m.PBAAPCAMCAB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OJOBBCKLNFK>(
            "OJOBBCKLNFK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OJOBBCKLNFK {
    const NAME: &'static str = "OJOBBCKLNFK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.NAPBHFPEFAB = is.read_uint32()?;
                },
                32 => {
                    self.AJCHIGGPBAH = is.read_uint32()?;
                },
                112 => {
                    self.FGIMEDPPDHI = is.read_enum_or_unknown()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.BFAHIMCFHNM)?;
                },
                104 => {
                    self.BFAHIMCFHNM.push(is.read_uint32()?);
                },
                64 => {
                    self.IGBFOBIGLAH = is.read_uint32()?;
                },
                72 => {
                    self.PBAAPCAMCAB = is.read_uint32()?;
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
        if self.NAPBHFPEFAB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NAPBHFPEFAB);
        }
        if self.AJCHIGGPBAH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.AJCHIGGPBAH);
        }
        if self.FGIMEDPPDHI != ::protobuf::EnumOrUnknown::new(super::OFNMFJICEOP::OFNMFJICEOP::OFFERING_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.FGIMEDPPDHI.value());
        }
        for value in &self.BFAHIMCFHNM {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.IGBFOBIGLAH != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IGBFOBIGLAH);
        }
        if self.PBAAPCAMCAB != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PBAAPCAMCAB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NAPBHFPEFAB != 0 {
            os.write_uint32(6, self.NAPBHFPEFAB)?;
        }
        if self.AJCHIGGPBAH != 0 {
            os.write_uint32(4, self.AJCHIGGPBAH)?;
        }
        if self.FGIMEDPPDHI != ::protobuf::EnumOrUnknown::new(super::OFNMFJICEOP::OFNMFJICEOP::OFFERING_STATE_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.FGIMEDPPDHI))?;
        }
        for v in &self.BFAHIMCFHNM {
            os.write_uint32(13, *v)?;
        };
        if self.IGBFOBIGLAH != 0 {
            os.write_uint32(8, self.IGBFOBIGLAH)?;
        }
        if self.PBAAPCAMCAB != 0 {
            os.write_uint32(9, self.PBAAPCAMCAB)?;
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

    fn new() -> OJOBBCKLNFK {
        OJOBBCKLNFK::new()
    }

    fn clear(&mut self) {
        self.NAPBHFPEFAB = 0;
        self.AJCHIGGPBAH = 0;
        self.FGIMEDPPDHI = ::protobuf::EnumOrUnknown::new(super::OFNMFJICEOP::OFNMFJICEOP::OFFERING_STATE_NONE);
        self.BFAHIMCFHNM.clear();
        self.IGBFOBIGLAH = 0;
        self.PBAAPCAMCAB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OJOBBCKLNFK {
        static instance: OJOBBCKLNFK = OJOBBCKLNFK {
            NAPBHFPEFAB: 0,
            AJCHIGGPBAH: 0,
            FGIMEDPPDHI: ::protobuf::EnumOrUnknown::from_i32(0),
            BFAHIMCFHNM: ::std::vec::Vec::new(),
            IGBFOBIGLAH: 0,
            PBAAPCAMCAB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OJOBBCKLNFK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OJOBBCKLNFK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OJOBBCKLNFK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OJOBBCKLNFK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OJOBBCKLNFK.proto\x1a\x11OFNMFJICEOP.proto\"\xe7\x01\n\x0bOJOBBCKL\
    NFK\x12\x20\n\x0bNAPBHFPEFAB\x18\x06\x20\x01(\rR\x0bNAPBHFPEFAB\x12\x20\
    \n\x0bAJCHIGGPBAH\x18\x04\x20\x01(\rR\x0bAJCHIGGPBAH\x12.\n\x0bFGIMEDPPD\
    HI\x18\x0e\x20\x01(\x0e2\x0c.OFNMFJICEOPR\x0bFGIMEDPPDHI\x12\x20\n\x0bBF\
    AHIMCFHNM\x18\r\x20\x03(\rR\x0bBFAHIMCFHNM\x12\x20\n\x0bIGBFOBIGLAH\x18\
    \x08\x20\x01(\rR\x0bIGBFOBIGLAH\x12\x20\n\x0bPBAAPCAMCAB\x18\t\x20\x01(\
    \rR\x0bPBAAPCAMCABb\x06proto3\
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
            deps.push(super::OFNMFJICEOP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OJOBBCKLNFK::generated_message_descriptor_data());
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