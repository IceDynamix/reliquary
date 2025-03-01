// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ICALLNOLJOI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ICALLNOLJOI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ICALLNOLJOI {
    // message fields
    // @@protoc_insertion_point(field:ICALLNOLJOI.FEPKAMAILMK)
    pub FEPKAMAILMK: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.BLOEFNLOLLJ)
    pub BLOEFNLOLLJ: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.PIMBLBKEECJ)
    pub PIMBLBKEECJ: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.KFDAICILNMB)
    pub KFDAICILNMB: bool,
    // @@protoc_insertion_point(field:ICALLNOLJOI.DNPHCJEBIKB)
    pub DNPHCJEBIKB: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.CHKAPOABFLN)
    pub CHKAPOABFLN: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.CFAAFJJAADP)
    pub CFAAFJJAADP: u32,
    // @@protoc_insertion_point(field:ICALLNOLJOI.MIFOLPKEOOO)
    pub MIFOLPKEOOO: ::protobuf::MessageField<super::PGKDOAEPBEC::PGKDOAEPBEC>,
    // special fields
    // @@protoc_insertion_point(special_field:ICALLNOLJOI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ICALLNOLJOI {
    fn default() -> &'a ICALLNOLJOI {
        <ICALLNOLJOI as ::protobuf::Message>::default_instance()
    }
}

impl ICALLNOLJOI {
    pub fn new() -> ICALLNOLJOI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FEPKAMAILMK",
            |m: &ICALLNOLJOI| { &m.FEPKAMAILMK },
            |m: &mut ICALLNOLJOI| { &mut m.FEPKAMAILMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLOEFNLOLLJ",
            |m: &ICALLNOLJOI| { &m.BLOEFNLOLLJ },
            |m: &mut ICALLNOLJOI| { &mut m.BLOEFNLOLLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIMBLBKEECJ",
            |m: &ICALLNOLJOI| { &m.PIMBLBKEECJ },
            |m: &mut ICALLNOLJOI| { &mut m.PIMBLBKEECJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFDAICILNMB",
            |m: &ICALLNOLJOI| { &m.KFDAICILNMB },
            |m: &mut ICALLNOLJOI| { &mut m.KFDAICILNMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNPHCJEBIKB",
            |m: &ICALLNOLJOI| { &m.DNPHCJEBIKB },
            |m: &mut ICALLNOLJOI| { &mut m.DNPHCJEBIKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CHKAPOABFLN",
            |m: &ICALLNOLJOI| { &m.CHKAPOABFLN },
            |m: &mut ICALLNOLJOI| { &mut m.CHKAPOABFLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFAAFJJAADP",
            |m: &ICALLNOLJOI| { &m.CFAAFJJAADP },
            |m: &mut ICALLNOLJOI| { &mut m.CFAAFJJAADP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PGKDOAEPBEC::PGKDOAEPBEC>(
            "MIFOLPKEOOO",
            |m: &ICALLNOLJOI| { &m.MIFOLPKEOOO },
            |m: &mut ICALLNOLJOI| { &mut m.MIFOLPKEOOO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ICALLNOLJOI>(
            "ICALLNOLJOI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ICALLNOLJOI {
    const NAME: &'static str = "ICALLNOLJOI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.FEPKAMAILMK = is.read_uint32()?;
                },
                80 => {
                    self.BLOEFNLOLLJ = is.read_uint32()?;
                },
                8 => {
                    self.PIMBLBKEECJ = is.read_uint32()?;
                },
                40 => {
                    self.KFDAICILNMB = is.read_bool()?;
                },
                112 => {
                    self.DNPHCJEBIKB = is.read_uint32()?;
                },
                24 => {
                    self.CHKAPOABFLN = is.read_uint32()?;
                },
                64 => {
                    self.CFAAFJJAADP = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MIFOLPKEOOO)?;
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
        if self.FEPKAMAILMK != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.FEPKAMAILMK);
        }
        if self.BLOEFNLOLLJ != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.BLOEFNLOLLJ);
        }
        if self.PIMBLBKEECJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.PIMBLBKEECJ);
        }
        if self.KFDAICILNMB != false {
            my_size += 1 + 1;
        }
        if self.DNPHCJEBIKB != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.DNPHCJEBIKB);
        }
        if self.CHKAPOABFLN != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.CHKAPOABFLN);
        }
        if self.CFAAFJJAADP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.CFAAFJJAADP);
        }
        if let Some(v) = self.MIFOLPKEOOO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FEPKAMAILMK != 0 {
            os.write_uint32(13, self.FEPKAMAILMK)?;
        }
        if self.BLOEFNLOLLJ != 0 {
            os.write_uint32(10, self.BLOEFNLOLLJ)?;
        }
        if self.PIMBLBKEECJ != 0 {
            os.write_uint32(1, self.PIMBLBKEECJ)?;
        }
        if self.KFDAICILNMB != false {
            os.write_bool(5, self.KFDAICILNMB)?;
        }
        if self.DNPHCJEBIKB != 0 {
            os.write_uint32(14, self.DNPHCJEBIKB)?;
        }
        if self.CHKAPOABFLN != 0 {
            os.write_uint32(3, self.CHKAPOABFLN)?;
        }
        if self.CFAAFJJAADP != 0 {
            os.write_uint32(8, self.CFAAFJJAADP)?;
        }
        if let Some(v) = self.MIFOLPKEOOO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> ICALLNOLJOI {
        ICALLNOLJOI::new()
    }

    fn clear(&mut self) {
        self.FEPKAMAILMK = 0;
        self.BLOEFNLOLLJ = 0;
        self.PIMBLBKEECJ = 0;
        self.KFDAICILNMB = false;
        self.DNPHCJEBIKB = 0;
        self.CHKAPOABFLN = 0;
        self.CFAAFJJAADP = 0;
        self.MIFOLPKEOOO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ICALLNOLJOI {
        static instance: ICALLNOLJOI = ICALLNOLJOI {
            FEPKAMAILMK: 0,
            BLOEFNLOLLJ: 0,
            PIMBLBKEECJ: 0,
            KFDAICILNMB: false,
            DNPHCJEBIKB: 0,
            CHKAPOABFLN: 0,
            CFAAFJJAADP: 0,
            MIFOLPKEOOO: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ICALLNOLJOI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ICALLNOLJOI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ICALLNOLJOI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ICALLNOLJOI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ICALLNOLJOI.proto\x1a\x11PGKDOAEPBEC.proto\"\xab\x02\n\x0bICALLNOL\
    JOI\x12\x20\n\x0bFEPKAMAILMK\x18\r\x20\x01(\rR\x0bFEPKAMAILMK\x12\x20\n\
    \x0bBLOEFNLOLLJ\x18\n\x20\x01(\rR\x0bBLOEFNLOLLJ\x12\x20\n\x0bPIMBLBKEEC\
    J\x18\x01\x20\x01(\rR\x0bPIMBLBKEECJ\x12\x20\n\x0bKFDAICILNMB\x18\x05\
    \x20\x01(\x08R\x0bKFDAICILNMB\x12\x20\n\x0bDNPHCJEBIKB\x18\x0e\x20\x01(\
    \rR\x0bDNPHCJEBIKB\x12\x20\n\x0bCHKAPOABFLN\x18\x03\x20\x01(\rR\x0bCHKAP\
    OABFLN\x12\x20\n\x0bCFAAFJJAADP\x18\x08\x20\x01(\rR\x0bCFAAFJJAADP\x12.\
    \n\x0bMIFOLPKEOOO\x18\x0f\x20\x01(\x0b2\x0c.PGKDOAEPBECR\x0bMIFOLPKEOOOb\
    \x06proto3\
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
            deps.push(super::PGKDOAEPBEC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ICALLNOLJOI::generated_message_descriptor_data());
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
