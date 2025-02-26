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

//! Generated file from `FightMatch3SwapCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FightMatch3SwapCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightMatch3SwapCsReq {
    // message fields
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.PHNLDPOKBKL)
    pub PHNLDPOKBKL: ::protobuf::MessageField<super::JJAEPDIHCNL::JJAEPDIHCNL>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.ECKKBLNELBM)
    pub ECKKBLNELBM: ::std::vec::Vec<super::EGCDDLKHFEB::EGCDDLKHFEB>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.FECLGLBFIDH)
    pub FECLGLBFIDH: ::protobuf::MessageField<super::JJAEPDIHCNL::JJAEPDIHCNL>,
    // @@protoc_insertion_point(field:FightMatch3SwapCsReq.BBLBLAFIGHM)
    pub BBLBLAFIGHM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightMatch3SwapCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightMatch3SwapCsReq {
    fn default() -> &'a FightMatch3SwapCsReq {
        <FightMatch3SwapCsReq as ::protobuf::Message>::default_instance()
    }
}

impl FightMatch3SwapCsReq {
    pub fn new() -> FightMatch3SwapCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JJAEPDIHCNL::JJAEPDIHCNL>(
            "PHNLDPOKBKL",
            |m: &FightMatch3SwapCsReq| { &m.PHNLDPOKBKL },
            |m: &mut FightMatch3SwapCsReq| { &mut m.PHNLDPOKBKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ECKKBLNELBM",
            |m: &FightMatch3SwapCsReq| { &m.ECKKBLNELBM },
            |m: &mut FightMatch3SwapCsReq| { &mut m.ECKKBLNELBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JJAEPDIHCNL::JJAEPDIHCNL>(
            "FECLGLBFIDH",
            |m: &FightMatch3SwapCsReq| { &m.FECLGLBFIDH },
            |m: &mut FightMatch3SwapCsReq| { &mut m.FECLGLBFIDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BBLBLAFIGHM",
            |m: &FightMatch3SwapCsReq| { &m.BBLBLAFIGHM },
            |m: &mut FightMatch3SwapCsReq| { &mut m.BBLBLAFIGHM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightMatch3SwapCsReq>(
            "FightMatch3SwapCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightMatch3SwapCsReq {
    const NAME: &'static str = "FightMatch3SwapCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PHNLDPOKBKL)?;
                },
                82 => {
                    self.ECKKBLNELBM.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FECLGLBFIDH)?;
                },
                88 => {
                    self.BBLBLAFIGHM = is.read_uint32()?;
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
        if let Some(v) = self.PHNLDPOKBKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.ECKKBLNELBM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.FECLGLBFIDH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BBLBLAFIGHM != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BBLBLAFIGHM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.PHNLDPOKBKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        for v in &self.ECKKBLNELBM {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if let Some(v) = self.FECLGLBFIDH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.BBLBLAFIGHM != 0 {
            os.write_uint32(11, self.BBLBLAFIGHM)?;
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

    fn new() -> FightMatch3SwapCsReq {
        FightMatch3SwapCsReq::new()
    }

    fn clear(&mut self) {
        self.PHNLDPOKBKL.clear();
        self.ECKKBLNELBM.clear();
        self.FECLGLBFIDH.clear();
        self.BBLBLAFIGHM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightMatch3SwapCsReq {
        static instance: FightMatch3SwapCsReq = FightMatch3SwapCsReq {
            PHNLDPOKBKL: ::protobuf::MessageField::none(),
            ECKKBLNELBM: ::std::vec::Vec::new(),
            FECLGLBFIDH: ::protobuf::MessageField::none(),
            BBLBLAFIGHM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightMatch3SwapCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightMatch3SwapCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightMatch3SwapCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightMatch3SwapCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aFightMatch3SwapCsReq.proto\x1a\x11EGCDDLKHFEB.proto\x1a\x11JJAEPDI\
    HCNL.proto\"\xc8\x01\n\x14FightMatch3SwapCsReq\x12.\n\x0bPHNLDPOKBKL\x18\
    \r\x20\x01(\x0b2\x0c.JJAEPDIHCNLR\x0bPHNLDPOKBKL\x12.\n\x0bECKKBLNELBM\
    \x18\n\x20\x03(\x0b2\x0c.EGCDDLKHFEBR\x0bECKKBLNELBM\x12.\n\x0bFECLGLBFI\
    DH\x18\x02\x20\x01(\x0b2\x0c.JJAEPDIHCNLR\x0bFECLGLBFIDH\x12\x20\n\x0bBB\
    LBLAFIGHM\x18\x0b\x20\x01(\rR\x0bBBLBLAFIGHMb\x06proto3\
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
            deps.push(super::EGCDDLKHFEB::file_descriptor().clone());
            deps.push(super::JJAEPDIHCNL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FightMatch3SwapCsReq::generated_message_descriptor_data());
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
