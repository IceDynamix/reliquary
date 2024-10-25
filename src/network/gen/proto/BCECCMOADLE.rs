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

//! Generated file from `BCECCMOADLE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BCECCMOADLE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BCECCMOADLE {
    // message fields
    // @@protoc_insertion_point(field:BCECCMOADLE.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:BCECCMOADLE.level)
    pub level: u32,
    // @@protoc_insertion_point(field:BCECCMOADLE.DBPCMPCLOCI)
    pub DBPCMPCLOCI: ::std::string::String,
    // @@protoc_insertion_point(field:BCECCMOADLE.LMNDOGAFOGC)
    pub LMNDOGAFOGC: u32,
    // @@protoc_insertion_point(field:BCECCMOADLE.NEJJEFEOJOJ)
    pub NEJJEFEOJOJ: ::protobuf::EnumOrUnknown<super::BPOEMOLJCCE::BPOEMOLJCCE>,
    // @@protoc_insertion_point(field:BCECCMOADLE.FLFKCIHCENF)
    pub FLFKCIHCENF: ::std::string::String,
    // @@protoc_insertion_point(field:BCECCMOADLE.KDCLOCAPBGE)
    pub KDCLOCAPBGE: ::std::string::String,
    // @@protoc_insertion_point(field:BCECCMOADLE.DDALCGLIHDE)
    pub DDALCGLIHDE: u64,
    // special fields
    // @@protoc_insertion_point(special_field:BCECCMOADLE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BCECCMOADLE {
    fn default() -> &'a BCECCMOADLE {
        <BCECCMOADLE as ::protobuf::Message>::default_instance()
    }
}

impl BCECCMOADLE {
    pub fn new() -> BCECCMOADLE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &BCECCMOADLE| { &m.uid },
            |m: &mut BCECCMOADLE| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &BCECCMOADLE| { &m.level },
            |m: &mut BCECCMOADLE| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBPCMPCLOCI",
            |m: &BCECCMOADLE| { &m.DBPCMPCLOCI },
            |m: &mut BCECCMOADLE| { &mut m.DBPCMPCLOCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMNDOGAFOGC",
            |m: &BCECCMOADLE| { &m.LMNDOGAFOGC },
            |m: &mut BCECCMOADLE| { &mut m.LMNDOGAFOGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEJJEFEOJOJ",
            |m: &BCECCMOADLE| { &m.NEJJEFEOJOJ },
            |m: &mut BCECCMOADLE| { &mut m.NEJJEFEOJOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FLFKCIHCENF",
            |m: &BCECCMOADLE| { &m.FLFKCIHCENF },
            |m: &mut BCECCMOADLE| { &mut m.FLFKCIHCENF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDCLOCAPBGE",
            |m: &BCECCMOADLE| { &m.KDCLOCAPBGE },
            |m: &mut BCECCMOADLE| { &mut m.KDCLOCAPBGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDALCGLIHDE",
            |m: &BCECCMOADLE| { &m.DDALCGLIHDE },
            |m: &mut BCECCMOADLE| { &mut m.DDALCGLIHDE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BCECCMOADLE>(
            "BCECCMOADLE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BCECCMOADLE {
    const NAME: &'static str = "BCECCMOADLE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.uid = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                26 => {
                    self.DBPCMPCLOCI = is.read_string()?;
                },
                32 => {
                    self.LMNDOGAFOGC = is.read_uint32()?;
                },
                40 => {
                    self.NEJJEFEOJOJ = is.read_enum_or_unknown()?;
                },
                50 => {
                    self.FLFKCIHCENF = is.read_string()?;
                },
                58 => {
                    self.KDCLOCAPBGE = is.read_string()?;
                },
                64 => {
                    self.DDALCGLIHDE = is.read_uint64()?;
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
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.uid);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if !self.DBPCMPCLOCI.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.DBPCMPCLOCI);
        }
        if self.LMNDOGAFOGC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LMNDOGAFOGC);
        }
        if self.NEJJEFEOJOJ != ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR) {
            my_size += ::protobuf::rt::int32_size(5, self.NEJJEFEOJOJ.value());
        }
        if !self.FLFKCIHCENF.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.FLFKCIHCENF);
        }
        if !self.KDCLOCAPBGE.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.KDCLOCAPBGE);
        }
        if self.DDALCGLIHDE != 0 {
            my_size += ::protobuf::rt::uint64_size(8, self.DDALCGLIHDE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.uid != 0 {
            os.write_uint32(1, self.uid)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if !self.DBPCMPCLOCI.is_empty() {
            os.write_string(3, &self.DBPCMPCLOCI)?;
        }
        if self.LMNDOGAFOGC != 0 {
            os.write_uint32(4, self.LMNDOGAFOGC)?;
        }
        if self.NEJJEFEOJOJ != ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.NEJJEFEOJOJ))?;
        }
        if !self.FLFKCIHCENF.is_empty() {
            os.write_string(6, &self.FLFKCIHCENF)?;
        }
        if !self.KDCLOCAPBGE.is_empty() {
            os.write_string(7, &self.KDCLOCAPBGE)?;
        }
        if self.DDALCGLIHDE != 0 {
            os.write_uint64(8, self.DDALCGLIHDE)?;
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

    fn new() -> BCECCMOADLE {
        BCECCMOADLE::new()
    }

    fn clear(&mut self) {
        self.uid = 0;
        self.level = 0;
        self.DBPCMPCLOCI.clear();
        self.LMNDOGAFOGC = 0;
        self.NEJJEFEOJOJ = ::protobuf::EnumOrUnknown::new(super::BPOEMOLJCCE::BPOEMOLJCCE::EDITOR);
        self.FLFKCIHCENF.clear();
        self.KDCLOCAPBGE.clear();
        self.DDALCGLIHDE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BCECCMOADLE {
        static instance: BCECCMOADLE = BCECCMOADLE {
            uid: 0,
            level: 0,
            DBPCMPCLOCI: ::std::string::String::new(),
            LMNDOGAFOGC: 0,
            NEJJEFEOJOJ: ::protobuf::EnumOrUnknown::from_i32(0),
            FLFKCIHCENF: ::std::string::String::new(),
            KDCLOCAPBGE: ::std::string::String::new(),
            DDALCGLIHDE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BCECCMOADLE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BCECCMOADLE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BCECCMOADLE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BCECCMOADLE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BCECCMOADLE.proto\x1a\x11BPOEMOLJCCE.proto\"\x8f\x02\n\x0bBCECCMOA\
    DLE\x12\x10\n\x03uid\x18\x01\x20\x01(\rR\x03uid\x12\x14\n\x05level\x18\
    \x02\x20\x01(\rR\x05level\x12\x20\n\x0bDBPCMPCLOCI\x18\x03\x20\x01(\tR\
    \x0bDBPCMPCLOCI\x12\x20\n\x0bLMNDOGAFOGC\x18\x04\x20\x01(\rR\x0bLMNDOGAF\
    OGC\x12.\n\x0bNEJJEFEOJOJ\x18\x05\x20\x01(\x0e2\x0c.BPOEMOLJCCER\x0bNEJJ\
    EFEOJOJ\x12\x20\n\x0bFLFKCIHCENF\x18\x06\x20\x01(\tR\x0bFLFKCIHCENF\x12\
    \x20\n\x0bKDCLOCAPBGE\x18\x07\x20\x01(\tR\x0bKDCLOCAPBGE\x12\x20\n\x0bDD\
    ALCGLIHDE\x18\x08\x20\x01(\x04R\x0bDDALCGLIHDEb\x06proto3\
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
            deps.push(super::BPOEMOLJCCE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BCECCMOADLE::generated_message_descriptor_data());
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
