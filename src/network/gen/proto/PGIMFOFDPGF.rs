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

//! Generated file from `PGIMFOFDPGF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PGIMFOFDPGF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PGIMFOFDPGF {
    // message fields
    // @@protoc_insertion_point(field:PGIMFOFDPGF.GHEBOAMPNDN)
    pub GHEBOAMPNDN: u32,
    // @@protoc_insertion_point(field:PGIMFOFDPGF.GFLOEMMKBAK)
    pub GFLOEMMKBAK: bool,
    // @@protoc_insertion_point(field:PGIMFOFDPGF.DJLKCHKMNMI)
    pub DJLKCHKMNMI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PGIMFOFDPGF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PGIMFOFDPGF {
    fn default() -> &'a PGIMFOFDPGF {
        <PGIMFOFDPGF as ::protobuf::Message>::default_instance()
    }
}

impl PGIMFOFDPGF {
    pub fn new() -> PGIMFOFDPGF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GHEBOAMPNDN",
            |m: &PGIMFOFDPGF| { &m.GHEBOAMPNDN },
            |m: &mut PGIMFOFDPGF| { &mut m.GHEBOAMPNDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFLOEMMKBAK",
            |m: &PGIMFOFDPGF| { &m.GFLOEMMKBAK },
            |m: &mut PGIMFOFDPGF| { &mut m.GFLOEMMKBAK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DJLKCHKMNMI",
            |m: &PGIMFOFDPGF| { &m.DJLKCHKMNMI },
            |m: &mut PGIMFOFDPGF| { &mut m.DJLKCHKMNMI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PGIMFOFDPGF>(
            "PGIMFOFDPGF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PGIMFOFDPGF {
    const NAME: &'static str = "PGIMFOFDPGF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.GHEBOAMPNDN = is.read_uint32()?;
                },
                120 => {
                    self.GFLOEMMKBAK = is.read_bool()?;
                },
                56 => {
                    self.DJLKCHKMNMI = is.read_uint32()?;
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
        if self.GHEBOAMPNDN != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.GHEBOAMPNDN);
        }
        if self.GFLOEMMKBAK != false {
            my_size += 1 + 1;
        }
        if self.DJLKCHKMNMI != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DJLKCHKMNMI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GHEBOAMPNDN != 0 {
            os.write_uint32(10, self.GHEBOAMPNDN)?;
        }
        if self.GFLOEMMKBAK != false {
            os.write_bool(15, self.GFLOEMMKBAK)?;
        }
        if self.DJLKCHKMNMI != 0 {
            os.write_uint32(7, self.DJLKCHKMNMI)?;
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

    fn new() -> PGIMFOFDPGF {
        PGIMFOFDPGF::new()
    }

    fn clear(&mut self) {
        self.GHEBOAMPNDN = 0;
        self.GFLOEMMKBAK = false;
        self.DJLKCHKMNMI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PGIMFOFDPGF {
        static instance: PGIMFOFDPGF = PGIMFOFDPGF {
            GHEBOAMPNDN: 0,
            GFLOEMMKBAK: false,
            DJLKCHKMNMI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PGIMFOFDPGF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PGIMFOFDPGF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PGIMFOFDPGF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PGIMFOFDPGF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PGIMFOFDPGF.proto\"s\n\x0bPGIMFOFDPGF\x12\x20\n\x0bGHEBOAMPNDN\x18\
    \n\x20\x01(\rR\x0bGHEBOAMPNDN\x12\x20\n\x0bGFLOEMMKBAK\x18\x0f\x20\x01(\
    \x08R\x0bGFLOEMMKBAK\x12\x20\n\x0bDJLKCHKMNMI\x18\x07\x20\x01(\rR\x0bDJL\
    KCHKMNMIb\x06proto3\
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
            messages.push(PGIMFOFDPGF::generated_message_descriptor_data());
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
