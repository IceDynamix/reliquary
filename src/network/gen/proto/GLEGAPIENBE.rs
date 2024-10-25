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

//! Generated file from `GLEGAPIENBE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GLEGAPIENBE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GLEGAPIENBE {
    // message fields
    // @@protoc_insertion_point(field:GLEGAPIENBE.EFDPHFELDEG)
    pub EFDPHFELDEG: u32,
    // @@protoc_insertion_point(field:GLEGAPIENBE.ANJDKLJAADP)
    pub ANJDKLJAADP: u32,
    // @@protoc_insertion_point(field:GLEGAPIENBE.LAFDBHAHPFH)
    pub LAFDBHAHPFH: bool,
    // @@protoc_insertion_point(field:GLEGAPIENBE.ICBAAFNNGPG)
    pub ICBAAFNNGPG: ::protobuf::MessageField<super::NIOGJMGAJJN::NIOGJMGAJJN>,
    // @@protoc_insertion_point(field:GLEGAPIENBE.IHKAJOHGLLN)
    pub IHKAJOHGLLN: ::std::vec::Vec<super::IEMLJLPCADK::IEMLJLPCADK>,
    // @@protoc_insertion_point(field:GLEGAPIENBE.HNCNCIGAFDN)
    pub HNCNCIGAFDN: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GLEGAPIENBE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GLEGAPIENBE {
    fn default() -> &'a GLEGAPIENBE {
        <GLEGAPIENBE as ::protobuf::Message>::default_instance()
    }
}

impl GLEGAPIENBE {
    pub fn new() -> GLEGAPIENBE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFDPHFELDEG",
            |m: &GLEGAPIENBE| { &m.EFDPHFELDEG },
            |m: &mut GLEGAPIENBE| { &mut m.EFDPHFELDEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANJDKLJAADP",
            |m: &GLEGAPIENBE| { &m.ANJDKLJAADP },
            |m: &mut GLEGAPIENBE| { &mut m.ANJDKLJAADP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LAFDBHAHPFH",
            |m: &GLEGAPIENBE| { &m.LAFDBHAHPFH },
            |m: &mut GLEGAPIENBE| { &mut m.LAFDBHAHPFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NIOGJMGAJJN::NIOGJMGAJJN>(
            "ICBAAFNNGPG",
            |m: &GLEGAPIENBE| { &m.ICBAAFNNGPG },
            |m: &mut GLEGAPIENBE| { &mut m.ICBAAFNNGPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IHKAJOHGLLN",
            |m: &GLEGAPIENBE| { &m.IHKAJOHGLLN },
            |m: &mut GLEGAPIENBE| { &mut m.IHKAJOHGLLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNCNCIGAFDN",
            |m: &GLEGAPIENBE| { &m.HNCNCIGAFDN },
            |m: &mut GLEGAPIENBE| { &mut m.HNCNCIGAFDN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GLEGAPIENBE>(
            "GLEGAPIENBE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GLEGAPIENBE {
    const NAME: &'static str = "GLEGAPIENBE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.EFDPHFELDEG = is.read_uint32()?;
                },
                64 => {
                    self.ANJDKLJAADP = is.read_uint32()?;
                },
                48 => {
                    self.LAFDBHAHPFH = is.read_bool()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ICBAAFNNGPG)?;
                },
                34 => {
                    self.IHKAJOHGLLN.push(is.read_message()?);
                },
                40 => {
                    self.HNCNCIGAFDN = is.read_bool()?;
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
        if self.EFDPHFELDEG != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.EFDPHFELDEG);
        }
        if self.ANJDKLJAADP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ANJDKLJAADP);
        }
        if self.LAFDBHAHPFH != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.ICBAAFNNGPG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.IHKAJOHGLLN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.HNCNCIGAFDN != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EFDPHFELDEG != 0 {
            os.write_uint32(7, self.EFDPHFELDEG)?;
        }
        if self.ANJDKLJAADP != 0 {
            os.write_uint32(8, self.ANJDKLJAADP)?;
        }
        if self.LAFDBHAHPFH != false {
            os.write_bool(6, self.LAFDBHAHPFH)?;
        }
        if let Some(v) = self.ICBAAFNNGPG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        for v in &self.IHKAJOHGLLN {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.HNCNCIGAFDN != false {
            os.write_bool(5, self.HNCNCIGAFDN)?;
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

    fn new() -> GLEGAPIENBE {
        GLEGAPIENBE::new()
    }

    fn clear(&mut self) {
        self.EFDPHFELDEG = 0;
        self.ANJDKLJAADP = 0;
        self.LAFDBHAHPFH = false;
        self.ICBAAFNNGPG.clear();
        self.IHKAJOHGLLN.clear();
        self.HNCNCIGAFDN = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GLEGAPIENBE {
        static instance: GLEGAPIENBE = GLEGAPIENBE {
            EFDPHFELDEG: 0,
            ANJDKLJAADP: 0,
            LAFDBHAHPFH: false,
            ICBAAFNNGPG: ::protobuf::MessageField::none(),
            IHKAJOHGLLN: ::std::vec::Vec::new(),
            HNCNCIGAFDN: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GLEGAPIENBE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GLEGAPIENBE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GLEGAPIENBE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GLEGAPIENBE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GLEGAPIENBE.proto\x1a\x11IEMLJLPCADK.proto\x1a\x11NIOGJMGAJJN.prot\
    o\"\xf5\x01\n\x0bGLEGAPIENBE\x12\x20\n\x0bEFDPHFELDEG\x18\x07\x20\x01(\r\
    R\x0bEFDPHFELDEG\x12\x20\n\x0bANJDKLJAADP\x18\x08\x20\x01(\rR\x0bANJDKLJ\
    AADP\x12\x20\n\x0bLAFDBHAHPFH\x18\x06\x20\x01(\x08R\x0bLAFDBHAHPFH\x12.\
    \n\x0bICBAAFNNGPG\x18\t\x20\x01(\x0b2\x0c.NIOGJMGAJJNR\x0bICBAAFNNGPG\
    \x12.\n\x0bIHKAJOHGLLN\x18\x04\x20\x03(\x0b2\x0c.IEMLJLPCADKR\x0bIHKAJOH\
    GLLN\x12\x20\n\x0bHNCNCIGAFDN\x18\x05\x20\x01(\x08R\x0bHNCNCIGAFDNb\x06p\
    roto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::IEMLJLPCADK::file_descriptor().clone());
            deps.push(super::NIOGJMGAJJN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GLEGAPIENBE::generated_message_descriptor_data());
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
