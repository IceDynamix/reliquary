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

//! Generated file from `LMHKCAHMNPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LMHKCAHMNPE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LMHKCAHMNPE {
    // message fields
    // @@protoc_insertion_point(field:LMHKCAHMNPE.JCEAHHGJMKK)
    pub JCEAHHGJMKK: u32,
    // @@protoc_insertion_point(field:LMHKCAHMNPE.OPDEJDOOHOP)
    pub OPDEJDOOHOP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LMHKCAHMNPE.HPJAHKFGFKB)
    pub HPJAHKFGFKB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LMHKCAHMNPE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LMHKCAHMNPE {
    fn default() -> &'a LMHKCAHMNPE {
        <LMHKCAHMNPE as ::protobuf::Message>::default_instance()
    }
}

impl LMHKCAHMNPE {
    pub fn new() -> LMHKCAHMNPE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCEAHHGJMKK",
            |m: &LMHKCAHMNPE| { &m.JCEAHHGJMKK },
            |m: &mut LMHKCAHMNPE| { &mut m.JCEAHHGJMKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OPDEJDOOHOP",
            |m: &LMHKCAHMNPE| { &m.OPDEJDOOHOP },
            |m: &mut LMHKCAHMNPE| { &mut m.OPDEJDOOHOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HPJAHKFGFKB",
            |m: &LMHKCAHMNPE| { &m.HPJAHKFGFKB },
            |m: &mut LMHKCAHMNPE| { &mut m.HPJAHKFGFKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LMHKCAHMNPE>(
            "LMHKCAHMNPE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LMHKCAHMNPE {
    const NAME: &'static str = "LMHKCAHMNPE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.JCEAHHGJMKK = is.read_uint32()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.OPDEJDOOHOP)?;
                },
                80 => {
                    self.OPDEJDOOHOP.push(is.read_uint32()?);
                },
                112 => {
                    self.HPJAHKFGFKB = is.read_uint32()?;
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
        if self.JCEAHHGJMKK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.JCEAHHGJMKK);
        }
        for value in &self.OPDEJDOOHOP {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.HPJAHKFGFKB != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HPJAHKFGFKB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JCEAHHGJMKK != 0 {
            os.write_uint32(2, self.JCEAHHGJMKK)?;
        }
        for v in &self.OPDEJDOOHOP {
            os.write_uint32(10, *v)?;
        };
        if self.HPJAHKFGFKB != 0 {
            os.write_uint32(14, self.HPJAHKFGFKB)?;
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

    fn new() -> LMHKCAHMNPE {
        LMHKCAHMNPE::new()
    }

    fn clear(&mut self) {
        self.JCEAHHGJMKK = 0;
        self.OPDEJDOOHOP.clear();
        self.HPJAHKFGFKB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LMHKCAHMNPE {
        static instance: LMHKCAHMNPE = LMHKCAHMNPE {
            JCEAHHGJMKK: 0,
            OPDEJDOOHOP: ::std::vec::Vec::new(),
            HPJAHKFGFKB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LMHKCAHMNPE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LMHKCAHMNPE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LMHKCAHMNPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LMHKCAHMNPE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LMHKCAHMNPE.proto\"s\n\x0bLMHKCAHMNPE\x12\x20\n\x0bJCEAHHGJMKK\x18\
    \x02\x20\x01(\rR\x0bJCEAHHGJMKK\x12\x20\n\x0bOPDEJDOOHOP\x18\n\x20\x03(\
    \rR\x0bOPDEJDOOHOP\x12\x20\n\x0bHPJAHKFGFKB\x18\x0e\x20\x01(\rR\x0bHPJAH\
    KFGFKBb\x06proto3\
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
            messages.push(LMHKCAHMNPE::generated_message_descriptor_data());
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
