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

//! Generated file from `EHOCFBLOPKL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EHOCFBLOPKL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EHOCFBLOPKL {
    // message fields
    // @@protoc_insertion_point(field:EHOCFBLOPKL.GPPFKHAAAGL)
    pub GPPFKHAAAGL: u32,
    // @@protoc_insertion_point(field:EHOCFBLOPKL.MCGHLJELBFG)
    pub MCGHLJELBFG: u32,
    // @@protoc_insertion_point(field:EHOCFBLOPKL.HKGAGGDOCFC)
    pub HKGAGGDOCFC: bool,
    // @@protoc_insertion_point(field:EHOCFBLOPKL.JICFGNPLEFO)
    pub JICFGNPLEFO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EHOCFBLOPKL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EHOCFBLOPKL {
    fn default() -> &'a EHOCFBLOPKL {
        <EHOCFBLOPKL as ::protobuf::Message>::default_instance()
    }
}

impl EHOCFBLOPKL {
    pub fn new() -> EHOCFBLOPKL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GPPFKHAAAGL",
            |m: &EHOCFBLOPKL| { &m.GPPFKHAAAGL },
            |m: &mut EHOCFBLOPKL| { &mut m.GPPFKHAAAGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MCGHLJELBFG",
            |m: &EHOCFBLOPKL| { &m.MCGHLJELBFG },
            |m: &mut EHOCFBLOPKL| { &mut m.MCGHLJELBFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HKGAGGDOCFC",
            |m: &EHOCFBLOPKL| { &m.HKGAGGDOCFC },
            |m: &mut EHOCFBLOPKL| { &mut m.HKGAGGDOCFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JICFGNPLEFO",
            |m: &EHOCFBLOPKL| { &m.JICFGNPLEFO },
            |m: &mut EHOCFBLOPKL| { &mut m.JICFGNPLEFO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EHOCFBLOPKL>(
            "EHOCFBLOPKL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EHOCFBLOPKL {
    const NAME: &'static str = "EHOCFBLOPKL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.GPPFKHAAAGL = is.read_uint32()?;
                },
                40 => {
                    self.MCGHLJELBFG = is.read_uint32()?;
                },
                48 => {
                    self.HKGAGGDOCFC = is.read_bool()?;
                },
                72 => {
                    self.JICFGNPLEFO = is.read_uint32()?;
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
        if self.GPPFKHAAAGL != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.GPPFKHAAAGL);
        }
        if self.MCGHLJELBFG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.MCGHLJELBFG);
        }
        if self.HKGAGGDOCFC != false {
            my_size += 1 + 1;
        }
        if self.JICFGNPLEFO != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.JICFGNPLEFO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GPPFKHAAAGL != 0 {
            os.write_uint32(11, self.GPPFKHAAAGL)?;
        }
        if self.MCGHLJELBFG != 0 {
            os.write_uint32(5, self.MCGHLJELBFG)?;
        }
        if self.HKGAGGDOCFC != false {
            os.write_bool(6, self.HKGAGGDOCFC)?;
        }
        if self.JICFGNPLEFO != 0 {
            os.write_uint32(9, self.JICFGNPLEFO)?;
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

    fn new() -> EHOCFBLOPKL {
        EHOCFBLOPKL::new()
    }

    fn clear(&mut self) {
        self.GPPFKHAAAGL = 0;
        self.MCGHLJELBFG = 0;
        self.HKGAGGDOCFC = false;
        self.JICFGNPLEFO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EHOCFBLOPKL {
        static instance: EHOCFBLOPKL = EHOCFBLOPKL {
            GPPFKHAAAGL: 0,
            MCGHLJELBFG: 0,
            HKGAGGDOCFC: false,
            JICFGNPLEFO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EHOCFBLOPKL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EHOCFBLOPKL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EHOCFBLOPKL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EHOCFBLOPKL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EHOCFBLOPKL.proto\"\x95\x01\n\x0bEHOCFBLOPKL\x12\x20\n\x0bGPPFKHAA\
    AGL\x18\x0b\x20\x01(\rR\x0bGPPFKHAAAGL\x12\x20\n\x0bMCGHLJELBFG\x18\x05\
    \x20\x01(\rR\x0bMCGHLJELBFG\x12\x20\n\x0bHKGAGGDOCFC\x18\x06\x20\x01(\
    \x08R\x0bHKGAGGDOCFC\x12\x20\n\x0bJICFGNPLEFO\x18\t\x20\x01(\rR\x0bJICFG\
    NPLEFOb\x06proto3\
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
            messages.push(EHOCFBLOPKL::generated_message_descriptor_data());
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