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

//! Generated file from `PIHEPMKCGCI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PIHEPMKCGCI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PIHEPMKCGCI {
    // message fields
    // @@protoc_insertion_point(field:PIHEPMKCGCI.GKHFKNJFKPG)
    pub GKHFKNJFKPG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PIHEPMKCGCI.CGGMDOHBEEA)
    pub CGGMDOHBEEA: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:PIHEPMKCGCI.COGPEHINCHA)
    pub COGPEHINCHA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PIHEPMKCGCI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PIHEPMKCGCI {
    fn default() -> &'a PIHEPMKCGCI {
        <PIHEPMKCGCI as ::protobuf::Message>::default_instance()
    }
}

impl PIHEPMKCGCI {
    pub fn new() -> PIHEPMKCGCI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GKHFKNJFKPG",
            |m: &PIHEPMKCGCI| { &m.GKHFKNJFKPG },
            |m: &mut PIHEPMKCGCI| { &mut m.GKHFKNJFKPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "CGGMDOHBEEA",
            |m: &PIHEPMKCGCI| { &m.CGGMDOHBEEA },
            |m: &mut PIHEPMKCGCI| { &mut m.CGGMDOHBEEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COGPEHINCHA",
            |m: &PIHEPMKCGCI| { &m.COGPEHINCHA },
            |m: &mut PIHEPMKCGCI| { &mut m.COGPEHINCHA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PIHEPMKCGCI>(
            "PIHEPMKCGCI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PIHEPMKCGCI {
    const NAME: &'static str = "PIHEPMKCGCI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.GKHFKNJFKPG)?;
                },
                88 => {
                    self.GKHFKNJFKPG.push(is.read_uint32()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CGGMDOHBEEA)?;
                },
                120 => {
                    self.COGPEHINCHA = is.read_uint32()?;
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
        for value in &self.GKHFKNJFKPG {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if let Some(v) = self.CGGMDOHBEEA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.COGPEHINCHA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.COGPEHINCHA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GKHFKNJFKPG {
            os.write_uint32(11, *v)?;
        };
        if let Some(v) = self.CGGMDOHBEEA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.COGPEHINCHA != 0 {
            os.write_uint32(15, self.COGPEHINCHA)?;
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

    fn new() -> PIHEPMKCGCI {
        PIHEPMKCGCI::new()
    }

    fn clear(&mut self) {
        self.GKHFKNJFKPG.clear();
        self.CGGMDOHBEEA.clear();
        self.COGPEHINCHA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PIHEPMKCGCI {
        static instance: PIHEPMKCGCI = PIHEPMKCGCI {
            GKHFKNJFKPG: ::std::vec::Vec::new(),
            CGGMDOHBEEA: ::protobuf::MessageField::none(),
            COGPEHINCHA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PIHEPMKCGCI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PIHEPMKCGCI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PIHEPMKCGCI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PIHEPMKCGCI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PIHEPMKCGCI.proto\x1a\x0eItemList.proto\"~\n\x0bPIHEPMKCGCI\x12\
    \x20\n\x0bGKHFKNJFKPG\x18\x0b\x20\x03(\rR\x0bGKHFKNJFKPG\x12+\n\x0bCGGMD\
    OHBEEA\x18\x05\x20\x01(\x0b2\t.ItemListR\x0bCGGMDOHBEEA\x12\x20\n\x0bCOG\
    PEHINCHA\x18\x0f\x20\x01(\rR\x0bCOGPEHINCHAb\x06proto3\
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
            messages.push(PIHEPMKCGCI::generated_message_descriptor_data());
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
