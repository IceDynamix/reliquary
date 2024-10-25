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

//! Generated file from `IFMGKGDIHGP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IFMGKGDIHGP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IFMGKGDIHGP {
    // message fields
    // @@protoc_insertion_point(field:IFMGKGDIHGP.BILCCKHCLKO)
    pub BILCCKHCLKO: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.CNKMGCPCCJB)
    pub CNKMGCPCCJB: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.LEEDBAPIENO)
    pub LEEDBAPIENO: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.ACMLKJIEEKM)
    pub ACMLKJIEEKM: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.JBLDBMHAFPO)
    pub JBLDBMHAFPO: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.OECGHDFOBMI)
    pub OECGHDFOBMI: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.CKECAONGABI)
    pub CKECAONGABI: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.KGKOFNKLEEH)
    pub KGKOFNKLEEH: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.OGJDFMIFPAM)
    pub OGJDFMIFPAM: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.KPPAHBBJDEF)
    pub KPPAHBBJDEF: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.id)
    pub id: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.JCPGFFLKELM)
    pub JCPGFFLKELM: u32,
    // @@protoc_insertion_point(field:IFMGKGDIHGP.OCNDNODLFAJ)
    pub OCNDNODLFAJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IFMGKGDIHGP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IFMGKGDIHGP {
    fn default() -> &'a IFMGKGDIHGP {
        <IFMGKGDIHGP as ::protobuf::Message>::default_instance()
    }
}

impl IFMGKGDIHGP {
    pub fn new() -> IFMGKGDIHGP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BILCCKHCLKO",
            |m: &IFMGKGDIHGP| { &m.BILCCKHCLKO },
            |m: &mut IFMGKGDIHGP| { &mut m.BILCCKHCLKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNKMGCPCCJB",
            |m: &IFMGKGDIHGP| { &m.CNKMGCPCCJB },
            |m: &mut IFMGKGDIHGP| { &mut m.CNKMGCPCCJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEEDBAPIENO",
            |m: &IFMGKGDIHGP| { &m.LEEDBAPIENO },
            |m: &mut IFMGKGDIHGP| { &mut m.LEEDBAPIENO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACMLKJIEEKM",
            |m: &IFMGKGDIHGP| { &m.ACMLKJIEEKM },
            |m: &mut IFMGKGDIHGP| { &mut m.ACMLKJIEEKM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JBLDBMHAFPO",
            |m: &IFMGKGDIHGP| { &m.JBLDBMHAFPO },
            |m: &mut IFMGKGDIHGP| { &mut m.JBLDBMHAFPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OECGHDFOBMI",
            |m: &IFMGKGDIHGP| { &m.OECGHDFOBMI },
            |m: &mut IFMGKGDIHGP| { &mut m.OECGHDFOBMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CKECAONGABI",
            |m: &IFMGKGDIHGP| { &m.CKECAONGABI },
            |m: &mut IFMGKGDIHGP| { &mut m.CKECAONGABI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGKOFNKLEEH",
            |m: &IFMGKGDIHGP| { &m.KGKOFNKLEEH },
            |m: &mut IFMGKGDIHGP| { &mut m.KGKOFNKLEEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGJDFMIFPAM",
            |m: &IFMGKGDIHGP| { &m.OGJDFMIFPAM },
            |m: &mut IFMGKGDIHGP| { &mut m.OGJDFMIFPAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPPAHBBJDEF",
            |m: &IFMGKGDIHGP| { &m.KPPAHBBJDEF },
            |m: &mut IFMGKGDIHGP| { &mut m.KPPAHBBJDEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &IFMGKGDIHGP| { &m.id },
            |m: &mut IFMGKGDIHGP| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCPGFFLKELM",
            |m: &IFMGKGDIHGP| { &m.JCPGFFLKELM },
            |m: &mut IFMGKGDIHGP| { &mut m.JCPGFFLKELM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OCNDNODLFAJ",
            |m: &IFMGKGDIHGP| { &m.OCNDNODLFAJ },
            |m: &mut IFMGKGDIHGP| { &mut m.OCNDNODLFAJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IFMGKGDIHGP>(
            "IFMGKGDIHGP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IFMGKGDIHGP {
    const NAME: &'static str = "IFMGKGDIHGP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.BILCCKHCLKO = is.read_uint32()?;
                },
                16 => {
                    self.CNKMGCPCCJB = is.read_uint32()?;
                },
                24 => {
                    self.LEEDBAPIENO = is.read_uint32()?;
                },
                32 => {
                    self.ACMLKJIEEKM = is.read_uint32()?;
                },
                40 => {
                    self.JBLDBMHAFPO = is.read_uint32()?;
                },
                48 => {
                    self.OECGHDFOBMI = is.read_uint32()?;
                },
                56 => {
                    self.CKECAONGABI = is.read_uint32()?;
                },
                64 => {
                    self.KGKOFNKLEEH = is.read_uint32()?;
                },
                72 => {
                    self.OGJDFMIFPAM = is.read_uint32()?;
                },
                80 => {
                    self.KPPAHBBJDEF = is.read_uint32()?;
                },
                88 => {
                    self.id = is.read_uint32()?;
                },
                96 => {
                    self.JCPGFFLKELM = is.read_uint32()?;
                },
                104 => {
                    self.OCNDNODLFAJ = is.read_uint32()?;
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
        if self.BILCCKHCLKO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.BILCCKHCLKO);
        }
        if self.CNKMGCPCCJB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CNKMGCPCCJB);
        }
        if self.LEEDBAPIENO != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LEEDBAPIENO);
        }
        if self.ACMLKJIEEKM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ACMLKJIEEKM);
        }
        if self.JBLDBMHAFPO != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.JBLDBMHAFPO);
        }
        if self.OECGHDFOBMI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.OECGHDFOBMI);
        }
        if self.CKECAONGABI != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.CKECAONGABI);
        }
        if self.KGKOFNKLEEH != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.KGKOFNKLEEH);
        }
        if self.OGJDFMIFPAM != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.OGJDFMIFPAM);
        }
        if self.KPPAHBBJDEF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.KPPAHBBJDEF);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.id);
        }
        if self.JCPGFFLKELM != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.JCPGFFLKELM);
        }
        if self.OCNDNODLFAJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OCNDNODLFAJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BILCCKHCLKO != 0 {
            os.write_uint32(1, self.BILCCKHCLKO)?;
        }
        if self.CNKMGCPCCJB != 0 {
            os.write_uint32(2, self.CNKMGCPCCJB)?;
        }
        if self.LEEDBAPIENO != 0 {
            os.write_uint32(3, self.LEEDBAPIENO)?;
        }
        if self.ACMLKJIEEKM != 0 {
            os.write_uint32(4, self.ACMLKJIEEKM)?;
        }
        if self.JBLDBMHAFPO != 0 {
            os.write_uint32(5, self.JBLDBMHAFPO)?;
        }
        if self.OECGHDFOBMI != 0 {
            os.write_uint32(6, self.OECGHDFOBMI)?;
        }
        if self.CKECAONGABI != 0 {
            os.write_uint32(7, self.CKECAONGABI)?;
        }
        if self.KGKOFNKLEEH != 0 {
            os.write_uint32(8, self.KGKOFNKLEEH)?;
        }
        if self.OGJDFMIFPAM != 0 {
            os.write_uint32(9, self.OGJDFMIFPAM)?;
        }
        if self.KPPAHBBJDEF != 0 {
            os.write_uint32(10, self.KPPAHBBJDEF)?;
        }
        if self.id != 0 {
            os.write_uint32(11, self.id)?;
        }
        if self.JCPGFFLKELM != 0 {
            os.write_uint32(12, self.JCPGFFLKELM)?;
        }
        if self.OCNDNODLFAJ != 0 {
            os.write_uint32(13, self.OCNDNODLFAJ)?;
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

    fn new() -> IFMGKGDIHGP {
        IFMGKGDIHGP::new()
    }

    fn clear(&mut self) {
        self.BILCCKHCLKO = 0;
        self.CNKMGCPCCJB = 0;
        self.LEEDBAPIENO = 0;
        self.ACMLKJIEEKM = 0;
        self.JBLDBMHAFPO = 0;
        self.OECGHDFOBMI = 0;
        self.CKECAONGABI = 0;
        self.KGKOFNKLEEH = 0;
        self.OGJDFMIFPAM = 0;
        self.KPPAHBBJDEF = 0;
        self.id = 0;
        self.JCPGFFLKELM = 0;
        self.OCNDNODLFAJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IFMGKGDIHGP {
        static instance: IFMGKGDIHGP = IFMGKGDIHGP {
            BILCCKHCLKO: 0,
            CNKMGCPCCJB: 0,
            LEEDBAPIENO: 0,
            ACMLKJIEEKM: 0,
            JBLDBMHAFPO: 0,
            OECGHDFOBMI: 0,
            CKECAONGABI: 0,
            KGKOFNKLEEH: 0,
            OGJDFMIFPAM: 0,
            KPPAHBBJDEF: 0,
            id: 0,
            JCPGFFLKELM: 0,
            OCNDNODLFAJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IFMGKGDIHGP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IFMGKGDIHGP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IFMGKGDIHGP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IFMGKGDIHGP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IFMGKGDIHGP.proto\"\xb5\x03\n\x0bIFMGKGDIHGP\x12\x20\n\x0bBILCCKHC\
    LKO\x18\x01\x20\x01(\rR\x0bBILCCKHCLKO\x12\x20\n\x0bCNKMGCPCCJB\x18\x02\
    \x20\x01(\rR\x0bCNKMGCPCCJB\x12\x20\n\x0bLEEDBAPIENO\x18\x03\x20\x01(\rR\
    \x0bLEEDBAPIENO\x12\x20\n\x0bACMLKJIEEKM\x18\x04\x20\x01(\rR\x0bACMLKJIE\
    EKM\x12\x20\n\x0bJBLDBMHAFPO\x18\x05\x20\x01(\rR\x0bJBLDBMHAFPO\x12\x20\
    \n\x0bOECGHDFOBMI\x18\x06\x20\x01(\rR\x0bOECGHDFOBMI\x12\x20\n\x0bCKECAO\
    NGABI\x18\x07\x20\x01(\rR\x0bCKECAONGABI\x12\x20\n\x0bKGKOFNKLEEH\x18\
    \x08\x20\x01(\rR\x0bKGKOFNKLEEH\x12\x20\n\x0bOGJDFMIFPAM\x18\t\x20\x01(\
    \rR\x0bOGJDFMIFPAM\x12\x20\n\x0bKPPAHBBJDEF\x18\n\x20\x01(\rR\x0bKPPAHBB\
    JDEF\x12\x0e\n\x02id\x18\x0b\x20\x01(\rR\x02id\x12\x20\n\x0bJCPGFFLKELM\
    \x18\x0c\x20\x01(\rR\x0bJCPGFFLKELM\x12\x20\n\x0bOCNDNODLFAJ\x18\r\x20\
    \x01(\rR\x0bOCNDNODLFAJb\x06proto3\
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
            messages.push(IFMGKGDIHGP::generated_message_descriptor_data());
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