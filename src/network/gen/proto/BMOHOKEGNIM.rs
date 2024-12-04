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

//! Generated file from `BMOHOKEGNIM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BMOHOKEGNIM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BMOHOKEGNIM {
    // message fields
    // @@protoc_insertion_point(field:BMOHOKEGNIM.DDJMHBBHHCN)
    pub DDJMHBBHHCN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMOHOKEGNIM.BOKJMIFIKOM)
    pub BOKJMIFIKOM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMOHOKEGNIM.AFAHFIDHLHF)
    pub AFAHFIDHLHF: u32,
    // @@protoc_insertion_point(field:BMOHOKEGNIM.BBNHNOFGIIP)
    pub BBNHNOFGIIP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMOHOKEGNIM.HMDDJONLHFL)
    pub HMDDJONLHFL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMOHOKEGNIM.ABAIJKEGGOI)
    pub ABAIJKEGGOI: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BMOHOKEGNIM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BMOHOKEGNIM {
    fn default() -> &'a BMOHOKEGNIM {
        <BMOHOKEGNIM as ::protobuf::Message>::default_instance()
    }
}

impl BMOHOKEGNIM {
    pub fn new() -> BMOHOKEGNIM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DDJMHBBHHCN",
            |m: &BMOHOKEGNIM| { &m.DDJMHBBHHCN },
            |m: &mut BMOHOKEGNIM| { &mut m.DDJMHBBHHCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BOKJMIFIKOM",
            |m: &BMOHOKEGNIM| { &m.BOKJMIFIKOM },
            |m: &mut BMOHOKEGNIM| { &mut m.BOKJMIFIKOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFAHFIDHLHF",
            |m: &BMOHOKEGNIM| { &m.AFAHFIDHLHF },
            |m: &mut BMOHOKEGNIM| { &mut m.AFAHFIDHLHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BBNHNOFGIIP",
            |m: &BMOHOKEGNIM| { &m.BBNHNOFGIIP },
            |m: &mut BMOHOKEGNIM| { &mut m.BBNHNOFGIIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HMDDJONLHFL",
            |m: &BMOHOKEGNIM| { &m.HMDDJONLHFL },
            |m: &mut BMOHOKEGNIM| { &mut m.HMDDJONLHFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ABAIJKEGGOI",
            |m: &BMOHOKEGNIM| { &m.ABAIJKEGGOI },
            |m: &mut BMOHOKEGNIM| { &mut m.ABAIJKEGGOI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BMOHOKEGNIM>(
            "BMOHOKEGNIM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BMOHOKEGNIM {
    const NAME: &'static str = "BMOHOKEGNIM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.DDJMHBBHHCN)?;
                },
                8 => {
                    self.DDJMHBBHHCN.push(is.read_uint32()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.BOKJMIFIKOM)?;
                },
                24 => {
                    self.BOKJMIFIKOM.push(is.read_uint32()?);
                },
                80 => {
                    self.AFAHFIDHLHF = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.BBNHNOFGIIP)?;
                },
                104 => {
                    self.BBNHNOFGIIP.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.HMDDJONLHFL)?;
                },
                56 => {
                    self.HMDDJONLHFL.push(is.read_uint32()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.ABAIJKEGGOI)?;
                },
                40 => {
                    self.ABAIJKEGGOI.push(is.read_uint32()?);
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
        for value in &self.DDJMHBBHHCN {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for value in &self.BOKJMIFIKOM {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.AFAHFIDHLHF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.AFAHFIDHLHF);
        }
        for value in &self.BBNHNOFGIIP {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        for value in &self.HMDDJONLHFL {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.ABAIJKEGGOI {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.DDJMHBBHHCN {
            os.write_uint32(1, *v)?;
        };
        for v in &self.BOKJMIFIKOM {
            os.write_uint32(3, *v)?;
        };
        if self.AFAHFIDHLHF != 0 {
            os.write_uint32(10, self.AFAHFIDHLHF)?;
        }
        for v in &self.BBNHNOFGIIP {
            os.write_uint32(13, *v)?;
        };
        for v in &self.HMDDJONLHFL {
            os.write_uint32(7, *v)?;
        };
        for v in &self.ABAIJKEGGOI {
            os.write_uint32(5, *v)?;
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

    fn new() -> BMOHOKEGNIM {
        BMOHOKEGNIM::new()
    }

    fn clear(&mut self) {
        self.DDJMHBBHHCN.clear();
        self.BOKJMIFIKOM.clear();
        self.AFAHFIDHLHF = 0;
        self.BBNHNOFGIIP.clear();
        self.HMDDJONLHFL.clear();
        self.ABAIJKEGGOI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BMOHOKEGNIM {
        static instance: BMOHOKEGNIM = BMOHOKEGNIM {
            DDJMHBBHHCN: ::std::vec::Vec::new(),
            BOKJMIFIKOM: ::std::vec::Vec::new(),
            AFAHFIDHLHF: 0,
            BBNHNOFGIIP: ::std::vec::Vec::new(),
            HMDDJONLHFL: ::std::vec::Vec::new(),
            ABAIJKEGGOI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BMOHOKEGNIM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BMOHOKEGNIM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BMOHOKEGNIM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BMOHOKEGNIM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BMOHOKEGNIM.proto\"\xd9\x01\n\x0bBMOHOKEGNIM\x12\x20\n\x0bDDJMHBBH\
    HCN\x18\x01\x20\x03(\rR\x0bDDJMHBBHHCN\x12\x20\n\x0bBOKJMIFIKOM\x18\x03\
    \x20\x03(\rR\x0bBOKJMIFIKOM\x12\x20\n\x0bAFAHFIDHLHF\x18\n\x20\x01(\rR\
    \x0bAFAHFIDHLHF\x12\x20\n\x0bBBNHNOFGIIP\x18\r\x20\x03(\rR\x0bBBNHNOFGII\
    P\x12\x20\n\x0bHMDDJONLHFL\x18\x07\x20\x03(\rR\x0bHMDDJONLHFL\x12\x20\n\
    \x0bABAIJKEGGOI\x18\x05\x20\x03(\rR\x0bABAIJKEGGOIb\x06proto3\
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
            messages.push(BMOHOKEGNIM::generated_message_descriptor_data());
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