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

//! Generated file from `AAPNHPAMDCK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AAPNHPAMDCK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AAPNHPAMDCK {
    // message fields
    // @@protoc_insertion_point(field:AAPNHPAMDCK.MPAMHBFIKFB)
    pub MPAMHBFIKFB: ::std::string::String,
    // @@protoc_insertion_point(field:AAPNHPAMDCK.DBELLIHMAJG)
    pub DBELLIHMAJG: ::std::string::String,
    // @@protoc_insertion_point(field:AAPNHPAMDCK.IOGLPEBJMDB)
    pub IOGLPEBJMDB: u32,
    // @@protoc_insertion_point(field:AAPNHPAMDCK.FKFOKHABEJN)
    pub FKFOKHABEJN: bool,
    // @@protoc_insertion_point(field:AAPNHPAMDCK.GCBOBAMCALK)
    pub GCBOBAMCALK: u32,
    // @@protoc_insertion_point(field:AAPNHPAMDCK.DNAJFMPCMLL)
    pub DNAJFMPCMLL: ::protobuf::EnumOrUnknown<super::ProductGiftType::ProductGiftType>,
    // special fields
    // @@protoc_insertion_point(special_field:AAPNHPAMDCK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AAPNHPAMDCK {
    fn default() -> &'a AAPNHPAMDCK {
        <AAPNHPAMDCK as ::protobuf::Message>::default_instance()
    }
}

impl AAPNHPAMDCK {
    pub fn new() -> AAPNHPAMDCK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPAMHBFIKFB",
            |m: &AAPNHPAMDCK| { &m.MPAMHBFIKFB },
            |m: &mut AAPNHPAMDCK| { &mut m.MPAMHBFIKFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBELLIHMAJG",
            |m: &AAPNHPAMDCK| { &m.DBELLIHMAJG },
            |m: &mut AAPNHPAMDCK| { &mut m.DBELLIHMAJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOGLPEBJMDB",
            |m: &AAPNHPAMDCK| { &m.IOGLPEBJMDB },
            |m: &mut AAPNHPAMDCK| { &mut m.IOGLPEBJMDB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKFOKHABEJN",
            |m: &AAPNHPAMDCK| { &m.FKFOKHABEJN },
            |m: &mut AAPNHPAMDCK| { &mut m.FKFOKHABEJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCBOBAMCALK",
            |m: &AAPNHPAMDCK| { &m.GCBOBAMCALK },
            |m: &mut AAPNHPAMDCK| { &mut m.GCBOBAMCALK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNAJFMPCMLL",
            |m: &AAPNHPAMDCK| { &m.DNAJFMPCMLL },
            |m: &mut AAPNHPAMDCK| { &mut m.DNAJFMPCMLL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AAPNHPAMDCK>(
            "AAPNHPAMDCK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AAPNHPAMDCK {
    const NAME: &'static str = "AAPNHPAMDCK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.MPAMHBFIKFB = is.read_string()?;
                },
                74 => {
                    self.DBELLIHMAJG = is.read_string()?;
                },
                16 => {
                    self.IOGLPEBJMDB = is.read_uint32()?;
                },
                40 => {
                    self.FKFOKHABEJN = is.read_bool()?;
                },
                88 => {
                    self.GCBOBAMCALK = is.read_uint32()?;
                },
                64 => {
                    self.DNAJFMPCMLL = is.read_enum_or_unknown()?;
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
        if !self.MPAMHBFIKFB.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.MPAMHBFIKFB);
        }
        if !self.DBELLIHMAJG.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.DBELLIHMAJG);
        }
        if self.IOGLPEBJMDB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IOGLPEBJMDB);
        }
        if self.FKFOKHABEJN != false {
            my_size += 1 + 1;
        }
        if self.GCBOBAMCALK != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.GCBOBAMCALK);
        }
        if self.DNAJFMPCMLL != ::protobuf::EnumOrUnknown::new(super::ProductGiftType::ProductGiftType::PRODUCT_GIFT_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.DNAJFMPCMLL.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.MPAMHBFIKFB.is_empty() {
            os.write_string(15, &self.MPAMHBFIKFB)?;
        }
        if !self.DBELLIHMAJG.is_empty() {
            os.write_string(9, &self.DBELLIHMAJG)?;
        }
        if self.IOGLPEBJMDB != 0 {
            os.write_uint32(2, self.IOGLPEBJMDB)?;
        }
        if self.FKFOKHABEJN != false {
            os.write_bool(5, self.FKFOKHABEJN)?;
        }
        if self.GCBOBAMCALK != 0 {
            os.write_uint32(11, self.GCBOBAMCALK)?;
        }
        if self.DNAJFMPCMLL != ::protobuf::EnumOrUnknown::new(super::ProductGiftType::ProductGiftType::PRODUCT_GIFT_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.DNAJFMPCMLL))?;
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

    fn new() -> AAPNHPAMDCK {
        AAPNHPAMDCK::new()
    }

    fn clear(&mut self) {
        self.MPAMHBFIKFB.clear();
        self.DBELLIHMAJG.clear();
        self.IOGLPEBJMDB = 0;
        self.FKFOKHABEJN = false;
        self.GCBOBAMCALK = 0;
        self.DNAJFMPCMLL = ::protobuf::EnumOrUnknown::new(super::ProductGiftType::ProductGiftType::PRODUCT_GIFT_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AAPNHPAMDCK {
        static instance: AAPNHPAMDCK = AAPNHPAMDCK {
            MPAMHBFIKFB: ::std::string::String::new(),
            DBELLIHMAJG: ::std::string::String::new(),
            IOGLPEBJMDB: 0,
            FKFOKHABEJN: false,
            GCBOBAMCALK: 0,
            DNAJFMPCMLL: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AAPNHPAMDCK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AAPNHPAMDCK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AAPNHPAMDCK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AAPNHPAMDCK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AAPNHPAMDCK.proto\x1a\x15ProductGiftType.proto\"\xeb\x01\n\x0bAAPN\
    HPAMDCK\x12\x20\n\x0bMPAMHBFIKFB\x18\x0f\x20\x01(\tR\x0bMPAMHBFIKFB\x12\
    \x20\n\x0bDBELLIHMAJG\x18\t\x20\x01(\tR\x0bDBELLIHMAJG\x12\x20\n\x0bIOGL\
    PEBJMDB\x18\x02\x20\x01(\rR\x0bIOGLPEBJMDB\x12\x20\n\x0bFKFOKHABEJN\x18\
    \x05\x20\x01(\x08R\x0bFKFOKHABEJN\x12\x20\n\x0bGCBOBAMCALK\x18\x0b\x20\
    \x01(\rR\x0bGCBOBAMCALK\x122\n\x0bDNAJFMPCMLL\x18\x08\x20\x01(\x0e2\x10.\
    ProductGiftTypeR\x0bDNAJFMPCMLLb\x06proto3\
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
            deps.push(super::ProductGiftType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AAPNHPAMDCK::generated_message_descriptor_data());
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
