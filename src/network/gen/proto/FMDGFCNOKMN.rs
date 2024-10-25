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

//! Generated file from `FMDGFCNOKMN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FMDGFCNOKMN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FMDGFCNOKMN {
    // message fields
    // @@protoc_insertion_point(field:FMDGFCNOKMN.LOIEFKHFOBE)
    pub LOIEFKHFOBE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:FMDGFCNOKMN.ICECOGMBECA)
    pub ICECOGMBECA: u32,
    // @@protoc_insertion_point(field:FMDGFCNOKMN.OLDHNGGKABL)
    pub OLDHNGGKABL: ::std::vec::Vec<super::OPEMMBHKCFD::OPEMMBHKCFD>,
    // special fields
    // @@protoc_insertion_point(special_field:FMDGFCNOKMN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FMDGFCNOKMN {
    fn default() -> &'a FMDGFCNOKMN {
        <FMDGFCNOKMN as ::protobuf::Message>::default_instance()
    }
}

impl FMDGFCNOKMN {
    pub fn new() -> FMDGFCNOKMN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LOIEFKHFOBE",
            |m: &FMDGFCNOKMN| { &m.LOIEFKHFOBE },
            |m: &mut FMDGFCNOKMN| { &mut m.LOIEFKHFOBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICECOGMBECA",
            |m: &FMDGFCNOKMN| { &m.ICECOGMBECA },
            |m: &mut FMDGFCNOKMN| { &mut m.ICECOGMBECA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OLDHNGGKABL",
            |m: &FMDGFCNOKMN| { &m.OLDHNGGKABL },
            |m: &mut FMDGFCNOKMN| { &mut m.OLDHNGGKABL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FMDGFCNOKMN>(
            "FMDGFCNOKMN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FMDGFCNOKMN {
    const NAME: &'static str = "FMDGFCNOKMN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.LOIEFKHFOBE)?;
                },
                40 => {
                    self.LOIEFKHFOBE.push(is.read_uint32()?);
                },
                96 => {
                    self.ICECOGMBECA = is.read_uint32()?;
                },
                26 => {
                    self.OLDHNGGKABL.push(is.read_message()?);
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
        for value in &self.LOIEFKHFOBE {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.ICECOGMBECA != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.ICECOGMBECA);
        }
        for value in &self.OLDHNGGKABL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LOIEFKHFOBE {
            os.write_uint32(5, *v)?;
        };
        if self.ICECOGMBECA != 0 {
            os.write_uint32(12, self.ICECOGMBECA)?;
        }
        for v in &self.OLDHNGGKABL {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FMDGFCNOKMN {
        FMDGFCNOKMN::new()
    }

    fn clear(&mut self) {
        self.LOIEFKHFOBE.clear();
        self.ICECOGMBECA = 0;
        self.OLDHNGGKABL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FMDGFCNOKMN {
        static instance: FMDGFCNOKMN = FMDGFCNOKMN {
            LOIEFKHFOBE: ::std::vec::Vec::new(),
            ICECOGMBECA: 0,
            OLDHNGGKABL: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FMDGFCNOKMN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FMDGFCNOKMN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FMDGFCNOKMN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FMDGFCNOKMN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FMDGFCNOKMN.proto\x1a\x11OPEMMBHKCFD.proto\"\x81\x01\n\x0bFMDGFCNO\
    KMN\x12\x20\n\x0bLOIEFKHFOBE\x18\x05\x20\x03(\rR\x0bLOIEFKHFOBE\x12\x20\
    \n\x0bICECOGMBECA\x18\x0c\x20\x01(\rR\x0bICECOGMBECA\x12.\n\x0bOLDHNGGKA\
    BL\x18\x03\x20\x03(\x0b2\x0c.OPEMMBHKCFDR\x0bOLDHNGGKABLb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::OPEMMBHKCFD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FMDGFCNOKMN::generated_message_descriptor_data());
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