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

//! Generated file from `MatchThreeLevelEndCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MatchThreeLevelEndCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MatchThreeLevelEndCsReq {
    // message fields
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.JEPPFDINBNB)
    pub JEPPFDINBNB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.FMKKABMDINJ)
    pub FMKKABMDINJ: u32,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.ILBHDLMLMCK)
    pub ILBHDLMLMCK: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.BKMPFEOCFIB)
    pub BKMPFEOCFIB: u32,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.uuid)
    pub uuid: ::std::string::String,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.EBGMBDMPEGM)
    pub EBGMBDMPEGM: u32,
    // @@protoc_insertion_point(field:MatchThreeLevelEndCsReq.ACJCPHIFMLN)
    pub ACJCPHIFMLN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MatchThreeLevelEndCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MatchThreeLevelEndCsReq {
    fn default() -> &'a MatchThreeLevelEndCsReq {
        <MatchThreeLevelEndCsReq as ::protobuf::Message>::default_instance()
    }
}

impl MatchThreeLevelEndCsReq {
    pub fn new() -> MatchThreeLevelEndCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JEPPFDINBNB",
            |m: &MatchThreeLevelEndCsReq| { &m.JEPPFDINBNB },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.JEPPFDINBNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMKKABMDINJ",
            |m: &MatchThreeLevelEndCsReq| { &m.FMKKABMDINJ },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.FMKKABMDINJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "ILBHDLMLMCK",
            |m: &MatchThreeLevelEndCsReq| { &m.ILBHDLMLMCK },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.ILBHDLMLMCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKMPFEOCFIB",
            |m: &MatchThreeLevelEndCsReq| { &m.BKMPFEOCFIB },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.BKMPFEOCFIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uuid",
            |m: &MatchThreeLevelEndCsReq| { &m.uuid },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.uuid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBGMBDMPEGM",
            |m: &MatchThreeLevelEndCsReq| { &m.EBGMBDMPEGM },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.EBGMBDMPEGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ACJCPHIFMLN",
            |m: &MatchThreeLevelEndCsReq| { &m.ACJCPHIFMLN },
            |m: &mut MatchThreeLevelEndCsReq| { &mut m.ACJCPHIFMLN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MatchThreeLevelEndCsReq>(
            "MatchThreeLevelEndCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MatchThreeLevelEndCsReq {
    const NAME: &'static str = "MatchThreeLevelEndCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.JEPPFDINBNB)?;
                },
                72 => {
                    self.JEPPFDINBNB.push(is.read_uint32()?);
                },
                104 => {
                    self.FMKKABMDINJ = is.read_uint32()?;
                },
                98 => {
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
                    self.ILBHDLMLMCK.insert(key, value);
                },
                112 => {
                    self.BKMPFEOCFIB = is.read_uint32()?;
                },
                26 => {
                    self.uuid = is.read_string()?;
                },
                56 => {
                    self.EBGMBDMPEGM = is.read_uint32()?;
                },
                48 => {
                    self.ACJCPHIFMLN = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.JEPPFDINBNB);
        if self.FMKKABMDINJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.FMKKABMDINJ);
        }
        for (k, v) in &self.ILBHDLMLMCK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.BKMPFEOCFIB != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.BKMPFEOCFIB);
        }
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.uuid);
        }
        if self.EBGMBDMPEGM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.EBGMBDMPEGM);
        }
        if self.ACJCPHIFMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ACJCPHIFMLN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(9, &self.JEPPFDINBNB)?;
        if self.FMKKABMDINJ != 0 {
            os.write_uint32(13, self.FMKKABMDINJ)?;
        }
        for (k, v) in &self.ILBHDLMLMCK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(98)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.BKMPFEOCFIB != 0 {
            os.write_uint32(14, self.BKMPFEOCFIB)?;
        }
        if !self.uuid.is_empty() {
            os.write_string(3, &self.uuid)?;
        }
        if self.EBGMBDMPEGM != 0 {
            os.write_uint32(7, self.EBGMBDMPEGM)?;
        }
        if self.ACJCPHIFMLN != 0 {
            os.write_uint32(6, self.ACJCPHIFMLN)?;
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

    fn new() -> MatchThreeLevelEndCsReq {
        MatchThreeLevelEndCsReq::new()
    }

    fn clear(&mut self) {
        self.JEPPFDINBNB.clear();
        self.FMKKABMDINJ = 0;
        self.ILBHDLMLMCK.clear();
        self.BKMPFEOCFIB = 0;
        self.uuid.clear();
        self.EBGMBDMPEGM = 0;
        self.ACJCPHIFMLN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MatchThreeLevelEndCsReq {
        static instance: ::protobuf::rt::Lazy<MatchThreeLevelEndCsReq> = ::protobuf::rt::Lazy::new();
        instance.get(MatchThreeLevelEndCsReq::new)
    }
}

impl ::protobuf::MessageFull for MatchThreeLevelEndCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MatchThreeLevelEndCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MatchThreeLevelEndCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MatchThreeLevelEndCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dMatchThreeLevelEndCsReq.proto\"\xe4\x02\n\x17MatchThreeLevelEndCsR\
    eq\x12\x20\n\x0bJEPPFDINBNB\x18\t\x20\x03(\rR\x0bJEPPFDINBNB\x12\x20\n\
    \x0bFMKKABMDINJ\x18\r\x20\x01(\rR\x0bFMKKABMDINJ\x12K\n\x0bILBHDLMLMCK\
    \x18\x0c\x20\x03(\x0b2).MatchThreeLevelEndCsReq.ILBHDLMLMCKEntryR\x0bILB\
    HDLMLMCK\x12\x20\n\x0bBKMPFEOCFIB\x18\x0e\x20\x01(\rR\x0bBKMPFEOCFIB\x12\
    \x12\n\x04uuid\x18\x03\x20\x01(\tR\x04uuid\x12\x20\n\x0bEBGMBDMPEGM\x18\
    \x07\x20\x01(\rR\x0bEBGMBDMPEGM\x12\x20\n\x0bACJCPHIFMLN\x18\x06\x20\x01\
    (\rR\x0bACJCPHIFMLN\x1a>\n\x10ILBHDLMLMCKEntry\x12\x10\n\x03key\x18\x01\
    \x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x02\
    8\x01b\x06proto3\
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
            messages.push(MatchThreeLevelEndCsReq::generated_message_descriptor_data());
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
