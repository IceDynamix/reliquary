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
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.CCCEDBIGCDG)
    pub CCCEDBIGCDG: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.OLDKAMACFMD)
    pub OLDKAMACFMD: ::protobuf::EnumOrUnknown<super::BOEHDKFADAH::BOEHDKFADAH>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.EIFBKOJDEDH)
    pub EIFBKOJDEDH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.MKMEAENLAME)
    pub MKMEAENLAME: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.AECJKNIGHJK)
    pub AECJKNIGHJK: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.LEKNEPCGOOG)
    pub LEKNEPCGOOG: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.DKHGCHAHOMK)
    pub DKHGCHAHOMK: u32,
    // @@protoc_insertion_point(field:SwordTrainingGameSettleScNotify.FBAJGAIOIFE)
    pub FBAJGAIOIFE: ::std::vec::Vec<u32>,
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
            "CCCEDBIGCDG",
            |m: &SwordTrainingGameSettleScNotify| { &m.CCCEDBIGCDG },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.CCCEDBIGCDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLDKAMACFMD",
            |m: &SwordTrainingGameSettleScNotify| { &m.OLDKAMACFMD },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.OLDKAMACFMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EIFBKOJDEDH",
            |m: &SwordTrainingGameSettleScNotify| { &m.EIFBKOJDEDH },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.EIFBKOJDEDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKMEAENLAME",
            |m: &SwordTrainingGameSettleScNotify| { &m.MKMEAENLAME },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.MKMEAENLAME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "AECJKNIGHJK",
            |m: &SwordTrainingGameSettleScNotify| { &m.AECJKNIGHJK },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.AECJKNIGHJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEKNEPCGOOG",
            |m: &SwordTrainingGameSettleScNotify| { &m.LEKNEPCGOOG },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.LEKNEPCGOOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKHGCHAHOMK",
            |m: &SwordTrainingGameSettleScNotify| { &m.DKHGCHAHOMK },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.DKHGCHAHOMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FBAJGAIOIFE",
            |m: &SwordTrainingGameSettleScNotify| { &m.FBAJGAIOIFE },
            |m: &mut SwordTrainingGameSettleScNotify| { &mut m.FBAJGAIOIFE },
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
                112 => {
                    self.CCCEDBIGCDG = is.read_uint32()?;
                },
                80 => {
                    self.OLDKAMACFMD = is.read_enum_or_unknown()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.EIFBKOJDEDH)?;
                },
                8 => {
                    self.EIFBKOJDEDH.push(is.read_uint32()?);
                },
                72 => {
                    self.MKMEAENLAME = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AECJKNIGHJK)?;
                },
                16 => {
                    self.LEKNEPCGOOG = is.read_uint32()?;
                },
                64 => {
                    self.DKHGCHAHOMK = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.FBAJGAIOIFE)?;
                },
                24 => {
                    self.FBAJGAIOIFE.push(is.read_uint32()?);
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
        if self.CCCEDBIGCDG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.CCCEDBIGCDG);
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::BOEHDKFADAH::BOEHDKFADAH::SWORD_TRAINING_GAME_SETTLE_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.OLDKAMACFMD.value());
        }
        for value in &self.EIFBKOJDEDH {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.MKMEAENLAME != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.MKMEAENLAME);
        }
        if let Some(v) = self.AECJKNIGHJK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LEKNEPCGOOG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LEKNEPCGOOG);
        }
        if self.DKHGCHAHOMK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DKHGCHAHOMK);
        }
        for value in &self.FBAJGAIOIFE {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CCCEDBIGCDG != 0 {
            os.write_uint32(14, self.CCCEDBIGCDG)?;
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::BOEHDKFADAH::BOEHDKFADAH::SWORD_TRAINING_GAME_SETTLE_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.OLDKAMACFMD))?;
        }
        for v in &self.EIFBKOJDEDH {
            os.write_uint32(1, *v)?;
        };
        if self.MKMEAENLAME != 0 {
            os.write_uint32(9, self.MKMEAENLAME)?;
        }
        if let Some(v) = self.AECJKNIGHJK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.LEKNEPCGOOG != 0 {
            os.write_uint32(2, self.LEKNEPCGOOG)?;
        }
        if self.DKHGCHAHOMK != 0 {
            os.write_uint32(8, self.DKHGCHAHOMK)?;
        }
        for v in &self.FBAJGAIOIFE {
            os.write_uint32(3, *v)?;
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

    fn new() -> SwordTrainingGameSettleScNotify {
        SwordTrainingGameSettleScNotify::new()
    }

    fn clear(&mut self) {
        self.CCCEDBIGCDG = 0;
        self.OLDKAMACFMD = ::protobuf::EnumOrUnknown::new(super::BOEHDKFADAH::BOEHDKFADAH::SWORD_TRAINING_GAME_SETTLE_NONE);
        self.EIFBKOJDEDH.clear();
        self.MKMEAENLAME = 0;
        self.AECJKNIGHJK.clear();
        self.LEKNEPCGOOG = 0;
        self.DKHGCHAHOMK = 0;
        self.FBAJGAIOIFE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SwordTrainingGameSettleScNotify {
        static instance: SwordTrainingGameSettleScNotify = SwordTrainingGameSettleScNotify {
            CCCEDBIGCDG: 0,
            OLDKAMACFMD: ::protobuf::EnumOrUnknown::from_i32(0),
            EIFBKOJDEDH: ::std::vec::Vec::new(),
            MKMEAENLAME: 0,
            AECJKNIGHJK: ::protobuf::MessageField::none(),
            LEKNEPCGOOG: 0,
            DKHGCHAHOMK: 0,
            FBAJGAIOIFE: ::std::vec::Vec::new(),
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
    \n%SwordTrainingGameSettleScNotify.proto\x1a\x11BOEHDKFADAH.proto\x1a\
    \x0eItemList.proto\"\xca\x02\n\x1fSwordTrainingGameSettleScNotify\x12\
    \x20\n\x0bCCCEDBIGCDG\x18\x0e\x20\x01(\rR\x0bCCCEDBIGCDG\x12.\n\x0bOLDKA\
    MACFMD\x18\n\x20\x01(\x0e2\x0c.BOEHDKFADAHR\x0bOLDKAMACFMD\x12\x20\n\x0b\
    EIFBKOJDEDH\x18\x01\x20\x03(\rR\x0bEIFBKOJDEDH\x12\x20\n\x0bMKMEAENLAME\
    \x18\t\x20\x01(\rR\x0bMKMEAENLAME\x12+\n\x0bAECJKNIGHJK\x18\x04\x20\x01(\
    \x0b2\t.ItemListR\x0bAECJKNIGHJK\x12\x20\n\x0bLEKNEPCGOOG\x18\x02\x20\
    \x01(\rR\x0bLEKNEPCGOOG\x12\x20\n\x0bDKHGCHAHOMK\x18\x08\x20\x01(\rR\x0b\
    DKHGCHAHOMK\x12\x20\n\x0bFBAJGAIOIFE\x18\x03\x20\x03(\rR\x0bFBAJGAIOIFEb\
    \x06proto3\
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
            deps.push(super::BOEHDKFADAH::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
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
