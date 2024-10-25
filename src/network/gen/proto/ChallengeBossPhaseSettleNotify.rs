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

//! Generated file from `ChallengeBossPhaseSettleNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChallengeBossPhaseSettleNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeBossPhaseSettleNotify {
    // message fields
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.BNEBJJBOJDG)
    pub BNEBJJBOJDG: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.EEPIGCKDPDK)
    pub EEPIGCKDPDK: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.DMIEBIKLCPG)
    pub DMIEBIKLCPG: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.HJAALLNAFOO)
    pub HJAALLNAFOO: ::std::vec::Vec<super::BattleTarget::BattleTarget>,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.KFHANKAEJFJ)
    pub KFHANKAEJFJ: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.IBJDEGFNOIN)
    pub IBJDEGFNOIN: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.OFECNECFJJI)
    pub OFECNECFJJI: bool,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.DBKHFAEKNKL)
    pub DBKHFAEKNKL: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.OGMPDLCDJDL)
    pub OGMPDLCDJDL: u32,
    // @@protoc_insertion_point(field:ChallengeBossPhaseSettleNotify.MGGDEINKDMH)
    pub MGGDEINKDMH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeBossPhaseSettleNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeBossPhaseSettleNotify {
    fn default() -> &'a ChallengeBossPhaseSettleNotify {
        <ChallengeBossPhaseSettleNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeBossPhaseSettleNotify {
    pub fn new() -> ChallengeBossPhaseSettleNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNEBJJBOJDG",
            |m: &ChallengeBossPhaseSettleNotify| { &m.BNEBJJBOJDG },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.BNEBJJBOJDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEPIGCKDPDK",
            |m: &ChallengeBossPhaseSettleNotify| { &m.EEPIGCKDPDK },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.EEPIGCKDPDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMIEBIKLCPG",
            |m: &ChallengeBossPhaseSettleNotify| { &m.DMIEBIKLCPG },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.DMIEBIKLCPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJAALLNAFOO",
            |m: &ChallengeBossPhaseSettleNotify| { &m.HJAALLNAFOO },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.HJAALLNAFOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFHANKAEJFJ",
            |m: &ChallengeBossPhaseSettleNotify| { &m.KFHANKAEJFJ },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.KFHANKAEJFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBJDEGFNOIN",
            |m: &ChallengeBossPhaseSettleNotify| { &m.IBJDEGFNOIN },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.IBJDEGFNOIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFECNECFJJI",
            |m: &ChallengeBossPhaseSettleNotify| { &m.OFECNECFJJI },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.OFECNECFJJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBKHFAEKNKL",
            |m: &ChallengeBossPhaseSettleNotify| { &m.DBKHFAEKNKL },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.DBKHFAEKNKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGMPDLCDJDL",
            |m: &ChallengeBossPhaseSettleNotify| { &m.OGMPDLCDJDL },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.OGMPDLCDJDL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGGDEINKDMH",
            |m: &ChallengeBossPhaseSettleNotify| { &m.MGGDEINKDMH },
            |m: &mut ChallengeBossPhaseSettleNotify| { &mut m.MGGDEINKDMH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeBossPhaseSettleNotify>(
            "ChallengeBossPhaseSettleNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeBossPhaseSettleNotify {
    const NAME: &'static str = "ChallengeBossPhaseSettleNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.BNEBJJBOJDG = is.read_bool()?;
                },
                56 => {
                    self.EEPIGCKDPDK = is.read_bool()?;
                },
                40 => {
                    self.DMIEBIKLCPG = is.read_uint32()?;
                },
                34 => {
                    self.HJAALLNAFOO.push(is.read_message()?);
                },
                96 => {
                    self.KFHANKAEJFJ = is.read_uint32()?;
                },
                72 => {
                    self.IBJDEGFNOIN = is.read_uint32()?;
                },
                48 => {
                    self.OFECNECFJJI = is.read_bool()?;
                },
                24 => {
                    self.DBKHFAEKNKL = is.read_uint32()?;
                },
                64 => {
                    self.OGMPDLCDJDL = is.read_uint32()?;
                },
                8 => {
                    self.MGGDEINKDMH = is.read_uint32()?;
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
        if self.BNEBJJBOJDG != false {
            my_size += 1 + 1;
        }
        if self.EEPIGCKDPDK != false {
            my_size += 1 + 1;
        }
        if self.DMIEBIKLCPG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DMIEBIKLCPG);
        }
        for value in &self.HJAALLNAFOO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KFHANKAEJFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.KFHANKAEJFJ);
        }
        if self.IBJDEGFNOIN != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.IBJDEGFNOIN);
        }
        if self.OFECNECFJJI != false {
            my_size += 1 + 1;
        }
        if self.DBKHFAEKNKL != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DBKHFAEKNKL);
        }
        if self.OGMPDLCDJDL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OGMPDLCDJDL);
        }
        if self.MGGDEINKDMH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MGGDEINKDMH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BNEBJJBOJDG != false {
            os.write_bool(14, self.BNEBJJBOJDG)?;
        }
        if self.EEPIGCKDPDK != false {
            os.write_bool(7, self.EEPIGCKDPDK)?;
        }
        if self.DMIEBIKLCPG != 0 {
            os.write_uint32(5, self.DMIEBIKLCPG)?;
        }
        for v in &self.HJAALLNAFOO {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.KFHANKAEJFJ != 0 {
            os.write_uint32(12, self.KFHANKAEJFJ)?;
        }
        if self.IBJDEGFNOIN != 0 {
            os.write_uint32(9, self.IBJDEGFNOIN)?;
        }
        if self.OFECNECFJJI != false {
            os.write_bool(6, self.OFECNECFJJI)?;
        }
        if self.DBKHFAEKNKL != 0 {
            os.write_uint32(3, self.DBKHFAEKNKL)?;
        }
        if self.OGMPDLCDJDL != 0 {
            os.write_uint32(8, self.OGMPDLCDJDL)?;
        }
        if self.MGGDEINKDMH != 0 {
            os.write_uint32(1, self.MGGDEINKDMH)?;
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

    fn new() -> ChallengeBossPhaseSettleNotify {
        ChallengeBossPhaseSettleNotify::new()
    }

    fn clear(&mut self) {
        self.BNEBJJBOJDG = false;
        self.EEPIGCKDPDK = false;
        self.DMIEBIKLCPG = 0;
        self.HJAALLNAFOO.clear();
        self.KFHANKAEJFJ = 0;
        self.IBJDEGFNOIN = 0;
        self.OFECNECFJJI = false;
        self.DBKHFAEKNKL = 0;
        self.OGMPDLCDJDL = 0;
        self.MGGDEINKDMH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeBossPhaseSettleNotify {
        static instance: ChallengeBossPhaseSettleNotify = ChallengeBossPhaseSettleNotify {
            BNEBJJBOJDG: false,
            EEPIGCKDPDK: false,
            DMIEBIKLCPG: 0,
            HJAALLNAFOO: ::std::vec::Vec::new(),
            KFHANKAEJFJ: 0,
            IBJDEGFNOIN: 0,
            OFECNECFJJI: false,
            DBKHFAEKNKL: 0,
            OGMPDLCDJDL: 0,
            MGGDEINKDMH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeBossPhaseSettleNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeBossPhaseSettleNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeBossPhaseSettleNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeBossPhaseSettleNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$ChallengeBossPhaseSettleNotify.proto\x1a\x12BattleTarget.proto\"\x83\
    \x03\n\x1eChallengeBossPhaseSettleNotify\x12\x20\n\x0bBNEBJJBOJDG\x18\
    \x0e\x20\x01(\x08R\x0bBNEBJJBOJDG\x12\x20\n\x0bEEPIGCKDPDK\x18\x07\x20\
    \x01(\x08R\x0bEEPIGCKDPDK\x12\x20\n\x0bDMIEBIKLCPG\x18\x05\x20\x01(\rR\
    \x0bDMIEBIKLCPG\x12/\n\x0bHJAALLNAFOO\x18\x04\x20\x03(\x0b2\r.BattleTarg\
    etR\x0bHJAALLNAFOO\x12\x20\n\x0bKFHANKAEJFJ\x18\x0c\x20\x01(\rR\x0bKFHAN\
    KAEJFJ\x12\x20\n\x0bIBJDEGFNOIN\x18\t\x20\x01(\rR\x0bIBJDEGFNOIN\x12\x20\
    \n\x0bOFECNECFJJI\x18\x06\x20\x01(\x08R\x0bOFECNECFJJI\x12\x20\n\x0bDBKH\
    FAEKNKL\x18\x03\x20\x01(\rR\x0bDBKHFAEKNKL\x12\x20\n\x0bOGMPDLCDJDL\x18\
    \x08\x20\x01(\rR\x0bOGMPDLCDJDL\x12\x20\n\x0bMGGDEINKDMH\x18\x01\x20\x01\
    (\rR\x0bMGGDEINKDMHb\x06proto3\
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
            deps.push(super::BattleTarget::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeBossPhaseSettleNotify::generated_message_descriptor_data());
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
