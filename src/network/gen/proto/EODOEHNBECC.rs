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

//! Generated file from `EODOEHNBECC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EODOEHNBECC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EODOEHNBECC {
    // message fields
    // @@protoc_insertion_point(field:EODOEHNBECC.DBBGAIADLNC)
    pub DBBGAIADLNC: bool,
    // @@protoc_insertion_point(field:EODOEHNBECC.OMPAAKLLLFD)
    pub OMPAAKLLLFD: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.GGPBFFEJPMG)
    pub GGPBFFEJPMG: ::protobuf::EnumOrUnknown<super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType>,
    // @@protoc_insertion_point(field:EODOEHNBECC.HBJMACNOHIO)
    pub HBJMACNOHIO: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.EDBGHNILGNM)
    pub EDBGHNILGNM: ::protobuf::MessageField<super::CGMLGCGMBDH::CGMLGCGMBDH>,
    // @@protoc_insertion_point(field:EODOEHNBECC.CCAMMHFPPGF)
    pub CCAMMHFPPGF: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.ABHPIGOGACI)
    pub ABHPIGOGACI: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.EOHLIDIKJJN)
    pub EOHLIDIKJJN: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.KGALGMDJIDD)
    pub KGALGMDJIDD: ::std::vec::Vec<super::CEEPCDDKLKA::CEEPCDDKLKA>,
    // @@protoc_insertion_point(field:EODOEHNBECC.MMHNOILFPPO)
    pub MMHNOILFPPO: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.PBNKIABMPPH)
    pub PBNKIABMPPH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EODOEHNBECC.IAFBOKOOAIL)
    pub IAFBOKOOAIL: u32,
    // @@protoc_insertion_point(field:EODOEHNBECC.APKMPFBDDLB)
    pub APKMPFBDDLB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EODOEHNBECC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EODOEHNBECC {
    fn default() -> &'a EODOEHNBECC {
        <EODOEHNBECC as ::protobuf::Message>::default_instance()
    }
}

impl EODOEHNBECC {
    pub fn new() -> EODOEHNBECC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBBGAIADLNC",
            |m: &EODOEHNBECC| { &m.DBBGAIADLNC },
            |m: &mut EODOEHNBECC| { &mut m.DBBGAIADLNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMPAAKLLLFD",
            |m: &EODOEHNBECC| { &m.OMPAAKLLLFD },
            |m: &mut EODOEHNBECC| { &mut m.OMPAAKLLLFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGPBFFEJPMG",
            |m: &EODOEHNBECC| { &m.GGPBFFEJPMG },
            |m: &mut EODOEHNBECC| { &mut m.GGPBFFEJPMG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBJMACNOHIO",
            |m: &EODOEHNBECC| { &m.HBJMACNOHIO },
            |m: &mut EODOEHNBECC| { &mut m.HBJMACNOHIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CGMLGCGMBDH::CGMLGCGMBDH>(
            "EDBGHNILGNM",
            |m: &EODOEHNBECC| { &m.EDBGHNILGNM },
            |m: &mut EODOEHNBECC| { &mut m.EDBGHNILGNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCAMMHFPPGF",
            |m: &EODOEHNBECC| { &m.CCAMMHFPPGF },
            |m: &mut EODOEHNBECC| { &mut m.CCAMMHFPPGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ABHPIGOGACI",
            |m: &EODOEHNBECC| { &m.ABHPIGOGACI },
            |m: &mut EODOEHNBECC| { &mut m.ABHPIGOGACI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EOHLIDIKJJN",
            |m: &EODOEHNBECC| { &m.EOHLIDIKJJN },
            |m: &mut EODOEHNBECC| { &mut m.EOHLIDIKJJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGALGMDJIDD",
            |m: &EODOEHNBECC| { &m.KGALGMDJIDD },
            |m: &mut EODOEHNBECC| { &mut m.KGALGMDJIDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMHNOILFPPO",
            |m: &EODOEHNBECC| { &m.MMHNOILFPPO },
            |m: &mut EODOEHNBECC| { &mut m.MMHNOILFPPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PBNKIABMPPH",
            |m: &EODOEHNBECC| { &m.PBNKIABMPPH },
            |m: &mut EODOEHNBECC| { &mut m.PBNKIABMPPH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IAFBOKOOAIL",
            |m: &EODOEHNBECC| { &m.IAFBOKOOAIL },
            |m: &mut EODOEHNBECC| { &mut m.IAFBOKOOAIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APKMPFBDDLB",
            |m: &EODOEHNBECC| { &m.APKMPFBDDLB },
            |m: &mut EODOEHNBECC| { &mut m.APKMPFBDDLB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EODOEHNBECC>(
            "EODOEHNBECC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EODOEHNBECC {
    const NAME: &'static str = "EODOEHNBECC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.DBBGAIADLNC = is.read_bool()?;
                },
                24 => {
                    self.OMPAAKLLLFD = is.read_uint32()?;
                },
                112 => {
                    self.GGPBFFEJPMG = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.HBJMACNOHIO = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EDBGHNILGNM)?;
                },
                40 => {
                    self.CCAMMHFPPGF = is.read_uint32()?;
                },
                16 => {
                    self.ABHPIGOGACI = is.read_uint32()?;
                },
                48 => {
                    self.EOHLIDIKJJN = is.read_uint32()?;
                },
                58 => {
                    self.KGALGMDJIDD.push(is.read_message()?);
                },
                96 => {
                    self.MMHNOILFPPO = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.PBNKIABMPPH)?;
                },
                72 => {
                    self.PBNKIABMPPH.push(is.read_uint32()?);
                },
                120 => {
                    self.IAFBOKOOAIL = is.read_uint32()?;
                },
                32 => {
                    self.APKMPFBDDLB = is.read_uint32()?;
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
        if self.DBBGAIADLNC != false {
            my_size += 1 + 1;
        }
        if self.OMPAAKLLLFD != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OMPAAKLLLFD);
        }
        if self.GGPBFFEJPMG != ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.GGPBFFEJPMG.value());
        }
        if self.HBJMACNOHIO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.HBJMACNOHIO);
        }
        if let Some(v) = self.EDBGHNILGNM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.CCAMMHFPPGF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.CCAMMHFPPGF);
        }
        if self.ABHPIGOGACI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ABHPIGOGACI);
        }
        if self.EOHLIDIKJJN != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.EOHLIDIKJJN);
        }
        for value in &self.KGALGMDJIDD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MMHNOILFPPO != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MMHNOILFPPO);
        }
        for value in &self.PBNKIABMPPH {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if self.IAFBOKOOAIL != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.IAFBOKOOAIL);
        }
        if self.APKMPFBDDLB != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.APKMPFBDDLB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DBBGAIADLNC != false {
            os.write_bool(10, self.DBBGAIADLNC)?;
        }
        if self.OMPAAKLLLFD != 0 {
            os.write_uint32(3, self.OMPAAKLLLFD)?;
        }
        if self.GGPBFFEJPMG != ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.GGPBFFEJPMG))?;
        }
        if self.HBJMACNOHIO != 0 {
            os.write_uint32(13, self.HBJMACNOHIO)?;
        }
        if let Some(v) = self.EDBGHNILGNM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.CCAMMHFPPGF != 0 {
            os.write_uint32(5, self.CCAMMHFPPGF)?;
        }
        if self.ABHPIGOGACI != 0 {
            os.write_uint32(2, self.ABHPIGOGACI)?;
        }
        if self.EOHLIDIKJJN != 0 {
            os.write_uint32(6, self.EOHLIDIKJJN)?;
        }
        for v in &self.KGALGMDJIDD {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.MMHNOILFPPO != 0 {
            os.write_uint32(12, self.MMHNOILFPPO)?;
        }
        for v in &self.PBNKIABMPPH {
            os.write_uint32(9, *v)?;
        };
        if self.IAFBOKOOAIL != 0 {
            os.write_uint32(15, self.IAFBOKOOAIL)?;
        }
        if self.APKMPFBDDLB != 0 {
            os.write_uint32(4, self.APKMPFBDDLB)?;
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

    fn new() -> EODOEHNBECC {
        EODOEHNBECC::new()
    }

    fn clear(&mut self) {
        self.DBBGAIADLNC = false;
        self.OMPAAKLLLFD = 0;
        self.GGPBFFEJPMG = ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE);
        self.HBJMACNOHIO = 0;
        self.EDBGHNILGNM.clear();
        self.CCAMMHFPPGF = 0;
        self.ABHPIGOGACI = 0;
        self.EOHLIDIKJJN = 0;
        self.KGALGMDJIDD.clear();
        self.MMHNOILFPPO = 0;
        self.PBNKIABMPPH.clear();
        self.IAFBOKOOAIL = 0;
        self.APKMPFBDDLB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EODOEHNBECC {
        static instance: EODOEHNBECC = EODOEHNBECC {
            DBBGAIADLNC: false,
            OMPAAKLLLFD: 0,
            GGPBFFEJPMG: ::protobuf::EnumOrUnknown::from_i32(0),
            HBJMACNOHIO: 0,
            EDBGHNILGNM: ::protobuf::MessageField::none(),
            CCAMMHFPPGF: 0,
            ABHPIGOGACI: 0,
            EOHLIDIKJJN: 0,
            KGALGMDJIDD: ::std::vec::Vec::new(),
            MMHNOILFPPO: 0,
            PBNKIABMPPH: ::std::vec::Vec::new(),
            IAFBOKOOAIL: 0,
            APKMPFBDDLB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EODOEHNBECC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EODOEHNBECC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EODOEHNBECC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EODOEHNBECC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EODOEHNBECC.proto\x1a\x11CEEPCDDKLKA.proto\x1a\x11CGMLGCGMBDH.prot\
    o\x1a%RogueCommonBuffSelectSourceType.proto\"\x85\x04\n\x0bEODOEHNBECC\
    \x12\x20\n\x0bDBBGAIADLNC\x18\n\x20\x01(\x08R\x0bDBBGAIADLNC\x12\x20\n\
    \x0bOMPAAKLLLFD\x18\x03\x20\x01(\rR\x0bOMPAAKLLLFD\x12B\n\x0bGGPBFFEJPMG\
    \x18\x0e\x20\x01(\x0e2\x20.RogueCommonBuffSelectSourceTypeR\x0bGGPBFFEJP\
    MG\x12\x20\n\x0bHBJMACNOHIO\x18\r\x20\x01(\rR\x0bHBJMACNOHIO\x12.\n\x0bE\
    DBGHNILGNM\x18\x08\x20\x01(\x0b2\x0c.CGMLGCGMBDHR\x0bEDBGHNILGNM\x12\x20\
    \n\x0bCCAMMHFPPGF\x18\x05\x20\x01(\rR\x0bCCAMMHFPPGF\x12\x20\n\x0bABHPIG\
    OGACI\x18\x02\x20\x01(\rR\x0bABHPIGOGACI\x12\x20\n\x0bEOHLIDIKJJN\x18\
    \x06\x20\x01(\rR\x0bEOHLIDIKJJN\x12.\n\x0bKGALGMDJIDD\x18\x07\x20\x03(\
    \x0b2\x0c.CEEPCDDKLKAR\x0bKGALGMDJIDD\x12\x20\n\x0bMMHNOILFPPO\x18\x0c\
    \x20\x01(\rR\x0bMMHNOILFPPO\x12\x20\n\x0bPBNKIABMPPH\x18\t\x20\x03(\rR\
    \x0bPBNKIABMPPH\x12\x20\n\x0bIAFBOKOOAIL\x18\x0f\x20\x01(\rR\x0bIAFBOKOO\
    AIL\x12\x20\n\x0bAPKMPFBDDLB\x18\x04\x20\x01(\rR\x0bAPKMPFBDDLBb\x06prot\
    o3\
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
            deps.push(super::CEEPCDDKLKA::file_descriptor().clone());
            deps.push(super::CGMLGCGMBDH::file_descriptor().clone());
            deps.push(super::RogueCommonBuffSelectSourceType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EODOEHNBECC::generated_message_descriptor_data());
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
