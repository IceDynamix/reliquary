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

//! Generated file from `NIKINNLIPOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NIKINNLIPOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NIKINNLIPOF {
    // message fields
    // @@protoc_insertion_point(field:NIKINNLIPOF.LBLCAOFONOO)
    pub LBLCAOFONOO: u32,
    // @@protoc_insertion_point(field:NIKINNLIPOF.BBGFGGGNACF)
    pub BBGFGGGNACF: u32,
    // @@protoc_insertion_point(field:NIKINNLIPOF.LDLKIHGICIJ)
    pub LDLKIHGICIJ: ::std::vec::Vec<super::BAGNJPMDKJN::BAGNJPMDKJN>,
    // special fields
    // @@protoc_insertion_point(special_field:NIKINNLIPOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NIKINNLIPOF {
    fn default() -> &'a NIKINNLIPOF {
        <NIKINNLIPOF as ::protobuf::Message>::default_instance()
    }
}

impl NIKINNLIPOF {
    pub fn new() -> NIKINNLIPOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBLCAOFONOO",
            |m: &NIKINNLIPOF| { &m.LBLCAOFONOO },
            |m: &mut NIKINNLIPOF| { &mut m.LBLCAOFONOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBGFGGGNACF",
            |m: &NIKINNLIPOF| { &m.BBGFGGGNACF },
            |m: &mut NIKINNLIPOF| { &mut m.BBGFGGGNACF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LDLKIHGICIJ",
            |m: &NIKINNLIPOF| { &m.LDLKIHGICIJ },
            |m: &mut NIKINNLIPOF| { &mut m.LDLKIHGICIJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NIKINNLIPOF>(
            "NIKINNLIPOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NIKINNLIPOF {
    const NAME: &'static str = "NIKINNLIPOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.LBLCAOFONOO = is.read_uint32()?;
                },
                104 => {
                    self.BBGFGGGNACF = is.read_uint32()?;
                },
                18 => {
                    self.LDLKIHGICIJ.push(is.read_message()?);
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
        if self.LBLCAOFONOO != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LBLCAOFONOO);
        }
        if self.BBGFGGGNACF != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.BBGFGGGNACF);
        }
        for value in &self.LDLKIHGICIJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LBLCAOFONOO != 0 {
            os.write_uint32(12, self.LBLCAOFONOO)?;
        }
        if self.BBGFGGGNACF != 0 {
            os.write_uint32(13, self.BBGFGGGNACF)?;
        }
        for v in &self.LDLKIHGICIJ {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> NIKINNLIPOF {
        NIKINNLIPOF::new()
    }

    fn clear(&mut self) {
        self.LBLCAOFONOO = 0;
        self.BBGFGGGNACF = 0;
        self.LDLKIHGICIJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NIKINNLIPOF {
        static instance: NIKINNLIPOF = NIKINNLIPOF {
            LBLCAOFONOO: 0,
            BBGFGGGNACF: 0,
            LDLKIHGICIJ: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NIKINNLIPOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NIKINNLIPOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NIKINNLIPOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NIKINNLIPOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NIKINNLIPOF.proto\x1a\x11BAGNJPMDKJN.proto\"\x81\x01\n\x0bNIKINNLI\
    POF\x12\x20\n\x0bLBLCAOFONOO\x18\x0c\x20\x01(\rR\x0bLBLCAOFONOO\x12\x20\
    \n\x0bBBGFGGGNACF\x18\r\x20\x01(\rR\x0bBBGFGGGNACF\x12.\n\x0bLDLKIHGICIJ\
    \x18\x02\x20\x03(\x0b2\x0c.BAGNJPMDKJNR\x0bLDLKIHGICIJb\x06proto3\
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
            deps.push(super::BAGNJPMDKJN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NIKINNLIPOF::generated_message_descriptor_data());
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
