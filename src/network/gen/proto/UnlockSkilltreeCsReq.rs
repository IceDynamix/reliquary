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

//! Generated file from `UnlockSkilltreeCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:UnlockSkilltreeCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UnlockSkilltreeCsReq {
    // message fields
    // @@protoc_insertion_point(field:UnlockSkilltreeCsReq.level)
    pub level: u32,
    // @@protoc_insertion_point(field:UnlockSkilltreeCsReq.point_id)
    pub point_id: u32,
    // @@protoc_insertion_point(field:UnlockSkilltreeCsReq.DCPBFLJFHBB)
    pub DCPBFLJFHBB: ::std::vec::Vec<super::ItemCost::ItemCost>,
    // special fields
    // @@protoc_insertion_point(special_field:UnlockSkilltreeCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UnlockSkilltreeCsReq {
    fn default() -> &'a UnlockSkilltreeCsReq {
        <UnlockSkilltreeCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UnlockSkilltreeCsReq {
    pub fn new() -> UnlockSkilltreeCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &UnlockSkilltreeCsReq| { &m.level },
            |m: &mut UnlockSkilltreeCsReq| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "point_id",
            |m: &UnlockSkilltreeCsReq| { &m.point_id },
            |m: &mut UnlockSkilltreeCsReq| { &mut m.point_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DCPBFLJFHBB",
            |m: &UnlockSkilltreeCsReq| { &m.DCPBFLJFHBB },
            |m: &mut UnlockSkilltreeCsReq| { &mut m.DCPBFLJFHBB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UnlockSkilltreeCsReq>(
            "UnlockSkilltreeCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UnlockSkilltreeCsReq {
    const NAME: &'static str = "UnlockSkilltreeCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.level = is.read_uint32()?;
                },
                64 => {
                    self.point_id = is.read_uint32()?;
                },
                50 => {
                    self.DCPBFLJFHBB.push(is.read_message()?);
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.level);
        }
        if self.point_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.point_id);
        }
        for value in &self.DCPBFLJFHBB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(4, self.level)?;
        }
        if self.point_id != 0 {
            os.write_uint32(8, self.point_id)?;
        }
        for v in &self.DCPBFLJFHBB {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UnlockSkilltreeCsReq {
        UnlockSkilltreeCsReq::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.point_id = 0;
        self.DCPBFLJFHBB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UnlockSkilltreeCsReq {
        static instance: UnlockSkilltreeCsReq = UnlockSkilltreeCsReq {
            level: 0,
            point_id: 0,
            DCPBFLJFHBB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UnlockSkilltreeCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UnlockSkilltreeCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UnlockSkilltreeCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockSkilltreeCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aUnlockSkilltreeCsReq.proto\x1a\x0eItemCost.proto\"t\n\x14UnlockSki\
    lltreeCsReq\x12\x14\n\x05level\x18\x04\x20\x01(\rR\x05level\x12\x19\n\
    \x08point_id\x18\x08\x20\x01(\rR\x07pointId\x12+\n\x0bDCPBFLJFHBB\x18\
    \x06\x20\x03(\x0b2\t.ItemCostR\x0bDCPBFLJFHBBb\x06proto3\
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
            deps.push(super::ItemCost::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UnlockSkilltreeCsReq::generated_message_descriptor_data());
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
