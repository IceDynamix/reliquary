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

//! Generated file from `GetChallengeGroupStatisticsScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetChallengeGroupStatisticsScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChallengeGroupStatisticsScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // message oneof groups
    pub NBDJIGNEIJE: ::std::option::Option<get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE>,
    // special fields
    // @@protoc_insertion_point(special_field:GetChallengeGroupStatisticsScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChallengeGroupStatisticsScRsp {
    fn default() -> &'a GetChallengeGroupStatisticsScRsp {
        <GetChallengeGroupStatisticsScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChallengeGroupStatisticsScRsp {
    pub fn new() -> GetChallengeGroupStatisticsScRsp {
        ::std::default::Default::default()
    }

    // .IFJLCHNNPDB BDIEFEDBMIC = 12;

    pub fn BDIEFEDBMIC(&self) -> &super::IFJLCHNNPDB::IFJLCHNNPDB {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(ref v)) => v,
            _ => <super::IFJLCHNNPDB::IFJLCHNNPDB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BDIEFEDBMIC(&mut self) {
        self.NBDJIGNEIJE = ::std::option::Option::None;
    }

    pub fn has_BDIEFEDBMIC(&self) -> bool {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BDIEFEDBMIC(&mut self, v: super::IFJLCHNNPDB::IFJLCHNNPDB) {
        self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BDIEFEDBMIC(&mut self) -> &mut super::IFJLCHNNPDB::IFJLCHNNPDB {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(_)) = self.NBDJIGNEIJE {
        } else {
            self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(super::IFJLCHNNPDB::IFJLCHNNPDB::new()));
        }
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BDIEFEDBMIC(&mut self) -> super::IFJLCHNNPDB::IFJLCHNNPDB {
        if self.has_BDIEFEDBMIC() {
            match self.NBDJIGNEIJE.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IFJLCHNNPDB::IFJLCHNNPDB::new()
        }
    }

    // .DMILKHDLOLJ BBFACNPMACC = 4;

    pub fn BBFACNPMACC(&self) -> &super::DMILKHDLOLJ::DMILKHDLOLJ {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(ref v)) => v,
            _ => <super::DMILKHDLOLJ::DMILKHDLOLJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_BBFACNPMACC(&mut self) {
        self.NBDJIGNEIJE = ::std::option::Option::None;
    }

    pub fn has_BBFACNPMACC(&self) -> bool {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_BBFACNPMACC(&mut self, v: super::DMILKHDLOLJ::DMILKHDLOLJ) {
        self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_BBFACNPMACC(&mut self) -> &mut super::DMILKHDLOLJ::DMILKHDLOLJ {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(_)) = self.NBDJIGNEIJE {
        } else {
            self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(super::DMILKHDLOLJ::DMILKHDLOLJ::new()));
        }
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_BBFACNPMACC(&mut self) -> super::DMILKHDLOLJ::DMILKHDLOLJ {
        if self.has_BBFACNPMACC() {
            match self.NBDJIGNEIJE.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DMILKHDLOLJ::DMILKHDLOLJ::new()
        }
    }

    // .KBCOKNIMFOH LBMKGHECBEI = 15;

    pub fn LBMKGHECBEI(&self) -> &super::KBCOKNIMFOH::KBCOKNIMFOH {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(ref v)) => v,
            _ => <super::KBCOKNIMFOH::KBCOKNIMFOH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LBMKGHECBEI(&mut self) {
        self.NBDJIGNEIJE = ::std::option::Option::None;
    }

    pub fn has_LBMKGHECBEI(&self) -> bool {
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LBMKGHECBEI(&mut self, v: super::KBCOKNIMFOH::KBCOKNIMFOH) {
        self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LBMKGHECBEI(&mut self) -> &mut super::KBCOKNIMFOH::KBCOKNIMFOH {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(_)) = self.NBDJIGNEIJE {
        } else {
            self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(super::KBCOKNIMFOH::KBCOKNIMFOH::new()));
        }
        match self.NBDJIGNEIJE {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LBMKGHECBEI(&mut self) -> super::KBCOKNIMFOH::KBCOKNIMFOH {
        if self.has_LBMKGHECBEI() {
            match self.NBDJIGNEIJE.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::KBCOKNIMFOH::KBCOKNIMFOH::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.retcode },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.GCFIIGOLPMF },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IFJLCHNNPDB::IFJLCHNNPDB>(
            "BDIEFEDBMIC",
            GetChallengeGroupStatisticsScRsp::has_BDIEFEDBMIC,
            GetChallengeGroupStatisticsScRsp::BDIEFEDBMIC,
            GetChallengeGroupStatisticsScRsp::mut_BDIEFEDBMIC,
            GetChallengeGroupStatisticsScRsp::set_BDIEFEDBMIC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DMILKHDLOLJ::DMILKHDLOLJ>(
            "BBFACNPMACC",
            GetChallengeGroupStatisticsScRsp::has_BBFACNPMACC,
            GetChallengeGroupStatisticsScRsp::BBFACNPMACC,
            GetChallengeGroupStatisticsScRsp::mut_BBFACNPMACC,
            GetChallengeGroupStatisticsScRsp::set_BBFACNPMACC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::KBCOKNIMFOH::KBCOKNIMFOH>(
            "LBMKGHECBEI",
            GetChallengeGroupStatisticsScRsp::has_LBMKGHECBEI,
            GetChallengeGroupStatisticsScRsp::LBMKGHECBEI,
            GetChallengeGroupStatisticsScRsp::mut_LBMKGHECBEI,
            GetChallengeGroupStatisticsScRsp::set_LBMKGHECBEI,
        ));
        oneofs.push(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChallengeGroupStatisticsScRsp>(
            "GetChallengeGroupStatisticsScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChallengeGroupStatisticsScRsp {
    const NAME: &'static str = "GetChallengeGroupStatisticsScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                112 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                98 => {
                    self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(is.read_message()?));
                },
                34 => {
                    self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(is.read_message()?));
                },
                122 => {
                    self.NBDJIGNEIJE = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(is.read_message()?));
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
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GCFIIGOLPMF);
        }
        if let ::std::option::Option::Some(ref v) = self.NBDJIGNEIJE {
            match v {
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(14, self.GCFIIGOLPMF)?;
        }
        if let ::std::option::Option::Some(ref v) = self.NBDJIGNEIJE {
            match v {
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BDIEFEDBMIC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::BBFACNPMACC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::NBDJIGNEIJE::LBMKGHECBEI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
            };
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

    fn new() -> GetChallengeGroupStatisticsScRsp {
        GetChallengeGroupStatisticsScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.GCFIIGOLPMF = 0;
        self.NBDJIGNEIJE = ::std::option::Option::None;
        self.NBDJIGNEIJE = ::std::option::Option::None;
        self.NBDJIGNEIJE = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChallengeGroupStatisticsScRsp {
        static instance: GetChallengeGroupStatisticsScRsp = GetChallengeGroupStatisticsScRsp {
            retcode: 0,
            GCFIIGOLPMF: 0,
            NBDJIGNEIJE: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChallengeGroupStatisticsScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChallengeGroupStatisticsScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChallengeGroupStatisticsScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChallengeGroupStatisticsScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `GetChallengeGroupStatisticsScRsp`
pub mod get_challenge_group_statistics_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:GetChallengeGroupStatisticsScRsp.NBDJIGNEIJE)
    pub enum NBDJIGNEIJE {
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.BDIEFEDBMIC)
        BDIEFEDBMIC(super::super::IFJLCHNNPDB::IFJLCHNNPDB),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.BBFACNPMACC)
        BBFACNPMACC(super::super::DMILKHDLOLJ::DMILKHDLOLJ),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.LBMKGHECBEI)
        LBMKGHECBEI(super::super::KBCOKNIMFOH::KBCOKNIMFOH),
    }

    impl ::protobuf::Oneof for NBDJIGNEIJE {
    }

    impl ::protobuf::OneofFull for NBDJIGNEIJE {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::GetChallengeGroupStatisticsScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("NBDJIGNEIJE").unwrap()).clone()
        }
    }

    impl NBDJIGNEIJE {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NBDJIGNEIJE>("NBDJIGNEIJE")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&GetChallengeGroupStatisticsScRsp.proto\x1a\x11DMILKHDLOLJ.proto\x1a\
    \x11IFJLCHNNPDB.proto\x1a\x11KBCOKNIMFOH.proto\"\x83\x02\n\x20GetChallen\
    geGroupStatisticsScRsp\x12\x18\n\x07retcode\x18\x08\x20\x01(\rR\x07retco\
    de\x12\x20\n\x0bGCFIIGOLPMF\x18\x0e\x20\x01(\rR\x0bGCFIIGOLPMF\x120\n\
    \x0bBDIEFEDBMIC\x18\x0c\x20\x01(\x0b2\x0c.IFJLCHNNPDBH\0R\x0bBDIEFEDBMIC\
    \x120\n\x0bBBFACNPMACC\x18\x04\x20\x01(\x0b2\x0c.DMILKHDLOLJH\0R\x0bBBFA\
    CNPMACC\x120\n\x0bLBMKGHECBEI\x18\x0f\x20\x01(\x0b2\x0c.KBCOKNIMFOHH\0R\
    \x0bLBMKGHECBEIB\r\n\x0bNBDJIGNEIJEb\x06proto3\
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
            deps.push(super::DMILKHDLOLJ::file_descriptor().clone());
            deps.push(super::IFJLCHNNPDB::file_descriptor().clone());
            deps.push(super::KBCOKNIMFOH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChallengeGroupStatisticsScRsp::generated_message_descriptor_data());
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
