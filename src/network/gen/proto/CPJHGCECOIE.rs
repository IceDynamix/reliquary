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

//! Generated file from `CPJHGCECOIE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CPJHGCECOIE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CPJHGCECOIE {
    // message fields
    // @@protoc_insertion_point(field:CPJHGCECOIE.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:CPJHGCECOIE.unique_id)
    pub unique_id: u32,
    // @@protoc_insertion_point(field:CPJHGCECOIE.PMJIBDKMDMH)
    pub PMJIBDKMDMH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:CPJHGCECOIE.PDJOPKEMAGF)
    pub PDJOPKEMAGF: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:CPJHGCECOIE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CPJHGCECOIE {
    fn default() -> &'a CPJHGCECOIE {
        <CPJHGCECOIE as ::protobuf::Message>::default_instance()
    }
}

impl CPJHGCECOIE {
    pub fn new() -> CPJHGCECOIE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &CPJHGCECOIE| { &m.avatar_id },
            |m: &mut CPJHGCECOIE| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &CPJHGCECOIE| { &m.unique_id },
            |m: &mut CPJHGCECOIE| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PMJIBDKMDMH",
            |m: &CPJHGCECOIE| { &m.PMJIBDKMDMH },
            |m: &mut CPJHGCECOIE| { &mut m.PMJIBDKMDMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PDJOPKEMAGF",
            |m: &CPJHGCECOIE| { &m.PDJOPKEMAGF },
            |m: &mut CPJHGCECOIE| { &mut m.PDJOPKEMAGF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CPJHGCECOIE>(
            "CPJHGCECOIE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CPJHGCECOIE {
    const NAME: &'static str = "CPJHGCECOIE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.avatar_id = is.read_uint32()?;
                },
                112 => {
                    self.unique_id = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.PMJIBDKMDMH)?;
                },
                16 => {
                    self.PMJIBDKMDMH.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.PDJOPKEMAGF)?;
                },
                72 => {
                    self.PDJOPKEMAGF.push(is.read_uint32()?);
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.avatar_id);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.unique_id);
        }
        for value in &self.PMJIBDKMDMH {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        for value in &self.PDJOPKEMAGF {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(5, self.avatar_id)?;
        }
        if self.unique_id != 0 {
            os.write_uint32(14, self.unique_id)?;
        }
        for v in &self.PMJIBDKMDMH {
            os.write_uint32(2, *v)?;
        };
        for v in &self.PDJOPKEMAGF {
            os.write_uint32(9, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CPJHGCECOIE {
        CPJHGCECOIE::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.unique_id = 0;
        self.PMJIBDKMDMH.clear();
        self.PDJOPKEMAGF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CPJHGCECOIE {
        static instance: CPJHGCECOIE = CPJHGCECOIE {
            avatar_id: 0,
            unique_id: 0,
            PMJIBDKMDMH: ::std::vec::Vec::new(),
            PDJOPKEMAGF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CPJHGCECOIE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CPJHGCECOIE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CPJHGCECOIE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPJHGCECOIE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CPJHGCECOIE.proto\"\x8b\x01\n\x0bCPJHGCECOIE\x12\x1b\n\tavatar_id\
    \x18\x05\x20\x01(\rR\x08avatarId\x12\x1b\n\tunique_id\x18\x0e\x20\x01(\r\
    R\x08uniqueId\x12\x20\n\x0bPMJIBDKMDMH\x18\x02\x20\x03(\rR\x0bPMJIBDKMDM\
    H\x12\x20\n\x0bPDJOPKEMAGF\x18\t\x20\x03(\rR\x0bPDJOPKEMAGFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CPJHGCECOIE::generated_message_descriptor_data());
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