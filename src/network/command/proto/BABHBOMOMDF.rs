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

//! Generated file from `BABHBOMOMDF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BABHBOMOMDF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BABHBOMOMDF {
    // message fields
    // @@protoc_insertion_point(field:BABHBOMOMDF.MNFJEIININL)
    pub MNFJEIININL: ::std::vec::Vec<super::LHOMIEDAOID::LHOMIEDAOID>,
    // @@protoc_insertion_point(field:BABHBOMOMDF.status)
    pub status: ::protobuf::EnumOrUnknown<super::ODBNIGDLNCF::ODBNIGDLNCF>,
    // @@protoc_insertion_point(field:BABHBOMOMDF.layer_id)
    pub layer_id: u32,
    // @@protoc_insertion_point(field:BABHBOMOMDF.BHPGJCICMJM)
    pub BHPGJCICMJM: u32,
    // @@protoc_insertion_point(field:BABHBOMOMDF.OGLDNEFKNDO)
    pub OGLDNEFKNDO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BABHBOMOMDF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BABHBOMOMDF {
    fn default() -> &'a BABHBOMOMDF {
        <BABHBOMOMDF as ::protobuf::Message>::default_instance()
    }
}

impl BABHBOMOMDF {
    pub fn new() -> BABHBOMOMDF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MNFJEIININL",
            |m: &BABHBOMOMDF| { &m.MNFJEIININL },
            |m: &mut BABHBOMOMDF| { &mut m.MNFJEIININL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &BABHBOMOMDF| { &m.status },
            |m: &mut BABHBOMOMDF| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "layer_id",
            |m: &BABHBOMOMDF| { &m.layer_id },
            |m: &mut BABHBOMOMDF| { &mut m.layer_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHPGJCICMJM",
            |m: &BABHBOMOMDF| { &m.BHPGJCICMJM },
            |m: &mut BABHBOMOMDF| { &mut m.BHPGJCICMJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGLDNEFKNDO",
            |m: &BABHBOMOMDF| { &m.OGLDNEFKNDO },
            |m: &mut BABHBOMOMDF| { &mut m.OGLDNEFKNDO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BABHBOMOMDF>(
            "BABHBOMOMDF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BABHBOMOMDF {
    const NAME: &'static str = "BABHBOMOMDF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.MNFJEIININL.push(is.read_message()?);
                },
                120 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.layer_id = is.read_uint32()?;
                },
                32 => {
                    self.BHPGJCICMJM = is.read_uint32()?;
                },
                24 => {
                    self.OGLDNEFKNDO = is.read_uint32()?;
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
        for value in &self.MNFJEIININL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::ODBNIGDLNCF::ODBNIGDLNCF::ROGUE_TOURN_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.status.value());
        }
        if self.layer_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.layer_id);
        }
        if self.BHPGJCICMJM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BHPGJCICMJM);
        }
        if self.OGLDNEFKNDO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OGLDNEFKNDO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MNFJEIININL {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::ODBNIGDLNCF::ODBNIGDLNCF::ROGUE_TOURN_LAYER_STATUS_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.layer_id != 0 {
            os.write_uint32(2, self.layer_id)?;
        }
        if self.BHPGJCICMJM != 0 {
            os.write_uint32(4, self.BHPGJCICMJM)?;
        }
        if self.OGLDNEFKNDO != 0 {
            os.write_uint32(3, self.OGLDNEFKNDO)?;
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

    fn new() -> BABHBOMOMDF {
        BABHBOMOMDF::new()
    }

    fn clear(&mut self) {
        self.MNFJEIININL.clear();
        self.status = ::protobuf::EnumOrUnknown::new(super::ODBNIGDLNCF::ODBNIGDLNCF::ROGUE_TOURN_LAYER_STATUS_NONE);
        self.layer_id = 0;
        self.BHPGJCICMJM = 0;
        self.OGLDNEFKNDO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BABHBOMOMDF {
        static instance: BABHBOMOMDF = BABHBOMOMDF {
            MNFJEIININL: ::std::vec::Vec::new(),
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            layer_id: 0,
            BHPGJCICMJM: 0,
            OGLDNEFKNDO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BABHBOMOMDF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BABHBOMOMDF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BABHBOMOMDF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BABHBOMOMDF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BABHBOMOMDF.proto\x1a\x11LHOMIEDAOID.proto\x1a\x11ODBNIGDLNCF.prot\
    o\"\xc2\x01\n\x0bBABHBOMOMDF\x12.\n\x0bMNFJEIININL\x18\x06\x20\x03(\x0b2\
    \x0c.LHOMIEDAOIDR\x0bMNFJEIININL\x12$\n\x06status\x18\x0f\x20\x01(\x0e2\
    \x0c.ODBNIGDLNCFR\x06status\x12\x19\n\x08layer_id\x18\x02\x20\x01(\rR\
    \x07layerId\x12\x20\n\x0bBHPGJCICMJM\x18\x04\x20\x01(\rR\x0bBHPGJCICMJM\
    \x12\x20\n\x0bOGLDNEFKNDO\x18\x03\x20\x01(\rR\x0bOGLDNEFKNDOb\x06proto3\
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
            deps.push(super::LHOMIEDAOID::file_descriptor().clone());
            deps.push(super::ODBNIGDLNCF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BABHBOMOMDF::generated_message_descriptor_data());
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
