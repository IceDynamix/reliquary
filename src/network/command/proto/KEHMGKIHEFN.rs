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

//! Generated file from `KEHMGKIHEFN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KEHMGKIHEFN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KEHMGKIHEFN {
    // message oneof groups
    pub GLMPLDBLMEC: ::std::option::Option<kehmgkihefn::GLMPLDBLMEC>,
    // special fields
    // @@protoc_insertion_point(special_field:KEHMGKIHEFN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KEHMGKIHEFN {
    fn default() -> &'a KEHMGKIHEFN {
        <KEHMGKIHEFN as ::protobuf::Message>::default_instance()
    }
}

impl KEHMGKIHEFN {
    pub fn new() -> KEHMGKIHEFN {
        ::std::default::Default::default()
    }

    // .FCNOLLFGPCK GIEIDJEEPAC = 15;

    pub fn GIEIDJEEPAC(&self) -> &super::FCNOLLFGPCK::FCNOLLFGPCK {
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(ref v)) => v,
            _ => <super::FCNOLLFGPCK::FCNOLLFGPCK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GIEIDJEEPAC(&mut self) {
        self.GLMPLDBLMEC = ::std::option::Option::None;
    }

    pub fn has_GIEIDJEEPAC(&self) -> bool {
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GIEIDJEEPAC(&mut self, v: super::FCNOLLFGPCK::FCNOLLFGPCK) {
        self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GIEIDJEEPAC(&mut self) -> &mut super::FCNOLLFGPCK::FCNOLLFGPCK {
        if let ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(_)) = self.GLMPLDBLMEC {
        } else {
            self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(super::FCNOLLFGPCK::FCNOLLFGPCK::new()));
        }
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GIEIDJEEPAC(&mut self) -> super::FCNOLLFGPCK::FCNOLLFGPCK {
        if self.has_GIEIDJEEPAC() {
            match self.GLMPLDBLMEC.take() {
                ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FCNOLLFGPCK::FCNOLLFGPCK::new()
        }
    }

    // .KAMCIOPBPGA ADDCJEJPFEF = 7;

    pub fn ADDCJEJPFEF(&self) -> &super::KAMCIOPBPGA::KAMCIOPBPGA {
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(ref v)) => v,
            _ => <super::KAMCIOPBPGA::KAMCIOPBPGA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ADDCJEJPFEF(&mut self) {
        self.GLMPLDBLMEC = ::std::option::Option::None;
    }

    pub fn has_ADDCJEJPFEF(&self) -> bool {
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ADDCJEJPFEF(&mut self, v: super::KAMCIOPBPGA::KAMCIOPBPGA) {
        self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ADDCJEJPFEF(&mut self) -> &mut super::KAMCIOPBPGA::KAMCIOPBPGA {
        if let ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(_)) = self.GLMPLDBLMEC {
        } else {
            self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(super::KAMCIOPBPGA::KAMCIOPBPGA::new()));
        }
        match self.GLMPLDBLMEC {
            ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ADDCJEJPFEF(&mut self) -> super::KAMCIOPBPGA::KAMCIOPBPGA {
        if self.has_ADDCJEJPFEF() {
            match self.GLMPLDBLMEC.take() {
                ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KAMCIOPBPGA::KAMCIOPBPGA::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FCNOLLFGPCK::FCNOLLFGPCK>(
            "GIEIDJEEPAC",
            KEHMGKIHEFN::has_GIEIDJEEPAC,
            KEHMGKIHEFN::GIEIDJEEPAC,
            KEHMGKIHEFN::mut_GIEIDJEEPAC,
            KEHMGKIHEFN::set_GIEIDJEEPAC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KAMCIOPBPGA::KAMCIOPBPGA>(
            "ADDCJEJPFEF",
            KEHMGKIHEFN::has_ADDCJEJPFEF,
            KEHMGKIHEFN::ADDCJEJPFEF,
            KEHMGKIHEFN::mut_ADDCJEJPFEF,
            KEHMGKIHEFN::set_ADDCJEJPFEF,
        ));
        oneofs.push(kehmgkihefn::GLMPLDBLMEC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KEHMGKIHEFN>(
            "KEHMGKIHEFN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KEHMGKIHEFN {
    const NAME: &'static str = "KEHMGKIHEFN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(is.read_message()?));
                },
                58 => {
                    self.GLMPLDBLMEC = ::std::option::Option::Some(kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.GLMPLDBLMEC {
            match v {
                &kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.GLMPLDBLMEC {
            match v {
                &kehmgkihefn::GLMPLDBLMEC::GIEIDJEEPAC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &kehmgkihefn::GLMPLDBLMEC::ADDCJEJPFEF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> KEHMGKIHEFN {
        KEHMGKIHEFN::new()
    }

    fn clear(&mut self) {
        self.GLMPLDBLMEC = ::std::option::Option::None;
        self.GLMPLDBLMEC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KEHMGKIHEFN {
        static instance: KEHMGKIHEFN = KEHMGKIHEFN {
            GLMPLDBLMEC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KEHMGKIHEFN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KEHMGKIHEFN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KEHMGKIHEFN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KEHMGKIHEFN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `KEHMGKIHEFN`
pub mod kehmgkihefn {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:KEHMGKIHEFN.GLMPLDBLMEC)
    pub enum GLMPLDBLMEC {
        // @@protoc_insertion_point(oneof_field:KEHMGKIHEFN.GIEIDJEEPAC)
        GIEIDJEEPAC(super::super::FCNOLLFGPCK::FCNOLLFGPCK),
        // @@protoc_insertion_point(oneof_field:KEHMGKIHEFN.ADDCJEJPFEF)
        ADDCJEJPFEF(super::super::KAMCIOPBPGA::KAMCIOPBPGA),
    }

    impl ::protobuf::Oneof for GLMPLDBLMEC {
    }

    impl ::protobuf::OneofFull for GLMPLDBLMEC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::KEHMGKIHEFN as ::protobuf::MessageFull>::descriptor().oneof_by_name("GLMPLDBLMEC").unwrap()).clone()
        }
    }

    impl GLMPLDBLMEC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<GLMPLDBLMEC>("GLMPLDBLMEC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KEHMGKIHEFN.proto\x1a\x11FCNOLLFGPCK.proto\x1a\x11KAMCIOPBPGA.prot\
    o\"\x80\x01\n\x0bKEHMGKIHEFN\x120\n\x0bGIEIDJEEPAC\x18\x0f\x20\x01(\x0b2\
    \x0c.FCNOLLFGPCKH\0R\x0bGIEIDJEEPAC\x120\n\x0bADDCJEJPFEF\x18\x07\x20\
    \x01(\x0b2\x0c.KAMCIOPBPGAH\0R\x0bADDCJEJPFEFB\r\n\x0bGLMPLDBLMECb\x06pr\
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
            deps.push(super::FCNOLLFGPCK::file_descriptor().clone());
            deps.push(super::KAMCIOPBPGA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KEHMGKIHEFN::generated_message_descriptor_data());
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
