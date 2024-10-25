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

//! Generated file from `JMDAEBFHNAI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JMDAEBFHNAI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JMDAEBFHNAI {
    // message fields
    // @@protoc_insertion_point(field:JMDAEBFHNAI.EPJPMBMOKAJ)
    pub EPJPMBMOKAJ: u32,
    // @@protoc_insertion_point(field:JMDAEBFHNAI.NGKABPLAEGP)
    pub NGKABPLAEGP: u32,
    // @@protoc_insertion_point(field:JMDAEBFHNAI.DBKHFAEKNKL)
    pub DBKHFAEKNKL: u32,
    // @@protoc_insertion_point(field:JMDAEBFHNAI.HLJELCNLJKI)
    pub HLJELCNLJKI: ::protobuf::EnumOrUnknown<super::AAAIFJOAAGO::AAAIFJOAAGO>,
    // special fields
    // @@protoc_insertion_point(special_field:JMDAEBFHNAI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JMDAEBFHNAI {
    fn default() -> &'a JMDAEBFHNAI {
        <JMDAEBFHNAI as ::protobuf::Message>::default_instance()
    }
}

impl JMDAEBFHNAI {
    pub fn new() -> JMDAEBFHNAI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPJPMBMOKAJ",
            |m: &JMDAEBFHNAI| { &m.EPJPMBMOKAJ },
            |m: &mut JMDAEBFHNAI| { &mut m.EPJPMBMOKAJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NGKABPLAEGP",
            |m: &JMDAEBFHNAI| { &m.NGKABPLAEGP },
            |m: &mut JMDAEBFHNAI| { &mut m.NGKABPLAEGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBKHFAEKNKL",
            |m: &JMDAEBFHNAI| { &m.DBKHFAEKNKL },
            |m: &mut JMDAEBFHNAI| { &mut m.DBKHFAEKNKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLJELCNLJKI",
            |m: &JMDAEBFHNAI| { &m.HLJELCNLJKI },
            |m: &mut JMDAEBFHNAI| { &mut m.HLJELCNLJKI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JMDAEBFHNAI>(
            "JMDAEBFHNAI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JMDAEBFHNAI {
    const NAME: &'static str = "JMDAEBFHNAI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.EPJPMBMOKAJ = is.read_uint32()?;
                },
                64 => {
                    self.NGKABPLAEGP = is.read_uint32()?;
                },
                32 => {
                    self.DBKHFAEKNKL = is.read_uint32()?;
                },
                16 => {
                    self.HLJELCNLJKI = is.read_enum_or_unknown()?;
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
        if self.EPJPMBMOKAJ != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.EPJPMBMOKAJ);
        }
        if self.NGKABPLAEGP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.NGKABPLAEGP);
        }
        if self.DBKHFAEKNKL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DBKHFAEKNKL);
        }
        if self.HLJELCNLJKI != ::protobuf::EnumOrUnknown::new(super::AAAIFJOAAGO::AAAIFJOAAGO::FIGHT_FEST_BATTLE_RANK_C) {
            my_size += ::protobuf::rt::int32_size(2, self.HLJELCNLJKI.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EPJPMBMOKAJ != 0 {
            os.write_uint32(10, self.EPJPMBMOKAJ)?;
        }
        if self.NGKABPLAEGP != 0 {
            os.write_uint32(8, self.NGKABPLAEGP)?;
        }
        if self.DBKHFAEKNKL != 0 {
            os.write_uint32(4, self.DBKHFAEKNKL)?;
        }
        if self.HLJELCNLJKI != ::protobuf::EnumOrUnknown::new(super::AAAIFJOAAGO::AAAIFJOAAGO::FIGHT_FEST_BATTLE_RANK_C) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.HLJELCNLJKI))?;
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

    fn new() -> JMDAEBFHNAI {
        JMDAEBFHNAI::new()
    }

    fn clear(&mut self) {
        self.EPJPMBMOKAJ = 0;
        self.NGKABPLAEGP = 0;
        self.DBKHFAEKNKL = 0;
        self.HLJELCNLJKI = ::protobuf::EnumOrUnknown::new(super::AAAIFJOAAGO::AAAIFJOAAGO::FIGHT_FEST_BATTLE_RANK_C);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JMDAEBFHNAI {
        static instance: JMDAEBFHNAI = JMDAEBFHNAI {
            EPJPMBMOKAJ: 0,
            NGKABPLAEGP: 0,
            DBKHFAEKNKL: 0,
            HLJELCNLJKI: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JMDAEBFHNAI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JMDAEBFHNAI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JMDAEBFHNAI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JMDAEBFHNAI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JMDAEBFHNAI.proto\x1a\x11AAAIFJOAAGO.proto\"\xa3\x01\n\x0bJMDAEBFH\
    NAI\x12\x20\n\x0bEPJPMBMOKAJ\x18\n\x20\x01(\rR\x0bEPJPMBMOKAJ\x12\x20\n\
    \x0bNGKABPLAEGP\x18\x08\x20\x01(\rR\x0bNGKABPLAEGP\x12\x20\n\x0bDBKHFAEK\
    NKL\x18\x04\x20\x01(\rR\x0bDBKHFAEKNKL\x12.\n\x0bHLJELCNLJKI\x18\x02\x20\
    \x01(\x0e2\x0c.AAAIFJOAAGOR\x0bHLJELCNLJKIb\x06proto3\
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
            deps.push(super::AAAIFJOAAGO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JMDAEBFHNAI::generated_message_descriptor_data());
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
