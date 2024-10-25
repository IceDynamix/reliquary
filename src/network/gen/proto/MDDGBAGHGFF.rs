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

//! Generated file from `MDDGBAGHGFF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MDDGBAGHGFF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MDDGBAGHGFF {
    // message fields
    // @@protoc_insertion_point(field:MDDGBAGHGFF.JFODJKAADCL)
    pub JFODJKAADCL: u32,
    // @@protoc_insertion_point(field:MDDGBAGHGFF.status)
    pub status: ::protobuf::EnumOrUnknown<super::IHOMJNBGCBJ::IHOMJNBGCBJ>,
    // @@protoc_insertion_point(field:MDDGBAGHGFF.FGIHHAAHGDA)
    pub FGIHHAAHGDA: u32,
    // @@protoc_insertion_point(field:MDDGBAGHGFF.PLOHLKGLOID)
    pub PLOHLKGLOID: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MDDGBAGHGFF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MDDGBAGHGFF {
    fn default() -> &'a MDDGBAGHGFF {
        <MDDGBAGHGFF as ::protobuf::Message>::default_instance()
    }
}

impl MDDGBAGHGFF {
    pub fn new() -> MDDGBAGHGFF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFODJKAADCL",
            |m: &MDDGBAGHGFF| { &m.JFODJKAADCL },
            |m: &mut MDDGBAGHGFF| { &mut m.JFODJKAADCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &MDDGBAGHGFF| { &m.status },
            |m: &mut MDDGBAGHGFF| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGIHHAAHGDA",
            |m: &MDDGBAGHGFF| { &m.FGIHHAAHGDA },
            |m: &mut MDDGBAGHGFF| { &mut m.FGIHHAAHGDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLOHLKGLOID",
            |m: &MDDGBAGHGFF| { &m.PLOHLKGLOID },
            |m: &mut MDDGBAGHGFF| { &mut m.PLOHLKGLOID },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MDDGBAGHGFF>(
            "MDDGBAGHGFF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MDDGBAGHGFF {
    const NAME: &'static str = "MDDGBAGHGFF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.JFODJKAADCL = is.read_uint32()?;
                },
                80 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.FGIHHAAHGDA = is.read_uint32()?;
                },
                120 => {
                    self.PLOHLKGLOID = is.read_uint32()?;
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
        if self.JFODJKAADCL != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.JFODJKAADCL);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::IHOMJNBGCBJ::IHOMJNBGCBJ::ROGUE_MAGIC_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.status.value());
        }
        if self.FGIHHAAHGDA != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FGIHHAAHGDA);
        }
        if self.PLOHLKGLOID != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.PLOHLKGLOID);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JFODJKAADCL != 0 {
            os.write_uint32(11, self.JFODJKAADCL)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::IHOMJNBGCBJ::IHOMJNBGCBJ::ROGUE_MAGIC_ROOM_STATUS_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.FGIHHAAHGDA != 0 {
            os.write_uint32(1, self.FGIHHAAHGDA)?;
        }
        if self.PLOHLKGLOID != 0 {
            os.write_uint32(15, self.PLOHLKGLOID)?;
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

    fn new() -> MDDGBAGHGFF {
        MDDGBAGHGFF::new()
    }

    fn clear(&mut self) {
        self.JFODJKAADCL = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::IHOMJNBGCBJ::IHOMJNBGCBJ::ROGUE_MAGIC_ROOM_STATUS_NONE);
        self.FGIHHAAHGDA = 0;
        self.PLOHLKGLOID = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MDDGBAGHGFF {
        static instance: MDDGBAGHGFF = MDDGBAGHGFF {
            JFODJKAADCL: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            FGIHHAAHGDA: 0,
            PLOHLKGLOID: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MDDGBAGHGFF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MDDGBAGHGFF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MDDGBAGHGFF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MDDGBAGHGFF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MDDGBAGHGFF.proto\x1a\x11IHOMJNBGCBJ.proto\"\x99\x01\n\x0bMDDGBAGH\
    GFF\x12\x20\n\x0bJFODJKAADCL\x18\x0b\x20\x01(\rR\x0bJFODJKAADCL\x12$\n\
    \x06status\x18\n\x20\x01(\x0e2\x0c.IHOMJNBGCBJR\x06status\x12\x20\n\x0bF\
    GIHHAAHGDA\x18\x01\x20\x01(\rR\x0bFGIHHAAHGDA\x12\x20\n\x0bPLOHLKGLOID\
    \x18\x0f\x20\x01(\rR\x0bPLOHLKGLOIDb\x06proto3\
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
            deps.push(super::IHOMJNBGCBJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MDDGBAGHGFF::generated_message_descriptor_data());
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
