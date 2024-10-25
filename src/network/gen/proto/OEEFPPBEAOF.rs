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

//! Generated file from `OEEFPPBEAOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OEEFPPBEAOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OEEFPPBEAOF {
    // message fields
    // @@protoc_insertion_point(field:OEEFPPBEAOF.OKLINBBACBJ)
    pub OKLINBBACBJ: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.AIAFDNMOCNP)
    pub AIAFDNMOCNP: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.DEOGEMOONHO)
    pub DEOGEMOONHO: bool,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.DMECAAFMHLN)
    pub DMECAAFMHLN: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.AJOGDNJBJOH)
    pub AJOGDNJBJOH: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.OAMDIMJJFFB)
    pub OAMDIMJJFFB: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.GPACHKLFGPC)
    pub GPACHKLFGPC: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.FJHEAFKHFGH)
    pub FJHEAFKHFGH: u32,
    // @@protoc_insertion_point(field:OEEFPPBEAOF.FBABHLPMKBK)
    pub FBABHLPMKBK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OEEFPPBEAOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OEEFPPBEAOF {
    fn default() -> &'a OEEFPPBEAOF {
        <OEEFPPBEAOF as ::protobuf::Message>::default_instance()
    }
}

impl OEEFPPBEAOF {
    pub fn new() -> OEEFPPBEAOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OKLINBBACBJ",
            |m: &OEEFPPBEAOF| { &m.OKLINBBACBJ },
            |m: &mut OEEFPPBEAOF| { &mut m.OKLINBBACBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AIAFDNMOCNP",
            |m: &OEEFPPBEAOF| { &m.AIAFDNMOCNP },
            |m: &mut OEEFPPBEAOF| { &mut m.AIAFDNMOCNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEOGEMOONHO",
            |m: &OEEFPPBEAOF| { &m.DEOGEMOONHO },
            |m: &mut OEEFPPBEAOF| { &mut m.DEOGEMOONHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMECAAFMHLN",
            |m: &OEEFPPBEAOF| { &m.DMECAAFMHLN },
            |m: &mut OEEFPPBEAOF| { &mut m.DMECAAFMHLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJOGDNJBJOH",
            |m: &OEEFPPBEAOF| { &m.AJOGDNJBJOH },
            |m: &mut OEEFPPBEAOF| { &mut m.AJOGDNJBJOH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OAMDIMJJFFB",
            |m: &OEEFPPBEAOF| { &m.OAMDIMJJFFB },
            |m: &mut OEEFPPBEAOF| { &mut m.OAMDIMJJFFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GPACHKLFGPC",
            |m: &OEEFPPBEAOF| { &m.GPACHKLFGPC },
            |m: &mut OEEFPPBEAOF| { &mut m.GPACHKLFGPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJHEAFKHFGH",
            |m: &OEEFPPBEAOF| { &m.FJHEAFKHFGH },
            |m: &mut OEEFPPBEAOF| { &mut m.FJHEAFKHFGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBABHLPMKBK",
            |m: &OEEFPPBEAOF| { &m.FBABHLPMKBK },
            |m: &mut OEEFPPBEAOF| { &mut m.FBABHLPMKBK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OEEFPPBEAOF>(
            "OEEFPPBEAOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OEEFPPBEAOF {
    const NAME: &'static str = "OEEFPPBEAOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.OKLINBBACBJ = is.read_uint32()?;
                },
                88 => {
                    self.AIAFDNMOCNP = is.read_uint32()?;
                },
                24 => {
                    self.DEOGEMOONHO = is.read_bool()?;
                },
                40 => {
                    self.DMECAAFMHLN = is.read_uint32()?;
                },
                80 => {
                    self.AJOGDNJBJOH = is.read_uint32()?;
                },
                96 => {
                    self.OAMDIMJJFFB = is.read_uint32()?;
                },
                48 => {
                    self.GPACHKLFGPC = is.read_uint32()?;
                },
                8 => {
                    self.FJHEAFKHFGH = is.read_uint32()?;
                },
                32 => {
                    self.FBABHLPMKBK = is.read_uint32()?;
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
        if self.OKLINBBACBJ != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OKLINBBACBJ);
        }
        if self.AIAFDNMOCNP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.AIAFDNMOCNP);
        }
        if self.DEOGEMOONHO != false {
            my_size += 1 + 1;
        }
        if self.DMECAAFMHLN != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DMECAAFMHLN);
        }
        if self.AJOGDNJBJOH != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.AJOGDNJBJOH);
        }
        if self.OAMDIMJJFFB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.OAMDIMJJFFB);
        }
        if self.GPACHKLFGPC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GPACHKLFGPC);
        }
        if self.FJHEAFKHFGH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FJHEAFKHFGH);
        }
        if self.FBABHLPMKBK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FBABHLPMKBK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OKLINBBACBJ != 0 {
            os.write_uint32(13, self.OKLINBBACBJ)?;
        }
        if self.AIAFDNMOCNP != 0 {
            os.write_uint32(11, self.AIAFDNMOCNP)?;
        }
        if self.DEOGEMOONHO != false {
            os.write_bool(3, self.DEOGEMOONHO)?;
        }
        if self.DMECAAFMHLN != 0 {
            os.write_uint32(5, self.DMECAAFMHLN)?;
        }
        if self.AJOGDNJBJOH != 0 {
            os.write_uint32(10, self.AJOGDNJBJOH)?;
        }
        if self.OAMDIMJJFFB != 0 {
            os.write_uint32(12, self.OAMDIMJJFFB)?;
        }
        if self.GPACHKLFGPC != 0 {
            os.write_uint32(6, self.GPACHKLFGPC)?;
        }
        if self.FJHEAFKHFGH != 0 {
            os.write_uint32(1, self.FJHEAFKHFGH)?;
        }
        if self.FBABHLPMKBK != 0 {
            os.write_uint32(4, self.FBABHLPMKBK)?;
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

    fn new() -> OEEFPPBEAOF {
        OEEFPPBEAOF::new()
    }

    fn clear(&mut self) {
        self.OKLINBBACBJ = 0;
        self.AIAFDNMOCNP = 0;
        self.DEOGEMOONHO = false;
        self.DMECAAFMHLN = 0;
        self.AJOGDNJBJOH = 0;
        self.OAMDIMJJFFB = 0;
        self.GPACHKLFGPC = 0;
        self.FJHEAFKHFGH = 0;
        self.FBABHLPMKBK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OEEFPPBEAOF {
        static instance: OEEFPPBEAOF = OEEFPPBEAOF {
            OKLINBBACBJ: 0,
            AIAFDNMOCNP: 0,
            DEOGEMOONHO: false,
            DMECAAFMHLN: 0,
            AJOGDNJBJOH: 0,
            OAMDIMJJFFB: 0,
            GPACHKLFGPC: 0,
            FJHEAFKHFGH: 0,
            FBABHLPMKBK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OEEFPPBEAOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OEEFPPBEAOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OEEFPPBEAOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OEEFPPBEAOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OEEFPPBEAOF.proto\"\xbf\x02\n\x0bOEEFPPBEAOF\x12\x20\n\x0bOKLINBBA\
    CBJ\x18\r\x20\x01(\rR\x0bOKLINBBACBJ\x12\x20\n\x0bAIAFDNMOCNP\x18\x0b\
    \x20\x01(\rR\x0bAIAFDNMOCNP\x12\x20\n\x0bDEOGEMOONHO\x18\x03\x20\x01(\
    \x08R\x0bDEOGEMOONHO\x12\x20\n\x0bDMECAAFMHLN\x18\x05\x20\x01(\rR\x0bDME\
    CAAFMHLN\x12\x20\n\x0bAJOGDNJBJOH\x18\n\x20\x01(\rR\x0bAJOGDNJBJOH\x12\
    \x20\n\x0bOAMDIMJJFFB\x18\x0c\x20\x01(\rR\x0bOAMDIMJJFFB\x12\x20\n\x0bGP\
    ACHKLFGPC\x18\x06\x20\x01(\rR\x0bGPACHKLFGPC\x12\x20\n\x0bFJHEAFKHFGH\
    \x18\x01\x20\x01(\rR\x0bFJHEAFKHFGH\x12\x20\n\x0bFBABHLPMKBK\x18\x04\x20\
    \x01(\rR\x0bFBABHLPMKBKb\x06proto3\
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
            messages.push(OEEFPPBEAOF::generated_message_descriptor_data());
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
