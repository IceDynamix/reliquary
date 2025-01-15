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

//! Generated file from `ADKIDBBBHNC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ADKIDBBBHNC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ADKIDBBBHNC {
    // message fields
    // @@protoc_insertion_point(field:ADKIDBBBHNC.PFEKOEIHBIJ)
    pub PFEKOEIHBIJ: u32,
    // message oneof groups
    pub DKFKPIPNNLF: ::std::option::Option<adkidbbbhnc::DKFKPIPNNLF>,
    // special fields
    // @@protoc_insertion_point(special_field:ADKIDBBBHNC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ADKIDBBBHNC {
    fn default() -> &'a ADKIDBBBHNC {
        <ADKIDBBBHNC as ::protobuf::Message>::default_instance()
    }
}

impl ADKIDBBBHNC {
    pub fn new() -> ADKIDBBBHNC {
        ::std::default::Default::default()
    }

    // .HNIBJCGHCAN JNKCJGMOOFK = 15;

    pub fn JNKCJGMOOFK(&self) -> &super::HNIBJCGHCAN::HNIBJCGHCAN {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(ref v)) => v,
            _ => <super::HNIBJCGHCAN::HNIBJCGHCAN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JNKCJGMOOFK(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
    }

    pub fn has_JNKCJGMOOFK(&self) -> bool {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JNKCJGMOOFK(&mut self, v: super::HNIBJCGHCAN::HNIBJCGHCAN) {
        self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JNKCJGMOOFK(&mut self) -> &mut super::HNIBJCGHCAN::HNIBJCGHCAN {
        if let ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(_)) = self.DKFKPIPNNLF {
        } else {
            self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(super::HNIBJCGHCAN::HNIBJCGHCAN::new()));
        }
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JNKCJGMOOFK(&mut self) -> super::HNIBJCGHCAN::HNIBJCGHCAN {
        if self.has_JNKCJGMOOFK() {
            match self.DKFKPIPNNLF.take() {
                ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HNIBJCGHCAN::HNIBJCGHCAN::new()
        }
    }

    // .KPCEFHJBOPF FLLMJKIEFDM = 13;

    pub fn FLLMJKIEFDM(&self) -> &super::KPCEFHJBOPF::KPCEFHJBOPF {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(ref v)) => v,
            _ => <super::KPCEFHJBOPF::KPCEFHJBOPF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FLLMJKIEFDM(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
    }

    pub fn has_FLLMJKIEFDM(&self) -> bool {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FLLMJKIEFDM(&mut self, v: super::KPCEFHJBOPF::KPCEFHJBOPF) {
        self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FLLMJKIEFDM(&mut self) -> &mut super::KPCEFHJBOPF::KPCEFHJBOPF {
        if let ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(_)) = self.DKFKPIPNNLF {
        } else {
            self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(super::KPCEFHJBOPF::KPCEFHJBOPF::new()));
        }
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FLLMJKIEFDM(&mut self) -> super::KPCEFHJBOPF::KPCEFHJBOPF {
        if self.has_FLLMJKIEFDM() {
            match self.DKFKPIPNNLF.take() {
                ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KPCEFHJBOPF::KPCEFHJBOPF::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PFEKOEIHBIJ",
            |m: &ADKIDBBBHNC| { &m.PFEKOEIHBIJ },
            |m: &mut ADKIDBBBHNC| { &mut m.PFEKOEIHBIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HNIBJCGHCAN::HNIBJCGHCAN>(
            "JNKCJGMOOFK",
            ADKIDBBBHNC::has_JNKCJGMOOFK,
            ADKIDBBBHNC::JNKCJGMOOFK,
            ADKIDBBBHNC::mut_JNKCJGMOOFK,
            ADKIDBBBHNC::set_JNKCJGMOOFK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KPCEFHJBOPF::KPCEFHJBOPF>(
            "FLLMJKIEFDM",
            ADKIDBBBHNC::has_FLLMJKIEFDM,
            ADKIDBBBHNC::FLLMJKIEFDM,
            ADKIDBBBHNC::mut_FLLMJKIEFDM,
            ADKIDBBBHNC::set_FLLMJKIEFDM,
        ));
        oneofs.push(adkidbbbhnc::DKFKPIPNNLF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ADKIDBBBHNC>(
            "ADKIDBBBHNC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ADKIDBBBHNC {
    const NAME: &'static str = "ADKIDBBBHNC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                14552 => {
                    self.PFEKOEIHBIJ = is.read_uint32()?;
                },
                122 => {
                    self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(is.read_message()?));
                },
                106 => {
                    self.DKFKPIPNNLF = ::std::option::Option::Some(adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(is.read_message()?));
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
        if self.PFEKOEIHBIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1819, self.PFEKOEIHBIJ);
        }
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(ref v) => {
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
        if self.PFEKOEIHBIJ != 0 {
            os.write_uint32(1819, self.PFEKOEIHBIJ)?;
        }
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &adkidbbbhnc::DKFKPIPNNLF::JNKCJGMOOFK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &adkidbbbhnc::DKFKPIPNNLF::FLLMJKIEFDM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> ADKIDBBBHNC {
        ADKIDBBBHNC::new()
    }

    fn clear(&mut self) {
        self.PFEKOEIHBIJ = 0;
        self.DKFKPIPNNLF = ::std::option::Option::None;
        self.DKFKPIPNNLF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ADKIDBBBHNC {
        static instance: ADKIDBBBHNC = ADKIDBBBHNC {
            PFEKOEIHBIJ: 0,
            DKFKPIPNNLF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ADKIDBBBHNC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ADKIDBBBHNC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ADKIDBBBHNC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ADKIDBBBHNC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ADKIDBBBHNC`
pub mod adkidbbbhnc {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ADKIDBBBHNC.DKFKPIPNNLF)
    pub enum DKFKPIPNNLF {
        // @@protoc_insertion_point(oneof_field:ADKIDBBBHNC.JNKCJGMOOFK)
        JNKCJGMOOFK(super::super::HNIBJCGHCAN::HNIBJCGHCAN),
        // @@protoc_insertion_point(oneof_field:ADKIDBBBHNC.FLLMJKIEFDM)
        FLLMJKIEFDM(super::super::KPCEFHJBOPF::KPCEFHJBOPF),
    }

    impl ::protobuf::Oneof for DKFKPIPNNLF {
    }

    impl ::protobuf::OneofFull for DKFKPIPNNLF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ADKIDBBBHNC as ::protobuf::MessageFull>::descriptor().oneof_by_name("DKFKPIPNNLF").unwrap()).clone()
        }
    }

    impl DKFKPIPNNLF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DKFKPIPNNLF>("DKFKPIPNNLF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ADKIDBBBHNC.proto\x1a\x11HNIBJCGHCAN.proto\x1a\x11KPCEFHJBOPF.prot\
    o\"\xa3\x01\n\x0bADKIDBBBHNC\x12!\n\x0bPFEKOEIHBIJ\x18\x9b\x0e\x20\x01(\
    \rR\x0bPFEKOEIHBIJ\x120\n\x0bJNKCJGMOOFK\x18\x0f\x20\x01(\x0b2\x0c.HNIBJ\
    CGHCANH\0R\x0bJNKCJGMOOFK\x120\n\x0bFLLMJKIEFDM\x18\r\x20\x01(\x0b2\x0c.\
    KPCEFHJBOPFH\0R\x0bFLLMJKIEFDMB\r\n\x0bDKFKPIPNNLFb\x06proto3\
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
            deps.push(super::HNIBJCGHCAN::file_descriptor().clone());
            deps.push(super::KPCEFHJBOPF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ADKIDBBBHNC::generated_message_descriptor_data());
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