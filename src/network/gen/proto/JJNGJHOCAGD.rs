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

//! Generated file from `JJNGJHOCAGD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JJNGJHOCAGD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JJNGJHOCAGD {
    // message fields
    // @@protoc_insertion_point(field:JJNGJHOCAGD.FBABHLPMKBK)
    pub FBABHLPMKBK: u32,
    // @@protoc_insertion_point(field:JJNGJHOCAGD.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:JJNGJHOCAGD.IMIJBGMIOMF)
    pub IMIJBGMIOMF: u32,
    // @@protoc_insertion_point(field:JJNGJHOCAGD.EDKBMNFOOAJ)
    pub EDKBMNFOOAJ: u32,
    // @@protoc_insertion_point(field:JJNGJHOCAGD.GPACHKLFGPC)
    pub GPACHKLFGPC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JJNGJHOCAGD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JJNGJHOCAGD {
    fn default() -> &'a JJNGJHOCAGD {
        <JJNGJHOCAGD as ::protobuf::Message>::default_instance()
    }
}

impl JJNGJHOCAGD {
    pub fn new() -> JJNGJHOCAGD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBABHLPMKBK",
            |m: &JJNGJHOCAGD| { &m.FBABHLPMKBK },
            |m: &mut JJNGJHOCAGD| { &mut m.FBABHLPMKBK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &JJNGJHOCAGD| { &m.uid },
            |m: &mut JJNGJHOCAGD| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IMIJBGMIOMF",
            |m: &JJNGJHOCAGD| { &m.IMIJBGMIOMF },
            |m: &mut JJNGJHOCAGD| { &mut m.IMIJBGMIOMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EDKBMNFOOAJ",
            |m: &JJNGJHOCAGD| { &m.EDKBMNFOOAJ },
            |m: &mut JJNGJHOCAGD| { &mut m.EDKBMNFOOAJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GPACHKLFGPC",
            |m: &JJNGJHOCAGD| { &m.GPACHKLFGPC },
            |m: &mut JJNGJHOCAGD| { &mut m.GPACHKLFGPC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JJNGJHOCAGD>(
            "JJNGJHOCAGD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JJNGJHOCAGD {
    const NAME: &'static str = "JJNGJHOCAGD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.FBABHLPMKBK = is.read_uint32()?;
                },
                104 => {
                    self.uid = is.read_uint32()?;
                },
                88 => {
                    self.IMIJBGMIOMF = is.read_uint32()?;
                },
                48 => {
                    self.EDKBMNFOOAJ = is.read_uint32()?;
                },
                24 => {
                    self.GPACHKLFGPC = is.read_uint32()?;
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
        if self.FBABHLPMKBK != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FBABHLPMKBK);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.uid);
        }
        if self.IMIJBGMIOMF != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.IMIJBGMIOMF);
        }
        if self.EDKBMNFOOAJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.EDKBMNFOOAJ);
        }
        if self.GPACHKLFGPC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.GPACHKLFGPC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FBABHLPMKBK != 0 {
            os.write_uint32(1, self.FBABHLPMKBK)?;
        }
        if self.uid != 0 {
            os.write_uint32(13, self.uid)?;
        }
        if self.IMIJBGMIOMF != 0 {
            os.write_uint32(11, self.IMIJBGMIOMF)?;
        }
        if self.EDKBMNFOOAJ != 0 {
            os.write_uint32(6, self.EDKBMNFOOAJ)?;
        }
        if self.GPACHKLFGPC != 0 {
            os.write_uint32(3, self.GPACHKLFGPC)?;
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

    fn new() -> JJNGJHOCAGD {
        JJNGJHOCAGD::new()
    }

    fn clear(&mut self) {
        self.FBABHLPMKBK = 0;
        self.uid = 0;
        self.IMIJBGMIOMF = 0;
        self.EDKBMNFOOAJ = 0;
        self.GPACHKLFGPC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JJNGJHOCAGD {
        static instance: JJNGJHOCAGD = JJNGJHOCAGD {
            FBABHLPMKBK: 0,
            uid: 0,
            IMIJBGMIOMF: 0,
            EDKBMNFOOAJ: 0,
            GPACHKLFGPC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JJNGJHOCAGD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JJNGJHOCAGD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JJNGJHOCAGD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JJNGJHOCAGD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JJNGJHOCAGD.proto\"\xa7\x01\n\x0bJJNGJHOCAGD\x12\x20\n\x0bFBABHLPM\
    KBK\x18\x01\x20\x01(\rR\x0bFBABHLPMKBK\x12\x10\n\x03uid\x18\r\x20\x01(\r\
    R\x03uid\x12\x20\n\x0bIMIJBGMIOMF\x18\x0b\x20\x01(\rR\x0bIMIJBGMIOMF\x12\
    \x20\n\x0bEDKBMNFOOAJ\x18\x06\x20\x01(\rR\x0bEDKBMNFOOAJ\x12\x20\n\x0bGP\
    ACHKLFGPC\x18\x03\x20\x01(\rR\x0bGPACHKLFGPCb\x06proto3\
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
            messages.push(JJNGJHOCAGD::generated_message_descriptor_data());
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
