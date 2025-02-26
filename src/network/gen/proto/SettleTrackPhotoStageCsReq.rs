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

//! Generated file from `SettleTrackPhotoStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SettleTrackPhotoStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SettleTrackPhotoStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:SettleTrackPhotoStageCsReq.CFONLBPOABP)
    pub CFONLBPOABP: u32,
    // @@protoc_insertion_point(field:SettleTrackPhotoStageCsReq.BCLNMIDFFOH)
    pub BCLNMIDFFOH: u32,
    // @@protoc_insertion_point(field:SettleTrackPhotoStageCsReq.LPIHANIOJFI)
    pub LPIHANIOJFI: ::std::vec::Vec<super::GEOAEOFJOGC::GEOAEOFJOGC>,
    // special fields
    // @@protoc_insertion_point(special_field:SettleTrackPhotoStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SettleTrackPhotoStageCsReq {
    fn default() -> &'a SettleTrackPhotoStageCsReq {
        <SettleTrackPhotoStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SettleTrackPhotoStageCsReq {
    pub fn new() -> SettleTrackPhotoStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFONLBPOABP",
            |m: &SettleTrackPhotoStageCsReq| { &m.CFONLBPOABP },
            |m: &mut SettleTrackPhotoStageCsReq| { &mut m.CFONLBPOABP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCLNMIDFFOH",
            |m: &SettleTrackPhotoStageCsReq| { &m.BCLNMIDFFOH },
            |m: &mut SettleTrackPhotoStageCsReq| { &mut m.BCLNMIDFFOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LPIHANIOJFI",
            |m: &SettleTrackPhotoStageCsReq| { &m.LPIHANIOJFI },
            |m: &mut SettleTrackPhotoStageCsReq| { &mut m.LPIHANIOJFI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SettleTrackPhotoStageCsReq>(
            "SettleTrackPhotoStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SettleTrackPhotoStageCsReq {
    const NAME: &'static str = "SettleTrackPhotoStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.CFONLBPOABP = is.read_uint32()?;
                },
                104 => {
                    self.BCLNMIDFFOH = is.read_uint32()?;
                },
                82 => {
                    self.LPIHANIOJFI.push(is.read_message()?);
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
        if self.CFONLBPOABP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CFONLBPOABP);
        }
        if self.BCLNMIDFFOH != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.BCLNMIDFFOH);
        }
        for value in &self.LPIHANIOJFI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CFONLBPOABP != 0 {
            os.write_uint32(15, self.CFONLBPOABP)?;
        }
        if self.BCLNMIDFFOH != 0 {
            os.write_uint32(13, self.BCLNMIDFFOH)?;
        }
        for v in &self.LPIHANIOJFI {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SettleTrackPhotoStageCsReq {
        SettleTrackPhotoStageCsReq::new()
    }

    fn clear(&mut self) {
        self.CFONLBPOABP = 0;
        self.BCLNMIDFFOH = 0;
        self.LPIHANIOJFI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SettleTrackPhotoStageCsReq {
        static instance: SettleTrackPhotoStageCsReq = SettleTrackPhotoStageCsReq {
            CFONLBPOABP: 0,
            BCLNMIDFFOH: 0,
            LPIHANIOJFI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SettleTrackPhotoStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SettleTrackPhotoStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SettleTrackPhotoStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SettleTrackPhotoStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20SettleTrackPhotoStageCsReq.proto\x1a\x11GEOAEOFJOGC.proto\"\x90\
    \x01\n\x1aSettleTrackPhotoStageCsReq\x12\x20\n\x0bCFONLBPOABP\x18\x0f\
    \x20\x01(\rR\x0bCFONLBPOABP\x12\x20\n\x0bBCLNMIDFFOH\x18\r\x20\x01(\rR\
    \x0bBCLNMIDFFOH\x12.\n\x0bLPIHANIOJFI\x18\n\x20\x03(\x0b2\x0c.GEOAEOFJOG\
    CR\x0bLPIHANIOJFIb\x06proto3\
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
            deps.push(super::GEOAEOFJOGC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SettleTrackPhotoStageCsReq::generated_message_descriptor_data());
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
