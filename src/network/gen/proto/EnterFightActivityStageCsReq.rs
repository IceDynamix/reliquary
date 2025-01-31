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

//! Generated file from `EnterFightActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterFightActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterFightActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.OCDLCGIILKI)
    pub OCDLCGIILKI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.IBAFDOBBEGD)
    pub IBAFDOBBEGD: u32,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.LPDMJMCOCKK)
    pub LPDMJMCOCKK: ::std::vec::Vec<super::HGNKIOALCBH::HGNKIOALCBH>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterFightActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterFightActivityStageCsReq {
    fn default() -> &'a EnterFightActivityStageCsReq {
        <EnterFightActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterFightActivityStageCsReq {
    pub fn new() -> EnterFightActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OCDLCGIILKI",
            |m: &EnterFightActivityStageCsReq| { &m.OCDLCGIILKI },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.OCDLCGIILKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EnterFightActivityStageCsReq| { &m.avatar_list },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBAFDOBBEGD",
            |m: &EnterFightActivityStageCsReq| { &m.IBAFDOBBEGD },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.IBAFDOBBEGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LPDMJMCOCKK",
            |m: &EnterFightActivityStageCsReq| { &m.LPDMJMCOCKK },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.LPDMJMCOCKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &EnterFightActivityStageCsReq| { &m.IOPPGEGDHGL },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.IOPPGEGDHGL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterFightActivityStageCsReq>(
            "EnterFightActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterFightActivityStageCsReq {
    const NAME: &'static str = "EnterFightActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.OCDLCGIILKI)?;
                },
                80 => {
                    self.OCDLCGIILKI.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_list)?;
                },
                56 => {
                    self.avatar_list.push(is.read_uint32()?);
                },
                32 => {
                    self.IBAFDOBBEGD = is.read_uint32()?;
                },
                42 => {
                    self.LPDMJMCOCKK.push(is.read_message()?);
                },
                104 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
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
        for value in &self.OCDLCGIILKI {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.avatar_list {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        if self.IBAFDOBBEGD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IBAFDOBBEGD);
        }
        for value in &self.LPDMJMCOCKK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.IOPPGEGDHGL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.OCDLCGIILKI {
            os.write_uint32(10, *v)?;
        };
        for v in &self.avatar_list {
            os.write_uint32(7, *v)?;
        };
        if self.IBAFDOBBEGD != 0 {
            os.write_uint32(4, self.IBAFDOBBEGD)?;
        }
        for v in &self.LPDMJMCOCKK {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(13, self.IOPPGEGDHGL)?;
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

    fn new() -> EnterFightActivityStageCsReq {
        EnterFightActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.OCDLCGIILKI.clear();
        self.avatar_list.clear();
        self.IBAFDOBBEGD = 0;
        self.LPDMJMCOCKK.clear();
        self.IOPPGEGDHGL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterFightActivityStageCsReq {
        static instance: EnterFightActivityStageCsReq = EnterFightActivityStageCsReq {
            OCDLCGIILKI: ::std::vec::Vec::new(),
            avatar_list: ::std::vec::Vec::new(),
            IBAFDOBBEGD: 0,
            LPDMJMCOCKK: ::std::vec::Vec::new(),
            IOPPGEGDHGL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterFightActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterFightActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterFightActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterFightActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"EnterFightActivityStageCsReq.proto\x1a\x11HGNKIOALCBH.proto\"\xd5\
    \x01\n\x1cEnterFightActivityStageCsReq\x12\x20\n\x0bOCDLCGIILKI\x18\n\
    \x20\x03(\rR\x0bOCDLCGIILKI\x12\x1f\n\x0bavatar_list\x18\x07\x20\x03(\rR\
    \navatarList\x12\x20\n\x0bIBAFDOBBEGD\x18\x04\x20\x01(\rR\x0bIBAFDOBBEGD\
    \x12.\n\x0bLPDMJMCOCKK\x18\x05\x20\x03(\x0b2\x0c.HGNKIOALCBHR\x0bLPDMJMC\
    OCKK\x12\x20\n\x0bIOPPGEGDHGL\x18\r\x20\x01(\rR\x0bIOPPGEGDHGLb\x06proto\
    3\
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
            deps.push(super::HGNKIOALCBH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterFightActivityStageCsReq::generated_message_descriptor_data());
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
