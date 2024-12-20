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

//! Generated file from `MapRotationData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MapRotationData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MapRotationData {
    // message fields
    // @@protoc_insertion_point(field:MapRotationData.PEMHGHGMHCL)
    pub PEMHGHGMHCL: bool,
    // @@protoc_insertion_point(field:MapRotationData.LGCHOINHLDJ)
    pub LGCHOINHLDJ: i32,
    // @@protoc_insertion_point(field:MapRotationData.energy_info)
    pub energy_info: ::protobuf::MessageField<super::RotatorEnergyInfo::RotatorEnergyInfo>,
    // @@protoc_insertion_point(field:MapRotationData.rotater_data_list)
    pub rotater_data_list: ::std::vec::Vec<super::RotaterData::RotaterData>,
    // @@protoc_insertion_point(field:MapRotationData.FPHPEJPOBGJ)
    pub FPHPEJPOBGJ: u32,
    // @@protoc_insertion_point(field:MapRotationData.charger_info_list)
    pub charger_info_list: ::std::vec::Vec<super::ChargerInfo::ChargerInfo>,
    // @@protoc_insertion_point(field:MapRotationData.map_info)
    pub map_info: ::protobuf::MessageField<super::RotateMapInfo::RotateMapInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:MapRotationData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MapRotationData {
    fn default() -> &'a MapRotationData {
        <MapRotationData as ::protobuf::Message>::default_instance()
    }
}

impl MapRotationData {
    pub fn new() -> MapRotationData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PEMHGHGMHCL",
            |m: &MapRotationData| { &m.PEMHGHGMHCL },
            |m: &mut MapRotationData| { &mut m.PEMHGHGMHCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGCHOINHLDJ",
            |m: &MapRotationData| { &m.LGCHOINHLDJ },
            |m: &mut MapRotationData| { &mut m.LGCHOINHLDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RotatorEnergyInfo::RotatorEnergyInfo>(
            "energy_info",
            |m: &MapRotationData| { &m.energy_info },
            |m: &mut MapRotationData| { &mut m.energy_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rotater_data_list",
            |m: &MapRotationData| { &m.rotater_data_list },
            |m: &mut MapRotationData| { &mut m.rotater_data_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FPHPEJPOBGJ",
            |m: &MapRotationData| { &m.FPHPEJPOBGJ },
            |m: &mut MapRotationData| { &mut m.FPHPEJPOBGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "charger_info_list",
            |m: &MapRotationData| { &m.charger_info_list },
            |m: &mut MapRotationData| { &mut m.charger_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RotateMapInfo::RotateMapInfo>(
            "map_info",
            |m: &MapRotationData| { &m.map_info },
            |m: &mut MapRotationData| { &mut m.map_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MapRotationData>(
            "MapRotationData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MapRotationData {
    const NAME: &'static str = "MapRotationData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.PEMHGHGMHCL = is.read_bool()?;
                },
                24 => {
                    self.LGCHOINHLDJ = is.read_int32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.energy_info)?;
                },
                34 => {
                    self.rotater_data_list.push(is.read_message()?);
                },
                120 => {
                    self.FPHPEJPOBGJ = is.read_uint32()?;
                },
                66 => {
                    self.charger_info_list.push(is.read_message()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.map_info)?;
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
        if self.PEMHGHGMHCL != false {
            my_size += 1 + 1;
        }
        if self.LGCHOINHLDJ != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.LGCHOINHLDJ);
        }
        if let Some(v) = self.energy_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.rotater_data_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.FPHPEJPOBGJ != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.FPHPEJPOBGJ);
        }
        for value in &self.charger_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.map_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PEMHGHGMHCL != false {
            os.write_bool(14, self.PEMHGHGMHCL)?;
        }
        if self.LGCHOINHLDJ != 0 {
            os.write_int32(3, self.LGCHOINHLDJ)?;
        }
        if let Some(v) = self.energy_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.rotater_data_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.FPHPEJPOBGJ != 0 {
            os.write_uint32(15, self.FPHPEJPOBGJ)?;
        }
        for v in &self.charger_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if let Some(v) = self.map_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> MapRotationData {
        MapRotationData::new()
    }

    fn clear(&mut self) {
        self.PEMHGHGMHCL = false;
        self.LGCHOINHLDJ = 0;
        self.energy_info.clear();
        self.rotater_data_list.clear();
        self.FPHPEJPOBGJ = 0;
        self.charger_info_list.clear();
        self.map_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MapRotationData {
        static instance: MapRotationData = MapRotationData {
            PEMHGHGMHCL: false,
            LGCHOINHLDJ: 0,
            energy_info: ::protobuf::MessageField::none(),
            rotater_data_list: ::std::vec::Vec::new(),
            FPHPEJPOBGJ: 0,
            charger_info_list: ::std::vec::Vec::new(),
            map_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MapRotationData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MapRotationData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MapRotationData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapRotationData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15MapRotationData.proto\x1a\x11ChargerInfo.proto\x1a\x11RotaterData.\
    proto\x1a\x13RotateMapInfo.proto\x1a\x17RotatorEnergyInfo.proto\"\xcb\
    \x02\n\x0fMapRotationData\x12\x20\n\x0bPEMHGHGMHCL\x18\x0e\x20\x01(\x08R\
    \x0bPEMHGHGMHCL\x12\x20\n\x0bLGCHOINHLDJ\x18\x03\x20\x01(\x05R\x0bLGCHOI\
    NHLDJ\x123\n\x0benergy_info\x18\n\x20\x01(\x0b2\x12.RotatorEnergyInfoR\n\
    energyInfo\x128\n\x11rotater_data_list\x18\x04\x20\x03(\x0b2\x0c.Rotater\
    DataR\x0frotaterDataList\x12\x20\n\x0bFPHPEJPOBGJ\x18\x0f\x20\x01(\rR\
    \x0bFPHPEJPOBGJ\x128\n\x11charger_info_list\x18\x08\x20\x03(\x0b2\x0c.Ch\
    argerInfoR\x0fchargerInfoList\x12)\n\x08map_info\x18\x05\x20\x01(\x0b2\
    \x0e.RotateMapInfoR\x07mapInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::ChargerInfo::file_descriptor().clone());
            deps.push(super::RotaterData::file_descriptor().clone());
            deps.push(super::RotateMapInfo::file_descriptor().clone());
            deps.push(super::RotatorEnergyInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MapRotationData::generated_message_descriptor_data());
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
