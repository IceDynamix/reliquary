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

//! Generated file from `LMFPBBGNLIO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LMFPBBGNLIO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LMFPBBGNLIO {
    // message oneof groups
    pub EFJNCDLNEPA: ::std::option::Option<lmfpbbgnlio::EFJNCDLNEPA>,
    // special fields
    // @@protoc_insertion_point(special_field:LMFPBBGNLIO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LMFPBBGNLIO {
    fn default() -> &'a LMFPBBGNLIO {
        <LMFPBBGNLIO as ::protobuf::Message>::default_instance()
    }
}

impl LMFPBBGNLIO {
    pub fn new() -> LMFPBBGNLIO {
        ::std::default::Default::default()
    }

    // .NGKLIKHIOPB OLLODGFKOCO = 11;

    pub fn OLLODGFKOCO(&self) -> &super::NGKLIKHIOPB::NGKLIKHIOPB {
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(ref v)) => v,
            _ => <super::NGKLIKHIOPB::NGKLIKHIOPB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_OLLODGFKOCO(&mut self) {
        self.EFJNCDLNEPA = ::std::option::Option::None;
    }

    pub fn has_OLLODGFKOCO(&self) -> bool {
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_OLLODGFKOCO(&mut self, v: super::NGKLIKHIOPB::NGKLIKHIOPB) {
        self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_OLLODGFKOCO(&mut self) -> &mut super::NGKLIKHIOPB::NGKLIKHIOPB {
        if let ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(_)) = self.EFJNCDLNEPA {
        } else {
            self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(super::NGKLIKHIOPB::NGKLIKHIOPB::new()));
        }
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_OLLODGFKOCO(&mut self) -> super::NGKLIKHIOPB::NGKLIKHIOPB {
        if self.has_OLLODGFKOCO() {
            match self.EFJNCDLNEPA.take() {
                ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NGKLIKHIOPB::NGKLIKHIOPB::new()
        }
    }

    // .KPODNODIPII NGEKGDBHALB = 8;

    pub fn NGEKGDBHALB(&self) -> &super::KPODNODIPII::KPODNODIPII {
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(ref v)) => v,
            _ => <super::KPODNODIPII::KPODNODIPII as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NGEKGDBHALB(&mut self) {
        self.EFJNCDLNEPA = ::std::option::Option::None;
    }

    pub fn has_NGEKGDBHALB(&self) -> bool {
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NGEKGDBHALB(&mut self, v: super::KPODNODIPII::KPODNODIPII) {
        self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NGEKGDBHALB(&mut self) -> &mut super::KPODNODIPII::KPODNODIPII {
        if let ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(_)) = self.EFJNCDLNEPA {
        } else {
            self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(super::KPODNODIPII::KPODNODIPII::new()));
        }
        match self.EFJNCDLNEPA {
            ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NGEKGDBHALB(&mut self) -> super::KPODNODIPII::KPODNODIPII {
        if self.has_NGEKGDBHALB() {
            match self.EFJNCDLNEPA.take() {
                ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KPODNODIPII::KPODNODIPII::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NGKLIKHIOPB::NGKLIKHIOPB>(
            "OLLODGFKOCO",
            LMFPBBGNLIO::has_OLLODGFKOCO,
            LMFPBBGNLIO::OLLODGFKOCO,
            LMFPBBGNLIO::mut_OLLODGFKOCO,
            LMFPBBGNLIO::set_OLLODGFKOCO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KPODNODIPII::KPODNODIPII>(
            "NGEKGDBHALB",
            LMFPBBGNLIO::has_NGEKGDBHALB,
            LMFPBBGNLIO::NGEKGDBHALB,
            LMFPBBGNLIO::mut_NGEKGDBHALB,
            LMFPBBGNLIO::set_NGEKGDBHALB,
        ));
        oneofs.push(lmfpbbgnlio::EFJNCDLNEPA::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LMFPBBGNLIO>(
            "LMFPBBGNLIO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LMFPBBGNLIO {
    const NAME: &'static str = "LMFPBBGNLIO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(is.read_message()?));
                },
                66 => {
                    self.EFJNCDLNEPA = ::std::option::Option::Some(lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.EFJNCDLNEPA {
            match v {
                &lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.EFJNCDLNEPA {
            match v {
                &lmfpbbgnlio::EFJNCDLNEPA::OLLODGFKOCO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
                },
                &lmfpbbgnlio::EFJNCDLNEPA::NGEKGDBHALB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
            };
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

    fn new() -> LMFPBBGNLIO {
        LMFPBBGNLIO::new()
    }

    fn clear(&mut self) {
        self.EFJNCDLNEPA = ::std::option::Option::None;
        self.EFJNCDLNEPA = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LMFPBBGNLIO {
        static instance: LMFPBBGNLIO = LMFPBBGNLIO {
            EFJNCDLNEPA: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LMFPBBGNLIO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LMFPBBGNLIO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LMFPBBGNLIO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LMFPBBGNLIO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LMFPBBGNLIO`
pub mod lmfpbbgnlio {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:LMFPBBGNLIO.EFJNCDLNEPA)
    pub enum EFJNCDLNEPA {
        // @@protoc_insertion_point(oneof_field:LMFPBBGNLIO.OLLODGFKOCO)
        OLLODGFKOCO(super::super::NGKLIKHIOPB::NGKLIKHIOPB),
        // @@protoc_insertion_point(oneof_field:LMFPBBGNLIO.NGEKGDBHALB)
        NGEKGDBHALB(super::super::KPODNODIPII::KPODNODIPII),
    }

    impl ::protobuf::Oneof for EFJNCDLNEPA {
    }

    impl ::protobuf::OneofFull for EFJNCDLNEPA {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LMFPBBGNLIO as ::protobuf::MessageFull>::descriptor().oneof_by_name("EFJNCDLNEPA").unwrap()).clone()
        }
    }

    impl EFJNCDLNEPA {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<EFJNCDLNEPA>("EFJNCDLNEPA")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LMFPBBGNLIO.proto\x1a\x11KPODNODIPII.proto\x1a\x11NGKLIKHIOPB.prot\
    o\"\x80\x01\n\x0bLMFPBBGNLIO\x120\n\x0bOLLODGFKOCO\x18\x0b\x20\x01(\x0b2\
    \x0c.NGKLIKHIOPBH\0R\x0bOLLODGFKOCO\x120\n\x0bNGEKGDBHALB\x18\x08\x20\
    \x01(\x0b2\x0c.KPODNODIPIIH\0R\x0bNGEKGDBHALBB\r\n\x0bEFJNCDLNEPAb\x06pr\
    oto3\
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
            deps.push(super::KPODNODIPII::file_descriptor().clone());
            deps.push(super::NGKLIKHIOPB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LMFPBBGNLIO::generated_message_descriptor_data());
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