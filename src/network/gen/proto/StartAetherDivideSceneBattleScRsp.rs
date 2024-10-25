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

//! Generated file from `StartAetherDivideSceneBattleScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartAetherDivideSceneBattleScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartAetherDivideSceneBattleScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleScRsp.JONHHDCOHBI)
    pub JONHHDCOHBI: ::protobuf::MessageField<super::LFEHJFJHJLH::LFEHJFJHJLH>,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleScRsp.EKOBDMKFJID)
    pub EKOBDMKFJID: u32,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartAetherDivideSceneBattleScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartAetherDivideSceneBattleScRsp {
    fn default() -> &'a StartAetherDivideSceneBattleScRsp {
        <StartAetherDivideSceneBattleScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartAetherDivideSceneBattleScRsp {
    pub fn new() -> StartAetherDivideSceneBattleScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LFEHJFJHJLH::LFEHJFJHJLH>(
            "JONHHDCOHBI",
            |m: &StartAetherDivideSceneBattleScRsp| { &m.JONHHDCOHBI },
            |m: &mut StartAetherDivideSceneBattleScRsp| { &mut m.JONHHDCOHBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKOBDMKFJID",
            |m: &StartAetherDivideSceneBattleScRsp| { &m.EKOBDMKFJID },
            |m: &mut StartAetherDivideSceneBattleScRsp| { &mut m.EKOBDMKFJID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartAetherDivideSceneBattleScRsp| { &m.retcode },
            |m: &mut StartAetherDivideSceneBattleScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartAetherDivideSceneBattleScRsp>(
            "StartAetherDivideSceneBattleScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartAetherDivideSceneBattleScRsp {
    const NAME: &'static str = "StartAetherDivideSceneBattleScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JONHHDCOHBI)?;
                },
                104 => {
                    self.EKOBDMKFJID = is.read_uint32()?;
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
        if let Some(v) = self.JONHHDCOHBI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EKOBDMKFJID != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.EKOBDMKFJID);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.JONHHDCOHBI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.EKOBDMKFJID != 0 {
            os.write_uint32(13, self.EKOBDMKFJID)?;
        }
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

    fn new() -> StartAetherDivideSceneBattleScRsp {
        StartAetherDivideSceneBattleScRsp::new()
    }

    fn clear(&mut self) {
        self.JONHHDCOHBI.clear();
        self.EKOBDMKFJID = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartAetherDivideSceneBattleScRsp {
        static instance: StartAetherDivideSceneBattleScRsp = StartAetherDivideSceneBattleScRsp {
            JONHHDCOHBI: ::protobuf::MessageField::none(),
            EKOBDMKFJID: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartAetherDivideSceneBattleScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartAetherDivideSceneBattleScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartAetherDivideSceneBattleScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartAetherDivideSceneBattleScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'StartAetherDivideSceneBattleScRsp.proto\x1a\x11LFEHJFJHJLH.proto\"\
    \x8f\x01\n!StartAetherDivideSceneBattleScRsp\x12.\n\x0bJONHHDCOHBI\x18\
    \x01\x20\x01(\x0b2\x0c.LFEHJFJHJLHR\x0bJONHHDCOHBI\x12\x20\n\x0bEKOBDMKF\
    JID\x18\r\x20\x01(\rR\x0bEKOBDMKFJID\x12\x18\n\x07retcode\x18\x02\x20\
    \x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::LFEHJFJHJLH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartAetherDivideSceneBattleScRsp::generated_message_descriptor_data());
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
