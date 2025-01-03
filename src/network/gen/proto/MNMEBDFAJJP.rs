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

//! Generated file from `MNMEBDFAJJP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MNMEBDFAJJP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MNMEBDFAJJP {
    // message oneof groups
    pub LIOKBKHBEKL: ::std::option::Option<mnmebdfajjp::LIOKBKHBEKL>,
    // special fields
    // @@protoc_insertion_point(special_field:MNMEBDFAJJP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MNMEBDFAJJP {
    fn default() -> &'a MNMEBDFAJJP {
        <MNMEBDFAJJP as ::protobuf::Message>::default_instance()
    }
}

impl MNMEBDFAJJP {
    pub fn new() -> MNMEBDFAJJP {
        ::std::default::Default::default()
    }

    // .NNBCDCCANHI IKAJCNAJHFG = 101;

    pub fn IKAJCNAJHFG(&self) -> &super::NNBCDCCANHI::NNBCDCCANHI {
        match self.LIOKBKHBEKL {
            ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(ref v)) => v,
            _ => <super::NNBCDCCANHI::NNBCDCCANHI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IKAJCNAJHFG(&mut self) {
        self.LIOKBKHBEKL = ::std::option::Option::None;
    }

    pub fn has_IKAJCNAJHFG(&self) -> bool {
        match self.LIOKBKHBEKL {
            ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IKAJCNAJHFG(&mut self, v: super::NNBCDCCANHI::NNBCDCCANHI) {
        self.LIOKBKHBEKL = ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IKAJCNAJHFG(&mut self) -> &mut super::NNBCDCCANHI::NNBCDCCANHI {
        if let ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(_)) = self.LIOKBKHBEKL {
        } else {
            self.LIOKBKHBEKL = ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(super::NNBCDCCANHI::NNBCDCCANHI::new()));
        }
        match self.LIOKBKHBEKL {
            ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IKAJCNAJHFG(&mut self) -> super::NNBCDCCANHI::NNBCDCCANHI {
        if self.has_IKAJCNAJHFG() {
            match self.LIOKBKHBEKL.take() {
                ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::NNBCDCCANHI::NNBCDCCANHI::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::NNBCDCCANHI::NNBCDCCANHI>(
            "IKAJCNAJHFG",
            MNMEBDFAJJP::has_IKAJCNAJHFG,
            MNMEBDFAJJP::IKAJCNAJHFG,
            MNMEBDFAJJP::mut_IKAJCNAJHFG,
            MNMEBDFAJJP::set_IKAJCNAJHFG,
        ));
        oneofs.push(mnmebdfajjp::LIOKBKHBEKL::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MNMEBDFAJJP>(
            "MNMEBDFAJJP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MNMEBDFAJJP {
    const NAME: &'static str = "MNMEBDFAJJP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                810 => {
                    self.LIOKBKHBEKL = ::std::option::Option::Some(mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.LIOKBKHBEKL {
            match v {
                &mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.LIOKBKHBEKL {
            match v {
                &mnmebdfajjp::LIOKBKHBEKL::IKAJCNAJHFG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(101, v, os)?;
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

    fn new() -> MNMEBDFAJJP {
        MNMEBDFAJJP::new()
    }

    fn clear(&mut self) {
        self.LIOKBKHBEKL = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MNMEBDFAJJP {
        static instance: MNMEBDFAJJP = MNMEBDFAJJP {
            LIOKBKHBEKL: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MNMEBDFAJJP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MNMEBDFAJJP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MNMEBDFAJJP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MNMEBDFAJJP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MNMEBDFAJJP`
pub mod mnmebdfajjp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:MNMEBDFAJJP.LIOKBKHBEKL)
    pub enum LIOKBKHBEKL {
        // @@protoc_insertion_point(oneof_field:MNMEBDFAJJP.IKAJCNAJHFG)
        IKAJCNAJHFG(super::super::NNBCDCCANHI::NNBCDCCANHI),
    }

    impl ::protobuf::Oneof for LIOKBKHBEKL {
    }

    impl ::protobuf::OneofFull for LIOKBKHBEKL {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::MNMEBDFAJJP as ::protobuf::MessageFull>::descriptor().oneof_by_name("LIOKBKHBEKL").unwrap()).clone()
        }
    }

    impl LIOKBKHBEKL {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<LIOKBKHBEKL>("LIOKBKHBEKL")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MNMEBDFAJJP.proto\x1a\x11NNBCDCCANHI.proto\"N\n\x0bMNMEBDFAJJP\x12\
    0\n\x0bIKAJCNAJHFG\x18e\x20\x01(\x0b2\x0c.NNBCDCCANHIH\0R\x0bIKAJCNAJHFG\
    B\r\n\x0bLIOKBKHBEKLb\x06proto3\
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
            deps.push(super::NNBCDCCANHI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MNMEBDFAJJP::generated_message_descriptor_data());
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
