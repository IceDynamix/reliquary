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

//! Generated file from `UpdatePsnSettingsInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdatePsnSettingsInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdatePsnSettingsInfoCsReq {
    // message oneof groups
    pub OLMDJGNMKFI: ::std::option::Option<update_psn_settings_info_cs_req::OLMDJGNMKFI>,
    // special fields
    // @@protoc_insertion_point(special_field:UpdatePsnSettingsInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdatePsnSettingsInfoCsReq {
    fn default() -> &'a UpdatePsnSettingsInfoCsReq {
        <UpdatePsnSettingsInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdatePsnSettingsInfoCsReq {
    pub fn new() -> UpdatePsnSettingsInfoCsReq {
        ::std::default::Default::default()
    }

    // .LODLBMPAHKB CBIONLOHECO = 1290;

    pub fn CBIONLOHECO(&self) -> &super::LODLBMPAHKB::LODLBMPAHKB {
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(ref v)) => v,
            _ => <super::LODLBMPAHKB::LODLBMPAHKB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_CBIONLOHECO(&mut self) {
        self.OLMDJGNMKFI = ::std::option::Option::None;
    }

    pub fn has_CBIONLOHECO(&self) -> bool {
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_CBIONLOHECO(&mut self, v: super::LODLBMPAHKB::LODLBMPAHKB) {
        self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_CBIONLOHECO(&mut self) -> &mut super::LODLBMPAHKB::LODLBMPAHKB {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(_)) = self.OLMDJGNMKFI {
        } else {
            self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(super::LODLBMPAHKB::LODLBMPAHKB::new()));
        }
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_CBIONLOHECO(&mut self) -> super::LODLBMPAHKB::LODLBMPAHKB {
        if self.has_CBIONLOHECO() {
            match self.OLMDJGNMKFI.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LODLBMPAHKB::LODLBMPAHKB::new()
        }
    }

    // .JPPECDLFCEE MCAJLEBOKOI = 583;

    pub fn MCAJLEBOKOI(&self) -> &super::JPPECDLFCEE::JPPECDLFCEE {
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(ref v)) => v,
            _ => <super::JPPECDLFCEE::JPPECDLFCEE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_MCAJLEBOKOI(&mut self) {
        self.OLMDJGNMKFI = ::std::option::Option::None;
    }

    pub fn has_MCAJLEBOKOI(&self) -> bool {
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_MCAJLEBOKOI(&mut self, v: super::JPPECDLFCEE::JPPECDLFCEE) {
        self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_MCAJLEBOKOI(&mut self) -> &mut super::JPPECDLFCEE::JPPECDLFCEE {
        if let ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(_)) = self.OLMDJGNMKFI {
        } else {
            self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(super::JPPECDLFCEE::JPPECDLFCEE::new()));
        }
        match self.OLMDJGNMKFI {
            ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_MCAJLEBOKOI(&mut self) -> super::JPPECDLFCEE::JPPECDLFCEE {
        if self.has_MCAJLEBOKOI() {
            match self.OLMDJGNMKFI.take() {
                ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JPPECDLFCEE::JPPECDLFCEE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LODLBMPAHKB::LODLBMPAHKB>(
            "CBIONLOHECO",
            UpdatePsnSettingsInfoCsReq::has_CBIONLOHECO,
            UpdatePsnSettingsInfoCsReq::CBIONLOHECO,
            UpdatePsnSettingsInfoCsReq::mut_CBIONLOHECO,
            UpdatePsnSettingsInfoCsReq::set_CBIONLOHECO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JPPECDLFCEE::JPPECDLFCEE>(
            "MCAJLEBOKOI",
            UpdatePsnSettingsInfoCsReq::has_MCAJLEBOKOI,
            UpdatePsnSettingsInfoCsReq::MCAJLEBOKOI,
            UpdatePsnSettingsInfoCsReq::mut_MCAJLEBOKOI,
            UpdatePsnSettingsInfoCsReq::set_MCAJLEBOKOI,
        ));
        oneofs.push(update_psn_settings_info_cs_req::OLMDJGNMKFI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdatePsnSettingsInfoCsReq>(
            "UpdatePsnSettingsInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdatePsnSettingsInfoCsReq {
    const NAME: &'static str = "UpdatePsnSettingsInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10322 => {
                    self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(is.read_message()?));
                },
                4666 => {
                    self.OLMDJGNMKFI = ::std::option::Option::Some(update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.OLMDJGNMKFI {
            match v {
                &update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.OLMDJGNMKFI {
            match v {
                &update_psn_settings_info_cs_req::OLMDJGNMKFI::CBIONLOHECO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1290, v, os)?;
                },
                &update_psn_settings_info_cs_req::OLMDJGNMKFI::MCAJLEBOKOI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(583, v, os)?;
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

    fn new() -> UpdatePsnSettingsInfoCsReq {
        UpdatePsnSettingsInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.OLMDJGNMKFI = ::std::option::Option::None;
        self.OLMDJGNMKFI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdatePsnSettingsInfoCsReq {
        static instance: UpdatePsnSettingsInfoCsReq = UpdatePsnSettingsInfoCsReq {
            OLMDJGNMKFI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdatePsnSettingsInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdatePsnSettingsInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdatePsnSettingsInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdatePsnSettingsInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `UpdatePsnSettingsInfoCsReq`
pub mod update_psn_settings_info_cs_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:UpdatePsnSettingsInfoCsReq.OLMDJGNMKFI)
    pub enum OLMDJGNMKFI {
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.CBIONLOHECO)
        CBIONLOHECO(super::super::LODLBMPAHKB::LODLBMPAHKB),
        // @@protoc_insertion_point(oneof_field:UpdatePsnSettingsInfoCsReq.MCAJLEBOKOI)
        MCAJLEBOKOI(super::super::JPPECDLFCEE::JPPECDLFCEE),
    }

    impl ::protobuf::Oneof for OLMDJGNMKFI {
    }

    impl ::protobuf::OneofFull for OLMDJGNMKFI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::UpdatePsnSettingsInfoCsReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("OLMDJGNMKFI").unwrap()).clone()
        }
    }

    impl OLMDJGNMKFI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<OLMDJGNMKFI>("OLMDJGNMKFI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20UpdatePsnSettingsInfoCsReq.proto\x1a\x11JPPECDLFCEE.proto\x1a\x11L\
    ODLBMPAHKB.proto\"\x91\x01\n\x1aUpdatePsnSettingsInfoCsReq\x121\n\x0bCBI\
    ONLOHECO\x18\x8a\n\x20\x01(\x0b2\x0c.LODLBMPAHKBH\0R\x0bCBIONLOHECO\x121\
    \n\x0bMCAJLEBOKOI\x18\xc7\x04\x20\x01(\x0b2\x0c.JPPECDLFCEEH\0R\x0bMCAJL\
    EBOKOIB\r\n\x0bOLMDJGNMKFIb\x06proto3\
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
            deps.push(super::JPPECDLFCEE::file_descriptor().clone());
            deps.push(super::LODLBMPAHKB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdatePsnSettingsInfoCsReq::generated_message_descriptor_data());
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
