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

//! Generated file from `BFAMGMKCPLN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BFAMGMKCPLN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BFAMGMKCPLN {
    // message oneof groups
    pub BEKBNAKELOK: ::std::option::Option<bfamgmkcpln::BEKBNAKELOK>,
    // special fields
    // @@protoc_insertion_point(special_field:BFAMGMKCPLN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BFAMGMKCPLN {
    fn default() -> &'a BFAMGMKCPLN {
        <BFAMGMKCPLN as ::protobuf::Message>::default_instance()
    }
}

impl BFAMGMKCPLN {
    pub fn new() -> BFAMGMKCPLN {
        ::std::default::Default::default()
    }

    // .ECHKEPBCIOJ KLGMJMBIODB = 11;

    pub fn KLGMJMBIODB(&self) -> &super::ECHKEPBCIOJ::ECHKEPBCIOJ {
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(ref v)) => v,
            _ => <super::ECHKEPBCIOJ::ECHKEPBCIOJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KLGMJMBIODB(&mut self) {
        self.BEKBNAKELOK = ::std::option::Option::None;
    }

    pub fn has_KLGMJMBIODB(&self) -> bool {
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KLGMJMBIODB(&mut self, v: super::ECHKEPBCIOJ::ECHKEPBCIOJ) {
        self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KLGMJMBIODB(&mut self) -> &mut super::ECHKEPBCIOJ::ECHKEPBCIOJ {
        if let ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(_)) = self.BEKBNAKELOK {
        } else {
            self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(super::ECHKEPBCIOJ::ECHKEPBCIOJ::new()));
        }
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KLGMJMBIODB(&mut self) -> super::ECHKEPBCIOJ::ECHKEPBCIOJ {
        if self.has_KLGMJMBIODB() {
            match self.BEKBNAKELOK.take() {
                ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ECHKEPBCIOJ::ECHKEPBCIOJ::new()
        }
    }

    // .LCCFLFOAKMI ODAAHKFADPF = 8;

    pub fn ODAAHKFADPF(&self) -> &super::LCCFLFOAKMI::LCCFLFOAKMI {
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(ref v)) => v,
            _ => <super::LCCFLFOAKMI::LCCFLFOAKMI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ODAAHKFADPF(&mut self) {
        self.BEKBNAKELOK = ::std::option::Option::None;
    }

    pub fn has_ODAAHKFADPF(&self) -> bool {
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ODAAHKFADPF(&mut self, v: super::LCCFLFOAKMI::LCCFLFOAKMI) {
        self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ODAAHKFADPF(&mut self) -> &mut super::LCCFLFOAKMI::LCCFLFOAKMI {
        if let ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(_)) = self.BEKBNAKELOK {
        } else {
            self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(super::LCCFLFOAKMI::LCCFLFOAKMI::new()));
        }
        match self.BEKBNAKELOK {
            ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ODAAHKFADPF(&mut self) -> super::LCCFLFOAKMI::LCCFLFOAKMI {
        if self.has_ODAAHKFADPF() {
            match self.BEKBNAKELOK.take() {
                ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LCCFLFOAKMI::LCCFLFOAKMI::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ECHKEPBCIOJ::ECHKEPBCIOJ>(
            "KLGMJMBIODB",
            BFAMGMKCPLN::has_KLGMJMBIODB,
            BFAMGMKCPLN::KLGMJMBIODB,
            BFAMGMKCPLN::mut_KLGMJMBIODB,
            BFAMGMKCPLN::set_KLGMJMBIODB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LCCFLFOAKMI::LCCFLFOAKMI>(
            "ODAAHKFADPF",
            BFAMGMKCPLN::has_ODAAHKFADPF,
            BFAMGMKCPLN::ODAAHKFADPF,
            BFAMGMKCPLN::mut_ODAAHKFADPF,
            BFAMGMKCPLN::set_ODAAHKFADPF,
        ));
        oneofs.push(bfamgmkcpln::BEKBNAKELOK::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BFAMGMKCPLN>(
            "BFAMGMKCPLN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BFAMGMKCPLN {
    const NAME: &'static str = "BFAMGMKCPLN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(is.read_message()?));
                },
                66 => {
                    self.BEKBNAKELOK = ::std::option::Option::Some(bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.BEKBNAKELOK {
            match v {
                &bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.BEKBNAKELOK {
            match v {
                &bfamgmkcpln::BEKBNAKELOK::KLGMJMBIODB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
                },
                &bfamgmkcpln::BEKBNAKELOK::ODAAHKFADPF(ref v) => {
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

    fn new() -> BFAMGMKCPLN {
        BFAMGMKCPLN::new()
    }

    fn clear(&mut self) {
        self.BEKBNAKELOK = ::std::option::Option::None;
        self.BEKBNAKELOK = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BFAMGMKCPLN {
        static instance: BFAMGMKCPLN = BFAMGMKCPLN {
            BEKBNAKELOK: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BFAMGMKCPLN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BFAMGMKCPLN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BFAMGMKCPLN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BFAMGMKCPLN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `BFAMGMKCPLN`
pub mod bfamgmkcpln {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:BFAMGMKCPLN.BEKBNAKELOK)
    pub enum BEKBNAKELOK {
        // @@protoc_insertion_point(oneof_field:BFAMGMKCPLN.KLGMJMBIODB)
        KLGMJMBIODB(super::super::ECHKEPBCIOJ::ECHKEPBCIOJ),
        // @@protoc_insertion_point(oneof_field:BFAMGMKCPLN.ODAAHKFADPF)
        ODAAHKFADPF(super::super::LCCFLFOAKMI::LCCFLFOAKMI),
    }

    impl ::protobuf::Oneof for BEKBNAKELOK {
    }

    impl ::protobuf::OneofFull for BEKBNAKELOK {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::BFAMGMKCPLN as ::protobuf::MessageFull>::descriptor().oneof_by_name("BEKBNAKELOK").unwrap()).clone()
        }
    }

    impl BEKBNAKELOK {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<BEKBNAKELOK>("BEKBNAKELOK")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BFAMGMKCPLN.proto\x1a\x11ECHKEPBCIOJ.proto\x1a\x11LCCFLFOAKMI.prot\
    o\"\x80\x01\n\x0bBFAMGMKCPLN\x120\n\x0bKLGMJMBIODB\x18\x0b\x20\x01(\x0b2\
    \x0c.ECHKEPBCIOJH\0R\x0bKLGMJMBIODB\x120\n\x0bODAAHKFADPF\x18\x08\x20\
    \x01(\x0b2\x0c.LCCFLFOAKMIH\0R\x0bODAAHKFADPFB\r\n\x0bBEKBNAKELOKb\x06pr\
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
            deps.push(super::ECHKEPBCIOJ::file_descriptor().clone());
            deps.push(super::LCCFLFOAKMI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BFAMGMKCPLN::generated_message_descriptor_data());
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