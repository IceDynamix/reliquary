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

//! Generated file from `OMJJAFALBLM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OMJJAFALBLM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OMJJAFALBLM {
    // message fields
    // @@protoc_insertion_point(field:OMJJAFALBLM.FAPLKPIBBGE)
    pub FAPLKPIBBGE: ::protobuf::MessageField<super::EGMKICMDEMB::EGMKICMDEMB>,
    // @@protoc_insertion_point(field:OMJJAFALBLM.KJCDHKMKOPK)
    pub KJCDHKMKOPK: ::std::vec::Vec<super::LFMIAHGBLHA::LFMIAHGBLHA>,
    // @@protoc_insertion_point(field:OMJJAFALBLM.OEJINGDKNND)
    pub OEJINGDKNND: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OMJJAFALBLM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OMJJAFALBLM {
    fn default() -> &'a OMJJAFALBLM {
        <OMJJAFALBLM as ::protobuf::Message>::default_instance()
    }
}

impl OMJJAFALBLM {
    pub fn new() -> OMJJAFALBLM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EGMKICMDEMB::EGMKICMDEMB>(
            "FAPLKPIBBGE",
            |m: &OMJJAFALBLM| { &m.FAPLKPIBBGE },
            |m: &mut OMJJAFALBLM| { &mut m.FAPLKPIBBGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KJCDHKMKOPK",
            |m: &OMJJAFALBLM| { &m.KJCDHKMKOPK },
            |m: &mut OMJJAFALBLM| { &mut m.KJCDHKMKOPK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OEJINGDKNND",
            |m: &OMJJAFALBLM| { &m.OEJINGDKNND },
            |m: &mut OMJJAFALBLM| { &mut m.OEJINGDKNND },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OMJJAFALBLM>(
            "OMJJAFALBLM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OMJJAFALBLM {
    const NAME: &'static str = "OMJJAFALBLM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FAPLKPIBBGE)?;
                },
                58 => {
                    self.KJCDHKMKOPK.push(is.read_message()?);
                },
                32 => {
                    self.OEJINGDKNND = is.read_uint32()?;
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
        if let Some(v) = self.FAPLKPIBBGE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.KJCDHKMKOPK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.OEJINGDKNND != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.OEJINGDKNND);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.FAPLKPIBBGE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        for v in &self.KJCDHKMKOPK {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.OEJINGDKNND != 0 {
            os.write_uint32(4, self.OEJINGDKNND)?;
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

    fn new() -> OMJJAFALBLM {
        OMJJAFALBLM::new()
    }

    fn clear(&mut self) {
        self.FAPLKPIBBGE.clear();
        self.KJCDHKMKOPK.clear();
        self.OEJINGDKNND = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OMJJAFALBLM {
        static instance: OMJJAFALBLM = OMJJAFALBLM {
            FAPLKPIBBGE: ::protobuf::MessageField::none(),
            KJCDHKMKOPK: ::std::vec::Vec::new(),
            OEJINGDKNND: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OMJJAFALBLM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OMJJAFALBLM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OMJJAFALBLM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OMJJAFALBLM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OMJJAFALBLM.proto\x1a\x11EGMKICMDEMB.proto\x1a\x11LFMIAHGBLHA.prot\
    o\"\x8f\x01\n\x0bOMJJAFALBLM\x12.\n\x0bFAPLKPIBBGE\x18\x08\x20\x01(\x0b2\
    \x0c.EGMKICMDEMBR\x0bFAPLKPIBBGE\x12.\n\x0bKJCDHKMKOPK\x18\x07\x20\x03(\
    \x0b2\x0c.LFMIAHGBLHAR\x0bKJCDHKMKOPK\x12\x20\n\x0bOEJINGDKNND\x18\x04\
    \x20\x01(\rR\x0bOEJINGDKNNDb\x06proto3\
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
            deps.push(super::EGMKICMDEMB::file_descriptor().clone());
            deps.push(super::LFMIAHGBLHA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OMJJAFALBLM::generated_message_descriptor_data());
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