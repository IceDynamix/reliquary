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

//! Generated file from `GetChapterScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetChapterScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChapterScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChapterScRsp.MBIICLDDNFF)
    pub MBIICLDDNFF: ::std::vec::Vec<super::FBMELLHHLMJ::FBMELLHHLMJ>,
    // @@protoc_insertion_point(field:GetChapterScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetChapterScRsp.FALDHGHICGI)
    pub FALDHGHICGI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetChapterScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChapterScRsp {
    fn default() -> &'a GetChapterScRsp {
        <GetChapterScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChapterScRsp {
    pub fn new() -> GetChapterScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MBIICLDDNFF",
            |m: &GetChapterScRsp| { &m.MBIICLDDNFF },
            |m: &mut GetChapterScRsp| { &mut m.MBIICLDDNFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChapterScRsp| { &m.retcode },
            |m: &mut GetChapterScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FALDHGHICGI",
            |m: &GetChapterScRsp| { &m.FALDHGHICGI },
            |m: &mut GetChapterScRsp| { &mut m.FALDHGHICGI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChapterScRsp>(
            "GetChapterScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChapterScRsp {
    const NAME: &'static str = "GetChapterScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.MBIICLDDNFF.push(is.read_message()?);
                },
                40 => {
                    self.retcode = is.read_uint32()?;
                },
                48 => {
                    self.FALDHGHICGI = is.read_uint32()?;
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
        for value in &self.MBIICLDDNFF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        if self.FALDHGHICGI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FALDHGHICGI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MBIICLDDNFF {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
        }
        if self.FALDHGHICGI != 0 {
            os.write_uint32(6, self.FALDHGHICGI)?;
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

    fn new() -> GetChapterScRsp {
        GetChapterScRsp::new()
    }

    fn clear(&mut self) {
        self.MBIICLDDNFF.clear();
        self.retcode = 0;
        self.FALDHGHICGI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChapterScRsp {
        static instance: GetChapterScRsp = GetChapterScRsp {
            MBIICLDDNFF: ::std::vec::Vec::new(),
            retcode: 0,
            FALDHGHICGI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChapterScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChapterScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChapterScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChapterScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15GetChapterScRsp.proto\x1a\x11FBMELLHHLMJ.proto\"}\n\x0fGetChapterS\
    cRsp\x12.\n\x0bMBIICLDDNFF\x18\r\x20\x03(\x0b2\x0c.FBMELLHHLMJR\x0bMBIIC\
    LDDNFF\x12\x18\n\x07retcode\x18\x05\x20\x01(\rR\x07retcode\x12\x20\n\x0b\
    FALDHGHICGI\x18\x06\x20\x01(\rR\x0bFALDHGHICGIb\x06proto3\
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
            deps.push(super::FBMELLHHLMJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChapterScRsp::generated_message_descriptor_data());
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
