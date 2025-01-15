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

//! Generated file from `EDBLNCMJOHJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EDBLNCMJOHJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EDBLNCMJOHJ {
    // message fields
    // @@protoc_insertion_point(field:EDBLNCMJOHJ.ACGEHFOBIKI)
    pub ACGEHFOBIKI: ::protobuf::MessageField<super::EPOGFCJNAAP::EPOGFCJNAAP>,
    // @@protoc_insertion_point(field:EDBLNCMJOHJ.JHFEDGIMCDG)
    pub JHFEDGIMCDG: u32,
    // @@protoc_insertion_point(field:EDBLNCMJOHJ.OHLNDHCKIOM)
    pub OHLNDHCKIOM: u32,
    // @@protoc_insertion_point(field:EDBLNCMJOHJ.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:EDBLNCMJOHJ.ICNFAMAFENL)
    pub ICNFAMAFENL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EDBLNCMJOHJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EDBLNCMJOHJ {
    fn default() -> &'a EDBLNCMJOHJ {
        <EDBLNCMJOHJ as ::protobuf::Message>::default_instance()
    }
}

impl EDBLNCMJOHJ {
    pub fn new() -> EDBLNCMJOHJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EPOGFCJNAAP::EPOGFCJNAAP>(
            "ACGEHFOBIKI",
            |m: &EDBLNCMJOHJ| { &m.ACGEHFOBIKI },
            |m: &mut EDBLNCMJOHJ| { &mut m.ACGEHFOBIKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JHFEDGIMCDG",
            |m: &EDBLNCMJOHJ| { &m.JHFEDGIMCDG },
            |m: &mut EDBLNCMJOHJ| { &mut m.JHFEDGIMCDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OHLNDHCKIOM",
            |m: &EDBLNCMJOHJ| { &m.OHLNDHCKIOM },
            |m: &mut EDBLNCMJOHJ| { &mut m.OHLNDHCKIOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &EDBLNCMJOHJ| { &m.avatar_id },
            |m: &mut EDBLNCMJOHJ| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICNFAMAFENL",
            |m: &EDBLNCMJOHJ| { &m.ICNFAMAFENL },
            |m: &mut EDBLNCMJOHJ| { &mut m.ICNFAMAFENL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EDBLNCMJOHJ>(
            "EDBLNCMJOHJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EDBLNCMJOHJ {
    const NAME: &'static str = "EDBLNCMJOHJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ACGEHFOBIKI)?;
                },
                120 => {
                    self.JHFEDGIMCDG = is.read_uint32()?;
                },
                88 => {
                    self.OHLNDHCKIOM = is.read_uint32()?;
                },
                56 => {
                    self.avatar_id = is.read_uint32()?;
                },
                72 => {
                    self.ICNFAMAFENL = is.read_uint32()?;
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
        if let Some(v) = self.ACGEHFOBIKI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.JHFEDGIMCDG != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.JHFEDGIMCDG);
        }
        if self.OHLNDHCKIOM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.OHLNDHCKIOM);
        }
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.avatar_id);
        }
        if self.ICNFAMAFENL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ICNFAMAFENL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ACGEHFOBIKI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.JHFEDGIMCDG != 0 {
            os.write_uint32(15, self.JHFEDGIMCDG)?;
        }
        if self.OHLNDHCKIOM != 0 {
            os.write_uint32(11, self.OHLNDHCKIOM)?;
        }
        if self.avatar_id != 0 {
            os.write_uint32(7, self.avatar_id)?;
        }
        if self.ICNFAMAFENL != 0 {
            os.write_uint32(9, self.ICNFAMAFENL)?;
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

    fn new() -> EDBLNCMJOHJ {
        EDBLNCMJOHJ::new()
    }

    fn clear(&mut self) {
        self.ACGEHFOBIKI.clear();
        self.JHFEDGIMCDG = 0;
        self.OHLNDHCKIOM = 0;
        self.avatar_id = 0;
        self.ICNFAMAFENL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EDBLNCMJOHJ {
        static instance: EDBLNCMJOHJ = EDBLNCMJOHJ {
            ACGEHFOBIKI: ::protobuf::MessageField::none(),
            JHFEDGIMCDG: 0,
            OHLNDHCKIOM: 0,
            avatar_id: 0,
            ICNFAMAFENL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EDBLNCMJOHJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EDBLNCMJOHJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EDBLNCMJOHJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EDBLNCMJOHJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EDBLNCMJOHJ.proto\x1a\x11EPOGFCJNAAP.proto\"\xc0\x01\n\x0bEDBLNCMJ\
    OHJ\x12.\n\x0bACGEHFOBIKI\x18\x02\x20\x01(\x0b2\x0c.EPOGFCJNAAPR\x0bACGE\
    HFOBIKI\x12\x20\n\x0bJHFEDGIMCDG\x18\x0f\x20\x01(\rR\x0bJHFEDGIMCDG\x12\
    \x20\n\x0bOHLNDHCKIOM\x18\x0b\x20\x01(\rR\x0bOHLNDHCKIOM\x12\x1b\n\tavat\
    ar_id\x18\x07\x20\x01(\rR\x08avatarId\x12\x20\n\x0bICNFAMAFENL\x18\t\x20\
    \x01(\rR\x0bICNFAMAFENLb\x06proto3\
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
            deps.push(super::EPOGFCJNAAP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EDBLNCMJOHJ::generated_message_descriptor_data());
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