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

//! Generated file from `JGJOCLOCIBP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JGJOCLOCIBP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JGJOCLOCIBP {
    // message fields
    // @@protoc_insertion_point(field:JGJOCLOCIBP.KEMDFOPLDJF)
    pub KEMDFOPLDJF: u32,
    // @@protoc_insertion_point(field:JGJOCLOCIBP.CNKHCMLBILG)
    pub CNKHCMLBILG: u32,
    // @@protoc_insertion_point(field:JGJOCLOCIBP.KFMMHAKCJCE)
    pub KFMMHAKCJCE: u32,
    // @@protoc_insertion_point(field:JGJOCLOCIBP.CFKNBLNJEBI)
    pub CFKNBLNJEBI: ::std::vec::Vec<super::MDLINJLHLGB::MDLINJLHLGB>,
    // @@protoc_insertion_point(field:JGJOCLOCIBP.MINMAIAELGL)
    pub MINMAIAELGL: ::std::vec::Vec<super::IDHDDLAPDKD::IDHDDLAPDKD>,
    // @@protoc_insertion_point(field:JGJOCLOCIBP.IEEFNOBDKPC)
    pub IEEFNOBDKPC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JGJOCLOCIBP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JGJOCLOCIBP {
    fn default() -> &'a JGJOCLOCIBP {
        <JGJOCLOCIBP as ::protobuf::Message>::default_instance()
    }
}

impl JGJOCLOCIBP {
    pub fn new() -> JGJOCLOCIBP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KEMDFOPLDJF",
            |m: &JGJOCLOCIBP| { &m.KEMDFOPLDJF },
            |m: &mut JGJOCLOCIBP| { &mut m.KEMDFOPLDJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CNKHCMLBILG",
            |m: &JGJOCLOCIBP| { &m.CNKHCMLBILG },
            |m: &mut JGJOCLOCIBP| { &mut m.CNKHCMLBILG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFMMHAKCJCE",
            |m: &JGJOCLOCIBP| { &m.KFMMHAKCJCE },
            |m: &mut JGJOCLOCIBP| { &mut m.KFMMHAKCJCE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CFKNBLNJEBI",
            |m: &JGJOCLOCIBP| { &m.CFKNBLNJEBI },
            |m: &mut JGJOCLOCIBP| { &mut m.CFKNBLNJEBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MINMAIAELGL",
            |m: &JGJOCLOCIBP| { &m.MINMAIAELGL },
            |m: &mut JGJOCLOCIBP| { &mut m.MINMAIAELGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IEEFNOBDKPC",
            |m: &JGJOCLOCIBP| { &m.IEEFNOBDKPC },
            |m: &mut JGJOCLOCIBP| { &mut m.IEEFNOBDKPC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JGJOCLOCIBP>(
            "JGJOCLOCIBP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JGJOCLOCIBP {
    const NAME: &'static str = "JGJOCLOCIBP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.KEMDFOPLDJF = is.read_uint32()?;
                },
                16 => {
                    self.CNKHCMLBILG = is.read_uint32()?;
                },
                24 => {
                    self.KFMMHAKCJCE = is.read_uint32()?;
                },
                34 => {
                    self.CFKNBLNJEBI.push(is.read_message()?);
                },
                42 => {
                    self.MINMAIAELGL.push(is.read_message()?);
                },
                48 => {
                    self.IEEFNOBDKPC = is.read_uint32()?;
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
        if self.KEMDFOPLDJF != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.KEMDFOPLDJF);
        }
        if self.CNKHCMLBILG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CNKHCMLBILG);
        }
        if self.KFMMHAKCJCE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.KFMMHAKCJCE);
        }
        for value in &self.CFKNBLNJEBI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MINMAIAELGL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IEEFNOBDKPC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IEEFNOBDKPC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KEMDFOPLDJF != 0 {
            os.write_uint32(1, self.KEMDFOPLDJF)?;
        }
        if self.CNKHCMLBILG != 0 {
            os.write_uint32(2, self.CNKHCMLBILG)?;
        }
        if self.KFMMHAKCJCE != 0 {
            os.write_uint32(3, self.KFMMHAKCJCE)?;
        }
        for v in &self.CFKNBLNJEBI {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.MINMAIAELGL {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.IEEFNOBDKPC != 0 {
            os.write_uint32(6, self.IEEFNOBDKPC)?;
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

    fn new() -> JGJOCLOCIBP {
        JGJOCLOCIBP::new()
    }

    fn clear(&mut self) {
        self.KEMDFOPLDJF = 0;
        self.CNKHCMLBILG = 0;
        self.KFMMHAKCJCE = 0;
        self.CFKNBLNJEBI.clear();
        self.MINMAIAELGL.clear();
        self.IEEFNOBDKPC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JGJOCLOCIBP {
        static instance: JGJOCLOCIBP = JGJOCLOCIBP {
            KEMDFOPLDJF: 0,
            CNKHCMLBILG: 0,
            KFMMHAKCJCE: 0,
            CFKNBLNJEBI: ::std::vec::Vec::new(),
            MINMAIAELGL: ::std::vec::Vec::new(),
            IEEFNOBDKPC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JGJOCLOCIBP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JGJOCLOCIBP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JGJOCLOCIBP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JGJOCLOCIBP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JGJOCLOCIBP.proto\x1a\x11IDHDDLAPDKD.proto\x1a\x11MDLINJLHLGB.prot\
    o\"\xf5\x01\n\x0bJGJOCLOCIBP\x12\x20\n\x0bKEMDFOPLDJF\x18\x01\x20\x01(\r\
    R\x0bKEMDFOPLDJF\x12\x20\n\x0bCNKHCMLBILG\x18\x02\x20\x01(\rR\x0bCNKHCML\
    BILG\x12\x20\n\x0bKFMMHAKCJCE\x18\x03\x20\x01(\rR\x0bKFMMHAKCJCE\x12.\n\
    \x0bCFKNBLNJEBI\x18\x04\x20\x03(\x0b2\x0c.MDLINJLHLGBR\x0bCFKNBLNJEBI\
    \x12.\n\x0bMINMAIAELGL\x18\x05\x20\x03(\x0b2\x0c.IDHDDLAPDKDR\x0bMINMAIA\
    ELGL\x12\x20\n\x0bIEEFNOBDKPC\x18\x06\x20\x01(\rR\x0bIEEFNOBDKPCb\x06pro\
    to3\
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
            deps.push(super::IDHDDLAPDKD::file_descriptor().clone());
            deps.push(super::MDLINJLHLGB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JGJOCLOCIBP::generated_message_descriptor_data());
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
