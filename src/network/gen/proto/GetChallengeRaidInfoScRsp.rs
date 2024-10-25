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

//! Generated file from `GetChallengeRaidInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetChallengeRaidInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChallengeRaidInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChallengeRaidInfoScRsp.LCGENCKODGN)
    pub LCGENCKODGN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetChallengeRaidInfoScRsp.GEFKGBAAAJB)
    pub GEFKGBAAAJB: ::std::vec::Vec<super::MHPAJOLEFOF::MHPAJOLEFOF>,
    // @@protoc_insertion_point(field:GetChallengeRaidInfoScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetChallengeRaidInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChallengeRaidInfoScRsp {
    fn default() -> &'a GetChallengeRaidInfoScRsp {
        <GetChallengeRaidInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChallengeRaidInfoScRsp {
    pub fn new() -> GetChallengeRaidInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LCGENCKODGN",
            |m: &GetChallengeRaidInfoScRsp| { &m.LCGENCKODGN },
            |m: &mut GetChallengeRaidInfoScRsp| { &mut m.LCGENCKODGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GEFKGBAAAJB",
            |m: &GetChallengeRaidInfoScRsp| { &m.GEFKGBAAAJB },
            |m: &mut GetChallengeRaidInfoScRsp| { &mut m.GEFKGBAAAJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChallengeRaidInfoScRsp| { &m.retcode },
            |m: &mut GetChallengeRaidInfoScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChallengeRaidInfoScRsp>(
            "GetChallengeRaidInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChallengeRaidInfoScRsp {
    const NAME: &'static str = "GetChallengeRaidInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.LCGENCKODGN)?;
                },
                88 => {
                    self.LCGENCKODGN.push(is.read_uint32()?);
                },
                10 => {
                    self.GEFKGBAAAJB.push(is.read_message()?);
                },
                16 => {
                    self.retcode = is.read_uint32()?;
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
        for value in &self.LCGENCKODGN {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.GEFKGBAAAJB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LCGENCKODGN {
            os.write_uint32(11, *v)?;
        };
        for v in &self.GEFKGBAAAJB {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
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

    fn new() -> GetChallengeRaidInfoScRsp {
        GetChallengeRaidInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.LCGENCKODGN.clear();
        self.GEFKGBAAAJB.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChallengeRaidInfoScRsp {
        static instance: GetChallengeRaidInfoScRsp = GetChallengeRaidInfoScRsp {
            LCGENCKODGN: ::std::vec::Vec::new(),
            GEFKGBAAAJB: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChallengeRaidInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChallengeRaidInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChallengeRaidInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChallengeRaidInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fGetChallengeRaidInfoScRsp.proto\x1a\x11MHPAJOLEFOF.proto\"\x87\x01\
    \n\x19GetChallengeRaidInfoScRsp\x12\x20\n\x0bLCGENCKODGN\x18\x0b\x20\x03\
    (\rR\x0bLCGENCKODGN\x12.\n\x0bGEFKGBAAAJB\x18\x01\x20\x03(\x0b2\x0c.MHPA\
    JOLEFOFR\x0bGEFKGBAAAJB\x12\x18\n\x07retcode\x18\x02\x20\x01(\rR\x07retc\
    odeb\x06proto3\
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
            deps.push(super::MHPAJOLEFOF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChallengeRaidInfoScRsp::generated_message_descriptor_data());
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