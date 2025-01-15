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

//! Generated file from `NJPJIGGONFA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NJPJIGGONFA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NJPJIGGONFA {
    // message fields
    // @@protoc_insertion_point(field:NJPJIGGONFA.DEPEKPIEGJO)
    pub DEPEKPIEGJO: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.PPAEDKGDKDA)
    pub PPAEDKGDKDA: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.HMJBGDPIMCP)
    pub HMJBGDPIMCP: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.MFJAGFLPKFO)
    pub MFJAGFLPKFO: i64,
    // @@protoc_insertion_point(field:NJPJIGGONFA.IEHJNGOIPPI)
    pub IEHJNGOIPPI: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.LPPOOBIPGMK)
    pub LPPOOBIPGMK: u32,
    // @@protoc_insertion_point(field:NJPJIGGONFA.GAGNIBBLDFF)
    pub GAGNIBBLDFF: ::protobuf::EnumOrUnknown<super::PunkLordShareType::PunkLordShareType>,
    // @@protoc_insertion_point(field:NJPJIGGONFA.JCNEDEGKKJA)
    pub JCNEDEGKKJA: bool,
    // special fields
    // @@protoc_insertion_point(special_field:NJPJIGGONFA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NJPJIGGONFA {
    fn default() -> &'a NJPJIGGONFA {
        <NJPJIGGONFA as ::protobuf::Message>::default_instance()
    }
}

impl NJPJIGGONFA {
    pub fn new() -> NJPJIGGONFA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEPEKPIEGJO",
            |m: &NJPJIGGONFA| { &m.DEPEKPIEGJO },
            |m: &mut NJPJIGGONFA| { &mut m.DEPEKPIEGJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPAEDKGDKDA",
            |m: &NJPJIGGONFA| { &m.PPAEDKGDKDA },
            |m: &mut NJPJIGGONFA| { &mut m.PPAEDKGDKDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HMJBGDPIMCP",
            |m: &NJPJIGGONFA| { &m.HMJBGDPIMCP },
            |m: &mut NJPJIGGONFA| { &mut m.HMJBGDPIMCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &NJPJIGGONFA| { &m.CFNJJEJIGOK },
            |m: &mut NJPJIGGONFA| { &mut m.CFNJJEJIGOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MFJAGFLPKFO",
            |m: &NJPJIGGONFA| { &m.MFJAGFLPKFO },
            |m: &mut NJPJIGGONFA| { &mut m.MFJAGFLPKFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IEHJNGOIPPI",
            |m: &NJPJIGGONFA| { &m.IEHJNGOIPPI },
            |m: &mut NJPJIGGONFA| { &mut m.IEHJNGOIPPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LPPOOBIPGMK",
            |m: &NJPJIGGONFA| { &m.LPPOOBIPGMK },
            |m: &mut NJPJIGGONFA| { &mut m.LPPOOBIPGMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GAGNIBBLDFF",
            |m: &NJPJIGGONFA| { &m.GAGNIBBLDFF },
            |m: &mut NJPJIGGONFA| { &mut m.GAGNIBBLDFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCNEDEGKKJA",
            |m: &NJPJIGGONFA| { &m.JCNEDEGKKJA },
            |m: &mut NJPJIGGONFA| { &mut m.JCNEDEGKKJA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NJPJIGGONFA>(
            "NJPJIGGONFA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NJPJIGGONFA {
    const NAME: &'static str = "NJPJIGGONFA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.DEPEKPIEGJO = is.read_uint32()?;
                },
                16 => {
                    self.PPAEDKGDKDA = is.read_uint32()?;
                },
                24 => {
                    self.HMJBGDPIMCP = is.read_uint32()?;
                },
                32 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
                },
                40 => {
                    self.MFJAGFLPKFO = is.read_int64()?;
                },
                48 => {
                    self.IEHJNGOIPPI = is.read_uint32()?;
                },
                56 => {
                    self.LPPOOBIPGMK = is.read_uint32()?;
                },
                64 => {
                    self.GAGNIBBLDFF = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.JCNEDEGKKJA = is.read_bool()?;
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
        if self.DEPEKPIEGJO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DEPEKPIEGJO);
        }
        if self.PPAEDKGDKDA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PPAEDKGDKDA);
        }
        if self.HMJBGDPIMCP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HMJBGDPIMCP);
        }
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.CFNJJEJIGOK);
        }
        if self.MFJAGFLPKFO != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.MFJAGFLPKFO);
        }
        if self.IEHJNGOIPPI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IEHJNGOIPPI);
        }
        if self.LPPOOBIPGMK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LPPOOBIPGMK);
        }
        if self.GAGNIBBLDFF != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.GAGNIBBLDFF.value());
        }
        if self.JCNEDEGKKJA != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DEPEKPIEGJO != 0 {
            os.write_uint32(1, self.DEPEKPIEGJO)?;
        }
        if self.PPAEDKGDKDA != 0 {
            os.write_uint32(2, self.PPAEDKGDKDA)?;
        }
        if self.HMJBGDPIMCP != 0 {
            os.write_uint32(3, self.HMJBGDPIMCP)?;
        }
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(4, self.CFNJJEJIGOK)?;
        }
        if self.MFJAGFLPKFO != 0 {
            os.write_int64(5, self.MFJAGFLPKFO)?;
        }
        if self.IEHJNGOIPPI != 0 {
            os.write_uint32(6, self.IEHJNGOIPPI)?;
        }
        if self.LPPOOBIPGMK != 0 {
            os.write_uint32(7, self.LPPOOBIPGMK)?;
        }
        if self.GAGNIBBLDFF != ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.GAGNIBBLDFF))?;
        }
        if self.JCNEDEGKKJA != false {
            os.write_bool(9, self.JCNEDEGKKJA)?;
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

    fn new() -> NJPJIGGONFA {
        NJPJIGGONFA::new()
    }

    fn clear(&mut self) {
        self.DEPEKPIEGJO = 0;
        self.PPAEDKGDKDA = 0;
        self.HMJBGDPIMCP = 0;
        self.CFNJJEJIGOK = 0;
        self.MFJAGFLPKFO = 0;
        self.IEHJNGOIPPI = 0;
        self.LPPOOBIPGMK = 0;
        self.GAGNIBBLDFF = ::protobuf::EnumOrUnknown::new(super::PunkLordShareType::PunkLordShareType::PUNK_LORD_SHARE_TYPE_NONE);
        self.JCNEDEGKKJA = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NJPJIGGONFA {
        static instance: NJPJIGGONFA = NJPJIGGONFA {
            DEPEKPIEGJO: 0,
            PPAEDKGDKDA: 0,
            HMJBGDPIMCP: 0,
            CFNJJEJIGOK: 0,
            MFJAGFLPKFO: 0,
            IEHJNGOIPPI: 0,
            LPPOOBIPGMK: 0,
            GAGNIBBLDFF: ::protobuf::EnumOrUnknown::from_i32(0),
            JCNEDEGKKJA: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NJPJIGGONFA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NJPJIGGONFA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NJPJIGGONFA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NJPJIGGONFA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NJPJIGGONFA.proto\x1a\x17PunkLordShareType.proto\"\xd3\x02\n\x0bNJ\
    PJIGGONFA\x12\x20\n\x0bDEPEKPIEGJO\x18\x01\x20\x01(\rR\x0bDEPEKPIEGJO\
    \x12\x20\n\x0bPPAEDKGDKDA\x18\x02\x20\x01(\rR\x0bPPAEDKGDKDA\x12\x20\n\
    \x0bHMJBGDPIMCP\x18\x03\x20\x01(\rR\x0bHMJBGDPIMCP\x12\x20\n\x0bCFNJJEJI\
    GOK\x18\x04\x20\x01(\rR\x0bCFNJJEJIGOK\x12\x20\n\x0bMFJAGFLPKFO\x18\x05\
    \x20\x01(\x03R\x0bMFJAGFLPKFO\x12\x20\n\x0bIEHJNGOIPPI\x18\x06\x20\x01(\
    \rR\x0bIEHJNGOIPPI\x12\x20\n\x0bLPPOOBIPGMK\x18\x07\x20\x01(\rR\x0bLPPOO\
    BIPGMK\x124\n\x0bGAGNIBBLDFF\x18\x08\x20\x01(\x0e2\x12.PunkLordShareType\
    R\x0bGAGNIBBLDFF\x12\x20\n\x0bJCNEDEGKKJA\x18\t\x20\x01(\x08R\x0bJCNEDEG\
    KKJAb\x06proto3\
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
            deps.push(super::PunkLordShareType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NJPJIGGONFA::generated_message_descriptor_data());
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