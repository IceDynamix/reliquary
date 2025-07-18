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

//! Generated file from `HGLKMJFEHMB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HGLKMJFEHMB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HGLKMJFEHMB {
    // message fields
    // @@protoc_insertion_point(field:HGLKMJFEHMB.IJABKDEPGMA)
    pub IJABKDEPGMA: bool,
    // @@protoc_insertion_point(field:HGLKMJFEHMB.visitor_id)
    pub visitor_id: u32,
    // @@protoc_insertion_point(field:HGLKMJFEHMB.EDHHGCPDKIK)
    pub EDHHGCPDKIK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HGLKMJFEHMB.OPAOKGJBOOE)
    pub OPAOKGJBOOE: u32,
    // @@protoc_insertion_point(field:HGLKMJFEHMB.status)
    pub status: ::protobuf::EnumOrUnknown<super::TrainVisitorStatus::TrainVisitorStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:HGLKMJFEHMB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HGLKMJFEHMB {
    fn default() -> &'a HGLKMJFEHMB {
        <HGLKMJFEHMB as ::protobuf::Message>::default_instance()
    }
}

impl HGLKMJFEHMB {
    pub fn new() -> HGLKMJFEHMB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJABKDEPGMA",
            |m: &HGLKMJFEHMB| { &m.IJABKDEPGMA },
            |m: &mut HGLKMJFEHMB| { &mut m.IJABKDEPGMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "visitor_id",
            |m: &HGLKMJFEHMB| { &m.visitor_id },
            |m: &mut HGLKMJFEHMB| { &mut m.visitor_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EDHHGCPDKIK",
            |m: &HGLKMJFEHMB| { &m.EDHHGCPDKIK },
            |m: &mut HGLKMJFEHMB| { &mut m.EDHHGCPDKIK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPAOKGJBOOE",
            |m: &HGLKMJFEHMB| { &m.OPAOKGJBOOE },
            |m: &mut HGLKMJFEHMB| { &mut m.OPAOKGJBOOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &HGLKMJFEHMB| { &m.status },
            |m: &mut HGLKMJFEHMB| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HGLKMJFEHMB>(
            "HGLKMJFEHMB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HGLKMJFEHMB {
    const NAME: &'static str = "HGLKMJFEHMB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.IJABKDEPGMA = is.read_bool()?;
                },
                88 => {
                    self.visitor_id = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.EDHHGCPDKIK)?;
                },
                120 => {
                    self.EDHHGCPDKIK.push(is.read_uint32()?);
                },
                104 => {
                    self.OPAOKGJBOOE = is.read_uint32()?;
                },
                40 => {
                    self.status = is.read_enum_or_unknown()?;
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
        if self.IJABKDEPGMA != false {
            my_size += 1 + 1;
        }
        if self.visitor_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.visitor_id);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.EDHHGCPDKIK);
        if self.OPAOKGJBOOE != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OPAOKGJBOOE);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IJABKDEPGMA != false {
            os.write_bool(7, self.IJABKDEPGMA)?;
        }
        if self.visitor_id != 0 {
            os.write_uint32(11, self.visitor_id)?;
        }
        os.write_repeated_packed_uint32(15, &self.EDHHGCPDKIK)?;
        if self.OPAOKGJBOOE != 0 {
            os.write_uint32(13, self.OPAOKGJBOOE)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.status))?;
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

    fn new() -> HGLKMJFEHMB {
        HGLKMJFEHMB::new()
    }

    fn clear(&mut self) {
        self.IJABKDEPGMA = false;
        self.visitor_id = 0;
        self.EDHHGCPDKIK.clear();
        self.OPAOKGJBOOE = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::TrainVisitorStatus::TrainVisitorStatus::TRAIN_VISITOR_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HGLKMJFEHMB {
        static instance: HGLKMJFEHMB = HGLKMJFEHMB {
            IJABKDEPGMA: false,
            visitor_id: 0,
            EDHHGCPDKIK: ::std::vec::Vec::new(),
            OPAOKGJBOOE: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HGLKMJFEHMB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HGLKMJFEHMB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HGLKMJFEHMB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HGLKMJFEHMB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HGLKMJFEHMB.proto\x1a\x18TrainVisitorStatus.proto\"\xbf\x01\n\x0bH\
    GLKMJFEHMB\x12\x20\n\x0bIJABKDEPGMA\x18\x07\x20\x01(\x08R\x0bIJABKDEPGMA\
    \x12\x1d\n\nvisitor_id\x18\x0b\x20\x01(\rR\tvisitorId\x12\x20\n\x0bEDHHG\
    CPDKIK\x18\x0f\x20\x03(\rR\x0bEDHHGCPDKIK\x12\x20\n\x0bOPAOKGJBOOE\x18\r\
    \x20\x01(\rR\x0bOPAOKGJBOOE\x12+\n\x06status\x18\x05\x20\x01(\x0e2\x13.T\
    rainVisitorStatusR\x06statusb\x06proto3\
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
            deps.push(super::TrainVisitorStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HGLKMJFEHMB::generated_message_descriptor_data());
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
