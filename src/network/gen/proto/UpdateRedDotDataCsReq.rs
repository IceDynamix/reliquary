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

//! Generated file from `UpdateRedDotDataCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdateRedDotDataCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateRedDotDataCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.EMGFKILDLKC)
    pub EMGFKILDLKC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.MDIABNLNKJJ)
    pub MDIABNLNKJJ: u32,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.JONNHKHBKAA)
    pub JONNHKHBKAA: ::protobuf::EnumOrUnknown<super::IBIECKLCAAL::IBIECKLCAAL>,
    // @@protoc_insertion_point(field:UpdateRedDotDataCsReq.ACPLFDCNGKO)
    pub ACPLFDCNGKO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateRedDotDataCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateRedDotDataCsReq {
    fn default() -> &'a UpdateRedDotDataCsReq {
        <UpdateRedDotDataCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateRedDotDataCsReq {
    pub fn new() -> UpdateRedDotDataCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMGFKILDLKC",
            |m: &UpdateRedDotDataCsReq| { &m.EMGFKILDLKC },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.EMGFKILDLKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &UpdateRedDotDataCsReq| { &m.GCFIIGOLPMF },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDIABNLNKJJ",
            |m: &UpdateRedDotDataCsReq| { &m.MDIABNLNKJJ },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.MDIABNLNKJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JONNHKHBKAA",
            |m: &UpdateRedDotDataCsReq| { &m.JONNHKHBKAA },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.JONNHKHBKAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACPLFDCNGKO",
            |m: &UpdateRedDotDataCsReq| { &m.ACPLFDCNGKO },
            |m: &mut UpdateRedDotDataCsReq| { &mut m.ACPLFDCNGKO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateRedDotDataCsReq>(
            "UpdateRedDotDataCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateRedDotDataCsReq {
    const NAME: &'static str = "UpdateRedDotDataCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.EMGFKILDLKC)?;
                },
                88 => {
                    self.EMGFKILDLKC.push(is.read_uint32()?);
                },
                56 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                8 => {
                    self.MDIABNLNKJJ = is.read_uint32()?;
                },
                104 => {
                    self.JONNHKHBKAA = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.ACPLFDCNGKO = is.read_uint32()?;
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
        for value in &self.EMGFKILDLKC {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.GCFIIGOLPMF);
        }
        if self.MDIABNLNKJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MDIABNLNKJJ);
        }
        if self.JONNHKHBKAA != ::protobuf::EnumOrUnknown::new(super::IBIECKLCAAL::IBIECKLCAAL::UPDATE_REDDOT_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.JONNHKHBKAA.value());
        }
        if self.ACPLFDCNGKO != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ACPLFDCNGKO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EMGFKILDLKC {
            os.write_uint32(11, *v)?;
        };
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(7, self.GCFIIGOLPMF)?;
        }
        if self.MDIABNLNKJJ != 0 {
            os.write_uint32(1, self.MDIABNLNKJJ)?;
        }
        if self.JONNHKHBKAA != ::protobuf::EnumOrUnknown::new(super::IBIECKLCAAL::IBIECKLCAAL::UPDATE_REDDOT_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.JONNHKHBKAA))?;
        }
        if self.ACPLFDCNGKO != 0 {
            os.write_uint32(9, self.ACPLFDCNGKO)?;
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

    fn new() -> UpdateRedDotDataCsReq {
        UpdateRedDotDataCsReq::new()
    }

    fn clear(&mut self) {
        self.EMGFKILDLKC.clear();
        self.GCFIIGOLPMF = 0;
        self.MDIABNLNKJJ = 0;
        self.JONNHKHBKAA = ::protobuf::EnumOrUnknown::new(super::IBIECKLCAAL::IBIECKLCAAL::UPDATE_REDDOT_NONE);
        self.ACPLFDCNGKO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateRedDotDataCsReq {
        static instance: UpdateRedDotDataCsReq = UpdateRedDotDataCsReq {
            EMGFKILDLKC: ::std::vec::Vec::new(),
            GCFIIGOLPMF: 0,
            MDIABNLNKJJ: 0,
            JONNHKHBKAA: ::protobuf::EnumOrUnknown::from_i32(0),
            ACPLFDCNGKO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateRedDotDataCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateRedDotDataCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateRedDotDataCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateRedDotDataCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bUpdateRedDotDataCsReq.proto\x1a\x11IBIECKLCAAL.proto\"\xcf\x01\n\
    \x15UpdateRedDotDataCsReq\x12\x20\n\x0bEMGFKILDLKC\x18\x0b\x20\x03(\rR\
    \x0bEMGFKILDLKC\x12\x20\n\x0bGCFIIGOLPMF\x18\x07\x20\x01(\rR\x0bGCFIIGOL\
    PMF\x12\x20\n\x0bMDIABNLNKJJ\x18\x01\x20\x01(\rR\x0bMDIABNLNKJJ\x12.\n\
    \x0bJONNHKHBKAA\x18\r\x20\x01(\x0e2\x0c.IBIECKLCAALR\x0bJONNHKHBKAA\x12\
    \x20\n\x0bACPLFDCNGKO\x18\t\x20\x01(\rR\x0bACPLFDCNGKOb\x06proto3\
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
            deps.push(super::IBIECKLCAAL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateRedDotDataCsReq::generated_message_descriptor_data());
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
