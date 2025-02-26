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

//! Generated file from `PJEBCBNPDIC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PJEBCBNPDIC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PJEBCBNPDIC {
    // message fields
    // @@protoc_insertion_point(field:PJEBCBNPDIC.HOLALCIPKNK)
    pub HOLALCIPKNK: ::protobuf::MessageField<super::OLFGBAMEFJI::OLFGBAMEFJI>,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.KNBDPFEIDNM)
    pub KNBDPFEIDNM: bool,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.NPBNMMKHKOP)
    pub NPBNMMKHKOP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.DBAHFEFGLMD)
    pub DBAHFEFGLMD: u32,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.KGJHJMDCAOC)
    pub KGJHJMDCAOC: bool,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.IFPOILOPFAG)
    pub IFPOILOPFAG: u32,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.CGAIJCCLKBH)
    pub CGAIJCCLKBH: ::protobuf::MessageField<super::KOIICMIEAEF::KOIICMIEAEF>,
    // @@protoc_insertion_point(field:PJEBCBNPDIC.BIIFELFEGNK)
    pub BIIFELFEGNK: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PJEBCBNPDIC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PJEBCBNPDIC {
    fn default() -> &'a PJEBCBNPDIC {
        <PJEBCBNPDIC as ::protobuf::Message>::default_instance()
    }
}

impl PJEBCBNPDIC {
    pub fn new() -> PJEBCBNPDIC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OLFGBAMEFJI::OLFGBAMEFJI>(
            "HOLALCIPKNK",
            |m: &PJEBCBNPDIC| { &m.HOLALCIPKNK },
            |m: &mut PJEBCBNPDIC| { &mut m.HOLALCIPKNK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KNBDPFEIDNM",
            |m: &PJEBCBNPDIC| { &m.KNBDPFEIDNM },
            |m: &mut PJEBCBNPDIC| { &mut m.KNBDPFEIDNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NPBNMMKHKOP",
            |m: &PJEBCBNPDIC| { &m.NPBNMMKHKOP },
            |m: &mut PJEBCBNPDIC| { &mut m.NPBNMMKHKOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBAHFEFGLMD",
            |m: &PJEBCBNPDIC| { &m.DBAHFEFGLMD },
            |m: &mut PJEBCBNPDIC| { &mut m.DBAHFEFGLMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGJHJMDCAOC",
            |m: &PJEBCBNPDIC| { &m.KGJHJMDCAOC },
            |m: &mut PJEBCBNPDIC| { &mut m.KGJHJMDCAOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFPOILOPFAG",
            |m: &PJEBCBNPDIC| { &m.IFPOILOPFAG },
            |m: &mut PJEBCBNPDIC| { &mut m.IFPOILOPFAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KOIICMIEAEF::KOIICMIEAEF>(
            "CGAIJCCLKBH",
            |m: &PJEBCBNPDIC| { &m.CGAIJCCLKBH },
            |m: &mut PJEBCBNPDIC| { &mut m.CGAIJCCLKBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIIFELFEGNK",
            |m: &PJEBCBNPDIC| { &m.BIIFELFEGNK },
            |m: &mut PJEBCBNPDIC| { &mut m.BIIFELFEGNK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PJEBCBNPDIC>(
            "PJEBCBNPDIC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PJEBCBNPDIC {
    const NAME: &'static str = "PJEBCBNPDIC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HOLALCIPKNK)?;
                },
                24 => {
                    self.KNBDPFEIDNM = is.read_bool()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.NPBNMMKHKOP)?;
                },
                96 => {
                    self.NPBNMMKHKOP.push(is.read_uint32()?);
                },
                32 => {
                    self.DBAHFEFGLMD = is.read_uint32()?;
                },
                72 => {
                    self.KGJHJMDCAOC = is.read_bool()?;
                },
                112 => {
                    self.IFPOILOPFAG = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CGAIJCCLKBH)?;
                },
                120 => {
                    self.BIIFELFEGNK = is.read_bool()?;
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
        if let Some(v) = self.HOLALCIPKNK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KNBDPFEIDNM != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.NPBNMMKHKOP);
        if self.DBAHFEFGLMD != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DBAHFEFGLMD);
        }
        if self.KGJHJMDCAOC != false {
            my_size += 1 + 1;
        }
        if self.IFPOILOPFAG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IFPOILOPFAG);
        }
        if let Some(v) = self.CGAIJCCLKBH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BIIFELFEGNK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.HOLALCIPKNK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.KNBDPFEIDNM != false {
            os.write_bool(3, self.KNBDPFEIDNM)?;
        }
        os.write_repeated_packed_uint32(12, &self.NPBNMMKHKOP)?;
        if self.DBAHFEFGLMD != 0 {
            os.write_uint32(4, self.DBAHFEFGLMD)?;
        }
        if self.KGJHJMDCAOC != false {
            os.write_bool(9, self.KGJHJMDCAOC)?;
        }
        if self.IFPOILOPFAG != 0 {
            os.write_uint32(14, self.IFPOILOPFAG)?;
        }
        if let Some(v) = self.CGAIJCCLKBH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.BIIFELFEGNK != false {
            os.write_bool(15, self.BIIFELFEGNK)?;
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

    fn new() -> PJEBCBNPDIC {
        PJEBCBNPDIC::new()
    }

    fn clear(&mut self) {
        self.HOLALCIPKNK.clear();
        self.KNBDPFEIDNM = false;
        self.NPBNMMKHKOP.clear();
        self.DBAHFEFGLMD = 0;
        self.KGJHJMDCAOC = false;
        self.IFPOILOPFAG = 0;
        self.CGAIJCCLKBH.clear();
        self.BIIFELFEGNK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PJEBCBNPDIC {
        static instance: PJEBCBNPDIC = PJEBCBNPDIC {
            HOLALCIPKNK: ::protobuf::MessageField::none(),
            KNBDPFEIDNM: false,
            NPBNMMKHKOP: ::std::vec::Vec::new(),
            DBAHFEFGLMD: 0,
            KGJHJMDCAOC: false,
            IFPOILOPFAG: 0,
            CGAIJCCLKBH: ::protobuf::MessageField::none(),
            BIIFELFEGNK: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PJEBCBNPDIC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PJEBCBNPDIC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PJEBCBNPDIC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PJEBCBNPDIC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PJEBCBNPDIC.proto\x1a\x11KOIICMIEAEF.proto\x1a\x11OLFGBAMEFJI.prot\
    o\"\xb9\x02\n\x0bPJEBCBNPDIC\x12.\n\x0bHOLALCIPKNK\x18\n\x20\x01(\x0b2\
    \x0c.OLFGBAMEFJIR\x0bHOLALCIPKNK\x12\x20\n\x0bKNBDPFEIDNM\x18\x03\x20\
    \x01(\x08R\x0bKNBDPFEIDNM\x12\x20\n\x0bNPBNMMKHKOP\x18\x0c\x20\x03(\rR\
    \x0bNPBNMMKHKOP\x12\x20\n\x0bDBAHFEFGLMD\x18\x04\x20\x01(\rR\x0bDBAHFEFG\
    LMD\x12\x20\n\x0bKGJHJMDCAOC\x18\t\x20\x01(\x08R\x0bKGJHJMDCAOC\x12\x20\
    \n\x0bIFPOILOPFAG\x18\x0e\x20\x01(\rR\x0bIFPOILOPFAG\x12.\n\x0bCGAIJCCLK\
    BH\x18\x0b\x20\x01(\x0b2\x0c.KOIICMIEAEFR\x0bCGAIJCCLKBH\x12\x20\n\x0bBI\
    IFELFEGNK\x18\x0f\x20\x01(\x08R\x0bBIIFELFEGNKb\x06proto3\
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
            deps.push(super::KOIICMIEAEF::file_descriptor().clone());
            deps.push(super::OLFGBAMEFJI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PJEBCBNPDIC::generated_message_descriptor_data());
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
