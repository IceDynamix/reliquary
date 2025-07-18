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

//! Generated file from `AddAvatarScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AddAvatarScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AddAvatarScNotify {
    // message fields
    // @@protoc_insertion_point(field:AddAvatarScNotify.src)
    pub src: ::protobuf::EnumOrUnknown<super::AddAvatarSrcState::AddAvatarSrcState>,
    // @@protoc_insertion_point(field:AddAvatarScNotify.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:AddAvatarScNotify.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:AddAvatarScNotify.is_new)
    pub is_new: bool,
    // special fields
    // @@protoc_insertion_point(special_field:AddAvatarScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddAvatarScNotify {
    fn default() -> &'a AddAvatarScNotify {
        <AddAvatarScNotify as ::protobuf::Message>::default_instance()
    }
}

impl AddAvatarScNotify {
    pub fn new() -> AddAvatarScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "src",
            |m: &AddAvatarScNotify| { &m.src },
            |m: &mut AddAvatarScNotify| { &mut m.src },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &AddAvatarScNotify| { &m.reward },
            |m: &mut AddAvatarScNotify| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &AddAvatarScNotify| { &m.base_avatar_id },
            |m: &mut AddAvatarScNotify| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_new",
            |m: &AddAvatarScNotify| { &m.is_new },
            |m: &mut AddAvatarScNotify| { &mut m.is_new },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddAvatarScNotify>(
            "AddAvatarScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddAvatarScNotify {
    const NAME: &'static str = "AddAvatarScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.src = is.read_enum_or_unknown()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                48 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                80 => {
                    self.is_new = is.read_bool()?;
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
        if self.src != ::protobuf::EnumOrUnknown::new(super::AddAvatarSrcState::AddAvatarSrcState::ADD_AVATAR_SRC_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.src.value());
        }
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.base_avatar_id);
        }
        if self.is_new != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.src != ::protobuf::EnumOrUnknown::new(super::AddAvatarSrcState::AddAvatarSrcState::ADD_AVATAR_SRC_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.src))?;
        }
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(6, self.base_avatar_id)?;
        }
        if self.is_new != false {
            os.write_bool(10, self.is_new)?;
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

    fn new() -> AddAvatarScNotify {
        AddAvatarScNotify::new()
    }

    fn clear(&mut self) {
        self.src = ::protobuf::EnumOrUnknown::new(super::AddAvatarSrcState::AddAvatarSrcState::ADD_AVATAR_SRC_NONE);
        self.reward.clear();
        self.base_avatar_id = 0;
        self.is_new = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddAvatarScNotify {
        static instance: AddAvatarScNotify = AddAvatarScNotify {
            src: ::protobuf::EnumOrUnknown::from_i32(0),
            reward: ::protobuf::MessageField::none(),
            base_avatar_id: 0,
            is_new: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddAvatarScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddAvatarScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddAvatarScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddAvatarScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17AddAvatarScNotify.proto\x1a\x17AddAvatarSrcState.proto\x1a\x0eItem\
    List.proto\"\x99\x01\n\x11AddAvatarScNotify\x12$\n\x03src\x18\x0f\x20\
    \x01(\x0e2\x12.AddAvatarSrcStateR\x03src\x12!\n\x06reward\x18\x07\x20\
    \x01(\x0b2\t.ItemListR\x06reward\x12$\n\x0ebase_avatar_id\x18\x06\x20\
    \x01(\rR\x0cbaseAvatarId\x12\x15\n\x06is_new\x18\n\x20\x01(\x08R\x05isNe\
    wb\x06proto3\
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
            deps.push(super::AddAvatarSrcState::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AddAvatarScNotify::generated_message_descriptor_data());
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
