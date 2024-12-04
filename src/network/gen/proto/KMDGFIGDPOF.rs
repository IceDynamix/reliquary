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

//! Generated file from `KMDGFIGDPOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KMDGFIGDPOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KMDGFIGDPOF {
    // message fields
    // @@protoc_insertion_point(field:KMDGFIGDPOF.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::NBCCNCBKADP::NBCCNCBKADP>,
    // @@protoc_insertion_point(field:KMDGFIGDPOF.IGODAFNLDCK)
    pub IGODAFNLDCK: u32,
    // @@protoc_insertion_point(field:KMDGFIGDPOF.NAEMGNJPKEG)
    pub NAEMGNJPKEG: ::std::vec::Vec<super::NEGIFANELHM::NEGIFANELHM>,
    // @@protoc_insertion_point(field:KMDGFIGDPOF.CLLGFFCFBPC)
    pub CLLGFFCFBPC: u32,
    // @@protoc_insertion_point(field:KMDGFIGDPOF.EBKNMDMKOAA)
    pub EBKNMDMKOAA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KMDGFIGDPOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KMDGFIGDPOF {
    fn default() -> &'a KMDGFIGDPOF {
        <KMDGFIGDPOF as ::protobuf::Message>::default_instance()
    }
}

impl KMDGFIGDPOF {
    pub fn new() -> KMDGFIGDPOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &KMDGFIGDPOF| { &m.OJBAILGKLBM },
            |m: &mut KMDGFIGDPOF| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGODAFNLDCK",
            |m: &KMDGFIGDPOF| { &m.IGODAFNLDCK },
            |m: &mut KMDGFIGDPOF| { &mut m.IGODAFNLDCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NAEMGNJPKEG",
            |m: &KMDGFIGDPOF| { &m.NAEMGNJPKEG },
            |m: &mut KMDGFIGDPOF| { &mut m.NAEMGNJPKEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLLGFFCFBPC",
            |m: &KMDGFIGDPOF| { &m.CLLGFFCFBPC },
            |m: &mut KMDGFIGDPOF| { &mut m.CLLGFFCFBPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBKNMDMKOAA",
            |m: &KMDGFIGDPOF| { &m.EBKNMDMKOAA },
            |m: &mut KMDGFIGDPOF| { &mut m.EBKNMDMKOAA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KMDGFIGDPOF>(
            "KMDGFIGDPOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KMDGFIGDPOF {
    const NAME: &'static str = "KMDGFIGDPOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.IGODAFNLDCK = is.read_uint32()?;
                },
                26 => {
                    self.NAEMGNJPKEG.push(is.read_message()?);
                },
                32 => {
                    self.CLLGFFCFBPC = is.read_uint32()?;
                },
                8 => {
                    self.EBKNMDMKOAA = is.read_uint32()?;
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
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::NBCCNCBKADP::NBCCNCBKADP::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.OJBAILGKLBM.value());
        }
        if self.IGODAFNLDCK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IGODAFNLDCK);
        }
        for value in &self.NAEMGNJPKEG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CLLGFFCFBPC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.CLLGFFCFBPC);
        }
        if self.EBKNMDMKOAA != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.EBKNMDMKOAA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::NBCCNCBKADP::NBCCNCBKADP::ROGUE_MAGIC_LAYER_STATUS_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
        }
        if self.IGODAFNLDCK != 0 {
            os.write_uint32(7, self.IGODAFNLDCK)?;
        }
        for v in &self.NAEMGNJPKEG {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.CLLGFFCFBPC != 0 {
            os.write_uint32(4, self.CLLGFFCFBPC)?;
        }
        if self.EBKNMDMKOAA != 0 {
            os.write_uint32(1, self.EBKNMDMKOAA)?;
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

    fn new() -> KMDGFIGDPOF {
        KMDGFIGDPOF::new()
    }

    fn clear(&mut self) {
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::NBCCNCBKADP::NBCCNCBKADP::ROGUE_MAGIC_LAYER_STATUS_NONE);
        self.IGODAFNLDCK = 0;
        self.NAEMGNJPKEG.clear();
        self.CLLGFFCFBPC = 0;
        self.EBKNMDMKOAA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KMDGFIGDPOF {
        static instance: KMDGFIGDPOF = KMDGFIGDPOF {
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            IGODAFNLDCK: 0,
            NAEMGNJPKEG: ::std::vec::Vec::new(),
            CLLGFFCFBPC: 0,
            EBKNMDMKOAA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KMDGFIGDPOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KMDGFIGDPOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KMDGFIGDPOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KMDGFIGDPOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KMDGFIGDPOF.proto\x1a\x11NBCCNCBKADP.proto\x1a\x11NEGIFANELHM.prot\
    o\"\xd3\x01\n\x0bKMDGFIGDPOF\x12.\n\x0bOJBAILGKLBM\x18\r\x20\x01(\x0e2\
    \x0c.NBCCNCBKADPR\x0bOJBAILGKLBM\x12\x20\n\x0bIGODAFNLDCK\x18\x07\x20\
    \x01(\rR\x0bIGODAFNLDCK\x12.\n\x0bNAEMGNJPKEG\x18\x03\x20\x03(\x0b2\x0c.\
    NEGIFANELHMR\x0bNAEMGNJPKEG\x12\x20\n\x0bCLLGFFCFBPC\x18\x04\x20\x01(\rR\
    \x0bCLLGFFCFBPC\x12\x20\n\x0bEBKNMDMKOAA\x18\x01\x20\x01(\rR\x0bEBKNMDMK\
    OAAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::NBCCNCBKADP::file_descriptor().clone());
            deps.push(super::NEGIFANELHM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KMDGFIGDPOF::generated_message_descriptor_data());
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