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

//! Generated file from `CJKCOAEOHCF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CJKCOAEOHCF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CJKCOAEOHCF {
    // message fields
    // @@protoc_insertion_point(field:CJKCOAEOHCF.BPMDFENIDBF)
    pub BPMDFENIDBF: u32,
    // @@protoc_insertion_point(field:CJKCOAEOHCF.ALIFPIHNMEK)
    pub ALIFPIHNMEK: ::protobuf::EnumOrUnknown<super::CAGLGPAOALO::CAGLGPAOALO>,
    // @@protoc_insertion_point(field:CJKCOAEOHCF.HIODKMAPOAE)
    pub HIODKMAPOAE: u32,
    // message oneof groups
    pub KFELKJLDKEH: ::std::option::Option<cjkcoaeohcf::KFELKJLDKEH>,
    // special fields
    // @@protoc_insertion_point(special_field:CJKCOAEOHCF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CJKCOAEOHCF {
    fn default() -> &'a CJKCOAEOHCF {
        <CJKCOAEOHCF as ::protobuf::Message>::default_instance()
    }
}

impl CJKCOAEOHCF {
    pub fn new() -> CJKCOAEOHCF {
        ::std::default::Default::default()
    }

    // .GPGGAINLLIN PCJLGHBIJCM = 14;

    pub fn PCJLGHBIJCM(&self) -> &super::GPGGAINLLIN::GPGGAINLLIN {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(ref v)) => v,
            _ => <super::GPGGAINLLIN::GPGGAINLLIN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_PCJLGHBIJCM(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_PCJLGHBIJCM(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_PCJLGHBIJCM(&mut self, v: super::GPGGAINLLIN::GPGGAINLLIN) {
        self.KFELKJLDKEH = ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(v))
    }

    // Mutable pointer to the field.
    pub fn mut_PCJLGHBIJCM(&mut self) -> &mut super::GPGGAINLLIN::GPGGAINLLIN {
        if let ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(_)) = self.KFELKJLDKEH {
        } else {
            self.KFELKJLDKEH = ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(super::GPGGAINLLIN::GPGGAINLLIN::new()));
        }
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_PCJLGHBIJCM(&mut self) -> super::GPGGAINLLIN::GPGGAINLLIN {
        if self.has_PCJLGHBIJCM() {
            match self.KFELKJLDKEH.take() {
                ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GPGGAINLLIN::GPGGAINLLIN::new()
        }
    }

    // uint32 HCAHEKMGEAN = 8;

    pub fn HCAHEKMGEAN(&self) -> u32 {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_HCAHEKMGEAN(&mut self) {
        self.KFELKJLDKEH = ::std::option::Option::None;
    }

    pub fn has_HCAHEKMGEAN(&self) -> bool {
        match self.KFELKJLDKEH {
            ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HCAHEKMGEAN(&mut self, v: u32) {
        self.KFELKJLDKEH = ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPMDFENIDBF",
            |m: &CJKCOAEOHCF| { &m.BPMDFENIDBF },
            |m: &mut CJKCOAEOHCF| { &mut m.BPMDFENIDBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALIFPIHNMEK",
            |m: &CJKCOAEOHCF| { &m.ALIFPIHNMEK },
            |m: &mut CJKCOAEOHCF| { &mut m.ALIFPIHNMEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIODKMAPOAE",
            |m: &CJKCOAEOHCF| { &m.HIODKMAPOAE },
            |m: &mut CJKCOAEOHCF| { &mut m.HIODKMAPOAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GPGGAINLLIN::GPGGAINLLIN>(
            "PCJLGHBIJCM",
            CJKCOAEOHCF::has_PCJLGHBIJCM,
            CJKCOAEOHCF::PCJLGHBIJCM,
            CJKCOAEOHCF::mut_PCJLGHBIJCM,
            CJKCOAEOHCF::set_PCJLGHBIJCM,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "HCAHEKMGEAN",
            CJKCOAEOHCF::has_HCAHEKMGEAN,
            CJKCOAEOHCF::HCAHEKMGEAN,
            CJKCOAEOHCF::set_HCAHEKMGEAN,
        ));
        oneofs.push(cjkcoaeohcf::KFELKJLDKEH::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CJKCOAEOHCF>(
            "CJKCOAEOHCF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CJKCOAEOHCF {
    const NAME: &'static str = "CJKCOAEOHCF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.BPMDFENIDBF = is.read_uint32()?;
                },
                120 => {
                    self.ALIFPIHNMEK = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.HIODKMAPOAE = is.read_uint32()?;
                },
                114 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(is.read_message()?));
                },
                64 => {
                    self.KFELKJLDKEH = ::std::option::Option::Some(cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(is.read_uint32()?));
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
        if self.BPMDFENIDBF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BPMDFENIDBF);
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::CAGLGPAOALO::CAGLGPAOALO::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(15, self.ALIFPIHNMEK.value());
        }
        if self.HIODKMAPOAE != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HIODKMAPOAE);
        }
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(v) => {
                    my_size += ::protobuf::rt::uint32_size(8, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BPMDFENIDBF != 0 {
            os.write_uint32(5, self.BPMDFENIDBF)?;
        }
        if self.ALIFPIHNMEK != ::protobuf::EnumOrUnknown::new(super::CAGLGPAOALO::CAGLGPAOALO::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.ALIFPIHNMEK))?;
        }
        if self.HIODKMAPOAE != 0 {
            os.write_uint32(2, self.HIODKMAPOAE)?;
        }
        if let ::std::option::Option::Some(ref v) = self.KFELKJLDKEH {
            match v {
                &cjkcoaeohcf::KFELKJLDKEH::PCJLGHBIJCM(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &cjkcoaeohcf::KFELKJLDKEH::HCAHEKMGEAN(v) => {
                    os.write_uint32(8, v)?;
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

    fn new() -> CJKCOAEOHCF {
        CJKCOAEOHCF::new()
    }

    fn clear(&mut self) {
        self.BPMDFENIDBF = 0;
        self.ALIFPIHNMEK = ::protobuf::EnumOrUnknown::new(super::CAGLGPAOALO::CAGLGPAOALO::SCENE_ENTITY_BUFF_CHANGE_TYPE_DEFAULT);
        self.HIODKMAPOAE = 0;
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.KFELKJLDKEH = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CJKCOAEOHCF {
        static instance: CJKCOAEOHCF = CJKCOAEOHCF {
            BPMDFENIDBF: 0,
            ALIFPIHNMEK: ::protobuf::EnumOrUnknown::from_i32(0),
            HIODKMAPOAE: 0,
            KFELKJLDKEH: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CJKCOAEOHCF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CJKCOAEOHCF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CJKCOAEOHCF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CJKCOAEOHCF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CJKCOAEOHCF`
pub mod cjkcoaeohcf {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:CJKCOAEOHCF.KFELKJLDKEH)
    pub enum KFELKJLDKEH {
        // @@protoc_insertion_point(oneof_field:CJKCOAEOHCF.PCJLGHBIJCM)
        PCJLGHBIJCM(super::super::GPGGAINLLIN::GPGGAINLLIN),
        // @@protoc_insertion_point(oneof_field:CJKCOAEOHCF.HCAHEKMGEAN)
        HCAHEKMGEAN(u32),
    }

    impl ::protobuf::Oneof for KFELKJLDKEH {
    }

    impl ::protobuf::OneofFull for KFELKJLDKEH {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::CJKCOAEOHCF as ::protobuf::MessageFull>::descriptor().oneof_by_name("KFELKJLDKEH").unwrap()).clone()
        }
    }

    impl KFELKJLDKEH {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<KFELKJLDKEH>("KFELKJLDKEH")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CJKCOAEOHCF.proto\x1a\x11CAGLGPAOALO.proto\x1a\x11GPGGAINLLIN.prot\
    o\"\xe6\x01\n\x0bCJKCOAEOHCF\x12\x20\n\x0bBPMDFENIDBF\x18\x05\x20\x01(\r\
    R\x0bBPMDFENIDBF\x12.\n\x0bALIFPIHNMEK\x18\x0f\x20\x01(\x0e2\x0c.CAGLGPA\
    OALOR\x0bALIFPIHNMEK\x12\x20\n\x0bHIODKMAPOAE\x18\x02\x20\x01(\rR\x0bHIO\
    DKMAPOAE\x120\n\x0bPCJLGHBIJCM\x18\x0e\x20\x01(\x0b2\x0c.GPGGAINLLINH\0R\
    \x0bPCJLGHBIJCM\x12\"\n\x0bHCAHEKMGEAN\x18\x08\x20\x01(\rH\0R\x0bHCAHEKM\
    GEANB\r\n\x0bKFELKJLDKEHb\x06proto3\
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
            deps.push(super::CAGLGPAOALO::file_descriptor().clone());
            deps.push(super::GPGGAINLLIN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CJKCOAEOHCF::generated_message_descriptor_data());
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
