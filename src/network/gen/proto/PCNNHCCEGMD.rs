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

//! Generated file from `PCNNHCCEGMD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PCNNHCCEGMD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PCNNHCCEGMD {
    // message fields
    // @@protoc_insertion_point(field:PCNNHCCEGMD.IOPEEMNLIDM)
    pub IOPEEMNLIDM: ::protobuf::EnumOrUnknown<super::MessageGroupStatus::MessageGroupStatus>,
    // @@protoc_insertion_point(field:PCNNHCCEGMD.HHMGAFAEEOA)
    pub HHMGAFAEEOA: u32,
    // @@protoc_insertion_point(field:PCNNHCCEGMD.FFKNMAONGIB)
    pub FFKNMAONGIB: u32,
    // @@protoc_insertion_point(field:PCNNHCCEGMD.IEMHJNKLGKP)
    pub IEMHJNKLGKP: ::std::vec::Vec<super::NFPIMBHCGKO::NFPIMBHCGKO>,
    // @@protoc_insertion_point(field:PCNNHCCEGMD.LCIBJMOPMPL)
    pub LCIBJMOPMPL: i64,
    // special fields
    // @@protoc_insertion_point(special_field:PCNNHCCEGMD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PCNNHCCEGMD {
    fn default() -> &'a PCNNHCCEGMD {
        <PCNNHCCEGMD as ::protobuf::Message>::default_instance()
    }
}

impl PCNNHCCEGMD {
    pub fn new() -> PCNNHCCEGMD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPEEMNLIDM",
            |m: &PCNNHCCEGMD| { &m.IOPEEMNLIDM },
            |m: &mut PCNNHCCEGMD| { &mut m.IOPEEMNLIDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHMGAFAEEOA",
            |m: &PCNNHCCEGMD| { &m.HHMGAFAEEOA },
            |m: &mut PCNNHCCEGMD| { &mut m.HHMGAFAEEOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFKNMAONGIB",
            |m: &PCNNHCCEGMD| { &m.FFKNMAONGIB },
            |m: &mut PCNNHCCEGMD| { &mut m.FFKNMAONGIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IEMHJNKLGKP",
            |m: &PCNNHCCEGMD| { &m.IEMHJNKLGKP },
            |m: &mut PCNNHCCEGMD| { &mut m.IEMHJNKLGKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCIBJMOPMPL",
            |m: &PCNNHCCEGMD| { &m.LCIBJMOPMPL },
            |m: &mut PCNNHCCEGMD| { &mut m.LCIBJMOPMPL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PCNNHCCEGMD>(
            "PCNNHCCEGMD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PCNNHCCEGMD {
    const NAME: &'static str = "PCNNHCCEGMD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.IOPEEMNLIDM = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.HHMGAFAEEOA = is.read_uint32()?;
                },
                40 => {
                    self.FFKNMAONGIB = is.read_uint32()?;
                },
                122 => {
                    self.IEMHJNKLGKP.push(is.read_message()?);
                },
                48 => {
                    self.LCIBJMOPMPL = is.read_int64()?;
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
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::MessageGroupStatus::MessageGroupStatus::MESSAGE_GROUP_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.IOPEEMNLIDM.value());
        }
        if self.HHMGAFAEEOA != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.HHMGAFAEEOA);
        }
        if self.FFKNMAONGIB != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.FFKNMAONGIB);
        }
        for value in &self.IEMHJNKLGKP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LCIBJMOPMPL != 0 {
            my_size += ::protobuf::rt::int64_size(6, self.LCIBJMOPMPL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::MessageGroupStatus::MessageGroupStatus::MESSAGE_GROUP_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.IOPEEMNLIDM))?;
        }
        if self.HHMGAFAEEOA != 0 {
            os.write_uint32(4, self.HHMGAFAEEOA)?;
        }
        if self.FFKNMAONGIB != 0 {
            os.write_uint32(5, self.FFKNMAONGIB)?;
        }
        for v in &self.IEMHJNKLGKP {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.LCIBJMOPMPL != 0 {
            os.write_int64(6, self.LCIBJMOPMPL)?;
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

    fn new() -> PCNNHCCEGMD {
        PCNNHCCEGMD::new()
    }

    fn clear(&mut self) {
        self.IOPEEMNLIDM = ::protobuf::EnumOrUnknown::new(super::MessageGroupStatus::MessageGroupStatus::MESSAGE_GROUP_NONE);
        self.HHMGAFAEEOA = 0;
        self.FFKNMAONGIB = 0;
        self.IEMHJNKLGKP.clear();
        self.LCIBJMOPMPL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PCNNHCCEGMD {
        static instance: PCNNHCCEGMD = PCNNHCCEGMD {
            IOPEEMNLIDM: ::protobuf::EnumOrUnknown::from_i32(0),
            HHMGAFAEEOA: 0,
            FFKNMAONGIB: 0,
            IEMHJNKLGKP: ::std::vec::Vec::new(),
            LCIBJMOPMPL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PCNNHCCEGMD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PCNNHCCEGMD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PCNNHCCEGMD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PCNNHCCEGMD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PCNNHCCEGMD.proto\x1a\x18MessageGroupStatus.proto\x1a\x11NFPIMBHCG\
    KO.proto\"\xda\x01\n\x0bPCNNHCCEGMD\x125\n\x0bIOPEEMNLIDM\x18\x03\x20\
    \x01(\x0e2\x13.MessageGroupStatusR\x0bIOPEEMNLIDM\x12\x20\n\x0bHHMGAFAEE\
    OA\x18\x04\x20\x01(\rR\x0bHHMGAFAEEOA\x12\x20\n\x0bFFKNMAONGIB\x18\x05\
    \x20\x01(\rR\x0bFFKNMAONGIB\x12.\n\x0bIEMHJNKLGKP\x18\x0f\x20\x03(\x0b2\
    \x0c.NFPIMBHCGKOR\x0bIEMHJNKLGKP\x12\x20\n\x0bLCIBJMOPMPL\x18\x06\x20\
    \x01(\x03R\x0bLCIBJMOPMPLb\x06proto3\
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
            deps.push(super::MessageGroupStatus::file_descriptor().clone());
            deps.push(super::NFPIMBHCGKO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PCNNHCCEGMD::generated_message_descriptor_data());
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