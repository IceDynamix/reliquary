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

//! Generated file from `MEOIFIOAECF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MEOIFIOAECF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MEOIFIOAECF {
    // message fields
    // @@protoc_insertion_point(field:MEOIFIOAECF.JJCCJJINLFL)
    pub JJCCJJINLFL: u32,
    // @@protoc_insertion_point(field:MEOIFIOAECF.locked)
    pub locked: bool,
    // @@protoc_insertion_point(field:MEOIFIOAECF.KACALGIOEDB)
    pub KACALGIOEDB: ::protobuf::MessageField<super::KPKKKJPJCPC::KPKKKJPJCPC>,
    // special fields
    // @@protoc_insertion_point(special_field:MEOIFIOAECF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MEOIFIOAECF {
    fn default() -> &'a MEOIFIOAECF {
        <MEOIFIOAECF as ::protobuf::Message>::default_instance()
    }
}

impl MEOIFIOAECF {
    pub fn new() -> MEOIFIOAECF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JJCCJJINLFL",
            |m: &MEOIFIOAECF| { &m.JJCCJJINLFL },
            |m: &mut MEOIFIOAECF| { &mut m.JJCCJJINLFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "locked",
            |m: &MEOIFIOAECF| { &m.locked },
            |m: &mut MEOIFIOAECF| { &mut m.locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KPKKKJPJCPC::KPKKKJPJCPC>(
            "KACALGIOEDB",
            |m: &MEOIFIOAECF| { &m.KACALGIOEDB },
            |m: &mut MEOIFIOAECF| { &mut m.KACALGIOEDB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MEOIFIOAECF>(
            "MEOIFIOAECF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MEOIFIOAECF {
    const NAME: &'static str = "MEOIFIOAECF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.JJCCJJINLFL = is.read_uint32()?;
                },
                16 => {
                    self.locked = is.read_bool()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KACALGIOEDB)?;
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
        if self.JJCCJJINLFL != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.JJCCJJINLFL);
        }
        if self.locked != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.KACALGIOEDB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JJCCJJINLFL != 0 {
            os.write_uint32(1, self.JJCCJJINLFL)?;
        }
        if self.locked != false {
            os.write_bool(2, self.locked)?;
        }
        if let Some(v) = self.KACALGIOEDB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> MEOIFIOAECF {
        MEOIFIOAECF::new()
    }

    fn clear(&mut self) {
        self.JJCCJJINLFL = 0;
        self.locked = false;
        self.KACALGIOEDB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MEOIFIOAECF {
        static instance: MEOIFIOAECF = MEOIFIOAECF {
            JJCCJJINLFL: 0,
            locked: false,
            KACALGIOEDB: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MEOIFIOAECF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MEOIFIOAECF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MEOIFIOAECF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MEOIFIOAECF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MEOIFIOAECF.proto\x1a\x11KPKKKJPJCPC.proto\"w\n\x0bMEOIFIOAECF\x12\
    \x20\n\x0bJJCCJJINLFL\x18\x01\x20\x01(\rR\x0bJJCCJJINLFL\x12\x16\n\x06lo\
    cked\x18\x02\x20\x01(\x08R\x06locked\x12.\n\x0bKACALGIOEDB\x18\x03\x20\
    \x01(\x0b2\x0c.KPKKKJPJCPCR\x0bKACALGIOEDBb\x06proto3\
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
            deps.push(super::KPKKKJPJCPC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MEOIFIOAECF::generated_message_descriptor_data());
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
