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

//! Generated file from `CFNPNIPPCLP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CFNPNIPPCLP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CFNPNIPPCLP {
    // message fields
    // @@protoc_insertion_point(field:CFNPNIPPCLP.FNKBEKIIMFK)
    pub FNKBEKIIMFK: bool,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.JALNENHNBDM)
    pub JALNENHNBDM: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.NCJNLEEFHKE)
    pub NCJNLEEFHKE: ::protobuf::EnumOrUnknown<super::ChessRogueCellSpecialType::ChessRogueCellSpecialType>,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.IFEHGGKMHCA)
    pub IFEHGGKMHCA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.MABLHMDBIAP)
    pub MABLHMDBIAP: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.DMMPJEEEABJ)
    pub DMMPJEEEABJ: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.HLDLDAPNILF)
    pub HLDLDAPNILF: ::protobuf::MessageField<super::KLAOKLBIMNM::KLAOKLBIMNM>,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.AJELLKFNBJE)
    pub AJELLKFNBJE: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.id)
    pub id: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.JFODJKAADCL)
    pub JFODJKAADCL: u32,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.PCOELCAOLHA)
    pub PCOELCAOLHA: ::protobuf::EnumOrUnknown<super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus>,
    // @@protoc_insertion_point(field:CFNPNIPPCLP.OCAKHJPHFAG)
    pub OCAKHJPHFAG: bool,
    // special fields
    // @@protoc_insertion_point(special_field:CFNPNIPPCLP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CFNPNIPPCLP {
    fn default() -> &'a CFNPNIPPCLP {
        <CFNPNIPPCLP as ::protobuf::Message>::default_instance()
    }
}

impl CFNPNIPPCLP {
    pub fn new() -> CFNPNIPPCLP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FNKBEKIIMFK",
            |m: &CFNPNIPPCLP| { &m.FNKBEKIIMFK },
            |m: &mut CFNPNIPPCLP| { &mut m.FNKBEKIIMFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JALNENHNBDM",
            |m: &CFNPNIPPCLP| { &m.JALNENHNBDM },
            |m: &mut CFNPNIPPCLP| { &mut m.JALNENHNBDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCJNLEEFHKE",
            |m: &CFNPNIPPCLP| { &m.NCJNLEEFHKE },
            |m: &mut CFNPNIPPCLP| { &mut m.NCJNLEEFHKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IFEHGGKMHCA",
            |m: &CFNPNIPPCLP| { &m.IFEHGGKMHCA },
            |m: &mut CFNPNIPPCLP| { &mut m.IFEHGGKMHCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MABLHMDBIAP",
            |m: &CFNPNIPPCLP| { &m.MABLHMDBIAP },
            |m: &mut CFNPNIPPCLP| { &mut m.MABLHMDBIAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMMPJEEEABJ",
            |m: &CFNPNIPPCLP| { &m.DMMPJEEEABJ },
            |m: &mut CFNPNIPPCLP| { &mut m.DMMPJEEEABJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KLAOKLBIMNM::KLAOKLBIMNM>(
            "HLDLDAPNILF",
            |m: &CFNPNIPPCLP| { &m.HLDLDAPNILF },
            |m: &mut CFNPNIPPCLP| { &mut m.HLDLDAPNILF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJELLKFNBJE",
            |m: &CFNPNIPPCLP| { &m.AJELLKFNBJE },
            |m: &mut CFNPNIPPCLP| { &mut m.AJELLKFNBJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &CFNPNIPPCLP| { &m.id },
            |m: &mut CFNPNIPPCLP| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFODJKAADCL",
            |m: &CFNPNIPPCLP| { &m.JFODJKAADCL },
            |m: &mut CFNPNIPPCLP| { &mut m.JFODJKAADCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCOELCAOLHA",
            |m: &CFNPNIPPCLP| { &m.PCOELCAOLHA },
            |m: &mut CFNPNIPPCLP| { &mut m.PCOELCAOLHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OCAKHJPHFAG",
            |m: &CFNPNIPPCLP| { &m.OCAKHJPHFAG },
            |m: &mut CFNPNIPPCLP| { &mut m.OCAKHJPHFAG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CFNPNIPPCLP>(
            "CFNPNIPPCLP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CFNPNIPPCLP {
    const NAME: &'static str = "CFNPNIPPCLP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.FNKBEKIIMFK = is.read_bool()?;
                },
                104 => {
                    self.JALNENHNBDM = is.read_uint32()?;
                },
                96 => {
                    self.NCJNLEEFHKE = is.read_enum_or_unknown()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.IFEHGGKMHCA)?;
                },
                32 => {
                    self.IFEHGGKMHCA.push(is.read_uint32()?);
                },
                112 => {
                    self.MABLHMDBIAP = is.read_uint32()?;
                },
                120 => {
                    self.DMMPJEEEABJ = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLDLDAPNILF)?;
                },
                56 => {
                    self.AJELLKFNBJE = is.read_uint32()?;
                },
                48 => {
                    self.id = is.read_uint32()?;
                },
                40 => {
                    self.JFODJKAADCL = is.read_uint32()?;
                },
                8 => {
                    self.PCOELCAOLHA = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.OCAKHJPHFAG = is.read_bool()?;
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
        if self.FNKBEKIIMFK != false {
            my_size += 1 + 1;
        }
        if self.JALNENHNBDM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.JALNENHNBDM);
        }
        if self.NCJNLEEFHKE != ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.NCJNLEEFHKE.value());
        }
        for value in &self.IFEHGGKMHCA {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.MABLHMDBIAP != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.MABLHMDBIAP);
        }
        if self.DMMPJEEEABJ != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.DMMPJEEEABJ);
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AJELLKFNBJE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.AJELLKFNBJE);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.id);
        }
        if self.JFODJKAADCL != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.JFODJKAADCL);
        }
        if self.PCOELCAOLHA != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            my_size += ::protobuf::rt::int32_size(1, self.PCOELCAOLHA.value());
        }
        if self.OCAKHJPHFAG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FNKBEKIIMFK != false {
            os.write_bool(8, self.FNKBEKIIMFK)?;
        }
        if self.JALNENHNBDM != 0 {
            os.write_uint32(13, self.JALNENHNBDM)?;
        }
        if self.NCJNLEEFHKE != ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.NCJNLEEFHKE))?;
        }
        for v in &self.IFEHGGKMHCA {
            os.write_uint32(4, *v)?;
        };
        if self.MABLHMDBIAP != 0 {
            os.write_uint32(14, self.MABLHMDBIAP)?;
        }
        if self.DMMPJEEEABJ != 0 {
            os.write_uint32(15, self.DMMPJEEEABJ)?;
        }
        if let Some(v) = self.HLDLDAPNILF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.AJELLKFNBJE != 0 {
            os.write_uint32(7, self.AJELLKFNBJE)?;
        }
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
        }
        if self.JFODJKAADCL != 0 {
            os.write_uint32(5, self.JFODJKAADCL)?;
        }
        if self.PCOELCAOLHA != ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.PCOELCAOLHA))?;
        }
        if self.OCAKHJPHFAG != false {
            os.write_bool(10, self.OCAKHJPHFAG)?;
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

    fn new() -> CFNPNIPPCLP {
        CFNPNIPPCLP::new()
    }

    fn clear(&mut self) {
        self.FNKBEKIIMFK = false;
        self.JALNENHNBDM = 0;
        self.NCJNLEEFHKE = ::protobuf::EnumOrUnknown::new(super::ChessRogueCellSpecialType::ChessRogueCellSpecialType::CHESS_ROGUE_CELL_SPECIAL_TYPE_NONE);
        self.IFEHGGKMHCA.clear();
        self.MABLHMDBIAP = 0;
        self.DMMPJEEEABJ = 0;
        self.HLDLDAPNILF.clear();
        self.AJELLKFNBJE = 0;
        self.id = 0;
        self.JFODJKAADCL = 0;
        self.PCOELCAOLHA = ::protobuf::EnumOrUnknown::new(super::ChessRogueBoardCellStatus::ChessRogueBoardCellStatus::IDLE);
        self.OCAKHJPHFAG = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CFNPNIPPCLP {
        static instance: CFNPNIPPCLP = CFNPNIPPCLP {
            FNKBEKIIMFK: false,
            JALNENHNBDM: 0,
            NCJNLEEFHKE: ::protobuf::EnumOrUnknown::from_i32(0),
            IFEHGGKMHCA: ::std::vec::Vec::new(),
            MABLHMDBIAP: 0,
            DMMPJEEEABJ: 0,
            HLDLDAPNILF: ::protobuf::MessageField::none(),
            AJELLKFNBJE: 0,
            id: 0,
            JFODJKAADCL: 0,
            PCOELCAOLHA: ::protobuf::EnumOrUnknown::from_i32(0),
            OCAKHJPHFAG: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CFNPNIPPCLP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CFNPNIPPCLP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CFNPNIPPCLP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CFNPNIPPCLP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CFNPNIPPCLP.proto\x1a\x1fChessRogueBoardCellStatus.proto\x1a\x1fCh\
    essRogueCellSpecialType.proto\x1a\x11KLAOKLBIMNM.proto\"\xd9\x03\n\x0bCF\
    NPNIPPCLP\x12\x20\n\x0bFNKBEKIIMFK\x18\x08\x20\x01(\x08R\x0bFNKBEKIIMFK\
    \x12\x20\n\x0bJALNENHNBDM\x18\r\x20\x01(\rR\x0bJALNENHNBDM\x12<\n\x0bNCJ\
    NLEEFHKE\x18\x0c\x20\x01(\x0e2\x1a.ChessRogueCellSpecialTypeR\x0bNCJNLEE\
    FHKE\x12\x20\n\x0bIFEHGGKMHCA\x18\x04\x20\x03(\rR\x0bIFEHGGKMHCA\x12\x20\
    \n\x0bMABLHMDBIAP\x18\x0e\x20\x01(\rR\x0bMABLHMDBIAP\x12\x20\n\x0bDMMPJE\
    EEABJ\x18\x0f\x20\x01(\rR\x0bDMMPJEEEABJ\x12.\n\x0bHLDLDAPNILF\x18\x02\
    \x20\x01(\x0b2\x0c.KLAOKLBIMNMR\x0bHLDLDAPNILF\x12\x20\n\x0bAJELLKFNBJE\
    \x18\x07\x20\x01(\rR\x0bAJELLKFNBJE\x12\x0e\n\x02id\x18\x06\x20\x01(\rR\
    \x02id\x12\x20\n\x0bJFODJKAADCL\x18\x05\x20\x01(\rR\x0bJFODJKAADCL\x12<\
    \n\x0bPCOELCAOLHA\x18\x01\x20\x01(\x0e2\x1a.ChessRogueBoardCellStatusR\
    \x0bPCOELCAOLHA\x12\x20\n\x0bOCAKHJPHFAG\x18\n\x20\x01(\x08R\x0bOCAKHJPH\
    FAGb\x06proto3\
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
            deps.push(super::ChessRogueBoardCellStatus::file_descriptor().clone());
            deps.push(super::ChessRogueCellSpecialType::file_descriptor().clone());
            deps.push(super::KLAOKLBIMNM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CFNPNIPPCLP::generated_message_descriptor_data());
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
