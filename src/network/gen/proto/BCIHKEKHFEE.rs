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

//! Generated file from `BCIHKEKHFEE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BCIHKEKHFEE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BCIHKEKHFEE {
    // message fields
    // @@protoc_insertion_point(field:BCIHKEKHFEE.PFDIJNMPDMM)
    pub PFDIJNMPDMM: ::std::vec::Vec<super::CAHEEDNLCOD::CAHEEDNLCOD>,
    // @@protoc_insertion_point(field:BCIHKEKHFEE.MAHHHLDNAFP)
    pub MAHHHLDNAFP: ::std::vec::Vec<super::FDMIHCJCAEM::FDMIHCJCAEM>,
    // @@protoc_insertion_point(field:BCIHKEKHFEE.JDIGFNALJFN)
    pub JDIGFNALJFN: ::protobuf::MessageField<super::PBPDBCCFBGH::PBPDBCCFBGH>,
    // @@protoc_insertion_point(field:BCIHKEKHFEE.BLHKEFPJGFG)
    pub BLHKEFPJGFG: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BCIHKEKHFEE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BCIHKEKHFEE {
    fn default() -> &'a BCIHKEKHFEE {
        <BCIHKEKHFEE as ::protobuf::Message>::default_instance()
    }
}

impl BCIHKEKHFEE {
    pub fn new() -> BCIHKEKHFEE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PFDIJNMPDMM",
            |m: &BCIHKEKHFEE| { &m.PFDIJNMPDMM },
            |m: &mut BCIHKEKHFEE| { &mut m.PFDIJNMPDMM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MAHHHLDNAFP",
            |m: &BCIHKEKHFEE| { &m.MAHHHLDNAFP },
            |m: &mut BCIHKEKHFEE| { &mut m.MAHHHLDNAFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PBPDBCCFBGH::PBPDBCCFBGH>(
            "JDIGFNALJFN",
            |m: &BCIHKEKHFEE| { &m.JDIGFNALJFN },
            |m: &mut BCIHKEKHFEE| { &mut m.JDIGFNALJFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "BLHKEFPJGFG",
            |m: &BCIHKEKHFEE| { &m.BLHKEFPJGFG },
            |m: &mut BCIHKEKHFEE| { &mut m.BLHKEFPJGFG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BCIHKEKHFEE>(
            "BCIHKEKHFEE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BCIHKEKHFEE {
    const NAME: &'static str = "BCIHKEKHFEE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.PFDIJNMPDMM.push(is.read_message()?);
                },
                50 => {
                    self.MAHHHLDNAFP.push(is.read_message()?);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JDIGFNALJFN)?;
                },
                122 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.BLHKEFPJGFG.insert(key, value);
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
        for value in &self.PFDIJNMPDMM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MAHHHLDNAFP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.JDIGFNALJFN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for (k, v) in &self.BLHKEFPJGFG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PFDIJNMPDMM {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.MAHHHLDNAFP {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if let Some(v) = self.JDIGFNALJFN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for (k, v) in &self.BLHKEFPJGFG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BCIHKEKHFEE {
        BCIHKEKHFEE::new()
    }

    fn clear(&mut self) {
        self.PFDIJNMPDMM.clear();
        self.MAHHHLDNAFP.clear();
        self.JDIGFNALJFN.clear();
        self.BLHKEFPJGFG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BCIHKEKHFEE {
        static instance: ::protobuf::rt::Lazy<BCIHKEKHFEE> = ::protobuf::rt::Lazy::new();
        instance.get(BCIHKEKHFEE::new)
    }
}

impl ::protobuf::MessageFull for BCIHKEKHFEE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BCIHKEKHFEE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BCIHKEKHFEE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BCIHKEKHFEE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BCIHKEKHFEE.proto\x1a\x11CAHEEDNLCOD.proto\x1a\x11FDMIHCJCAEM.prot\
    o\x1a\x11PBPDBCCFBGH.proto\"\x9e\x02\n\x0bBCIHKEKHFEE\x12.\n\x0bPFDIJNMP\
    DMM\x18\x08\x20\x03(\x0b2\x0c.CAHEEDNLCODR\x0bPFDIJNMPDMM\x12.\n\x0bMAHH\
    HLDNAFP\x18\x06\x20\x03(\x0b2\x0c.FDMIHCJCAEMR\x0bMAHHHLDNAFP\x12.\n\x0b\
    JDIGFNALJFN\x18\n\x20\x01(\x0b2\x0c.PBPDBCCFBGHR\x0bJDIGFNALJFN\x12?\n\
    \x0bBLHKEFPJGFG\x18\x0f\x20\x03(\x0b2\x1d.BCIHKEKHFEE.BLHKEFPJGFGEntryR\
    \x0bBLHKEFPJGFG\x1a>\n\x10BLHKEFPJGFGEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\
    \x01b\x06proto3\
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
            deps.push(super::CAHEEDNLCOD::file_descriptor().clone());
            deps.push(super::FDMIHCJCAEM::file_descriptor().clone());
            deps.push(super::PBPDBCCFBGH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BCIHKEKHFEE::generated_message_descriptor_data());
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
