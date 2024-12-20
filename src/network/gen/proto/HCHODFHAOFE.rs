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

//! Generated file from `HCHODFHAOFE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HCHODFHAOFE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HCHODFHAOFE {
    // message fields
    // @@protoc_insertion_point(field:HCHODFHAOFE.HNNDNEPPCJB)
    pub HNNDNEPPCJB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HCHODFHAOFE.CMCOLNPPBGG)
    pub CMCOLNPPBGG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HCHODFHAOFE.IHKGMCMLOKL)
    pub IHKGMCMLOKL: u32,
    // @@protoc_insertion_point(field:HCHODFHAOFE.PLNEDDGNPOI)
    pub PLNEDDGNPOI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HCHODFHAOFE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HCHODFHAOFE {
    fn default() -> &'a HCHODFHAOFE {
        <HCHODFHAOFE as ::protobuf::Message>::default_instance()
    }
}

impl HCHODFHAOFE {
    pub fn new() -> HCHODFHAOFE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HNNDNEPPCJB",
            |m: &HCHODFHAOFE| { &m.HNNDNEPPCJB },
            |m: &mut HCHODFHAOFE| { &mut m.HNNDNEPPCJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CMCOLNPPBGG",
            |m: &HCHODFHAOFE| { &m.CMCOLNPPBGG },
            |m: &mut HCHODFHAOFE| { &mut m.CMCOLNPPBGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHKGMCMLOKL",
            |m: &HCHODFHAOFE| { &m.IHKGMCMLOKL },
            |m: &mut HCHODFHAOFE| { &mut m.IHKGMCMLOKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLNEDDGNPOI",
            |m: &HCHODFHAOFE| { &m.PLNEDDGNPOI },
            |m: &mut HCHODFHAOFE| { &mut m.PLNEDDGNPOI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HCHODFHAOFE>(
            "HCHODFHAOFE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HCHODFHAOFE {
    const NAME: &'static str = "HCHODFHAOFE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.HNNDNEPPCJB)?;
                },
                16 => {
                    self.HNNDNEPPCJB.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.CMCOLNPPBGG)?;
                },
                72 => {
                    self.CMCOLNPPBGG.push(is.read_uint32()?);
                },
                112 => {
                    self.IHKGMCMLOKL = is.read_uint32()?;
                },
                120 => {
                    self.PLNEDDGNPOI = is.read_uint32()?;
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
        for value in &self.HNNDNEPPCJB {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        for value in &self.CMCOLNPPBGG {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if self.IHKGMCMLOKL != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IHKGMCMLOKL);
        }
        if self.PLNEDDGNPOI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.PLNEDDGNPOI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.HNNDNEPPCJB {
            os.write_uint32(2, *v)?;
        };
        for v in &self.CMCOLNPPBGG {
            os.write_uint32(9, *v)?;
        };
        if self.IHKGMCMLOKL != 0 {
            os.write_uint32(14, self.IHKGMCMLOKL)?;
        }
        if self.PLNEDDGNPOI != 0 {
            os.write_uint32(15, self.PLNEDDGNPOI)?;
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

    fn new() -> HCHODFHAOFE {
        HCHODFHAOFE::new()
    }

    fn clear(&mut self) {
        self.HNNDNEPPCJB.clear();
        self.CMCOLNPPBGG.clear();
        self.IHKGMCMLOKL = 0;
        self.PLNEDDGNPOI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HCHODFHAOFE {
        static instance: HCHODFHAOFE = HCHODFHAOFE {
            HNNDNEPPCJB: ::std::vec::Vec::new(),
            CMCOLNPPBGG: ::std::vec::Vec::new(),
            IHKGMCMLOKL: 0,
            PLNEDDGNPOI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HCHODFHAOFE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HCHODFHAOFE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HCHODFHAOFE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HCHODFHAOFE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HCHODFHAOFE.proto\"\x95\x01\n\x0bHCHODFHAOFE\x12\x20\n\x0bHNNDNEPP\
    CJB\x18\x02\x20\x03(\rR\x0bHNNDNEPPCJB\x12\x20\n\x0bCMCOLNPPBGG\x18\t\
    \x20\x03(\rR\x0bCMCOLNPPBGG\x12\x20\n\x0bIHKGMCMLOKL\x18\x0e\x20\x01(\rR\
    \x0bIHKGMCMLOKL\x12\x20\n\x0bPLNEDDGNPOI\x18\x0f\x20\x01(\rR\x0bPLNEDDGN\
    POIb\x06proto3\
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
            messages.push(HCHODFHAOFE::generated_message_descriptor_data());
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
