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

//! Generated file from `APLMIDIJFFA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:APLMIDIJFFA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct APLMIDIJFFA {
    // message oneof groups
    pub FGODPDIPECG: ::std::option::Option<aplmidijffa::FGODPDIPECG>,
    // special fields
    // @@protoc_insertion_point(special_field:APLMIDIJFFA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a APLMIDIJFFA {
    fn default() -> &'a APLMIDIJFFA {
        <APLMIDIJFFA as ::protobuf::Message>::default_instance()
    }
}

impl APLMIDIJFFA {
    pub fn new() -> APLMIDIJFFA {
        ::std::default::Default::default()
    }

    // .BGBDLOPNMAA KOHGAOKNGJI = 8;

    pub fn KOHGAOKNGJI(&self) -> &super::BGBDLOPNMAA::BGBDLOPNMAA {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(ref v)) => v,
            _ => <super::BGBDLOPNMAA::BGBDLOPNMAA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KOHGAOKNGJI(&mut self) {
        self.FGODPDIPECG = ::std::option::Option::None;
    }

    pub fn has_KOHGAOKNGJI(&self) -> bool {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KOHGAOKNGJI(&mut self, v: super::BGBDLOPNMAA::BGBDLOPNMAA) {
        self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KOHGAOKNGJI(&mut self) -> &mut super::BGBDLOPNMAA::BGBDLOPNMAA {
        if let ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(_)) = self.FGODPDIPECG {
        } else {
            self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(super::BGBDLOPNMAA::BGBDLOPNMAA::new()));
        }
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KOHGAOKNGJI(&mut self) -> super::BGBDLOPNMAA::BGBDLOPNMAA {
        if self.has_KOHGAOKNGJI() {
            match self.FGODPDIPECG.take() {
                ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BGBDLOPNMAA::BGBDLOPNMAA::new()
        }
    }

    // .GJPLDENOPEN DCCCIHEKHFO = 9;

    pub fn DCCCIHEKHFO(&self) -> &super::GJPLDENOPEN::GJPLDENOPEN {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(ref v)) => v,
            _ => <super::GJPLDENOPEN::GJPLDENOPEN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DCCCIHEKHFO(&mut self) {
        self.FGODPDIPECG = ::std::option::Option::None;
    }

    pub fn has_DCCCIHEKHFO(&self) -> bool {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DCCCIHEKHFO(&mut self, v: super::GJPLDENOPEN::GJPLDENOPEN) {
        self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DCCCIHEKHFO(&mut self) -> &mut super::GJPLDENOPEN::GJPLDENOPEN {
        if let ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(_)) = self.FGODPDIPECG {
        } else {
            self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(super::GJPLDENOPEN::GJPLDENOPEN::new()));
        }
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DCCCIHEKHFO(&mut self) -> super::GJPLDENOPEN::GJPLDENOPEN {
        if self.has_DCCCIHEKHFO() {
            match self.FGODPDIPECG.take() {
                ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GJPLDENOPEN::GJPLDENOPEN::new()
        }
    }

    // .KPGPGKJDFII FUNC_UNLOCK_ID_RELIC = 13;

    pub fn FUNC_UNLOCK_ID_RELIC(&self) -> &super::KPGPGKJDFII::KPGPGKJDFII {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(ref v)) => v,
            _ => <super::KPGPGKJDFII::KPGPGKJDFII as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FUNC_UNLOCK_ID_RELIC(&mut self) {
        self.FGODPDIPECG = ::std::option::Option::None;
    }

    pub fn has_FUNC_UNLOCK_ID_RELIC(&self) -> bool {
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FUNC_UNLOCK_ID_RELIC(&mut self, v: super::KPGPGKJDFII::KPGPGKJDFII) {
        self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FUNC_UNLOCK_ID_RELIC(&mut self) -> &mut super::KPGPGKJDFII::KPGPGKJDFII {
        if let ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(_)) = self.FGODPDIPECG {
        } else {
            self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(super::KPGPGKJDFII::KPGPGKJDFII::new()));
        }
        match self.FGODPDIPECG {
            ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FUNC_UNLOCK_ID_RELIC(&mut self) -> super::KPGPGKJDFII::KPGPGKJDFII {
        if self.has_FUNC_UNLOCK_ID_RELIC() {
            match self.FGODPDIPECG.take() {
                ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KPGPGKJDFII::KPGPGKJDFII::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BGBDLOPNMAA::BGBDLOPNMAA>(
            "KOHGAOKNGJI",
            APLMIDIJFFA::has_KOHGAOKNGJI,
            APLMIDIJFFA::KOHGAOKNGJI,
            APLMIDIJFFA::mut_KOHGAOKNGJI,
            APLMIDIJFFA::set_KOHGAOKNGJI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GJPLDENOPEN::GJPLDENOPEN>(
            "DCCCIHEKHFO",
            APLMIDIJFFA::has_DCCCIHEKHFO,
            APLMIDIJFFA::DCCCIHEKHFO,
            APLMIDIJFFA::mut_DCCCIHEKHFO,
            APLMIDIJFFA::set_DCCCIHEKHFO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KPGPGKJDFII::KPGPGKJDFII>(
            "FUNC_UNLOCK_ID_RELIC",
            APLMIDIJFFA::has_FUNC_UNLOCK_ID_RELIC,
            APLMIDIJFFA::FUNC_UNLOCK_ID_RELIC,
            APLMIDIJFFA::mut_FUNC_UNLOCK_ID_RELIC,
            APLMIDIJFFA::set_FUNC_UNLOCK_ID_RELIC,
        ));
        oneofs.push(aplmidijffa::FGODPDIPECG::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<APLMIDIJFFA>(
            "APLMIDIJFFA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for APLMIDIJFFA {
    const NAME: &'static str = "APLMIDIJFFA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(is.read_message()?));
                },
                74 => {
                    self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(is.read_message()?));
                },
                106 => {
                    self.FGODPDIPECG = ::std::option::Option::Some(aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.FGODPDIPECG {
            match v {
                &aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.FGODPDIPECG {
            match v {
                &aplmidijffa::FGODPDIPECG::KOHGAOKNGJI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &aplmidijffa::FGODPDIPECG::DCCCIHEKHFO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &aplmidijffa::FGODPDIPECG::FUNCUNLOCKIDRELIC(ref v) => {
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

    fn new() -> APLMIDIJFFA {
        APLMIDIJFFA::new()
    }

    fn clear(&mut self) {
        self.FGODPDIPECG = ::std::option::Option::None;
        self.FGODPDIPECG = ::std::option::Option::None;
        self.FGODPDIPECG = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static APLMIDIJFFA {
        static instance: APLMIDIJFFA = APLMIDIJFFA {
            FGODPDIPECG: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for APLMIDIJFFA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("APLMIDIJFFA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for APLMIDIJFFA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APLMIDIJFFA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `APLMIDIJFFA`
pub mod aplmidijffa {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:APLMIDIJFFA.FGODPDIPECG)
    pub enum FGODPDIPECG {
        // @@protoc_insertion_point(oneof_field:APLMIDIJFFA.KOHGAOKNGJI)
        KOHGAOKNGJI(super::super::BGBDLOPNMAA::BGBDLOPNMAA),
        // @@protoc_insertion_point(oneof_field:APLMIDIJFFA.DCCCIHEKHFO)
        DCCCIHEKHFO(super::super::GJPLDENOPEN::GJPLDENOPEN),
        // @@protoc_insertion_point(oneof_field:APLMIDIJFFA.FUNC_UNLOCK_ID_RELIC)
        FUNCUNLOCKIDRELIC(super::super::KPGPGKJDFII::KPGPGKJDFII),
    }

    impl ::protobuf::Oneof for FGODPDIPECG {
    }

    impl ::protobuf::OneofFull for FGODPDIPECG {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::APLMIDIJFFA as ::protobuf::MessageFull>::descriptor().oneof_by_name("FGODPDIPECG").unwrap()).clone()
        }
    }

    impl FGODPDIPECG {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<FGODPDIPECG>("FGODPDIPECG")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11APLMIDIJFFA.proto\x1a\x11BGBDLOPNMAA.proto\x1a\x11GJPLDENOPEN.prot\
    o\x1a\x11KPGPGKJDFII.proto\"\xc1\x01\n\x0bAPLMIDIJFFA\x120\n\x0bKOHGAOKN\
    GJI\x18\x08\x20\x01(\x0b2\x0c.BGBDLOPNMAAH\0R\x0bKOHGAOKNGJI\x120\n\x0bD\
    CCCIHEKHFO\x18\t\x20\x01(\x0b2\x0c.GJPLDENOPENH\0R\x0bDCCCIHEKHFO\x12?\n\
    \x14FUNC_UNLOCK_ID_RELIC\x18\r\x20\x01(\x0b2\x0c.KPGPGKJDFIIH\0R\x11FUNC\
    UNLOCKIDRELICB\r\n\x0bFGODPDIPECGb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::BGBDLOPNMAA::file_descriptor().clone());
            deps.push(super::GJPLDENOPEN::file_descriptor().clone());
            deps.push(super::KPGPGKJDFII::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(APLMIDIJFFA::generated_message_descriptor_data());
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
