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

//! Generated file from `SpaceZooBornScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SpaceZooBornScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SpaceZooBornScRsp {
    // message fields
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.KOPMCFPAPEL)
    pub KOPMCFPAPEL: ::std::vec::Vec<super::KJEPLFPALCB::KJEPLFPALCB>,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.NEPLPODCHPE)
    pub NEPLPODCHPE: bool,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SpaceZooBornScRsp.OFEIKONFEIF)
    pub OFEIKONFEIF: ::protobuf::MessageField<super::DGDPOCNFNBM::DGDPOCNFNBM>,
    // special fields
    // @@protoc_insertion_point(special_field:SpaceZooBornScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SpaceZooBornScRsp {
    fn default() -> &'a SpaceZooBornScRsp {
        <SpaceZooBornScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SpaceZooBornScRsp {
    pub fn new() -> SpaceZooBornScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KOPMCFPAPEL",
            |m: &SpaceZooBornScRsp| { &m.KOPMCFPAPEL },
            |m: &mut SpaceZooBornScRsp| { &mut m.KOPMCFPAPEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEPLPODCHPE",
            |m: &SpaceZooBornScRsp| { &m.NEPLPODCHPE },
            |m: &mut SpaceZooBornScRsp| { &mut m.NEPLPODCHPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SpaceZooBornScRsp| { &m.retcode },
            |m: &mut SpaceZooBornScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DGDPOCNFNBM::DGDPOCNFNBM>(
            "OFEIKONFEIF",
            |m: &SpaceZooBornScRsp| { &m.OFEIKONFEIF },
            |m: &mut SpaceZooBornScRsp| { &mut m.OFEIKONFEIF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SpaceZooBornScRsp>(
            "SpaceZooBornScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SpaceZooBornScRsp {
    const NAME: &'static str = "SpaceZooBornScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.KOPMCFPAPEL.push(is.read_message()?);
                },
                80 => {
                    self.NEPLPODCHPE = is.read_bool()?;
                },
                24 => {
                    self.retcode = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OFEIKONFEIF)?;
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
        for value in &self.KOPMCFPAPEL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NEPLPODCHPE != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        if let Some(v) = self.OFEIKONFEIF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KOPMCFPAPEL {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.NEPLPODCHPE != false {
            os.write_bool(10, self.NEPLPODCHPE)?;
        }
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
        }
        if let Some(v) = self.OFEIKONFEIF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> SpaceZooBornScRsp {
        SpaceZooBornScRsp::new()
    }

    fn clear(&mut self) {
        self.KOPMCFPAPEL.clear();
        self.NEPLPODCHPE = false;
        self.retcode = 0;
        self.OFEIKONFEIF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SpaceZooBornScRsp {
        static instance: SpaceZooBornScRsp = SpaceZooBornScRsp {
            KOPMCFPAPEL: ::std::vec::Vec::new(),
            NEPLPODCHPE: false,
            retcode: 0,
            OFEIKONFEIF: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SpaceZooBornScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SpaceZooBornScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SpaceZooBornScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpaceZooBornScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17SpaceZooBornScRsp.proto\x1a\x11DGDPOCNFNBM.proto\x1a\x11KJEPLFPALC\
    B.proto\"\xaf\x01\n\x11SpaceZooBornScRsp\x12.\n\x0bKOPMCFPAPEL\x18\x08\
    \x20\x03(\x0b2\x0c.KJEPLFPALCBR\x0bKOPMCFPAPEL\x12\x20\n\x0bNEPLPODCHPE\
    \x18\n\x20\x01(\x08R\x0bNEPLPODCHPE\x12\x18\n\x07retcode\x18\x03\x20\x01\
    (\rR\x07retcode\x12.\n\x0bOFEIKONFEIF\x18\x01\x20\x01(\x0b2\x0c.DGDPOCNF\
    NBMR\x0bOFEIKONFEIFb\x06proto3\
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
            deps.push(super::DGDPOCNFNBM::file_descriptor().clone());
            deps.push(super::KJEPLFPALCB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SpaceZooBornScRsp::generated_message_descriptor_data());
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
