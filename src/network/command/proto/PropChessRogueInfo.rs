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

//! Generated file from `PropChessRogueInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PropChessRogueInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PropChessRogueInfo {
    // message fields
    // @@protoc_insertion_point(field:PropChessRogueInfo.AKCGHBFGBCC)
    pub AKCGHBFGBCC: bool,
    // @@protoc_insertion_point(field:PropChessRogueInfo.enter_next_cell)
    pub enter_next_cell: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PropChessRogueInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PropChessRogueInfo {
    fn default() -> &'a PropChessRogueInfo {
        <PropChessRogueInfo as ::protobuf::Message>::default_instance()
    }
}

impl PropChessRogueInfo {
    pub fn new() -> PropChessRogueInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCGHBFGBCC",
            |m: &PropChessRogueInfo| { &m.AKCGHBFGBCC },
            |m: &mut PropChessRogueInfo| { &mut m.AKCGHBFGBCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "enter_next_cell",
            |m: &PropChessRogueInfo| { &m.enter_next_cell },
            |m: &mut PropChessRogueInfo| { &mut m.enter_next_cell },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PropChessRogueInfo>(
            "PropChessRogueInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PropChessRogueInfo {
    const NAME: &'static str = "PropChessRogueInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.AKCGHBFGBCC = is.read_bool()?;
                },
                32 => {
                    self.enter_next_cell = is.read_bool()?;
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
        if self.AKCGHBFGBCC != false {
            my_size += 1 + 1;
        }
        if self.enter_next_cell != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AKCGHBFGBCC != false {
            os.write_bool(11, self.AKCGHBFGBCC)?;
        }
        if self.enter_next_cell != false {
            os.write_bool(4, self.enter_next_cell)?;
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

    fn new() -> PropChessRogueInfo {
        PropChessRogueInfo::new()
    }

    fn clear(&mut self) {
        self.AKCGHBFGBCC = false;
        self.enter_next_cell = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PropChessRogueInfo {
        static instance: PropChessRogueInfo = PropChessRogueInfo {
            AKCGHBFGBCC: false,
            enter_next_cell: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PropChessRogueInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PropChessRogueInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PropChessRogueInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropChessRogueInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18PropChessRogueInfo.proto\"^\n\x12PropChessRogueInfo\x12\x20\n\x0bA\
    KCGHBFGBCC\x18\x0b\x20\x01(\x08R\x0bAKCGHBFGBCC\x12&\n\x0fenter_next_cel\
    l\x18\x04\x20\x01(\x08R\renterNextCellb\x06proto3\
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
            messages.push(PropChessRogueInfo::generated_message_descriptor_data());
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
