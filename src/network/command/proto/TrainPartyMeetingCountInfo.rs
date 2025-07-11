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

//! Generated file from `TrainPartyMeetingCountInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TrainPartyMeetingCountInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyMeetingCountInfo {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.BLHPICIOFAI)
    pub BLHPICIOFAI: u32,
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.KDDPPGOCOMB)
    pub KDDPPGOCOMB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.JODNMDOAMKC)
    pub JODNMDOAMKC: u32,
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.PAPKGJOJPII)
    pub PAPKGJOJPII: u32,
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.NCDCGFKOLOE)
    pub NCDCGFKOLOE: u32,
    // @@protoc_insertion_point(field:TrainPartyMeetingCountInfo.HCFOCPKFOBG)
    pub HCFOCPKFOBG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyMeetingCountInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyMeetingCountInfo {
    fn default() -> &'a TrainPartyMeetingCountInfo {
        <TrainPartyMeetingCountInfo as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyMeetingCountInfo {
    pub fn new() -> TrainPartyMeetingCountInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLHPICIOFAI",
            |m: &TrainPartyMeetingCountInfo| { &m.BLHPICIOFAI },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.BLHPICIOFAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KDDPPGOCOMB",
            |m: &TrainPartyMeetingCountInfo| { &m.KDDPPGOCOMB },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.KDDPPGOCOMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JODNMDOAMKC",
            |m: &TrainPartyMeetingCountInfo| { &m.JODNMDOAMKC },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.JODNMDOAMKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PAPKGJOJPII",
            |m: &TrainPartyMeetingCountInfo| { &m.PAPKGJOJPII },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.PAPKGJOJPII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCDCGFKOLOE",
            |m: &TrainPartyMeetingCountInfo| { &m.NCDCGFKOLOE },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.NCDCGFKOLOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCFOCPKFOBG",
            |m: &TrainPartyMeetingCountInfo| { &m.HCFOCPKFOBG },
            |m: &mut TrainPartyMeetingCountInfo| { &mut m.HCFOCPKFOBG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyMeetingCountInfo>(
            "TrainPartyMeetingCountInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyMeetingCountInfo {
    const NAME: &'static str = "TrainPartyMeetingCountInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.BLHPICIOFAI = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.KDDPPGOCOMB)?;
                },
                8 => {
                    self.KDDPPGOCOMB.push(is.read_uint32()?);
                },
                32 => {
                    self.JODNMDOAMKC = is.read_uint32()?;
                },
                112 => {
                    self.PAPKGJOJPII = is.read_uint32()?;
                },
                48 => {
                    self.NCDCGFKOLOE = is.read_uint32()?;
                },
                16 => {
                    self.HCFOCPKFOBG = is.read_uint32()?;
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
        if self.BLHPICIOFAI != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.BLHPICIOFAI);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.KDDPPGOCOMB);
        if self.JODNMDOAMKC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.JODNMDOAMKC);
        }
        if self.PAPKGJOJPII != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PAPKGJOJPII);
        }
        if self.NCDCGFKOLOE != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NCDCGFKOLOE);
        }
        if self.HCFOCPKFOBG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HCFOCPKFOBG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BLHPICIOFAI != 0 {
            os.write_uint32(10, self.BLHPICIOFAI)?;
        }
        os.write_repeated_packed_uint32(1, &self.KDDPPGOCOMB)?;
        if self.JODNMDOAMKC != 0 {
            os.write_uint32(4, self.JODNMDOAMKC)?;
        }
        if self.PAPKGJOJPII != 0 {
            os.write_uint32(14, self.PAPKGJOJPII)?;
        }
        if self.NCDCGFKOLOE != 0 {
            os.write_uint32(6, self.NCDCGFKOLOE)?;
        }
        if self.HCFOCPKFOBG != 0 {
            os.write_uint32(2, self.HCFOCPKFOBG)?;
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

    fn new() -> TrainPartyMeetingCountInfo {
        TrainPartyMeetingCountInfo::new()
    }

    fn clear(&mut self) {
        self.BLHPICIOFAI = 0;
        self.KDDPPGOCOMB.clear();
        self.JODNMDOAMKC = 0;
        self.PAPKGJOJPII = 0;
        self.NCDCGFKOLOE = 0;
        self.HCFOCPKFOBG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyMeetingCountInfo {
        static instance: TrainPartyMeetingCountInfo = TrainPartyMeetingCountInfo {
            BLHPICIOFAI: 0,
            KDDPPGOCOMB: ::std::vec::Vec::new(),
            JODNMDOAMKC: 0,
            PAPKGJOJPII: 0,
            NCDCGFKOLOE: 0,
            HCFOCPKFOBG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyMeetingCountInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyMeetingCountInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyMeetingCountInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyMeetingCountInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20TrainPartyMeetingCountInfo.proto\"\xe8\x01\n\x1aTrainPartyMeetingC\
    ountInfo\x12\x20\n\x0bBLHPICIOFAI\x18\n\x20\x01(\rR\x0bBLHPICIOFAI\x12\
    \x20\n\x0bKDDPPGOCOMB\x18\x01\x20\x03(\rR\x0bKDDPPGOCOMB\x12\x20\n\x0bJO\
    DNMDOAMKC\x18\x04\x20\x01(\rR\x0bJODNMDOAMKC\x12\x20\n\x0bPAPKGJOJPII\
    \x18\x0e\x20\x01(\rR\x0bPAPKGJOJPII\x12\x20\n\x0bNCDCGFKOLOE\x18\x06\x20\
    \x01(\rR\x0bNCDCGFKOLOE\x12\x20\n\x0bHCFOCPKFOBG\x18\x02\x20\x01(\rR\x0b\
    HCFOCPKFOBGb\x06proto3\
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
            messages.push(TrainPartyMeetingCountInfo::generated_message_descriptor_data());
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
