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

//! Generated file from `MusicRhythmDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MusicRhythmDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicRhythmDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.GBMLNHOCJMO)
    pub GBMLNHOCJMO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.current_song_id)
    pub current_song_id: u32,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.OPFOILFDBKG)
    pub OPFOILFDBKG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.FEAHHAMLDFB)
    pub FEAHHAMLDFB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.FCLINCKMILK)
    pub FCLINCKMILK: bool,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.music_rhythm_groups)
    pub music_rhythm_groups: ::std::vec::Vec<super::MusicRhythmGroup::MusicRhythmGroup>,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.music_rhythm_level)
    pub music_rhythm_level: ::std::vec::Vec<super::MusicRhythmLevel::MusicRhythmLevel>,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.current_level_id)
    pub current_level_id: u32,
    // @@protoc_insertion_point(field:MusicRhythmDataScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MusicRhythmDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicRhythmDataScRsp {
    fn default() -> &'a MusicRhythmDataScRsp {
        <MusicRhythmDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MusicRhythmDataScRsp {
    pub fn new() -> MusicRhythmDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GBMLNHOCJMO",
            |m: &MusicRhythmDataScRsp| { &m.GBMLNHOCJMO },
            |m: &mut MusicRhythmDataScRsp| { &mut m.GBMLNHOCJMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "current_song_id",
            |m: &MusicRhythmDataScRsp| { &m.current_song_id },
            |m: &mut MusicRhythmDataScRsp| { &mut m.current_song_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OPFOILFDBKG",
            |m: &MusicRhythmDataScRsp| { &m.OPFOILFDBKG },
            |m: &mut MusicRhythmDataScRsp| { &mut m.OPFOILFDBKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FEAHHAMLDFB",
            |m: &MusicRhythmDataScRsp| { &m.FEAHHAMLDFB },
            |m: &mut MusicRhythmDataScRsp| { &mut m.FEAHHAMLDFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCLINCKMILK",
            |m: &MusicRhythmDataScRsp| { &m.FCLINCKMILK },
            |m: &mut MusicRhythmDataScRsp| { &mut m.FCLINCKMILK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "music_rhythm_groups",
            |m: &MusicRhythmDataScRsp| { &m.music_rhythm_groups },
            |m: &mut MusicRhythmDataScRsp| { &mut m.music_rhythm_groups },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "music_rhythm_level",
            |m: &MusicRhythmDataScRsp| { &m.music_rhythm_level },
            |m: &mut MusicRhythmDataScRsp| { &mut m.music_rhythm_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "current_level_id",
            |m: &MusicRhythmDataScRsp| { &m.current_level_id },
            |m: &mut MusicRhythmDataScRsp| { &mut m.current_level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MusicRhythmDataScRsp| { &m.retcode },
            |m: &mut MusicRhythmDataScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicRhythmDataScRsp>(
            "MusicRhythmDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicRhythmDataScRsp {
    const NAME: &'static str = "MusicRhythmDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.GBMLNHOCJMO)?;
                },
                120 => {
                    self.GBMLNHOCJMO.push(is.read_uint32()?);
                },
                16 => {
                    self.current_song_id = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.OPFOILFDBKG)?;
                },
                64 => {
                    self.OPFOILFDBKG.push(is.read_uint32()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.FEAHHAMLDFB)?;
                },
                40 => {
                    self.FEAHHAMLDFB.push(is.read_uint32()?);
                },
                48 => {
                    self.FCLINCKMILK = is.read_bool()?;
                },
                10 => {
                    self.music_rhythm_groups.push(is.read_message()?);
                },
                106 => {
                    self.music_rhythm_level.push(is.read_message()?);
                },
                80 => {
                    self.current_level_id = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
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
        for value in &self.GBMLNHOCJMO {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.current_song_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.current_song_id);
        }
        for value in &self.OPFOILFDBKG {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.FEAHHAMLDFB {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.FCLINCKMILK != false {
            my_size += 1 + 1;
        }
        for value in &self.music_rhythm_groups {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.music_rhythm_level {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.current_level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.current_level_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GBMLNHOCJMO {
            os.write_uint32(15, *v)?;
        };
        if self.current_song_id != 0 {
            os.write_uint32(2, self.current_song_id)?;
        }
        for v in &self.OPFOILFDBKG {
            os.write_uint32(8, *v)?;
        };
        for v in &self.FEAHHAMLDFB {
            os.write_uint32(5, *v)?;
        };
        if self.FCLINCKMILK != false {
            os.write_bool(6, self.FCLINCKMILK)?;
        }
        for v in &self.music_rhythm_groups {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.music_rhythm_level {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.current_level_id != 0 {
            os.write_uint32(10, self.current_level_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
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

    fn new() -> MusicRhythmDataScRsp {
        MusicRhythmDataScRsp::new()
    }

    fn clear(&mut self) {
        self.GBMLNHOCJMO.clear();
        self.current_song_id = 0;
        self.OPFOILFDBKG.clear();
        self.FEAHHAMLDFB.clear();
        self.FCLINCKMILK = false;
        self.music_rhythm_groups.clear();
        self.music_rhythm_level.clear();
        self.current_level_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicRhythmDataScRsp {
        static instance: MusicRhythmDataScRsp = MusicRhythmDataScRsp {
            GBMLNHOCJMO: ::std::vec::Vec::new(),
            current_song_id: 0,
            OPFOILFDBKG: ::std::vec::Vec::new(),
            FEAHHAMLDFB: ::std::vec::Vec::new(),
            FCLINCKMILK: false,
            music_rhythm_groups: ::std::vec::Vec::new(),
            music_rhythm_level: ::std::vec::Vec::new(),
            current_level_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MusicRhythmDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicRhythmDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicRhythmDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicRhythmDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aMusicRhythmDataScRsp.proto\x1a\x16MusicRhythmGroup.proto\x1a\x16Mu\
    sicRhythmLevel.proto\"\x8e\x03\n\x14MusicRhythmDataScRsp\x12\x20\n\x0bGB\
    MLNHOCJMO\x18\x0f\x20\x03(\rR\x0bGBMLNHOCJMO\x12&\n\x0fcurrent_song_id\
    \x18\x02\x20\x01(\rR\rcurrentSongId\x12\x20\n\x0bOPFOILFDBKG\x18\x08\x20\
    \x03(\rR\x0bOPFOILFDBKG\x12\x20\n\x0bFEAHHAMLDFB\x18\x05\x20\x03(\rR\x0b\
    FEAHHAMLDFB\x12\x20\n\x0bFCLINCKMILK\x18\x06\x20\x01(\x08R\x0bFCLINCKMIL\
    K\x12A\n\x13music_rhythm_groups\x18\x01\x20\x03(\x0b2\x11.MusicRhythmGro\
    upR\x11musicRhythmGroups\x12?\n\x12music_rhythm_level\x18\r\x20\x03(\x0b\
    2\x11.MusicRhythmLevelR\x10musicRhythmLevel\x12(\n\x10current_level_id\
    \x18\n\x20\x01(\rR\x0ecurrentLevelId\x12\x18\n\x07retcode\x18\x0c\x20\
    \x01(\rR\x07retcodeB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::MusicRhythmGroup::file_descriptor().clone());
            deps.push(super::MusicRhythmLevel::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MusicRhythmDataScRsp::generated_message_descriptor_data());
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
