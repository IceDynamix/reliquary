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

//! Generated file from `UpdateRedDotDataCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UpdateRedDotDataCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateRedDotDataCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.HONEMGCFBGI)
    pub HONEMGCFBGI: ::protobuf::EnumOrUnknown<super::OJLJHFNFDKP::OJLJHFNFDKP>,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.OIEKCNPMDFC)
    pub OIEKCNPMDFC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.FJNHDHOHBCL)
    pub FJNHDHOHBCL: u32,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.DKKLLMOHGFD)
    pub DKKLLMOHGFD: u32,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.NOPDKLDEKKF)
    pub NOPDKLDEKKF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateRedDotDataCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateRedDotDataCsReq {
    fn default() -> &'a UpdateRedDotDataCsReq {
        <UpdateRedDotDataCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateRedDotDataCsReq {
    pub fn new() -> UpdateRedDotDataCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HONEMGCFBGI",
            |m: &UpdateRedDotDataCsReq| { &m.HONEMGCFBGI },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.HONEMGCFBGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OIEKCNPMDFC",
            |m: &UpdateRedDotDataCsReq| { &m.OIEKCNPMDFC },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.OIEKCNPMDFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNHDHOHBCL",
            |m: &UpdateRedDotDataCsReq| { &m.FJNHDHOHBCL },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.FJNHDHOHBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKKLLMOHGFD",
            |m: &UpdateRedDotDataCsReq| { &m.DKKLLMOHGFD },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.DKKLLMOHGFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOPDKLDEKKF",
            |m: &UpdateRedDotDataCsReq| { &m.NOPDKLDEKKF },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.NOPDKLDEKKF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateRedDotDataCsReq>(
            "UpdateRedDotDataCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateRedDotDataCsReq {
    const NAME: &'static str = "UpdateRedDotDataCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.HONEMGCFBGI = is.read_enum_or_unknown()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.OIEKCNPMDFC)?;
                },
                88 => {
                    self.OIEKCNPMDFC.push(is.read_uint32()?);
                },
                64 => {
                    self.FJNHDHOHBCL = is.read_uint32()?;
                },
                24 => {
                    self.DKKLLMOHGFD = is.read_uint32()?;
                },
                56 => {
                    self.NOPDKLDEKKF = is.read_uint32()?;
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
        if self.HONEMGCFBGI != ::protobuf::EnumOrUnknown::new(super::OJLJHFNFDKP::OJLJHFNFDKP::UPDATE_REDDOT_NONE) {
            my_size += ::protobuf::rt::int32_size(9, self.HONEMGCFBGI.value());
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.OIEKCNPMDFC);
        if self.FJNHDHOHBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FJNHDHOHBCL);
        }
        if self.DKKLLMOHGFD != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DKKLLMOHGFD);
        }
        if self.NOPDKLDEKKF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NOPDKLDEKKF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HONEMGCFBGI != ::protobuf::EnumOrUnknown::new(super::OJLJHFNFDKP::OJLJHFNFDKP::UPDATE_REDDOT_NONE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.HONEMGCFBGI))?;
        }
        os.write_repeated_packed_uint32(11, &self.OIEKCNPMDFC)?;
        if self.FJNHDHOHBCL != 0 {
            os.write_uint32(8, self.FJNHDHOHBCL)?;
        }
        if self.DKKLLMOHGFD != 0 {
            os.write_uint32(3, self.DKKLLMOHGFD)?;
        }
        if self.NOPDKLDEKKF != 0 {
            os.write_uint32(7, self.NOPDKLDEKKF)?;
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

    fn new() -> UpdateRedDotDataCsReq {
        UpdateRedDotDataCsReq::new()
    }

    fn clear(&mut self) {
        self.HONEMGCFBGI = ::protobuf::EnumOrUnknown::new(super::OJLJHFNFDKP::OJLJHFNFDKP::UPDATE_REDDOT_NONE);
        self.OIEKCNPMDFC.clear();
        self.FJNHDHOHBCL = 0;
        self.DKKLLMOHGFD = 0;
        self.NOPDKLDEKKF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateRedDotDataCsReq {
        static instance: UpdateRedDotDataCsReq = UpdateRedDotDataCsReq {
            HONEMGCFBGI: ::protobuf::EnumOrUnknown::from_i32(0),
            OIEKCNPMDFC: ::std::vec::Vec::new(),
            FJNHDHOHBCL: 0,
            DKKLLMOHGFD: 0,
            NOPDKLDEKKF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateRedDotDataCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateRedDotDataCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateRedDotDataCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateRedDotDataCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bUpdateRedDotDataCsReq.proto\x1a\x11OJLJHFNFDKP.proto\"\xcf\x01\n\
    \x15UpdateRedDotDataCsReq\x12.\n\x0bHONEMGCFBGI\x18\t\x20\x01(\x0e2\x0c.\
    OJLJHFNFDKPR\x0bHONEMGCFBGI\x12\x20\n\x0bOIEKCNPMDFC\x18\x0b\x20\x03(\rR\
    \x0bOIEKCNPMDFC\x12\x20\n\x0bFJNHDHOHBCL\x18\x08\x20\x01(\rR\x0bFJNHDHOH\
    BCL\x12\x20\n\x0bDKKLLMOHGFD\x18\x03\x20\x01(\rR\x0bDKKLLMOHGFD\x12\x20\
    \n\x0bNOPDKLDEKKF\x18\x07\x20\x01(\rR\x0bNOPDKLDEKKFb\x06proto3\
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
            deps.push(super::OJLJHFNFDKP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateRedDotDataCsReq::generated_message_descriptor_data());
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
