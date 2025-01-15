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

//! Generated file from `NNPAOMCBANA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NNPAOMCBANA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NNPAOMCBANA {
    // message oneof groups
    pub KKHCANEDOIM: ::std::option::Option<nnpaomcbana::KKHCANEDOIM>,
    // special fields
    // @@protoc_insertion_point(special_field:NNPAOMCBANA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NNPAOMCBANA {
    fn default() -> &'a NNPAOMCBANA {
        <NNPAOMCBANA as ::protobuf::Message>::default_instance()
    }
}

impl NNPAOMCBANA {
    pub fn new() -> NNPAOMCBANA {
        ::std::default::Default::default()
    }

    // .FCGKPCKCJEH GLDGCNMLODL = 7;

    pub fn GLDGCNMLODL(&self) -> &super::FCGKPCKCJEH::FCGKPCKCJEH {
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(ref v)) => v,
            _ => <super::FCGKPCKCJEH::FCGKPCKCJEH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GLDGCNMLODL(&mut self) {
        self.KKHCANEDOIM = ::std::option::Option::None;
    }

    pub fn has_GLDGCNMLODL(&self) -> bool {
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GLDGCNMLODL(&mut self, v: super::FCGKPCKCJEH::FCGKPCKCJEH) {
        self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GLDGCNMLODL(&mut self) -> &mut super::FCGKPCKCJEH::FCGKPCKCJEH {
        if let ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(_)) = self.KKHCANEDOIM {
        } else {
            self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(super::FCGKPCKCJEH::FCGKPCKCJEH::new()));
        }
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GLDGCNMLODL(&mut self) -> super::FCGKPCKCJEH::FCGKPCKCJEH {
        if self.has_GLDGCNMLODL() {
            match self.KKHCANEDOIM.take() {
                ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FCGKPCKCJEH::FCGKPCKCJEH::new()
        }
    }

    // .GIHFGGMCHKA JDBFOCMJMJD = 9;

    pub fn JDBFOCMJMJD(&self) -> &super::GIHFGGMCHKA::GIHFGGMCHKA {
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(ref v)) => v,
            _ => <super::GIHFGGMCHKA::GIHFGGMCHKA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JDBFOCMJMJD(&mut self) {
        self.KKHCANEDOIM = ::std::option::Option::None;
    }

    pub fn has_JDBFOCMJMJD(&self) -> bool {
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JDBFOCMJMJD(&mut self, v: super::GIHFGGMCHKA::GIHFGGMCHKA) {
        self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JDBFOCMJMJD(&mut self) -> &mut super::GIHFGGMCHKA::GIHFGGMCHKA {
        if let ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(_)) = self.KKHCANEDOIM {
        } else {
            self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(super::GIHFGGMCHKA::GIHFGGMCHKA::new()));
        }
        match self.KKHCANEDOIM {
            ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JDBFOCMJMJD(&mut self) -> super::GIHFGGMCHKA::GIHFGGMCHKA {
        if self.has_JDBFOCMJMJD() {
            match self.KKHCANEDOIM.take() {
                ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GIHFGGMCHKA::GIHFGGMCHKA::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FCGKPCKCJEH::FCGKPCKCJEH>(
            "GLDGCNMLODL",
            NNPAOMCBANA::has_GLDGCNMLODL,
            NNPAOMCBANA::GLDGCNMLODL,
            NNPAOMCBANA::mut_GLDGCNMLODL,
            NNPAOMCBANA::set_GLDGCNMLODL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GIHFGGMCHKA::GIHFGGMCHKA>(
            "JDBFOCMJMJD",
            NNPAOMCBANA::has_JDBFOCMJMJD,
            NNPAOMCBANA::JDBFOCMJMJD,
            NNPAOMCBANA::mut_JDBFOCMJMJD,
            NNPAOMCBANA::set_JDBFOCMJMJD,
        ));
        oneofs.push(nnpaomcbana::KKHCANEDOIM::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NNPAOMCBANA>(
            "NNPAOMCBANA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NNPAOMCBANA {
    const NAME: &'static str = "NNPAOMCBANA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(is.read_message()?));
                },
                74 => {
                    self.KKHCANEDOIM = ::std::option::Option::Some(nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.KKHCANEDOIM {
            match v {
                &nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.KKHCANEDOIM {
            match v {
                &nnpaomcbana::KKHCANEDOIM::GLDGCNMLODL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
                &nnpaomcbana::KKHCANEDOIM::JDBFOCMJMJD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> NNPAOMCBANA {
        NNPAOMCBANA::new()
    }

    fn clear(&mut self) {
        self.KKHCANEDOIM = ::std::option::Option::None;
        self.KKHCANEDOIM = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NNPAOMCBANA {
        static instance: NNPAOMCBANA = NNPAOMCBANA {
            KKHCANEDOIM: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NNPAOMCBANA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NNPAOMCBANA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NNPAOMCBANA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NNPAOMCBANA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `NNPAOMCBANA`
pub mod nnpaomcbana {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:NNPAOMCBANA.KKHCANEDOIM)
    pub enum KKHCANEDOIM {
        // @@protoc_insertion_point(oneof_field:NNPAOMCBANA.GLDGCNMLODL)
        GLDGCNMLODL(super::super::FCGKPCKCJEH::FCGKPCKCJEH),
        // @@protoc_insertion_point(oneof_field:NNPAOMCBANA.JDBFOCMJMJD)
        JDBFOCMJMJD(super::super::GIHFGGMCHKA::GIHFGGMCHKA),
    }

    impl ::protobuf::Oneof for KKHCANEDOIM {
    }

    impl ::protobuf::OneofFull for KKHCANEDOIM {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::NNPAOMCBANA as ::protobuf::MessageFull>::descriptor().oneof_by_name("KKHCANEDOIM").unwrap()).clone()
        }
    }

    impl KKHCANEDOIM {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KKHCANEDOIM>("KKHCANEDOIM")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NNPAOMCBANA.proto\x1a\x11FCGKPCKCJEH.proto\x1a\x11GIHFGGMCHKA.prot\
    o\"\x80\x01\n\x0bNNPAOMCBANA\x120\n\x0bGLDGCNMLODL\x18\x07\x20\x01(\x0b2\
    \x0c.FCGKPCKCJEHH\0R\x0bGLDGCNMLODL\x120\n\x0bJDBFOCMJMJD\x18\t\x20\x01(\
    \x0b2\x0c.GIHFGGMCHKAH\0R\x0bJDBFOCMJMJDB\r\n\x0bKKHCANEDOIMb\x06proto3\
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
            deps.push(super::FCGKPCKCJEH::file_descriptor().clone());
            deps.push(super::GIHFGGMCHKA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NNPAOMCBANA::generated_message_descriptor_data());
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