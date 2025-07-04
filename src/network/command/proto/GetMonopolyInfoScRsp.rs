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

//! Generated file from `GetMonopolyInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetMonopolyInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetMonopolyInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.stt)
    pub stt: ::protobuf::MessageField<super::KJBMLBGIBJF::KJBMLBGIBJF>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.rogue_buff_info)
    pub rogue_buff_info: ::protobuf::MessageField<super::AFDALBGANPC::AFDALBGANPC>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.HEHJKFILINN)
    pub HEHJKFILINN: ::protobuf::MessageField<super::NFDGIJLOLGD::NFDGIJLOLGD>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.rogue_map)
    pub rogue_map: ::protobuf::MessageField<super::JAJGKKDPALC::JAJGKKDPALC>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.FGHCIADCMNJ)
    pub FGHCIADCMNJ: ::protobuf::MessageField<super::HFDGMJJFOHM::HFDGMJJFOHM>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.IEDGKHDJJDC)
    pub IEDGKHDJJDC: ::protobuf::MessageField<super::EDKGOMNEHOH::EDKGOMNEHOH>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.OELHKEIPIDJ)
    pub OELHKEIPIDJ: ::protobuf::MessageField<super::CANNIBGCLCL::CANNIBGCLCL>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.HLJMHNABFMC)
    pub HLJMHNABFMC: ::protobuf::MessageField<super::ODAIJIGEAJL::ODAIJIGEAJL>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.GPFGDOKNGEL)
    pub GPFGDOKNGEL: ::protobuf::MessageField<super::ICIHABOLHPN::ICIHABOLHPN>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.COKCGFMEIBA)
    pub COKCGFMEIBA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.LJAOGAPDFHA)
    pub LJAOGAPDFHA: ::protobuf::MessageField<super::AEDKPBFCKGO::AEDKPBFCKGO>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.JNHJEELJFHF)
    pub JNHJEELJFHF: ::protobuf::MessageField<super::LLGNIKNMCKE::LLGNIKNMCKE>,
    // special fields
    // @@protoc_insertion_point(special_field:GetMonopolyInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetMonopolyInfoScRsp {
    fn default() -> &'a GetMonopolyInfoScRsp {
        <GetMonopolyInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetMonopolyInfoScRsp {
    pub fn new() -> GetMonopolyInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KJBMLBGIBJF::KJBMLBGIBJF>(
            "stt",
            |m: &GetMonopolyInfoScRsp| { &m.stt },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.stt },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AFDALBGANPC::AFDALBGANPC>(
            "rogue_buff_info",
            |m: &GetMonopolyInfoScRsp| { &m.rogue_buff_info },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.rogue_buff_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NFDGIJLOLGD::NFDGIJLOLGD>(
            "HEHJKFILINN",
            |m: &GetMonopolyInfoScRsp| { &m.HEHJKFILINN },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.HEHJKFILINN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JAJGKKDPALC::JAJGKKDPALC>(
            "rogue_map",
            |m: &GetMonopolyInfoScRsp| { &m.rogue_map },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.rogue_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HFDGMJJFOHM::HFDGMJJFOHM>(
            "FGHCIADCMNJ",
            |m: &GetMonopolyInfoScRsp| { &m.FGHCIADCMNJ },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.FGHCIADCMNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EDKGOMNEHOH::EDKGOMNEHOH>(
            "IEDGKHDJJDC",
            |m: &GetMonopolyInfoScRsp| { &m.IEDGKHDJJDC },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.IEDGKHDJJDC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CANNIBGCLCL::CANNIBGCLCL>(
            "OELHKEIPIDJ",
            |m: &GetMonopolyInfoScRsp| { &m.OELHKEIPIDJ },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.OELHKEIPIDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ODAIJIGEAJL::ODAIJIGEAJL>(
            "HLJMHNABFMC",
            |m: &GetMonopolyInfoScRsp| { &m.HLJMHNABFMC },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.HLJMHNABFMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ICIHABOLHPN::ICIHABOLHPN>(
            "GPFGDOKNGEL",
            |m: &GetMonopolyInfoScRsp| { &m.GPFGDOKNGEL },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.GPFGDOKNGEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "COKCGFMEIBA",
            |m: &GetMonopolyInfoScRsp| { &m.COKCGFMEIBA },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.COKCGFMEIBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AEDKPBFCKGO::AEDKPBFCKGO>(
            "LJAOGAPDFHA",
            |m: &GetMonopolyInfoScRsp| { &m.LJAOGAPDFHA },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.LJAOGAPDFHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetMonopolyInfoScRsp| { &m.retcode },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LLGNIKNMCKE::LLGNIKNMCKE>(
            "JNHJEELJFHF",
            |m: &GetMonopolyInfoScRsp| { &m.JNHJEELJFHF },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.JNHJEELJFHF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetMonopolyInfoScRsp>(
            "GetMonopolyInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetMonopolyInfoScRsp {
    const NAME: &'static str = "GetMonopolyInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stt)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_buff_info)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HEHJKFILINN)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_map)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FGHCIADCMNJ)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IEDGKHDJJDC)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OELHKEIPIDJ)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HLJMHNABFMC)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GPFGDOKNGEL)?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.COKCGFMEIBA)?;
                },
                8 => {
                    self.COKCGFMEIBA.push(is.read_uint32()?);
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LJAOGAPDFHA)?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JNHJEELJFHF)?;
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
        if let Some(v) = self.stt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_buff_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HEHJKFILINN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.rogue_map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FGHCIADCMNJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IEDGKHDJJDC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.OELHKEIPIDJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HLJMHNABFMC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GPFGDOKNGEL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.COKCGFMEIBA);
        if let Some(v) = self.LJAOGAPDFHA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if let Some(v) = self.JNHJEELJFHF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.stt.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.rogue_buff_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.HEHJKFILINN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.rogue_map.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.FGHCIADCMNJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.IEDGKHDJJDC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.OELHKEIPIDJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.HLJMHNABFMC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.GPFGDOKNGEL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        os.write_repeated_packed_uint32(1, &self.COKCGFMEIBA)?;
        if let Some(v) = self.LJAOGAPDFHA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if let Some(v) = self.JNHJEELJFHF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
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

    fn new() -> GetMonopolyInfoScRsp {
        GetMonopolyInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.stt.clear();
        self.rogue_buff_info.clear();
        self.HEHJKFILINN.clear();
        self.rogue_map.clear();
        self.FGHCIADCMNJ.clear();
        self.IEDGKHDJJDC.clear();
        self.OELHKEIPIDJ.clear();
        self.HLJMHNABFMC.clear();
        self.GPFGDOKNGEL.clear();
        self.COKCGFMEIBA.clear();
        self.LJAOGAPDFHA.clear();
        self.retcode = 0;
        self.JNHJEELJFHF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetMonopolyInfoScRsp {
        static instance: GetMonopolyInfoScRsp = GetMonopolyInfoScRsp {
            stt: ::protobuf::MessageField::none(),
            rogue_buff_info: ::protobuf::MessageField::none(),
            HEHJKFILINN: ::protobuf::MessageField::none(),
            rogue_map: ::protobuf::MessageField::none(),
            FGHCIADCMNJ: ::protobuf::MessageField::none(),
            IEDGKHDJJDC: ::protobuf::MessageField::none(),
            OELHKEIPIDJ: ::protobuf::MessageField::none(),
            HLJMHNABFMC: ::protobuf::MessageField::none(),
            GPFGDOKNGEL: ::protobuf::MessageField::none(),
            COKCGFMEIBA: ::std::vec::Vec::new(),
            LJAOGAPDFHA: ::protobuf::MessageField::none(),
            retcode: 0,
            JNHJEELJFHF: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetMonopolyInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetMonopolyInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetMonopolyInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMonopolyInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetMonopolyInfoScRsp.proto\x1a\x11AEDKPBFCKGO.proto\x1a\x11AFDALBG\
    ANPC.proto\x1a\x11CANNIBGCLCL.proto\x1a\x11EDKGOMNEHOH.proto\x1a\x11HFDG\
    MJJFOHM.proto\x1a\x11ICIHABOLHPN.proto\x1a\x11JAJGKKDPALC.proto\x1a\x11K\
    JBMLBGIBJF.proto\x1a\x11LLGNIKNMCKE.proto\x1a\x11NFDGIJLOLGD.proto\x1a\
    \x11ODAIJIGEAJL.proto\"\xd3\x04\n\x14GetMonopolyInfoScRsp\x12\x1e\n\x03s\
    tt\x18\x05\x20\x01(\x0b2\x0c.KJBMLBGIBJFR\x03stt\x124\n\x0frogue_buff_in\
    fo\x18\x0e\x20\x01(\x0b2\x0c.AFDALBGANPCR\rrogueBuffInfo\x12.\n\x0bHEHJK\
    FILINN\x18\x0b\x20\x01(\x0b2\x0c.NFDGIJLOLGDR\x0bHEHJKFILINN\x12)\n\trog\
    ue_map\x18\r\x20\x01(\x0b2\x0c.JAJGKKDPALCR\x08rogueMap\x12.\n\x0bFGHCIA\
    DCMNJ\x18\x08\x20\x01(\x0b2\x0c.HFDGMJJFOHMR\x0bFGHCIADCMNJ\x12.\n\x0bIE\
    DGKHDJJDC\x18\x04\x20\x01(\x0b2\x0c.EDKGOMNEHOHR\x0bIEDGKHDJJDC\x12.\n\
    \x0bOELHKEIPIDJ\x18\x07\x20\x01(\x0b2\x0c.CANNIBGCLCLR\x0bOELHKEIPIDJ\
    \x12.\n\x0bHLJMHNABFMC\x18\x0c\x20\x01(\x0b2\x0c.ODAIJIGEAJLR\x0bHLJMHNA\
    BFMC\x12.\n\x0bGPFGDOKNGEL\x18\x0f\x20\x01(\x0b2\x0c.ICIHABOLHPNR\x0bGPF\
    GDOKNGEL\x12\x20\n\x0bCOKCGFMEIBA\x18\x01\x20\x03(\rR\x0bCOKCGFMEIBA\x12\
    .\n\x0bLJAOGAPDFHA\x18\t\x20\x01(\x0b2\x0c.AEDKPBFCKGOR\x0bLJAOGAPDFHA\
    \x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\x07retcode\x12.\n\x0bJNHJEELJF\
    HF\x18\n\x20\x01(\x0b2\x0c.LLGNIKNMCKER\x0bJNHJEELJFHFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::AEDKPBFCKGO::file_descriptor().clone());
            deps.push(super::AFDALBGANPC::file_descriptor().clone());
            deps.push(super::CANNIBGCLCL::file_descriptor().clone());
            deps.push(super::EDKGOMNEHOH::file_descriptor().clone());
            deps.push(super::HFDGMJJFOHM::file_descriptor().clone());
            deps.push(super::ICIHABOLHPN::file_descriptor().clone());
            deps.push(super::JAJGKKDPALC::file_descriptor().clone());
            deps.push(super::KJBMLBGIBJF::file_descriptor().clone());
            deps.push(super::LLGNIKNMCKE::file_descriptor().clone());
            deps.push(super::NFDGIJLOLGD::file_descriptor().clone());
            deps.push(super::ODAIJIGEAJL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetMonopolyInfoScRsp::generated_message_descriptor_data());
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
