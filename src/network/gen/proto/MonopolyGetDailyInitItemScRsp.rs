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

//! Generated file from `MonopolyGetDailyInitItemScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MonopolyGetDailyInitItemScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MonopolyGetDailyInitItemScRsp {
    // message fields
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.DPOMMIEBKFO)
    pub DPOMMIEBKFO: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.FGCHMCCHEOD)
    pub FGCHMCCHEOD: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.OHEKKIHEJIO)
    pub OHEKKIHEJIO: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.GOABEHOOEBJ)
    pub GOABEHOOEBJ: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.KIKDJBEDIMB)
    pub KIKDJBEDIMB: u32,
    // @@protoc_insertion_point(field:MonopolyGetDailyInitItemScRsp.IEKHFILHDPG)
    pub IEKHFILHDPG: i64,
    // special fields
    // @@protoc_insertion_point(special_field:MonopolyGetDailyInitItemScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MonopolyGetDailyInitItemScRsp {
    fn default() -> &'a MonopolyGetDailyInitItemScRsp {
        <MonopolyGetDailyInitItemScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MonopolyGetDailyInitItemScRsp {
    pub fn new() -> MonopolyGetDailyInitItemScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DPOMMIEBKFO",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.DPOMMIEBKFO },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.DPOMMIEBKFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGCHMCCHEOD",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.FGCHMCCHEOD },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.FGCHMCCHEOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OHEKKIHEJIO",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.OHEKKIHEJIO },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.OHEKKIHEJIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOABEHOOEBJ",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.GOABEHOOEBJ },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.GOABEHOOEBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.ADADHIHDHJC },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIKDJBEDIMB",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.KIKDJBEDIMB },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.KIKDJBEDIMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IEKHFILHDPG",
            |m: &MonopolyGetDailyInitItemScRsp| { &m.IEKHFILHDPG },
            |m: &mut MonopolyGetDailyInitItemScRsp| { &mut m.IEKHFILHDPG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MonopolyGetDailyInitItemScRsp>(
            "MonopolyGetDailyInitItemScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MonopolyGetDailyInitItemScRsp {
    const NAME: &'static str = "MonopolyGetDailyInitItemScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.DPOMMIEBKFO = is.read_uint32()?;
                },
                72 => {
                    self.FGCHMCCHEOD = is.read_uint32()?;
                },
                120 => {
                    self.OHEKKIHEJIO = is.read_uint32()?;
                },
                32 => {
                    self.GOABEHOOEBJ = is.read_uint32()?;
                },
                64 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                88 => {
                    self.KIKDJBEDIMB = is.read_uint32()?;
                },
                40 => {
                    self.IEKHFILHDPG = is.read_int64()?;
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
        if self.DPOMMIEBKFO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DPOMMIEBKFO);
        }
        if self.FGCHMCCHEOD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FGCHMCCHEOD);
        }
        if self.OHEKKIHEJIO != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OHEKKIHEJIO);
        }
        if self.GOABEHOOEBJ != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.GOABEHOOEBJ);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ADADHIHDHJC);
        }
        if self.KIKDJBEDIMB != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.KIKDJBEDIMB);
        }
        if self.IEKHFILHDPG != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.IEKHFILHDPG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DPOMMIEBKFO != 0 {
            os.write_uint32(1, self.DPOMMIEBKFO)?;
        }
        if self.FGCHMCCHEOD != 0 {
            os.write_uint32(9, self.FGCHMCCHEOD)?;
        }
        if self.OHEKKIHEJIO != 0 {
            os.write_uint32(15, self.OHEKKIHEJIO)?;
        }
        if self.GOABEHOOEBJ != 0 {
            os.write_uint32(4, self.GOABEHOOEBJ)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(8, self.ADADHIHDHJC)?;
        }
        if self.KIKDJBEDIMB != 0 {
            os.write_uint32(11, self.KIKDJBEDIMB)?;
        }
        if self.IEKHFILHDPG != 0 {
            os.write_int64(5, self.IEKHFILHDPG)?;
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

    fn new() -> MonopolyGetDailyInitItemScRsp {
        MonopolyGetDailyInitItemScRsp::new()
    }

    fn clear(&mut self) {
        self.DPOMMIEBKFO = 0;
        self.FGCHMCCHEOD = 0;
        self.OHEKKIHEJIO = 0;
        self.GOABEHOOEBJ = 0;
        self.ADADHIHDHJC = 0;
        self.KIKDJBEDIMB = 0;
        self.IEKHFILHDPG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MonopolyGetDailyInitItemScRsp {
        static instance: MonopolyGetDailyInitItemScRsp = MonopolyGetDailyInitItemScRsp {
            DPOMMIEBKFO: 0,
            FGCHMCCHEOD: 0,
            OHEKKIHEJIO: 0,
            GOABEHOOEBJ: 0,
            ADADHIHDHJC: 0,
            KIKDJBEDIMB: 0,
            IEKHFILHDPG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MonopolyGetDailyInitItemScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MonopolyGetDailyInitItemScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MonopolyGetDailyInitItemScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonopolyGetDailyInitItemScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#MonopolyGetDailyInitItemScRsp.proto\"\x8d\x02\n\x1dMonopolyGetDailyIn\
    itItemScRsp\x12\x20\n\x0bDPOMMIEBKFO\x18\x01\x20\x01(\rR\x0bDPOMMIEBKFO\
    \x12\x20\n\x0bFGCHMCCHEOD\x18\t\x20\x01(\rR\x0bFGCHMCCHEOD\x12\x20\n\x0b\
    OHEKKIHEJIO\x18\x0f\x20\x01(\rR\x0bOHEKKIHEJIO\x12\x20\n\x0bGOABEHOOEBJ\
    \x18\x04\x20\x01(\rR\x0bGOABEHOOEBJ\x12\x20\n\x0bADADHIHDHJC\x18\x08\x20\
    \x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bKIKDJBEDIMB\x18\x0b\x20\x01(\rR\x0b\
    KIKDJBEDIMB\x12\x20\n\x0bIEKHFILHDPG\x18\x05\x20\x01(\x03R\x0bIEKHFILHDP\
    Gb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MonopolyGetDailyInitItemScRsp::generated_message_descriptor_data());
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
