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

//! Generated file from `JCHABMJKNAH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JCHABMJKNAH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JCHABMJKNAH {
    // message fields
    // @@protoc_insertion_point(field:JCHABMJKNAH.GOIGDICGLNM)
    pub GOIGDICGLNM: ::protobuf::MessageField<super::PHBMAJPCNLB::PHBMAJPCNLB>,
    // @@protoc_insertion_point(field:JCHABMJKNAH.MLFNPOILPCF)
    pub MLFNPOILPCF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JCHABMJKNAH.EICPPJJLGDK)
    pub EICPPJJLGDK: u32,
    // @@protoc_insertion_point(field:JCHABMJKNAH.GHLJDDLIGIM)
    pub GHLJDDLIGIM: ::protobuf::EnumOrUnknown<super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus>,
    // @@protoc_insertion_point(field:JCHABMJKNAH.PCJKENIBPFN)
    pub PCJKENIBPFN: ::std::vec::Vec<super::HODLNIOLGEA::HODLNIOLGEA>,
    // @@protoc_insertion_point(field:JCHABMJKNAH.EACJONNIHKP)
    pub EACJONNIHKP: u32,
    // @@protoc_insertion_point(field:JCHABMJKNAH.KGLPMNHMNMM)
    pub KGLPMNHMNMM: ::protobuf::MessageField<super::EJPLFEAMOMD::EJPLFEAMOMD>,
    // special fields
    // @@protoc_insertion_point(special_field:JCHABMJKNAH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JCHABMJKNAH {
    fn default() -> &'a JCHABMJKNAH {
        <JCHABMJKNAH as ::protobuf::Message>::default_instance()
    }
}

impl JCHABMJKNAH {
    pub fn new() -> JCHABMJKNAH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PHBMAJPCNLB::PHBMAJPCNLB>(
            "GOIGDICGLNM",
            |m: &JCHABMJKNAH| { &m.GOIGDICGLNM },
            |m: &mut JCHABMJKNAH| { &mut m.GOIGDICGLNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MLFNPOILPCF",
            |m: &JCHABMJKNAH| { &m.MLFNPOILPCF },
            |m: &mut JCHABMJKNAH| { &mut m.MLFNPOILPCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EICPPJJLGDK",
            |m: &JCHABMJKNAH| { &m.EICPPJJLGDK },
            |m: &mut JCHABMJKNAH| { &mut m.EICPPJJLGDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GHLJDDLIGIM",
            |m: &JCHABMJKNAH| { &m.GHLJDDLIGIM },
            |m: &mut JCHABMJKNAH| { &mut m.GHLJDDLIGIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCJKENIBPFN",
            |m: &JCHABMJKNAH| { &m.PCJKENIBPFN },
            |m: &mut JCHABMJKNAH| { &mut m.PCJKENIBPFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EACJONNIHKP",
            |m: &JCHABMJKNAH| { &m.EACJONNIHKP },
            |m: &mut JCHABMJKNAH| { &mut m.EACJONNIHKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EJPLFEAMOMD::EJPLFEAMOMD>(
            "KGLPMNHMNMM",
            |m: &JCHABMJKNAH| { &m.KGLPMNHMNMM },
            |m: &mut JCHABMJKNAH| { &mut m.KGLPMNHMNMM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JCHABMJKNAH>(
            "JCHABMJKNAH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JCHABMJKNAH {
    const NAME: &'static str = "JCHABMJKNAH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GOIGDICGLNM)?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.MLFNPOILPCF)?;
                },
                24 => {
                    self.MLFNPOILPCF.push(is.read_uint32()?);
                },
                56 => {
                    self.EICPPJJLGDK = is.read_uint32()?;
                },
                80 => {
                    self.GHLJDDLIGIM = is.read_enum_or_unknown()?;
                },
                106 => {
                    self.PCJKENIBPFN.push(is.read_message()?);
                },
                40 => {
                    self.EACJONNIHKP = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KGLPMNHMNMM)?;
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
        if let Some(v) = self.GOIGDICGLNM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.MLFNPOILPCF {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.EICPPJJLGDK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.EICPPJJLGDK);
        }
        if self.GHLJDDLIGIM != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            my_size += ::protobuf::rt::int32_size(10, self.GHLJDDLIGIM.value());
        }
        for value in &self.PCJKENIBPFN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.EACJONNIHKP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EACJONNIHKP);
        }
        if let Some(v) = self.KGLPMNHMNMM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.GOIGDICGLNM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.MLFNPOILPCF {
            os.write_uint32(3, *v)?;
        };
        if self.EICPPJJLGDK != 0 {
            os.write_uint32(7, self.EICPPJJLGDK)?;
        }
        if self.GHLJDDLIGIM != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.GHLJDDLIGIM))?;
        }
        for v in &self.PCJKENIBPFN {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.EACJONNIHKP != 0 {
            os.write_uint32(5, self.EACJONNIHKP)?;
        }
        if let Some(v) = self.KGLPMNHMNMM.as_ref() {
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

    fn new() -> JCHABMJKNAH {
        JCHABMJKNAH::new()
    }

    fn clear(&mut self) {
        self.GOIGDICGLNM.clear();
        self.MLFNPOILPCF.clear();
        self.EICPPJJLGDK = 0;
        self.GHLJDDLIGIM = ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE);
        self.PCJKENIBPFN.clear();
        self.EACJONNIHKP = 0;
        self.KGLPMNHMNMM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JCHABMJKNAH {
        static instance: JCHABMJKNAH = JCHABMJKNAH {
            GOIGDICGLNM: ::protobuf::MessageField::none(),
            MLFNPOILPCF: ::std::vec::Vec::new(),
            EICPPJJLGDK: 0,
            GHLJDDLIGIM: ::protobuf::EnumOrUnknown::from_i32(0),
            PCJKENIBPFN: ::std::vec::Vec::new(),
            EACJONNIHKP: 0,
            KGLPMNHMNMM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JCHABMJKNAH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JCHABMJKNAH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JCHABMJKNAH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JCHABMJKNAH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JCHABMJKNAH.proto\x1a\x1fChessRogueBoardCellStatus.proto\x1a\x11EJ\
    PLFEAMOMD.proto\x1a\x11HODLNIOLGEA.proto\x1a\x11PHBMAJPCNLB.proto\"\xc1\
    \x02\n\x0bJCHABMJKNAH\x12.\n\x0bGOIGDICGLNM\x18\x0f\x20\x01(\x0b2\x0c.PH\
    BMAJPCNLBR\x0bGOIGDICGLNM\x12\x20\n\x0bMLFNPOILPCF\x18\x03\x20\x03(\rR\
    \x0bMLFNPOILPCF\x12\x20\n\x0bEICPPJJLGDK\x18\x07\x20\x01(\rR\x0bEICPPJJL\
    GDK\x12<\n\x0bGHLJDDLIGIM\x18\n\x20\x01(\x0e2\x1a.ChessRogueBoardCellSta\
    tusR\x0bGHLJDDLIGIM\x12.\n\x0bPCJKENIBPFN\x18\r\x20\x03(\x0b2\x0c.HODLNI\
    OLGEAR\x0bPCJKENIBPFN\x12\x20\n\x0bEACJONNIHKP\x18\x05\x20\x01(\rR\x0bEA\
    CJONNIHKP\x12.\n\x0bKGLPMNHMNMM\x18\x02\x20\x01(\x0b2\x0c.EJPLFEAMOMDR\
    \x0bKGLPMNHMNMMb\x06proto3\
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
            deps.push(super::ChessRogueBoardCellStatus::file_descriptor().clone());
            deps.push(super::EJPLFEAMOMD::file_descriptor().clone());
            deps.push(super::HODLNIOLGEA::file_descriptor().clone());
            deps.push(super::PHBMAJPCNLB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JCHABMJKNAH::generated_message_descriptor_data());
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