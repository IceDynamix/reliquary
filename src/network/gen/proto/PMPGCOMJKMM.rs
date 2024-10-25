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

//! Generated file from `PMPGCOMJKMM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PMPGCOMJKMM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PMPGCOMJKMM {
    // message fields
    // @@protoc_insertion_point(field:PMPGCOMJKMM.GEBIEMEFINN)
    pub GEBIEMEFINN: i32,
    // @@protoc_insertion_point(field:PMPGCOMJKMM.IANMNDOBEOF)
    pub IANMNDOBEOF: i32,
    // @@protoc_insertion_point(field:PMPGCOMJKMM.COHAKBPNCDG)
    pub COHAKBPNCDG: u32,
    // @@protoc_insertion_point(field:PMPGCOMJKMM.EJMJFLGFHJO)
    pub EJMJFLGFHJO: u32,
    // @@protoc_insertion_point(field:PMPGCOMJKMM.unique_id)
    pub unique_id: u64,
    // @@protoc_insertion_point(field:PMPGCOMJKMM.MIBHBOCHMCH)
    pub MIBHBOCHMCH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PMPGCOMJKMM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PMPGCOMJKMM {
    fn default() -> &'a PMPGCOMJKMM {
        <PMPGCOMJKMM as ::protobuf::Message>::default_instance()
    }
}

impl PMPGCOMJKMM {
    pub fn new() -> PMPGCOMJKMM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEBIEMEFINN",
            |m: &PMPGCOMJKMM| { &m.GEBIEMEFINN },
            |m: &mut PMPGCOMJKMM| { &mut m.GEBIEMEFINN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IANMNDOBEOF",
            |m: &PMPGCOMJKMM| { &m.IANMNDOBEOF },
            |m: &mut PMPGCOMJKMM| { &mut m.IANMNDOBEOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COHAKBPNCDG",
            |m: &PMPGCOMJKMM| { &m.COHAKBPNCDG },
            |m: &mut PMPGCOMJKMM| { &mut m.COHAKBPNCDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EJMJFLGFHJO",
            |m: &PMPGCOMJKMM| { &m.EJMJFLGFHJO },
            |m: &mut PMPGCOMJKMM| { &mut m.EJMJFLGFHJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &PMPGCOMJKMM| { &m.unique_id },
            |m: &mut PMPGCOMJKMM| { &mut m.unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MIBHBOCHMCH",
            |m: &PMPGCOMJKMM| { &m.MIBHBOCHMCH },
            |m: &mut PMPGCOMJKMM| { &mut m.MIBHBOCHMCH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PMPGCOMJKMM>(
            "PMPGCOMJKMM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PMPGCOMJKMM {
    const NAME: &'static str = "PMPGCOMJKMM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.GEBIEMEFINN = is.read_int32()?;
                },
                112 => {
                    self.IANMNDOBEOF = is.read_int32()?;
                },
                96 => {
                    self.COHAKBPNCDG = is.read_uint32()?;
                },
                16 => {
                    self.EJMJFLGFHJO = is.read_uint32()?;
                },
                88 => {
                    self.unique_id = is.read_uint64()?;
                },
                8 => {
                    self.MIBHBOCHMCH = is.read_uint32()?;
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
        if self.GEBIEMEFINN != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.GEBIEMEFINN);
        }
        if self.IANMNDOBEOF != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.IANMNDOBEOF);
        }
        if self.COHAKBPNCDG != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.COHAKBPNCDG);
        }
        if self.EJMJFLGFHJO != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.EJMJFLGFHJO);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint64_size(11, self.unique_id);
        }
        if self.MIBHBOCHMCH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MIBHBOCHMCH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GEBIEMEFINN != 0 {
            os.write_int32(5, self.GEBIEMEFINN)?;
        }
        if self.IANMNDOBEOF != 0 {
            os.write_int32(14, self.IANMNDOBEOF)?;
        }
        if self.COHAKBPNCDG != 0 {
            os.write_uint32(12, self.COHAKBPNCDG)?;
        }
        if self.EJMJFLGFHJO != 0 {
            os.write_uint32(2, self.EJMJFLGFHJO)?;
        }
        if self.unique_id != 0 {
            os.write_uint64(11, self.unique_id)?;
        }
        if self.MIBHBOCHMCH != 0 {
            os.write_uint32(1, self.MIBHBOCHMCH)?;
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

    fn new() -> PMPGCOMJKMM {
        PMPGCOMJKMM::new()
    }

    fn clear(&mut self) {
        self.GEBIEMEFINN = 0;
        self.IANMNDOBEOF = 0;
        self.COHAKBPNCDG = 0;
        self.EJMJFLGFHJO = 0;
        self.unique_id = 0;
        self.MIBHBOCHMCH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PMPGCOMJKMM {
        static instance: PMPGCOMJKMM = PMPGCOMJKMM {
            GEBIEMEFINN: 0,
            IANMNDOBEOF: 0,
            COHAKBPNCDG: 0,
            EJMJFLGFHJO: 0,
            unique_id: 0,
            MIBHBOCHMCH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PMPGCOMJKMM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PMPGCOMJKMM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PMPGCOMJKMM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PMPGCOMJKMM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PMPGCOMJKMM.proto\"\xd4\x01\n\x0bPMPGCOMJKMM\x12\x20\n\x0bGEBIEMEF\
    INN\x18\x05\x20\x01(\x05R\x0bGEBIEMEFINN\x12\x20\n\x0bIANMNDOBEOF\x18\
    \x0e\x20\x01(\x05R\x0bIANMNDOBEOF\x12\x20\n\x0bCOHAKBPNCDG\x18\x0c\x20\
    \x01(\rR\x0bCOHAKBPNCDG\x12\x20\n\x0bEJMJFLGFHJO\x18\x02\x20\x01(\rR\x0b\
    EJMJFLGFHJO\x12\x1b\n\tunique_id\x18\x0b\x20\x01(\x04R\x08uniqueId\x12\
    \x20\n\x0bMIBHBOCHMCH\x18\x01\x20\x01(\rR\x0bMIBHBOCHMCHb\x06proto3\
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
            messages.push(PMPGCOMJKMM::generated_message_descriptor_data());
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
