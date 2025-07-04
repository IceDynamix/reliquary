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

//! Generated file from `SearchPlayerScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SearchPlayerScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SearchPlayerScRsp {
    // message fields
    // @@protoc_insertion_point(field:SearchPlayerScRsp.simple_info_list)
    pub simple_info_list: ::std::vec::Vec<super::PlayerSimpleInfo::PlayerSimpleInfo>,
    // @@protoc_insertion_point(field:SearchPlayerScRsp.result_uid_list)
    pub result_uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SearchPlayerScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SearchPlayerScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SearchPlayerScRsp {
    fn default() -> &'a SearchPlayerScRsp {
        <SearchPlayerScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SearchPlayerScRsp {
    pub fn new() -> SearchPlayerScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "simple_info_list",
            |m: &SearchPlayerScRsp| { &m.simple_info_list },
            |m: &mut SearchPlayerScRsp| { &mut m.simple_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "result_uid_list",
            |m: &SearchPlayerScRsp| { &m.result_uid_list },
            |m: &mut SearchPlayerScRsp| { &mut m.result_uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SearchPlayerScRsp| { &m.retcode },
            |m: &mut SearchPlayerScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SearchPlayerScRsp>(
            "SearchPlayerScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SearchPlayerScRsp {
    const NAME: &'static str = "SearchPlayerScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.simple_info_list.push(is.read_message()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.result_uid_list)?;
                },
                96 => {
                    self.result_uid_list.push(is.read_uint32()?);
                },
                24 => {
                    self.retcode = is.read_uint32()?;
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
        for value in &self.simple_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.result_uid_list);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.simple_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        os.write_repeated_packed_uint32(12, &self.result_uid_list)?;
        if self.retcode != 0 {
            os.write_uint32(3, self.retcode)?;
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

    fn new() -> SearchPlayerScRsp {
        SearchPlayerScRsp::new()
    }

    fn clear(&mut self) {
        self.simple_info_list.clear();
        self.result_uid_list.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SearchPlayerScRsp {
        static instance: SearchPlayerScRsp = SearchPlayerScRsp {
            simple_info_list: ::std::vec::Vec::new(),
            result_uid_list: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SearchPlayerScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SearchPlayerScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SearchPlayerScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SearchPlayerScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17SearchPlayerScRsp.proto\x1a\x16PlayerSimpleInfo.proto\"\x92\x01\n\
    \x11SearchPlayerScRsp\x12;\n\x10simple_info_list\x18\t\x20\x03(\x0b2\x11\
    .PlayerSimpleInfoR\x0esimpleInfoList\x12&\n\x0fresult_uid_list\x18\x0c\
    \x20\x03(\rR\rresultUidList\x12\x18\n\x07retcode\x18\x03\x20\x01(\rR\x07\
    retcodeb\x06proto3\
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
            deps.push(super::PlayerSimpleInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SearchPlayerScRsp::generated_message_descriptor_data());
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
