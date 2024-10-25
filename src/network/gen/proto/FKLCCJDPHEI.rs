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

//! Generated file from `FKLCCJDPHEI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FKLCCJDPHEI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FKLCCJDPHEI {
    // message fields
    // @@protoc_insertion_point(field:FKLCCJDPHEI.FCDGOMCIOKM)
    pub FCDGOMCIOKM: u32,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.MBMGMOJOAKN)
    pub MBMGMOJOAKN: f64,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.JMMHKNGFOCN)
    pub JMMHKNGFOCN: f64,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.BLEINMLIFDD)
    pub BLEINMLIFDD: u32,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.KKCHCLOLPMB)
    pub KKCHCLOLPMB: u32,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.PDPGDMPGOBM)
    pub PDPGDMPGOBM: f64,
    // @@protoc_insertion_point(field:FKLCCJDPHEI.CFPOKHPIDPL)
    pub CFPOKHPIDPL: f64,
    // special fields
    // @@protoc_insertion_point(special_field:FKLCCJDPHEI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FKLCCJDPHEI {
    fn default() -> &'a FKLCCJDPHEI {
        <FKLCCJDPHEI as ::protobuf::Message>::default_instance()
    }
}

impl FKLCCJDPHEI {
    pub fn new() -> FKLCCJDPHEI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCDGOMCIOKM",
            |m: &FKLCCJDPHEI| { &m.FCDGOMCIOKM },
            |m: &mut FKLCCJDPHEI| { &mut m.FCDGOMCIOKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MBMGMOJOAKN",
            |m: &FKLCCJDPHEI| { &m.MBMGMOJOAKN },
            |m: &mut FKLCCJDPHEI| { &mut m.MBMGMOJOAKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JMMHKNGFOCN",
            |m: &FKLCCJDPHEI| { &m.JMMHKNGFOCN },
            |m: &mut FKLCCJDPHEI| { &mut m.JMMHKNGFOCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLEINMLIFDD",
            |m: &FKLCCJDPHEI| { &m.BLEINMLIFDD },
            |m: &mut FKLCCJDPHEI| { &mut m.BLEINMLIFDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKCHCLOLPMB",
            |m: &FKLCCJDPHEI| { &m.KKCHCLOLPMB },
            |m: &mut FKLCCJDPHEI| { &mut m.KKCHCLOLPMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDPGDMPGOBM",
            |m: &FKLCCJDPHEI| { &m.PDPGDMPGOBM },
            |m: &mut FKLCCJDPHEI| { &mut m.PDPGDMPGOBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFPOKHPIDPL",
            |m: &FKLCCJDPHEI| { &m.CFPOKHPIDPL },
            |m: &mut FKLCCJDPHEI| { &mut m.CFPOKHPIDPL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FKLCCJDPHEI>(
            "FKLCCJDPHEI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FKLCCJDPHEI {
    const NAME: &'static str = "FKLCCJDPHEI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FCDGOMCIOKM = is.read_uint32()?;
                },
                17 => {
                    self.MBMGMOJOAKN = is.read_double()?;
                },
                25 => {
                    self.JMMHKNGFOCN = is.read_double()?;
                },
                32 => {
                    self.BLEINMLIFDD = is.read_uint32()?;
                },
                40 => {
                    self.KKCHCLOLPMB = is.read_uint32()?;
                },
                49 => {
                    self.PDPGDMPGOBM = is.read_double()?;
                },
                57 => {
                    self.CFPOKHPIDPL = is.read_double()?;
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
        if self.FCDGOMCIOKM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FCDGOMCIOKM);
        }
        if self.MBMGMOJOAKN != 0. {
            my_size += 1 + 8;
        }
        if self.JMMHKNGFOCN != 0. {
            my_size += 1 + 8;
        }
        if self.BLEINMLIFDD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BLEINMLIFDD);
        }
        if self.KKCHCLOLPMB != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KKCHCLOLPMB);
        }
        if self.PDPGDMPGOBM != 0. {
            my_size += 1 + 8;
        }
        if self.CFPOKHPIDPL != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FCDGOMCIOKM != 0 {
            os.write_uint32(1, self.FCDGOMCIOKM)?;
        }
        if self.MBMGMOJOAKN != 0. {
            os.write_double(2, self.MBMGMOJOAKN)?;
        }
        if self.JMMHKNGFOCN != 0. {
            os.write_double(3, self.JMMHKNGFOCN)?;
        }
        if self.BLEINMLIFDD != 0 {
            os.write_uint32(4, self.BLEINMLIFDD)?;
        }
        if self.KKCHCLOLPMB != 0 {
            os.write_uint32(5, self.KKCHCLOLPMB)?;
        }
        if self.PDPGDMPGOBM != 0. {
            os.write_double(6, self.PDPGDMPGOBM)?;
        }
        if self.CFPOKHPIDPL != 0. {
            os.write_double(7, self.CFPOKHPIDPL)?;
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

    fn new() -> FKLCCJDPHEI {
        FKLCCJDPHEI::new()
    }

    fn clear(&mut self) {
        self.FCDGOMCIOKM = 0;
        self.MBMGMOJOAKN = 0.;
        self.JMMHKNGFOCN = 0.;
        self.BLEINMLIFDD = 0;
        self.KKCHCLOLPMB = 0;
        self.PDPGDMPGOBM = 0.;
        self.CFPOKHPIDPL = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FKLCCJDPHEI {
        static instance: FKLCCJDPHEI = FKLCCJDPHEI {
            FCDGOMCIOKM: 0,
            MBMGMOJOAKN: 0.,
            JMMHKNGFOCN: 0.,
            BLEINMLIFDD: 0,
            KKCHCLOLPMB: 0,
            PDPGDMPGOBM: 0.,
            CFPOKHPIDPL: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FKLCCJDPHEI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FKLCCJDPHEI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FKLCCJDPHEI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FKLCCJDPHEI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FKLCCJDPHEI.proto\"\xfb\x01\n\x0bFKLCCJDPHEI\x12\x20\n\x0bFCDGOMCI\
    OKM\x18\x01\x20\x01(\rR\x0bFCDGOMCIOKM\x12\x20\n\x0bMBMGMOJOAKN\x18\x02\
    \x20\x01(\x01R\x0bMBMGMOJOAKN\x12\x20\n\x0bJMMHKNGFOCN\x18\x03\x20\x01(\
    \x01R\x0bJMMHKNGFOCN\x12\x20\n\x0bBLEINMLIFDD\x18\x04\x20\x01(\rR\x0bBLE\
    INMLIFDD\x12\x20\n\x0bKKCHCLOLPMB\x18\x05\x20\x01(\rR\x0bKKCHCLOLPMB\x12\
    \x20\n\x0bPDPGDMPGOBM\x18\x06\x20\x01(\x01R\x0bPDPGDMPGOBM\x12\x20\n\x0b\
    CFPOKHPIDPL\x18\x07\x20\x01(\x01R\x0bCFPOKHPIDPLb\x06proto3\
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
            messages.push(FKLCCJDPHEI::generated_message_descriptor_data());
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
