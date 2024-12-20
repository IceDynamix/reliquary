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

//! Generated file from `RogueTournGetMiscRealTimeDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueTournGetMiscRealTimeDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournGetMiscRealTimeDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:RogueTournGetMiscRealTimeDataScRsp.LBCCHMLPACD)
    pub LBCCHMLPACD: ::protobuf::MessageField<super::MLJBIFELFCN::MLJBIFELFCN>,
    // @@protoc_insertion_point(field:RogueTournGetMiscRealTimeDataScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:RogueTournGetMiscRealTimeDataScRsp.KIHFIIMHNIF)
    pub KIHFIIMHNIF: ::protobuf::MessageField<super::EHOCFBLOPKL::EHOCFBLOPKL>,
    // @@protoc_insertion_point(field:RogueTournGetMiscRealTimeDataScRsp.JCBGNKMLIKN)
    pub JCBGNKMLIKN: ::protobuf::MessageField<super::JHDCKDNIFID::JHDCKDNIFID>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournGetMiscRealTimeDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournGetMiscRealTimeDataScRsp {
    fn default() -> &'a RogueTournGetMiscRealTimeDataScRsp {
        <RogueTournGetMiscRealTimeDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournGetMiscRealTimeDataScRsp {
    pub fn new() -> RogueTournGetMiscRealTimeDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MLJBIFELFCN::MLJBIFELFCN>(
            "LBCCHMLPACD",
            |m: &RogueTournGetMiscRealTimeDataScRsp| { &m.LBCCHMLPACD },
            |m: &mut RogueTournGetMiscRealTimeDataScRsp| { &mut m.LBCCHMLPACD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &RogueTournGetMiscRealTimeDataScRsp| { &m.ADADHIHDHJC },
            |m: &mut RogueTournGetMiscRealTimeDataScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EHOCFBLOPKL::EHOCFBLOPKL>(
            "KIHFIIMHNIF",
            |m: &RogueTournGetMiscRealTimeDataScRsp| { &m.KIHFIIMHNIF },
            |m: &mut RogueTournGetMiscRealTimeDataScRsp| { &mut m.KIHFIIMHNIF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JHDCKDNIFID::JHDCKDNIFID>(
            "JCBGNKMLIKN",
            |m: &RogueTournGetMiscRealTimeDataScRsp| { &m.JCBGNKMLIKN },
            |m: &mut RogueTournGetMiscRealTimeDataScRsp| { &mut m.JCBGNKMLIKN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournGetMiscRealTimeDataScRsp>(
            "RogueTournGetMiscRealTimeDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournGetMiscRealTimeDataScRsp {
    const NAME: &'static str = "RogueTournGetMiscRealTimeDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LBCCHMLPACD)?;
                },
                96 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KIHFIIMHNIF)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JCBGNKMLIKN)?;
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
        if let Some(v) = self.LBCCHMLPACD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.ADADHIHDHJC);
        }
        if let Some(v) = self.KIHFIIMHNIF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.JCBGNKMLIKN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.LBCCHMLPACD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(12, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.KIHFIIMHNIF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.JCBGNKMLIKN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> RogueTournGetMiscRealTimeDataScRsp {
        RogueTournGetMiscRealTimeDataScRsp::new()
    }

    fn clear(&mut self) {
        self.LBCCHMLPACD.clear();
        self.ADADHIHDHJC = 0;
        self.KIHFIIMHNIF.clear();
        self.JCBGNKMLIKN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournGetMiscRealTimeDataScRsp {
        static instance: RogueTournGetMiscRealTimeDataScRsp = RogueTournGetMiscRealTimeDataScRsp {
            LBCCHMLPACD: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            KIHFIIMHNIF: ::protobuf::MessageField::none(),
            JCBGNKMLIKN: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournGetMiscRealTimeDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournGetMiscRealTimeDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournGetMiscRealTimeDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournGetMiscRealTimeDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(RogueTournGetMiscRealTimeDataScRsp.proto\x1a\x11EHOCFBLOPKL.proto\x1a\
    \x11JHDCKDNIFID.proto\x1a\x11MLJBIFELFCN.proto\"\xd6\x01\n\"RogueTournGe\
    tMiscRealTimeDataScRsp\x12.\n\x0bLBCCHMLPACD\x18\x08\x20\x01(\x0b2\x0c.M\
    LJBIFELFCNR\x0bLBCCHMLPACD\x12\x20\n\x0bADADHIHDHJC\x18\x0c\x20\x01(\rR\
    \x0bADADHIHDHJC\x12.\n\x0bKIHFIIMHNIF\x18\x03\x20\x01(\x0b2\x0c.EHOCFBLO\
    PKLR\x0bKIHFIIMHNIF\x12.\n\x0bJCBGNKMLIKN\x18\x07\x20\x01(\x0b2\x0c.JHDC\
    KDNIFIDR\x0bJCBGNKMLIKNb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EHOCFBLOPKL::file_descriptor().clone());
            deps.push(super::JHDCKDNIFID::file_descriptor().clone());
            deps.push(super::MLJBIFELFCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueTournGetMiscRealTimeDataScRsp::generated_message_descriptor_data());
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
