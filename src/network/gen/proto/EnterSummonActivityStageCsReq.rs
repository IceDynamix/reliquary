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

//! Generated file from `EnterSummonActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterSummonActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterSummonActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.AHFNGPLDAII)
    pub AHFNGPLDAII: u32,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::PMCFMHJGKCE::PMCFMHJGKCE>,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.PHNOLPNNJCE)
    pub PHNOLPNNJCE: ::protobuf::MessageField<super::PMCFMHJGKCE::PMCFMHJGKCE>,
    // special fields
    // @@protoc_insertion_point(special_field:EnterSummonActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterSummonActivityStageCsReq {
    fn default() -> &'a EnterSummonActivityStageCsReq {
        <EnterSummonActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterSummonActivityStageCsReq {
    pub fn new() -> EnterSummonActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AHFNGPLDAII",
            |m: &EnterSummonActivityStageCsReq| { &m.AHFNGPLDAII },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.AHFNGPLDAII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &EnterSummonActivityStageCsReq| { &m.GCFIIGOLPMF },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EnterSummonActivityStageCsReq| { &m.avatar_list },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PMCFMHJGKCE::PMCFMHJGKCE>(
            "PHNOLPNNJCE",
            |m: &EnterSummonActivityStageCsReq| { &m.PHNOLPNNJCE },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.PHNOLPNNJCE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterSummonActivityStageCsReq>(
            "EnterSummonActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterSummonActivityStageCsReq {
    const NAME: &'static str = "EnterSummonActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.AHFNGPLDAII = is.read_uint32()?;
                },
                16 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                50 => {
                    self.avatar_list.push(is.read_message()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PHNOLPNNJCE)?;
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
        if self.AHFNGPLDAII != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.AHFNGPLDAII);
        }
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.GCFIIGOLPMF);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.PHNOLPNNJCE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AHFNGPLDAII != 0 {
            os.write_uint32(9, self.AHFNGPLDAII)?;
        }
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(2, self.GCFIIGOLPMF)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if let Some(v) = self.PHNOLPNNJCE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> EnterSummonActivityStageCsReq {
        EnterSummonActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.AHFNGPLDAII = 0;
        self.GCFIIGOLPMF = 0;
        self.avatar_list.clear();
        self.PHNOLPNNJCE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterSummonActivityStageCsReq {
        static instance: EnterSummonActivityStageCsReq = EnterSummonActivityStageCsReq {
            AHFNGPLDAII: 0,
            GCFIIGOLPMF: 0,
            avatar_list: ::std::vec::Vec::new(),
            PHNOLPNNJCE: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterSummonActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterSummonActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterSummonActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterSummonActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#EnterSummonActivityStageCsReq.proto\x1a\x11PMCFMHJGKCE.proto\"\xc2\
    \x01\n\x1dEnterSummonActivityStageCsReq\x12\x20\n\x0bAHFNGPLDAII\x18\t\
    \x20\x01(\rR\x0bAHFNGPLDAII\x12\x20\n\x0bGCFIIGOLPMF\x18\x02\x20\x01(\rR\
    \x0bGCFIIGOLPMF\x12-\n\x0bavatar_list\x18\x06\x20\x03(\x0b2\x0c.PMCFMHJG\
    KCER\navatarList\x12.\n\x0bPHNOLPNNJCE\x18\x0b\x20\x01(\x0b2\x0c.PMCFMHJ\
    GKCER\x0bPHNOLPNNJCEb\x06proto3\
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
            deps.push(super::PMCFMHJGKCE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterSummonActivityStageCsReq::generated_message_descriptor_data());
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