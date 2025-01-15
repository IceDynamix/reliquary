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

//! Generated file from `FFOOKABHHFF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FFOOKABHHFF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FFOOKABHHFF {
    // message fields
    // @@protoc_insertion_point(field:FFOOKABHHFF.PBIGGPACCPB)
    pub PBIGGPACCPB: u32,
    // @@protoc_insertion_point(field:FFOOKABHHFF.IHACFPLKGHP)
    pub IHACFPLKGHP: ::protobuf::MessageField<super::NFIHEGJPADD::NFIHEGJPADD>,
    // special fields
    // @@protoc_insertion_point(special_field:FFOOKABHHFF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FFOOKABHHFF {
    fn default() -> &'a FFOOKABHHFF {
        <FFOOKABHHFF as ::protobuf::Message>::default_instance()
    }
}

impl FFOOKABHHFF {
    pub fn new() -> FFOOKABHHFF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBIGGPACCPB",
            |m: &FFOOKABHHFF| { &m.PBIGGPACCPB },
            |m: &mut FFOOKABHHFF| { &mut m.PBIGGPACCPB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NFIHEGJPADD::NFIHEGJPADD>(
            "IHACFPLKGHP",
            |m: &FFOOKABHHFF| { &m.IHACFPLKGHP },
            |m: &mut FFOOKABHHFF| { &mut m.IHACFPLKGHP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FFOOKABHHFF>(
            "FFOOKABHHFF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FFOOKABHHFF {
    const NAME: &'static str = "FFOOKABHHFF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.PBIGGPACCPB = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IHACFPLKGHP)?;
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
        if self.PBIGGPACCPB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.PBIGGPACCPB);
        }
        if let Some(v) = self.IHACFPLKGHP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PBIGGPACCPB != 0 {
            os.write_uint32(12, self.PBIGGPACCPB)?;
        }
        if let Some(v) = self.IHACFPLKGHP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> FFOOKABHHFF {
        FFOOKABHHFF::new()
    }

    fn clear(&mut self) {
        self.PBIGGPACCPB = 0;
        self.IHACFPLKGHP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FFOOKABHHFF {
        static instance: FFOOKABHHFF = FFOOKABHHFF {
            PBIGGPACCPB: 0,
            IHACFPLKGHP: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FFOOKABHHFF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FFOOKABHHFF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FFOOKABHHFF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FFOOKABHHFF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FFOOKABHHFF.proto\x1a\x11NFIHEGJPADD.proto\"_\n\x0bFFOOKABHHFF\x12\
    \x20\n\x0bPBIGGPACCPB\x18\x0c\x20\x01(\rR\x0bPBIGGPACCPB\x12.\n\x0bIHACF\
    PLKGHP\x18\x04\x20\x01(\x0b2\x0c.NFIHEGJPADDR\x0bIHACFPLKGHPb\x06proto3\
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
            deps.push(super::NFIHEGJPADD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FFOOKABHHFF::generated_message_descriptor_data());
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