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

//! Generated file from `ParkourEndLevelCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ParkourEndLevelCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ParkourEndLevelCsReq {
    // message fields
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.BAABDDJEHMC)
    pub BAABDDJEHMC: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.EEFCBBKKFLC)
    pub EEFCBBKKFLC: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.ACJCPHIFMLN)
    pub ACJCPHIFMLN: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.POFMCALHOOC)
    pub POFMCALHOOC: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.end_reason)
    pub end_reason: ::protobuf::EnumOrUnknown<super::POAHABDKPKJ::POAHABDKPKJ>,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.HMBHEIGKDBK)
    pub HMBHEIGKDBK: ::std::vec::Vec<super::FJJOFEKPDDH::FJJOFEKPDDH>,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.rank)
    pub rank: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.AEDBPADEGFI)
    pub AEDBPADEGFI: ::std::vec::Vec<super::BFPOLEGCCPJ::BFPOLEGCCPJ>,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.time)
    pub time: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelCsReq.IFENFKGGIEM)
    pub IFENFKGGIEM: ::protobuf::MessageField<super::OOALAODNCPE::OOALAODNCPE>,
    // special fields
    // @@protoc_insertion_point(special_field:ParkourEndLevelCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ParkourEndLevelCsReq {
    fn default() -> &'a ParkourEndLevelCsReq {
        <ParkourEndLevelCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ParkourEndLevelCsReq {
    pub fn new() -> ParkourEndLevelCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BAABDDJEHMC",
            |m: &ParkourEndLevelCsReq| { &m.BAABDDJEHMC },
            |m: &mut ParkourEndLevelCsReq| { &mut m.BAABDDJEHMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEFCBBKKFLC",
            |m: &ParkourEndLevelCsReq| { &m.EEFCBBKKFLC },
            |m: &mut ParkourEndLevelCsReq| { &mut m.EEFCBBKKFLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACJCPHIFMLN",
            |m: &ParkourEndLevelCsReq| { &m.ACJCPHIFMLN },
            |m: &mut ParkourEndLevelCsReq| { &mut m.ACJCPHIFMLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POFMCALHOOC",
            |m: &ParkourEndLevelCsReq| { &m.POFMCALHOOC },
            |m: &mut ParkourEndLevelCsReq| { &mut m.POFMCALHOOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_reason",
            |m: &ParkourEndLevelCsReq| { &m.end_reason },
            |m: &mut ParkourEndLevelCsReq| { &mut m.end_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HMBHEIGKDBK",
            |m: &ParkourEndLevelCsReq| { &m.HMBHEIGKDBK },
            |m: &mut ParkourEndLevelCsReq| { &mut m.HMBHEIGKDBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &ParkourEndLevelCsReq| { &m.rank },
            |m: &mut ParkourEndLevelCsReq| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AEDBPADEGFI",
            |m: &ParkourEndLevelCsReq| { &m.AEDBPADEGFI },
            |m: &mut ParkourEndLevelCsReq| { &mut m.AEDBPADEGFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &ParkourEndLevelCsReq| { &m.time },
            |m: &mut ParkourEndLevelCsReq| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OOALAODNCPE::OOALAODNCPE>(
            "IFENFKGGIEM",
            |m: &ParkourEndLevelCsReq| { &m.IFENFKGGIEM },
            |m: &mut ParkourEndLevelCsReq| { &mut m.IFENFKGGIEM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ParkourEndLevelCsReq>(
            "ParkourEndLevelCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ParkourEndLevelCsReq {
    const NAME: &'static str = "ParkourEndLevelCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.BAABDDJEHMC = is.read_uint32()?;
                },
                32 => {
                    self.EEFCBBKKFLC = is.read_uint32()?;
                },
                24 => {
                    self.ACJCPHIFMLN = is.read_uint32()?;
                },
                56 => {
                    self.POFMCALHOOC = is.read_uint32()?;
                },
                88 => {
                    self.end_reason = is.read_enum_or_unknown()?;
                },
                50 => {
                    self.HMBHEIGKDBK.push(is.read_message()?);
                },
                112 => {
                    self.rank = is.read_uint32()?;
                },
                98 => {
                    self.AEDBPADEGFI.push(is.read_message()?);
                },
                104 => {
                    self.time = is.read_uint32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IFENFKGGIEM)?;
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
        if self.BAABDDJEHMC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BAABDDJEHMC);
        }
        if self.EEFCBBKKFLC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.EEFCBBKKFLC);
        }
        if self.ACJCPHIFMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.ACJCPHIFMLN);
        }
        if self.POFMCALHOOC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.POFMCALHOOC);
        }
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.end_reason.value());
        }
        for value in &self.HMBHEIGKDBK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.rank);
        }
        for value in &self.AEDBPADEGFI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.time != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.time);
        }
        if let Some(v) = self.IFENFKGGIEM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BAABDDJEHMC != 0 {
            os.write_uint32(9, self.BAABDDJEHMC)?;
        }
        if self.EEFCBBKKFLC != 0 {
            os.write_uint32(4, self.EEFCBBKKFLC)?;
        }
        if self.ACJCPHIFMLN != 0 {
            os.write_uint32(3, self.ACJCPHIFMLN)?;
        }
        if self.POFMCALHOOC != 0 {
            os.write_uint32(7, self.POFMCALHOOC)?;
        }
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.end_reason))?;
        }
        for v in &self.HMBHEIGKDBK {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.rank != 0 {
            os.write_uint32(14, self.rank)?;
        }
        for v in &self.AEDBPADEGFI {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.time != 0 {
            os.write_uint32(13, self.time)?;
        }
        if let Some(v) = self.IFENFKGGIEM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
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

    fn new() -> ParkourEndLevelCsReq {
        ParkourEndLevelCsReq::new()
    }

    fn clear(&mut self) {
        self.BAABDDJEHMC = 0;
        self.EEFCBBKKFLC = 0;
        self.ACJCPHIFMLN = 0;
        self.POFMCALHOOC = 0;
        self.end_reason = ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE);
        self.HMBHEIGKDBK.clear();
        self.rank = 0;
        self.AEDBPADEGFI.clear();
        self.time = 0;
        self.IFENFKGGIEM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ParkourEndLevelCsReq {
        static instance: ParkourEndLevelCsReq = ParkourEndLevelCsReq {
            BAABDDJEHMC: 0,
            EEFCBBKKFLC: 0,
            ACJCPHIFMLN: 0,
            POFMCALHOOC: 0,
            end_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            HMBHEIGKDBK: ::std::vec::Vec::new(),
            rank: 0,
            AEDBPADEGFI: ::std::vec::Vec::new(),
            time: 0,
            IFENFKGGIEM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ParkourEndLevelCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ParkourEndLevelCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ParkourEndLevelCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ParkourEndLevelCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aParkourEndLevelCsReq.proto\x1a\x11BFPOLEGCCPJ.proto\x1a\x11FJJOFEK\
    PDDH.proto\x1a\x11OOALAODNCPE.proto\x1a\x11POAHABDKPKJ.proto\"\x83\x03\n\
    \x14ParkourEndLevelCsReq\x12\x20\n\x0bBAABDDJEHMC\x18\t\x20\x01(\rR\x0bB\
    AABDDJEHMC\x12\x20\n\x0bEEFCBBKKFLC\x18\x04\x20\x01(\rR\x0bEEFCBBKKFLC\
    \x12\x20\n\x0bACJCPHIFMLN\x18\x03\x20\x01(\rR\x0bACJCPHIFMLN\x12\x20\n\
    \x0bPOFMCALHOOC\x18\x07\x20\x01(\rR\x0bPOFMCALHOOC\x12+\n\nend_reason\
    \x18\x0b\x20\x01(\x0e2\x0c.POAHABDKPKJR\tendReason\x12.\n\x0bHMBHEIGKDBK\
    \x18\x06\x20\x03(\x0b2\x0c.FJJOFEKPDDHR\x0bHMBHEIGKDBK\x12\x12\n\x04rank\
    \x18\x0e\x20\x01(\rR\x04rank\x12.\n\x0bAEDBPADEGFI\x18\x0c\x20\x03(\x0b2\
    \x0c.BFPOLEGCCPJR\x0bAEDBPADEGFI\x12\x12\n\x04time\x18\r\x20\x01(\rR\x04\
    time\x12.\n\x0bIFENFKGGIEM\x18\n\x20\x01(\x0b2\x0c.OOALAODNCPER\x0bIFENF\
    KGGIEMb\x06proto3\
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
            deps.push(super::BFPOLEGCCPJ::file_descriptor().clone());
            deps.push(super::FJJOFEKPDDH::file_descriptor().clone());
            deps.push(super::OOALAODNCPE::file_descriptor().clone());
            deps.push(super::POAHABDKPKJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ParkourEndLevelCsReq::generated_message_descriptor_data());
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
