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

//! Generated file from `MMGHBJCIBMN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MMGHBJCIBMN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MMGHBJCIBMN {
    // message fields
    // @@protoc_insertion_point(field:MMGHBJCIBMN.KDGIOOMHEMO)
    pub KDGIOOMHEMO: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.MLEJNMFKNOF)
    pub MLEJNMFKNOF: bool,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.LKJKGCEGCNG)
    pub LKJKGCEGCNG: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.MIIEHBAIDIP)
    pub MIIEHBAIDIP: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.LGKIEPGEJMN)
    pub LGKIEPGEJMN: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.MCGLMLLCGFH)
    pub MCGLMLLCGFH: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.JAEDJDFLMJA)
    pub JAEDJDFLMJA: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.FDOLBAJDPNP)
    pub FDOLBAJDPNP: u32,
    // @@protoc_insertion_point(field:MMGHBJCIBMN.IHDAIPJLEBF)
    pub IHDAIPJLEBF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MMGHBJCIBMN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MMGHBJCIBMN {
    fn default() -> &'a MMGHBJCIBMN {
        <MMGHBJCIBMN as ::protobuf::Message>::default_instance()
    }
}

impl MMGHBJCIBMN {
    pub fn new() -> MMGHBJCIBMN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDGIOOMHEMO",
            |m: &MMGHBJCIBMN| { &m.KDGIOOMHEMO },
            |m: &mut MMGHBJCIBMN| { &mut m.KDGIOOMHEMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MLEJNMFKNOF",
            |m: &MMGHBJCIBMN| { &m.MLEJNMFKNOF },
            |m: &mut MMGHBJCIBMN| { &mut m.MLEJNMFKNOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKJKGCEGCNG",
            |m: &MMGHBJCIBMN| { &m.LKJKGCEGCNG },
            |m: &mut MMGHBJCIBMN| { &mut m.LKJKGCEGCNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MIIEHBAIDIP",
            |m: &MMGHBJCIBMN| { &m.MIIEHBAIDIP },
            |m: &mut MMGHBJCIBMN| { &mut m.MIIEHBAIDIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGKIEPGEJMN",
            |m: &MMGHBJCIBMN| { &m.LGKIEPGEJMN },
            |m: &mut MMGHBJCIBMN| { &mut m.LGKIEPGEJMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MCGLMLLCGFH",
            |m: &MMGHBJCIBMN| { &m.MCGLMLLCGFH },
            |m: &mut MMGHBJCIBMN| { &mut m.MCGLMLLCGFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JAEDJDFLMJA",
            |m: &MMGHBJCIBMN| { &m.JAEDJDFLMJA },
            |m: &mut MMGHBJCIBMN| { &mut m.JAEDJDFLMJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDOLBAJDPNP",
            |m: &MMGHBJCIBMN| { &m.FDOLBAJDPNP },
            |m: &mut MMGHBJCIBMN| { &mut m.FDOLBAJDPNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHDAIPJLEBF",
            |m: &MMGHBJCIBMN| { &m.IHDAIPJLEBF },
            |m: &mut MMGHBJCIBMN| { &mut m.IHDAIPJLEBF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MMGHBJCIBMN>(
            "MMGHBJCIBMN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MMGHBJCIBMN {
    const NAME: &'static str = "MMGHBJCIBMN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.KDGIOOMHEMO = is.read_uint32()?;
                },
                96 => {
                    self.MLEJNMFKNOF = is.read_bool()?;
                },
                112 => {
                    self.LKJKGCEGCNG = is.read_uint32()?;
                },
                120 => {
                    self.MIIEHBAIDIP = is.read_uint32()?;
                },
                88 => {
                    self.LGKIEPGEJMN = is.read_uint32()?;
                },
                64 => {
                    self.MCGLMLLCGFH = is.read_uint32()?;
                },
                40 => {
                    self.JAEDJDFLMJA = is.read_uint32()?;
                },
                56 => {
                    self.FDOLBAJDPNP = is.read_uint32()?;
                },
                16 => {
                    self.IHDAIPJLEBF = is.read_uint32()?;
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
        if self.KDGIOOMHEMO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.KDGIOOMHEMO);
        }
        if self.MLEJNMFKNOF != false {
            my_size += 1 + 1;
        }
        if self.LKJKGCEGCNG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LKJKGCEGCNG);
        }
        if self.MIIEHBAIDIP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.MIIEHBAIDIP);
        }
        if self.LGKIEPGEJMN != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.LGKIEPGEJMN);
        }
        if self.MCGLMLLCGFH != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.MCGLMLLCGFH);
        }
        if self.JAEDJDFLMJA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.JAEDJDFLMJA);
        }
        if self.FDOLBAJDPNP != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FDOLBAJDPNP);
        }
        if self.IHDAIPJLEBF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IHDAIPJLEBF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KDGIOOMHEMO != 0 {
            os.write_uint32(3, self.KDGIOOMHEMO)?;
        }
        if self.MLEJNMFKNOF != false {
            os.write_bool(12, self.MLEJNMFKNOF)?;
        }
        if self.LKJKGCEGCNG != 0 {
            os.write_uint32(14, self.LKJKGCEGCNG)?;
        }
        if self.MIIEHBAIDIP != 0 {
            os.write_uint32(15, self.MIIEHBAIDIP)?;
        }
        if self.LGKIEPGEJMN != 0 {
            os.write_uint32(11, self.LGKIEPGEJMN)?;
        }
        if self.MCGLMLLCGFH != 0 {
            os.write_uint32(8, self.MCGLMLLCGFH)?;
        }
        if self.JAEDJDFLMJA != 0 {
            os.write_uint32(5, self.JAEDJDFLMJA)?;
        }
        if self.FDOLBAJDPNP != 0 {
            os.write_uint32(7, self.FDOLBAJDPNP)?;
        }
        if self.IHDAIPJLEBF != 0 {
            os.write_uint32(2, self.IHDAIPJLEBF)?;
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

    fn new() -> MMGHBJCIBMN {
        MMGHBJCIBMN::new()
    }

    fn clear(&mut self) {
        self.KDGIOOMHEMO = 0;
        self.MLEJNMFKNOF = false;
        self.LKJKGCEGCNG = 0;
        self.MIIEHBAIDIP = 0;
        self.LGKIEPGEJMN = 0;
        self.MCGLMLLCGFH = 0;
        self.JAEDJDFLMJA = 0;
        self.FDOLBAJDPNP = 0;
        self.IHDAIPJLEBF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MMGHBJCIBMN {
        static instance: MMGHBJCIBMN = MMGHBJCIBMN {
            KDGIOOMHEMO: 0,
            MLEJNMFKNOF: false,
            LKJKGCEGCNG: 0,
            MIIEHBAIDIP: 0,
            LGKIEPGEJMN: 0,
            MCGLMLLCGFH: 0,
            JAEDJDFLMJA: 0,
            FDOLBAJDPNP: 0,
            IHDAIPJLEBF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MMGHBJCIBMN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MMGHBJCIBMN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MMGHBJCIBMN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MMGHBJCIBMN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MMGHBJCIBMN.proto\"\xbf\x02\n\x0bMMGHBJCIBMN\x12\x20\n\x0bKDGIOOMH\
    EMO\x18\x03\x20\x01(\rR\x0bKDGIOOMHEMO\x12\x20\n\x0bMLEJNMFKNOF\x18\x0c\
    \x20\x01(\x08R\x0bMLEJNMFKNOF\x12\x20\n\x0bLKJKGCEGCNG\x18\x0e\x20\x01(\
    \rR\x0bLKJKGCEGCNG\x12\x20\n\x0bMIIEHBAIDIP\x18\x0f\x20\x01(\rR\x0bMIIEH\
    BAIDIP\x12\x20\n\x0bLGKIEPGEJMN\x18\x0b\x20\x01(\rR\x0bLGKIEPGEJMN\x12\
    \x20\n\x0bMCGLMLLCGFH\x18\x08\x20\x01(\rR\x0bMCGLMLLCGFH\x12\x20\n\x0bJA\
    EDJDFLMJA\x18\x05\x20\x01(\rR\x0bJAEDJDFLMJA\x12\x20\n\x0bFDOLBAJDPNP\
    \x18\x07\x20\x01(\rR\x0bFDOLBAJDPNP\x12\x20\n\x0bIHDAIPJLEBF\x18\x02\x20\
    \x01(\rR\x0bIHDAIPJLEBFb\x06proto3\
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
            messages.push(MMGHBJCIBMN::generated_message_descriptor_data());
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