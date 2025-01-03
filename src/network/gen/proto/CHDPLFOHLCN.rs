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

//! Generated file from `CHDPLFOHLCN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CHDPLFOHLCN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHDPLFOHLCN {
    // message fields
    // @@protoc_insertion_point(field:CHDPLFOHLCN.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<super::KIGHHJDHGOA::KIGHHJDHGOA>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.MGIEBBICLCK)
    pub MGIEBBICLCK: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.EPECBKPMBHE)
    pub EPECBKPMBHE: ::protobuf::MessageField<super::BIMKJOHKBNO::BIMKJOHKBNO>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.KAGEGBLHJDJ)
    pub KAGEGBLHJDJ: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.PGKJOGNBOPO)
    pub PGKJOGNBOPO: ::std::collections::HashMap<u32, super::JGOIHDOBEEK::JGOIHDOBEEK>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.IPHBFHCEDGJ)
    pub IPHBFHCEDGJ: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.PGNCDFGJLAA)
    pub PGNCDFGJLAA: ::std::vec::Vec<super::PJDIBBHNNIO::PJDIBBHNNIO>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.KAHOEKAEFHD)
    pub KAHOEKAEFHD: ::protobuf::MessageField<super::EvolveBuild::EvolveBuildBattleInfo>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.KHOPBJOOJNC)
    pub KHOPBJOOJNC: ::std::vec::Vec<super::LLDJJOIOOMP::LLDJJOIOOMP>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.KALINKEDDBE)
    pub KALINKEDDBE: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.DHNDKIFPOLF)
    pub DHNDKIFPOLF: ::protobuf::MessageField<super::KEJJGGCCGAN::KEJJGGCCGAN>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.CAIECHBOLMJ)
    pub CAIECHBOLMJ: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.DKNMCKJOIPL)
    pub DKNMCKJOIPL: ::std::vec::Vec<super::KAOOHLAKBPN::KAOOHLAKBPN>,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.NLBJKFNNONH)
    pub NLBJKFNNONH: bool,
    // @@protoc_insertion_point(field:CHDPLFOHLCN.LNEFJMMOMKB)
    pub LNEFJMMOMKB: ::protobuf::MessageField<super::HMCNJLLHBLI::HMCNJLLHBLI>,
    // special fields
    // @@protoc_insertion_point(special_field:CHDPLFOHLCN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHDPLFOHLCN {
    fn default() -> &'a CHDPLFOHLCN {
        <CHDPLFOHLCN as ::protobuf::Message>::default_instance()
    }
}

impl CHDPLFOHLCN {
    pub fn new() -> CHDPLFOHLCN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(16);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &CHDPLFOHLCN| { &m.EBDDNGHLIGH },
            |m: &mut CHDPLFOHLCN| { &mut m.EBDDNGHLIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGIEBBICLCK",
            |m: &CHDPLFOHLCN| { &m.MGIEBBICLCK },
            |m: &mut CHDPLFOHLCN| { &mut m.MGIEBBICLCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BIMKJOHKBNO::BIMKJOHKBNO>(
            "EPECBKPMBHE",
            |m: &CHDPLFOHLCN| { &m.EPECBKPMBHE },
            |m: &mut CHDPLFOHLCN| { &mut m.EPECBKPMBHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KAGEGBLHJDJ",
            |m: &CHDPLFOHLCN| { &m.KAGEGBLHJDJ },
            |m: &mut CHDPLFOHLCN| { &mut m.KAGEGBLHJDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "PGKJOGNBOPO",
            |m: &CHDPLFOHLCN| { &m.PGKJOGNBOPO },
            |m: &mut CHDPLFOHLCN| { &mut m.PGKJOGNBOPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPHBFHCEDGJ",
            |m: &CHDPLFOHLCN| { &m.IPHBFHCEDGJ },
            |m: &mut CHDPLFOHLCN| { &mut m.IPHBFHCEDGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PGNCDFGJLAA",
            |m: &CHDPLFOHLCN| { &m.PGNCDFGJLAA },
            |m: &mut CHDPLFOHLCN| { &mut m.PGNCDFGJLAA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EvolveBuild::EvolveBuildBattleInfo>(
            "KAHOEKAEFHD",
            |m: &CHDPLFOHLCN| { &m.KAHOEKAEFHD },
            |m: &mut CHDPLFOHLCN| { &mut m.KAHOEKAEFHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KHOPBJOOJNC",
            |m: &CHDPLFOHLCN| { &m.KHOPBJOOJNC },
            |m: &mut CHDPLFOHLCN| { &mut m.KHOPBJOOJNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KALINKEDDBE",
            |m: &CHDPLFOHLCN| { &m.KALINKEDDBE },
            |m: &mut CHDPLFOHLCN| { &mut m.KALINKEDDBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KEJJGGCCGAN::KEJJGGCCGAN>(
            "DHNDKIFPOLF",
            |m: &CHDPLFOHLCN| { &m.DHNDKIFPOLF },
            |m: &mut CHDPLFOHLCN| { &mut m.DHNDKIFPOLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CAIECHBOLMJ",
            |m: &CHDPLFOHLCN| { &m.CAIECHBOLMJ },
            |m: &mut CHDPLFOHLCN| { &mut m.CAIECHBOLMJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DKNMCKJOIPL",
            |m: &CHDPLFOHLCN| { &m.DKNMCKJOIPL },
            |m: &mut CHDPLFOHLCN| { &mut m.DKNMCKJOIPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &CHDPLFOHLCN| { &m.DNMJBNNJLEL },
            |m: &mut CHDPLFOHLCN| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLBJKFNNONH",
            |m: &CHDPLFOHLCN| { &m.NLBJKFNNONH },
            |m: &mut CHDPLFOHLCN| { &mut m.NLBJKFNNONH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HMCNJLLHBLI::HMCNJLLHBLI>(
            "LNEFJMMOMKB",
            |m: &CHDPLFOHLCN| { &m.LNEFJMMOMKB },
            |m: &mut CHDPLFOHLCN| { &mut m.LNEFJMMOMKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHDPLFOHLCN>(
            "CHDPLFOHLCN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHDPLFOHLCN {
    const NAME: &'static str = "CHDPLFOHLCN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.EBDDNGHLIGH.push(is.read_message()?);
                },
                24 => {
                    self.MGIEBBICLCK = is.read_uint32()?;
                },
                14146 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EPECBKPMBHE)?;
                },
                88 => {
                    self.KAGEGBLHJDJ = is.read_uint32()?;
                },
                12482 => {
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
                    self.PGKJOGNBOPO.insert(key, value);
                },
                104 => {
                    self.IPHBFHCEDGJ = is.read_uint32()?;
                },
                5922 => {
                    self.PGNCDFGJLAA.push(is.read_message()?);
                },
                5722 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KAHOEKAEFHD)?;
                },
                42 => {
                    self.KHOPBJOOJNC.push(is.read_message()?);
                },
                72 => {
                    self.KALINKEDDBE = is.read_uint32()?;
                },
                6986 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DHNDKIFPOLF)?;
                },
                80 => {
                    self.CAIECHBOLMJ = is.read_uint32()?;
                },
                18 => {
                    self.DKNMCKJOIPL.push(is.read_message()?);
                },
                64 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                8 => {
                    self.NLBJKFNNONH = is.read_bool()?;
                },
                11714 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNEFJMMOMKB)?;
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
        for value in &self.EBDDNGHLIGH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MGIEBBICLCK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MGIEBBICLCK);
        }
        if let Some(v) = self.EPECBKPMBHE.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KAGEGBLHJDJ != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.KAGEGBLHJDJ);
        }
        for (k, v) in &self.PGKJOGNBOPO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.IPHBFHCEDGJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.IPHBFHCEDGJ);
        }
        for value in &self.PGNCDFGJLAA {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.KAHOEKAEFHD.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.KHOPBJOOJNC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KALINKEDDBE != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.KALINKEDDBE);
        }
        if let Some(v) = self.DHNDKIFPOLF.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.CAIECHBOLMJ != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.CAIECHBOLMJ);
        }
        for value in &self.DKNMCKJOIPL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DNMJBNNJLEL);
        }
        if self.NLBJKFNNONH != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.LNEFJMMOMKB.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EBDDNGHLIGH {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.MGIEBBICLCK != 0 {
            os.write_uint32(3, self.MGIEBBICLCK)?;
        }
        if let Some(v) = self.EPECBKPMBHE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1768, v, os)?;
        }
        if self.KAGEGBLHJDJ != 0 {
            os.write_uint32(11, self.KAGEGBLHJDJ)?;
        }
        for (k, v) in &self.PGKJOGNBOPO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(12482)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.IPHBFHCEDGJ != 0 {
            os.write_uint32(13, self.IPHBFHCEDGJ)?;
        }
        for v in &self.PGNCDFGJLAA {
            ::protobuf::rt::write_message_field_with_cached_size(740, v, os)?;
        };
        if let Some(v) = self.KAHOEKAEFHD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(715, v, os)?;
        }
        for v in &self.KHOPBJOOJNC {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.KALINKEDDBE != 0 {
            os.write_uint32(9, self.KALINKEDDBE)?;
        }
        if let Some(v) = self.DHNDKIFPOLF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(873, v, os)?;
        }
        if self.CAIECHBOLMJ != 0 {
            os.write_uint32(10, self.CAIECHBOLMJ)?;
        }
        for v in &self.DKNMCKJOIPL {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(8, self.DNMJBNNJLEL)?;
        }
        if self.NLBJKFNNONH != false {
            os.write_bool(1, self.NLBJKFNNONH)?;
        }
        if let Some(v) = self.LNEFJMMOMKB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1464, v, os)?;
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

    fn new() -> CHDPLFOHLCN {
        CHDPLFOHLCN::new()
    }

    fn clear(&mut self) {
        self.EBDDNGHLIGH.clear();
        self.MGIEBBICLCK = 0;
        self.EPECBKPMBHE.clear();
        self.KAGEGBLHJDJ = 0;
        self.PGKJOGNBOPO.clear();
        self.IPHBFHCEDGJ = 0;
        self.PGNCDFGJLAA.clear();
        self.KAHOEKAEFHD.clear();
        self.KHOPBJOOJNC.clear();
        self.KALINKEDDBE = 0;
        self.DHNDKIFPOLF.clear();
        self.CAIECHBOLMJ = 0;
        self.DKNMCKJOIPL.clear();
        self.DNMJBNNJLEL = 0;
        self.NLBJKFNNONH = false;
        self.LNEFJMMOMKB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHDPLFOHLCN {
        static instance: ::protobuf::rt::Lazy<CHDPLFOHLCN> = ::protobuf::rt::Lazy::new();
        instance.get(CHDPLFOHLCN::new)
    }
}

impl ::protobuf::MessageFull for CHDPLFOHLCN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHDPLFOHLCN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHDPLFOHLCN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHDPLFOHLCN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CHDPLFOHLCN.proto\x1a\x11BIMKJOHKBNO.proto\x1a\x1bEvolveBuildBattl\
    eInfo.proto\x1a\x11HMCNJLLHBLI.proto\x1a\x11JGOIHDOBEEK.proto\x1a\x11KAO\
    OHLAKBPN.proto\x1a\x11KEJJGGCCGAN.proto\x1a\x11KIGHHJDHGOA.proto\x1a\x11\
    LLDJJOIOOMP.proto\x1a\x11PJDIBBHNNIO.proto\"\x9a\x06\n\x0bCHDPLFOHLCN\
    \x12.\n\x0bEBDDNGHLIGH\x18\x0e\x20\x03(\x0b2\x0c.KIGHHJDHGOAR\x0bEBDDNGH\
    LIGH\x12\x20\n\x0bMGIEBBICLCK\x18\x03\x20\x01(\rR\x0bMGIEBBICLCK\x12/\n\
    \x0bEPECBKPMBHE\x18\xe8\r\x20\x01(\x0b2\x0c.BIMKJOHKBNOR\x0bEPECBKPMBHE\
    \x12\x20\n\x0bKAGEGBLHJDJ\x18\x0b\x20\x01(\rR\x0bKAGEGBLHJDJ\x12@\n\x0bP\
    GKJOGNBOPO\x18\x98\x0c\x20\x03(\x0b2\x1d.CHDPLFOHLCN.PGKJOGNBOPOEntryR\
    \x0bPGKJOGNBOPO\x12\x20\n\x0bIPHBFHCEDGJ\x18\r\x20\x01(\rR\x0bIPHBFHCEDG\
    J\x12/\n\x0bPGNCDFGJLAA\x18\xe4\x05\x20\x03(\x0b2\x0c.PJDIBBHNNIOR\x0bPG\
    NCDFGJLAA\x129\n\x0bKAHOEKAEFHD\x18\xcb\x05\x20\x01(\x0b2\x16.EvolveBuil\
    dBattleInfoR\x0bKAHOEKAEFHD\x12.\n\x0bKHOPBJOOJNC\x18\x05\x20\x03(\x0b2\
    \x0c.LLDJJOIOOMPR\x0bKHOPBJOOJNC\x12\x20\n\x0bKALINKEDDBE\x18\t\x20\x01(\
    \rR\x0bKALINKEDDBE\x12/\n\x0bDHNDKIFPOLF\x18\xe9\x06\x20\x01(\x0b2\x0c.K\
    EJJGGCCGANR\x0bDHNDKIFPOLF\x12\x20\n\x0bCAIECHBOLMJ\x18\n\x20\x01(\rR\
    \x0bCAIECHBOLMJ\x12.\n\x0bDKNMCKJOIPL\x18\x02\x20\x03(\x0b2\x0c.KAOOHLAK\
    BPNR\x0bDKNMCKJOIPL\x12\x20\n\x0bDNMJBNNJLEL\x18\x08\x20\x01(\rR\x0bDNMJ\
    BNNJLEL\x12\x20\n\x0bNLBJKFNNONH\x18\x01\x20\x01(\x08R\x0bNLBJKFNNONH\
    \x12/\n\x0bLNEFJMMOMKB\x18\xb8\x0b\x20\x01(\x0b2\x0c.HMCNJLLHBLIR\x0bLNE\
    FJMMOMKB\x1aL\n\x10PGKJOGNBOPOEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\
    \x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.JGOIHDOBEEKR\x05value\
    :\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(9);
            deps.push(super::BIMKJOHKBNO::file_descriptor().clone());
            deps.push(super::EvolveBuildBattleInfo::file_descriptor().clone());
            deps.push(super::HMCNJLLHBLI::file_descriptor().clone());
            deps.push(super::JGOIHDOBEEK::file_descriptor().clone());
            deps.push(super::KAOOHLAKBPN::file_descriptor().clone());
            deps.push(super::KEJJGGCCGAN::file_descriptor().clone());
            deps.push(super::KIGHHJDHGOA::file_descriptor().clone());
            deps.push(super::LLDJJOIOOMP::file_descriptor().clone());
            deps.push(super::PJDIBBHNNIO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CHDPLFOHLCN::generated_message_descriptor_data());
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
