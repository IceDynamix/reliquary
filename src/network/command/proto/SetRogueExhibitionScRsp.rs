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

//! Generated file from `SetRogueExhibitionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SetRogueExhibitionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetRogueExhibitionScRsp {
    // message fields
    // @@protoc_insertion_point(field:SetRogueExhibitionScRsp.PJPJOKKFNIM)
    pub PJPJOKKFNIM: ::std::vec::Vec<super::DMODINLGCCB::DMODINLGCCB>,
    // @@protoc_insertion_point(field:SetRogueExhibitionScRsp.MKCEFANCAIG)
    pub MKCEFANCAIG: ::std::vec::Vec<super::GBPFLAGFAIJ::GBPFLAGFAIJ>,
    // @@protoc_insertion_point(field:SetRogueExhibitionScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SetRogueExhibitionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetRogueExhibitionScRsp {
    fn default() -> &'a SetRogueExhibitionScRsp {
        <SetRogueExhibitionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SetRogueExhibitionScRsp {
    pub fn new() -> SetRogueExhibitionScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PJPJOKKFNIM",
            |m: &SetRogueExhibitionScRsp| { &m.PJPJOKKFNIM },
            |m: &mut SetRogueExhibitionScRsp| { &mut m.PJPJOKKFNIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MKCEFANCAIG",
            |m: &SetRogueExhibitionScRsp| { &m.MKCEFANCAIG },
            |m: &mut SetRogueExhibitionScRsp| { &mut m.MKCEFANCAIG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SetRogueExhibitionScRsp| { &m.retcode },
            |m: &mut SetRogueExhibitionScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetRogueExhibitionScRsp>(
            "SetRogueExhibitionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetRogueExhibitionScRsp {
    const NAME: &'static str = "SetRogueExhibitionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.PJPJOKKFNIM.push(is.read_message()?);
                },
                34 => {
                    self.MKCEFANCAIG.push(is.read_message()?);
                },
                104 => {
                    self.retcode = is.read_uint32()?;
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
        for value in &self.PJPJOKKFNIM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MKCEFANCAIG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PJPJOKKFNIM {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.MKCEFANCAIG {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(13, self.retcode)?;
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

    fn new() -> SetRogueExhibitionScRsp {
        SetRogueExhibitionScRsp::new()
    }

    fn clear(&mut self) {
        self.PJPJOKKFNIM.clear();
        self.MKCEFANCAIG.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetRogueExhibitionScRsp {
        static instance: SetRogueExhibitionScRsp = SetRogueExhibitionScRsp {
            PJPJOKKFNIM: ::std::vec::Vec::new(),
            MKCEFANCAIG: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetRogueExhibitionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetRogueExhibitionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetRogueExhibitionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetRogueExhibitionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dSetRogueExhibitionScRsp.proto\x1a\x11DMODINLGCCB.proto\x1a\x11GBPF\
    LAGFAIJ.proto\"\x93\x01\n\x17SetRogueExhibitionScRsp\x12.\n\x0bPJPJOKKFN\
    IM\x18\x0c\x20\x03(\x0b2\x0c.DMODINLGCCBR\x0bPJPJOKKFNIM\x12.\n\x0bMKCEF\
    ANCAIG\x18\x04\x20\x03(\x0b2\x0c.GBPFLAGFAIJR\x0bMKCEFANCAIG\x12\x18\n\
    \x07retcode\x18\r\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::DMODINLGCCB::file_descriptor().clone());
            deps.push(super::GBPFLAGFAIJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetRogueExhibitionScRsp::generated_message_descriptor_data());
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
