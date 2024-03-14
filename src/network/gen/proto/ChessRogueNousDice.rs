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

//! Generated file from `ChessRogueNousDice.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueNousDice)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueNousDice {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueNousDice.GFGCPJOHMEL)
    pub GFGCPJOHMEL: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.cur_surface_index)
    pub cur_surface_index: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.NOGJMMEDGLL)
    pub NOGJMMEDGLL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueNousDice.reroll_times)
    pub reroll_times: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.dice_info)
    pub dice_info: ::protobuf::MessageField<super::ChessRogueNousDiceInfo::ChessRogueNousDiceInfo>,
    // @@protoc_insertion_point(field:ChessRogueNousDice.dice_branch_id_remote)
    pub dice_branch_id_remote: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.dice_branch_id)
    pub dice_branch_id: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.CHGLMLDAMJH)
    pub CHGLMLDAMJH: bool,
    // @@protoc_insertion_point(field:ChessRogueNousDice.cheat_times)
    pub cheat_times: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.HIJBOPMGBAP)
    pub HIJBOPMGBAP: u32,
    // @@protoc_insertion_point(field:ChessRogueNousDice.BDENADDOCAC)
    pub BDENADDOCAC: bool,
    // @@protoc_insertion_point(field:ChessRogueNousDice.cur_surface_id)
    pub cur_surface_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueNousDice.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueNousDice {
    fn default() -> &'a ChessRogueNousDice {
        <ChessRogueNousDice as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueNousDice {
    pub fn new() -> ChessRogueNousDice {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFGCPJOHMEL",
            |m: &ChessRogueNousDice| { &m.GFGCPJOHMEL },
            |m: &mut ChessRogueNousDice| { &mut m.GFGCPJOHMEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_surface_index",
            |m: &ChessRogueNousDice| { &m.cur_surface_index },
            |m: &mut ChessRogueNousDice| { &mut m.cur_surface_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NOGJMMEDGLL",
            |m: &ChessRogueNousDice| { &m.NOGJMMEDGLL },
            |m: &mut ChessRogueNousDice| { &mut m.NOGJMMEDGLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reroll_times",
            |m: &ChessRogueNousDice| { &m.reroll_times },
            |m: &mut ChessRogueNousDice| { &mut m.reroll_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueNousDiceInfo::ChessRogueNousDiceInfo>(
            "dice_info",
            |m: &ChessRogueNousDice| { &m.dice_info },
            |m: &mut ChessRogueNousDice| { &mut m.dice_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dice_branch_id_remote",
            |m: &ChessRogueNousDice| { &m.dice_branch_id_remote },
            |m: &mut ChessRogueNousDice| { &mut m.dice_branch_id_remote },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dice_branch_id",
            |m: &ChessRogueNousDice| { &m.dice_branch_id },
            |m: &mut ChessRogueNousDice| { &mut m.dice_branch_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CHGLMLDAMJH",
            |m: &ChessRogueNousDice| { &m.CHGLMLDAMJH },
            |m: &mut ChessRogueNousDice| { &mut m.CHGLMLDAMJH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cheat_times",
            |m: &ChessRogueNousDice| { &m.cheat_times },
            |m: &mut ChessRogueNousDice| { &mut m.cheat_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HIJBOPMGBAP",
            |m: &ChessRogueNousDice| { &m.HIJBOPMGBAP },
            |m: &mut ChessRogueNousDice| { &mut m.HIJBOPMGBAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDENADDOCAC",
            |m: &ChessRogueNousDice| { &m.BDENADDOCAC },
            |m: &mut ChessRogueNousDice| { &mut m.BDENADDOCAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_surface_id",
            |m: &ChessRogueNousDice| { &m.cur_surface_id },
            |m: &mut ChessRogueNousDice| { &mut m.cur_surface_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueNousDice>(
            "ChessRogueNousDice",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueNousDice {
    const NAME: &'static str = "ChessRogueNousDice";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.GFGCPJOHMEL = is.read_uint32()?;
                },
                104 => {
                    self.cur_surface_index = is.read_uint32()?;
                },
                6666 => {
                    is.read_repeated_packed_uint32_into(&mut self.NOGJMMEDGLL)?;
                },
                6664 => {
                    self.NOGJMMEDGLL.push(is.read_uint32()?);
                },
                16 => {
                    self.reroll_times = is.read_uint32()?;
                },
                1666 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.dice_info)?;
                },
                112 => {
                    self.dice_branch_id_remote = is.read_uint32()?;
                },
                8 => {
                    self.dice_branch_id = is.read_uint32()?;
                },
                9480 => {
                    self.CHGLMLDAMJH = is.read_bool()?;
                },
                32 => {
                    self.cheat_times = is.read_uint32()?;
                },
                40 => {
                    self.HIJBOPMGBAP = is.read_uint32()?;
                },
                80 => {
                    self.BDENADDOCAC = is.read_bool()?;
                },
                24 => {
                    self.cur_surface_id = is.read_uint32()?;
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
        if self.GFGCPJOHMEL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.GFGCPJOHMEL);
        }
        if self.cur_surface_index != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.cur_surface_index);
        }
        for value in &self.NOGJMMEDGLL {
            my_size += ::protobuf::rt::uint32_size(833, *value);
        };
        if self.reroll_times != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.reroll_times);
        }
        if let Some(v) = self.dice_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.dice_branch_id_remote != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.dice_branch_id_remote);
        }
        if self.dice_branch_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.dice_branch_id);
        }
        if self.CHGLMLDAMJH != false {
            my_size += 2 + 1;
        }
        if self.cheat_times != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.cheat_times);
        }
        if self.HIJBOPMGBAP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.HIJBOPMGBAP);
        }
        if self.BDENADDOCAC != false {
            my_size += 1 + 1;
        }
        if self.cur_surface_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.cur_surface_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GFGCPJOHMEL != 0 {
            os.write_uint32(9, self.GFGCPJOHMEL)?;
        }
        if self.cur_surface_index != 0 {
            os.write_uint32(13, self.cur_surface_index)?;
        }
        for v in &self.NOGJMMEDGLL {
            os.write_uint32(833, *v)?;
        };
        if self.reroll_times != 0 {
            os.write_uint32(2, self.reroll_times)?;
        }
        if let Some(v) = self.dice_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(208, v, os)?;
        }
        if self.dice_branch_id_remote != 0 {
            os.write_uint32(14, self.dice_branch_id_remote)?;
        }
        if self.dice_branch_id != 0 {
            os.write_uint32(1, self.dice_branch_id)?;
        }
        if self.CHGLMLDAMJH != false {
            os.write_bool(1185, self.CHGLMLDAMJH)?;
        }
        if self.cheat_times != 0 {
            os.write_uint32(4, self.cheat_times)?;
        }
        if self.HIJBOPMGBAP != 0 {
            os.write_uint32(5, self.HIJBOPMGBAP)?;
        }
        if self.BDENADDOCAC != false {
            os.write_bool(10, self.BDENADDOCAC)?;
        }
        if self.cur_surface_id != 0 {
            os.write_uint32(3, self.cur_surface_id)?;
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

    fn new() -> ChessRogueNousDice {
        ChessRogueNousDice::new()
    }

    fn clear(&mut self) {
        self.GFGCPJOHMEL = 0;
        self.cur_surface_index = 0;
        self.NOGJMMEDGLL.clear();
        self.reroll_times = 0;
        self.dice_info.clear();
        self.dice_branch_id_remote = 0;
        self.dice_branch_id = 0;
        self.CHGLMLDAMJH = false;
        self.cheat_times = 0;
        self.HIJBOPMGBAP = 0;
        self.BDENADDOCAC = false;
        self.cur_surface_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueNousDice {
        static instance: ChessRogueNousDice = ChessRogueNousDice {
            GFGCPJOHMEL: 0,
            cur_surface_index: 0,
            NOGJMMEDGLL: ::std::vec::Vec::new(),
            reroll_times: 0,
            dice_info: ::protobuf::MessageField::none(),
            dice_branch_id_remote: 0,
            dice_branch_id: 0,
            CHGLMLDAMJH: false,
            cheat_times: 0,
            HIJBOPMGBAP: 0,
            BDENADDOCAC: false,
            cur_surface_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueNousDice {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueNousDice").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueNousDice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueNousDice {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ChessRogueNousDice.proto\x1a\x1cChessRogueNousDiceInfo.proto\"\xe6\
    \x03\n\x12ChessRogueNousDice\x12\x20\n\x0bGFGCPJOHMEL\x18\t\x20\x01(\rR\
    \x0bGFGCPJOHMEL\x12*\n\x11cur_surface_index\x18\r\x20\x01(\rR\x0fcurSurf\
    aceIndex\x12!\n\x0bNOGJMMEDGLL\x18\xc1\x06\x20\x03(\rR\x0bNOGJMMEDGLL\
    \x12!\n\x0creroll_times\x18\x02\x20\x01(\rR\x0brerollTimes\x125\n\tdice_\
    info\x18\xd0\x01\x20\x01(\x0b2\x17.ChessRogueNousDiceInfoR\x08diceInfo\
    \x121\n\x15dice_branch_id_remote\x18\x0e\x20\x01(\rR\x12diceBranchIdRemo\
    te\x12$\n\x0edice_branch_id\x18\x01\x20\x01(\rR\x0cdiceBranchId\x12!\n\
    \x0bCHGLMLDAMJH\x18\xa1\t\x20\x01(\x08R\x0bCHGLMLDAMJH\x12\x1f\n\x0bchea\
    t_times\x18\x04\x20\x01(\rR\ncheatTimes\x12\x20\n\x0bHIJBOPMGBAP\x18\x05\
    \x20\x01(\rR\x0bHIJBOPMGBAP\x12\x20\n\x0bBDENADDOCAC\x18\n\x20\x01(\x08R\
    \x0bBDENADDOCAC\x12$\n\x0ecur_surface_id\x18\x03\x20\x01(\rR\x0ccurSurfa\
    ceIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ChessRogueNousDiceInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueNousDice::generated_message_descriptor_data());
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