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

//! Generated file from `JAACJJAFINF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JAACJJAFINF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JAACJJAFINF {
    // message fields
    // @@protoc_insertion_point(field:JAACJJAFINF.level)
    pub level: u32,
    // @@protoc_insertion_point(field:JAACJJAFINF.NJLDIPKCLBP)
    pub NJLDIPKCLBP: u32,
    // @@protoc_insertion_point(field:JAACJJAFINF.AJFGJKFAJEK)
    pub AJFGJKFAJEK: ::std::vec::Vec<super::ENHCPFKCFFM::ENHCPFKCFFM>,
    // @@protoc_insertion_point(field:JAACJJAFINF.LPOGAOGNHIN)
    pub LPOGAOGNHIN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JAACJJAFINF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JAACJJAFINF {
    fn default() -> &'a JAACJJAFINF {
        <JAACJJAFINF as ::protobuf::Message>::default_instance()
    }
}

impl JAACJJAFINF {
    pub fn new() -> JAACJJAFINF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &JAACJJAFINF| { &m.level },
            |m: &mut JAACJJAFINF| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NJLDIPKCLBP",
            |m: &JAACJJAFINF| { &m.NJLDIPKCLBP },
            |m: &mut JAACJJAFINF| { &mut m.NJLDIPKCLBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AJFGJKFAJEK",
            |m: &JAACJJAFINF| { &m.AJFGJKFAJEK },
            |m: &mut JAACJJAFINF| { &mut m.AJFGJKFAJEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LPOGAOGNHIN",
            |m: &JAACJJAFINF| { &m.LPOGAOGNHIN },
            |m: &mut JAACJJAFINF| { &mut m.LPOGAOGNHIN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JAACJJAFINF>(
            "JAACJJAFINF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JAACJJAFINF {
    const NAME: &'static str = "JAACJJAFINF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.level = is.read_uint32()?;
                },
                40 => {
                    self.NJLDIPKCLBP = is.read_uint32()?;
                },
                34 => {
                    self.AJFGJKFAJEK.push(is.read_message()?);
                },
                80 => {
                    self.LPOGAOGNHIN = is.read_uint32()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.level);
        }
        if self.NJLDIPKCLBP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NJLDIPKCLBP);
        }
        for value in &self.AJFGJKFAJEK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LPOGAOGNHIN != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.LPOGAOGNHIN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(1, self.level)?;
        }
        if self.NJLDIPKCLBP != 0 {
            os.write_uint32(5, self.NJLDIPKCLBP)?;
        }
        for v in &self.AJFGJKFAJEK {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.LPOGAOGNHIN != 0 {
            os.write_uint32(10, self.LPOGAOGNHIN)?;
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

    fn new() -> JAACJJAFINF {
        JAACJJAFINF::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.NJLDIPKCLBP = 0;
        self.AJFGJKFAJEK.clear();
        self.LPOGAOGNHIN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JAACJJAFINF {
        static instance: JAACJJAFINF = JAACJJAFINF {
            level: 0,
            NJLDIPKCLBP: 0,
            AJFGJKFAJEK: ::std::vec::Vec::new(),
            LPOGAOGNHIN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JAACJJAFINF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JAACJJAFINF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JAACJJAFINF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JAACJJAFINF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JAACJJAFINF.proto\x1a\x11ENHCPFKCFFM.proto\"\x97\x01\n\x0bJAACJJAF\
    INF\x12\x14\n\x05level\x18\x01\x20\x01(\rR\x05level\x12\x20\n\x0bNJLDIPK\
    CLBP\x18\x05\x20\x01(\rR\x0bNJLDIPKCLBP\x12.\n\x0bAJFGJKFAJEK\x18\x04\
    \x20\x03(\x0b2\x0c.ENHCPFKCFFMR\x0bAJFGJKFAJEK\x12\x20\n\x0bLPOGAOGNHIN\
    \x18\n\x20\x01(\rR\x0bLPOGAOGNHINb\x06proto3\
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
            deps.push(super::ENHCPFKCFFM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JAACJJAFINF::generated_message_descriptor_data());
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