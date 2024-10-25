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

//! Generated file from `EHEACEHNEGA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EHEACEHNEGA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EHEACEHNEGA {
    // message fields
    // @@protoc_insertion_point(field:EHEACEHNEGA.KNMDKFFBGHP)
    pub KNMDKFFBGHP: ::protobuf::MessageField<super::OIKHAJADEMG::OIKHAJADEMG>,
    // @@protoc_insertion_point(field:EHEACEHNEGA.HFGMHPLJFKP)
    pub HFGMHPLJFKP: ::protobuf::MessageField<super::IFIJMOECCOE::IFIJMOECCOE>,
    // @@protoc_insertion_point(field:EHEACEHNEGA.GBBCBCAOOKI)
    pub GBBCBCAOOKI: ::protobuf::MessageField<super::OLFEIIBMPGL::OLFEIIBMPGL>,
    // message oneof groups
    pub LDCNBBEOCCC: ::std::option::Option<eheacehnega::LDCNBBEOCCC>,
    // special fields
    // @@protoc_insertion_point(special_field:EHEACEHNEGA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EHEACEHNEGA {
    fn default() -> &'a EHEACEHNEGA {
        <EHEACEHNEGA as ::protobuf::Message>::default_instance()
    }
}

impl EHEACEHNEGA {
    pub fn new() -> EHEACEHNEGA {
        ::std::default::Default::default()
    }

    // .EFIGAKCIHHM BFBOLGKIMIB = 181;

    pub fn BFBOLGKIMIB(&self) -> &super::EFIGAKCIHHM::EFIGAKCIHHM {
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(ref v)) => v,
            _ => <super::EFIGAKCIHHM::EFIGAKCIHHM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BFBOLGKIMIB(&mut self) {
        self.LDCNBBEOCCC = ::std::option::Option::None;
    }

    pub fn has_BFBOLGKIMIB(&self) -> bool {
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BFBOLGKIMIB(&mut self, v: super::EFIGAKCIHHM::EFIGAKCIHHM) {
        self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BFBOLGKIMIB(&mut self) -> &mut super::EFIGAKCIHHM::EFIGAKCIHHM {
        if let ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(_)) = self.LDCNBBEOCCC {
        } else {
            self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(super::EFIGAKCIHHM::EFIGAKCIHHM::new()));
        }
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BFBOLGKIMIB(&mut self) -> super::EFIGAKCIHHM::EFIGAKCIHHM {
        if self.has_BFBOLGKIMIB() {
            match self.LDCNBBEOCCC.take() {
                ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EFIGAKCIHHM::EFIGAKCIHHM::new()
        }
    }

    // .CPDLMGKIJLJ LIGEDIHEFMB = 960;

    pub fn LIGEDIHEFMB(&self) -> &super::CPDLMGKIJLJ::CPDLMGKIJLJ {
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(ref v)) => v,
            _ => <super::CPDLMGKIJLJ::CPDLMGKIJLJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LIGEDIHEFMB(&mut self) {
        self.LDCNBBEOCCC = ::std::option::Option::None;
    }

    pub fn has_LIGEDIHEFMB(&self) -> bool {
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LIGEDIHEFMB(&mut self, v: super::CPDLMGKIJLJ::CPDLMGKIJLJ) {
        self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LIGEDIHEFMB(&mut self) -> &mut super::CPDLMGKIJLJ::CPDLMGKIJLJ {
        if let ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(_)) = self.LDCNBBEOCCC {
        } else {
            self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(super::CPDLMGKIJLJ::CPDLMGKIJLJ::new()));
        }
        match self.LDCNBBEOCCC {
            ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LIGEDIHEFMB(&mut self) -> super::CPDLMGKIJLJ::CPDLMGKIJLJ {
        if self.has_LIGEDIHEFMB() {
            match self.LDCNBBEOCCC.take() {
                ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CPDLMGKIJLJ::CPDLMGKIJLJ::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OIKHAJADEMG::OIKHAJADEMG>(
            "KNMDKFFBGHP",
            |m: &EHEACEHNEGA| { &m.KNMDKFFBGHP },
            |m: &mut EHEACEHNEGA| { &mut m.KNMDKFFBGHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IFIJMOECCOE::IFIJMOECCOE>(
            "HFGMHPLJFKP",
            |m: &EHEACEHNEGA| { &m.HFGMHPLJFKP },
            |m: &mut EHEACEHNEGA| { &mut m.HFGMHPLJFKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OLFEIIBMPGL::OLFEIIBMPGL>(
            "GBBCBCAOOKI",
            |m: &EHEACEHNEGA| { &m.GBBCBCAOOKI },
            |m: &mut EHEACEHNEGA| { &mut m.GBBCBCAOOKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EFIGAKCIHHM::EFIGAKCIHHM>(
            "BFBOLGKIMIB",
            EHEACEHNEGA::has_BFBOLGKIMIB,
            EHEACEHNEGA::BFBOLGKIMIB,
            EHEACEHNEGA::mut_BFBOLGKIMIB,
            EHEACEHNEGA::set_BFBOLGKIMIB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CPDLMGKIJLJ::CPDLMGKIJLJ>(
            "LIGEDIHEFMB",
            EHEACEHNEGA::has_LIGEDIHEFMB,
            EHEACEHNEGA::LIGEDIHEFMB,
            EHEACEHNEGA::mut_LIGEDIHEFMB,
            EHEACEHNEGA::set_LIGEDIHEFMB,
        ));
        oneofs.push(eheacehnega::LDCNBBEOCCC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EHEACEHNEGA>(
            "EHEACEHNEGA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EHEACEHNEGA {
    const NAME: &'static str = "EHEACEHNEGA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KNMDKFFBGHP)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HFGMHPLJFKP)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GBBCBCAOOKI)?;
                },
                1450 => {
                    self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(is.read_message()?));
                },
                7682 => {
                    self.LDCNBBEOCCC = ::std::option::Option::Some(eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(is.read_message()?));
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
        if let Some(v) = self.KNMDKFFBGHP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HFGMHPLJFKP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GBBCBCAOOKI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.LDCNBBEOCCC {
            match v {
                &eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(ref v) => {
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
        if let Some(v) = self.KNMDKFFBGHP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.HFGMHPLJFKP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.GBBCBCAOOKI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.LDCNBBEOCCC {
            match v {
                &eheacehnega::LDCNBBEOCCC::BFBOLGKIMIB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(181, v, os)?;
                },
                &eheacehnega::LDCNBBEOCCC::LIGEDIHEFMB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(960, v, os)?;
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

    fn new() -> EHEACEHNEGA {
        EHEACEHNEGA::new()
    }

    fn clear(&mut self) {
        self.KNMDKFFBGHP.clear();
        self.HFGMHPLJFKP.clear();
        self.GBBCBCAOOKI.clear();
        self.LDCNBBEOCCC = ::std::option::Option::None;
        self.LDCNBBEOCCC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EHEACEHNEGA {
        static instance: EHEACEHNEGA = EHEACEHNEGA {
            KNMDKFFBGHP: ::protobuf::MessageField::none(),
            HFGMHPLJFKP: ::protobuf::MessageField::none(),
            GBBCBCAOOKI: ::protobuf::MessageField::none(),
            LDCNBBEOCCC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EHEACEHNEGA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EHEACEHNEGA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EHEACEHNEGA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EHEACEHNEGA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `EHEACEHNEGA`
pub mod eheacehnega {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:EHEACEHNEGA.LDCNBBEOCCC)
    pub enum LDCNBBEOCCC {
        // @@protoc_insertion_point(oneof_field:EHEACEHNEGA.BFBOLGKIMIB)
        BFBOLGKIMIB(super::super::EFIGAKCIHHM::EFIGAKCIHHM),
        // @@protoc_insertion_point(oneof_field:EHEACEHNEGA.LIGEDIHEFMB)
        LIGEDIHEFMB(super::super::CPDLMGKIJLJ::CPDLMGKIJLJ),
    }

    impl ::protobuf::Oneof for LDCNBBEOCCC {
    }

    impl ::protobuf::OneofFull for LDCNBBEOCCC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::EHEACEHNEGA as ::protobuf::MessageFull>::descriptor().oneof_by_name("LDCNBBEOCCC").unwrap()).clone()
        }
    }

    impl LDCNBBEOCCC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<LDCNBBEOCCC>("LDCNBBEOCCC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EHEACEHNEGA.proto\x1a\x11CPDLMGKIJLJ.proto\x1a\x11EFIGAKCIHHM.prot\
    o\x1a\x11IFIJMOECCOE.proto\x1a\x11OIKHAJADEMG.proto\x1a\x11OLFEIIBMPGL.p\
    roto\"\x92\x02\n\x0bEHEACEHNEGA\x12.\n\x0bKNMDKFFBGHP\x18\x06\x20\x01(\
    \x0b2\x0c.OIKHAJADEMGR\x0bKNMDKFFBGHP\x12.\n\x0bHFGMHPLJFKP\x18\x0f\x20\
    \x01(\x0b2\x0c.IFIJMOECCOER\x0bHFGMHPLJFKP\x12.\n\x0bGBBCBCAOOKI\x18\x01\
    \x20\x01(\x0b2\x0c.OLFEIIBMPGLR\x0bGBBCBCAOOKI\x121\n\x0bBFBOLGKIMIB\x18\
    \xb5\x01\x20\x01(\x0b2\x0c.EFIGAKCIHHMH\0R\x0bBFBOLGKIMIB\x121\n\x0bLIGE\
    DIHEFMB\x18\xc0\x07\x20\x01(\x0b2\x0c.CPDLMGKIJLJH\0R\x0bLIGEDIHEFMBB\r\
    \n\x0bLDCNBBEOCCCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::CPDLMGKIJLJ::file_descriptor().clone());
            deps.push(super::EFIGAKCIHHM::file_descriptor().clone());
            deps.push(super::IFIJMOECCOE::file_descriptor().clone());
            deps.push(super::OIKHAJADEMG::file_descriptor().clone());
            deps.push(super::OLFEIIBMPGL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EHEACEHNEGA::generated_message_descriptor_data());
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