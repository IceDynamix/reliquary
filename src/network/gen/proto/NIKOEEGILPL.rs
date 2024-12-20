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

//! Generated file from `NIKOEEGILPL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NIKOEEGILPL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NIKOEEGILPL {
    // message fields
    // @@protoc_insertion_point(field:NIKOEEGILPL.NCPFPAMHAOE)
    pub NCPFPAMHAOE: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.BPAFKIHJJCO)
    pub BPAFKIHJJCO: ::std::string::String,
    // @@protoc_insertion_point(field:NIKOEEGILPL.FIIDGPNOEDE)
    pub FIIDGPNOEDE: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.ECBIMNBDGOM)
    pub ECBIMNBDGOM: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.ACCBPEFDPEE)
    pub ACCBPEFDPEE: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.MCAOMCMJGKH)
    pub MCAOMCMJGKH: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.LCHLFLCPGPL)
    pub LCHLFLCPGPL: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.GIEFONPPGGN)
    pub GIEFONPPGGN: u32,
    // @@protoc_insertion_point(field:NIKOEEGILPL.HELICMLLEHC)
    pub HELICMLLEHC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NIKOEEGILPL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NIKOEEGILPL {
    fn default() -> &'a NIKOEEGILPL {
        <NIKOEEGILPL as ::protobuf::Message>::default_instance()
    }
}

impl NIKOEEGILPL {
    pub fn new() -> NIKOEEGILPL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCPFPAMHAOE",
            |m: &NIKOEEGILPL| { &m.NCPFPAMHAOE },
            |m: &mut NIKOEEGILPL| { &mut m.NCPFPAMHAOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPAFKIHJJCO",
            |m: &NIKOEEGILPL| { &m.BPAFKIHJJCO },
            |m: &mut NIKOEEGILPL| { &mut m.BPAFKIHJJCO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FIIDGPNOEDE",
            |m: &NIKOEEGILPL| { &m.FIIDGPNOEDE },
            |m: &mut NIKOEEGILPL| { &mut m.FIIDGPNOEDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ECBIMNBDGOM",
            |m: &NIKOEEGILPL| { &m.ECBIMNBDGOM },
            |m: &mut NIKOEEGILPL| { &mut m.ECBIMNBDGOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACCBPEFDPEE",
            |m: &NIKOEEGILPL| { &m.ACCBPEFDPEE },
            |m: &mut NIKOEEGILPL| { &mut m.ACCBPEFDPEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MCAOMCMJGKH",
            |m: &NIKOEEGILPL| { &m.MCAOMCMJGKH },
            |m: &mut NIKOEEGILPL| { &mut m.MCAOMCMJGKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCHLFLCPGPL",
            |m: &NIKOEEGILPL| { &m.LCHLFLCPGPL },
            |m: &mut NIKOEEGILPL| { &mut m.LCHLFLCPGPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIEFONPPGGN",
            |m: &NIKOEEGILPL| { &m.GIEFONPPGGN },
            |m: &mut NIKOEEGILPL| { &mut m.GIEFONPPGGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HELICMLLEHC",
            |m: &NIKOEEGILPL| { &m.HELICMLLEHC },
            |m: &mut NIKOEEGILPL| { &mut m.HELICMLLEHC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NIKOEEGILPL>(
            "NIKOEEGILPL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NIKOEEGILPL {
    const NAME: &'static str = "NIKOEEGILPL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.NCPFPAMHAOE = is.read_uint32()?;
                },
                18 => {
                    self.BPAFKIHJJCO = is.read_string()?;
                },
                24 => {
                    self.FIIDGPNOEDE = is.read_uint32()?;
                },
                32 => {
                    self.ECBIMNBDGOM = is.read_uint32()?;
                },
                40 => {
                    self.ACCBPEFDPEE = is.read_uint32()?;
                },
                48 => {
                    self.MCAOMCMJGKH = is.read_uint32()?;
                },
                56 => {
                    self.LCHLFLCPGPL = is.read_uint32()?;
                },
                64 => {
                    self.GIEFONPPGGN = is.read_uint32()?;
                },
                72 => {
                    self.HELICMLLEHC = is.read_uint32()?;
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
        if self.NCPFPAMHAOE != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.NCPFPAMHAOE);
        }
        if !self.BPAFKIHJJCO.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.BPAFKIHJJCO);
        }
        if self.FIIDGPNOEDE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.FIIDGPNOEDE);
        }
        if self.ECBIMNBDGOM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ECBIMNBDGOM);
        }
        if self.ACCBPEFDPEE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.ACCBPEFDPEE);
        }
        if self.MCAOMCMJGKH != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.MCAOMCMJGKH);
        }
        if self.LCHLFLCPGPL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LCHLFLCPGPL);
        }
        if self.GIEFONPPGGN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GIEFONPPGGN);
        }
        if self.HELICMLLEHC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.HELICMLLEHC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NCPFPAMHAOE != 0 {
            os.write_uint32(1, self.NCPFPAMHAOE)?;
        }
        if !self.BPAFKIHJJCO.is_empty() {
            os.write_string(2, &self.BPAFKIHJJCO)?;
        }
        if self.FIIDGPNOEDE != 0 {
            os.write_uint32(3, self.FIIDGPNOEDE)?;
        }
        if self.ECBIMNBDGOM != 0 {
            os.write_uint32(4, self.ECBIMNBDGOM)?;
        }
        if self.ACCBPEFDPEE != 0 {
            os.write_uint32(5, self.ACCBPEFDPEE)?;
        }
        if self.MCAOMCMJGKH != 0 {
            os.write_uint32(6, self.MCAOMCMJGKH)?;
        }
        if self.LCHLFLCPGPL != 0 {
            os.write_uint32(7, self.LCHLFLCPGPL)?;
        }
        if self.GIEFONPPGGN != 0 {
            os.write_uint32(8, self.GIEFONPPGGN)?;
        }
        if self.HELICMLLEHC != 0 {
            os.write_uint32(9, self.HELICMLLEHC)?;
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

    fn new() -> NIKOEEGILPL {
        NIKOEEGILPL::new()
    }

    fn clear(&mut self) {
        self.NCPFPAMHAOE = 0;
        self.BPAFKIHJJCO.clear();
        self.FIIDGPNOEDE = 0;
        self.ECBIMNBDGOM = 0;
        self.ACCBPEFDPEE = 0;
        self.MCAOMCMJGKH = 0;
        self.LCHLFLCPGPL = 0;
        self.GIEFONPPGGN = 0;
        self.HELICMLLEHC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NIKOEEGILPL {
        static instance: NIKOEEGILPL = NIKOEEGILPL {
            NCPFPAMHAOE: 0,
            BPAFKIHJJCO: ::std::string::String::new(),
            FIIDGPNOEDE: 0,
            ECBIMNBDGOM: 0,
            ACCBPEFDPEE: 0,
            MCAOMCMJGKH: 0,
            LCHLFLCPGPL: 0,
            GIEFONPPGGN: 0,
            HELICMLLEHC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NIKOEEGILPL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NIKOEEGILPL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NIKOEEGILPL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NIKOEEGILPL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NIKOEEGILPL.proto\"\xbf\x02\n\x0bNIKOEEGILPL\x12\x20\n\x0bNCPFPAMH\
    AOE\x18\x01\x20\x01(\rR\x0bNCPFPAMHAOE\x12\x20\n\x0bBPAFKIHJJCO\x18\x02\
    \x20\x01(\tR\x0bBPAFKIHJJCO\x12\x20\n\x0bFIIDGPNOEDE\x18\x03\x20\x01(\rR\
    \x0bFIIDGPNOEDE\x12\x20\n\x0bECBIMNBDGOM\x18\x04\x20\x01(\rR\x0bECBIMNBD\
    GOM\x12\x20\n\x0bACCBPEFDPEE\x18\x05\x20\x01(\rR\x0bACCBPEFDPEE\x12\x20\
    \n\x0bMCAOMCMJGKH\x18\x06\x20\x01(\rR\x0bMCAOMCMJGKH\x12\x20\n\x0bLCHLFL\
    CPGPL\x18\x07\x20\x01(\rR\x0bLCHLFLCPGPL\x12\x20\n\x0bGIEFONPPGGN\x18\
    \x08\x20\x01(\rR\x0bGIEFONPPGGN\x12\x20\n\x0bHELICMLLEHC\x18\t\x20\x01(\
    \rR\x0bHELICMLLEHCb\x06proto3\
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
            messages.push(NIKOEEGILPL::generated_message_descriptor_data());
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
