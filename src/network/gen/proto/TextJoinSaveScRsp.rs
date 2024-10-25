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

//! Generated file from `TextJoinSaveScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TextJoinSaveScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TextJoinSaveScRsp {
    // message fields
    // @@protoc_insertion_point(field:TextJoinSaveScRsp.AINAOENMOME)
    pub AINAOENMOME: ::std::string::String,
    // @@protoc_insertion_point(field:TextJoinSaveScRsp.OCIJJOLCNBE)
    pub OCIJJOLCNBE: u32,
    // @@protoc_insertion_point(field:TextJoinSaveScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TextJoinSaveScRsp.PPPJDALAKIG)
    pub PPPJDALAKIG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TextJoinSaveScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TextJoinSaveScRsp {
    fn default() -> &'a TextJoinSaveScRsp {
        <TextJoinSaveScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TextJoinSaveScRsp {
    pub fn new() -> TextJoinSaveScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AINAOENMOME",
            |m: &TextJoinSaveScRsp| { &m.AINAOENMOME },
            |m: &mut TextJoinSaveScRsp| { &mut m.AINAOENMOME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OCIJJOLCNBE",
            |m: &TextJoinSaveScRsp| { &m.OCIJJOLCNBE },
            |m: &mut TextJoinSaveScRsp| { &mut m.OCIJJOLCNBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TextJoinSaveScRsp| { &m.retcode },
            |m: &mut TextJoinSaveScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPPJDALAKIG",
            |m: &TextJoinSaveScRsp| { &m.PPPJDALAKIG },
            |m: &mut TextJoinSaveScRsp| { &mut m.PPPJDALAKIG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TextJoinSaveScRsp>(
            "TextJoinSaveScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TextJoinSaveScRsp {
    const NAME: &'static str = "TextJoinSaveScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.AINAOENMOME = is.read_string()?;
                },
                80 => {
                    self.OCIJJOLCNBE = is.read_uint32()?;
                },
                112 => {
                    self.retcode = is.read_uint32()?;
                },
                48 => {
                    self.PPPJDALAKIG = is.read_uint32()?;
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
        if !self.AINAOENMOME.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.AINAOENMOME);
        }
        if self.OCIJJOLCNBE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OCIJJOLCNBE);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.retcode);
        }
        if self.PPPJDALAKIG != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.PPPJDALAKIG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.AINAOENMOME.is_empty() {
            os.write_string(3, &self.AINAOENMOME)?;
        }
        if self.OCIJJOLCNBE != 0 {
            os.write_uint32(10, self.OCIJJOLCNBE)?;
        }
        if self.retcode != 0 {
            os.write_uint32(14, self.retcode)?;
        }
        if self.PPPJDALAKIG != 0 {
            os.write_uint32(6, self.PPPJDALAKIG)?;
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

    fn new() -> TextJoinSaveScRsp {
        TextJoinSaveScRsp::new()
    }

    fn clear(&mut self) {
        self.AINAOENMOME.clear();
        self.OCIJJOLCNBE = 0;
        self.retcode = 0;
        self.PPPJDALAKIG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TextJoinSaveScRsp {
        static instance: TextJoinSaveScRsp = TextJoinSaveScRsp {
            AINAOENMOME: ::std::string::String::new(),
            OCIJJOLCNBE: 0,
            retcode: 0,
            PPPJDALAKIG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TextJoinSaveScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TextJoinSaveScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TextJoinSaveScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TextJoinSaveScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17TextJoinSaveScRsp.proto\"\x93\x01\n\x11TextJoinSaveScRsp\x12\x20\n\
    \x0bAINAOENMOME\x18\x03\x20\x01(\tR\x0bAINAOENMOME\x12\x20\n\x0bOCIJJOLC\
    NBE\x18\n\x20\x01(\rR\x0bOCIJJOLCNBE\x12\x18\n\x07retcode\x18\x0e\x20\
    \x01(\rR\x07retcode\x12\x20\n\x0bPPPJDALAKIG\x18\x06\x20\x01(\rR\x0bPPPJ\
    DALAKIGb\x06proto3\
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
            messages.push(TextJoinSaveScRsp::generated_message_descriptor_data());
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
