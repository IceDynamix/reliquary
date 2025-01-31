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

//! Generated file from `GetAetherDivideInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetAetherDivideInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAetherDivideInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.AJFGHJKDMLK)
    pub AJFGHJKDMLK: u32,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.ABOOCDHAPAF)
    pub ABOOCDHAPAF: ::std::vec::Vec<super::JKHDPPNJELA::JKHDPPNJELA>,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.GFIMLBECJLC)
    pub GFIMLBECJLC: u32,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.FGILHCHHNDM)
    pub FGILHCHHNDM: ::std::vec::Vec<super::CHEONOGHMHD::CHEONOGHMHD>,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.CECNPIOKEOH)
    pub CECNPIOKEOH: u32,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.PEBJFCLBMGC)
    pub PEBJFCLBMGC: u32,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.AJFGJKFAJEK)
    pub AJFGJKFAJEK: ::std::vec::Vec<super::FIAMDPMMDJL::FIAMDPMMDJL>,
    // @@protoc_insertion_point(field:GetAetherDivideInfoScRsp.OBHCOCHAFAA)
    pub OBHCOCHAFAA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetAetherDivideInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAetherDivideInfoScRsp {
    fn default() -> &'a GetAetherDivideInfoScRsp {
        <GetAetherDivideInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAetherDivideInfoScRsp {
    pub fn new() -> GetAetherDivideInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJFGHJKDMLK",
            |m: &GetAetherDivideInfoScRsp| { &m.AJFGHJKDMLK },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.AJFGHJKDMLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ABOOCDHAPAF",
            |m: &GetAetherDivideInfoScRsp| { &m.ABOOCDHAPAF },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.ABOOCDHAPAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFIMLBECJLC",
            |m: &GetAetherDivideInfoScRsp| { &m.GFIMLBECJLC },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.GFIMLBECJLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAetherDivideInfoScRsp| { &m.retcode },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FGILHCHHNDM",
            |m: &GetAetherDivideInfoScRsp| { &m.FGILHCHHNDM },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.FGILHCHHNDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CECNPIOKEOH",
            |m: &GetAetherDivideInfoScRsp| { &m.CECNPIOKEOH },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.CECNPIOKEOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PEBJFCLBMGC",
            |m: &GetAetherDivideInfoScRsp| { &m.PEBJFCLBMGC },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.PEBJFCLBMGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AJFGJKFAJEK",
            |m: &GetAetherDivideInfoScRsp| { &m.AJFGJKFAJEK },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.AJFGJKFAJEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBHCOCHAFAA",
            |m: &GetAetherDivideInfoScRsp| { &m.OBHCOCHAFAA },
            |m: &mut GetAetherDivideInfoScRsp| { &mut m.OBHCOCHAFAA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAetherDivideInfoScRsp>(
            "GetAetherDivideInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAetherDivideInfoScRsp {
    const NAME: &'static str = "GetAetherDivideInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.AJFGHJKDMLK = is.read_uint32()?;
                },
                50 => {
                    self.ABOOCDHAPAF.push(is.read_message()?);
                },
                104 => {
                    self.GFIMLBECJLC = is.read_uint32()?;
                },
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                98 => {
                    self.FGILHCHHNDM.push(is.read_message()?);
                },
                24 => {
                    self.CECNPIOKEOH = is.read_uint32()?;
                },
                64 => {
                    self.PEBJFCLBMGC = is.read_uint32()?;
                },
                34 => {
                    self.AJFGJKFAJEK.push(is.read_message()?);
                },
                120 => {
                    self.OBHCOCHAFAA = is.read_uint32()?;
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
        if self.AJFGHJKDMLK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.AJFGHJKDMLK);
        }
        for value in &self.ABOOCDHAPAF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.GFIMLBECJLC != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.GFIMLBECJLC);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        for value in &self.FGILHCHHNDM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CECNPIOKEOH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.CECNPIOKEOH);
        }
        if self.PEBJFCLBMGC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PEBJFCLBMGC);
        }
        for value in &self.AJFGJKFAJEK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.OBHCOCHAFAA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OBHCOCHAFAA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AJFGHJKDMLK != 0 {
            os.write_uint32(7, self.AJFGHJKDMLK)?;
        }
        for v in &self.ABOOCDHAPAF {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.GFIMLBECJLC != 0 {
            os.write_uint32(13, self.GFIMLBECJLC)?;
        }
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        for v in &self.FGILHCHHNDM {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.CECNPIOKEOH != 0 {
            os.write_uint32(3, self.CECNPIOKEOH)?;
        }
        if self.PEBJFCLBMGC != 0 {
            os.write_uint32(8, self.PEBJFCLBMGC)?;
        }
        for v in &self.AJFGJKFAJEK {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.OBHCOCHAFAA != 0 {
            os.write_uint32(15, self.OBHCOCHAFAA)?;
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

    fn new() -> GetAetherDivideInfoScRsp {
        GetAetherDivideInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.AJFGHJKDMLK = 0;
        self.ABOOCDHAPAF.clear();
        self.GFIMLBECJLC = 0;
        self.retcode = 0;
        self.FGILHCHHNDM.clear();
        self.CECNPIOKEOH = 0;
        self.PEBJFCLBMGC = 0;
        self.AJFGJKFAJEK.clear();
        self.OBHCOCHAFAA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAetherDivideInfoScRsp {
        static instance: GetAetherDivideInfoScRsp = GetAetherDivideInfoScRsp {
            AJFGHJKDMLK: 0,
            ABOOCDHAPAF: ::std::vec::Vec::new(),
            GFIMLBECJLC: 0,
            retcode: 0,
            FGILHCHHNDM: ::std::vec::Vec::new(),
            CECNPIOKEOH: 0,
            PEBJFCLBMGC: 0,
            AJFGJKFAJEK: ::std::vec::Vec::new(),
            OBHCOCHAFAA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAetherDivideInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAetherDivideInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAetherDivideInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAetherDivideInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eGetAetherDivideInfoScRsp.proto\x1a\x11CHEONOGHMHD.proto\x1a\x11FIA\
    MDPMMDJL.proto\x1a\x11JKHDPPNJELA.proto\"\xee\x02\n\x18GetAetherDivideIn\
    foScRsp\x12\x20\n\x0bAJFGHJKDMLK\x18\x07\x20\x01(\rR\x0bAJFGHJKDMLK\x12.\
    \n\x0bABOOCDHAPAF\x18\x06\x20\x03(\x0b2\x0c.JKHDPPNJELAR\x0bABOOCDHAPAF\
    \x12\x20\n\x0bGFIMLBECJLC\x18\r\x20\x01(\rR\x0bGFIMLBECJLC\x12\x18\n\x07\
    retcode\x18\t\x20\x01(\rR\x07retcode\x12.\n\x0bFGILHCHHNDM\x18\x0c\x20\
    \x03(\x0b2\x0c.CHEONOGHMHDR\x0bFGILHCHHNDM\x12\x20\n\x0bCECNPIOKEOH\x18\
    \x03\x20\x01(\rR\x0bCECNPIOKEOH\x12\x20\n\x0bPEBJFCLBMGC\x18\x08\x20\x01\
    (\rR\x0bPEBJFCLBMGC\x12.\n\x0bAJFGJKFAJEK\x18\x04\x20\x03(\x0b2\x0c.FIAM\
    DPMMDJLR\x0bAJFGJKFAJEK\x12\x20\n\x0bOBHCOCHAFAA\x18\x0f\x20\x01(\rR\x0b\
    OBHCOCHAFAAb\x06proto3\
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
            deps.push(super::CHEONOGHMHD::file_descriptor().clone());
            deps.push(super::FIAMDPMMDJL::file_descriptor().clone());
            deps.push(super::JKHDPPNJELA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetAetherDivideInfoScRsp::generated_message_descriptor_data());
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
