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

//! Generated file from `RaidInfoNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RaidInfoNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RaidInfoNotify {
    // message fields
    // @@protoc_insertion_point(field:RaidInfoNotify.ODOAJJGMBCL)
    pub ODOAJJGMBCL: u32,
    // @@protoc_insertion_point(field:RaidInfoNotify.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::HIAOLBAKJAP::HIAOLBAKJAP>,
    // @@protoc_insertion_point(field:RaidInfoNotify.KHPMKNEHICM)
    pub KHPMKNEHICM: u64,
    // @@protoc_insertion_point(field:RaidInfoNotify.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:RaidInfoNotify.AGPKHOOCMPE)
    pub AGPKHOOCMPE: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:RaidInfoNotify.OABBJAHIPLF)
    pub OABBJAHIPLF: ::std::vec::Vec<super::GDBPCLPHBOL::GDBPCLPHBOL>,
    // special fields
    // @@protoc_insertion_point(special_field:RaidInfoNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RaidInfoNotify {
    fn default() -> &'a RaidInfoNotify {
        <RaidInfoNotify as ::protobuf::Message>::default_instance()
    }
}

impl RaidInfoNotify {
    pub fn new() -> RaidInfoNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODOAJJGMBCL",
            |m: &RaidInfoNotify| { &m.ODOAJJGMBCL },
            |m: &mut RaidInfoNotify| { &mut m.ODOAJJGMBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &RaidInfoNotify| { &m.OJBAILGKLBM },
            |m: &mut RaidInfoNotify| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KHPMKNEHICM",
            |m: &RaidInfoNotify| { &m.KHPMKNEHICM },
            |m: &mut RaidInfoNotify| { &mut m.KHPMKNEHICM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &RaidInfoNotify| { &m.DNMJBNNJLEL },
            |m: &mut RaidInfoNotify| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "AGPKHOOCMPE",
            |m: &RaidInfoNotify| { &m.AGPKHOOCMPE },
            |m: &mut RaidInfoNotify| { &mut m.AGPKHOOCMPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OABBJAHIPLF",
            |m: &RaidInfoNotify| { &m.OABBJAHIPLF },
            |m: &mut RaidInfoNotify| { &mut m.OABBJAHIPLF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RaidInfoNotify>(
            "RaidInfoNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RaidInfoNotify {
    const NAME: &'static str = "RaidInfoNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.ODOAJJGMBCL = is.read_uint32()?;
                },
                16 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.KHPMKNEHICM = is.read_uint64()?;
                },
                72 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AGPKHOOCMPE)?;
                },
                26 => {
                    self.OABBJAHIPLF.push(is.read_message()?);
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
        if self.ODOAJJGMBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ODOAJJGMBCL);
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::HIAOLBAKJAP::HIAOLBAKJAP::RAID_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.OJBAILGKLBM.value());
        }
        if self.KHPMKNEHICM != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.KHPMKNEHICM);
        }
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.DNMJBNNJLEL);
        }
        if let Some(v) = self.AGPKHOOCMPE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.OABBJAHIPLF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ODOAJJGMBCL != 0 {
            os.write_uint32(8, self.ODOAJJGMBCL)?;
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::HIAOLBAKJAP::HIAOLBAKJAP::RAID_STATUS_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
        }
        if self.KHPMKNEHICM != 0 {
            os.write_uint64(6, self.KHPMKNEHICM)?;
        }
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(9, self.DNMJBNNJLEL)?;
        }
        if let Some(v) = self.AGPKHOOCMPE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        for v in &self.OABBJAHIPLF {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> RaidInfoNotify {
        RaidInfoNotify::new()
    }

    fn clear(&mut self) {
        self.ODOAJJGMBCL = 0;
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::HIAOLBAKJAP::HIAOLBAKJAP::RAID_STATUS_NONE);
        self.KHPMKNEHICM = 0;
        self.DNMJBNNJLEL = 0;
        self.AGPKHOOCMPE.clear();
        self.OABBJAHIPLF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RaidInfoNotify {
        static instance: RaidInfoNotify = RaidInfoNotify {
            ODOAJJGMBCL: 0,
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            KHPMKNEHICM: 0,
            DNMJBNNJLEL: 0,
            AGPKHOOCMPE: ::protobuf::MessageField::none(),
            OABBJAHIPLF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RaidInfoNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RaidInfoNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RaidInfoNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaidInfoNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14RaidInfoNotify.proto\x1a\x11GDBPCLPHBOL.proto\x1a\x11HIAOLBAKJAP.p\
    roto\x1a\x0eItemList.proto\"\x83\x02\n\x0eRaidInfoNotify\x12\x20\n\x0bOD\
    OAJJGMBCL\x18\x08\x20\x01(\rR\x0bODOAJJGMBCL\x12.\n\x0bOJBAILGKLBM\x18\
    \x02\x20\x01(\x0e2\x0c.HIAOLBAKJAPR\x0bOJBAILGKLBM\x12\x20\n\x0bKHPMKNEH\
    ICM\x18\x06\x20\x01(\x04R\x0bKHPMKNEHICM\x12\x20\n\x0bDNMJBNNJLEL\x18\t\
    \x20\x01(\rR\x0bDNMJBNNJLEL\x12+\n\x0bAGPKHOOCMPE\x18\r\x20\x01(\x0b2\t.\
    ItemListR\x0bAGPKHOOCMPE\x12.\n\x0bOABBJAHIPLF\x18\x03\x20\x03(\x0b2\x0c\
    .GDBPCLPHBOLR\x0bOABBJAHIPLFb\x06proto3\
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
            deps.push(super::GDBPCLPHBOL::file_descriptor().clone());
            deps.push(super::HIAOLBAKJAP::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RaidInfoNotify::generated_message_descriptor_data());
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
