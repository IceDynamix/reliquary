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

//! Generated file from `OKKAKMHNPLK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OKKAKMHNPLK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OKKAKMHNPLK {
    // message fields
    // @@protoc_insertion_point(field:OKKAKMHNPLK.MLALNFGMGKE)
    pub MLALNFGMGKE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OKKAKMHNPLK.OBDHNNFMJJG)
    pub OBDHNNFMJJG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OKKAKMHNPLK.OODGDNEHNMA)
    pub OODGDNEHNMA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OKKAKMHNPLK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OKKAKMHNPLK {
    fn default() -> &'a OKKAKMHNPLK {
        <OKKAKMHNPLK as ::protobuf::Message>::default_instance()
    }
}

impl OKKAKMHNPLK {
    pub fn new() -> OKKAKMHNPLK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MLALNFGMGKE",
            |m: &OKKAKMHNPLK| { &m.MLALNFGMGKE },
            |m: &mut OKKAKMHNPLK| { &mut m.MLALNFGMGKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OBDHNNFMJJG",
            |m: &OKKAKMHNPLK| { &m.OBDHNNFMJJG },
            |m: &mut OKKAKMHNPLK| { &mut m.OBDHNNFMJJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OODGDNEHNMA",
            |m: &OKKAKMHNPLK| { &m.OODGDNEHNMA },
            |m: &mut OKKAKMHNPLK| { &mut m.OODGDNEHNMA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OKKAKMHNPLK>(
            "OKKAKMHNPLK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OKKAKMHNPLK {
    const NAME: &'static str = "OKKAKMHNPLK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.MLALNFGMGKE)?;
                },
                40 => {
                    self.MLALNFGMGKE.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.OBDHNNFMJJG)?;
                },
                104 => {
                    self.OBDHNNFMJJG.push(is.read_uint32()?);
                },
                120 => {
                    self.OODGDNEHNMA = is.read_uint32()?;
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
        for value in &self.MLALNFGMGKE {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for value in &self.OBDHNNFMJJG {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.OODGDNEHNMA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OODGDNEHNMA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.MLALNFGMGKE {
            os.write_uint32(5, *v)?;
        };
        for v in &self.OBDHNNFMJJG {
            os.write_uint32(13, *v)?;
        };
        if self.OODGDNEHNMA != 0 {
            os.write_uint32(15, self.OODGDNEHNMA)?;
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

    fn new() -> OKKAKMHNPLK {
        OKKAKMHNPLK::new()
    }

    fn clear(&mut self) {
        self.MLALNFGMGKE.clear();
        self.OBDHNNFMJJG.clear();
        self.OODGDNEHNMA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OKKAKMHNPLK {
        static instance: OKKAKMHNPLK = OKKAKMHNPLK {
            MLALNFGMGKE: ::std::vec::Vec::new(),
            OBDHNNFMJJG: ::std::vec::Vec::new(),
            OODGDNEHNMA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OKKAKMHNPLK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OKKAKMHNPLK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OKKAKMHNPLK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OKKAKMHNPLK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OKKAKMHNPLK.proto\"s\n\x0bOKKAKMHNPLK\x12\x20\n\x0bMLALNFGMGKE\x18\
    \x05\x20\x03(\rR\x0bMLALNFGMGKE\x12\x20\n\x0bOBDHNNFMJJG\x18\r\x20\x03(\
    \rR\x0bOBDHNNFMJJG\x12\x20\n\x0bOODGDNEHNMA\x18\x0f\x20\x01(\rR\x0bOODGD\
    NEHNMAb\x06proto3\
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
            messages.push(OKKAKMHNPLK::generated_message_descriptor_data());
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