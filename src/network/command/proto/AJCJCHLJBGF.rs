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

//! Generated file from `AJCJCHLJBGF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AJCJCHLJBGF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AJCJCHLJBGF {
    // message fields
    // @@protoc_insertion_point(field:AJCJCHLJBGF.item_list)
    pub item_list: ::std::vec::Vec<super::CEODDCEIDDL::CEODDCEIDDL>,
    // @@protoc_insertion_point(field:AJCJCHLJBGF.PCAJNCBMIMM)
    pub PCAJNCBMIMM: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:AJCJCHLJBGF.item_value)
    pub item_value: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // special fields
    // @@protoc_insertion_point(special_field:AJCJCHLJBGF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AJCJCHLJBGF {
    fn default() -> &'a AJCJCHLJBGF {
        <AJCJCHLJBGF as ::protobuf::Message>::default_instance()
    }
}

impl AJCJCHLJBGF {
    pub fn new() -> AJCJCHLJBGF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_list",
            |m: &AJCJCHLJBGF| { &m.item_list },
            |m: &mut AJCJCHLJBGF| { &mut m.item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "PCAJNCBMIMM",
            |m: &AJCJCHLJBGF| { &m.PCAJNCBMIMM },
            |m: &mut AJCJCHLJBGF| { &mut m.PCAJNCBMIMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "item_value",
            |m: &AJCJCHLJBGF| { &m.item_value },
            |m: &mut AJCJCHLJBGF| { &mut m.item_value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AJCJCHLJBGF>(
            "AJCJCHLJBGF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AJCJCHLJBGF {
    const NAME: &'static str = "AJCJCHLJBGF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    self.item_list.push(is.read_message()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PCAJNCBMIMM)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.item_value)?;
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
        for value in &self.item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.PCAJNCBMIMM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.item_value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.item_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if let Some(v) = self.PCAJNCBMIMM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.item_value.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> AJCJCHLJBGF {
        AJCJCHLJBGF::new()
    }

    fn clear(&mut self) {
        self.item_list.clear();
        self.PCAJNCBMIMM.clear();
        self.item_value.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AJCJCHLJBGF {
        static instance: AJCJCHLJBGF = AJCJCHLJBGF {
            item_list: ::std::vec::Vec::new(),
            PCAJNCBMIMM: ::protobuf::MessageField::none(),
            item_value: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AJCJCHLJBGF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AJCJCHLJBGF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AJCJCHLJBGF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AJCJCHLJBGF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AJCJCHLJBGF.proto\x1a\x11CEODDCEIDDL.proto\x1a\x11IIKNGNHDMFI.prot\
    o\"\x95\x01\n\x0bAJCJCHLJBGF\x12)\n\titem_list\x18\x04\x20\x03(\x0b2\x0c\
    .CEODDCEIDDLR\x08itemList\x12.\n\x0bPCAJNCBMIMM\x18\x06\x20\x01(\x0b2\
    \x0c.IIKNGNHDMFIR\x0bPCAJNCBMIMM\x12+\n\nitem_value\x18\x07\x20\x01(\x0b\
    2\x0c.IIKNGNHDMFIR\titemValueb\x06proto3\
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
            deps.push(super::CEODDCEIDDL::file_descriptor().clone());
            deps.push(super::IIKNGNHDMFI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AJCJCHLJBGF::generated_message_descriptor_data());
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
