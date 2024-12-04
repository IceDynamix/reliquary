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

//! Generated file from `CNHNJPOMCLD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CNHNJPOMCLD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CNHNJPOMCLD {
    // message fields
    // @@protoc_insertion_point(field:CNHNJPOMCLD.IDJNPOONENE)
    pub IDJNPOONENE: u32,
    // @@protoc_insertion_point(field:CNHNJPOMCLD.ELPMNKHEPKJ)
    pub ELPMNKHEPKJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:CNHNJPOMCLD.OFAGGKBMPJN)
    pub OFAGGKBMPJN: u32,
    // @@protoc_insertion_point(field:CNHNJPOMCLD.HAJEJEFLKJD)
    pub HAJEJEFLKJD: ::protobuf::MessageField<super::ItemList::ItemList>,
    // special fields
    // @@protoc_insertion_point(special_field:CNHNJPOMCLD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CNHNJPOMCLD {
    fn default() -> &'a CNHNJPOMCLD {
        <CNHNJPOMCLD as ::protobuf::Message>::default_instance()
    }
}

impl CNHNJPOMCLD {
    pub fn new() -> CNHNJPOMCLD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDJNPOONENE",
            |m: &CNHNJPOMCLD| { &m.IDJNPOONENE },
            |m: &mut CNHNJPOMCLD| { &mut m.IDJNPOONENE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "ELPMNKHEPKJ",
            |m: &CNHNJPOMCLD| { &m.ELPMNKHEPKJ },
            |m: &mut CNHNJPOMCLD| { &mut m.ELPMNKHEPKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFAGGKBMPJN",
            |m: &CNHNJPOMCLD| { &m.OFAGGKBMPJN },
            |m: &mut CNHNJPOMCLD| { &mut m.OFAGGKBMPJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "HAJEJEFLKJD",
            |m: &CNHNJPOMCLD| { &m.HAJEJEFLKJD },
            |m: &mut CNHNJPOMCLD| { &mut m.HAJEJEFLKJD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CNHNJPOMCLD>(
            "CNHNJPOMCLD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CNHNJPOMCLD {
    const NAME: &'static str = "CNHNJPOMCLD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.IDJNPOONENE = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ELPMNKHEPKJ)?;
                },
                64 => {
                    self.OFAGGKBMPJN = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HAJEJEFLKJD)?;
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
        if self.IDJNPOONENE != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IDJNPOONENE);
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OFAGGKBMPJN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OFAGGKBMPJN);
        }
        if let Some(v) = self.HAJEJEFLKJD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IDJNPOONENE != 0 {
            os.write_uint32(4, self.IDJNPOONENE)?;
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.OFAGGKBMPJN != 0 {
            os.write_uint32(8, self.OFAGGKBMPJN)?;
        }
        if let Some(v) = self.HAJEJEFLKJD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> CNHNJPOMCLD {
        CNHNJPOMCLD::new()
    }

    fn clear(&mut self) {
        self.IDJNPOONENE = 0;
        self.ELPMNKHEPKJ.clear();
        self.OFAGGKBMPJN = 0;
        self.HAJEJEFLKJD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CNHNJPOMCLD {
        static instance: CNHNJPOMCLD = CNHNJPOMCLD {
            IDJNPOONENE: 0,
            ELPMNKHEPKJ: ::protobuf::MessageField::none(),
            OFAGGKBMPJN: 0,
            HAJEJEFLKJD: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CNHNJPOMCLD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CNHNJPOMCLD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CNHNJPOMCLD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNHNJPOMCLD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CNHNJPOMCLD.proto\x1a\x0eItemList.proto\"\xab\x01\n\x0bCNHNJPOMCLD\
    \x12\x20\n\x0bIDJNPOONENE\x18\x04\x20\x01(\rR\x0bIDJNPOONENE\x12+\n\x0bE\
    LPMNKHEPKJ\x18\x07\x20\x01(\x0b2\t.ItemListR\x0bELPMNKHEPKJ\x12\x20\n\
    \x0bOFAGGKBMPJN\x18\x08\x20\x01(\rR\x0bOFAGGKBMPJN\x12+\n\x0bHAJEJEFLKJD\
    \x18\x0b\x20\x01(\x0b2\t.ItemListR\x0bHAJEJEFLKJDb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CNHNJPOMCLD::generated_message_descriptor_data());
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