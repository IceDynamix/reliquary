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

//! Generated file from `EvolveBuildCardInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EvolveBuildCardInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EvolveBuildCardInfo {
    // message fields
    // @@protoc_insertion_point(field:EvolveBuildCardInfo.card_id)
    pub card_id: u32,
    // @@protoc_insertion_point(field:EvolveBuildCardInfo.param)
    pub param: f64,
    // @@protoc_insertion_point(field:EvolveBuildCardInfo.is_enable)
    pub is_enable: bool,
    // @@protoc_insertion_point(field:EvolveBuildCardInfo.param_list)
    pub param_list: ::std::vec::Vec<f64>,
    // special fields
    // @@protoc_insertion_point(special_field:EvolveBuildCardInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EvolveBuildCardInfo {
    fn default() -> &'a EvolveBuildCardInfo {
        <EvolveBuildCardInfo as ::protobuf::Message>::default_instance()
    }
}

impl EvolveBuildCardInfo {
    pub fn new() -> EvolveBuildCardInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "card_id",
            |m: &EvolveBuildCardInfo| { &m.card_id },
            |m: &mut EvolveBuildCardInfo| { &mut m.card_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "param",
            |m: &EvolveBuildCardInfo| { &m.param },
            |m: &mut EvolveBuildCardInfo| { &mut m.param },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_enable",
            |m: &EvolveBuildCardInfo| { &m.is_enable },
            |m: &mut EvolveBuildCardInfo| { &mut m.is_enable },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "param_list",
            |m: &EvolveBuildCardInfo| { &m.param_list },
            |m: &mut EvolveBuildCardInfo| { &mut m.param_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EvolveBuildCardInfo>(
            "EvolveBuildCardInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EvolveBuildCardInfo {
    const NAME: &'static str = "EvolveBuildCardInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.card_id = is.read_uint32()?;
                },
                17 => {
                    self.param = is.read_double()?;
                },
                24 => {
                    self.is_enable = is.read_bool()?;
                },
                34 => {
                    is.read_repeated_packed_double_into(&mut self.param_list)?;
                },
                33 => {
                    self.param_list.push(is.read_double()?);
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
        if self.card_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.card_id);
        }
        if self.param != 0. {
            my_size += 1 + 8;
        }
        if self.is_enable != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_double_size(4, &self.param_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.card_id != 0 {
            os.write_uint32(1, self.card_id)?;
        }
        if self.param != 0. {
            os.write_double(2, self.param)?;
        }
        if self.is_enable != false {
            os.write_bool(3, self.is_enable)?;
        }
        os.write_repeated_packed_double(4, &self.param_list)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> EvolveBuildCardInfo {
        EvolveBuildCardInfo::new()
    }

    fn clear(&mut self) {
        self.card_id = 0;
        self.param = 0.;
        self.is_enable = false;
        self.param_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EvolveBuildCardInfo {
        static instance: EvolveBuildCardInfo = EvolveBuildCardInfo {
            card_id: 0,
            param: 0.,
            is_enable: false,
            param_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EvolveBuildCardInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EvolveBuildCardInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EvolveBuildCardInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvolveBuildCardInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19EvolveBuildCardInfo.proto\"\x80\x01\n\x13EvolveBuildCardInfo\x12\
    \x17\n\x07card_id\x18\x01\x20\x01(\rR\x06cardId\x12\x14\n\x05param\x18\
    \x02\x20\x01(\x01R\x05param\x12\x1b\n\tis_enable\x18\x03\x20\x01(\x08R\
    \x08isEnable\x12\x1d\n\nparam_list\x18\x04\x20\x03(\x01R\tparamListb\x06\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EvolveBuildCardInfo::generated_message_descriptor_data());
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
