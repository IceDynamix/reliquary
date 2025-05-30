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

//! Generated file from `PEDLPHDBNAF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PEDLPHDBNAF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PEDLPHDBNAF {
    // message oneof groups
    pub EFKCNLGOFBN: ::std::option::Option<pedlphdbnaf::EFKCNLGOFBN>,
    // special fields
    // @@protoc_insertion_point(special_field:PEDLPHDBNAF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PEDLPHDBNAF {
    fn default() -> &'a PEDLPHDBNAF {
        <PEDLPHDBNAF as ::protobuf::Message>::default_instance()
    }
}

impl PEDLPHDBNAF {
    pub fn new() -> PEDLPHDBNAF {
        ::std::default::Default::default()
    }

    // .ILDHFMHBKNC LFCPHAJCEKF = 101;

    pub fn LFCPHAJCEKF(&self) -> &super::ILDHFMHBKNC::ILDHFMHBKNC {
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(ref v)) => v,
            _ => <super::ILDHFMHBKNC::ILDHFMHBKNC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LFCPHAJCEKF(&mut self) {
        self.EFKCNLGOFBN = ::std::option::Option::None;
    }

    pub fn has_LFCPHAJCEKF(&self) -> bool {
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LFCPHAJCEKF(&mut self, v: super::ILDHFMHBKNC::ILDHFMHBKNC) {
        self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LFCPHAJCEKF(&mut self) -> &mut super::ILDHFMHBKNC::ILDHFMHBKNC {
        if let ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(_)) = self.EFKCNLGOFBN {
        } else {
            self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(super::ILDHFMHBKNC::ILDHFMHBKNC::new()));
        }
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LFCPHAJCEKF(&mut self) -> super::ILDHFMHBKNC::ILDHFMHBKNC {
        if self.has_LFCPHAJCEKF() {
            match self.EFKCNLGOFBN.take() {
                ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ILDHFMHBKNC::ILDHFMHBKNC::new()
        }
    }

    // .LKAPFHAHNEM CELMKOLBJNN = 102;

    pub fn CELMKOLBJNN(&self) -> &super::LKAPFHAHNEM::LKAPFHAHNEM {
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(ref v)) => v,
            _ => <super::LKAPFHAHNEM::LKAPFHAHNEM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_CELMKOLBJNN(&mut self) {
        self.EFKCNLGOFBN = ::std::option::Option::None;
    }

    pub fn has_CELMKOLBJNN(&self) -> bool {
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_CELMKOLBJNN(&mut self, v: super::LKAPFHAHNEM::LKAPFHAHNEM) {
        self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(v))
    }

    // Mutable pointer to the field.
    pub fn mut_CELMKOLBJNN(&mut self) -> &mut super::LKAPFHAHNEM::LKAPFHAHNEM {
        if let ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(_)) = self.EFKCNLGOFBN {
        } else {
            self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(super::LKAPFHAHNEM::LKAPFHAHNEM::new()));
        }
        match self.EFKCNLGOFBN {
            ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_CELMKOLBJNN(&mut self) -> super::LKAPFHAHNEM::LKAPFHAHNEM {
        if self.has_CELMKOLBJNN() {
            match self.EFKCNLGOFBN.take() {
                ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LKAPFHAHNEM::LKAPFHAHNEM::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ILDHFMHBKNC::ILDHFMHBKNC>(
            "LFCPHAJCEKF",
            PEDLPHDBNAF::has_LFCPHAJCEKF,
            PEDLPHDBNAF::LFCPHAJCEKF,
            PEDLPHDBNAF::mut_LFCPHAJCEKF,
            PEDLPHDBNAF::set_LFCPHAJCEKF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LKAPFHAHNEM::LKAPFHAHNEM>(
            "CELMKOLBJNN",
            PEDLPHDBNAF::has_CELMKOLBJNN,
            PEDLPHDBNAF::CELMKOLBJNN,
            PEDLPHDBNAF::mut_CELMKOLBJNN,
            PEDLPHDBNAF::set_CELMKOLBJNN,
        ));
        oneofs.push(pedlphdbnaf::EFKCNLGOFBN::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PEDLPHDBNAF>(
            "PEDLPHDBNAF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PEDLPHDBNAF {
    const NAME: &'static str = "PEDLPHDBNAF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                810 => {
                    self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(is.read_message()?));
                },
                818 => {
                    self.EFKCNLGOFBN = ::std::option::Option::Some(pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.EFKCNLGOFBN {
            match v {
                &pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.EFKCNLGOFBN {
            match v {
                &pedlphdbnaf::EFKCNLGOFBN::LFCPHAJCEKF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(101, v, os)?;
                },
                &pedlphdbnaf::EFKCNLGOFBN::CELMKOLBJNN(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(102, v, os)?;
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

    fn new() -> PEDLPHDBNAF {
        PEDLPHDBNAF::new()
    }

    fn clear(&mut self) {
        self.EFKCNLGOFBN = ::std::option::Option::None;
        self.EFKCNLGOFBN = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PEDLPHDBNAF {
        static instance: PEDLPHDBNAF = PEDLPHDBNAF {
            EFKCNLGOFBN: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PEDLPHDBNAF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PEDLPHDBNAF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PEDLPHDBNAF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PEDLPHDBNAF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PEDLPHDBNAF`
pub mod pedlphdbnaf {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PEDLPHDBNAF.EFKCNLGOFBN)
    pub enum EFKCNLGOFBN {
        // @@protoc_insertion_point(oneof_field:PEDLPHDBNAF.LFCPHAJCEKF)
        LFCPHAJCEKF(super::super::ILDHFMHBKNC::ILDHFMHBKNC),
        // @@protoc_insertion_point(oneof_field:PEDLPHDBNAF.CELMKOLBJNN)
        CELMKOLBJNN(super::super::LKAPFHAHNEM::LKAPFHAHNEM),
    }

    impl ::protobuf::Oneof for EFKCNLGOFBN {
    }

    impl ::protobuf::OneofFull for EFKCNLGOFBN {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PEDLPHDBNAF as ::protobuf::MessageFull>::descriptor().oneof_by_name("EFKCNLGOFBN").unwrap()).clone()
        }
    }

    impl EFKCNLGOFBN {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<EFKCNLGOFBN>("EFKCNLGOFBN")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PEDLPHDBNAF.proto\x1a\x11ILDHFMHBKNC.proto\x1a\x11LKAPFHAHNEM.prot\
    o\"\x80\x01\n\x0bPEDLPHDBNAF\x120\n\x0bLFCPHAJCEKF\x18e\x20\x01(\x0b2\
    \x0c.ILDHFMHBKNCH\0R\x0bLFCPHAJCEKF\x120\n\x0bCELMKOLBJNN\x18f\x20\x01(\
    \x0b2\x0c.LKAPFHAHNEMH\0R\x0bCELMKOLBJNNB\r\n\x0bEFKCNLGOFBNb\x06proto3\
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
            deps.push(super::ILDHFMHBKNC::file_descriptor().clone());
            deps.push(super::LKAPFHAHNEM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PEDLPHDBNAF::generated_message_descriptor_data());
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
