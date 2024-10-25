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

//! Generated file from `GetSceneMapInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetSceneMapInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSceneMapInfoCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PFAIKMPEOCD)
    pub PFAIKMPEOCD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PBAMLLJIGEH)
    pub PBAMLLJIGEH: u32,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PNFPBICGDPC)
    pub PNFPBICGDPC: bool,
    // @@protoc_insertion_point(field:GetSceneMapInfoCsReq.PCABIMPKMBN)
    pub PCABIMPKMBN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetSceneMapInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSceneMapInfoCsReq {
    fn default() -> &'a GetSceneMapInfoCsReq {
        <GetSceneMapInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetSceneMapInfoCsReq {
    pub fn new() -> GetSceneMapInfoCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PFAIKMPEOCD",
            |m: &GetSceneMapInfoCsReq| { &m.PFAIKMPEOCD },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PFAIKMPEOCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBAMLLJIGEH",
            |m: &GetSceneMapInfoCsReq| { &m.PBAMLLJIGEH },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PBAMLLJIGEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNFPBICGDPC",
            |m: &GetSceneMapInfoCsReq| { &m.PNFPBICGDPC },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PNFPBICGDPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCABIMPKMBN",
            |m: &GetSceneMapInfoCsReq| { &m.PCABIMPKMBN },
            |m: &mut GetSceneMapInfoCsReq| { &mut m.PCABIMPKMBN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSceneMapInfoCsReq>(
            "GetSceneMapInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSceneMapInfoCsReq {
    const NAME: &'static str = "GetSceneMapInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.PFAIKMPEOCD)?;
                },
                120 => {
                    self.PFAIKMPEOCD.push(is.read_uint32()?);
                },
                32 => {
                    self.PBAMLLJIGEH = is.read_uint32()?;
                },
                104 => {
                    self.PNFPBICGDPC = is.read_bool()?;
                },
                112 => {
                    self.PCABIMPKMBN = is.read_uint32()?;
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
        for value in &self.PFAIKMPEOCD {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.PBAMLLJIGEH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.PBAMLLJIGEH);
        }
        if self.PNFPBICGDPC != false {
            my_size += 1 + 1;
        }
        if self.PCABIMPKMBN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PCABIMPKMBN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PFAIKMPEOCD {
            os.write_uint32(15, *v)?;
        };
        if self.PBAMLLJIGEH != 0 {
            os.write_uint32(4, self.PBAMLLJIGEH)?;
        }
        if self.PNFPBICGDPC != false {
            os.write_bool(13, self.PNFPBICGDPC)?;
        }
        if self.PCABIMPKMBN != 0 {
            os.write_uint32(14, self.PCABIMPKMBN)?;
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

    fn new() -> GetSceneMapInfoCsReq {
        GetSceneMapInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.PFAIKMPEOCD.clear();
        self.PBAMLLJIGEH = 0;
        self.PNFPBICGDPC = false;
        self.PCABIMPKMBN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSceneMapInfoCsReq {
        static instance: GetSceneMapInfoCsReq = GetSceneMapInfoCsReq {
            PFAIKMPEOCD: ::std::vec::Vec::new(),
            PBAMLLJIGEH: 0,
            PNFPBICGDPC: false,
            PCABIMPKMBN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSceneMapInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSceneMapInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSceneMapInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSceneMapInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetSceneMapInfoCsReq.proto\"\x9e\x01\n\x14GetSceneMapInfoCsReq\x12\
    \x20\n\x0bPFAIKMPEOCD\x18\x0f\x20\x03(\rR\x0bPFAIKMPEOCD\x12\x20\n\x0bPB\
    AMLLJIGEH\x18\x04\x20\x01(\rR\x0bPBAMLLJIGEH\x12\x20\n\x0bPNFPBICGDPC\
    \x18\r\x20\x01(\x08R\x0bPNFPBICGDPC\x12\x20\n\x0bPCABIMPKMBN\x18\x0e\x20\
    \x01(\rR\x0bPCABIMPKMBNb\x06proto3\
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
            messages.push(GetSceneMapInfoCsReq::generated_message_descriptor_data());
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
