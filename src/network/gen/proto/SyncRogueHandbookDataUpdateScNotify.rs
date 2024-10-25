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

//! Generated file from `SyncRogueHandbookDataUpdateScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SyncRogueHandbookDataUpdateScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SyncRogueHandbookDataUpdateScNotify {
    // message fields
    // @@protoc_insertion_point(field:SyncRogueHandbookDataUpdateScNotify.NNLPKGJLNHD)
    pub NNLPKGJLNHD: ::std::vec::Vec<super::DMDEOGJEGPI::DMDEOGJEGPI>,
    // @@protoc_insertion_point(field:SyncRogueHandbookDataUpdateScNotify.PPIMHAIDMII)
    pub PPIMHAIDMII: ::std::vec::Vec<super::AFDGAGNIGAE::AFDGAGNIGAE>,
    // @@protoc_insertion_point(field:SyncRogueHandbookDataUpdateScNotify.ECBFBBNGNPM)
    pub ECBFBBNGNPM: ::std::vec::Vec<super::BFDCJNAGBCH::BFDCJNAGBCH>,
    // @@protoc_insertion_point(field:SyncRogueHandbookDataUpdateScNotify.JJPHKACOFHP)
    pub JJPHKACOFHP: ::std::vec::Vec<super::OFNCDKNKFJJ::OFNCDKNKFJJ>,
    // @@protoc_insertion_point(field:SyncRogueHandbookDataUpdateScNotify.JDGHMMCMPJH)
    pub JDGHMMCMPJH: ::std::vec::Vec<super::BGELCFOCKOG::BGELCFOCKOG>,
    // special fields
    // @@protoc_insertion_point(special_field:SyncRogueHandbookDataUpdateScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SyncRogueHandbookDataUpdateScNotify {
    fn default() -> &'a SyncRogueHandbookDataUpdateScNotify {
        <SyncRogueHandbookDataUpdateScNotify as ::protobuf::Message>::default_instance()
    }
}

impl SyncRogueHandbookDataUpdateScNotify {
    pub fn new() -> SyncRogueHandbookDataUpdateScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NNLPKGJLNHD",
            |m: &SyncRogueHandbookDataUpdateScNotify| { &m.NNLPKGJLNHD },
            |m: &mut SyncRogueHandbookDataUpdateScNotify| { &mut m.NNLPKGJLNHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PPIMHAIDMII",
            |m: &SyncRogueHandbookDataUpdateScNotify| { &m.PPIMHAIDMII },
            |m: &mut SyncRogueHandbookDataUpdateScNotify| { &mut m.PPIMHAIDMII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ECBFBBNGNPM",
            |m: &SyncRogueHandbookDataUpdateScNotify| { &m.ECBFBBNGNPM },
            |m: &mut SyncRogueHandbookDataUpdateScNotify| { &mut m.ECBFBBNGNPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JJPHKACOFHP",
            |m: &SyncRogueHandbookDataUpdateScNotify| { &m.JJPHKACOFHP },
            |m: &mut SyncRogueHandbookDataUpdateScNotify| { &mut m.JJPHKACOFHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JDGHMMCMPJH",
            |m: &SyncRogueHandbookDataUpdateScNotify| { &m.JDGHMMCMPJH },
            |m: &mut SyncRogueHandbookDataUpdateScNotify| { &mut m.JDGHMMCMPJH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SyncRogueHandbookDataUpdateScNotify>(
            "SyncRogueHandbookDataUpdateScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SyncRogueHandbookDataUpdateScNotify {
    const NAME: &'static str = "SyncRogueHandbookDataUpdateScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.NNLPKGJLNHD.push(is.read_message()?);
                },
                90 => {
                    self.PPIMHAIDMII.push(is.read_message()?);
                },
                42 => {
                    self.ECBFBBNGNPM.push(is.read_message()?);
                },
                82 => {
                    self.JJPHKACOFHP.push(is.read_message()?);
                },
                10 => {
                    self.JDGHMMCMPJH.push(is.read_message()?);
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
        for value in &self.NNLPKGJLNHD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PPIMHAIDMII {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.ECBFBBNGNPM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.JJPHKACOFHP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.JDGHMMCMPJH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NNLPKGJLNHD {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.PPIMHAIDMII {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.ECBFBBNGNPM {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.JJPHKACOFHP {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.JDGHMMCMPJH {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> SyncRogueHandbookDataUpdateScNotify {
        SyncRogueHandbookDataUpdateScNotify::new()
    }

    fn clear(&mut self) {
        self.NNLPKGJLNHD.clear();
        self.PPIMHAIDMII.clear();
        self.ECBFBBNGNPM.clear();
        self.JJPHKACOFHP.clear();
        self.JDGHMMCMPJH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SyncRogueHandbookDataUpdateScNotify {
        static instance: SyncRogueHandbookDataUpdateScNotify = SyncRogueHandbookDataUpdateScNotify {
            NNLPKGJLNHD: ::std::vec::Vec::new(),
            PPIMHAIDMII: ::std::vec::Vec::new(),
            ECBFBBNGNPM: ::std::vec::Vec::new(),
            JJPHKACOFHP: ::std::vec::Vec::new(),
            JDGHMMCMPJH: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SyncRogueHandbookDataUpdateScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SyncRogueHandbookDataUpdateScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SyncRogueHandbookDataUpdateScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncRogueHandbookDataUpdateScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)SyncRogueHandbookDataUpdateScNotify.proto\x1a\x11AFDGAGNIGAE.proto\
    \x1a\x11BFDCJNAGBCH.proto\x1a\x11BGELCFOCKOG.proto\x1a\x11DMDEOGJEGPI.pr\
    oto\x1a\x11OFNCDKNKFJJ.proto\"\x95\x02\n#SyncRogueHandbookDataUpdateScNo\
    tify\x12.\n\x0bNNLPKGJLNHD\x18\x0e\x20\x03(\x0b2\x0c.DMDEOGJEGPIR\x0bNNL\
    PKGJLNHD\x12.\n\x0bPPIMHAIDMII\x18\x0b\x20\x03(\x0b2\x0c.AFDGAGNIGAER\
    \x0bPPIMHAIDMII\x12.\n\x0bECBFBBNGNPM\x18\x05\x20\x03(\x0b2\x0c.BFDCJNAG\
    BCHR\x0bECBFBBNGNPM\x12.\n\x0bJJPHKACOFHP\x18\n\x20\x03(\x0b2\x0c.OFNCDK\
    NKFJJR\x0bJJPHKACOFHP\x12.\n\x0bJDGHMMCMPJH\x18\x01\x20\x03(\x0b2\x0c.BG\
    ELCFOCKOGR\x0bJDGHMMCMPJHb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::AFDGAGNIGAE::file_descriptor().clone());
            deps.push(super::BFDCJNAGBCH::file_descriptor().clone());
            deps.push(super::BGELCFOCKOG::file_descriptor().clone());
            deps.push(super::DMDEOGJEGPI::file_descriptor().clone());
            deps.push(super::OFNCDKNKFJJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SyncRogueHandbookDataUpdateScNotify::generated_message_descriptor_data());
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