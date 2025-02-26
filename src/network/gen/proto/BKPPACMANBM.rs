// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `BKPPACMANBM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BKPPACMANBM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BKPPACMANBM {
    // message fields
    // @@protoc_insertion_point(field:BKPPACMANBM.BHFEFEODNIM)
    pub BHFEFEODNIM: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.HKNOAKGCJBK)
    pub HKNOAKGCJBK: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.EMJDEBDMHLL)
    pub EMJDEBDMHLL: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.KFJCNGBHAGK)
    pub KFJCNGBHAGK: ::protobuf::MessageField<super::LPJOLCINFDN::LPJOLCINFDN>,
    // @@protoc_insertion_point(field:BKPPACMANBM.JFPGBKBPBNF)
    pub JFPGBKBPBNF: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.CFDFMGLLICO)
    pub CFDFMGLLICO: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.FHKKMPDDMGO)
    pub FHKKMPDDMGO: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.GEKKNDONHLJ)
    pub GEKKNDONHLJ: u32,
    // @@protoc_insertion_point(field:BKPPACMANBM.EHBDEIJJOHK)
    pub EHBDEIJJOHK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BKPPACMANBM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BKPPACMANBM {
    fn default() -> &'a BKPPACMANBM {
        <BKPPACMANBM as ::protobuf::Message>::default_instance()
    }
}

impl BKPPACMANBM {
    pub fn new() -> BKPPACMANBM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHFEFEODNIM",
            |m: &BKPPACMANBM| { &m.BHFEFEODNIM },
            |m: &mut BKPPACMANBM| { &mut m.BHFEFEODNIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HKNOAKGCJBK",
            |m: &BKPPACMANBM| { &m.HKNOAKGCJBK },
            |m: &mut BKPPACMANBM| { &mut m.HKNOAKGCJBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EMJDEBDMHLL",
            |m: &BKPPACMANBM| { &m.EMJDEBDMHLL },
            |m: &mut BKPPACMANBM| { &mut m.EMJDEBDMHLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LPJOLCINFDN::LPJOLCINFDN>(
            "KFJCNGBHAGK",
            |m: &BKPPACMANBM| { &m.KFJCNGBHAGK },
            |m: &mut BKPPACMANBM| { &mut m.KFJCNGBHAGK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFPGBKBPBNF",
            |m: &BKPPACMANBM| { &m.JFPGBKBPBNF },
            |m: &mut BKPPACMANBM| { &mut m.JFPGBKBPBNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFDFMGLLICO",
            |m: &BKPPACMANBM| { &m.CFDFMGLLICO },
            |m: &mut BKPPACMANBM| { &mut m.CFDFMGLLICO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHKKMPDDMGO",
            |m: &BKPPACMANBM| { &m.FHKKMPDDMGO },
            |m: &mut BKPPACMANBM| { &mut m.FHKKMPDDMGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEKKNDONHLJ",
            |m: &BKPPACMANBM| { &m.GEKKNDONHLJ },
            |m: &mut BKPPACMANBM| { &mut m.GEKKNDONHLJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EHBDEIJJOHK",
            |m: &BKPPACMANBM| { &m.EHBDEIJJOHK },
            |m: &mut BKPPACMANBM| { &mut m.EHBDEIJJOHK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BKPPACMANBM>(
            "BKPPACMANBM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BKPPACMANBM {
    const NAME: &'static str = "BKPPACMANBM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.BHFEFEODNIM = is.read_uint32()?;
                },
                32 => {
                    self.HKNOAKGCJBK = is.read_uint32()?;
                },
                40 => {
                    self.EMJDEBDMHLL = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KFJCNGBHAGK)?;
                },
                80 => {
                    self.JFPGBKBPBNF = is.read_uint32()?;
                },
                104 => {
                    self.CFDFMGLLICO = is.read_uint32()?;
                },
                120 => {
                    self.FHKKMPDDMGO = is.read_uint32()?;
                },
                48 => {
                    self.GEKKNDONHLJ = is.read_uint32()?;
                },
                96 => {
                    self.EHBDEIJJOHK = is.read_uint32()?;
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
        if self.BHFEFEODNIM != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BHFEFEODNIM);
        }
        if self.HKNOAKGCJBK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.HKNOAKGCJBK);
        }
        if self.EMJDEBDMHLL != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EMJDEBDMHLL);
        }
        if let Some(v) = self.KFJCNGBHAGK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.JFPGBKBPBNF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.JFPGBKBPBNF);
        }
        if self.CFDFMGLLICO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CFDFMGLLICO);
        }
        if self.FHKKMPDDMGO != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.FHKKMPDDMGO);
        }
        if self.GEKKNDONHLJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GEKKNDONHLJ);
        }
        if self.EHBDEIJJOHK != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.EHBDEIJJOHK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BHFEFEODNIM != 0 {
            os.write_uint32(3, self.BHFEFEODNIM)?;
        }
        if self.HKNOAKGCJBK != 0 {
            os.write_uint32(4, self.HKNOAKGCJBK)?;
        }
        if self.EMJDEBDMHLL != 0 {
            os.write_uint32(5, self.EMJDEBDMHLL)?;
        }
        if let Some(v) = self.KFJCNGBHAGK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.JFPGBKBPBNF != 0 {
            os.write_uint32(10, self.JFPGBKBPBNF)?;
        }
        if self.CFDFMGLLICO != 0 {
            os.write_uint32(13, self.CFDFMGLLICO)?;
        }
        if self.FHKKMPDDMGO != 0 {
            os.write_uint32(15, self.FHKKMPDDMGO)?;
        }
        if self.GEKKNDONHLJ != 0 {
            os.write_uint32(6, self.GEKKNDONHLJ)?;
        }
        if self.EHBDEIJJOHK != 0 {
            os.write_uint32(12, self.EHBDEIJJOHK)?;
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

    fn new() -> BKPPACMANBM {
        BKPPACMANBM::new()
    }

    fn clear(&mut self) {
        self.BHFEFEODNIM = 0;
        self.HKNOAKGCJBK = 0;
        self.EMJDEBDMHLL = 0;
        self.KFJCNGBHAGK.clear();
        self.JFPGBKBPBNF = 0;
        self.CFDFMGLLICO = 0;
        self.FHKKMPDDMGO = 0;
        self.GEKKNDONHLJ = 0;
        self.EHBDEIJJOHK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BKPPACMANBM {
        static instance: BKPPACMANBM = BKPPACMANBM {
            BHFEFEODNIM: 0,
            HKNOAKGCJBK: 0,
            EMJDEBDMHLL: 0,
            KFJCNGBHAGK: ::protobuf::MessageField::none(),
            JFPGBKBPBNF: 0,
            CFDFMGLLICO: 0,
            FHKKMPDDMGO: 0,
            GEKKNDONHLJ: 0,
            EHBDEIJJOHK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BKPPACMANBM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BKPPACMANBM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BKPPACMANBM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BKPPACMANBM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BKPPACMANBM.proto\x1a\x11LPJOLCINFDN.proto\"\xcd\x02\n\x0bBKPPACMA\
    NBM\x12\x20\n\x0bBHFEFEODNIM\x18\x03\x20\x01(\rR\x0bBHFEFEODNIM\x12\x20\
    \n\x0bHKNOAKGCJBK\x18\x04\x20\x01(\rR\x0bHKNOAKGCJBK\x12\x20\n\x0bEMJDEB\
    DMHLL\x18\x05\x20\x01(\rR\x0bEMJDEBDMHLL\x12.\n\x0bKFJCNGBHAGK\x18\x02\
    \x20\x01(\x0b2\x0c.LPJOLCINFDNR\x0bKFJCNGBHAGK\x12\x20\n\x0bJFPGBKBPBNF\
    \x18\n\x20\x01(\rR\x0bJFPGBKBPBNF\x12\x20\n\x0bCFDFMGLLICO\x18\r\x20\x01\
    (\rR\x0bCFDFMGLLICO\x12\x20\n\x0bFHKKMPDDMGO\x18\x0f\x20\x01(\rR\x0bFHKK\
    MPDDMGO\x12\x20\n\x0bGEKKNDONHLJ\x18\x06\x20\x01(\rR\x0bGEKKNDONHLJ\x12\
    \x20\n\x0bEHBDEIJJOHK\x18\x0c\x20\x01(\rR\x0bEHBDEIJJOHKb\x06proto3\
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
            deps.push(super::LPJOLCINFDN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BKPPACMANBM::generated_message_descriptor_data());
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
