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

//! Generated file from `StartPunkLordRaidScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartPunkLordRaidScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartPunkLordRaidScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.AGEGDMGNPDK)
    pub AGEGDMGNPDK: i64,
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.CLOMMFKJPMM)
    pub CLOMMFKJPMM: bool,
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.IDOMKBKKKKL)
    pub IDOMKBKKKKL: ::protobuf::MessageField<super::FNLGPLNCPCL::FNLGPLNCPCL>,
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.DPMKAMMIOLB)
    pub DPMKAMMIOLB: ::protobuf::MessageField<super::KAOAHKAOHFI::KAOAHKAOHFI>,
    // @@protoc_insertion_point(field:StartPunkLordRaidScRsp.DMILCFHLIHP)
    pub DMILCFHLIHP: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:StartPunkLordRaidScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartPunkLordRaidScRsp {
    fn default() -> &'a StartPunkLordRaidScRsp {
        <StartPunkLordRaidScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartPunkLordRaidScRsp {
    pub fn new() -> StartPunkLordRaidScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartPunkLordRaidScRsp| { &m.retcode },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGEGDMGNPDK",
            |m: &StartPunkLordRaidScRsp| { &m.AGEGDMGNPDK },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.AGEGDMGNPDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLOMMFKJPMM",
            |m: &StartPunkLordRaidScRsp| { &m.CLOMMFKJPMM },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.CLOMMFKJPMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FNLGPLNCPCL::FNLGPLNCPCL>(
            "IDOMKBKKKKL",
            |m: &StartPunkLordRaidScRsp| { &m.IDOMKBKKKKL },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.IDOMKBKKKKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KAOAHKAOHFI::KAOAHKAOHFI>(
            "DPMKAMMIOLB",
            |m: &StartPunkLordRaidScRsp| { &m.DPMKAMMIOLB },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.DPMKAMMIOLB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DMILCFHLIHP",
            |m: &StartPunkLordRaidScRsp| { &m.DMILCFHLIHP },
            |m: &mut StartPunkLordRaidScRsp| { &mut m.DMILCFHLIHP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartPunkLordRaidScRsp>(
            "StartPunkLordRaidScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartPunkLordRaidScRsp {
    const NAME: &'static str = "StartPunkLordRaidScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                96 => {
                    self.AGEGDMGNPDK = is.read_int64()?;
                },
                120 => {
                    self.CLOMMFKJPMM = is.read_bool()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IDOMKBKKKKL)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DPMKAMMIOLB)?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.DMILCFHLIHP)?;
                },
                80 => {
                    self.DMILCFHLIHP.push(is.read_uint32()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if self.AGEGDMGNPDK != 0 {
            my_size += ::protobuf::rt::int64_size(12, self.AGEGDMGNPDK);
        }
        if self.CLOMMFKJPMM != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DPMKAMMIOLB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.DMILCFHLIHP);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if self.AGEGDMGNPDK != 0 {
            os.write_int64(12, self.AGEGDMGNPDK)?;
        }
        if self.CLOMMFKJPMM != false {
            os.write_bool(15, self.CLOMMFKJPMM)?;
        }
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.DPMKAMMIOLB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        os.write_repeated_packed_uint32(10, &self.DMILCFHLIHP)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StartPunkLordRaidScRsp {
        StartPunkLordRaidScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.AGEGDMGNPDK = 0;
        self.CLOMMFKJPMM = false;
        self.IDOMKBKKKKL.clear();
        self.DPMKAMMIOLB.clear();
        self.DMILCFHLIHP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartPunkLordRaidScRsp {
        static instance: StartPunkLordRaidScRsp = StartPunkLordRaidScRsp {
            retcode: 0,
            AGEGDMGNPDK: 0,
            CLOMMFKJPMM: false,
            IDOMKBKKKKL: ::protobuf::MessageField::none(),
            DPMKAMMIOLB: ::protobuf::MessageField::none(),
            DMILCFHLIHP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartPunkLordRaidScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartPunkLordRaidScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartPunkLordRaidScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartPunkLordRaidScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cStartPunkLordRaidScRsp.proto\x1a\x11FNLGPLNCPCL.proto\x1a\x11KAOAH\
    KAOHFI.proto\"\xf8\x01\n\x16StartPunkLordRaidScRsp\x12\x18\n\x07retcode\
    \x18\x06\x20\x01(\rR\x07retcode\x12\x20\n\x0bAGEGDMGNPDK\x18\x0c\x20\x01\
    (\x03R\x0bAGEGDMGNPDK\x12\x20\n\x0bCLOMMFKJPMM\x18\x0f\x20\x01(\x08R\x0b\
    CLOMMFKJPMM\x12.\n\x0bIDOMKBKKKKL\x18\t\x20\x01(\x0b2\x0c.FNLGPLNCPCLR\
    \x0bIDOMKBKKKKL\x12.\n\x0bDPMKAMMIOLB\x18\x03\x20\x01(\x0b2\x0c.KAOAHKAO\
    HFIR\x0bDPMKAMMIOLB\x12\x20\n\x0bDMILCFHLIHP\x18\n\x20\x03(\rR\x0bDMILCF\
    HLIHPb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::FNLGPLNCPCL::file_descriptor().clone());
            deps.push(super::KAOAHKAOHFI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartPunkLordRaidScRsp::generated_message_descriptor_data());
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
