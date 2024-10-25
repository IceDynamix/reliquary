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

//! Generated file from `UpdateGunPlayDataCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdateGunPlayDataCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateGunPlayDataCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateGunPlayDataCsReq.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // @@protoc_insertion_point(field:UpdateGunPlayDataCsReq.NIEPAOBBFJF)
    pub NIEPAOBBFJF: u64,
    // @@protoc_insertion_point(field:UpdateGunPlayDataCsReq.GHCJKLFLCEM)
    pub GHCJKLFLCEM: ::protobuf::MessageField<super::NNEOOKDGDIK::NNEOOKDGDIK>,
    // @@protoc_insertion_point(field:UpdateGunPlayDataCsReq.LMGBJJKFLKK)
    pub LMGBJJKFLKK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateGunPlayDataCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateGunPlayDataCsReq {
    fn default() -> &'a UpdateGunPlayDataCsReq {
        <UpdateGunPlayDataCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateGunPlayDataCsReq {
    pub fn new() -> UpdateGunPlayDataCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &UpdateGunPlayDataCsReq| { &m.GCFIIGOLPMF },
            |m: &mut UpdateGunPlayDataCsReq| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIEPAOBBFJF",
            |m: &UpdateGunPlayDataCsReq| { &m.NIEPAOBBFJF },
            |m: &mut UpdateGunPlayDataCsReq| { &mut m.NIEPAOBBFJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NNEOOKDGDIK::NNEOOKDGDIK>(
            "GHCJKLFLCEM",
            |m: &UpdateGunPlayDataCsReq| { &m.GHCJKLFLCEM },
            |m: &mut UpdateGunPlayDataCsReq| { &mut m.GHCJKLFLCEM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMGBJJKFLKK",
            |m: &UpdateGunPlayDataCsReq| { &m.LMGBJJKFLKK },
            |m: &mut UpdateGunPlayDataCsReq| { &mut m.LMGBJJKFLKK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateGunPlayDataCsReq>(
            "UpdateGunPlayDataCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateGunPlayDataCsReq {
    const NAME: &'static str = "UpdateGunPlayDataCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                56 => {
                    self.NIEPAOBBFJF = is.read_uint64()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GHCJKLFLCEM)?;
                },
                64 => {
                    self.LMGBJJKFLKK = is.read_uint32()?;
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
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.GCFIIGOLPMF);
        }
        if self.NIEPAOBBFJF != 0 {
            my_size += ::protobuf::rt::uint64_size(7, self.NIEPAOBBFJF);
        }
        if let Some(v) = self.GHCJKLFLCEM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LMGBJJKFLKK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LMGBJJKFLKK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(9, self.GCFIIGOLPMF)?;
        }
        if self.NIEPAOBBFJF != 0 {
            os.write_uint64(7, self.NIEPAOBBFJF)?;
        }
        if let Some(v) = self.GHCJKLFLCEM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.LMGBJJKFLKK != 0 {
            os.write_uint32(8, self.LMGBJJKFLKK)?;
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

    fn new() -> UpdateGunPlayDataCsReq {
        UpdateGunPlayDataCsReq::new()
    }

    fn clear(&mut self) {
        self.GCFIIGOLPMF = 0;
        self.NIEPAOBBFJF = 0;
        self.GHCJKLFLCEM.clear();
        self.LMGBJJKFLKK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateGunPlayDataCsReq {
        static instance: UpdateGunPlayDataCsReq = UpdateGunPlayDataCsReq {
            GCFIIGOLPMF: 0,
            NIEPAOBBFJF: 0,
            GHCJKLFLCEM: ::protobuf::MessageField::none(),
            LMGBJJKFLKK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateGunPlayDataCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateGunPlayDataCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateGunPlayDataCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateGunPlayDataCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cUpdateGunPlayDataCsReq.proto\x1a\x11NNEOOKDGDIK.proto\"\xae\x01\n\
    \x16UpdateGunPlayDataCsReq\x12\x20\n\x0bGCFIIGOLPMF\x18\t\x20\x01(\rR\
    \x0bGCFIIGOLPMF\x12\x20\n\x0bNIEPAOBBFJF\x18\x07\x20\x01(\x04R\x0bNIEPAO\
    BBFJF\x12.\n\x0bGHCJKLFLCEM\x18\r\x20\x01(\x0b2\x0c.NNEOOKDGDIKR\x0bGHCJ\
    KLFLCEM\x12\x20\n\x0bLMGBJJKFLKK\x18\x08\x20\x01(\rR\x0bLMGBJJKFLKKb\x06\
    proto3\
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
            deps.push(super::NNEOOKDGDIK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateGunPlayDataCsReq::generated_message_descriptor_data());
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
