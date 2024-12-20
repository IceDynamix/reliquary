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

//! Generated file from `SwordTrainingGameSettleScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SwordTrainingGameSettleScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SwordTrainingGameSettleScNotify {
    // message fields
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.IKHDMKBFPMA)
    pub IKHDMKBFPMA: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.NAMPNOLBOEK)
    pub NAMPNOLBOEK: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::OJGDBABMNMH::OJGDBABMNMH>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.GFKIHHOPJDG)
    pub GFKIHHOPJDG: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.HDAIJPHEEJO)
    pub HDAIJPHEEJO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.ELPMNKHEPKJ)
    pub ELPMNKHEPKJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.NOKLENCDFNF)
    pub NOKLENCDFNF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.KOBMCLGJJDB)
    pub KOBMCLGJJDB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SwordTrainingGameSettleScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SwordTrainingGameSettleScNotify {
    fn default() -> &'a SwordTrainingGameSettleScNotify {
        <SwordTrainingGameSettleScNotify as ::protobuf::Message>::default_instance()
    }
}

impl SwordTrainingGameSettleScNotify {
    pub fn new() -> SwordTrainingGameSettleScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKHDMKBFPMA",
            |m: &SwordTrainingGameSettleScNotify| { &m.IKHDMKBFPMA },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.IKHDMKBFPMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAMPNOLBOEK",
            |m: &SwordTrainingGameSettleScNotify| { &m.NAMPNOLBOEK },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.NAMPNOLBOEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &SwordTrainingGameSettleScNotify| { &m.KGGHLADEKGP },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFKIHHOPJDG",
            |m: &SwordTrainingGameSettleScNotify| { &m.GFKIHHOPJDG },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.GFKIHHOPJDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HDAIJPHEEJO",
            |m: &SwordTrainingGameSettleScNotify| { &m.HDAIJPHEEJO },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.HDAIJPHEEJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "ELPMNKHEPKJ",
            |m: &SwordTrainingGameSettleScNotify| { &m.ELPMNKHEPKJ },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.ELPMNKHEPKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NOKLENCDFNF",
            |m: &SwordTrainingGameSettleScNotify| { &m.NOKLENCDFNF },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.NOKLENCDFNF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOBMCLGJJDB",
            |m: &SwordTrainingGameSettleScNotify| { &m.KOBMCLGJJDB },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.KOBMCLGJJDB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SwordTrainingGameSettleScNotify>(
            "SwordTrainingGameSettleScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SwordTrainingGameSettleScNotify {
    const NAME: &'static str = "SwordTrainingGameSettleScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.IKHDMKBFPMA = is.read_uint32()?;
                },
                24 => {
                    self.NAMPNOLBOEK = is.read_uint32()?;
                },
                120 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.GFKIHHOPJDG = is.read_uint32()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.HDAIJPHEEJO)?;
                },
                88 => {
                    self.HDAIJPHEEJO.push(is.read_uint32()?);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ELPMNKHEPKJ)?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.NOKLENCDFNF)?;
                },
                16 => {
                    self.NOKLENCDFNF.push(is.read_uint32()?);
                },
                72 => {
                    self.KOBMCLGJJDB = is.read_uint32()?;
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
        if self.IKHDMKBFPMA != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.IKHDMKBFPMA);
        }
        if self.NAMPNOLBOEK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NAMPNOLBOEK);
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::OJGDBABMNMH::OJGDBABMNMH::SWORD_TRAINING_GAME_SETTLE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.KGGHLADEKGP.value());
        }
        if self.GFKIHHOPJDG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GFKIHHOPJDG);
        }
        for value in &self.HDAIJPHEEJO {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.NOKLENCDFNF {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.KOBMCLGJJDB != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.KOBMCLGJJDB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IKHDMKBFPMA != 0 {
            os.write_uint32(6, self.IKHDMKBFPMA)?;
        }
        if self.NAMPNOLBOEK != 0 {
            os.write_uint32(3, self.NAMPNOLBOEK)?;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::OJGDBABMNMH::OJGDBABMNMH::SWORD_TRAINING_GAME_SETTLE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if self.GFKIHHOPJDG != 0 {
            os.write_uint32(14, self.GFKIHHOPJDG)?;
        }
        for v in &self.HDAIJPHEEJO {
            os.write_uint32(11, *v)?;
        };
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.NOKLENCDFNF {
            os.write_uint32(2, *v)?;
        };
        if self.KOBMCLGJJDB != 0 {
            os.write_uint32(9, self.KOBMCLGJJDB)?;
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

    fn new() -> SwordTrainingGameSettleScNotify {
        SwordTrainingGameSettleScNotify::new()
    }

    fn clear(&mut self) {
        self.IKHDMKBFPMA = 0;
        self.NAMPNOLBOEK = 0;
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::OJGDBABMNMH::OJGDBABMNMH::SWORD_TRAINING_GAME_SETTLE_NONE);
        self.GFKIHHOPJDG = 0;
        self.HDAIJPHEEJO.clear();
        self.ELPMNKHEPKJ.clear();
        self.NOKLENCDFNF.clear();
        self.KOBMCLGJJDB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SwordTrainingGameSettleScNotify {
        static instance: SwordTrainingGameSettleScNotify = SwordTrainingGameSettleScNotify {
            IKHDMKBFPMA: 0,
            NAMPNOLBOEK: 0,
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            GFKIHHOPJDG: 0,
            HDAIJPHEEJO: ::std::vec::Vec::new(),
            ELPMNKHEPKJ: ::protobuf::MessageField::none(),
            NOKLENCDFNF: ::std::vec::Vec::new(),
            KOBMCLGJJDB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SwordTrainingGameSettleScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SwordTrainingGameSettleScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SwordTrainingGameSettleScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SwordTrainingGameSettleScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%SwordTrainingGameSettleScNotify.proto\x1a\x0eItemList.proto\x1a\x11OJ\
    GDBABMNMH.proto\"\xca\x02\n\x1fSwordTrainingGameSettleScNotify\x12\x20\n\
    \x0bIKHDMKBFPMA\x18\x06\x20\x01(\rR\x0bIKHDMKBFPMA\x12\x20\n\x0bNAMPNOLB\
    OEK\x18\x03\x20\x01(\rR\x0bNAMPNOLBOEK\x12.\n\x0bKGGHLADEKGP\x18\x0f\x20\
    \x01(\x0e2\x0c.OJGDBABMNMHR\x0bKGGHLADEKGP\x12\x20\n\x0bGFKIHHOPJDG\x18\
    \x0e\x20\x01(\rR\x0bGFKIHHOPJDG\x12\x20\n\x0bHDAIJPHEEJO\x18\x0b\x20\x03\
    (\rR\x0bHDAIJPHEEJO\x12+\n\x0bELPMNKHEPKJ\x18\n\x20\x01(\x0b2\t.ItemList\
    R\x0bELPMNKHEPKJ\x12\x20\n\x0bNOKLENCDFNF\x18\x02\x20\x03(\rR\x0bNOKLENC\
    DFNF\x12\x20\n\x0bKOBMCLGJJDB\x18\t\x20\x01(\rR\x0bKOBMCLGJJDBb\x06proto\
    3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::OJGDBABMNMH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SwordTrainingGameSettleScNotify::generated_message_descriptor_data());
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
