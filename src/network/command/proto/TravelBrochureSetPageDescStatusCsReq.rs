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

//! Generated file from `TravelBrochureSetPageDescStatusCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TravelBrochureSetPageDescStatusCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TravelBrochureSetPageDescStatusCsReq {
    // message fields
    // @@protoc_insertion_point(field:TravelBrochureSetPageDescStatusCsReq.CPODEJOFPDD)
    pub CPODEJOFPDD: u32,
    // @@protoc_insertion_point(field:TravelBrochureSetPageDescStatusCsReq.GEIBGFDENJA)
    pub GEIBGFDENJA: ::protobuf::EnumOrUnknown<super::DCJAOPDINOI::DCJAOPDINOI>,
    // special fields
    // @@protoc_insertion_point(special_field:TravelBrochureSetPageDescStatusCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TravelBrochureSetPageDescStatusCsReq {
    fn default() -> &'a TravelBrochureSetPageDescStatusCsReq {
        <TravelBrochureSetPageDescStatusCsReq as ::protobuf::Message>::default_instance()
    }
}

impl TravelBrochureSetPageDescStatusCsReq {
    pub fn new() -> TravelBrochureSetPageDescStatusCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPODEJOFPDD",
            |m: &TravelBrochureSetPageDescStatusCsReq| { &m.CPODEJOFPDD },
            |m: &mut TravelBrochureSetPageDescStatusCsReq| { &mut m.CPODEJOFPDD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEIBGFDENJA",
            |m: &TravelBrochureSetPageDescStatusCsReq| { &m.GEIBGFDENJA },
            |m: &mut TravelBrochureSetPageDescStatusCsReq| { &mut m.GEIBGFDENJA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TravelBrochureSetPageDescStatusCsReq>(
            "TravelBrochureSetPageDescStatusCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TravelBrochureSetPageDescStatusCsReq {
    const NAME: &'static str = "TravelBrochureSetPageDescStatusCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.CPODEJOFPDD = is.read_uint32()?;
                },
                120 => {
                    self.GEIBGFDENJA = is.read_enum_or_unknown()?;
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
        if self.CPODEJOFPDD != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.CPODEJOFPDD);
        }
        if self.GEIBGFDENJA != ::protobuf::EnumOrUnknown::new(super::DCJAOPDINOI::DCJAOPDINOI::PAGE_DESC_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.GEIBGFDENJA.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CPODEJOFPDD != 0 {
            os.write_uint32(6, self.CPODEJOFPDD)?;
        }
        if self.GEIBGFDENJA != ::protobuf::EnumOrUnknown::new(super::DCJAOPDINOI::DCJAOPDINOI::PAGE_DESC_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.GEIBGFDENJA))?;
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

    fn new() -> TravelBrochureSetPageDescStatusCsReq {
        TravelBrochureSetPageDescStatusCsReq::new()
    }

    fn clear(&mut self) {
        self.CPODEJOFPDD = 0;
        self.GEIBGFDENJA = ::protobuf::EnumOrUnknown::new(super::DCJAOPDINOI::DCJAOPDINOI::PAGE_DESC_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TravelBrochureSetPageDescStatusCsReq {
        static instance: TravelBrochureSetPageDescStatusCsReq = TravelBrochureSetPageDescStatusCsReq {
            CPODEJOFPDD: 0,
            GEIBGFDENJA: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TravelBrochureSetPageDescStatusCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TravelBrochureSetPageDescStatusCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TravelBrochureSetPageDescStatusCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TravelBrochureSetPageDescStatusCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*TravelBrochureSetPageDescStatusCsReq.proto\x1a\x11DCJAOPDINOI.proto\"\
    x\n$TravelBrochureSetPageDescStatusCsReq\x12\x20\n\x0bCPODEJOFPDD\x18\
    \x06\x20\x01(\rR\x0bCPODEJOFPDD\x12.\n\x0bGEIBGFDENJA\x18\x0f\x20\x01(\
    \x0e2\x0c.DCJAOPDINOIR\x0bGEIBGFDENJAb\x06proto3\
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
            deps.push(super::DCJAOPDINOI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TravelBrochureSetPageDescStatusCsReq::generated_message_descriptor_data());
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
