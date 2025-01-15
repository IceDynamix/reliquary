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

//! Generated file from `UpdateMarkChestCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdateMarkChestCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateMarkChestCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateMarkChestCsReq.OMBECIMNGEE)
    pub OMBECIMNGEE: u32,
    // @@protoc_insertion_point(field:UpdateMarkChestCsReq.CJGMLCAKLFI)
    pub CJGMLCAKLFI: ::std::vec::Vec<super::KEKMJKOHBOH::KEKMJKOHBOH>,
    // @@protoc_insertion_point(field:UpdateMarkChestCsReq.CBAHFGPJEMP)
    pub CBAHFGPJEMP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateMarkChestCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateMarkChestCsReq {
    fn default() -> &'a UpdateMarkChestCsReq {
        <UpdateMarkChestCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateMarkChestCsReq {
    pub fn new() -> UpdateMarkChestCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMBECIMNGEE",
            |m: &UpdateMarkChestCsReq| { &m.OMBECIMNGEE },
            |m: &mut UpdateMarkChestCsReq| { &mut m.OMBECIMNGEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJGMLCAKLFI",
            |m: &UpdateMarkChestCsReq| { &m.CJGMLCAKLFI },
            |m: &mut UpdateMarkChestCsReq| { &mut m.CJGMLCAKLFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CBAHFGPJEMP",
            |m: &UpdateMarkChestCsReq| { &m.CBAHFGPJEMP },
            |m: &mut UpdateMarkChestCsReq| { &mut m.CBAHFGPJEMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateMarkChestCsReq>(
            "UpdateMarkChestCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateMarkChestCsReq {
    const NAME: &'static str = "UpdateMarkChestCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.OMBECIMNGEE = is.read_uint32()?;
                },
                66 => {
                    self.CJGMLCAKLFI.push(is.read_message()?);
                },
                120 => {
                    self.CBAHFGPJEMP = is.read_uint32()?;
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
        if self.OMBECIMNGEE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OMBECIMNGEE);
        }
        for value in &self.CJGMLCAKLFI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CBAHFGPJEMP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CBAHFGPJEMP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OMBECIMNGEE != 0 {
            os.write_uint32(10, self.OMBECIMNGEE)?;
        }
        for v in &self.CJGMLCAKLFI {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.CBAHFGPJEMP != 0 {
            os.write_uint32(15, self.CBAHFGPJEMP)?;
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

    fn new() -> UpdateMarkChestCsReq {
        UpdateMarkChestCsReq::new()
    }

    fn clear(&mut self) {
        self.OMBECIMNGEE = 0;
        self.CJGMLCAKLFI.clear();
        self.CBAHFGPJEMP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateMarkChestCsReq {
        static instance: UpdateMarkChestCsReq = UpdateMarkChestCsReq {
            OMBECIMNGEE: 0,
            CJGMLCAKLFI: ::std::vec::Vec::new(),
            CBAHFGPJEMP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateMarkChestCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateMarkChestCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateMarkChestCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateMarkChestCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aUpdateMarkChestCsReq.proto\x1a\x11KEKMJKOHBOH.proto\"\x8a\x01\n\
    \x14UpdateMarkChestCsReq\x12\x20\n\x0bOMBECIMNGEE\x18\n\x20\x01(\rR\x0bO\
    MBECIMNGEE\x12.\n\x0bCJGMLCAKLFI\x18\x08\x20\x03(\x0b2\x0c.KEKMJKOHBOHR\
    \x0bCJGMLCAKLFI\x12\x20\n\x0bCBAHFGPJEMP\x18\x0f\x20\x01(\rR\x0bCBAHFGPJ\
    EMPb\x06proto3\
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
            deps.push(super::KEKMJKOHBOH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateMarkChestCsReq::generated_message_descriptor_data());
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