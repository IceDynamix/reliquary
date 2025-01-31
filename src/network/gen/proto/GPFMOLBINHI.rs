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

//! Generated file from `GPFMOLBINHI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GPFMOLBINHI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GPFMOLBINHI {
    // message fields
    // @@protoc_insertion_point(field:GPFMOLBINHI.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::NPFHEFGPJMB::NPFHEFGPJMB>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.HAJKGDAGKAF)
    pub HAJKGDAGKAF: ::std::vec::Vec<super::COCAIPNLEPD::COCAIPNLEPD>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.NGDAJKNLELE)
    pub NGDAJKNLELE: ::std::vec::Vec<super::ELDJOHKCCPL::ELDJOHKCCPL>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // @@protoc_insertion_point(field:GPFMOLBINHI.CAPBIHHHLIC)
    pub CAPBIHHHLIC: ::std::collections::HashMap<u32, super::CPKNPHCIJIB::CPKNPHCIJIB>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.IGPIPNAAEBJ)
    pub IGPIPNAAEBJ: ::protobuf::MessageField<super::NMKAECDMHOP::NMKAECDMHOP>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.PKKDLCLIFAC)
    pub PKKDLCLIFAC: ::std::vec::Vec<super::NPFHEFGPJMB::NPFHEFGPJMB>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.ADAFAOHFLMA)
    pub ADAFAOHFLMA: ::protobuf::MessageField<super::EvolveBuildBattleInfo::EvolveBuildBattleInfo>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.GDDALCGOOBG)
    pub GDDALCGOOBG: ::protobuf::MessageField<super::OLGKJDLMEIO::OLGKJDLMEIO>,
    // @@protoc_insertion_point(field:GPFMOLBINHI.NAIPABGBFMH)
    pub NAIPABGBFMH: ::protobuf::MessageField<super::DIHJEAPMCLL::DIHJEAPMCLL>,
    // special fields
    // @@protoc_insertion_point(special_field:GPFMOLBINHI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GPFMOLBINHI {
    fn default() -> &'a GPFMOLBINHI {
        <GPFMOLBINHI as ::protobuf::Message>::default_instance()
    }
}

impl GPFMOLBINHI {
    pub fn new() -> GPFMOLBINHI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &GPFMOLBINHI| { &m.avatar_list },
            |m: &mut GPFMOLBINHI| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HAJKGDAGKAF",
            |m: &GPFMOLBINHI| { &m.HAJKGDAGKAF },
            |m: &mut GPFMOLBINHI| { &mut m.HAJKGDAGKAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGDAJKNLELE",
            |m: &GPFMOLBINHI| { &m.NGDAJKNLELE },
            |m: &mut GPFMOLBINHI| { &mut m.NGDAJKNLELE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &GPFMOLBINHI| { &m.CFNJJEJIGOK },
            |m: &mut GPFMOLBINHI| { &mut m.CFNJJEJIGOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "CAPBIHHHLIC",
            |m: &GPFMOLBINHI| { &m.CAPBIHHHLIC },
            |m: &mut GPFMOLBINHI| { &mut m.CAPBIHHHLIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NMKAECDMHOP::NMKAECDMHOP>(
            "IGPIPNAAEBJ",
            |m: &GPFMOLBINHI| { &m.IGPIPNAAEBJ },
            |m: &mut GPFMOLBINHI| { &mut m.IGPIPNAAEBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PKKDLCLIFAC",
            |m: &GPFMOLBINHI| { &m.PKKDLCLIFAC },
            |m: &mut GPFMOLBINHI| { &mut m.PKKDLCLIFAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EvolveBuildBattleInfo::EvolveBuildBattleInfo>(
            "ADAFAOHFLMA",
            |m: &GPFMOLBINHI| { &m.ADAFAOHFLMA },
            |m: &mut GPFMOLBINHI| { &mut m.ADAFAOHFLMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OLGKJDLMEIO::OLGKJDLMEIO>(
            "GDDALCGOOBG",
            |m: &GPFMOLBINHI| { &m.GDDALCGOOBG },
            |m: &mut GPFMOLBINHI| { &mut m.GDDALCGOOBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DIHJEAPMCLL::DIHJEAPMCLL>(
            "NAIPABGBFMH",
            |m: &GPFMOLBINHI| { &m.NAIPABGBFMH },
            |m: &mut GPFMOLBINHI| { &mut m.NAIPABGBFMH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GPFMOLBINHI>(
            "GPFMOLBINHI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GPFMOLBINHI {
    const NAME: &'static str = "GPFMOLBINHI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.avatar_list.push(is.read_message()?);
                },
                18 => {
                    self.HAJKGDAGKAF.push(is.read_message()?);
                },
                26 => {
                    self.NGDAJKNLELE.push(is.read_message()?);
                },
                56 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
                },
                74 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.CAPBIHHHLIC.insert(key, value);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IGPIPNAAEBJ)?;
                },
                90 => {
                    self.PKKDLCLIFAC.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ADAFAOHFLMA)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GDDALCGOOBG)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NAIPABGBFMH)?;
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
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.HAJKGDAGKAF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.NGDAJKNLELE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.CFNJJEJIGOK);
        }
        for (k, v) in &self.CAPBIHHHLIC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let Some(v) = self.IGPIPNAAEBJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.PKKDLCLIFAC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.ADAFAOHFLMA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GDDALCGOOBG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NAIPABGBFMH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.HAJKGDAGKAF {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.NGDAJKNLELE {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(7, self.CFNJJEJIGOK)?;
        }
        for (k, v) in &self.CAPBIHHHLIC {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.IGPIPNAAEBJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.PKKDLCLIFAC {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.ADAFAOHFLMA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.GDDALCGOOBG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.NAIPABGBFMH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> GPFMOLBINHI {
        GPFMOLBINHI::new()
    }

    fn clear(&mut self) {
        self.avatar_list.clear();
        self.HAJKGDAGKAF.clear();
        self.NGDAJKNLELE.clear();
        self.CFNJJEJIGOK = 0;
        self.CAPBIHHHLIC.clear();
        self.IGPIPNAAEBJ.clear();
        self.PKKDLCLIFAC.clear();
        self.ADAFAOHFLMA.clear();
        self.GDDALCGOOBG.clear();
        self.NAIPABGBFMH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GPFMOLBINHI {
        static instance: ::protobuf::rt::Lazy<GPFMOLBINHI> = ::protobuf::rt::Lazy::new();
        instance.get(GPFMOLBINHI::new)
    }
}

impl ::protobuf::MessageFull for GPFMOLBINHI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GPFMOLBINHI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GPFMOLBINHI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GPFMOLBINHI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GPFMOLBINHI.proto\x1a\x11COCAIPNLEPD.proto\x1a\x11CPKNPHCIJIB.prot\
    o\x1a\x11DIHJEAPMCLL.proto\x1a\x11ELDJOHKCCPL.proto\x1a\x1bEvolveBuildBa\
    ttleInfo.proto\x1a\x11NMKAECDMHOP.proto\x1a\x11NPFHEFGPJMB.proto\x1a\x11\
    OLGKJDLMEIO.proto\"\xc7\x04\n\x0bGPFMOLBINHI\x12-\n\x0bavatar_list\x18\
    \x01\x20\x03(\x0b2\x0c.NPFHEFGPJMBR\navatarList\x12.\n\x0bHAJKGDAGKAF\
    \x18\x02\x20\x03(\x0b2\x0c.COCAIPNLEPDR\x0bHAJKGDAGKAF\x12.\n\x0bNGDAJKN\
    LELE\x18\x03\x20\x03(\x0b2\x0c.ELDJOHKCCPLR\x0bNGDAJKNLELE\x12\x20\n\x0b\
    CFNJJEJIGOK\x18\x07\x20\x01(\rR\x0bCFNJJEJIGOK\x12?\n\x0bCAPBIHHHLIC\x18\
    \t\x20\x03(\x0b2\x1d.GPFMOLBINHI.CAPBIHHHLICEntryR\x0bCAPBIHHHLIC\x12.\n\
    \x0bIGPIPNAAEBJ\x18\n\x20\x01(\x0b2\x0c.NMKAECDMHOPR\x0bIGPIPNAAEBJ\x12.\
    \n\x0bPKKDLCLIFAC\x18\x0b\x20\x03(\x0b2\x0c.NPFHEFGPJMBR\x0bPKKDLCLIFAC\
    \x128\n\x0bADAFAOHFLMA\x18\x0c\x20\x01(\x0b2\x16.EvolveBuildBattleInfoR\
    \x0bADAFAOHFLMA\x12.\n\x0bGDDALCGOOBG\x18\r\x20\x01(\x0b2\x0c.OLGKJDLMEI\
    OR\x0bGDDALCGOOBG\x12.\n\x0bNAIPABGBFMH\x18\x0e\x20\x01(\x0b2\x0c.DIHJEA\
    PMCLLR\x0bNAIPABGBFMH\x1aL\n\x10CAPBIHHHLICEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.CPKNP\
    HCIJIBR\x05value:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::COCAIPNLEPD::file_descriptor().clone());
            deps.push(super::CPKNPHCIJIB::file_descriptor().clone());
            deps.push(super::DIHJEAPMCLL::file_descriptor().clone());
            deps.push(super::ELDJOHKCCPL::file_descriptor().clone());
            deps.push(super::EvolveBuildBattleInfo::file_descriptor().clone());
            deps.push(super::NMKAECDMHOP::file_descriptor().clone());
            deps.push(super::NPFHEFGPJMB::file_descriptor().clone());
            deps.push(super::OLGKJDLMEIO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GPFMOLBINHI::generated_message_descriptor_data());
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
