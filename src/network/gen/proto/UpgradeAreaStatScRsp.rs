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

//! Generated file from `UpgradeAreaStatScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpgradeAreaStatScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpgradeAreaStatScRsp {
    // message fields
    // @@protoc_insertion_point(field:UpgradeAreaStatScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:UpgradeAreaStatScRsp.BOJBPOELFCI)
    pub BOJBPOELFCI: ::protobuf::EnumOrUnknown<super::StatType::StatType>,
    // @@protoc_insertion_point(field:UpgradeAreaStatScRsp.DBAHFEFGLMD)
    pub DBAHFEFGLMD: u32,
    // @@protoc_insertion_point(field:UpgradeAreaStatScRsp.level)
    pub level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpgradeAreaStatScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpgradeAreaStatScRsp {
    fn default() -> &'a UpgradeAreaStatScRsp {
        <UpgradeAreaStatScRsp as ::protobuf::Message>::default_instance()
    }
}

impl UpgradeAreaStatScRsp {
    pub fn new() -> UpgradeAreaStatScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &UpgradeAreaStatScRsp| { &m.retcode },
            |m: &mut UpgradeAreaStatScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BOJBPOELFCI",
            |m: &UpgradeAreaStatScRsp| { &m.BOJBPOELFCI },
            |m: &mut UpgradeAreaStatScRsp| { &mut m.BOJBPOELFCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBAHFEFGLMD",
            |m: &UpgradeAreaStatScRsp| { &m.DBAHFEFGLMD },
            |m: &mut UpgradeAreaStatScRsp| { &mut m.DBAHFEFGLMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &UpgradeAreaStatScRsp| { &m.level },
            |m: &mut UpgradeAreaStatScRsp| { &mut m.level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpgradeAreaStatScRsp>(
            "UpgradeAreaStatScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpgradeAreaStatScRsp {
    const NAME: &'static str = "UpgradeAreaStatScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.BOJBPOELFCI = is.read_enum_or_unknown()?;
                },
                64 => {
                    self.DBAHFEFGLMD = is.read_uint32()?;
                },
                40 => {
                    self.level = is.read_uint32()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        if self.BOJBPOELFCI != ::protobuf::EnumOrUnknown::new(super::StatType::StatType::STAT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.BOJBPOELFCI.value());
        }
        if self.DBAHFEFGLMD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DBAHFEFGLMD);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
        }
        if self.BOJBPOELFCI != ::protobuf::EnumOrUnknown::new(super::StatType::StatType::STAT_TYPE_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.BOJBPOELFCI))?;
        }
        if self.DBAHFEFGLMD != 0 {
            os.write_uint32(8, self.DBAHFEFGLMD)?;
        }
        if self.level != 0 {
            os.write_uint32(5, self.level)?;
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

    fn new() -> UpgradeAreaStatScRsp {
        UpgradeAreaStatScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.BOJBPOELFCI = ::protobuf::EnumOrUnknown::new(super::StatType::StatType::STAT_TYPE_NONE);
        self.DBAHFEFGLMD = 0;
        self.level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpgradeAreaStatScRsp {
        static instance: UpgradeAreaStatScRsp = UpgradeAreaStatScRsp {
            retcode: 0,
            BOJBPOELFCI: ::protobuf::EnumOrUnknown::from_i32(0),
            DBAHFEFGLMD: 0,
            level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpgradeAreaStatScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpgradeAreaStatScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpgradeAreaStatScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpgradeAreaStatScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aUpgradeAreaStatScRsp.proto\x1a\x0eStatType.proto\"\x95\x01\n\x14Up\
    gradeAreaStatScRsp\x12\x18\n\x07retcode\x18\x03\x20\x01(\rR\x07retcode\
    \x12+\n\x0bBOJBPOELFCI\x18\x0b\x20\x01(\x0e2\t.StatTypeR\x0bBOJBPOELFCI\
    \x12\x20\n\x0bDBAHFEFGLMD\x18\x08\x20\x01(\rR\x0bDBAHFEFGLMD\x12\x14\n\
    \x05level\x18\x05\x20\x01(\rR\x05levelb\x06proto3\
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
            deps.push(super::StatType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpgradeAreaStatScRsp::generated_message_descriptor_data());
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
