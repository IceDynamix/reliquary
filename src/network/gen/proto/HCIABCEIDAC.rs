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

//! Generated file from `HCIABCEIDAC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HCIABCEIDAC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HCIABCEIDAC {
    // message fields
    // @@protoc_insertion_point(field:HCIABCEIDAC.HEKDEHADKAO)
    pub HEKDEHADKAO: ::std::collections::HashMap<::std::string::String, f32>,
    // @@protoc_insertion_point(field:HCIABCEIDAC.GPGBNLJGHCD)
    pub GPGBNLJGHCD: u32,
    // @@protoc_insertion_point(field:HCIABCEIDAC.level)
    pub level: u32,
    // @@protoc_insertion_point(field:HCIABCEIDAC.NPONEEKCLOH)
    pub NPONEEKCLOH: u32,
    // @@protoc_insertion_point(field:HCIABCEIDAC.LDHAIOFHFLF)
    pub LDHAIOFHFLF: u32,
    // @@protoc_insertion_point(field:HCIABCEIDAC.ACLNAMNHCHE)
    pub ACLNAMNHCHE: u64,
    // @@protoc_insertion_point(field:HCIABCEIDAC.HOHOKFOEHGJ)
    pub HOHOKFOEHGJ: f32,
    // @@protoc_insertion_point(field:HCIABCEIDAC.base_avatar_id)
    pub base_avatar_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HCIABCEIDAC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HCIABCEIDAC {
    fn default() -> &'a HCIABCEIDAC {
        <HCIABCEIDAC as ::protobuf::Message>::default_instance()
    }
}

impl HCIABCEIDAC {
    pub fn new() -> HCIABCEIDAC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "HEKDEHADKAO",
            |m: &HCIABCEIDAC| { &m.HEKDEHADKAO },
            |m: &mut HCIABCEIDAC| { &mut m.HEKDEHADKAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GPGBNLJGHCD",
            |m: &HCIABCEIDAC| { &m.GPGBNLJGHCD },
            |m: &mut HCIABCEIDAC| { &mut m.GPGBNLJGHCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &HCIABCEIDAC| { &m.level },
            |m: &mut HCIABCEIDAC| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPONEEKCLOH",
            |m: &HCIABCEIDAC| { &m.NPONEEKCLOH },
            |m: &mut HCIABCEIDAC| { &mut m.NPONEEKCLOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LDHAIOFHFLF",
            |m: &HCIABCEIDAC| { &m.LDHAIOFHFLF },
            |m: &mut HCIABCEIDAC| { &mut m.LDHAIOFHFLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACLNAMNHCHE",
            |m: &HCIABCEIDAC| { &m.ACLNAMNHCHE },
            |m: &mut HCIABCEIDAC| { &mut m.ACLNAMNHCHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HOHOKFOEHGJ",
            |m: &HCIABCEIDAC| { &m.HOHOKFOEHGJ },
            |m: &mut HCIABCEIDAC| { &mut m.HOHOKFOEHGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &HCIABCEIDAC| { &m.base_avatar_id },
            |m: &mut HCIABCEIDAC| { &mut m.base_avatar_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HCIABCEIDAC>(
            "HCIABCEIDAC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HCIABCEIDAC {
    const NAME: &'static str = "HCIABCEIDAC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            21 => value = is.read_float()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.HEKDEHADKAO.insert(key, value);
                },
                104 => {
                    self.GPGBNLJGHCD = is.read_uint32()?;
                },
                40 => {
                    self.level = is.read_uint32()?;
                },
                56 => {
                    self.NPONEEKCLOH = is.read_uint32()?;
                },
                48 => {
                    self.LDHAIOFHFLF = is.read_uint32()?;
                },
                72 => {
                    self.ACLNAMNHCHE = is.read_uint64()?;
                },
                13 => {
                    self.HOHOKFOEHGJ = is.read_float()?;
                },
                112 => {
                    self.base_avatar_id = is.read_uint32()?;
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
        for (k, v) in &self.HEKDEHADKAO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.GPGBNLJGHCD != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.GPGBNLJGHCD);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.level);
        }
        if self.NPONEEKCLOH != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NPONEEKCLOH);
        }
        if self.LDHAIOFHFLF != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.LDHAIOFHFLF);
        }
        if self.ACLNAMNHCHE != 0 {
            my_size += ::protobuf::rt::uint64_size(9, self.ACLNAMNHCHE);
        }
        if self.HOHOKFOEHGJ != 0. {
            my_size += 1 + 4;
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.base_avatar_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.HEKDEHADKAO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += 1 + 4;
            os.write_raw_varint32(26)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_float(2, *v)?;
        };
        if self.GPGBNLJGHCD != 0 {
            os.write_uint32(13, self.GPGBNLJGHCD)?;
        }
        if self.level != 0 {
            os.write_uint32(5, self.level)?;
        }
        if self.NPONEEKCLOH != 0 {
            os.write_uint32(7, self.NPONEEKCLOH)?;
        }
        if self.LDHAIOFHFLF != 0 {
            os.write_uint32(6, self.LDHAIOFHFLF)?;
        }
        if self.ACLNAMNHCHE != 0 {
            os.write_uint64(9, self.ACLNAMNHCHE)?;
        }
        if self.HOHOKFOEHGJ != 0. {
            os.write_float(1, self.HOHOKFOEHGJ)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(14, self.base_avatar_id)?;
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

    fn new() -> HCIABCEIDAC {
        HCIABCEIDAC::new()
    }

    fn clear(&mut self) {
        self.HEKDEHADKAO.clear();
        self.GPGBNLJGHCD = 0;
        self.level = 0;
        self.NPONEEKCLOH = 0;
        self.LDHAIOFHFLF = 0;
        self.ACLNAMNHCHE = 0;
        self.HOHOKFOEHGJ = 0.;
        self.base_avatar_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HCIABCEIDAC {
        static instance: ::protobuf::rt::Lazy<HCIABCEIDAC> = ::protobuf::rt::Lazy::new();
        instance.get(HCIABCEIDAC::new)
    }
}

impl ::protobuf::MessageFull for HCIABCEIDAC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HCIABCEIDAC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HCIABCEIDAC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HCIABCEIDAC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HCIABCEIDAC.proto\"\xf4\x02\n\x0bHCIABCEIDAC\x12?\n\x0bHEKDEHADKAO\
    \x18\x03\x20\x03(\x0b2\x1d.HCIABCEIDAC.HEKDEHADKAOEntryR\x0bHEKDEHADKAO\
    \x12\x20\n\x0bGPGBNLJGHCD\x18\r\x20\x01(\rR\x0bGPGBNLJGHCD\x12\x14\n\x05\
    level\x18\x05\x20\x01(\rR\x05level\x12\x20\n\x0bNPONEEKCLOH\x18\x07\x20\
    \x01(\rR\x0bNPONEEKCLOH\x12\x20\n\x0bLDHAIOFHFLF\x18\x06\x20\x01(\rR\x0b\
    LDHAIOFHFLF\x12\x20\n\x0bACLNAMNHCHE\x18\t\x20\x01(\x04R\x0bACLNAMNHCHE\
    \x12\x20\n\x0bHOHOKFOEHGJ\x18\x01\x20\x01(\x02R\x0bHOHOKFOEHGJ\x12$\n\
    \x0ebase_avatar_id\x18\x0e\x20\x01(\rR\x0cbaseAvatarId\x1a>\n\x10HEKDEHA\
    DKAOEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\
    \x18\x02\x20\x01(\x02R\x05value:\x028\x01b\x06proto3\
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
            messages.push(HCIABCEIDAC::generated_message_descriptor_data());
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
