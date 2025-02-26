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

//! Generated file from `SwordTrainingTurnActionCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SwordTrainingTurnActionCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SwordTrainingTurnActionCsReq {
    // message fields
    // @@protoc_insertion_point(field:SwordTrainingTurnActionCsReq.BHNFGPEHOMO)
    pub BHNFGPEHOMO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SwordTrainingTurnActionCsReq.PHAJEHIBKFI)
    pub PHAJEHIBKFI: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:SwordTrainingTurnActionCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SwordTrainingTurnActionCsReq {
    fn default() -> &'a SwordTrainingTurnActionCsReq {
        <SwordTrainingTurnActionCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SwordTrainingTurnActionCsReq {
    pub fn new() -> SwordTrainingTurnActionCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BHNFGPEHOMO",
            |m: &SwordTrainingTurnActionCsReq| { &m.BHNFGPEHOMO },
            |m: &mut SwordTrainingTurnActionCsReq| { &mut m.BHNFGPEHOMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PHAJEHIBKFI",
            |m: &SwordTrainingTurnActionCsReq| { &m.PHAJEHIBKFI },
            |m: &mut SwordTrainingTurnActionCsReq| { &mut m.PHAJEHIBKFI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SwordTrainingTurnActionCsReq>(
            "SwordTrainingTurnActionCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SwordTrainingTurnActionCsReq {
    const NAME: &'static str = "SwordTrainingTurnActionCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.BHNFGPEHOMO)?;
                },
                88 => {
                    self.BHNFGPEHOMO.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.PHAJEHIBKFI)?;
                },
                80 => {
                    self.PHAJEHIBKFI.push(is.read_uint32()?);
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.BHNFGPEHOMO);
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.PHAJEHIBKFI);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(11, &self.BHNFGPEHOMO)?;
        os.write_repeated_packed_uint32(10, &self.PHAJEHIBKFI)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SwordTrainingTurnActionCsReq {
        SwordTrainingTurnActionCsReq::new()
    }

    fn clear(&mut self) {
        self.BHNFGPEHOMO.clear();
        self.PHAJEHIBKFI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SwordTrainingTurnActionCsReq {
        static instance: SwordTrainingTurnActionCsReq = SwordTrainingTurnActionCsReq {
            BHNFGPEHOMO: ::std::vec::Vec::new(),
            PHAJEHIBKFI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SwordTrainingTurnActionCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SwordTrainingTurnActionCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SwordTrainingTurnActionCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SwordTrainingTurnActionCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"SwordTrainingTurnActionCsReq.proto\"b\n\x1cSwordTrainingTurnActionCs\
    Req\x12\x20\n\x0bBHNFGPEHOMO\x18\x0b\x20\x03(\rR\x0bBHNFGPEHOMO\x12\x20\
    \n\x0bPHAJEHIBKFI\x18\n\x20\x03(\rR\x0bPHAJEHIBKFIb\x06proto3\
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
            messages.push(SwordTrainingTurnActionCsReq::generated_message_descriptor_data());
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
