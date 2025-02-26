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

//! Generated file from `GetAlleyInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetAlleyInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAlleyInfoCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetAlleyInfoCsReq.level)
    pub level: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoCsReq.NEBOAFHPJFF)
    pub NEBOAFHPJFF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetAlleyInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAlleyInfoCsReq {
    fn default() -> &'a GetAlleyInfoCsReq {
        <GetAlleyInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetAlleyInfoCsReq {
    pub fn new() -> GetAlleyInfoCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &GetAlleyInfoCsReq| { &m.level },
            |m: &mut GetAlleyInfoCsReq| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEBOAFHPJFF",
            |m: &GetAlleyInfoCsReq| { &m.NEBOAFHPJFF },
            |m: &mut GetAlleyInfoCsReq| { &mut m.NEBOAFHPJFF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAlleyInfoCsReq>(
            "GetAlleyInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAlleyInfoCsReq {
    const NAME: &'static str = "GetAlleyInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.level = is.read_uint32()?;
                },
                56 => {
                    self.NEBOAFHPJFF = is.read_uint32()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.level);
        }
        if self.NEBOAFHPJFF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NEBOAFHPJFF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(15, self.level)?;
        }
        if self.NEBOAFHPJFF != 0 {
            os.write_uint32(7, self.NEBOAFHPJFF)?;
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

    fn new() -> GetAlleyInfoCsReq {
        GetAlleyInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.NEBOAFHPJFF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAlleyInfoCsReq {
        static instance: GetAlleyInfoCsReq = GetAlleyInfoCsReq {
            level: 0,
            NEBOAFHPJFF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAlleyInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAlleyInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAlleyInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAlleyInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetAlleyInfoCsReq.proto\"K\n\x11GetAlleyInfoCsReq\x12\x14\n\x05lev\
    el\x18\x0f\x20\x01(\rR\x05level\x12\x20\n\x0bNEBOAFHPJFF\x18\x07\x20\x01\
    (\rR\x0bNEBOAFHPJFFb\x06proto3\
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
            messages.push(GetAlleyInfoCsReq::generated_message_descriptor_data());
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
