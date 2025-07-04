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

//! Generated file from `ParkourEndLevelScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ParkourEndLevelScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ParkourEndLevelScRsp {
    // message fields
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.ACJCPHIFMLN)
    pub ACJCPHIFMLN: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.KKEHMBPJOOC)
    pub KKEHMBPJOOC: bool,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.FDGMOEOAJKP)
    pub FDGMOEOAJKP: bool,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.time)
    pub time: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.end_reason)
    pub end_reason: ::protobuf::EnumOrUnknown<super::POAHABDKPKJ::POAHABDKPKJ>,
    // @@protoc_insertion_point(field:ParkourEndLevelScRsp.BLMDKJKOIOH)
    pub BLMDKJKOIOH: ::protobuf::MessageField<super::GGDEMGBOFGO::GGDEMGBOFGO>,
    // special fields
    // @@protoc_insertion_point(special_field:ParkourEndLevelScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ParkourEndLevelScRsp {
    fn default() -> &'a ParkourEndLevelScRsp {
        <ParkourEndLevelScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ParkourEndLevelScRsp {
    pub fn new() -> ParkourEndLevelScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACJCPHIFMLN",
            |m: &ParkourEndLevelScRsp| { &m.ACJCPHIFMLN },
            |m: &mut ParkourEndLevelScRsp| { &mut m.ACJCPHIFMLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKEHMBPJOOC",
            |m: &ParkourEndLevelScRsp| { &m.KKEHMBPJOOC },
            |m: &mut ParkourEndLevelScRsp| { &mut m.KKEHMBPJOOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDGMOEOAJKP",
            |m: &ParkourEndLevelScRsp| { &m.FDGMOEOAJKP },
            |m: &mut ParkourEndLevelScRsp| { &mut m.FDGMOEOAJKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &ParkourEndLevelScRsp| { &m.time },
            |m: &mut ParkourEndLevelScRsp| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ParkourEndLevelScRsp| { &m.retcode },
            |m: &mut ParkourEndLevelScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_reason",
            |m: &ParkourEndLevelScRsp| { &m.end_reason },
            |m: &mut ParkourEndLevelScRsp| { &mut m.end_reason },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GGDEMGBOFGO::GGDEMGBOFGO>(
            "BLMDKJKOIOH",
            |m: &ParkourEndLevelScRsp| { &m.BLMDKJKOIOH },
            |m: &mut ParkourEndLevelScRsp| { &mut m.BLMDKJKOIOH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ParkourEndLevelScRsp>(
            "ParkourEndLevelScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ParkourEndLevelScRsp {
    const NAME: &'static str = "ParkourEndLevelScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.ACJCPHIFMLN = is.read_uint32()?;
                },
                96 => {
                    self.KKEHMBPJOOC = is.read_bool()?;
                },
                32 => {
                    self.FDGMOEOAJKP = is.read_bool()?;
                },
                80 => {
                    self.time = is.read_uint32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                120 => {
                    self.end_reason = is.read_enum_or_unknown()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BLMDKJKOIOH)?;
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
        if self.ACJCPHIFMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ACJCPHIFMLN);
        }
        if self.KKEHMBPJOOC != false {
            my_size += 1 + 1;
        }
        if self.FDGMOEOAJKP != false {
            my_size += 1 + 1;
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.time);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.end_reason.value());
        }
        if let Some(v) = self.BLMDKJKOIOH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ACJCPHIFMLN != 0 {
            os.write_uint32(6, self.ACJCPHIFMLN)?;
        }
        if self.KKEHMBPJOOC != false {
            os.write_bool(12, self.KKEHMBPJOOC)?;
        }
        if self.FDGMOEOAJKP != false {
            os.write_bool(4, self.FDGMOEOAJKP)?;
        }
        if self.time != 0 {
            os.write_uint32(10, self.time)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if self.end_reason != ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.end_reason))?;
        }
        if let Some(v) = self.BLMDKJKOIOH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> ParkourEndLevelScRsp {
        ParkourEndLevelScRsp::new()
    }

    fn clear(&mut self) {
        self.ACJCPHIFMLN = 0;
        self.KKEHMBPJOOC = false;
        self.FDGMOEOAJKP = false;
        self.time = 0;
        self.retcode = 0;
        self.end_reason = ::protobuf::EnumOrUnknown::new(super::POAHABDKPKJ::POAHABDKPKJ::PARKOUR_END_LEVEL_REASON_NONE);
        self.BLMDKJKOIOH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ParkourEndLevelScRsp {
        static instance: ParkourEndLevelScRsp = ParkourEndLevelScRsp {
            ACJCPHIFMLN: 0,
            KKEHMBPJOOC: false,
            FDGMOEOAJKP: false,
            time: 0,
            retcode: 0,
            end_reason: ::protobuf::EnumOrUnknown::from_i32(0),
            BLMDKJKOIOH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ParkourEndLevelScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ParkourEndLevelScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ParkourEndLevelScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ParkourEndLevelScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aParkourEndLevelScRsp.proto\x1a\x11GGDEMGBOFGO.proto\x1a\x11POAHABD\
    KPKJ.proto\"\x87\x02\n\x14ParkourEndLevelScRsp\x12\x20\n\x0bACJCPHIFMLN\
    \x18\x06\x20\x01(\rR\x0bACJCPHIFMLN\x12\x20\n\x0bKKEHMBPJOOC\x18\x0c\x20\
    \x01(\x08R\x0bKKEHMBPJOOC\x12\x20\n\x0bFDGMOEOAJKP\x18\x04\x20\x01(\x08R\
    \x0bFDGMOEOAJKP\x12\x12\n\x04time\x18\n\x20\x01(\rR\x04time\x12\x18\n\
    \x07retcode\x18\x08\x20\x01(\rR\x07retcode\x12+\n\nend_reason\x18\x0f\
    \x20\x01(\x0e2\x0c.POAHABDKPKJR\tendReason\x12.\n\x0bBLMDKJKOIOH\x18\x0e\
    \x20\x01(\x0b2\x0c.GGDEMGBOFGOR\x0bBLMDKJKOIOHb\x06proto3\
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
            deps.push(super::GGDEMGBOFGO::file_descriptor().clone());
            deps.push(super::POAHABDKPKJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ParkourEndLevelScRsp::generated_message_descriptor_data());
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
