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

//! Generated file from `PIMKBDJNKHN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PIMKBDJNKHN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PIMKBDJNKHN {
    // message fields
    // @@protoc_insertion_point(field:PIMKBDJNKHN.ANMHHPFPMFE)
    pub ANMHHPFPMFE: ::protobuf::MessageField<super::DIJLDOPJKAP::DIJLDOPJKAP>,
    // @@protoc_insertion_point(field:PIMKBDJNKHN.GNOFBHALCGA)
    pub GNOFBHALCGA: ::protobuf::MessageField<super::DisplayAvatarDetailInfo::DisplayAvatarDetailInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:PIMKBDJNKHN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PIMKBDJNKHN {
    fn default() -> &'a PIMKBDJNKHN {
        <PIMKBDJNKHN as ::protobuf::Message>::default_instance()
    }
}

impl PIMKBDJNKHN {
    pub fn new() -> PIMKBDJNKHN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DIJLDOPJKAP::DIJLDOPJKAP>(
            "ANMHHPFPMFE",
            |m: &PIMKBDJNKHN| { &m.ANMHHPFPMFE },
            |m: &mut PIMKBDJNKHN| { &mut m.ANMHHPFPMFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DisplayAvatarDetailInfo::DisplayAvatarDetailInfo>(
            "GNOFBHALCGA",
            |m: &PIMKBDJNKHN| { &m.GNOFBHALCGA },
            |m: &mut PIMKBDJNKHN| { &mut m.GNOFBHALCGA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PIMKBDJNKHN>(
            "PIMKBDJNKHN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PIMKBDJNKHN {
    const NAME: &'static str = "PIMKBDJNKHN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ANMHHPFPMFE)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GNOFBHALCGA)?;
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
        if let Some(v) = self.ANMHHPFPMFE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GNOFBHALCGA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ANMHHPFPMFE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.GNOFBHALCGA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> PIMKBDJNKHN {
        PIMKBDJNKHN::new()
    }

    fn clear(&mut self) {
        self.ANMHHPFPMFE.clear();
        self.GNOFBHALCGA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PIMKBDJNKHN {
        static instance: PIMKBDJNKHN = PIMKBDJNKHN {
            ANMHHPFPMFE: ::protobuf::MessageField::none(),
            GNOFBHALCGA: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PIMKBDJNKHN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PIMKBDJNKHN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PIMKBDJNKHN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PIMKBDJNKHN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PIMKBDJNKHN.proto\x1a\x11DIJLDOPJKAP.proto\x1a\x1dDisplayAvatarDet\
    ailInfo.proto\"y\n\x0bPIMKBDJNKHN\x12.\n\x0bANMHHPFPMFE\x18\x0b\x20\x01(\
    \x0b2\x0c.DIJLDOPJKAPR\x0bANMHHPFPMFE\x12:\n\x0bGNOFBHALCGA\x18\x06\x20\
    \x01(\x0b2\x18.DisplayAvatarDetailInfoR\x0bGNOFBHALCGAb\x06proto3\
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
            deps.push(super::DIJLDOPJKAP::file_descriptor().clone());
            deps.push(super::DisplayAvatarDetailInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PIMKBDJNKHN::generated_message_descriptor_data());
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
