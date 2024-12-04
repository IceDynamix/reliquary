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

//! Generated file from `CAGDHENEFND.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CAGDHENEFND)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CAGDHENEFND {
    // message fields
    // @@protoc_insertion_point(field:CAGDHENEFND.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::GMFDCJMEIJM::GMFDCJMEIJM>,
    // @@protoc_insertion_point(field:CAGDHENEFND.ONALKMOMMEC)
    pub ONALKMOMMEC: u32,
    // @@protoc_insertion_point(field:CAGDHENEFND.PMMELAOHNGO)
    pub PMMELAOHNGO: u32,
    // message oneof groups
    pub ELPNOOCAFBE: ::std::option::Option<cagdhenefnd::ELPNOOCAFBE>,
    // special fields
    // @@protoc_insertion_point(special_field:CAGDHENEFND.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CAGDHENEFND {
    fn default() -> &'a CAGDHENEFND {
        <CAGDHENEFND as ::protobuf::Message>::default_instance()
    }
}

impl CAGDHENEFND {
    pub fn new() -> CAGDHENEFND {
        ::std::default::Default::default()
    }

    // .IJBEDOLECAF IFLIEHLHEJC = 4;

    pub fn IFLIEHLHEJC(&self) -> &super::IJBEDOLECAF::IJBEDOLECAF {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(ref v)) => v,
            _ => <super::IJBEDOLECAF::IJBEDOLECAF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IFLIEHLHEJC(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_IFLIEHLHEJC(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IFLIEHLHEJC(&mut self, v: super::IJBEDOLECAF::IJBEDOLECAF) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IFLIEHLHEJC(&mut self) -> &mut super::IJBEDOLECAF::IJBEDOLECAF {
        if let ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(_)) = self.ELPNOOCAFBE {
        } else {
            self.ELPNOOCAFBE = ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(super::IJBEDOLECAF::IJBEDOLECAF::new()));
        }
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IFLIEHLHEJC(&mut self) -> super::IJBEDOLECAF::IJBEDOLECAF {
        if self.has_IFLIEHLHEJC() {
            match self.ELPNOOCAFBE.take() {
                ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IJBEDOLECAF::IJBEDOLECAF::new()
        }
    }

    // uint32 PLLPBJFPAPH = 11;

    pub fn PLLPBJFPAPH(&self) -> u32 {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_PLLPBJFPAPH(&mut self) {
        self.ELPNOOCAFBE = ::std::option::Option::None;
    }

    pub fn has_PLLPBJFPAPH(&self) -> bool {
        match self.ELPNOOCAFBE {
            ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PLLPBJFPAPH(&mut self, v: u32) {
        self.ELPNOOCAFBE = ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &CAGDHENEFND| { &m.KGGHLADEKGP },
            |m: &mut CAGDHENEFND| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ONALKMOMMEC",
            |m: &CAGDHENEFND| { &m.ONALKMOMMEC },
            |m: &mut CAGDHENEFND| { &mut m.ONALKMOMMEC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMMELAOHNGO",
            |m: &CAGDHENEFND| { &m.PMMELAOHNGO },
            |m: &mut CAGDHENEFND| { &mut m.PMMELAOHNGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IJBEDOLECAF::IJBEDOLECAF>(
            "IFLIEHLHEJC",
            CAGDHENEFND::has_IFLIEHLHEJC,
            CAGDHENEFND::IFLIEHLHEJC,
            CAGDHENEFND::mut_IFLIEHLHEJC,
            CAGDHENEFND::set_IFLIEHLHEJC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "PLLPBJFPAPH",
            CAGDHENEFND::has_PLLPBJFPAPH,
            CAGDHENEFND::PLLPBJFPAPH,
            CAGDHENEFND::set_PLLPBJFPAPH,
        ));
        oneofs.push(cagdhenefnd::ELPNOOCAFBE::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CAGDHENEFND>(
            "CAGDHENEFND",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CAGDHENEFND {
    const NAME: &'static str = "CAGDHENEFND";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.ONALKMOMMEC = is.read_uint32()?;
                },
                24 => {
                    self.PMMELAOHNGO = is.read_uint32()?;
                },
                34 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(is.read_message()?));
                },
                88 => {
                    self.ELPNOOCAFBE = ::std::option::Option::Some(cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(is.read_uint32()?));
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
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::GMFDCJMEIJM::GMFDCJMEIJM::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(12, self.KGGHLADEKGP.value());
        }
        if self.ONALKMOMMEC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ONALKMOMMEC);
        }
        if self.PMMELAOHNGO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.PMMELAOHNGO);
        }
        if let ::std::option::Option::Some(ref v) = self.ELPNOOCAFBE {
            match v {
                &cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(v) => {
                    my_size += ::protobuf::rt::uint32_size(11, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::GMFDCJMEIJM::GMFDCJMEIJM::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if self.ONALKMOMMEC != 0 {
            os.write_uint32(2, self.ONALKMOMMEC)?;
        }
        if self.PMMELAOHNGO != 0 {
            os.write_uint32(3, self.PMMELAOHNGO)?;
        }
        if let ::std::option::Option::Some(ref v) = self.ELPNOOCAFBE {
            match v {
                &cagdhenefnd::ELPNOOCAFBE::IFLIEHLHEJC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &cagdhenefnd::ELPNOOCAFBE::PLLPBJFPAPH(v) => {
                    os.write_uint32(11, v)?;
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

    fn new() -> CAGDHENEFND {
        CAGDHENEFND::new()
    }

    fn clear(&mut self) {
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::GMFDCJMEIJM::GMFDCJMEIJM::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT);
        self.ONALKMOMMEC = 0;
        self.PMMELAOHNGO = 0;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.ELPNOOCAFBE = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CAGDHENEFND {
        static instance: CAGDHENEFND = CAGDHENEFND {
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            ONALKMOMMEC: 0,
            PMMELAOHNGO: 0,
            ELPNOOCAFBE: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CAGDHENEFND {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CAGDHENEFND").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CAGDHENEFND {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CAGDHENEFND {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CAGDHENEFND`
pub mod cagdhenefnd {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CAGDHENEFND.ELPNOOCAFBE)
    pub enum ELPNOOCAFBE {
        // @@protoc_insertion_point(oneof_field:CAGDHENEFND.IFLIEHLHEJC)
        IFLIEHLHEJC(super::super::IJBEDOLECAF::IJBEDOLECAF),
        // @@protoc_insertion_point(oneof_field:CAGDHENEFND.PLLPBJFPAPH)
        PLLPBJFPAPH(u32),
    }

    impl ::protobuf::Oneof for ELPNOOCAFBE {
    }

    impl ::protobuf::OneofFull for ELPNOOCAFBE {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CAGDHENEFND as ::protobuf::MessageFull>::descriptor().oneof_by_name("ELPNOOCAFBE").unwrap()).clone()
        }
    }

    impl ELPNOOCAFBE {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<ELPNOOCAFBE>("ELPNOOCAFBE")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CAGDHENEFND.proto\x1a\x11GMFDCJMEIJM.proto\x1a\x11IJBEDOLECAF.prot\
    o\"\xe6\x01\n\x0bCAGDHENEFND\x12.\n\x0bKGGHLADEKGP\x18\x0c\x20\x01(\x0e2\
    \x0c.GMFDCJMEIJMR\x0bKGGHLADEKGP\x12\x20\n\x0bONALKMOMMEC\x18\x02\x20\
    \x01(\rR\x0bONALKMOMMEC\x12\x20\n\x0bPMMELAOHNGO\x18\x03\x20\x01(\rR\x0b\
    PMMELAOHNGO\x120\n\x0bIFLIEHLHEJC\x18\x04\x20\x01(\x0b2\x0c.IJBEDOLECAFH\
    \0R\x0bIFLIEHLHEJC\x12\"\n\x0bPLLPBJFPAPH\x18\x0b\x20\x01(\rH\0R\x0bPLLP\
    BJFPAPHB\r\n\x0bELPNOOCAFBEb\x06proto3\
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
            deps.push(super::GMFDCJMEIJM::file_descriptor().clone());
            deps.push(super::IJBEDOLECAF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CAGDHENEFND::generated_message_descriptor_data());
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