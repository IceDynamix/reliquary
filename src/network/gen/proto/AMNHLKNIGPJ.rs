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

//! Generated file from `AMNHLKNIGPJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AMNHLKNIGPJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AMNHLKNIGPJ {
    // message fields
    // @@protoc_insertion_point(field:AMNHLKNIGPJ.KBFNONOKDLP)
    pub KBFNONOKDLP: ::protobuf::MessageField<super::DOKJKIMMFHG::DOKJKIMMFHG>,
    // @@protoc_insertion_point(field:AMNHLKNIGPJ.EGPAJJPKFHM)
    pub EGPAJJPKFHM: ::protobuf::MessageField<super::NCENDKFCHKJ::NCENDKFCHKJ>,
    // @@protoc_insertion_point(field:AMNHLKNIGPJ.DAHDHMFLEFF)
    pub DAHDHMFLEFF: u32,
    // @@protoc_insertion_point(field:AMNHLKNIGPJ.MPECOOBEOKD)
    pub MPECOOBEOKD: ::protobuf::MessageField<super::JGBJBDGOHGP::JGBJBDGOHGP>,
    // @@protoc_insertion_point(field:AMNHLKNIGPJ.MMOGCPNHJHM)
    pub MMOGCPNHJHM: ::protobuf::MessageField<super::MNLNOCKOHGL::MNLNOCKOHGL>,
    // special fields
    // @@protoc_insertion_point(special_field:AMNHLKNIGPJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AMNHLKNIGPJ {
    fn default() -> &'a AMNHLKNIGPJ {
        <AMNHLKNIGPJ as ::protobuf::Message>::default_instance()
    }
}

impl AMNHLKNIGPJ {
    pub fn new() -> AMNHLKNIGPJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DOKJKIMMFHG::DOKJKIMMFHG>(
            "KBFNONOKDLP",
            |m: &AMNHLKNIGPJ| { &m.KBFNONOKDLP },
            |m: &mut AMNHLKNIGPJ| { &mut m.KBFNONOKDLP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NCENDKFCHKJ::NCENDKFCHKJ>(
            "EGPAJJPKFHM",
            |m: &AMNHLKNIGPJ| { &m.EGPAJJPKFHM },
            |m: &mut AMNHLKNIGPJ| { &mut m.EGPAJJPKFHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAHDHMFLEFF",
            |m: &AMNHLKNIGPJ| { &m.DAHDHMFLEFF },
            |m: &mut AMNHLKNIGPJ| { &mut m.DAHDHMFLEFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JGBJBDGOHGP::JGBJBDGOHGP>(
            "MPECOOBEOKD",
            |m: &AMNHLKNIGPJ| { &m.MPECOOBEOKD },
            |m: &mut AMNHLKNIGPJ| { &mut m.MPECOOBEOKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MNLNOCKOHGL::MNLNOCKOHGL>(
            "MMOGCPNHJHM",
            |m: &AMNHLKNIGPJ| { &m.MMOGCPNHJHM },
            |m: &mut AMNHLKNIGPJ| { &mut m.MMOGCPNHJHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AMNHLKNIGPJ>(
            "AMNHLKNIGPJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AMNHLKNIGPJ {
    const NAME: &'static str = "AMNHLKNIGPJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KBFNONOKDLP)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EGPAJJPKFHM)?;
                },
                96 => {
                    self.DAHDHMFLEFF = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MPECOOBEOKD)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MMOGCPNHJHM)?;
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
        if let Some(v) = self.KBFNONOKDLP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EGPAJJPKFHM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DAHDHMFLEFF != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DAHDHMFLEFF);
        }
        if let Some(v) = self.MPECOOBEOKD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MMOGCPNHJHM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.KBFNONOKDLP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.EGPAJJPKFHM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.DAHDHMFLEFF != 0 {
            os.write_uint32(12, self.DAHDHMFLEFF)?;
        }
        if let Some(v) = self.MPECOOBEOKD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.MMOGCPNHJHM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> AMNHLKNIGPJ {
        AMNHLKNIGPJ::new()
    }

    fn clear(&mut self) {
        self.KBFNONOKDLP.clear();
        self.EGPAJJPKFHM.clear();
        self.DAHDHMFLEFF = 0;
        self.MPECOOBEOKD.clear();
        self.MMOGCPNHJHM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AMNHLKNIGPJ {
        static instance: AMNHLKNIGPJ = AMNHLKNIGPJ {
            KBFNONOKDLP: ::protobuf::MessageField::none(),
            EGPAJJPKFHM: ::protobuf::MessageField::none(),
            DAHDHMFLEFF: 0,
            MPECOOBEOKD: ::protobuf::MessageField::none(),
            MMOGCPNHJHM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AMNHLKNIGPJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AMNHLKNIGPJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AMNHLKNIGPJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AMNHLKNIGPJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AMNHLKNIGPJ.proto\x1a\x11DOKJKIMMFHG.proto\x1a\x11JGBJBDGOHGP.prot\
    o\x1a\x11MNLNOCKOHGL.proto\x1a\x11NCENDKFCHKJ.proto\"\xef\x01\n\x0bAMNHL\
    KNIGPJ\x12.\n\x0bKBFNONOKDLP\x18\x0e\x20\x01(\x0b2\x0c.DOKJKIMMFHGR\x0bK\
    BFNONOKDLP\x12.\n\x0bEGPAJJPKFHM\x18\r\x20\x01(\x0b2\x0c.NCENDKFCHKJR\
    \x0bEGPAJJPKFHM\x12\x20\n\x0bDAHDHMFLEFF\x18\x0c\x20\x01(\rR\x0bDAHDHMFL\
    EFF\x12.\n\x0bMPECOOBEOKD\x18\x06\x20\x01(\x0b2\x0c.JGBJBDGOHGPR\x0bMPEC\
    OOBEOKD\x12.\n\x0bMMOGCPNHJHM\x18\x04\x20\x01(\x0b2\x0c.MNLNOCKOHGLR\x0b\
    MMOGCPNHJHMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::DOKJKIMMFHG::file_descriptor().clone());
            deps.push(super::JGBJBDGOHGP::file_descriptor().clone());
            deps.push(super::MNLNOCKOHGL::file_descriptor().clone());
            deps.push(super::NCENDKFCHKJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AMNHLKNIGPJ::generated_message_descriptor_data());
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