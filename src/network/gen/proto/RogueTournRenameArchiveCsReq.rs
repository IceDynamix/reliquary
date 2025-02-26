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

//! Generated file from `RogueTournRenameArchiveCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueTournRenameArchiveCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournRenameArchiveCsReq {
    // message fields
    // @@protoc_insertion_point(field:RogueTournRenameArchiveCsReq.KHNCEDGFPGL)
    pub KHNCEDGFPGL: u32,
    // @@protoc_insertion_point(field:RogueTournRenameArchiveCsReq.MJJBNDAKOIF)
    pub MJJBNDAKOIF: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournRenameArchiveCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournRenameArchiveCsReq {
    fn default() -> &'a RogueTournRenameArchiveCsReq {
        <RogueTournRenameArchiveCsReq as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournRenameArchiveCsReq {
    pub fn new() -> RogueTournRenameArchiveCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHNCEDGFPGL",
            |m: &RogueTournRenameArchiveCsReq| { &m.KHNCEDGFPGL },
            |m: &mut RogueTournRenameArchiveCsReq| { &mut m.KHNCEDGFPGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MJJBNDAKOIF",
            |m: &RogueTournRenameArchiveCsReq| { &m.MJJBNDAKOIF },
            |m: &mut RogueTournRenameArchiveCsReq| { &mut m.MJJBNDAKOIF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournRenameArchiveCsReq>(
            "RogueTournRenameArchiveCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournRenameArchiveCsReq {
    const NAME: &'static str = "RogueTournRenameArchiveCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.KHNCEDGFPGL = is.read_uint32()?;
                },
                106 => {
                    self.MJJBNDAKOIF = is.read_string()?;
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
        if self.KHNCEDGFPGL != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.KHNCEDGFPGL);
        }
        if !self.MJJBNDAKOIF.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.MJJBNDAKOIF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KHNCEDGFPGL != 0 {
            os.write_uint32(14, self.KHNCEDGFPGL)?;
        }
        if !self.MJJBNDAKOIF.is_empty() {
            os.write_string(13, &self.MJJBNDAKOIF)?;
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

    fn new() -> RogueTournRenameArchiveCsReq {
        RogueTournRenameArchiveCsReq::new()
    }

    fn clear(&mut self) {
        self.KHNCEDGFPGL = 0;
        self.MJJBNDAKOIF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournRenameArchiveCsReq {
        static instance: RogueTournRenameArchiveCsReq = RogueTournRenameArchiveCsReq {
            KHNCEDGFPGL: 0,
            MJJBNDAKOIF: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournRenameArchiveCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournRenameArchiveCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournRenameArchiveCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournRenameArchiveCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"RogueTournRenameArchiveCsReq.proto\"b\n\x1cRogueTournRenameArchiveCs\
    Req\x12\x20\n\x0bKHNCEDGFPGL\x18\x0e\x20\x01(\rR\x0bKHNCEDGFPGL\x12\x20\
    \n\x0bMJJBNDAKOIF\x18\r\x20\x01(\tR\x0bMJJBNDAKOIFb\x06proto3\
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
            messages.push(RogueTournRenameArchiveCsReq::generated_message_descriptor_data());
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
