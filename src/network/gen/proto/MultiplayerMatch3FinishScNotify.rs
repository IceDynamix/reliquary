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

//! Generated file from `MultiplayerMatch3FinishScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MultiplayerMatch3FinishScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MultiplayerMatch3FinishScNotify {
    // message fields
    // @@protoc_insertion_point(field:MultiplayerMatch3FinishScNotify.HPMEBECHEEN)
    pub HPMEBECHEEN: u32,
    // @@protoc_insertion_point(field:MultiplayerMatch3FinishScNotify.JOKEIGFCDOI)
    pub JOKEIGFCDOI: ::protobuf::MessageField<super::MLBKADJEBNA::MLBKADJEBNA>,
    // @@protoc_insertion_point(field:MultiplayerMatch3FinishScNotify.GIBBAHJJDIJ)
    pub GIBBAHJJDIJ: u32,
    // @@protoc_insertion_point(field:MultiplayerMatch3FinishScNotify.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::JPEDGMMIIFJ::JPEDGMMIIFJ>,
    // special fields
    // @@protoc_insertion_point(special_field:MultiplayerMatch3FinishScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MultiplayerMatch3FinishScNotify {
    fn default() -> &'a MultiplayerMatch3FinishScNotify {
        <MultiplayerMatch3FinishScNotify as ::protobuf::Message>::default_instance()
    }
}

impl MultiplayerMatch3FinishScNotify {
    pub fn new() -> MultiplayerMatch3FinishScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPMEBECHEEN",
            |m: &MultiplayerMatch3FinishScNotify| { &m.HPMEBECHEEN },
            |m: &mut MultiplayerMatch3FinishScNotify| { &mut m.HPMEBECHEEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MLBKADJEBNA::MLBKADJEBNA>(
            "JOKEIGFCDOI",
            |m: &MultiplayerMatch3FinishScNotify| { &m.JOKEIGFCDOI },
            |m: &mut MultiplayerMatch3FinishScNotify| { &mut m.JOKEIGFCDOI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIBBAHJJDIJ",
            |m: &MultiplayerMatch3FinishScNotify| { &m.GIBBAHJJDIJ },
            |m: &mut MultiplayerMatch3FinishScNotify| { &mut m.GIBBAHJJDIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &MultiplayerMatch3FinishScNotify| { &m.KGGHLADEKGP },
            |m: &mut MultiplayerMatch3FinishScNotify| { &mut m.KGGHLADEKGP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MultiplayerMatch3FinishScNotify>(
            "MultiplayerMatch3FinishScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MultiplayerMatch3FinishScNotify {
    const NAME: &'static str = "MultiplayerMatch3FinishScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.HPMEBECHEEN = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JOKEIGFCDOI)?;
                },
                16 => {
                    self.GIBBAHJJDIJ = is.read_uint32()?;
                },
                48 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
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
        if self.HPMEBECHEEN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HPMEBECHEEN);
        }
        if let Some(v) = self.JOKEIGFCDOI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.GIBBAHJJDIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.GIBBAHJJDIJ);
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::JPEDGMMIIFJ::JPEDGMMIIFJ::MATCH3_FINISH_REASON_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(6, self.KGGHLADEKGP.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HPMEBECHEEN != 0 {
            os.write_uint32(11, self.HPMEBECHEEN)?;
        }
        if let Some(v) = self.JOKEIGFCDOI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.GIBBAHJJDIJ != 0 {
            os.write_uint32(2, self.GIBBAHJJDIJ)?;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::JPEDGMMIIFJ::JPEDGMMIIFJ::MATCH3_FINISH_REASON_DEFAULT) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
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

    fn new() -> MultiplayerMatch3FinishScNotify {
        MultiplayerMatch3FinishScNotify::new()
    }

    fn clear(&mut self) {
        self.HPMEBECHEEN = 0;
        self.JOKEIGFCDOI.clear();
        self.GIBBAHJJDIJ = 0;
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::JPEDGMMIIFJ::JPEDGMMIIFJ::MATCH3_FINISH_REASON_DEFAULT);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MultiplayerMatch3FinishScNotify {
        static instance: MultiplayerMatch3FinishScNotify = MultiplayerMatch3FinishScNotify {
            HPMEBECHEEN: 0,
            JOKEIGFCDOI: ::protobuf::MessageField::none(),
            GIBBAHJJDIJ: 0,
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MultiplayerMatch3FinishScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MultiplayerMatch3FinishScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MultiplayerMatch3FinishScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiplayerMatch3FinishScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%MultiplayerMatch3FinishScNotify.proto\x1a\x11JPEDGMMIIFJ.proto\x1a\
    \x11MLBKADJEBNA.proto\"\xc5\x01\n\x1fMultiplayerMatch3FinishScNotify\x12\
    \x20\n\x0bHPMEBECHEEN\x18\x0b\x20\x01(\rR\x0bHPMEBECHEEN\x12.\n\x0bJOKEI\
    GFCDOI\x18\x03\x20\x01(\x0b2\x0c.MLBKADJEBNAR\x0bJOKEIGFCDOI\x12\x20\n\
    \x0bGIBBAHJJDIJ\x18\x02\x20\x01(\rR\x0bGIBBAHJJDIJ\x12.\n\x0bKGGHLADEKGP\
    \x18\x06\x20\x01(\x0e2\x0c.JPEDGMMIIFJR\x0bKGGHLADEKGPb\x06proto3\
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
            deps.push(super::JPEDGMMIIFJ::file_descriptor().clone());
            deps.push(super::MLBKADJEBNA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MultiplayerMatch3FinishScNotify::generated_message_descriptor_data());
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
