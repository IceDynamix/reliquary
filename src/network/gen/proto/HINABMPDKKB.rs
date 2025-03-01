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

//! Generated file from `HINABMPDKKB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HINABMPDKKB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HINABMPDKKB {
    // message fields
    // @@protoc_insertion_point(field:HINABMPDKKB.BCCGJIHNCDN)
    pub BCCGJIHNCDN: ::std::vec::Vec<super::CMGFHBHAFFB::CMGFHBHAFFB>,
    // @@protoc_insertion_point(field:HINABMPDKKB.ECKFAMLGIBC)
    pub ECKFAMLGIBC: ::protobuf::EnumOrUnknown<super::GLBJJINGPOL::GLBJJINGPOL>,
    // @@protoc_insertion_point(field:HINABMPDKKB.KOPAAKCMKOI)
    pub KOPAAKCMKOI: ::std::vec::Vec<super::NPDFOHHEMBP::NPDFOHHEMBP>,
    // @@protoc_insertion_point(field:HINABMPDKKB.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // @@protoc_insertion_point(field:HINABMPDKKB.KJDOLBOBKJF)
    pub KJDOLBOBKJF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HINABMPDKKB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HINABMPDKKB {
    fn default() -> &'a HINABMPDKKB {
        <HINABMPDKKB as ::protobuf::Message>::default_instance()
    }
}

impl HINABMPDKKB {
    pub fn new() -> HINABMPDKKB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BCCGJIHNCDN",
            |m: &HINABMPDKKB| { &m.BCCGJIHNCDN },
            |m: &mut HINABMPDKKB| { &mut m.BCCGJIHNCDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ECKFAMLGIBC",
            |m: &HINABMPDKKB| { &m.ECKFAMLGIBC },
            |m: &mut HINABMPDKKB| { &mut m.ECKFAMLGIBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KOPAAKCMKOI",
            |m: &HINABMPDKKB| { &m.KOPAAKCMKOI },
            |m: &mut HINABMPDKKB| { &mut m.KOPAAKCMKOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &HINABMPDKKB| { &m.FJNHDHOHBCL },
            |m: &mut HINABMPDKKB| { &mut m.FJNHDHOHBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KJDOLBOBKJF",
            |m: &HINABMPDKKB| { &m.KJDOLBOBKJF },
            |m: &mut HINABMPDKKB| { &mut m.KJDOLBOBKJF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HINABMPDKKB>(
            "HINABMPDKKB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HINABMPDKKB {
    const NAME: &'static str = "HINABMPDKKB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.BCCGJIHNCDN.push(is.read_message()?);
                },
                120 => {
                    self.ECKFAMLGIBC = is.read_enum_or_unknown()?;
                },
                66 => {
                    self.KOPAAKCMKOI.push(is.read_message()?);
                },
                72 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
                },
                16 => {
                    self.KJDOLBOBKJF = is.read_uint32()?;
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
        for value in &self.BCCGJIHNCDN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ECKFAMLGIBC != ::protobuf::EnumOrUnknown::new(super::GLBJJINGPOL::GLBJJINGPOL::SCENE_GROUP_REFRESH_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.ECKFAMLGIBC.value());
        }
        for value in &self.KOPAAKCMKOI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FJNHDHOHBCL);
        }
        if self.KJDOLBOBKJF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.KJDOLBOBKJF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.BCCGJIHNCDN {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.ECKFAMLGIBC != ::protobuf::EnumOrUnknown::new(super::GLBJJINGPOL::GLBJJINGPOL::SCENE_GROUP_REFRESH_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.ECKFAMLGIBC))?;
        }
        for v in &self.KOPAAKCMKOI {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(9, self.FJNHDHOHBCL)?;
        }
        if self.KJDOLBOBKJF != 0 {
            os.write_uint32(2, self.KJDOLBOBKJF)?;
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

    fn new() -> HINABMPDKKB {
        HINABMPDKKB::new()
    }

    fn clear(&mut self) {
        self.BCCGJIHNCDN.clear();
        self.ECKFAMLGIBC = ::protobuf::EnumOrUnknown::new(super::GLBJJINGPOL::GLBJJINGPOL::SCENE_GROUP_REFRESH_TYPE_NONE);
        self.KOPAAKCMKOI.clear();
        self.FJNHDHOHBCL = 0;
        self.KJDOLBOBKJF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HINABMPDKKB {
        static instance: HINABMPDKKB = HINABMPDKKB {
            BCCGJIHNCDN: ::std::vec::Vec::new(),
            ECKFAMLGIBC: ::protobuf::EnumOrUnknown::from_i32(0),
            KOPAAKCMKOI: ::std::vec::Vec::new(),
            FJNHDHOHBCL: 0,
            KJDOLBOBKJF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HINABMPDKKB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HINABMPDKKB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HINABMPDKKB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HINABMPDKKB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HINABMPDKKB.proto\x1a\x11CMGFHBHAFFB.proto\x1a\x11GLBJJINGPOL.prot\
    o\x1a\x11NPDFOHHEMBP.proto\"\xe1\x01\n\x0bHINABMPDKKB\x12.\n\x0bBCCGJIHN\
    CDN\x18\x0e\x20\x03(\x0b2\x0c.CMGFHBHAFFBR\x0bBCCGJIHNCDN\x12.\n\x0bECKF\
    AMLGIBC\x18\x0f\x20\x01(\x0e2\x0c.GLBJJINGPOLR\x0bECKFAMLGIBC\x12.\n\x0b\
    KOPAAKCMKOI\x18\x08\x20\x03(\x0b2\x0c.NPDFOHHEMBPR\x0bKOPAAKCMKOI\x12\
    \x20\n\x0bFJNHDHOHBCL\x18\t\x20\x01(\rR\x0bFJNHDHOHBCL\x12\x20\n\x0bKJDO\
    LBOBKJF\x18\x02\x20\x01(\rR\x0bKJDOLBOBKJFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CMGFHBHAFFB::file_descriptor().clone());
            deps.push(super::GLBJJINGPOL::file_descriptor().clone());
            deps.push(super::NPDFOHHEMBP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HINABMPDKKB::generated_message_descriptor_data());
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
