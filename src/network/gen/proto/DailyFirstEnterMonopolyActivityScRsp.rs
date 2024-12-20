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

//! Generated file from `DailyFirstEnterMonopolyActivityScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DailyFirstEnterMonopolyActivityScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyFirstEnterMonopolyActivityScRsp {
    // message fields
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.KJJEFPLAFOA)
    pub KJJEFPLAFOA: ::protobuf::MessageField<super::MMGHBJCIBMN::MMGHBJCIBMN>,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.BJDKLPNHDBM)
    pub BJDKLPNHDBM: u32,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.IEKHFILHDPG)
    pub IEKHFILHDPG: i64,
    // @@protoc_insertion_point(field:DailyFirstEnterMonopolyActivityScRsp.MMPJOCDINMP)
    pub MMPJOCDINMP: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DailyFirstEnterMonopolyActivityScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyFirstEnterMonopolyActivityScRsp {
    fn default() -> &'a DailyFirstEnterMonopolyActivityScRsp {
        <DailyFirstEnterMonopolyActivityScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DailyFirstEnterMonopolyActivityScRsp {
    pub fn new() -> DailyFirstEnterMonopolyActivityScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MMGHBJCIBMN::MMGHBJCIBMN>(
            "KJJEFPLAFOA",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.KJJEFPLAFOA },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.KJJEFPLAFOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJDKLPNHDBM",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.BJDKLPNHDBM },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.BJDKLPNHDBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.ADADHIHDHJC },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IEKHFILHDPG",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.IEKHFILHDPG },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.IEKHFILHDPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMPJOCDINMP",
            |m: &DailyFirstEnterMonopolyActivityScRsp| { &m.MMPJOCDINMP },
            |m: &mut DailyFirstEnterMonopolyActivityScRsp| { &mut m.MMPJOCDINMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyFirstEnterMonopolyActivityScRsp>(
            "DailyFirstEnterMonopolyActivityScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyFirstEnterMonopolyActivityScRsp {
    const NAME: &'static str = "DailyFirstEnterMonopolyActivityScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KJJEFPLAFOA)?;
                },
                104 => {
                    self.BJDKLPNHDBM = is.read_uint32()?;
                },
                120 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                72 => {
                    self.IEKHFILHDPG = is.read_int64()?;
                },
                64 => {
                    self.MMPJOCDINMP = is.read_bool()?;
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
        if let Some(v) = self.KJJEFPLAFOA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BJDKLPNHDBM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.BJDKLPNHDBM);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.ADADHIHDHJC);
        }
        if self.IEKHFILHDPG != 0 {
            my_size += ::protobuf::rt::int64_size(9, self.IEKHFILHDPG);
        }
        if self.MMPJOCDINMP != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.KJJEFPLAFOA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.BJDKLPNHDBM != 0 {
            os.write_uint32(13, self.BJDKLPNHDBM)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(15, self.ADADHIHDHJC)?;
        }
        if self.IEKHFILHDPG != 0 {
            os.write_int64(9, self.IEKHFILHDPG)?;
        }
        if self.MMPJOCDINMP != false {
            os.write_bool(8, self.MMPJOCDINMP)?;
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

    fn new() -> DailyFirstEnterMonopolyActivityScRsp {
        DailyFirstEnterMonopolyActivityScRsp::new()
    }

    fn clear(&mut self) {
        self.KJJEFPLAFOA.clear();
        self.BJDKLPNHDBM = 0;
        self.ADADHIHDHJC = 0;
        self.IEKHFILHDPG = 0;
        self.MMPJOCDINMP = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyFirstEnterMonopolyActivityScRsp {
        static instance: DailyFirstEnterMonopolyActivityScRsp = DailyFirstEnterMonopolyActivityScRsp {
            KJJEFPLAFOA: ::protobuf::MessageField::none(),
            BJDKLPNHDBM: 0,
            ADADHIHDHJC: 0,
            IEKHFILHDPG: 0,
            MMPJOCDINMP: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyFirstEnterMonopolyActivityScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyFirstEnterMonopolyActivityScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyFirstEnterMonopolyActivityScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyFirstEnterMonopolyActivityScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*DailyFirstEnterMonopolyActivityScRsp.proto\x1a\x11MMGHBJCIBMN.proto\"\
    \xde\x01\n$DailyFirstEnterMonopolyActivityScRsp\x12.\n\x0bKJJEFPLAFOA\
    \x18\x02\x20\x01(\x0b2\x0c.MMGHBJCIBMNR\x0bKJJEFPLAFOA\x12\x20\n\x0bBJDK\
    LPNHDBM\x18\r\x20\x01(\rR\x0bBJDKLPNHDBM\x12\x20\n\x0bADADHIHDHJC\x18\
    \x0f\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bIEKHFILHDPG\x18\t\x20\x01(\
    \x03R\x0bIEKHFILHDPG\x12\x20\n\x0bMMPJOCDINMP\x18\x08\x20\x01(\x08R\x0bM\
    MPJOCDINMPb\x06proto3\
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
            deps.push(super::MMGHBJCIBMN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DailyFirstEnterMonopolyActivityScRsp::generated_message_descriptor_data());
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
