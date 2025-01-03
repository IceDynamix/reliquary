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

//! Generated file from `EBGLBLBNNME.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EBGLBLBNNME)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EBGLBLBNNME {
    // message fields
    // @@protoc_insertion_point(field:EBGLBLBNNME.ANOLIPJIGMN)
    pub ANOLIPJIGMN: u32,
    // @@protoc_insertion_point(field:EBGLBLBNNME.IAJHMELEJOB)
    pub IAJHMELEJOB: f64,
    // @@protoc_insertion_point(field:EBGLBLBNNME.OLPBENMDNJL)
    pub OLPBENMDNJL: f64,
    // @@protoc_insertion_point(field:EBGLBLBNNME.IJOLEKJFLGF)
    pub IJOLEKJFLGF: u32,
    // @@protoc_insertion_point(field:EBGLBLBNNME.BEPEKKHIFKE)
    pub BEPEKKHIFKE: u32,
    // @@protoc_insertion_point(field:EBGLBLBNNME.FMJPIKHPDJG)
    pub FMJPIKHPDJG: f64,
    // @@protoc_insertion_point(field:EBGLBLBNNME.NPMLANODKJM)
    pub NPMLANODKJM: f64,
    // special fields
    // @@protoc_insertion_point(special_field:EBGLBLBNNME.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EBGLBLBNNME {
    fn default() -> &'a EBGLBLBNNME {
        <EBGLBLBNNME as ::protobuf::Message>::default_instance()
    }
}

impl EBGLBLBNNME {
    pub fn new() -> EBGLBLBNNME {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANOLIPJIGMN",
            |m: &EBGLBLBNNME| { &m.ANOLIPJIGMN },
            |m: &mut EBGLBLBNNME| { &mut m.ANOLIPJIGMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IAJHMELEJOB",
            |m: &EBGLBLBNNME| { &m.IAJHMELEJOB },
            |m: &mut EBGLBLBNNME| { &mut m.IAJHMELEJOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLPBENMDNJL",
            |m: &EBGLBLBNNME| { &m.OLPBENMDNJL },
            |m: &mut EBGLBLBNNME| { &mut m.OLPBENMDNJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJOLEKJFLGF",
            |m: &EBGLBLBNNME| { &m.IJOLEKJFLGF },
            |m: &mut EBGLBLBNNME| { &mut m.IJOLEKJFLGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEPEKKHIFKE",
            |m: &EBGLBLBNNME| { &m.BEPEKKHIFKE },
            |m: &mut EBGLBLBNNME| { &mut m.BEPEKKHIFKE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMJPIKHPDJG",
            |m: &EBGLBLBNNME| { &m.FMJPIKHPDJG },
            |m: &mut EBGLBLBNNME| { &mut m.FMJPIKHPDJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPMLANODKJM",
            |m: &EBGLBLBNNME| { &m.NPMLANODKJM },
            |m: &mut EBGLBLBNNME| { &mut m.NPMLANODKJM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EBGLBLBNNME>(
            "EBGLBLBNNME",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EBGLBLBNNME {
    const NAME: &'static str = "EBGLBLBNNME";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.ANOLIPJIGMN = is.read_uint32()?;
                },
                17 => {
                    self.IAJHMELEJOB = is.read_double()?;
                },
                25 => {
                    self.OLPBENMDNJL = is.read_double()?;
                },
                32 => {
                    self.IJOLEKJFLGF = is.read_uint32()?;
                },
                40 => {
                    self.BEPEKKHIFKE = is.read_uint32()?;
                },
                49 => {
                    self.FMJPIKHPDJG = is.read_double()?;
                },
                57 => {
                    self.NPMLANODKJM = is.read_double()?;
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
        if self.ANOLIPJIGMN != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ANOLIPJIGMN);
        }
        if self.IAJHMELEJOB != 0. {
            my_size += 1 + 8;
        }
        if self.OLPBENMDNJL != 0. {
            my_size += 1 + 8;
        }
        if self.IJOLEKJFLGF != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.IJOLEKJFLGF);
        }
        if self.BEPEKKHIFKE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BEPEKKHIFKE);
        }
        if self.FMJPIKHPDJG != 0. {
            my_size += 1 + 8;
        }
        if self.NPMLANODKJM != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ANOLIPJIGMN != 0 {
            os.write_uint32(1, self.ANOLIPJIGMN)?;
        }
        if self.IAJHMELEJOB != 0. {
            os.write_double(2, self.IAJHMELEJOB)?;
        }
        if self.OLPBENMDNJL != 0. {
            os.write_double(3, self.OLPBENMDNJL)?;
        }
        if self.IJOLEKJFLGF != 0 {
            os.write_uint32(4, self.IJOLEKJFLGF)?;
        }
        if self.BEPEKKHIFKE != 0 {
            os.write_uint32(5, self.BEPEKKHIFKE)?;
        }
        if self.FMJPIKHPDJG != 0. {
            os.write_double(6, self.FMJPIKHPDJG)?;
        }
        if self.NPMLANODKJM != 0. {
            os.write_double(7, self.NPMLANODKJM)?;
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

    fn new() -> EBGLBLBNNME {
        EBGLBLBNNME::new()
    }

    fn clear(&mut self) {
        self.ANOLIPJIGMN = 0;
        self.IAJHMELEJOB = 0.;
        self.OLPBENMDNJL = 0.;
        self.IJOLEKJFLGF = 0;
        self.BEPEKKHIFKE = 0;
        self.FMJPIKHPDJG = 0.;
        self.NPMLANODKJM = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EBGLBLBNNME {
        static instance: EBGLBLBNNME = EBGLBLBNNME {
            ANOLIPJIGMN: 0,
            IAJHMELEJOB: 0.,
            OLPBENMDNJL: 0.,
            IJOLEKJFLGF: 0,
            BEPEKKHIFKE: 0,
            FMJPIKHPDJG: 0.,
            NPMLANODKJM: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EBGLBLBNNME {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EBGLBLBNNME").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EBGLBLBNNME {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EBGLBLBNNME {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EBGLBLBNNME.proto\"\xfb\x01\n\x0bEBGLBLBNNME\x12\x20\n\x0bANOLIPJI\
    GMN\x18\x01\x20\x01(\rR\x0bANOLIPJIGMN\x12\x20\n\x0bIAJHMELEJOB\x18\x02\
    \x20\x01(\x01R\x0bIAJHMELEJOB\x12\x20\n\x0bOLPBENMDNJL\x18\x03\x20\x01(\
    \x01R\x0bOLPBENMDNJL\x12\x20\n\x0bIJOLEKJFLGF\x18\x04\x20\x01(\rR\x0bIJO\
    LEKJFLGF\x12\x20\n\x0bBEPEKKHIFKE\x18\x05\x20\x01(\rR\x0bBEPEKKHIFKE\x12\
    \x20\n\x0bFMJPIKHPDJG\x18\x06\x20\x01(\x01R\x0bFMJPIKHPDJG\x12\x20\n\x0b\
    NPMLANODKJM\x18\x07\x20\x01(\x01R\x0bNPMLANODKJMb\x06proto3\
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
            messages.push(EBGLBLBNNME::generated_message_descriptor_data());
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
