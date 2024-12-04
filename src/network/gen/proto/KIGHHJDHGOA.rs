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

//! Generated file from `KIGHHJDHGOA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KIGHHJDHGOA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KIGHHJDHGOA {
    // message fields
    // @@protoc_insertion_point(field:KIGHHJDHGOA.IPNHCCODNDI)
    pub IPNHCCODNDI: u32,
    // @@protoc_insertion_point(field:KIGHHJDHGOA.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:KIGHHJDHGOA.GGFFPIJILIB)
    pub GGFFPIJILIB: u32,
    // @@protoc_insertion_point(field:KIGHHJDHGOA.GBFOEDCEFMI)
    pub GBFOEDCEFMI: u32,
    // @@protoc_insertion_point(field:KIGHHJDHGOA.DLGPDENIBFH)
    pub DLGPDENIBFH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KIGHHJDHGOA.FGBPHGGFBEG)
    pub FGBPHGGFBEG: ::std::collections::HashMap<::std::string::String, f32>,
    // special fields
    // @@protoc_insertion_point(special_field:KIGHHJDHGOA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KIGHHJDHGOA {
    fn default() -> &'a KIGHHJDHGOA {
        <KIGHHJDHGOA as ::protobuf::Message>::default_instance()
    }
}

impl KIGHHJDHGOA {
    pub fn new() -> KIGHHJDHGOA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPNHCCODNDI",
            |m: &KIGHHJDHGOA| { &m.IPNHCCODNDI },
            |m: &mut KIGHHJDHGOA| { &mut m.IPNHCCODNDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &KIGHHJDHGOA| { &m.JKOCJIMAGBN },
            |m: &mut KIGHHJDHGOA| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGFFPIJILIB",
            |m: &KIGHHJDHGOA| { &m.GGFFPIJILIB },
            |m: &mut KIGHHJDHGOA| { &mut m.GGFFPIJILIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBFOEDCEFMI",
            |m: &KIGHHJDHGOA| { &m.GBFOEDCEFMI },
            |m: &mut KIGHHJDHGOA| { &mut m.GBFOEDCEFMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DLGPDENIBFH",
            |m: &KIGHHJDHGOA| { &m.DLGPDENIBFH },
            |m: &mut KIGHHJDHGOA| { &mut m.DLGPDENIBFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "FGBPHGGFBEG",
            |m: &KIGHHJDHGOA| { &m.FGBPHGGFBEG },
            |m: &mut KIGHHJDHGOA| { &mut m.FGBPHGGFBEG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KIGHHJDHGOA>(
            "KIGHHJDHGOA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KIGHHJDHGOA {
    const NAME: &'static str = "KIGHHJDHGOA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IPNHCCODNDI = is.read_uint32()?;
                },
                16 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                24 => {
                    self.GGFFPIJILIB = is.read_uint32()?;
                },
                32 => {
                    self.GBFOEDCEFMI = is.read_uint32()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.DLGPDENIBFH)?;
                },
                40 => {
                    self.DLGPDENIBFH.push(is.read_uint32()?);
                },
                50 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            21 => value = is.read_float()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.FGBPHGGFBEG.insert(key, value);
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
        if self.IPNHCCODNDI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.IPNHCCODNDI);
        }
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.JKOCJIMAGBN);
        }
        if self.GGFFPIJILIB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.GGFFPIJILIB);
        }
        if self.GBFOEDCEFMI != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.GBFOEDCEFMI);
        }
        for value in &self.DLGPDENIBFH {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for (k, v) in &self.FGBPHGGFBEG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IPNHCCODNDI != 0 {
            os.write_uint32(1, self.IPNHCCODNDI)?;
        }
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(2, self.JKOCJIMAGBN)?;
        }
        if self.GGFFPIJILIB != 0 {
            os.write_uint32(3, self.GGFFPIJILIB)?;
        }
        if self.GBFOEDCEFMI != 0 {
            os.write_uint32(4, self.GBFOEDCEFMI)?;
        }
        for v in &self.DLGPDENIBFH {
            os.write_uint32(5, *v)?;
        };
        for (k, v) in &self.FGBPHGGFBEG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            os.write_raw_varint32(50)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_float(2, *v)?;
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

    fn new() -> KIGHHJDHGOA {
        KIGHHJDHGOA::new()
    }

    fn clear(&mut self) {
        self.IPNHCCODNDI = 0;
        self.JKOCJIMAGBN = 0;
        self.GGFFPIJILIB = 0;
        self.GBFOEDCEFMI = 0;
        self.DLGPDENIBFH.clear();
        self.FGBPHGGFBEG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KIGHHJDHGOA {
        static instance: ::protobuf::rt::Lazy<KIGHHJDHGOA> = ::protobuf::rt::Lazy::new();
        instance.get(KIGHHJDHGOA::new)
    }
}

impl ::protobuf::MessageFull for KIGHHJDHGOA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KIGHHJDHGOA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KIGHHJDHGOA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KIGHHJDHGOA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KIGHHJDHGOA.proto\"\xb8\x02\n\x0bKIGHHJDHGOA\x12\x20\n\x0bIPNHCCOD\
    NDI\x18\x01\x20\x01(\rR\x0bIPNHCCODNDI\x12\x20\n\x0bJKOCJIMAGBN\x18\x02\
    \x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\n\x0bGGFFPIJILIB\x18\x03\x20\x01(\rR\
    \x0bGGFFPIJILIB\x12\x20\n\x0bGBFOEDCEFMI\x18\x04\x20\x01(\rR\x0bGBFOEDCE\
    FMI\x12\x20\n\x0bDLGPDENIBFH\x18\x05\x20\x03(\rR\x0bDLGPDENIBFH\x12?\n\
    \x0bFGBPHGGFBEG\x18\x06\x20\x03(\x0b2\x1d.KIGHHJDHGOA.FGBPHGGFBEGEntryR\
    \x0bFGBPHGGFBEG\x1a>\n\x10FGBPHGGFBEGEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x02R\x05value:\x028\
    \x01b\x06proto3\
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
            messages.push(KIGHHJDHGOA::generated_message_descriptor_data());
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