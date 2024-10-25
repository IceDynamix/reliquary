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

//! Generated file from `BODMNHNOALH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BODMNHNOALH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BODMNHNOALH {
    // message fields
    // @@protoc_insertion_point(field:BODMNHNOALH.GKONJAMICJM)
    pub GKONJAMICJM: bool,
    // @@protoc_insertion_point(field:BODMNHNOALH.KHGJLJHPLPM)
    pub KHGJLJHPLPM: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.ALBIGNGFBFO)
    pub ALBIGNGFBFO: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.PKEJEOEBBJG)
    pub PKEJEOEBBJG: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.PKIMFMOGBBI)
    pub PKIMFMOGBBI: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.NGKABPLAEGP)
    pub NGKABPLAEGP: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.KODGNCJFGEL)
    pub KODGNCJFGEL: u32,
    // @@protoc_insertion_point(field:BODMNHNOALH.IJEPENGMPLG)
    pub IJEPENGMPLG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BODMNHNOALH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BODMNHNOALH {
    fn default() -> &'a BODMNHNOALH {
        <BODMNHNOALH as ::protobuf::Message>::default_instance()
    }
}

impl BODMNHNOALH {
    pub fn new() -> BODMNHNOALH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GKONJAMICJM",
            |m: &BODMNHNOALH| { &m.GKONJAMICJM },
            |m: &mut BODMNHNOALH| { &mut m.GKONJAMICJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHGJLJHPLPM",
            |m: &BODMNHNOALH| { &m.KHGJLJHPLPM },
            |m: &mut BODMNHNOALH| { &mut m.KHGJLJHPLPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ALBIGNGFBFO",
            |m: &BODMNHNOALH| { &m.ALBIGNGFBFO },
            |m: &mut BODMNHNOALH| { &mut m.ALBIGNGFBFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PKEJEOEBBJG",
            |m: &BODMNHNOALH| { &m.PKEJEOEBBJG },
            |m: &mut BODMNHNOALH| { &mut m.PKEJEOEBBJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PKIMFMOGBBI",
            |m: &BODMNHNOALH| { &m.PKIMFMOGBBI },
            |m: &mut BODMNHNOALH| { &mut m.PKIMFMOGBBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NGKABPLAEGP",
            |m: &BODMNHNOALH| { &m.NGKABPLAEGP },
            |m: &mut BODMNHNOALH| { &mut m.NGKABPLAEGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KODGNCJFGEL",
            |m: &BODMNHNOALH| { &m.KODGNCJFGEL },
            |m: &mut BODMNHNOALH| { &mut m.KODGNCJFGEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJEPENGMPLG",
            |m: &BODMNHNOALH| { &m.IJEPENGMPLG },
            |m: &mut BODMNHNOALH| { &mut m.IJEPENGMPLG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BODMNHNOALH>(
            "BODMNHNOALH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BODMNHNOALH {
    const NAME: &'static str = "BODMNHNOALH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.GKONJAMICJM = is.read_bool()?;
                },
                16 => {
                    self.KHGJLJHPLPM = is.read_uint32()?;
                },
                32 => {
                    self.ALBIGNGFBFO = is.read_uint32()?;
                },
                88 => {
                    self.PKEJEOEBBJG = is.read_uint32()?;
                },
                64 => {
                    self.PKIMFMOGBBI = is.read_uint32()?;
                },
                48 => {
                    self.NGKABPLAEGP = is.read_uint32()?;
                },
                96 => {
                    self.KODGNCJFGEL = is.read_uint32()?;
                },
                40 => {
                    self.IJEPENGMPLG = is.read_uint32()?;
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
        if self.GKONJAMICJM != false {
            my_size += 1 + 1;
        }
        if self.KHGJLJHPLPM != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.KHGJLJHPLPM);
        }
        if self.ALBIGNGFBFO != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.ALBIGNGFBFO);
        }
        if self.PKEJEOEBBJG != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.PKEJEOEBBJG);
        }
        if self.PKIMFMOGBBI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PKIMFMOGBBI);
        }
        if self.NGKABPLAEGP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NGKABPLAEGP);
        }
        if self.KODGNCJFGEL != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.KODGNCJFGEL);
        }
        if self.IJEPENGMPLG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.IJEPENGMPLG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GKONJAMICJM != false {
            os.write_bool(15, self.GKONJAMICJM)?;
        }
        if self.KHGJLJHPLPM != 0 {
            os.write_uint32(2, self.KHGJLJHPLPM)?;
        }
        if self.ALBIGNGFBFO != 0 {
            os.write_uint32(4, self.ALBIGNGFBFO)?;
        }
        if self.PKEJEOEBBJG != 0 {
            os.write_uint32(11, self.PKEJEOEBBJG)?;
        }
        if self.PKIMFMOGBBI != 0 {
            os.write_uint32(8, self.PKIMFMOGBBI)?;
        }
        if self.NGKABPLAEGP != 0 {
            os.write_uint32(6, self.NGKABPLAEGP)?;
        }
        if self.KODGNCJFGEL != 0 {
            os.write_uint32(12, self.KODGNCJFGEL)?;
        }
        if self.IJEPENGMPLG != 0 {
            os.write_uint32(5, self.IJEPENGMPLG)?;
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

    fn new() -> BODMNHNOALH {
        BODMNHNOALH::new()
    }

    fn clear(&mut self) {
        self.GKONJAMICJM = false;
        self.KHGJLJHPLPM = 0;
        self.ALBIGNGFBFO = 0;
        self.PKEJEOEBBJG = 0;
        self.PKIMFMOGBBI = 0;
        self.NGKABPLAEGP = 0;
        self.KODGNCJFGEL = 0;
        self.IJEPENGMPLG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BODMNHNOALH {
        static instance: BODMNHNOALH = BODMNHNOALH {
            GKONJAMICJM: false,
            KHGJLJHPLPM: 0,
            ALBIGNGFBFO: 0,
            PKEJEOEBBJG: 0,
            PKIMFMOGBBI: 0,
            NGKABPLAEGP: 0,
            KODGNCJFGEL: 0,
            IJEPENGMPLG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BODMNHNOALH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BODMNHNOALH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BODMNHNOALH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BODMNHNOALH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BODMNHNOALH.proto\"\x9d\x02\n\x0bBODMNHNOALH\x12\x20\n\x0bGKONJAMI\
    CJM\x18\x0f\x20\x01(\x08R\x0bGKONJAMICJM\x12\x20\n\x0bKHGJLJHPLPM\x18\
    \x02\x20\x01(\rR\x0bKHGJLJHPLPM\x12\x20\n\x0bALBIGNGFBFO\x18\x04\x20\x01\
    (\rR\x0bALBIGNGFBFO\x12\x20\n\x0bPKEJEOEBBJG\x18\x0b\x20\x01(\rR\x0bPKEJ\
    EOEBBJG\x12\x20\n\x0bPKIMFMOGBBI\x18\x08\x20\x01(\rR\x0bPKIMFMOGBBI\x12\
    \x20\n\x0bNGKABPLAEGP\x18\x06\x20\x01(\rR\x0bNGKABPLAEGP\x12\x20\n\x0bKO\
    DGNCJFGEL\x18\x0c\x20\x01(\rR\x0bKODGNCJFGEL\x12\x20\n\x0bIJEPENGMPLG\
    \x18\x05\x20\x01(\rR\x0bIJEPENGMPLGb\x06proto3\
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
            messages.push(BODMNHNOALH::generated_message_descriptor_data());
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
