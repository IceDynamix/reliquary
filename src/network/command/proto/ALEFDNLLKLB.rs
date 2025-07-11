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

//! Generated file from `ALEFDNLLKLB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ALEFDNLLKLB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ALEFDNLLKLB {
    // message fields
    // @@protoc_insertion_point(field:ALEFDNLLKLB.NNCJOECKCKA)
    pub NNCJOECKCKA: ::protobuf::MessageField<super::BPPMEIGAHGI::BPPMEIGAHGI>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.skill_info)
    pub skill_info: ::protobuf::MessageField<super::GJBNIIINKFB::GJBNIIINKFB>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.AFPDJDKNENI)
    pub AFPDJDKNENI: ::protobuf::MessageField<super::OCBOLHFOIGI::OCBOLHFOIGI>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.LJGCPNOGIFO)
    pub LJGCPNOGIFO: ::protobuf::MessageField<super::DMJLKIFEMMN::DMJLKIFEMMN>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.FDEIGEPCCBP)
    pub FDEIGEPCCBP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.pending_action)
    pub pending_action: ::protobuf::MessageField<super::JKMFMMPMNAM::JKMFMMPMNAM>,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.EENJBPMNDOL)
    pub EENJBPMNDOL: u32,
    // @@protoc_insertion_point(field:ALEFDNLLKLB.GBEABIMOBIC)
    pub GBEABIMOBIC: ::protobuf::MessageField<super::PGGGCFBKDPK::PGGGCFBKDPK>,
    // special fields
    // @@protoc_insertion_point(special_field:ALEFDNLLKLB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ALEFDNLLKLB {
    fn default() -> &'a ALEFDNLLKLB {
        <ALEFDNLLKLB as ::protobuf::Message>::default_instance()
    }
}

impl ALEFDNLLKLB {
    pub fn new() -> ALEFDNLLKLB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BPPMEIGAHGI::BPPMEIGAHGI>(
            "NNCJOECKCKA",
            |m: &ALEFDNLLKLB| { &m.NNCJOECKCKA },
            |m: &mut ALEFDNLLKLB| { &mut m.NNCJOECKCKA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GJBNIIINKFB::GJBNIIINKFB>(
            "skill_info",
            |m: &ALEFDNLLKLB| { &m.skill_info },
            |m: &mut ALEFDNLLKLB| { &mut m.skill_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OCBOLHFOIGI::OCBOLHFOIGI>(
            "AFPDJDKNENI",
            |m: &ALEFDNLLKLB| { &m.AFPDJDKNENI },
            |m: &mut ALEFDNLLKLB| { &mut m.AFPDJDKNENI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DMJLKIFEMMN::DMJLKIFEMMN>(
            "LJGCPNOGIFO",
            |m: &ALEFDNLLKLB| { &m.LJGCPNOGIFO },
            |m: &mut ALEFDNLLKLB| { &mut m.LJGCPNOGIFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FDEIGEPCCBP",
            |m: &ALEFDNLLKLB| { &m.FDEIGEPCCBP },
            |m: &mut ALEFDNLLKLB| { &mut m.FDEIGEPCCBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JKMFMMPMNAM::JKMFMMPMNAM>(
            "pending_action",
            |m: &ALEFDNLLKLB| { &m.pending_action },
            |m: &mut ALEFDNLLKLB| { &mut m.pending_action },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EENJBPMNDOL",
            |m: &ALEFDNLLKLB| { &m.EENJBPMNDOL },
            |m: &mut ALEFDNLLKLB| { &mut m.EENJBPMNDOL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PGGGCFBKDPK::PGGGCFBKDPK>(
            "GBEABIMOBIC",
            |m: &ALEFDNLLKLB| { &m.GBEABIMOBIC },
            |m: &mut ALEFDNLLKLB| { &mut m.GBEABIMOBIC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ALEFDNLLKLB>(
            "ALEFDNLLKLB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ALEFDNLLKLB {
    const NAME: &'static str = "ALEFDNLLKLB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NNCJOECKCKA)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.skill_info)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AFPDJDKNENI)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LJGCPNOGIFO)?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.FDEIGEPCCBP)?;
                },
                104 => {
                    self.FDEIGEPCCBP.push(is.read_uint32()?);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pending_action)?;
                },
                32 => {
                    self.EENJBPMNDOL = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GBEABIMOBIC)?;
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
        if let Some(v) = self.NNCJOECKCKA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.skill_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.AFPDJDKNENI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LJGCPNOGIFO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.FDEIGEPCCBP);
        if let Some(v) = self.pending_action.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EENJBPMNDOL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.EENJBPMNDOL);
        }
        if let Some(v) = self.GBEABIMOBIC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NNCJOECKCKA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.skill_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.AFPDJDKNENI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.LJGCPNOGIFO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        os.write_repeated_packed_uint32(13, &self.FDEIGEPCCBP)?;
        if let Some(v) = self.pending_action.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.EENJBPMNDOL != 0 {
            os.write_uint32(4, self.EENJBPMNDOL)?;
        }
        if let Some(v) = self.GBEABIMOBIC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> ALEFDNLLKLB {
        ALEFDNLLKLB::new()
    }

    fn clear(&mut self) {
        self.NNCJOECKCKA.clear();
        self.skill_info.clear();
        self.AFPDJDKNENI.clear();
        self.LJGCPNOGIFO.clear();
        self.FDEIGEPCCBP.clear();
        self.pending_action.clear();
        self.EENJBPMNDOL = 0;
        self.GBEABIMOBIC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ALEFDNLLKLB {
        static instance: ALEFDNLLKLB = ALEFDNLLKLB {
            NNCJOECKCKA: ::protobuf::MessageField::none(),
            skill_info: ::protobuf::MessageField::none(),
            AFPDJDKNENI: ::protobuf::MessageField::none(),
            LJGCPNOGIFO: ::protobuf::MessageField::none(),
            FDEIGEPCCBP: ::std::vec::Vec::new(),
            pending_action: ::protobuf::MessageField::none(),
            EENJBPMNDOL: 0,
            GBEABIMOBIC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ALEFDNLLKLB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ALEFDNLLKLB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ALEFDNLLKLB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ALEFDNLLKLB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ALEFDNLLKLB.proto\x1a\x11BPPMEIGAHGI.proto\x1a\x11DMJLKIFEMMN.prot\
    o\x1a\x11GJBNIIINKFB.proto\x1a\x11JKMFMMPMNAM.proto\x1a\x11OCBOLHFOIGI.p\
    roto\x1a\x11PGGGCFBKDPK.proto\"\xf3\x02\n\x0bALEFDNLLKLB\x12.\n\x0bNNCJO\
    ECKCKA\x18\x03\x20\x01(\x0b2\x0c.BPPMEIGAHGIR\x0bNNCJOECKCKA\x12+\n\nski\
    ll_info\x18\x06\x20\x01(\x0b2\x0c.GJBNIIINKFBR\tskillInfo\x12.\n\x0bAFPD\
    JDKNENI\x18\x05\x20\x01(\x0b2\x0c.OCBOLHFOIGIR\x0bAFPDJDKNENI\x12.\n\x0b\
    LJGCPNOGIFO\x18\n\x20\x01(\x0b2\x0c.DMJLKIFEMMNR\x0bLJGCPNOGIFO\x12\x20\
    \n\x0bFDEIGEPCCBP\x18\r\x20\x03(\rR\x0bFDEIGEPCCBP\x123\n\x0epending_act\
    ion\x18\x01\x20\x01(\x0b2\x0c.JKMFMMPMNAMR\rpendingAction\x12\x20\n\x0bE\
    ENJBPMNDOL\x18\x04\x20\x01(\rR\x0bEENJBPMNDOL\x12.\n\x0bGBEABIMOBIC\x18\
    \x08\x20\x01(\x0b2\x0c.PGGGCFBKDPKR\x0bGBEABIMOBICb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::BPPMEIGAHGI::file_descriptor().clone());
            deps.push(super::DMJLKIFEMMN::file_descriptor().clone());
            deps.push(super::GJBNIIINKFB::file_descriptor().clone());
            deps.push(super::JKMFMMPMNAM::file_descriptor().clone());
            deps.push(super::OCBOLHFOIGI::file_descriptor().clone());
            deps.push(super::PGGGCFBKDPK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ALEFDNLLKLB::generated_message_descriptor_data());
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
