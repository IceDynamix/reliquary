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

//! Generated file from `JHMALOGKGBL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JHMALOGKGBL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JHMALOGKGBL {
    // message fields
    // @@protoc_insertion_point(field:JHMALOGKGBL.BJONNKPCHCM)
    pub BJONNKPCHCM: bool,
    // @@protoc_insertion_point(field:JHMALOGKGBL.GOHGIEMLNOM)
    pub GOHGIEMLNOM: u32,
    // @@protoc_insertion_point(field:JHMALOGKGBL.step)
    pub step: ::protobuf::EnumOrUnknown<super::IKHDNPGOKFG::IKHDNPGOKFG>,
    // @@protoc_insertion_point(field:JHMALOGKGBL.JDBLLKCADAN)
    pub JDBLLKCADAN: bool,
    // @@protoc_insertion_point(field:JHMALOGKGBL.LEMIJFFKLGI)
    pub LEMIJFFKLGI: ::protobuf::EnumOrUnknown<super::LICPIMFIDGF::LICPIMFIDGF>,
    // special fields
    // @@protoc_insertion_point(special_field:JHMALOGKGBL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JHMALOGKGBL {
    fn default() -> &'a JHMALOGKGBL {
        <JHMALOGKGBL as ::protobuf::Message>::default_instance()
    }
}

impl JHMALOGKGBL {
    pub fn new() -> JHMALOGKGBL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJONNKPCHCM",
            |m: &JHMALOGKGBL| { &m.BJONNKPCHCM },
            |m: &mut JHMALOGKGBL| { &mut m.BJONNKPCHCM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOHGIEMLNOM",
            |m: &JHMALOGKGBL| { &m.GOHGIEMLNOM },
            |m: &mut JHMALOGKGBL| { &mut m.GOHGIEMLNOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &JHMALOGKGBL| { &m.step },
            |m: &mut JHMALOGKGBL| { &mut m.step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JDBLLKCADAN",
            |m: &JHMALOGKGBL| { &m.JDBLLKCADAN },
            |m: &mut JHMALOGKGBL| { &mut m.JDBLLKCADAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEMIJFFKLGI",
            |m: &JHMALOGKGBL| { &m.LEMIJFFKLGI },
            |m: &mut JHMALOGKGBL| { &mut m.LEMIJFFKLGI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JHMALOGKGBL>(
            "JHMALOGKGBL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JHMALOGKGBL {
    const NAME: &'static str = "JHMALOGKGBL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.BJONNKPCHCM = is.read_bool()?;
                },
                112 => {
                    self.GOHGIEMLNOM = is.read_uint32()?;
                },
                32 => {
                    self.step = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.JDBLLKCADAN = is.read_bool()?;
                },
                56 => {
                    self.LEMIJFFKLGI = is.read_enum_or_unknown()?;
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
        if self.BJONNKPCHCM != false {
            my_size += 1 + 1;
        }
        if self.GOHGIEMLNOM != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GOHGIEMLNOM);
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::IKHDNPGOKFG::IKHDNPGOKFG::HEART_DIAL_STEP_TYPE_MISSING) {
            my_size += ::protobuf::rt::int32_size(4, self.step.value());
        }
        if self.JDBLLKCADAN != false {
            my_size += 1 + 1;
        }
        if self.LEMIJFFKLGI != ::protobuf::EnumOrUnknown::new(super::LICPIMFIDGF::LICPIMFIDGF::HEART_DIAL_EMOTION_TYPE_PEACE) {
            my_size += ::protobuf::rt::int32_size(7, self.LEMIJFFKLGI.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BJONNKPCHCM != false {
            os.write_bool(3, self.BJONNKPCHCM)?;
        }
        if self.GOHGIEMLNOM != 0 {
            os.write_uint32(14, self.GOHGIEMLNOM)?;
        }
        if self.step != ::protobuf::EnumOrUnknown::new(super::IKHDNPGOKFG::IKHDNPGOKFG::HEART_DIAL_STEP_TYPE_MISSING) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.step))?;
        }
        if self.JDBLLKCADAN != false {
            os.write_bool(10, self.JDBLLKCADAN)?;
        }
        if self.LEMIJFFKLGI != ::protobuf::EnumOrUnknown::new(super::LICPIMFIDGF::LICPIMFIDGF::HEART_DIAL_EMOTION_TYPE_PEACE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.LEMIJFFKLGI))?;
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

    fn new() -> JHMALOGKGBL {
        JHMALOGKGBL::new()
    }

    fn clear(&mut self) {
        self.BJONNKPCHCM = false;
        self.GOHGIEMLNOM = 0;
        self.step = ::protobuf::EnumOrUnknown::new(super::IKHDNPGOKFG::IKHDNPGOKFG::HEART_DIAL_STEP_TYPE_MISSING);
        self.JDBLLKCADAN = false;
        self.LEMIJFFKLGI = ::protobuf::EnumOrUnknown::new(super::LICPIMFIDGF::LICPIMFIDGF::HEART_DIAL_EMOTION_TYPE_PEACE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JHMALOGKGBL {
        static instance: JHMALOGKGBL = JHMALOGKGBL {
            BJONNKPCHCM: false,
            GOHGIEMLNOM: 0,
            step: ::protobuf::EnumOrUnknown::from_i32(0),
            JDBLLKCADAN: false,
            LEMIJFFKLGI: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JHMALOGKGBL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JHMALOGKGBL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JHMALOGKGBL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JHMALOGKGBL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JHMALOGKGBL.proto\x1a\x11IKHDNPGOKFG.proto\x1a\x11LICPIMFIDGF.prot\
    o\"\xc5\x01\n\x0bJHMALOGKGBL\x12\x20\n\x0bBJONNKPCHCM\x18\x03\x20\x01(\
    \x08R\x0bBJONNKPCHCM\x12\x20\n\x0bGOHGIEMLNOM\x18\x0e\x20\x01(\rR\x0bGOH\
    GIEMLNOM\x12\x20\n\x04step\x18\x04\x20\x01(\x0e2\x0c.IKHDNPGOKFGR\x04ste\
    p\x12\x20\n\x0bJDBLLKCADAN\x18\n\x20\x01(\x08R\x0bJDBLLKCADAN\x12.\n\x0b\
    LEMIJFFKLGI\x18\x07\x20\x01(\x0e2\x0c.LICPIMFIDGFR\x0bLEMIJFFKLGIb\x06pr\
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
            deps.push(super::IKHDNPGOKFG::file_descriptor().clone());
            deps.push(super::LICPIMFIDGF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JHMALOGKGBL::generated_message_descriptor_data());
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