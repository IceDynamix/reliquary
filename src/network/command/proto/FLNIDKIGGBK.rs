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

//! Generated file from `FLNIDKIGGBK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FLNIDKIGGBK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FLNIDKIGGBK {
    // message fields
    // @@protoc_insertion_point(field:FLNIDKIGGBK.BKMAMGAPEGH)
    pub BKMAMGAPEGH: u32,
    // @@protoc_insertion_point(field:FLNIDKIGGBK.PEHINGJKGCB)
    pub PEHINGJKGCB: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:FLNIDKIGGBK.DKHIGCIPEKF)
    pub DKHIGCIPEKF: bool,
    // @@protoc_insertion_point(field:FLNIDKIGGBK.KNEINMNLCDI)
    pub KNEINMNLCDI: ::protobuf::MessageField<super::IIKNGNHDMFI::IIKNGNHDMFI>,
    // @@protoc_insertion_point(field:FLNIDKIGGBK.OEBAFBIGMBC)
    pub OEBAFBIGMBC: ::std::vec::Vec<super::NPAIINEKEFB::NPAIINEKEFB>,
    // @@protoc_insertion_point(field:FLNIDKIGGBK.GJPANOCNGBM)
    pub GJPANOCNGBM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FLNIDKIGGBK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FLNIDKIGGBK {
    fn default() -> &'a FLNIDKIGGBK {
        <FLNIDKIGGBK as ::protobuf::Message>::default_instance()
    }
}

impl FLNIDKIGGBK {
    pub fn new() -> FLNIDKIGGBK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKMAMGAPEGH",
            |m: &FLNIDKIGGBK| { &m.BKMAMGAPEGH },
            |m: &mut FLNIDKIGGBK| { &mut m.BKMAMGAPEGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "PEHINGJKGCB",
            |m: &FLNIDKIGGBK| { &m.PEHINGJKGCB },
            |m: &mut FLNIDKIGGBK| { &mut m.PEHINGJKGCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKHIGCIPEKF",
            |m: &FLNIDKIGGBK| { &m.DKHIGCIPEKF },
            |m: &mut FLNIDKIGGBK| { &mut m.DKHIGCIPEKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IIKNGNHDMFI::IIKNGNHDMFI>(
            "KNEINMNLCDI",
            |m: &FLNIDKIGGBK| { &m.KNEINMNLCDI },
            |m: &mut FLNIDKIGGBK| { &mut m.KNEINMNLCDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OEBAFBIGMBC",
            |m: &FLNIDKIGGBK| { &m.OEBAFBIGMBC },
            |m: &mut FLNIDKIGGBK| { &mut m.OEBAFBIGMBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJPANOCNGBM",
            |m: &FLNIDKIGGBK| { &m.GJPANOCNGBM },
            |m: &mut FLNIDKIGGBK| { &mut m.GJPANOCNGBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FLNIDKIGGBK>(
            "FLNIDKIGGBK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FLNIDKIGGBK {
    const NAME: &'static str = "FLNIDKIGGBK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.BKMAMGAPEGH = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PEHINGJKGCB)?;
                },
                56 => {
                    self.DKHIGCIPEKF = is.read_bool()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KNEINMNLCDI)?;
                },
                90 => {
                    self.OEBAFBIGMBC.push(is.read_message()?);
                },
                80 => {
                    self.GJPANOCNGBM = is.read_uint32()?;
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
        if self.BKMAMGAPEGH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BKMAMGAPEGH);
        }
        if let Some(v) = self.PEHINGJKGCB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DKHIGCIPEKF != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.KNEINMNLCDI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.OEBAFBIGMBC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.GJPANOCNGBM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.GJPANOCNGBM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BKMAMGAPEGH != 0 {
            os.write_uint32(3, self.BKMAMGAPEGH)?;
        }
        if let Some(v) = self.PEHINGJKGCB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.DKHIGCIPEKF != false {
            os.write_bool(7, self.DKHIGCIPEKF)?;
        }
        if let Some(v) = self.KNEINMNLCDI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.OEBAFBIGMBC {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.GJPANOCNGBM != 0 {
            os.write_uint32(10, self.GJPANOCNGBM)?;
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

    fn new() -> FLNIDKIGGBK {
        FLNIDKIGGBK::new()
    }

    fn clear(&mut self) {
        self.BKMAMGAPEGH = 0;
        self.PEHINGJKGCB.clear();
        self.DKHIGCIPEKF = false;
        self.KNEINMNLCDI.clear();
        self.OEBAFBIGMBC.clear();
        self.GJPANOCNGBM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FLNIDKIGGBK {
        static instance: FLNIDKIGGBK = FLNIDKIGGBK {
            BKMAMGAPEGH: 0,
            PEHINGJKGCB: ::protobuf::MessageField::none(),
            DKHIGCIPEKF: false,
            KNEINMNLCDI: ::protobuf::MessageField::none(),
            OEBAFBIGMBC: ::std::vec::Vec::new(),
            GJPANOCNGBM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FLNIDKIGGBK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FLNIDKIGGBK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FLNIDKIGGBK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FLNIDKIGGBK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FLNIDKIGGBK.proto\x1a\x11IIKNGNHDMFI.proto\x1a\x11NPAIINEKEFB.prot\
    o\"\x83\x02\n\x0bFLNIDKIGGBK\x12\x20\n\x0bBKMAMGAPEGH\x18\x03\x20\x01(\r\
    R\x0bBKMAMGAPEGH\x12.\n\x0bPEHINGJKGCB\x18\x01\x20\x01(\x0b2\x0c.IIKNGNH\
    DMFIR\x0bPEHINGJKGCB\x12\x20\n\x0bDKHIGCIPEKF\x18\x07\x20\x01(\x08R\x0bD\
    KHIGCIPEKF\x12.\n\x0bKNEINMNLCDI\x18\x0f\x20\x01(\x0b2\x0c.IIKNGNHDMFIR\
    \x0bKNEINMNLCDI\x12.\n\x0bOEBAFBIGMBC\x18\x0b\x20\x03(\x0b2\x0c.NPAIINEK\
    EFBR\x0bOEBAFBIGMBC\x12\x20\n\x0bGJPANOCNGBM\x18\n\x20\x01(\rR\x0bGJPANO\
    CNGBMb\x06proto3\
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
            deps.push(super::IIKNGNHDMFI::file_descriptor().clone());
            deps.push(super::NPAIINEKEFB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FLNIDKIGGBK::generated_message_descriptor_data());
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
