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

//! Generated file from `LPBFCAJECAB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LPBFCAJECAB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LPBFCAJECAB {
    // message fields
    // @@protoc_insertion_point(field:LPBFCAJECAB.NBHIADFEKFF)
    pub NBHIADFEKFF: bool,
    // @@protoc_insertion_point(field:LPBFCAJECAB.LFKFBNPNNKP)
    pub LFKFBNPNNKP: ::protobuf::MessageField<super::BOENKOAFFPJ::BOENKOAFFPJ>,
    // special fields
    // @@protoc_insertion_point(special_field:LPBFCAJECAB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LPBFCAJECAB {
    fn default() -> &'a LPBFCAJECAB {
        <LPBFCAJECAB as ::protobuf::Message>::default_instance()
    }
}

impl LPBFCAJECAB {
    pub fn new() -> LPBFCAJECAB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBHIADFEKFF",
            |m: &LPBFCAJECAB| { &m.NBHIADFEKFF },
            |m: &mut LPBFCAJECAB| { &mut m.NBHIADFEKFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BOENKOAFFPJ::BOENKOAFFPJ>(
            "LFKFBNPNNKP",
            |m: &LPBFCAJECAB| { &m.LFKFBNPNNKP },
            |m: &mut LPBFCAJECAB| { &mut m.LFKFBNPNNKP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LPBFCAJECAB>(
            "LPBFCAJECAB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LPBFCAJECAB {
    const NAME: &'static str = "LPBFCAJECAB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.NBHIADFEKFF = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LFKFBNPNNKP)?;
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
        if self.NBHIADFEKFF != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.LFKFBNPNNKP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NBHIADFEKFF != false {
            os.write_bool(11, self.NBHIADFEKFF)?;
        }
        if let Some(v) = self.LFKFBNPNNKP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> LPBFCAJECAB {
        LPBFCAJECAB::new()
    }

    fn clear(&mut self) {
        self.NBHIADFEKFF = false;
        self.LFKFBNPNNKP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LPBFCAJECAB {
        static instance: LPBFCAJECAB = LPBFCAJECAB {
            NBHIADFEKFF: false,
            LFKFBNPNNKP: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LPBFCAJECAB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LPBFCAJECAB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LPBFCAJECAB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LPBFCAJECAB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LPBFCAJECAB.proto\x1a\x11BOENKOAFFPJ.proto\"_\n\x0bLPBFCAJECAB\x12\
    \x20\n\x0bNBHIADFEKFF\x18\x0b\x20\x01(\x08R\x0bNBHIADFEKFF\x12.\n\x0bLFK\
    FBNPNNKP\x18\r\x20\x01(\x0b2\x0c.BOENKOAFFPJR\x0bLFKFBNPNNKPb\x06proto3\
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
            deps.push(super::BOENKOAFFPJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LPBFCAJECAB::generated_message_descriptor_data());
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
