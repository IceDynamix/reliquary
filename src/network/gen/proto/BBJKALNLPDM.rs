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

//! Generated file from `BBJKALNLPDM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BBJKALNLPDM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BBJKALNLPDM {
    // message fields
    // @@protoc_insertion_point(field:BBJKALNLPDM.DGLANFAMDDD)
    pub DGLANFAMDDD: u32,
    // @@protoc_insertion_point(field:BBJKALNLPDM.GJAPJLCHMFD)
    pub GJAPJLCHMFD: f64,
    // @@protoc_insertion_point(field:BBJKALNLPDM.IIMIMHIIBLL)
    pub IIMIMHIIBLL: f64,
    // @@protoc_insertion_point(field:BBJKALNLPDM.LFGKKOHHIGE)
    pub LFGKKOHHIGE: u32,
    // @@protoc_insertion_point(field:BBJKALNLPDM.DLKCBLHGBPF)
    pub DLKCBLHGBPF: u32,
    // @@protoc_insertion_point(field:BBJKALNLPDM.BKGMFMOJECF)
    pub BKGMFMOJECF: f64,
    // @@protoc_insertion_point(field:BBJKALNLPDM.MPKIFCKFHDE)
    pub MPKIFCKFHDE: f64,
    // special fields
    // @@protoc_insertion_point(special_field:BBJKALNLPDM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BBJKALNLPDM {
    fn default() -> &'a BBJKALNLPDM {
        <BBJKALNLPDM as ::protobuf::Message>::default_instance()
    }
}

impl BBJKALNLPDM {
    pub fn new() -> BBJKALNLPDM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGLANFAMDDD",
            |m: &BBJKALNLPDM| { &m.DGLANFAMDDD },
            |m: &mut BBJKALNLPDM| { &mut m.DGLANFAMDDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJAPJLCHMFD",
            |m: &BBJKALNLPDM| { &m.GJAPJLCHMFD },
            |m: &mut BBJKALNLPDM| { &mut m.GJAPJLCHMFD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIMIMHIIBLL",
            |m: &BBJKALNLPDM| { &m.IIMIMHIIBLL },
            |m: &mut BBJKALNLPDM| { &mut m.IIMIMHIIBLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFGKKOHHIGE",
            |m: &BBJKALNLPDM| { &m.LFGKKOHHIGE },
            |m: &mut BBJKALNLPDM| { &mut m.LFGKKOHHIGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLKCBLHGBPF",
            |m: &BBJKALNLPDM| { &m.DLKCBLHGBPF },
            |m: &mut BBJKALNLPDM| { &mut m.DLKCBLHGBPF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKGMFMOJECF",
            |m: &BBJKALNLPDM| { &m.BKGMFMOJECF },
            |m: &mut BBJKALNLPDM| { &mut m.BKGMFMOJECF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPKIFCKFHDE",
            |m: &BBJKALNLPDM| { &m.MPKIFCKFHDE },
            |m: &mut BBJKALNLPDM| { &mut m.MPKIFCKFHDE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BBJKALNLPDM>(
            "BBJKALNLPDM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BBJKALNLPDM {
    const NAME: &'static str = "BBJKALNLPDM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.DGLANFAMDDD = is.read_uint32()?;
                },
                17 => {
                    self.GJAPJLCHMFD = is.read_double()?;
                },
                25 => {
                    self.IIMIMHIIBLL = is.read_double()?;
                },
                32 => {
                    self.LFGKKOHHIGE = is.read_uint32()?;
                },
                40 => {
                    self.DLKCBLHGBPF = is.read_uint32()?;
                },
                49 => {
                    self.BKGMFMOJECF = is.read_double()?;
                },
                57 => {
                    self.MPKIFCKFHDE = is.read_double()?;
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
        if self.DGLANFAMDDD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DGLANFAMDDD);
        }
        if self.GJAPJLCHMFD != 0. {
            my_size += 1 + 8;
        }
        if self.IIMIMHIIBLL != 0. {
            my_size += 1 + 8;
        }
        if self.LFGKKOHHIGE != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LFGKKOHHIGE);
        }
        if self.DLKCBLHGBPF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DLKCBLHGBPF);
        }
        if self.BKGMFMOJECF != 0. {
            my_size += 1 + 8;
        }
        if self.MPKIFCKFHDE != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DGLANFAMDDD != 0 {
            os.write_uint32(1, self.DGLANFAMDDD)?;
        }
        if self.GJAPJLCHMFD != 0. {
            os.write_double(2, self.GJAPJLCHMFD)?;
        }
        if self.IIMIMHIIBLL != 0. {
            os.write_double(3, self.IIMIMHIIBLL)?;
        }
        if self.LFGKKOHHIGE != 0 {
            os.write_uint32(4, self.LFGKKOHHIGE)?;
        }
        if self.DLKCBLHGBPF != 0 {
            os.write_uint32(5, self.DLKCBLHGBPF)?;
        }
        if self.BKGMFMOJECF != 0. {
            os.write_double(6, self.BKGMFMOJECF)?;
        }
        if self.MPKIFCKFHDE != 0. {
            os.write_double(7, self.MPKIFCKFHDE)?;
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

    fn new() -> BBJKALNLPDM {
        BBJKALNLPDM::new()
    }

    fn clear(&mut self) {
        self.DGLANFAMDDD = 0;
        self.GJAPJLCHMFD = 0.;
        self.IIMIMHIIBLL = 0.;
        self.LFGKKOHHIGE = 0;
        self.DLKCBLHGBPF = 0;
        self.BKGMFMOJECF = 0.;
        self.MPKIFCKFHDE = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BBJKALNLPDM {
        static instance: BBJKALNLPDM = BBJKALNLPDM {
            DGLANFAMDDD: 0,
            GJAPJLCHMFD: 0.,
            IIMIMHIIBLL: 0.,
            LFGKKOHHIGE: 0,
            DLKCBLHGBPF: 0,
            BKGMFMOJECF: 0.,
            MPKIFCKFHDE: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BBJKALNLPDM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BBJKALNLPDM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BBJKALNLPDM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BBJKALNLPDM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BBJKALNLPDM.proto\"\xfb\x01\n\x0bBBJKALNLPDM\x12\x20\n\x0bDGLANFAM\
    DDD\x18\x01\x20\x01(\rR\x0bDGLANFAMDDD\x12\x20\n\x0bGJAPJLCHMFD\x18\x02\
    \x20\x01(\x01R\x0bGJAPJLCHMFD\x12\x20\n\x0bIIMIMHIIBLL\x18\x03\x20\x01(\
    \x01R\x0bIIMIMHIIBLL\x12\x20\n\x0bLFGKKOHHIGE\x18\x04\x20\x01(\rR\x0bLFG\
    KKOHHIGE\x12\x20\n\x0bDLKCBLHGBPF\x18\x05\x20\x01(\rR\x0bDLKCBLHGBPF\x12\
    \x20\n\x0bBKGMFMOJECF\x18\x06\x20\x01(\x01R\x0bBKGMFMOJECF\x12\x20\n\x0b\
    MPKIFCKFHDE\x18\x07\x20\x01(\x01R\x0bMPKIFCKFHDEb\x06proto3\
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
            messages.push(BBJKALNLPDM::generated_message_descriptor_data());
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