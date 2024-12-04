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

//! Generated file from `OEMIPBDLKKD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OEMIPBDLKKD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OEMIPBDLKKD {
    // message fields
    // @@protoc_insertion_point(field:OEMIPBDLKKD.PPNKHDECJBP)
    pub PPNKHDECJBP: ::std::vec::Vec<super::GFIHCDPGHMA::GFIHCDPGHMA>,
    // @@protoc_insertion_point(field:OEMIPBDLKKD.EPIAILLKNCI)
    pub EPIAILLKNCI: ::protobuf::MessageField<super::AOBGKPKLNPO::AOBGKPKLNPO>,
    // special fields
    // @@protoc_insertion_point(special_field:OEMIPBDLKKD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OEMIPBDLKKD {
    fn default() -> &'a OEMIPBDLKKD {
        <OEMIPBDLKKD as ::protobuf::Message>::default_instance()
    }
}

impl OEMIPBDLKKD {
    pub fn new() -> OEMIPBDLKKD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PPNKHDECJBP",
            |m: &OEMIPBDLKKD| { &m.PPNKHDECJBP },
            |m: &mut OEMIPBDLKKD| { &mut m.PPNKHDECJBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AOBGKPKLNPO::AOBGKPKLNPO>(
            "EPIAILLKNCI",
            |m: &OEMIPBDLKKD| { &m.EPIAILLKNCI },
            |m: &mut OEMIPBDLKKD| { &mut m.EPIAILLKNCI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OEMIPBDLKKD>(
            "OEMIPBDLKKD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OEMIPBDLKKD {
    const NAME: &'static str = "OEMIPBDLKKD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.PPNKHDECJBP.push(is.read_message()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EPIAILLKNCI)?;
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
        for value in &self.PPNKHDECJBP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.EPIAILLKNCI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PPNKHDECJBP {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if let Some(v) = self.EPIAILLKNCI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> OEMIPBDLKKD {
        OEMIPBDLKKD::new()
    }

    fn clear(&mut self) {
        self.PPNKHDECJBP.clear();
        self.EPIAILLKNCI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OEMIPBDLKKD {
        static instance: OEMIPBDLKKD = OEMIPBDLKKD {
            PPNKHDECJBP: ::std::vec::Vec::new(),
            EPIAILLKNCI: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OEMIPBDLKKD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OEMIPBDLKKD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OEMIPBDLKKD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OEMIPBDLKKD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OEMIPBDLKKD.proto\x1a\x11AOBGKPKLNPO.proto\x1a\x11GFIHCDPGHMA.prot\
    o\"m\n\x0bOEMIPBDLKKD\x12.\n\x0bPPNKHDECJBP\x18\x06\x20\x03(\x0b2\x0c.GF\
    IHCDPGHMAR\x0bPPNKHDECJBP\x12.\n\x0bEPIAILLKNCI\x18\x05\x20\x01(\x0b2\
    \x0c.AOBGKPKLNPOR\x0bEPIAILLKNCIb\x06proto3\
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
            deps.push(super::AOBGKPKLNPO::file_descriptor().clone());
            deps.push(super::GFIHCDPGHMA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OEMIPBDLKKD::generated_message_descriptor_data());
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