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

//! Generated file from `DGCOHFIBHPK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DGCOHFIBHPK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DGCOHFIBHPK {
    // message fields
    // @@protoc_insertion_point(field:DGCOHFIBHPK.GEOIHFIMNIC)
    pub GEOIHFIMNIC: i64,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.FFHMJJADIBG)
    pub FFHMJJADIBG: ::std::string::String,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.AEOHJINGIFK)
    pub AEOHJINGIFK: ::std::string::String,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.MMMINKGDBAF)
    pub MMMINKGDBAF: ::protobuf::EnumOrUnknown<super::BMDNJEKCOAJ::BMDNJEKCOAJ>,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.level)
    pub level: u32,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.LHCINMIBDIO)
    pub LHCINMIBDIO: bool,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.PAJOANFMALG)
    pub PAJOANFMALG: u32,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.IIPGDDFDNBF)
    pub IIPGDDFDNBF: ::std::string::String,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.PDCILADMJIJ)
    pub PDCILADMJIJ: ::std::string::String,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.AMIKBDJKDCC)
    pub AMIKBDJKDCC: ::std::vec::Vec<super::AssistSimpleInfo::AssistSimpleInfo>,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.HBHNPOFEPOD)
    pub HBHNPOFEPOD: u32,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.KPDLOHFMCKP)
    pub KPDLOHFMCKP: ::protobuf::EnumOrUnknown<super::PILEIJLENCM::PILEIJLENCM>,
    // @@protoc_insertion_point(field:DGCOHFIBHPK.DEPEKPIEGJO)
    pub DEPEKPIEGJO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DGCOHFIBHPK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DGCOHFIBHPK {
    fn default() -> &'a DGCOHFIBHPK {
        <DGCOHFIBHPK as ::protobuf::Message>::default_instance()
    }
}

impl DGCOHFIBHPK {
    pub fn new() -> DGCOHFIBHPK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEOIHFIMNIC",
            |m: &DGCOHFIBHPK| { &m.GEOIHFIMNIC },
            |m: &mut DGCOHFIBHPK| { &mut m.GEOIHFIMNIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFHMJJADIBG",
            |m: &DGCOHFIBHPK| { &m.FFHMJJADIBG },
            |m: &mut DGCOHFIBHPK| { &mut m.FFHMJJADIBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AEOHJINGIFK",
            |m: &DGCOHFIBHPK| { &m.AEOHJINGIFK },
            |m: &mut DGCOHFIBHPK| { &mut m.AEOHJINGIFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMMINKGDBAF",
            |m: &DGCOHFIBHPK| { &m.MMMINKGDBAF },
            |m: &mut DGCOHFIBHPK| { &mut m.MMMINKGDBAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &DGCOHFIBHPK| { &m.level },
            |m: &mut DGCOHFIBHPK| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LHCINMIBDIO",
            |m: &DGCOHFIBHPK| { &m.LHCINMIBDIO },
            |m: &mut DGCOHFIBHPK| { &mut m.LHCINMIBDIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PAJOANFMALG",
            |m: &DGCOHFIBHPK| { &m.PAJOANFMALG },
            |m: &mut DGCOHFIBHPK| { &mut m.PAJOANFMALG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IIPGDDFDNBF",
            |m: &DGCOHFIBHPK| { &m.IIPGDDFDNBF },
            |m: &mut DGCOHFIBHPK| { &mut m.IIPGDDFDNBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDCILADMJIJ",
            |m: &DGCOHFIBHPK| { &m.PDCILADMJIJ },
            |m: &mut DGCOHFIBHPK| { &mut m.PDCILADMJIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AMIKBDJKDCC",
            |m: &DGCOHFIBHPK| { &m.AMIKBDJKDCC },
            |m: &mut DGCOHFIBHPK| { &mut m.AMIKBDJKDCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBHNPOFEPOD",
            |m: &DGCOHFIBHPK| { &m.HBHNPOFEPOD },
            |m: &mut DGCOHFIBHPK| { &mut m.HBHNPOFEPOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KPDLOHFMCKP",
            |m: &DGCOHFIBHPK| { &m.KPDLOHFMCKP },
            |m: &mut DGCOHFIBHPK| { &mut m.KPDLOHFMCKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEPEKPIEGJO",
            |m: &DGCOHFIBHPK| { &m.DEPEKPIEGJO },
            |m: &mut DGCOHFIBHPK| { &mut m.DEPEKPIEGJO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DGCOHFIBHPK>(
            "DGCOHFIBHPK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DGCOHFIBHPK {
    const NAME: &'static str = "DGCOHFIBHPK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.GEOIHFIMNIC = is.read_int64()?;
                },
                98 => {
                    self.FFHMJJADIBG = is.read_string()?;
                },
                122 => {
                    self.AEOHJINGIFK = is.read_string()?;
                },
                104 => {
                    self.MMMINKGDBAF = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.level = is.read_uint32()?;
                },
                40 => {
                    self.LHCINMIBDIO = is.read_bool()?;
                },
                112 => {
                    self.PAJOANFMALG = is.read_uint32()?;
                },
                66 => {
                    self.IIPGDDFDNBF = is.read_string()?;
                },
                50 => {
                    self.PDCILADMJIJ = is.read_string()?;
                },
                58 => {
                    self.AMIKBDJKDCC.push(is.read_message()?);
                },
                88 => {
                    self.HBHNPOFEPOD = is.read_uint32()?;
                },
                16 => {
                    self.KPDLOHFMCKP = is.read_enum_or_unknown()?;
                },
                80 => {
                    self.DEPEKPIEGJO = is.read_uint32()?;
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
        if self.GEOIHFIMNIC != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.GEOIHFIMNIC);
        }
        if !self.FFHMJJADIBG.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.FFHMJJADIBG);
        }
        if !self.AEOHJINGIFK.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.AEOHJINGIFK);
        }
        if self.MMMINKGDBAF != ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR) {
            my_size += ::protobuf::rt::int32_size(13, self.MMMINKGDBAF.value());
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.level);
        }
        if self.LHCINMIBDIO != false {
            my_size += 1 + 1;
        }
        if self.PAJOANFMALG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PAJOANFMALG);
        }
        if !self.IIPGDDFDNBF.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.IIPGDDFDNBF);
        }
        if !self.PDCILADMJIJ.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.PDCILADMJIJ);
        }
        for value in &self.AMIKBDJKDCC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.HBHNPOFEPOD != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HBHNPOFEPOD);
        }
        if self.KPDLOHFMCKP != ::protobuf::EnumOrUnknown::new(super::PILEIJLENCM::PILEIJLENCM::FRIEND_ONLINE_STATUS_OFFLINE) {
            my_size += ::protobuf::rt::int32_size(2, self.KPDLOHFMCKP.value());
        }
        if self.DEPEKPIEGJO != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.DEPEKPIEGJO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GEOIHFIMNIC != 0 {
            os.write_int64(1, self.GEOIHFIMNIC)?;
        }
        if !self.FFHMJJADIBG.is_empty() {
            os.write_string(12, &self.FFHMJJADIBG)?;
        }
        if !self.AEOHJINGIFK.is_empty() {
            os.write_string(15, &self.AEOHJINGIFK)?;
        }
        if self.MMMINKGDBAF != ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.MMMINKGDBAF))?;
        }
        if self.level != 0 {
            os.write_uint32(4, self.level)?;
        }
        if self.LHCINMIBDIO != false {
            os.write_bool(5, self.LHCINMIBDIO)?;
        }
        if self.PAJOANFMALG != 0 {
            os.write_uint32(14, self.PAJOANFMALG)?;
        }
        if !self.IIPGDDFDNBF.is_empty() {
            os.write_string(8, &self.IIPGDDFDNBF)?;
        }
        if !self.PDCILADMJIJ.is_empty() {
            os.write_string(6, &self.PDCILADMJIJ)?;
        }
        for v in &self.AMIKBDJKDCC {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.HBHNPOFEPOD != 0 {
            os.write_uint32(11, self.HBHNPOFEPOD)?;
        }
        if self.KPDLOHFMCKP != ::protobuf::EnumOrUnknown::new(super::PILEIJLENCM::PILEIJLENCM::FRIEND_ONLINE_STATUS_OFFLINE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.KPDLOHFMCKP))?;
        }
        if self.DEPEKPIEGJO != 0 {
            os.write_uint32(10, self.DEPEKPIEGJO)?;
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

    fn new() -> DGCOHFIBHPK {
        DGCOHFIBHPK::new()
    }

    fn clear(&mut self) {
        self.GEOIHFIMNIC = 0;
        self.FFHMJJADIBG.clear();
        self.AEOHJINGIFK.clear();
        self.MMMINKGDBAF = ::protobuf::EnumOrUnknown::new(super::BMDNJEKCOAJ::BMDNJEKCOAJ::EDITOR);
        self.level = 0;
        self.LHCINMIBDIO = false;
        self.PAJOANFMALG = 0;
        self.IIPGDDFDNBF.clear();
        self.PDCILADMJIJ.clear();
        self.AMIKBDJKDCC.clear();
        self.HBHNPOFEPOD = 0;
        self.KPDLOHFMCKP = ::protobuf::EnumOrUnknown::new(super::PILEIJLENCM::PILEIJLENCM::FRIEND_ONLINE_STATUS_OFFLINE);
        self.DEPEKPIEGJO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DGCOHFIBHPK {
        static instance: DGCOHFIBHPK = DGCOHFIBHPK {
            GEOIHFIMNIC: 0,
            FFHMJJADIBG: ::std::string::String::new(),
            AEOHJINGIFK: ::std::string::String::new(),
            MMMINKGDBAF: ::protobuf::EnumOrUnknown::from_i32(0),
            level: 0,
            LHCINMIBDIO: false,
            PAJOANFMALG: 0,
            IIPGDDFDNBF: ::std::string::String::new(),
            PDCILADMJIJ: ::std::string::String::new(),
            AMIKBDJKDCC: ::std::vec::Vec::new(),
            HBHNPOFEPOD: 0,
            KPDLOHFMCKP: ::protobuf::EnumOrUnknown::from_i32(0),
            DEPEKPIEGJO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DGCOHFIBHPK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DGCOHFIBHPK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DGCOHFIBHPK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DGCOHFIBHPK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DGCOHFIBHPK.proto\x1a\x16AssistSimpleInfo.proto\x1a\x11BMDNJEKCOAJ\
    .proto\x1a\x11PILEIJLENCM.proto\"\xea\x03\n\x0bDGCOHFIBHPK\x12\x20\n\x0b\
    GEOIHFIMNIC\x18\x01\x20\x01(\x03R\x0bGEOIHFIMNIC\x12\x20\n\x0bFFHMJJADIB\
    G\x18\x0c\x20\x01(\tR\x0bFFHMJJADIBG\x12\x20\n\x0bAEOHJINGIFK\x18\x0f\
    \x20\x01(\tR\x0bAEOHJINGIFK\x12.\n\x0bMMMINKGDBAF\x18\r\x20\x01(\x0e2\
    \x0c.BMDNJEKCOAJR\x0bMMMINKGDBAF\x12\x14\n\x05level\x18\x04\x20\x01(\rR\
    \x05level\x12\x20\n\x0bLHCINMIBDIO\x18\x05\x20\x01(\x08R\x0bLHCINMIBDIO\
    \x12\x20\n\x0bPAJOANFMALG\x18\x0e\x20\x01(\rR\x0bPAJOANFMALG\x12\x20\n\
    \x0bIIPGDDFDNBF\x18\x08\x20\x01(\tR\x0bIIPGDDFDNBF\x12\x20\n\x0bPDCILADM\
    JIJ\x18\x06\x20\x01(\tR\x0bPDCILADMJIJ\x123\n\x0bAMIKBDJKDCC\x18\x07\x20\
    \x03(\x0b2\x11.AssistSimpleInfoR\x0bAMIKBDJKDCC\x12\x20\n\x0bHBHNPOFEPOD\
    \x18\x0b\x20\x01(\rR\x0bHBHNPOFEPOD\x12.\n\x0bKPDLOHFMCKP\x18\x02\x20\
    \x01(\x0e2\x0c.PILEIJLENCMR\x0bKPDLOHFMCKP\x12\x20\n\x0bDEPEKPIEGJO\x18\
    \n\x20\x01(\rR\x0bDEPEKPIEGJOb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::AssistSimpleInfo::file_descriptor().clone());
            deps.push(super::BMDNJEKCOAJ::file_descriptor().clone());
            deps.push(super::PILEIJLENCM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DGCOHFIBHPK::generated_message_descriptor_data());
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