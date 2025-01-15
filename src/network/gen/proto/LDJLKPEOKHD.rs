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

//! Generated file from `LDJLKPEOKHD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LDJLKPEOKHD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LDJLKPEOKHD {
    // message oneof groups
    pub DKFKPIPNNLF: ::std::option::Option<ldjlkpeokhd::DKFKPIPNNLF>,
    // special fields
    // @@protoc_insertion_point(special_field:LDJLKPEOKHD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LDJLKPEOKHD {
    fn default() -> &'a LDJLKPEOKHD {
        <LDJLKPEOKHD as ::protobuf::Message>::default_instance()
    }
}

impl LDJLKPEOKHD {
    pub fn new() -> LDJLKPEOKHD {
        ::std::default::Default::default()
    }

    // .HHDKGBGMEBG HHALBJMNPBB = 1;

    pub fn HHALBJMNPBB(&self) -> &super::HHDKGBGMEBG::HHDKGBGMEBG {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(ref v)) => v,
            _ => <super::HHDKGBGMEBG::HHDKGBGMEBG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_HHALBJMNPBB(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
    }

    pub fn has_HHALBJMNPBB(&self) -> bool {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HHALBJMNPBB(&mut self, v: super::HHDKGBGMEBG::HHDKGBGMEBG) {
        self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_HHALBJMNPBB(&mut self) -> &mut super::HHDKGBGMEBG::HHDKGBGMEBG {
        if let ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(_)) = self.DKFKPIPNNLF {
        } else {
            self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(super::HHDKGBGMEBG::HHDKGBGMEBG::new()));
        }
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_HHALBJMNPBB(&mut self) -> super::HHDKGBGMEBG::HHDKGBGMEBG {
        if self.has_HHALBJMNPBB() {
            match self.DKFKPIPNNLF.take() {
                ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HHDKGBGMEBG::HHDKGBGMEBG::new()
        }
    }

    // .DEIEMBNNKEH DJCLHGCAHHL = 15;

    pub fn DJCLHGCAHHL(&self) -> &super::DEIEMBNNKEH::DEIEMBNNKEH {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(ref v)) => v,
            _ => <super::DEIEMBNNKEH::DEIEMBNNKEH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DJCLHGCAHHL(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
    }

    pub fn has_DJCLHGCAHHL(&self) -> bool {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DJCLHGCAHHL(&mut self, v: super::DEIEMBNNKEH::DEIEMBNNKEH) {
        self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DJCLHGCAHHL(&mut self) -> &mut super::DEIEMBNNKEH::DEIEMBNNKEH {
        if let ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(_)) = self.DKFKPIPNNLF {
        } else {
            self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(super::DEIEMBNNKEH::DEIEMBNNKEH::new()));
        }
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DJCLHGCAHHL(&mut self) -> super::DEIEMBNNKEH::DEIEMBNNKEH {
        if self.has_DJCLHGCAHHL() {
            match self.DKFKPIPNNLF.take() {
                ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DEIEMBNNKEH::DEIEMBNNKEH::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HHDKGBGMEBG::HHDKGBGMEBG>(
            "HHALBJMNPBB",
            LDJLKPEOKHD::has_HHALBJMNPBB,
            LDJLKPEOKHD::HHALBJMNPBB,
            LDJLKPEOKHD::mut_HHALBJMNPBB,
            LDJLKPEOKHD::set_HHALBJMNPBB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DEIEMBNNKEH::DEIEMBNNKEH>(
            "DJCLHGCAHHL",
            LDJLKPEOKHD::has_DJCLHGCAHHL,
            LDJLKPEOKHD::DJCLHGCAHHL,
            LDJLKPEOKHD::mut_DJCLHGCAHHL,
            LDJLKPEOKHD::set_DJCLHGCAHHL,
        ));
        oneofs.push(ldjlkpeokhd::DKFKPIPNNLF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LDJLKPEOKHD>(
            "LDJLKPEOKHD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LDJLKPEOKHD {
    const NAME: &'static str = "LDJLKPEOKHD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(is.read_message()?));
                },
                122 => {
                    self.DKFKPIPNNLF = ::std::option::Option::Some(ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &ldjlkpeokhd::DKFKPIPNNLF::HHALBJMNPBB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &ldjlkpeokhd::DKFKPIPNNLF::DJCLHGCAHHL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> LDJLKPEOKHD {
        LDJLKPEOKHD::new()
    }

    fn clear(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
        self.DKFKPIPNNLF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LDJLKPEOKHD {
        static instance: LDJLKPEOKHD = LDJLKPEOKHD {
            DKFKPIPNNLF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LDJLKPEOKHD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LDJLKPEOKHD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LDJLKPEOKHD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LDJLKPEOKHD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `LDJLKPEOKHD`
pub mod ldjlkpeokhd {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:LDJLKPEOKHD.DKFKPIPNNLF)
    pub enum DKFKPIPNNLF {
        // @@protoc_insertion_point(oneof_field:LDJLKPEOKHD.HHALBJMNPBB)
        HHALBJMNPBB(super::super::HHDKGBGMEBG::HHDKGBGMEBG),
        // @@protoc_insertion_point(oneof_field:LDJLKPEOKHD.DJCLHGCAHHL)
        DJCLHGCAHHL(super::super::DEIEMBNNKEH::DEIEMBNNKEH),
    }

    impl ::protobuf::Oneof for DKFKPIPNNLF {
    }

    impl ::protobuf::OneofFull for DKFKPIPNNLF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::LDJLKPEOKHD as ::protobuf::MessageFull>::descriptor().oneof_by_name("DKFKPIPNNLF").unwrap()).clone()
        }
    }

    impl DKFKPIPNNLF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DKFKPIPNNLF>("DKFKPIPNNLF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LDJLKPEOKHD.proto\x1a\x11DEIEMBNNKEH.proto\x1a\x11HHDKGBGMEBG.prot\
    o\"\x80\x01\n\x0bLDJLKPEOKHD\x120\n\x0bHHALBJMNPBB\x18\x01\x20\x01(\x0b2\
    \x0c.HHDKGBGMEBGH\0R\x0bHHALBJMNPBB\x120\n\x0bDJCLHGCAHHL\x18\x0f\x20\
    \x01(\x0b2\x0c.DEIEMBNNKEHH\0R\x0bDJCLHGCAHHLB\r\n\x0bDKFKPIPNNLFb\x06pr\
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
            deps.push(super::DEIEMBNNKEH::file_descriptor().clone());
            deps.push(super::HHDKGBGMEBG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LDJLKPEOKHD::generated_message_descriptor_data());
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