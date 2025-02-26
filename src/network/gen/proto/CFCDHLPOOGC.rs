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

//! Generated file from `CFCDHLPOOGC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CFCDHLPOOGC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CFCDHLPOOGC {
    // message fields
    // @@protoc_insertion_point(field:CFCDHLPOOGC.IKOBKINKHCF)
    pub IKOBKINKHCF: ::protobuf::MessageField<super::MIKFNBBOPIP::MIKFNBBOPIP>,
    // message oneof groups
    pub KFELKJLDKEH: ::std::option::Option<cfcdhlpoogc::KFELKJLDKEH>,
    // special fields
    // @@protoc_insertion_point(special_field:CFCDHLPOOGC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CFCDHLPOOGC {
    fn default() -> &'a CFCDHLPOOGC {
        <CFCDHLPOOGC as ::protobuf::Message>::default_instance()
    }
}

impl CFCDHLPOOGC {
    pub fn new() -> CFCDHLPOOGC {
        ::std::default::Default::default()
    }

    // .LHADMKCGCKO ICFFJLICIMB = 298;

    pub fn ICFFJLICIMB(&self) -> &super::LHADMKCGCKO::LHADMKCGCKO {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(ref v)) => v,
            _ => <super::LHADMKCGCKO::LHADMKCGCKO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ICFFJLICIMB(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_ICFFJLICIMB(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ICFFJLICIMB(&mut self, v: super::LHADMKCGCKO::LHADMKCGCKO) {
        self.KFELKJLDKEH = ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ICFFJLICIMB(&mut self) -> &mut super::LHADMKCGCKO::LHADMKCGCKO {
        if let ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(_)) = self.KFELKJLDKEH {
        } else {
            self.KFELKJLDKEH = ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(super::LHADMKCGCKO::LHADMKCGCKO::new()));
        }
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ICFFJLICIMB(&mut self) -> super::LHADMKCGCKO::LHADMKCGCKO {
        if self.has_ICFFJLICIMB() {
            match self.KFELKJLDKEH.take() {
                ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LHADMKCGCKO::LHADMKCGCKO::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MIKFNBBOPIP::MIKFNBBOPIP>(
            "IKOBKINKHCF",
            |m: &CFCDHLPOOGC| { &m.IKOBKINKHCF },
            |m: &mut CFCDHLPOOGC| { &mut m.IKOBKINKHCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LHADMKCGCKO::LHADMKCGCKO>(
            "ICFFJLICIMB",
            CFCDHLPOOGC::has_ICFFJLICIMB,
            CFCDHLPOOGC::ICFFJLICIMB,
            CFCDHLPOOGC::mut_ICFFJLICIMB,
            CFCDHLPOOGC::set_ICFFJLICIMB,
        ));
        oneofs.push(cfcdhlpoogc::KFELKJLDKEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CFCDHLPOOGC>(
            "CFCDHLPOOGC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CFCDHLPOOGC {
    const NAME: &'static str = "CFCDHLPOOGC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IKOBKINKHCF)?;
                },
                2386 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(is.read_message()?));
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
        if let Some(v) = self.IKOBKINKHCF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.IKOBKINKHCF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &cfcdhlpoogc::KFELKJLDKEH::ICFFJLICIMB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(298, v, os)?;
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

    fn new() -> CFCDHLPOOGC {
        CFCDHLPOOGC::new()
    }

    fn clear(&mut self) {
        self.IKOBKINKHCF.clear();
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CFCDHLPOOGC {
        static instance: CFCDHLPOOGC = CFCDHLPOOGC {
            IKOBKINKHCF: ::protobuf::MessageField::none(),
            KFELKJLDKEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CFCDHLPOOGC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CFCDHLPOOGC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CFCDHLPOOGC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CFCDHLPOOGC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CFCDHLPOOGC`
pub mod cfcdhlpoogc {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CFCDHLPOOGC.KFELKJLDKEH)
    pub enum KFELKJLDKEH {
        // @@protoc_insertion_point(oneof_field:CFCDHLPOOGC.ICFFJLICIMB)
        ICFFJLICIMB(super::super::LHADMKCGCKO::LHADMKCGCKO),
    }

    impl ::protobuf::Oneof for KFELKJLDKEH {
    }

    impl ::protobuf::OneofFull for KFELKJLDKEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CFCDHLPOOGC as ::protobuf::MessageFull>::descriptor().oneof_by_name("KFELKJLDKEH").unwrap()).clone()
        }
    }

    impl KFELKJLDKEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KFELKJLDKEH>("KFELKJLDKEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CFCDHLPOOGC.proto\x1a\x11LHADMKCGCKO.proto\x1a\x11MIKFNBBOPIP.prot\
    o\"\x7f\n\x0bCFCDHLPOOGC\x12.\n\x0bIKOBKINKHCF\x18\x0c\x20\x01(\x0b2\x0c\
    .MIKFNBBOPIPR\x0bIKOBKINKHCF\x121\n\x0bICFFJLICIMB\x18\xaa\x02\x20\x01(\
    \x0b2\x0c.LHADMKCGCKOH\0R\x0bICFFJLICIMBB\r\n\x0bKFELKJLDKEHb\x06proto3\
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
            deps.push(super::LHADMKCGCKO::file_descriptor().clone());
            deps.push(super::MIKFNBBOPIP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CFCDHLPOOGC::generated_message_descriptor_data());
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
