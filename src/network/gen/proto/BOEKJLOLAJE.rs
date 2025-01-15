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

//! Generated file from `BOEKJLOLAJE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BOEKJLOLAJE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BOEKJLOLAJE {
    // message fields
    // @@protoc_insertion_point(field:BOEKJLOLAJE.KPOOIMOBAEO)
    pub KPOOIMOBAEO: ::std::vec::Vec<super::OOKFHBFLPEB::OOKFHBFLPEB>,
    // @@protoc_insertion_point(field:BOEKJLOLAJE.MMNMMNIADEP)
    pub MMNMMNIADEP: u32,
    // @@protoc_insertion_point(field:BOEKJLOLAJE.LCLLIKPFFOP)
    pub LCLLIKPFFOP: ::std::vec::Vec<super::DBKPLONPCHJ::DBKPLONPCHJ>,
    // @@protoc_insertion_point(field:BOEKJLOLAJE.AKCEMFLNKOL)
    pub AKCEMFLNKOL: bool,
    // special fields
    // @@protoc_insertion_point(special_field:BOEKJLOLAJE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BOEKJLOLAJE {
    fn default() -> &'a BOEKJLOLAJE {
        <BOEKJLOLAJE as ::protobuf::Message>::default_instance()
    }
}

impl BOEKJLOLAJE {
    pub fn new() -> BOEKJLOLAJE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KPOOIMOBAEO",
            |m: &BOEKJLOLAJE| { &m.KPOOIMOBAEO },
            |m: &mut BOEKJLOLAJE| { &mut m.KPOOIMOBAEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMNMMNIADEP",
            |m: &BOEKJLOLAJE| { &m.MMNMMNIADEP },
            |m: &mut BOEKJLOLAJE| { &mut m.MMNMMNIADEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LCLLIKPFFOP",
            |m: &BOEKJLOLAJE| { &m.LCLLIKPFFOP },
            |m: &mut BOEKJLOLAJE| { &mut m.LCLLIKPFFOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCEMFLNKOL",
            |m: &BOEKJLOLAJE| { &m.AKCEMFLNKOL },
            |m: &mut BOEKJLOLAJE| { &mut m.AKCEMFLNKOL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BOEKJLOLAJE>(
            "BOEKJLOLAJE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BOEKJLOLAJE {
    const NAME: &'static str = "BOEKJLOLAJE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.KPOOIMOBAEO.push(is.read_message()?);
                },
                56 => {
                    self.MMNMMNIADEP = is.read_uint32()?;
                },
                10 => {
                    self.LCLLIKPFFOP.push(is.read_message()?);
                },
                40 => {
                    self.AKCEMFLNKOL = is.read_bool()?;
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
        for value in &self.KPOOIMOBAEO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MMNMMNIADEP != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.MMNMMNIADEP);
        }
        for value in &self.LCLLIKPFFOP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.AKCEMFLNKOL != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KPOOIMOBAEO {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.MMNMMNIADEP != 0 {
            os.write_uint32(7, self.MMNMMNIADEP)?;
        }
        for v in &self.LCLLIKPFFOP {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.AKCEMFLNKOL != false {
            os.write_bool(5, self.AKCEMFLNKOL)?;
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

    fn new() -> BOEKJLOLAJE {
        BOEKJLOLAJE::new()
    }

    fn clear(&mut self) {
        self.KPOOIMOBAEO.clear();
        self.MMNMMNIADEP = 0;
        self.LCLLIKPFFOP.clear();
        self.AKCEMFLNKOL = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BOEKJLOLAJE {
        static instance: BOEKJLOLAJE = BOEKJLOLAJE {
            KPOOIMOBAEO: ::std::vec::Vec::new(),
            MMNMMNIADEP: 0,
            LCLLIKPFFOP: ::std::vec::Vec::new(),
            AKCEMFLNKOL: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BOEKJLOLAJE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BOEKJLOLAJE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BOEKJLOLAJE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BOEKJLOLAJE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BOEKJLOLAJE.proto\x1a\x11DBKPLONPCHJ.proto\x1a\x11OOKFHBFLPEB.prot\
    o\"\xb1\x01\n\x0bBOEKJLOLAJE\x12.\n\x0bKPOOIMOBAEO\x18\x08\x20\x03(\x0b2\
    \x0c.OOKFHBFLPEBR\x0bKPOOIMOBAEO\x12\x20\n\x0bMMNMMNIADEP\x18\x07\x20\
    \x01(\rR\x0bMMNMMNIADEP\x12.\n\x0bLCLLIKPFFOP\x18\x01\x20\x03(\x0b2\x0c.\
    DBKPLONPCHJR\x0bLCLLIKPFFOP\x12\x20\n\x0bAKCEMFLNKOL\x18\x05\x20\x01(\
    \x08R\x0bAKCEMFLNKOLb\x06proto3\
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
            deps.push(super::DBKPLONPCHJ::file_descriptor().clone());
            deps.push(super::OOKFHBFLPEB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BOEKJLOLAJE::generated_message_descriptor_data());
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