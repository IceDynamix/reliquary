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

//! Generated file from `BattlePassInfoNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattlePassInfoNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattlePassInfoNotify {
    // message fields
    // @@protoc_insertion_point(field:BattlePassInfoNotify.level)
    pub level: u32,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.CKNFMIFFAHI)
    pub CKNFMIFFAHI: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.ELPGEDMBCNB)
    pub ELPGEDMBCNB: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.BAAIAFIJFAF)
    pub BAAIAFIJFAF: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.NPMCPLGFOHL)
    pub NPMCPLGFOHL: u32,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.FJIDIKAJMJI)
    pub FJIDIKAJMJI: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.PFFMENBDHJC)
    pub PFFMENBDHJC: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.OLOBAGLBGHG)
    pub OLOBAGLBGHG: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.IKOBOEKHOOK)
    pub IKOBOEKHOOK: ::protobuf::EnumOrUnknown<super::BpTierType::BpTierType>,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.GMBNOJBOHKK)
    pub GMBNOJBOHKK: u64,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.OFGGAMHLPMP)
    pub OFGGAMHLPMP: u32,
    // @@protoc_insertion_point(field:BattlePassInfoNotify.HMBAFEEKNAJ)
    pub HMBAFEEKNAJ: u64,
    // special fields
    // @@protoc_insertion_point(special_field:BattlePassInfoNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattlePassInfoNotify {
    fn default() -> &'a BattlePassInfoNotify {
        <BattlePassInfoNotify as ::protobuf::Message>::default_instance()
    }
}

impl BattlePassInfoNotify {
    pub fn new() -> BattlePassInfoNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &BattlePassInfoNotify| { &m.level },
            |m: &mut BattlePassInfoNotify| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CKNFMIFFAHI",
            |m: &BattlePassInfoNotify| { &m.CKNFMIFFAHI },
            |m: &mut BattlePassInfoNotify| { &mut m.CKNFMIFFAHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ELPGEDMBCNB",
            |m: &BattlePassInfoNotify| { &m.ELPGEDMBCNB },
            |m: &mut BattlePassInfoNotify| { &mut m.ELPGEDMBCNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BAAIAFIJFAF",
            |m: &BattlePassInfoNotify| { &m.BAAIAFIJFAF },
            |m: &mut BattlePassInfoNotify| { &mut m.BAAIAFIJFAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &BattlePassInfoNotify| { &m.exp },
            |m: &mut BattlePassInfoNotify| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPMCPLGFOHL",
            |m: &BattlePassInfoNotify| { &m.NPMCPLGFOHL },
            |m: &mut BattlePassInfoNotify| { &mut m.NPMCPLGFOHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJIDIKAJMJI",
            |m: &BattlePassInfoNotify| { &m.FJIDIKAJMJI },
            |m: &mut BattlePassInfoNotify| { &mut m.FJIDIKAJMJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PFFMENBDHJC",
            |m: &BattlePassInfoNotify| { &m.PFFMENBDHJC },
            |m: &mut BattlePassInfoNotify| { &mut m.PFFMENBDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLOBAGLBGHG",
            |m: &BattlePassInfoNotify| { &m.OLOBAGLBGHG },
            |m: &mut BattlePassInfoNotify| { &mut m.OLOBAGLBGHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKOBOEKHOOK",
            |m: &BattlePassInfoNotify| { &m.IKOBOEKHOOK },
            |m: &mut BattlePassInfoNotify| { &mut m.IKOBOEKHOOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMBNOJBOHKK",
            |m: &BattlePassInfoNotify| { &m.GMBNOJBOHKK },
            |m: &mut BattlePassInfoNotify| { &mut m.GMBNOJBOHKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFGGAMHLPMP",
            |m: &BattlePassInfoNotify| { &m.OFGGAMHLPMP },
            |m: &mut BattlePassInfoNotify| { &mut m.OFGGAMHLPMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMBAFEEKNAJ",
            |m: &BattlePassInfoNotify| { &m.HMBAFEEKNAJ },
            |m: &mut BattlePassInfoNotify| { &mut m.HMBAFEEKNAJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattlePassInfoNotify>(
            "BattlePassInfoNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattlePassInfoNotify {
    const NAME: &'static str = "BattlePassInfoNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.CKNFMIFFAHI = is.read_uint64()?;
                },
                80 => {
                    self.ELPGEDMBCNB = is.read_uint64()?;
                },
                112 => {
                    self.BAAIAFIJFAF = is.read_uint64()?;
                },
                88 => {
                    self.exp = is.read_uint32()?;
                },
                72 => {
                    self.NPMCPLGFOHL = is.read_uint32()?;
                },
                104 => {
                    self.FJIDIKAJMJI = is.read_uint64()?;
                },
                8 => {
                    self.PFFMENBDHJC = is.read_uint64()?;
                },
                64 => {
                    self.OLOBAGLBGHG = is.read_uint64()?;
                },
                120 => {
                    self.IKOBOEKHOOK = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.GMBNOJBOHKK = is.read_uint64()?;
                },
                24 => {
                    self.OFGGAMHLPMP = is.read_uint32()?;
                },
                56 => {
                    self.HMBAFEEKNAJ = is.read_uint64()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.level);
        }
        if self.CKNFMIFFAHI != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.CKNFMIFFAHI);
        }
        if self.ELPGEDMBCNB != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.ELPGEDMBCNB);
        }
        if self.BAAIAFIJFAF != 0 {
            my_size += ::protobuf::rt::uint64_size(14, self.BAAIAFIJFAF);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.exp);
        }
        if self.NPMCPLGFOHL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NPMCPLGFOHL);
        }
        if self.FJIDIKAJMJI != 0 {
            my_size += ::protobuf::rt::uint64_size(13, self.FJIDIKAJMJI);
        }
        if self.PFFMENBDHJC != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.PFFMENBDHJC);
        }
        if self.OLOBAGLBGHG != 0 {
            my_size += ::protobuf::rt::uint64_size(8, self.OLOBAGLBGHG);
        }
        if self.IKOBOEKHOOK != ::protobuf::EnumOrUnknown::new(super::BpTierType::BpTierType::BP_TIER_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.IKOBOEKHOOK.value());
        }
        if self.GMBNOJBOHKK != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.GMBNOJBOHKK);
        }
        if self.OFGGAMHLPMP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OFGGAMHLPMP);
        }
        if self.HMBAFEEKNAJ != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.HMBAFEEKNAJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(6, self.level)?;
        }
        if self.CKNFMIFFAHI != 0 {
            os.write_uint64(12, self.CKNFMIFFAHI)?;
        }
        if self.ELPGEDMBCNB != 0 {
            os.write_uint64(10, self.ELPGEDMBCNB)?;
        }
        if self.BAAIAFIJFAF != 0 {
            os.write_uint64(14, self.BAAIAFIJFAF)?;
        }
        if self.exp != 0 {
            os.write_uint32(11, self.exp)?;
        }
        if self.NPMCPLGFOHL != 0 {
            os.write_uint32(9, self.NPMCPLGFOHL)?;
        }
        if self.FJIDIKAJMJI != 0 {
            os.write_uint64(13, self.FJIDIKAJMJI)?;
        }
        if self.PFFMENBDHJC != 0 {
            os.write_uint64(1, self.PFFMENBDHJC)?;
        }
        if self.OLOBAGLBGHG != 0 {
            os.write_uint64(8, self.OLOBAGLBGHG)?;
        }
        if self.IKOBOEKHOOK != ::protobuf::EnumOrUnknown::new(super::BpTierType::BpTierType::BP_TIER_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.IKOBOEKHOOK))?;
        }
        if self.GMBNOJBOHKK != 0 {
            os.write_uint64(5, self.GMBNOJBOHKK)?;
        }
        if self.OFGGAMHLPMP != 0 {
            os.write_uint32(3, self.OFGGAMHLPMP)?;
        }
        if self.HMBAFEEKNAJ != 0 {
            os.write_uint64(7, self.HMBAFEEKNAJ)?;
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

    fn new() -> BattlePassInfoNotify {
        BattlePassInfoNotify::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.CKNFMIFFAHI = 0;
        self.ELPGEDMBCNB = 0;
        self.BAAIAFIJFAF = 0;
        self.exp = 0;
        self.NPMCPLGFOHL = 0;
        self.FJIDIKAJMJI = 0;
        self.PFFMENBDHJC = 0;
        self.OLOBAGLBGHG = 0;
        self.IKOBOEKHOOK = ::protobuf::EnumOrUnknown::new(super::BpTierType::BpTierType::BP_TIER_TYPE_NONE);
        self.GMBNOJBOHKK = 0;
        self.OFGGAMHLPMP = 0;
        self.HMBAFEEKNAJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattlePassInfoNotify {
        static instance: BattlePassInfoNotify = BattlePassInfoNotify {
            level: 0,
            CKNFMIFFAHI: 0,
            ELPGEDMBCNB: 0,
            BAAIAFIJFAF: 0,
            exp: 0,
            NPMCPLGFOHL: 0,
            FJIDIKAJMJI: 0,
            PFFMENBDHJC: 0,
            OLOBAGLBGHG: 0,
            IKOBOEKHOOK: ::protobuf::EnumOrUnknown::from_i32(0),
            GMBNOJBOHKK: 0,
            OFGGAMHLPMP: 0,
            HMBAFEEKNAJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattlePassInfoNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattlePassInfoNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattlePassInfoNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattlePassInfoNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aBattlePassInfoNotify.proto\x1a\x10BpTierType.proto\"\xc1\x03\n\x14\
    BattlePassInfoNotify\x12\x14\n\x05level\x18\x06\x20\x01(\rR\x05level\x12\
    \x20\n\x0bCKNFMIFFAHI\x18\x0c\x20\x01(\x04R\x0bCKNFMIFFAHI\x12\x20\n\x0b\
    ELPGEDMBCNB\x18\n\x20\x01(\x04R\x0bELPGEDMBCNB\x12\x20\n\x0bBAAIAFIJFAF\
    \x18\x0e\x20\x01(\x04R\x0bBAAIAFIJFAF\x12\x10\n\x03exp\x18\x0b\x20\x01(\
    \rR\x03exp\x12\x20\n\x0bNPMCPLGFOHL\x18\t\x20\x01(\rR\x0bNPMCPLGFOHL\x12\
    \x20\n\x0bFJIDIKAJMJI\x18\r\x20\x01(\x04R\x0bFJIDIKAJMJI\x12\x20\n\x0bPF\
    FMENBDHJC\x18\x01\x20\x01(\x04R\x0bPFFMENBDHJC\x12\x20\n\x0bOLOBAGLBGHG\
    \x18\x08\x20\x01(\x04R\x0bOLOBAGLBGHG\x12-\n\x0bIKOBOEKHOOK\x18\x0f\x20\
    \x01(\x0e2\x0b.BpTierTypeR\x0bIKOBOEKHOOK\x12\x20\n\x0bGMBNOJBOHKK\x18\
    \x05\x20\x01(\x04R\x0bGMBNOJBOHKK\x12\x20\n\x0bOFGGAMHLPMP\x18\x03\x20\
    \x01(\rR\x0bOFGGAMHLPMP\x12\x20\n\x0bHMBAFEEKNAJ\x18\x07\x20\x01(\x04R\
    \x0bHMBAFEEKNAJb\x06proto3\
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
            deps.push(super::BpTierType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattlePassInfoNotify::generated_message_descriptor_data());
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
