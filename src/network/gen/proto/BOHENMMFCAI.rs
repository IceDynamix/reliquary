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

//! Generated file from `BOHENMMFCAI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BOHENMMFCAI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BOHENMMFCAI {
    // message fields
    // @@protoc_insertion_point(field:BOHENMMFCAI.OOJLEJJAIID)
    pub OOJLEJJAIID: bool,
    // @@protoc_insertion_point(field:BOHENMMFCAI.DFJBLLNODJJ)
    pub DFJBLLNODJJ: u32,
    // @@protoc_insertion_point(field:BOHENMMFCAI.ANPAFOEPAHF)
    pub ANPAFOEPAHF: bool,
    // @@protoc_insertion_point(field:BOHENMMFCAI.EIMEAJLBGFP)
    pub EIMEAJLBGFP: ::std::string::String,
    // @@protoc_insertion_point(field:BOHENMMFCAI.CPGHFDDHDAH)
    pub CPGHFDDHDAH: bool,
    // @@protoc_insertion_point(field:BOHENMMFCAI.BPLAAPFGNBI)
    pub BPLAAPFGNBI: bool,
    // @@protoc_insertion_point(field:BOHENMMFCAI.DEEFBBOICLJ)
    pub DEEFBBOICLJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BOHENMMFCAI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BOHENMMFCAI {
    fn default() -> &'a BOHENMMFCAI {
        <BOHENMMFCAI as ::protobuf::Message>::default_instance()
    }
}

impl BOHENMMFCAI {
    pub fn new() -> BOHENMMFCAI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OOJLEJJAIID",
            |m: &BOHENMMFCAI| { &m.OOJLEJJAIID },
            |m: &mut BOHENMMFCAI| { &mut m.OOJLEJJAIID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DFJBLLNODJJ",
            |m: &BOHENMMFCAI| { &m.DFJBLLNODJJ },
            |m: &mut BOHENMMFCAI| { &mut m.DFJBLLNODJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANPAFOEPAHF",
            |m: &BOHENMMFCAI| { &m.ANPAFOEPAHF },
            |m: &mut BOHENMMFCAI| { &mut m.ANPAFOEPAHF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EIMEAJLBGFP",
            |m: &BOHENMMFCAI| { &m.EIMEAJLBGFP },
            |m: &mut BOHENMMFCAI| { &mut m.EIMEAJLBGFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPGHFDDHDAH",
            |m: &BOHENMMFCAI| { &m.CPGHFDDHDAH },
            |m: &mut BOHENMMFCAI| { &mut m.CPGHFDDHDAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPLAAPFGNBI",
            |m: &BOHENMMFCAI| { &m.BPLAAPFGNBI },
            |m: &mut BOHENMMFCAI| { &mut m.BPLAAPFGNBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEEFBBOICLJ",
            |m: &BOHENMMFCAI| { &m.DEEFBBOICLJ },
            |m: &mut BOHENMMFCAI| { &mut m.DEEFBBOICLJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BOHENMMFCAI>(
            "BOHENMMFCAI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BOHENMMFCAI {
    const NAME: &'static str = "BOHENMMFCAI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.OOJLEJJAIID = is.read_bool()?;
                },
                16 => {
                    self.DFJBLLNODJJ = is.read_uint32()?;
                },
                24 => {
                    self.ANPAFOEPAHF = is.read_bool()?;
                },
                34 => {
                    self.EIMEAJLBGFP = is.read_string()?;
                },
                40 => {
                    self.CPGHFDDHDAH = is.read_bool()?;
                },
                48 => {
                    self.BPLAAPFGNBI = is.read_bool()?;
                },
                56 => {
                    self.DEEFBBOICLJ = is.read_uint32()?;
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
        if self.OOJLEJJAIID != false {
            my_size += 1 + 1;
        }
        if self.DFJBLLNODJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.DFJBLLNODJJ);
        }
        if self.ANPAFOEPAHF != false {
            my_size += 1 + 1;
        }
        if !self.EIMEAJLBGFP.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.EIMEAJLBGFP);
        }
        if self.CPGHFDDHDAH != false {
            my_size += 1 + 1;
        }
        if self.BPLAAPFGNBI != false {
            my_size += 1 + 1;
        }
        if self.DEEFBBOICLJ != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DEEFBBOICLJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OOJLEJJAIID != false {
            os.write_bool(1, self.OOJLEJJAIID)?;
        }
        if self.DFJBLLNODJJ != 0 {
            os.write_uint32(2, self.DFJBLLNODJJ)?;
        }
        if self.ANPAFOEPAHF != false {
            os.write_bool(3, self.ANPAFOEPAHF)?;
        }
        if !self.EIMEAJLBGFP.is_empty() {
            os.write_string(4, &self.EIMEAJLBGFP)?;
        }
        if self.CPGHFDDHDAH != false {
            os.write_bool(5, self.CPGHFDDHDAH)?;
        }
        if self.BPLAAPFGNBI != false {
            os.write_bool(6, self.BPLAAPFGNBI)?;
        }
        if self.DEEFBBOICLJ != 0 {
            os.write_uint32(7, self.DEEFBBOICLJ)?;
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

    fn new() -> BOHENMMFCAI {
        BOHENMMFCAI::new()
    }

    fn clear(&mut self) {
        self.OOJLEJJAIID = false;
        self.DFJBLLNODJJ = 0;
        self.ANPAFOEPAHF = false;
        self.EIMEAJLBGFP.clear();
        self.CPGHFDDHDAH = false;
        self.BPLAAPFGNBI = false;
        self.DEEFBBOICLJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BOHENMMFCAI {
        static instance: BOHENMMFCAI = BOHENMMFCAI {
            OOJLEJJAIID: false,
            DFJBLLNODJJ: 0,
            ANPAFOEPAHF: false,
            EIMEAJLBGFP: ::std::string::String::new(),
            CPGHFDDHDAH: false,
            BPLAAPFGNBI: false,
            DEEFBBOICLJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BOHENMMFCAI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BOHENMMFCAI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BOHENMMFCAI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BOHENMMFCAI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BOHENMMFCAI.proto\"\xfb\x01\n\x0bBOHENMMFCAI\x12\x20\n\x0bOOJLEJJA\
    IID\x18\x01\x20\x01(\x08R\x0bOOJLEJJAIID\x12\x20\n\x0bDFJBLLNODJJ\x18\
    \x02\x20\x01(\rR\x0bDFJBLLNODJJ\x12\x20\n\x0bANPAFOEPAHF\x18\x03\x20\x01\
    (\x08R\x0bANPAFOEPAHF\x12\x20\n\x0bEIMEAJLBGFP\x18\x04\x20\x01(\tR\x0bEI\
    MEAJLBGFP\x12\x20\n\x0bCPGHFDDHDAH\x18\x05\x20\x01(\x08R\x0bCPGHFDDHDAH\
    \x12\x20\n\x0bBPLAAPFGNBI\x18\x06\x20\x01(\x08R\x0bBPLAAPFGNBI\x12\x20\n\
    \x0bDEEFBBOICLJ\x18\x07\x20\x01(\rR\x0bDEEFBBOICLJb\x06proto3\
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
            messages.push(BOHENMMFCAI::generated_message_descriptor_data());
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