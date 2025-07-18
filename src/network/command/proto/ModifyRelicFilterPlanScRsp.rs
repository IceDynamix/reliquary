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

//! Generated file from `ModifyRelicFilterPlanScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ModifyRelicFilterPlanScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ModifyRelicFilterPlanScRsp {
    // message fields
    // @@protoc_insertion_point(field:ModifyRelicFilterPlanScRsp.update_timestamp)
    pub update_timestamp: i64,
    // @@protoc_insertion_point(field:ModifyRelicFilterPlanScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ModifyRelicFilterPlanScRsp.slot_index)
    pub slot_index: u32,
    // message oneof groups
    pub InfoOneofCase: ::std::option::Option<modify_relic_filter_plan_sc_rsp::InfoOneofCase>,
    // special fields
    // @@protoc_insertion_point(special_field:ModifyRelicFilterPlanScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ModifyRelicFilterPlanScRsp {
    fn default() -> &'a ModifyRelicFilterPlanScRsp {
        <ModifyRelicFilterPlanScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ModifyRelicFilterPlanScRsp {
    pub fn new() -> ModifyRelicFilterPlanScRsp {
        ::std::default::Default::default()
    }

    // string name = 2;

    pub fn name(&self) -> &str {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_name(&mut self) {
        self.InfoOneofCase = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(_)) = self.InfoOneofCase {
        } else {
            self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(::std::string::String::new()));
        }
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.InfoOneofCase.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .RelicFilterPlanIcon icon = 4;

    pub fn icon(&self) -> &super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(ref v)) => v,
            _ => <super::RelicFilterPlanIcon::RelicFilterPlanIcon as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_icon(&mut self) {
        self.InfoOneofCase = ::std::option::Option::None;
    }

    pub fn has_icon(&self) -> bool {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: super::RelicFilterPlanIcon::RelicFilterPlanIcon) {
        self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(v))
    }

    // Mutable pointer to the field.
    pub fn mut_icon(&mut self) -> &mut super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        if let ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(_)) = self.InfoOneofCase {
        } else {
            self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(super::RelicFilterPlanIcon::RelicFilterPlanIcon::new()));
        }
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_icon(&mut self) -> super::RelicFilterPlanIcon::RelicFilterPlanIcon {
        if self.has_icon() {
            match self.InfoOneofCase.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(v)) => v,
                _ => panic!(),
            }
        } else {
            super::RelicFilterPlanIcon::RelicFilterPlanIcon::new()
        }
    }

    // .RelicFilterPlanSettings settings = 12;

    pub fn settings(&self) -> &super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(ref v)) => v,
            _ => <super::RelicFilterPlanSettings::RelicFilterPlanSettings as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_settings(&mut self) {
        self.InfoOneofCase = ::std::option::Option::None;
    }

    pub fn has_settings(&self) -> bool {
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_settings(&mut self, v: super::RelicFilterPlanSettings::RelicFilterPlanSettings) {
        self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(v))
    }

    // Mutable pointer to the field.
    pub fn mut_settings(&mut self) -> &mut super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        if let ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(_)) = self.InfoOneofCase {
        } else {
            self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(super::RelicFilterPlanSettings::RelicFilterPlanSettings::new()));
        }
        match self.InfoOneofCase {
            ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_settings(&mut self) -> super::RelicFilterPlanSettings::RelicFilterPlanSettings {
        if self.has_settings() {
            match self.InfoOneofCase.take() {
                ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(v)) => v,
                _ => panic!(),
            }
        } else {
            super::RelicFilterPlanSettings::RelicFilterPlanSettings::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "update_timestamp",
            |m: &ModifyRelicFilterPlanScRsp| { &m.update_timestamp },
            |m: &mut ModifyRelicFilterPlanScRsp| { &mut m.update_timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ModifyRelicFilterPlanScRsp| { &m.retcode },
            |m: &mut ModifyRelicFilterPlanScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot_index",
            |m: &ModifyRelicFilterPlanScRsp| { &m.slot_index },
            |m: &mut ModifyRelicFilterPlanScRsp| { &mut m.slot_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "name",
            ModifyRelicFilterPlanScRsp::has_name,
            ModifyRelicFilterPlanScRsp::name,
            ModifyRelicFilterPlanScRsp::set_name,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::RelicFilterPlanIcon::RelicFilterPlanIcon>(
            "icon",
            ModifyRelicFilterPlanScRsp::has_icon,
            ModifyRelicFilterPlanScRsp::icon,
            ModifyRelicFilterPlanScRsp::mut_icon,
            ModifyRelicFilterPlanScRsp::set_icon,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::RelicFilterPlanSettings::RelicFilterPlanSettings>(
            "settings",
            ModifyRelicFilterPlanScRsp::has_settings,
            ModifyRelicFilterPlanScRsp::settings,
            ModifyRelicFilterPlanScRsp::mut_settings,
            ModifyRelicFilterPlanScRsp::set_settings,
        ));
        oneofs.push(modify_relic_filter_plan_sc_rsp::InfoOneofCase::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ModifyRelicFilterPlanScRsp>(
            "ModifyRelicFilterPlanScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ModifyRelicFilterPlanScRsp {
    const NAME: &'static str = "ModifyRelicFilterPlanScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.update_timestamp = is.read_int64()?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                88 => {
                    self.slot_index = is.read_uint32()?;
                },
                18 => {
                    self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(is.read_string()?));
                },
                34 => {
                    self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(is.read_message()?));
                },
                98 => {
                    self.InfoOneofCase = ::std::option::Option::Some(modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(is.read_message()?));
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
        if self.update_timestamp != 0 {
            my_size += ::protobuf::rt::int64_size(14, self.update_timestamp);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if self.slot_index != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.slot_index);
        }
        if let ::std::option::Option::Some(ref v) = self.InfoOneofCase {
            match v {
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.update_timestamp != 0 {
            os.write_int64(14, self.update_timestamp)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if self.slot_index != 0 {
            os.write_uint32(11, self.slot_index)?;
        }
        if let ::std::option::Option::Some(ref v) = self.InfoOneofCase {
            match v {
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Name(ref v) => {
                    os.write_string(2, v)?;
                },
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Icon(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &modify_relic_filter_plan_sc_rsp::InfoOneofCase::Settings(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
            };
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

    fn new() -> ModifyRelicFilterPlanScRsp {
        ModifyRelicFilterPlanScRsp::new()
    }

    fn clear(&mut self) {
        self.update_timestamp = 0;
        self.retcode = 0;
        self.slot_index = 0;
        self.InfoOneofCase = ::std::option::Option::None;
        self.InfoOneofCase = ::std::option::Option::None;
        self.InfoOneofCase = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ModifyRelicFilterPlanScRsp {
        static instance: ModifyRelicFilterPlanScRsp = ModifyRelicFilterPlanScRsp {
            update_timestamp: 0,
            retcode: 0,
            slot_index: 0,
            InfoOneofCase: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ModifyRelicFilterPlanScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ModifyRelicFilterPlanScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ModifyRelicFilterPlanScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ModifyRelicFilterPlanScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ModifyRelicFilterPlanScRsp`
pub mod modify_relic_filter_plan_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ModifyRelicFilterPlanScRsp.InfoOneofCase)
    pub enum InfoOneofCase {
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanScRsp.name)
        Name(::std::string::String),
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanScRsp.icon)
        Icon(super::super::RelicFilterPlanIcon::RelicFilterPlanIcon),
        // @@protoc_insertion_point(oneof_field:ModifyRelicFilterPlanScRsp.settings)
        Settings(super::super::RelicFilterPlanSettings::RelicFilterPlanSettings),
    }

    impl ::protobuf::Oneof for InfoOneofCase {
    }

    impl ::protobuf::OneofFull for InfoOneofCase {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ModifyRelicFilterPlanScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("InfoOneofCase").unwrap()).clone()
        }
    }

    impl InfoOneofCase {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<InfoOneofCase>("InfoOneofCase")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ModifyRelicFilterPlanScRsp.proto\x1a\x19RelicFilterPlanIcon.proto\
    \x1a\x1dRelicFilterPlanSettings.proto\"\x8b\x02\n\x1aModifyRelicFilterPl\
    anScRsp\x12)\n\x10update_timestamp\x18\x0e\x20\x01(\x03R\x0fupdateTimest\
    amp\x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\x07retcode\x12\x1d\n\nslot_\
    index\x18\x0b\x20\x01(\rR\tslotIndex\x12\x14\n\x04name\x18\x02\x20\x01(\
    \tH\0R\x04name\x12*\n\x04icon\x18\x04\x20\x01(\x0b2\x14.RelicFilterPlanI\
    conH\0R\x04icon\x126\n\x08settings\x18\x0c\x20\x01(\x0b2\x18.RelicFilter\
    PlanSettingsH\0R\x08settingsB\x0f\n\rInfoOneofCaseb\x06proto3\
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
            deps.push(super::RelicFilterPlanIcon::file_descriptor().clone());
            deps.push(super::RelicFilterPlanSettings::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ModifyRelicFilterPlanScRsp::generated_message_descriptor_data());
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
