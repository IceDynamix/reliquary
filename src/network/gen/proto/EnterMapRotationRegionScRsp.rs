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

//! Generated file from `EnterMapRotationRegionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterMapRotationRegionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterMapRotationRegionScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.FDGGLLHCPLI)
    pub FDGGLLHCPLI: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.LLLHPFLFKPP)
    pub LLLHPFLFKPP: u32,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.LNKKMEHBDPG)
    pub LNKKMEHBDPG: ::protobuf::MessageField<super::LDFPBJIHOPD::LDFPBJIHOPD>,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.KBMFLPNPKOJ)
    pub KBMFLPNPKOJ: ::protobuf::MessageField<super::LAMKDPDMLEC::LAMKDPDMLEC>,
    // @@protoc_insertion_point(field:EnterMapRotationRegionScRsp.IFDGJOJKBPN)
    pub IFDGJOJKBPN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterMapRotationRegionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterMapRotationRegionScRsp {
    fn default() -> &'a EnterMapRotationRegionScRsp {
        <EnterMapRotationRegionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnterMapRotationRegionScRsp {
    pub fn new() -> EnterMapRotationRegionScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDGGLLHCPLI",
            |m: &EnterMapRotationRegionScRsp| { &m.FDGGLLHCPLI },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.FDGGLLHCPLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EnterMapRotationRegionScRsp| { &m.retcode },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLLHPFLFKPP",
            |m: &EnterMapRotationRegionScRsp| { &m.LLLHPFLFKPP },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.LLLHPFLFKPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LDFPBJIHOPD::LDFPBJIHOPD>(
            "LNKKMEHBDPG",
            |m: &EnterMapRotationRegionScRsp| { &m.LNKKMEHBDPG },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.LNKKMEHBDPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LAMKDPDMLEC::LAMKDPDMLEC>(
            "KBMFLPNPKOJ",
            |m: &EnterMapRotationRegionScRsp| { &m.KBMFLPNPKOJ },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.KBMFLPNPKOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFDGJOJKBPN",
            |m: &EnterMapRotationRegionScRsp| { &m.IFDGJOJKBPN },
            |m: &mut EnterMapRotationRegionScRsp| { &mut m.IFDGJOJKBPN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterMapRotationRegionScRsp>(
            "EnterMapRotationRegionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterMapRotationRegionScRsp {
    const NAME: &'static str = "EnterMapRotationRegionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.FDGGLLHCPLI = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                40 => {
                    self.LLLHPFLFKPP = is.read_uint32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNKKMEHBDPG)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KBMFLPNPKOJ)?;
                },
                32 => {
                    self.IFDGJOJKBPN = is.read_uint32()?;
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
        if self.FDGGLLHCPLI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.FDGGLLHCPLI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.LLLHPFLFKPP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LLLHPFLFKPP);
        }
        if let Some(v) = self.LNKKMEHBDPG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KBMFLPNPKOJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IFDGJOJKBPN != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IFDGJOJKBPN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FDGGLLHCPLI != 0 {
            os.write_uint32(2, self.FDGGLLHCPLI)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.LLLHPFLFKPP != 0 {
            os.write_uint32(5, self.LLLHPFLFKPP)?;
        }
        if let Some(v) = self.LNKKMEHBDPG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.KBMFLPNPKOJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.IFDGJOJKBPN != 0 {
            os.write_uint32(4, self.IFDGJOJKBPN)?;
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

    fn new() -> EnterMapRotationRegionScRsp {
        EnterMapRotationRegionScRsp::new()
    }

    fn clear(&mut self) {
        self.FDGGLLHCPLI = 0;
        self.retcode = 0;
        self.LLLHPFLFKPP = 0;
        self.LNKKMEHBDPG.clear();
        self.KBMFLPNPKOJ.clear();
        self.IFDGJOJKBPN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterMapRotationRegionScRsp {
        static instance: EnterMapRotationRegionScRsp = EnterMapRotationRegionScRsp {
            FDGGLLHCPLI: 0,
            retcode: 0,
            LLLHPFLFKPP: 0,
            LNKKMEHBDPG: ::protobuf::MessageField::none(),
            KBMFLPNPKOJ: ::protobuf::MessageField::none(),
            IFDGJOJKBPN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterMapRotationRegionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterMapRotationRegionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterMapRotationRegionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterMapRotationRegionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!EnterMapRotationRegionScRsp.proto\x1a\x11LAMKDPDMLEC.proto\x1a\x11LDF\
    PBJIHOPD.proto\"\xfd\x01\n\x1bEnterMapRotationRegionScRsp\x12\x20\n\x0bF\
    DGGLLHCPLI\x18\x02\x20\x01(\rR\x0bFDGGLLHCPLI\x12\x18\n\x07retcode\x18\
    \x0c\x20\x01(\rR\x07retcode\x12\x20\n\x0bLLLHPFLFKPP\x18\x05\x20\x01(\rR\
    \x0bLLLHPFLFKPP\x12.\n\x0bLNKKMEHBDPG\x18\n\x20\x01(\x0b2\x0c.LDFPBJIHOP\
    DR\x0bLNKKMEHBDPG\x12.\n\x0bKBMFLPNPKOJ\x18\x0f\x20\x01(\x0b2\x0c.LAMKDP\
    DMLECR\x0bKBMFLPNPKOJ\x12\x20\n\x0bIFDGJOJKBPN\x18\x04\x20\x01(\rR\x0bIF\
    DGJOJKBPNb\x06proto3\
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
            deps.push(super::LAMKDPDMLEC::file_descriptor().clone());
            deps.push(super::LDFPBJIHOPD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterMapRotationRegionScRsp::generated_message_descriptor_data());
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
