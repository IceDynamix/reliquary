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

//! Generated file from `DNPMGACEBMM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DNPMGACEBMM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DNPMGACEBMM {
    // message fields
    // @@protoc_insertion_point(field:DNPMGACEBMM.AGHLHOMLAOA)
    pub AGHLHOMLAOA: ::protobuf::MessageField<super::ENPCFKIMJAH::ENPCFKIMJAH>,
    // @@protoc_insertion_point(field:DNPMGACEBMM.EIFHOCNALBA)
    pub EIFHOCNALBA: u32,
    // @@protoc_insertion_point(field:DNPMGACEBMM.BGJKOLNCOPN)
    pub BGJKOLNCOPN: u32,
    // @@protoc_insertion_point(field:DNPMGACEBMM.FIOAACNKLPC)
    pub FIOAACNKLPC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DNPMGACEBMM.status)
    pub status: ::protobuf::EnumOrUnknown<super::RogueStatus::RogueStatus>,
    // @@protoc_insertion_point(field:DNPMGACEBMM.PHHANACJEGG)
    pub PHHANACJEGG: u32,
    // @@protoc_insertion_point(field:DNPMGACEBMM.MOICCJNMBBI)
    pub MOICCJNMBBI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DNPMGACEBMM.PFPDFFCGKNB)
    pub PFPDFFCGKNB: ::protobuf::MessageField<super::DDBKAILCBBP::DDBKAILCBBP>,
    // @@protoc_insertion_point(field:DNPMGACEBMM.CGHKMJGKKIJ)
    pub CGHKMJGKKIJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DNPMGACEBMM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DNPMGACEBMM {
    fn default() -> &'a DNPMGACEBMM {
        <DNPMGACEBMM as ::protobuf::Message>::default_instance()
    }
}

impl DNPMGACEBMM {
    pub fn new() -> DNPMGACEBMM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ENPCFKIMJAH::ENPCFKIMJAH>(
            "AGHLHOMLAOA",
            |m: &DNPMGACEBMM| { &m.AGHLHOMLAOA },
            |m: &mut DNPMGACEBMM| { &mut m.AGHLHOMLAOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EIFHOCNALBA",
            |m: &DNPMGACEBMM| { &m.EIFHOCNALBA },
            |m: &mut DNPMGACEBMM| { &mut m.EIFHOCNALBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGJKOLNCOPN",
            |m: &DNPMGACEBMM| { &m.BGJKOLNCOPN },
            |m: &mut DNPMGACEBMM| { &mut m.BGJKOLNCOPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FIOAACNKLPC",
            |m: &DNPMGACEBMM| { &m.FIOAACNKLPC },
            |m: &mut DNPMGACEBMM| { &mut m.FIOAACNKLPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &DNPMGACEBMM| { &m.status },
            |m: &mut DNPMGACEBMM| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHHANACJEGG",
            |m: &DNPMGACEBMM| { &m.PHHANACJEGG },
            |m: &mut DNPMGACEBMM| { &mut m.PHHANACJEGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MOICCJNMBBI",
            |m: &DNPMGACEBMM| { &m.MOICCJNMBBI },
            |m: &mut DNPMGACEBMM| { &mut m.MOICCJNMBBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DDBKAILCBBP::DDBKAILCBBP>(
            "PFPDFFCGKNB",
            |m: &DNPMGACEBMM| { &m.PFPDFFCGKNB },
            |m: &mut DNPMGACEBMM| { &mut m.PFPDFFCGKNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CGHKMJGKKIJ",
            |m: &DNPMGACEBMM| { &m.CGHKMJGKKIJ },
            |m: &mut DNPMGACEBMM| { &mut m.CGHKMJGKKIJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DNPMGACEBMM>(
            "DNPMGACEBMM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DNPMGACEBMM {
    const NAME: &'static str = "DNPMGACEBMM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AGHLHOMLAOA)?;
                },
                80 => {
                    self.EIFHOCNALBA = is.read_uint32()?;
                },
                24 => {
                    self.BGJKOLNCOPN = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.FIOAACNKLPC)?;
                },
                32 => {
                    self.FIOAACNKLPC.push(is.read_uint32()?);
                },
                56 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.PHHANACJEGG = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.MOICCJNMBBI)?;
                },
                72 => {
                    self.MOICCJNMBBI.push(is.read_uint32()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PFPDFFCGKNB)?;
                },
                96 => {
                    self.CGHKMJGKKIJ = is.read_uint32()?;
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
        if let Some(v) = self.AGHLHOMLAOA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EIFHOCNALBA != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.EIFHOCNALBA);
        }
        if self.BGJKOLNCOPN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BGJKOLNCOPN);
        }
        for value in &self.FIOAACNKLPC {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.status.value());
        }
        if self.PHHANACJEGG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PHHANACJEGG);
        }
        for value in &self.MOICCJNMBBI {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if let Some(v) = self.PFPDFFCGKNB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.CGHKMJGKKIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.CGHKMJGKKIJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.AGHLHOMLAOA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.EIFHOCNALBA != 0 {
            os.write_uint32(10, self.EIFHOCNALBA)?;
        }
        if self.BGJKOLNCOPN != 0 {
            os.write_uint32(3, self.BGJKOLNCOPN)?;
        }
        for v in &self.FIOAACNKLPC {
            os.write_uint32(4, *v)?;
        };
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.PHHANACJEGG != 0 {
            os.write_uint32(1, self.PHHANACJEGG)?;
        }
        for v in &self.MOICCJNMBBI {
            os.write_uint32(9, *v)?;
        };
        if let Some(v) = self.PFPDFFCGKNB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.CGHKMJGKKIJ != 0 {
            os.write_uint32(12, self.CGHKMJGKKIJ)?;
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

    fn new() -> DNPMGACEBMM {
        DNPMGACEBMM::new()
    }

    fn clear(&mut self) {
        self.AGHLHOMLAOA.clear();
        self.EIFHOCNALBA = 0;
        self.BGJKOLNCOPN = 0;
        self.FIOAACNKLPC.clear();
        self.status = ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE);
        self.PHHANACJEGG = 0;
        self.MOICCJNMBBI.clear();
        self.PFPDFFCGKNB.clear();
        self.CGHKMJGKKIJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DNPMGACEBMM {
        static instance: DNPMGACEBMM = DNPMGACEBMM {
            AGHLHOMLAOA: ::protobuf::MessageField::none(),
            EIFHOCNALBA: 0,
            BGJKOLNCOPN: 0,
            FIOAACNKLPC: ::std::vec::Vec::new(),
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            PHHANACJEGG: 0,
            MOICCJNMBBI: ::std::vec::Vec::new(),
            PFPDFFCGKNB: ::protobuf::MessageField::none(),
            CGHKMJGKKIJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DNPMGACEBMM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DNPMGACEBMM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DNPMGACEBMM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DNPMGACEBMM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DNPMGACEBMM.proto\x1a\x11DDBKAILCBBP.proto\x1a\x11ENPCFKIMJAH.prot\
    o\x1a\x11RogueStatus.proto\"\xdf\x02\n\x0bDNPMGACEBMM\x12.\n\x0bAGHLHOML\
    AOA\x18\x05\x20\x01(\x0b2\x0c.ENPCFKIMJAHR\x0bAGHLHOMLAOA\x12\x20\n\x0bE\
    IFHOCNALBA\x18\n\x20\x01(\rR\x0bEIFHOCNALBA\x12\x20\n\x0bBGJKOLNCOPN\x18\
    \x03\x20\x01(\rR\x0bBGJKOLNCOPN\x12\x20\n\x0bFIOAACNKLPC\x18\x04\x20\x03\
    (\rR\x0bFIOAACNKLPC\x12$\n\x06status\x18\x07\x20\x01(\x0e2\x0c.RogueStat\
    usR\x06status\x12\x20\n\x0bPHHANACJEGG\x18\x01\x20\x01(\rR\x0bPHHANACJEG\
    G\x12\x20\n\x0bMOICCJNMBBI\x18\t\x20\x03(\rR\x0bMOICCJNMBBI\x12.\n\x0bPF\
    PDFFCGKNB\x18\r\x20\x01(\x0b2\x0c.DDBKAILCBBPR\x0bPFPDFFCGKNB\x12\x20\n\
    \x0bCGHKMJGKKIJ\x18\x0c\x20\x01(\rR\x0bCGHKMJGKKIJb\x06proto3\
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
            deps.push(super::DDBKAILCBBP::file_descriptor().clone());
            deps.push(super::ENPCFKIMJAH::file_descriptor().clone());
            deps.push(super::RogueStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DNPMGACEBMM::generated_message_descriptor_data());
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
