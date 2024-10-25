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

//! Generated file from `ChessRogueCheatRollScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueCheatRollScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueCheatRollScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueCheatRollScRsp.LHINNDFFHHC)
    pub LHINNDFFHHC: ::protobuf::MessageField<super::LNFJDOCNPOM::LNFJDOCNPOM>,
    // @@protoc_insertion_point(field:ChessRogueCheatRollScRsp.LGEMOEGMGPP)
    pub LGEMOEGMGPP: u32,
    // @@protoc_insertion_point(field:ChessRogueCheatRollScRsp.CFKJKLEGNAP)
    pub CFKJKLEGNAP: u32,
    // @@protoc_insertion_point(field:ChessRogueCheatRollScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueCheatRollScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueCheatRollScRsp {
    fn default() -> &'a ChessRogueCheatRollScRsp {
        <ChessRogueCheatRollScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueCheatRollScRsp {
    pub fn new() -> ChessRogueCheatRollScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LNFJDOCNPOM::LNFJDOCNPOM>(
            "LHINNDFFHHC",
            |m: &ChessRogueCheatRollScRsp| { &m.LHINNDFFHHC },
            |m: &mut ChessRogueCheatRollScRsp| { &mut m.LHINNDFFHHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGEMOEGMGPP",
            |m: &ChessRogueCheatRollScRsp| { &m.LGEMOEGMGPP },
            |m: &mut ChessRogueCheatRollScRsp| { &mut m.LGEMOEGMGPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFKJKLEGNAP",
            |m: &ChessRogueCheatRollScRsp| { &m.CFKJKLEGNAP },
            |m: &mut ChessRogueCheatRollScRsp| { &mut m.CFKJKLEGNAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueCheatRollScRsp| { &m.retcode },
            |m: &mut ChessRogueCheatRollScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueCheatRollScRsp>(
            "ChessRogueCheatRollScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueCheatRollScRsp {
    const NAME: &'static str = "ChessRogueCheatRollScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LHINNDFFHHC)?;
                },
                104 => {
                    self.LGEMOEGMGPP = is.read_uint32()?;
                },
                48 => {
                    self.CFKJKLEGNAP = is.read_uint32()?;
                },
                40 => {
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
        if let Some(v) = self.LHINNDFFHHC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LGEMOEGMGPP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LGEMOEGMGPP);
        }
        if self.CFKJKLEGNAP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.CFKJKLEGNAP);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.LHINNDFFHHC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.LGEMOEGMGPP != 0 {
            os.write_uint32(13, self.LGEMOEGMGPP)?;
        }
        if self.CFKJKLEGNAP != 0 {
            os.write_uint32(6, self.CFKJKLEGNAP)?;
        }
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
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

    fn new() -> ChessRogueCheatRollScRsp {
        ChessRogueCheatRollScRsp::new()
    }

    fn clear(&mut self) {
        self.LHINNDFFHHC.clear();
        self.LGEMOEGMGPP = 0;
        self.CFKJKLEGNAP = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueCheatRollScRsp {
        static instance: ChessRogueCheatRollScRsp = ChessRogueCheatRollScRsp {
            LHINNDFFHHC: ::protobuf::MessageField::none(),
            LGEMOEGMGPP: 0,
            CFKJKLEGNAP: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueCheatRollScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueCheatRollScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueCheatRollScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueCheatRollScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eChessRogueCheatRollScRsp.proto\x1a\x11LNFJDOCNPOM.proto\"\xa8\x01\
    \n\x18ChessRogueCheatRollScRsp\x12.\n\x0bLHINNDFFHHC\x18\x0e\x20\x01(\
    \x0b2\x0c.LNFJDOCNPOMR\x0bLHINNDFFHHC\x12\x20\n\x0bLGEMOEGMGPP\x18\r\x20\
    \x01(\rR\x0bLGEMOEGMGPP\x12\x20\n\x0bCFKJKLEGNAP\x18\x06\x20\x01(\rR\x0b\
    CFKJKLEGNAP\x12\x18\n\x07retcode\x18\x05\x20\x01(\rR\x07retcodeb\x06prot\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::LNFJDOCNPOM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueCheatRollScRsp::generated_message_descriptor_data());
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
