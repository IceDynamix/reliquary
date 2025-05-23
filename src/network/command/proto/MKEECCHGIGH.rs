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

//! Generated file from `MKEECCHGIGH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MKEECCHGIGH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MKEECCHGIGH {
    // message fields
    // @@protoc_insertion_point(field:MKEECCHGIGH.NMIMBIOPEKI)
    pub NMIMBIOPEKI: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.KKANCJALJPO)
    pub KKANCJALJPO: f64,
    // @@protoc_insertion_point(field:MKEECCHGIGH.MFJKFLGPGKO)
    pub MFJKFLGPGKO: f64,
    // @@protoc_insertion_point(field:MKEECCHGIGH.FGMLCKANIAN)
    pub FGMLCKANIAN: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.GOAEBJJPAJO)
    pub GOAEBJJPAJO: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.NILAKIDFHEJ)
    pub NILAKIDFHEJ: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.PIGNDAJGDGJ)
    pub PIGNDAJGDGJ: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.ABMNLNNOKLO)
    pub ABMNLNNOKLO: f64,
    // @@protoc_insertion_point(field:MKEECCHGIGH.JBJMOPHGMFA)
    pub JBJMOPHGMFA: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.POBIBILOANI)
    pub POBIBILOANI: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.IHBBEKCOEAE)
    pub IHBBEKCOEAE: f64,
    // @@protoc_insertion_point(field:MKEECCHGIGH.LOOLLAGMNLH)
    pub LOOLLAGMNLH: u32,
    // @@protoc_insertion_point(field:MKEECCHGIGH.FKEAAIPKPAA)
    pub FKEAAIPKPAA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MKEECCHGIGH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MKEECCHGIGH {
    fn default() -> &'a MKEECCHGIGH {
        <MKEECCHGIGH as ::protobuf::Message>::default_instance()
    }
}

impl MKEECCHGIGH {
    pub fn new() -> MKEECCHGIGH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMIMBIOPEKI",
            |m: &MKEECCHGIGH| { &m.NMIMBIOPEKI },
            |m: &mut MKEECCHGIGH| { &mut m.NMIMBIOPEKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KKANCJALJPO",
            |m: &MKEECCHGIGH| { &m.KKANCJALJPO },
            |m: &mut MKEECCHGIGH| { &mut m.KKANCJALJPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MFJKFLGPGKO",
            |m: &MKEECCHGIGH| { &m.MFJKFLGPGKO },
            |m: &mut MKEECCHGIGH| { &mut m.MFJKFLGPGKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGMLCKANIAN",
            |m: &MKEECCHGIGH| { &m.FGMLCKANIAN },
            |m: &mut MKEECCHGIGH| { &mut m.FGMLCKANIAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOAEBJJPAJO",
            |m: &MKEECCHGIGH| { &m.GOAEBJJPAJO },
            |m: &mut MKEECCHGIGH| { &mut m.GOAEBJJPAJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NILAKIDFHEJ",
            |m: &MKEECCHGIGH| { &m.NILAKIDFHEJ },
            |m: &mut MKEECCHGIGH| { &mut m.NILAKIDFHEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIGNDAJGDGJ",
            |m: &MKEECCHGIGH| { &m.PIGNDAJGDGJ },
            |m: &mut MKEECCHGIGH| { &mut m.PIGNDAJGDGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ABMNLNNOKLO",
            |m: &MKEECCHGIGH| { &m.ABMNLNNOKLO },
            |m: &mut MKEECCHGIGH| { &mut m.ABMNLNNOKLO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JBJMOPHGMFA",
            |m: &MKEECCHGIGH| { &m.JBJMOPHGMFA },
            |m: &mut MKEECCHGIGH| { &mut m.JBJMOPHGMFA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POBIBILOANI",
            |m: &MKEECCHGIGH| { &m.POBIBILOANI },
            |m: &mut MKEECCHGIGH| { &mut m.POBIBILOANI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHBBEKCOEAE",
            |m: &MKEECCHGIGH| { &m.IHBBEKCOEAE },
            |m: &mut MKEECCHGIGH| { &mut m.IHBBEKCOEAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOOLLAGMNLH",
            |m: &MKEECCHGIGH| { &m.LOOLLAGMNLH },
            |m: &mut MKEECCHGIGH| { &mut m.LOOLLAGMNLH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKEAAIPKPAA",
            |m: &MKEECCHGIGH| { &m.FKEAAIPKPAA },
            |m: &mut MKEECCHGIGH| { &mut m.FKEAAIPKPAA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MKEECCHGIGH>(
            "MKEECCHGIGH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MKEECCHGIGH {
    const NAME: &'static str = "MKEECCHGIGH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.NMIMBIOPEKI = is.read_uint32()?;
                },
                17 => {
                    self.KKANCJALJPO = is.read_double()?;
                },
                25 => {
                    self.MFJKFLGPGKO = is.read_double()?;
                },
                32 => {
                    self.FGMLCKANIAN = is.read_uint32()?;
                },
                40 => {
                    self.GOAEBJJPAJO = is.read_uint32()?;
                },
                48 => {
                    self.NILAKIDFHEJ = is.read_uint32()?;
                },
                56 => {
                    self.PIGNDAJGDGJ = is.read_uint32()?;
                },
                65 => {
                    self.ABMNLNNOKLO = is.read_double()?;
                },
                72 => {
                    self.JBJMOPHGMFA = is.read_uint32()?;
                },
                80 => {
                    self.POBIBILOANI = is.read_uint32()?;
                },
                89 => {
                    self.IHBBEKCOEAE = is.read_double()?;
                },
                96 => {
                    self.LOOLLAGMNLH = is.read_uint32()?;
                },
                104 => {
                    self.FKEAAIPKPAA = is.read_uint32()?;
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
        if self.NMIMBIOPEKI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.NMIMBIOPEKI);
        }
        if self.KKANCJALJPO != 0. {
            my_size += 1 + 8;
        }
        if self.MFJKFLGPGKO != 0. {
            my_size += 1 + 8;
        }
        if self.FGMLCKANIAN != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FGMLCKANIAN);
        }
        if self.GOAEBJJPAJO != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GOAEBJJPAJO);
        }
        if self.NILAKIDFHEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NILAKIDFHEJ);
        }
        if self.PIGNDAJGDGJ != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PIGNDAJGDGJ);
        }
        if self.ABMNLNNOKLO != 0. {
            my_size += 1 + 8;
        }
        if self.JBJMOPHGMFA != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.JBJMOPHGMFA);
        }
        if self.POBIBILOANI != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.POBIBILOANI);
        }
        if self.IHBBEKCOEAE != 0. {
            my_size += 1 + 8;
        }
        if self.LOOLLAGMNLH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LOOLLAGMNLH);
        }
        if self.FKEAAIPKPAA != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.FKEAAIPKPAA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NMIMBIOPEKI != 0 {
            os.write_uint32(1, self.NMIMBIOPEKI)?;
        }
        if self.KKANCJALJPO != 0. {
            os.write_double(2, self.KKANCJALJPO)?;
        }
        if self.MFJKFLGPGKO != 0. {
            os.write_double(3, self.MFJKFLGPGKO)?;
        }
        if self.FGMLCKANIAN != 0 {
            os.write_uint32(4, self.FGMLCKANIAN)?;
        }
        if self.GOAEBJJPAJO != 0 {
            os.write_uint32(5, self.GOAEBJJPAJO)?;
        }
        if self.NILAKIDFHEJ != 0 {
            os.write_uint32(6, self.NILAKIDFHEJ)?;
        }
        if self.PIGNDAJGDGJ != 0 {
            os.write_uint32(7, self.PIGNDAJGDGJ)?;
        }
        if self.ABMNLNNOKLO != 0. {
            os.write_double(8, self.ABMNLNNOKLO)?;
        }
        if self.JBJMOPHGMFA != 0 {
            os.write_uint32(9, self.JBJMOPHGMFA)?;
        }
        if self.POBIBILOANI != 0 {
            os.write_uint32(10, self.POBIBILOANI)?;
        }
        if self.IHBBEKCOEAE != 0. {
            os.write_double(11, self.IHBBEKCOEAE)?;
        }
        if self.LOOLLAGMNLH != 0 {
            os.write_uint32(12, self.LOOLLAGMNLH)?;
        }
        if self.FKEAAIPKPAA != 0 {
            os.write_uint32(13, self.FKEAAIPKPAA)?;
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

    fn new() -> MKEECCHGIGH {
        MKEECCHGIGH::new()
    }

    fn clear(&mut self) {
        self.NMIMBIOPEKI = 0;
        self.KKANCJALJPO = 0.;
        self.MFJKFLGPGKO = 0.;
        self.FGMLCKANIAN = 0;
        self.GOAEBJJPAJO = 0;
        self.NILAKIDFHEJ = 0;
        self.PIGNDAJGDGJ = 0;
        self.ABMNLNNOKLO = 0.;
        self.JBJMOPHGMFA = 0;
        self.POBIBILOANI = 0;
        self.IHBBEKCOEAE = 0.;
        self.LOOLLAGMNLH = 0;
        self.FKEAAIPKPAA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MKEECCHGIGH {
        static instance: MKEECCHGIGH = MKEECCHGIGH {
            NMIMBIOPEKI: 0,
            KKANCJALJPO: 0.,
            MFJKFLGPGKO: 0.,
            FGMLCKANIAN: 0,
            GOAEBJJPAJO: 0,
            NILAKIDFHEJ: 0,
            PIGNDAJGDGJ: 0,
            ABMNLNNOKLO: 0.,
            JBJMOPHGMFA: 0,
            POBIBILOANI: 0,
            IHBBEKCOEAE: 0.,
            LOOLLAGMNLH: 0,
            FKEAAIPKPAA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MKEECCHGIGH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MKEECCHGIGH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MKEECCHGIGH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MKEECCHGIGH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MKEECCHGIGH.proto\"\xc7\x03\n\x0bMKEECCHGIGH\x12\x20\n\x0bNMIMBIOP\
    EKI\x18\x01\x20\x01(\rR\x0bNMIMBIOPEKI\x12\x20\n\x0bKKANCJALJPO\x18\x02\
    \x20\x01(\x01R\x0bKKANCJALJPO\x12\x20\n\x0bMFJKFLGPGKO\x18\x03\x20\x01(\
    \x01R\x0bMFJKFLGPGKO\x12\x20\n\x0bFGMLCKANIAN\x18\x04\x20\x01(\rR\x0bFGM\
    LCKANIAN\x12\x20\n\x0bGOAEBJJPAJO\x18\x05\x20\x01(\rR\x0bGOAEBJJPAJO\x12\
    \x20\n\x0bNILAKIDFHEJ\x18\x06\x20\x01(\rR\x0bNILAKIDFHEJ\x12\x20\n\x0bPI\
    GNDAJGDGJ\x18\x07\x20\x01(\rR\x0bPIGNDAJGDGJ\x12\x20\n\x0bABMNLNNOKLO\
    \x18\x08\x20\x01(\x01R\x0bABMNLNNOKLO\x12\x20\n\x0bJBJMOPHGMFA\x18\t\x20\
    \x01(\rR\x0bJBJMOPHGMFA\x12\x20\n\x0bPOBIBILOANI\x18\n\x20\x01(\rR\x0bPO\
    BIBILOANI\x12\x20\n\x0bIHBBEKCOEAE\x18\x0b\x20\x01(\x01R\x0bIHBBEKCOEAE\
    \x12\x20\n\x0bLOOLLAGMNLH\x18\x0c\x20\x01(\rR\x0bLOOLLAGMNLH\x12\x20\n\
    \x0bFKEAAIPKPAA\x18\r\x20\x01(\rR\x0bFKEAAIPKPAAb\x06proto3\
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
            messages.push(MKEECCHGIGH::generated_message_descriptor_data());
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
