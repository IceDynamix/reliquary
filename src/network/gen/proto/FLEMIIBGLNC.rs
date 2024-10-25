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

//! Generated file from `FLEMIIBGLNC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FLEMIIBGLNC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FLEMIIBGLNC {
    // message fields
    // @@protoc_insertion_point(field:FLEMIIBGLNC.LELABBBOIKN)
    pub LELABBBOIKN: ::protobuf::EnumOrUnknown<super::BLBJBAMDNBN::BLBJBAMDNBN>,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.OOGAPOKFKAI)
    pub OOGAPOKFKAI: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.rank)
    pub rank: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.GCCIOHEJPNE)
    pub GCCIOHEJPNE: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.FILMAOEBILH)
    pub FILMAOEBILH: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.KENEFEHNNDI)
    pub KENEFEHNNDI: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.DNAMDNCLOJC)
    pub DNAMDNCLOJC: u32,
    // @@protoc_insertion_point(field:FLEMIIBGLNC.EKFPKFECMGC)
    pub EKFPKFECMGC: bool,
    // special fields
    // @@protoc_insertion_point(special_field:FLEMIIBGLNC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FLEMIIBGLNC {
    fn default() -> &'a FLEMIIBGLNC {
        <FLEMIIBGLNC as ::protobuf::Message>::default_instance()
    }
}

impl FLEMIIBGLNC {
    pub fn new() -> FLEMIIBGLNC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LELABBBOIKN",
            |m: &FLEMIIBGLNC| { &m.LELABBBOIKN },
            |m: &mut FLEMIIBGLNC| { &mut m.LELABBBOIKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OOGAPOKFKAI",
            |m: &FLEMIIBGLNC| { &m.OOGAPOKFKAI },
            |m: &mut FLEMIIBGLNC| { &mut m.OOGAPOKFKAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &FLEMIIBGLNC| { &m.rank },
            |m: &mut FLEMIIBGLNC| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCCIOHEJPNE",
            |m: &FLEMIIBGLNC| { &m.GCCIOHEJPNE },
            |m: &mut FLEMIIBGLNC| { &mut m.GCCIOHEJPNE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FILMAOEBILH",
            |m: &FLEMIIBGLNC| { &m.FILMAOEBILH },
            |m: &mut FLEMIIBGLNC| { &mut m.FILMAOEBILH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KENEFEHNNDI",
            |m: &FLEMIIBGLNC| { &m.KENEFEHNNDI },
            |m: &mut FLEMIIBGLNC| { &mut m.KENEFEHNNDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNAMDNCLOJC",
            |m: &FLEMIIBGLNC| { &m.DNAMDNCLOJC },
            |m: &mut FLEMIIBGLNC| { &mut m.DNAMDNCLOJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKFPKFECMGC",
            |m: &FLEMIIBGLNC| { &m.EKFPKFECMGC },
            |m: &mut FLEMIIBGLNC| { &mut m.EKFPKFECMGC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FLEMIIBGLNC>(
            "FLEMIIBGLNC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FLEMIIBGLNC {
    const NAME: &'static str = "FLEMIIBGLNC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.LELABBBOIKN = is.read_enum_or_unknown()?;
                },
                96 => {
                    self.OOGAPOKFKAI = is.read_uint32()?;
                },
                8 => {
                    self.rank = is.read_uint32()?;
                },
                64 => {
                    self.GCCIOHEJPNE = is.read_uint32()?;
                },
                120 => {
                    self.FILMAOEBILH = is.read_uint32()?;
                },
                104 => {
                    self.KENEFEHNNDI = is.read_uint32()?;
                },
                88 => {
                    self.DNAMDNCLOJC = is.read_uint32()?;
                },
                40 => {
                    self.EKFPKFECMGC = is.read_bool()?;
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
        if self.LELABBBOIKN != ::protobuf::EnumOrUnknown::new(super::BLBJBAMDNBN::BLBJBAMDNBN::MATCH3_PLAYER_STATE_ALIVE) {
            my_size += ::protobuf::rt::int32_size(14, self.LELABBBOIKN.value());
        }
        if self.OOGAPOKFKAI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.OOGAPOKFKAI);
        }
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.rank);
        }
        if self.GCCIOHEJPNE != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GCCIOHEJPNE);
        }
        if self.FILMAOEBILH != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.FILMAOEBILH);
        }
        if self.KENEFEHNNDI != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.KENEFEHNNDI);
        }
        if self.DNAMDNCLOJC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.DNAMDNCLOJC);
        }
        if self.EKFPKFECMGC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LELABBBOIKN != ::protobuf::EnumOrUnknown::new(super::BLBJBAMDNBN::BLBJBAMDNBN::MATCH3_PLAYER_STATE_ALIVE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.LELABBBOIKN))?;
        }
        if self.OOGAPOKFKAI != 0 {
            os.write_uint32(12, self.OOGAPOKFKAI)?;
        }
        if self.rank != 0 {
            os.write_uint32(1, self.rank)?;
        }
        if self.GCCIOHEJPNE != 0 {
            os.write_uint32(8, self.GCCIOHEJPNE)?;
        }
        if self.FILMAOEBILH != 0 {
            os.write_uint32(15, self.FILMAOEBILH)?;
        }
        if self.KENEFEHNNDI != 0 {
            os.write_uint32(13, self.KENEFEHNNDI)?;
        }
        if self.DNAMDNCLOJC != 0 {
            os.write_uint32(11, self.DNAMDNCLOJC)?;
        }
        if self.EKFPKFECMGC != false {
            os.write_bool(5, self.EKFPKFECMGC)?;
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

    fn new() -> FLEMIIBGLNC {
        FLEMIIBGLNC::new()
    }

    fn clear(&mut self) {
        self.LELABBBOIKN = ::protobuf::EnumOrUnknown::new(super::BLBJBAMDNBN::BLBJBAMDNBN::MATCH3_PLAYER_STATE_ALIVE);
        self.OOGAPOKFKAI = 0;
        self.rank = 0;
        self.GCCIOHEJPNE = 0;
        self.FILMAOEBILH = 0;
        self.KENEFEHNNDI = 0;
        self.DNAMDNCLOJC = 0;
        self.EKFPKFECMGC = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FLEMIIBGLNC {
        static instance: FLEMIIBGLNC = FLEMIIBGLNC {
            LELABBBOIKN: ::protobuf::EnumOrUnknown::from_i32(0),
            OOGAPOKFKAI: 0,
            rank: 0,
            GCCIOHEJPNE: 0,
            FILMAOEBILH: 0,
            KENEFEHNNDI: 0,
            DNAMDNCLOJC: 0,
            EKFPKFECMGC: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FLEMIIBGLNC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FLEMIIBGLNC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FLEMIIBGLNC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FLEMIIBGLNC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FLEMIIBGLNC.proto\x1a\x11BLBJBAMDNBN.proto\"\x9d\x02\n\x0bFLEMIIBG\
    LNC\x12.\n\x0bLELABBBOIKN\x18\x0e\x20\x01(\x0e2\x0c.BLBJBAMDNBNR\x0bLELA\
    BBBOIKN\x12\x20\n\x0bOOGAPOKFKAI\x18\x0c\x20\x01(\rR\x0bOOGAPOKFKAI\x12\
    \x12\n\x04rank\x18\x01\x20\x01(\rR\x04rank\x12\x20\n\x0bGCCIOHEJPNE\x18\
    \x08\x20\x01(\rR\x0bGCCIOHEJPNE\x12\x20\n\x0bFILMAOEBILH\x18\x0f\x20\x01\
    (\rR\x0bFILMAOEBILH\x12\x20\n\x0bKENEFEHNNDI\x18\r\x20\x01(\rR\x0bKENEFE\
    HNNDI\x12\x20\n\x0bDNAMDNCLOJC\x18\x0b\x20\x01(\rR\x0bDNAMDNCLOJC\x12\
    \x20\n\x0bEKFPKFECMGC\x18\x05\x20\x01(\x08R\x0bEKFPKFECMGCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::BLBJBAMDNBN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FLEMIIBGLNC::generated_message_descriptor_data());
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
