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

//! Generated file from `CDNGNDNLNAJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CDNGNDNLNAJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDNGNDNLNAJ {
    // message fields
    // @@protoc_insertion_point(field:CDNGNDNLNAJ.AAKHCNEDBCD)
    pub AAKHCNEDBCD: ::protobuf::MessageField<super::LHLEEHCBMOL::LHLEEHCBMOL>,
    // @@protoc_insertion_point(field:CDNGNDNLNAJ.HBHEAJIJEGF)
    pub HBHEAJIJEGF: ::protobuf::MessageField<super::CGCONJFFFBB::CGCONJFFFBB>,
    // @@protoc_insertion_point(field:CDNGNDNLNAJ.EEIHDCPOLEF)
    pub EEIHDCPOLEF: ::protobuf::MessageField<super::EDFABKMNBLI::EDFABKMNBLI>,
    // @@protoc_insertion_point(field:CDNGNDNLNAJ.GONEAKBDGEK)
    pub GONEAKBDGEK: u32,
    // @@protoc_insertion_point(field:CDNGNDNLNAJ.GBFCLMLIMHC)
    pub GBFCLMLIMHC: ::protobuf::MessageField<super::GCFEHMENONM::GCFEHMENONM>,
    // special fields
    // @@protoc_insertion_point(special_field:CDNGNDNLNAJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDNGNDNLNAJ {
    fn default() -> &'a CDNGNDNLNAJ {
        <CDNGNDNLNAJ as ::protobuf::Message>::default_instance()
    }
}

impl CDNGNDNLNAJ {
    pub fn new() -> CDNGNDNLNAJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LHLEEHCBMOL::LHLEEHCBMOL>(
            "AAKHCNEDBCD",
            |m: &CDNGNDNLNAJ| { &m.AAKHCNEDBCD },
            |m: &mut CDNGNDNLNAJ| { &mut m.AAKHCNEDBCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CGCONJFFFBB::CGCONJFFFBB>(
            "HBHEAJIJEGF",
            |m: &CDNGNDNLNAJ| { &m.HBHEAJIJEGF },
            |m: &mut CDNGNDNLNAJ| { &mut m.HBHEAJIJEGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EDFABKMNBLI::EDFABKMNBLI>(
            "EEIHDCPOLEF",
            |m: &CDNGNDNLNAJ| { &m.EEIHDCPOLEF },
            |m: &mut CDNGNDNLNAJ| { &mut m.EEIHDCPOLEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GONEAKBDGEK",
            |m: &CDNGNDNLNAJ| { &m.GONEAKBDGEK },
            |m: &mut CDNGNDNLNAJ| { &mut m.GONEAKBDGEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GCFEHMENONM::GCFEHMENONM>(
            "GBFCLMLIMHC",
            |m: &CDNGNDNLNAJ| { &m.GBFCLMLIMHC },
            |m: &mut CDNGNDNLNAJ| { &mut m.GBFCLMLIMHC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDNGNDNLNAJ>(
            "CDNGNDNLNAJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDNGNDNLNAJ {
    const NAME: &'static str = "CDNGNDNLNAJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AAKHCNEDBCD)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HBHEAJIJEGF)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EEIHDCPOLEF)?;
                },
                112 => {
                    self.GONEAKBDGEK = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GBFCLMLIMHC)?;
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
        if let Some(v) = self.AAKHCNEDBCD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HBHEAJIJEGF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EEIHDCPOLEF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.GONEAKBDGEK != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GONEAKBDGEK);
        }
        if let Some(v) = self.GBFCLMLIMHC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.AAKHCNEDBCD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.HBHEAJIJEGF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.EEIHDCPOLEF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.GONEAKBDGEK != 0 {
            os.write_uint32(14, self.GONEAKBDGEK)?;
        }
        if let Some(v) = self.GBFCLMLIMHC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> CDNGNDNLNAJ {
        CDNGNDNLNAJ::new()
    }

    fn clear(&mut self) {
        self.AAKHCNEDBCD.clear();
        self.HBHEAJIJEGF.clear();
        self.EEIHDCPOLEF.clear();
        self.GONEAKBDGEK = 0;
        self.GBFCLMLIMHC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDNGNDNLNAJ {
        static instance: CDNGNDNLNAJ = CDNGNDNLNAJ {
            AAKHCNEDBCD: ::protobuf::MessageField::none(),
            HBHEAJIJEGF: ::protobuf::MessageField::none(),
            EEIHDCPOLEF: ::protobuf::MessageField::none(),
            GONEAKBDGEK: 0,
            GBFCLMLIMHC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDNGNDNLNAJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDNGNDNLNAJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDNGNDNLNAJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDNGNDNLNAJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CDNGNDNLNAJ.proto\x1a\x11CGCONJFFFBB.proto\x1a\x11EDFABKMNBLI.prot\
    o\x1a\x11GCFEHMENONM.proto\x1a\x11LHLEEHCBMOL.proto\"\xef\x01\n\x0bCDNGN\
    DNLNAJ\x12.\n\x0bAAKHCNEDBCD\x18\x08\x20\x01(\x0b2\x0c.LHLEEHCBMOLR\x0bA\
    AKHCNEDBCD\x12.\n\x0bHBHEAJIJEGF\x18\x04\x20\x01(\x0b2\x0c.CGCONJFFFBBR\
    \x0bHBHEAJIJEGF\x12.\n\x0bEEIHDCPOLEF\x18\x03\x20\x01(\x0b2\x0c.EDFABKMN\
    BLIR\x0bEEIHDCPOLEF\x12\x20\n\x0bGONEAKBDGEK\x18\x0e\x20\x01(\rR\x0bGONE\
    AKBDGEK\x12.\n\x0bGBFCLMLIMHC\x18\x02\x20\x01(\x0b2\x0c.GCFEHMENONMR\x0b\
    GBFCLMLIMHCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::CGCONJFFFBB::file_descriptor().clone());
            deps.push(super::EDFABKMNBLI::file_descriptor().clone());
            deps.push(super::GCFEHMENONM::file_descriptor().clone());
            deps.push(super::LHLEEHCBMOL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CDNGNDNLNAJ::generated_message_descriptor_data());
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
