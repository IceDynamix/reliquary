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

//! Generated file from `PlayerLoginScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PlayerLoginScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerLoginScRsp {
    // message fields
    // @@protoc_insertion_point(field:PlayerLoginScRsp.JOAMEJFEOAM)
    pub JOAMEJFEOAM: u32,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.JLPKEOBINCP)
    pub JLPKEOBINCP: bool,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.GPKGJLNHPJK)
    pub GPKGJLNHPJK: ::protobuf::MessageField<super::DJOLEKKPBGB::DJOLEKKPBGB>,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.PDIKPEIFANN)
    pub PDIKPEIFANN: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.NHMHABJKHOG)
    pub NHMHABJKHOG: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.NMOOODOHHCI)
    pub NMOOODOHHCI: u64,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.LOGOEFCPCHK)
    pub LOGOEFCPCHK: u64,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.IGKBEAMLNBJ)
    pub IGKBEAMLNBJ: bool,
    // @@protoc_insertion_point(field:PlayerLoginScRsp.FLOEMBLNODK)
    pub FLOEMBLNODK: i32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerLoginScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerLoginScRsp {
    fn default() -> &'a PlayerLoginScRsp {
        <PlayerLoginScRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlayerLoginScRsp {
    pub fn new() -> PlayerLoginScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JOAMEJFEOAM",
            |m: &PlayerLoginScRsp| { &m.JOAMEJFEOAM },
            |m: &mut PlayerLoginScRsp| { &mut m.JOAMEJFEOAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLPKEOBINCP",
            |m: &PlayerLoginScRsp| { &m.JLPKEOBINCP },
            |m: &mut PlayerLoginScRsp| { &mut m.JLPKEOBINCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DJOLEKKPBGB::DJOLEKKPBGB>(
            "GPKGJLNHPJK",
            |m: &PlayerLoginScRsp| { &m.GPKGJLNHPJK },
            |m: &mut PlayerLoginScRsp| { &mut m.GPKGJLNHPJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDIKPEIFANN",
            |m: &PlayerLoginScRsp| { &m.PDIKPEIFANN },
            |m: &mut PlayerLoginScRsp| { &mut m.PDIKPEIFANN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHMHABJKHOG",
            |m: &PlayerLoginScRsp| { &m.NHMHABJKHOG },
            |m: &mut PlayerLoginScRsp| { &mut m.NHMHABJKHOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &PlayerLoginScRsp| { &m.retcode },
            |m: &mut PlayerLoginScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMOOODOHHCI",
            |m: &PlayerLoginScRsp| { &m.NMOOODOHHCI },
            |m: &mut PlayerLoginScRsp| { &mut m.NMOOODOHHCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOGOEFCPCHK",
            |m: &PlayerLoginScRsp| { &m.LOGOEFCPCHK },
            |m: &mut PlayerLoginScRsp| { &mut m.LOGOEFCPCHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGKBEAMLNBJ",
            |m: &PlayerLoginScRsp| { &m.IGKBEAMLNBJ },
            |m: &mut PlayerLoginScRsp| { &mut m.IGKBEAMLNBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FLOEMBLNODK",
            |m: &PlayerLoginScRsp| { &m.FLOEMBLNODK },
            |m: &mut PlayerLoginScRsp| { &mut m.FLOEMBLNODK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerLoginScRsp>(
            "PlayerLoginScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerLoginScRsp {
    const NAME: &'static str = "PlayerLoginScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.JOAMEJFEOAM = is.read_uint32()?;
                },
                64 => {
                    self.JLPKEOBINCP = is.read_bool()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GPKGJLNHPJK)?;
                },
                26 => {
                    self.PDIKPEIFANN = is.read_string()?;
                },
                114 => {
                    self.NHMHABJKHOG = is.read_string()?;
                },
                120 => {
                    self.retcode = is.read_uint32()?;
                },
                56 => {
                    self.NMOOODOHHCI = is.read_uint64()?;
                },
                40 => {
                    self.LOGOEFCPCHK = is.read_uint64()?;
                },
                72 => {
                    self.IGKBEAMLNBJ = is.read_bool()?;
                },
                104 => {
                    self.FLOEMBLNODK = is.read_int32()?;
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
        if self.JOAMEJFEOAM != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.JOAMEJFEOAM);
        }
        if self.JLPKEOBINCP != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.GPKGJLNHPJK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.PDIKPEIFANN.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.PDIKPEIFANN);
        }
        if !self.NHMHABJKHOG.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.NHMHABJKHOG);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        if self.NMOOODOHHCI != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.NMOOODOHHCI);
        }
        if self.LOGOEFCPCHK != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.LOGOEFCPCHK);
        }
        if self.IGKBEAMLNBJ != false {
            my_size += 1 + 1;
        }
        if self.FLOEMBLNODK != 0 {
            my_size += ::protobuf::rt::int32_size(13, self.FLOEMBLNODK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JOAMEJFEOAM != 0 {
            os.write_uint32(12, self.JOAMEJFEOAM)?;
        }
        if self.JLPKEOBINCP != false {
            os.write_bool(8, self.JLPKEOBINCP)?;
        }
        if let Some(v) = self.GPKGJLNHPJK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if !self.PDIKPEIFANN.is_empty() {
            os.write_string(3, &self.PDIKPEIFANN)?;
        }
        if !self.NHMHABJKHOG.is_empty() {
            os.write_string(14, &self.NHMHABJKHOG)?;
        }
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
        }
        if self.NMOOODOHHCI != 0 {
            os.write_uint64(7, self.NMOOODOHHCI)?;
        }
        if self.LOGOEFCPCHK != 0 {
            os.write_uint64(5, self.LOGOEFCPCHK)?;
        }
        if self.IGKBEAMLNBJ != false {
            os.write_bool(9, self.IGKBEAMLNBJ)?;
        }
        if self.FLOEMBLNODK != 0 {
            os.write_int32(13, self.FLOEMBLNODK)?;
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

    fn new() -> PlayerLoginScRsp {
        PlayerLoginScRsp::new()
    }

    fn clear(&mut self) {
        self.JOAMEJFEOAM = 0;
        self.JLPKEOBINCP = false;
        self.GPKGJLNHPJK.clear();
        self.PDIKPEIFANN.clear();
        self.NHMHABJKHOG.clear();
        self.retcode = 0;
        self.NMOOODOHHCI = 0;
        self.LOGOEFCPCHK = 0;
        self.IGKBEAMLNBJ = false;
        self.FLOEMBLNODK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerLoginScRsp {
        static instance: PlayerLoginScRsp = PlayerLoginScRsp {
            JOAMEJFEOAM: 0,
            JLPKEOBINCP: false,
            GPKGJLNHPJK: ::protobuf::MessageField::none(),
            PDIKPEIFANN: ::std::string::String::new(),
            NHMHABJKHOG: ::std::string::String::new(),
            retcode: 0,
            NMOOODOHHCI: 0,
            LOGOEFCPCHK: 0,
            IGKBEAMLNBJ: false,
            FLOEMBLNODK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerLoginScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerLoginScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerLoginScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerLoginScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerLoginScRsp.proto\x1a\x11DJOLEKKPBGB.proto\"\xec\x02\n\x10Pla\
    yerLoginScRsp\x12\x20\n\x0bJOAMEJFEOAM\x18\x0c\x20\x01(\rR\x0bJOAMEJFEOA\
    M\x12\x20\n\x0bJLPKEOBINCP\x18\x08\x20\x01(\x08R\x0bJLPKEOBINCP\x12.\n\
    \x0bGPKGJLNHPJK\x18\x02\x20\x01(\x0b2\x0c.DJOLEKKPBGBR\x0bGPKGJLNHPJK\
    \x12\x20\n\x0bPDIKPEIFANN\x18\x03\x20\x01(\tR\x0bPDIKPEIFANN\x12\x20\n\
    \x0bNHMHABJKHOG\x18\x0e\x20\x01(\tR\x0bNHMHABJKHOG\x12\x18\n\x07retcode\
    \x18\x0f\x20\x01(\rR\x07retcode\x12\x20\n\x0bNMOOODOHHCI\x18\x07\x20\x01\
    (\x04R\x0bNMOOODOHHCI\x12\x20\n\x0bLOGOEFCPCHK\x18\x05\x20\x01(\x04R\x0b\
    LOGOEFCPCHK\x12\x20\n\x0bIGKBEAMLNBJ\x18\t\x20\x01(\x08R\x0bIGKBEAMLNBJ\
    \x12\x20\n\x0bFLOEMBLNODK\x18\r\x20\x01(\x05R\x0bFLOEMBLNODKb\x06proto3\
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
            deps.push(super::DJOLEKKPBGB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerLoginScRsp::generated_message_descriptor_data());
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
