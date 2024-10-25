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

//! Generated file from `LLLCMNHMNCP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LLLCMNHMNCP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LLLCMNHMNCP {
    // message fields
    // @@protoc_insertion_point(field:LLLCMNHMNCP.slot)
    pub slot: u32,
    // @@protoc_insertion_point(field:LLLCMNHMNCP.BJHIGFJHEAI)
    pub BJHIGFJHEAI: u32,
    // @@protoc_insertion_point(field:LLLCMNHMNCP.PPOMOBIDLFB)
    pub PPOMOBIDLFB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LLLCMNHMNCP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LLLCMNHMNCP {
    fn default() -> &'a LLLCMNHMNCP {
        <LLLCMNHMNCP as ::protobuf::Message>::default_instance()
    }
}

impl LLLCMNHMNCP {
    pub fn new() -> LLLCMNHMNCP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &LLLCMNHMNCP| { &m.slot },
            |m: &mut LLLCMNHMNCP| { &mut m.slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJHIGFJHEAI",
            |m: &LLLCMNHMNCP| { &m.BJHIGFJHEAI },
            |m: &mut LLLCMNHMNCP| { &mut m.BJHIGFJHEAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPOMOBIDLFB",
            |m: &LLLCMNHMNCP| { &m.PPOMOBIDLFB },
            |m: &mut LLLCMNHMNCP| { &mut m.PPOMOBIDLFB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LLLCMNHMNCP>(
            "LLLCMNHMNCP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LLLCMNHMNCP {
    const NAME: &'static str = "LLLCMNHMNCP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.slot = is.read_uint32()?;
                },
                72 => {
                    self.BJHIGFJHEAI = is.read_uint32()?;
                },
                80 => {
                    self.PPOMOBIDLFB = is.read_uint32()?;
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
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.slot);
        }
        if self.BJHIGFJHEAI != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BJHIGFJHEAI);
        }
        if self.PPOMOBIDLFB != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.PPOMOBIDLFB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.slot != 0 {
            os.write_uint32(2, self.slot)?;
        }
        if self.BJHIGFJHEAI != 0 {
            os.write_uint32(9, self.BJHIGFJHEAI)?;
        }
        if self.PPOMOBIDLFB != 0 {
            os.write_uint32(10, self.PPOMOBIDLFB)?;
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

    fn new() -> LLLCMNHMNCP {
        LLLCMNHMNCP::new()
    }

    fn clear(&mut self) {
        self.slot = 0;
        self.BJHIGFJHEAI = 0;
        self.PPOMOBIDLFB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LLLCMNHMNCP {
        static instance: LLLCMNHMNCP = LLLCMNHMNCP {
            slot: 0,
            BJHIGFJHEAI: 0,
            PPOMOBIDLFB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LLLCMNHMNCP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LLLCMNHMNCP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LLLCMNHMNCP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LLLCMNHMNCP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LLLCMNHMNCP.proto\"e\n\x0bLLLCMNHMNCP\x12\x12\n\x04slot\x18\x02\
    \x20\x01(\rR\x04slot\x12\x20\n\x0bBJHIGFJHEAI\x18\t\x20\x01(\rR\x0bBJHIG\
    FJHEAI\x12\x20\n\x0bPPOMOBIDLFB\x18\n\x20\x01(\rR\x0bPPOMOBIDLFBb\x06pro\
    to3\
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
            messages.push(LLLCMNHMNCP::generated_message_descriptor_data());
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
