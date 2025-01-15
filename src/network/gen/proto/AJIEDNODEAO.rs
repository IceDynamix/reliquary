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

//! Generated file from `AJIEDNODEAO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AJIEDNODEAO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AJIEDNODEAO {
    // message fields
    // @@protoc_insertion_point(field:AJIEDNODEAO.HHLBDGMIBNP)
    pub HHLBDGMIBNP: u32,
    // @@protoc_insertion_point(field:AJIEDNODEAO.BGFEKHFMEIH)
    pub BGFEKHFMEIH: bool,
    // @@protoc_insertion_point(field:AJIEDNODEAO.PGGMKGMBBKJ)
    pub PGGMKGMBBKJ: u32,
    // @@protoc_insertion_point(field:AJIEDNODEAO.CPJGLAJDNAC)
    pub CPJGLAJDNAC: ::protobuf::MessageField<super::APKJGAJLLEA::APKJGAJLLEA>,
    // @@protoc_insertion_point(field:AJIEDNODEAO.LJILKDFEMCF)
    pub LJILKDFEMCF: u32,
    // @@protoc_insertion_point(field:AJIEDNODEAO.EMFIDIGHFCC)
    pub EMFIDIGHFCC: u32,
    // @@protoc_insertion_point(field:AJIEDNODEAO.AOPKNFPCPBB)
    pub AOPKNFPCPBB: u32,
    // @@protoc_insertion_point(field:AJIEDNODEAO.DOIMONBNDII)
    pub DOIMONBNDII: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AJIEDNODEAO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AJIEDNODEAO {
    fn default() -> &'a AJIEDNODEAO {
        <AJIEDNODEAO as ::protobuf::Message>::default_instance()
    }
}

impl AJIEDNODEAO {
    pub fn new() -> AJIEDNODEAO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHLBDGMIBNP",
            |m: &AJIEDNODEAO| { &m.HHLBDGMIBNP },
            |m: &mut AJIEDNODEAO| { &mut m.HHLBDGMIBNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGFEKHFMEIH",
            |m: &AJIEDNODEAO| { &m.BGFEKHFMEIH },
            |m: &mut AJIEDNODEAO| { &mut m.BGFEKHFMEIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGGMKGMBBKJ",
            |m: &AJIEDNODEAO| { &m.PGGMKGMBBKJ },
            |m: &mut AJIEDNODEAO| { &mut m.PGGMKGMBBKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::APKJGAJLLEA::APKJGAJLLEA>(
            "CPJGLAJDNAC",
            |m: &AJIEDNODEAO| { &m.CPJGLAJDNAC },
            |m: &mut AJIEDNODEAO| { &mut m.CPJGLAJDNAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJILKDFEMCF",
            |m: &AJIEDNODEAO| { &m.LJILKDFEMCF },
            |m: &mut AJIEDNODEAO| { &mut m.LJILKDFEMCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EMFIDIGHFCC",
            |m: &AJIEDNODEAO| { &m.EMFIDIGHFCC },
            |m: &mut AJIEDNODEAO| { &mut m.EMFIDIGHFCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AOPKNFPCPBB",
            |m: &AJIEDNODEAO| { &m.AOPKNFPCPBB },
            |m: &mut AJIEDNODEAO| { &mut m.AOPKNFPCPBB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DOIMONBNDII",
            |m: &AJIEDNODEAO| { &m.DOIMONBNDII },
            |m: &mut AJIEDNODEAO| { &mut m.DOIMONBNDII },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AJIEDNODEAO>(
            "AJIEDNODEAO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AJIEDNODEAO {
    const NAME: &'static str = "AJIEDNODEAO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.HHLBDGMIBNP = is.read_uint32()?;
                },
                96 => {
                    self.BGFEKHFMEIH = is.read_bool()?;
                },
                24 => {
                    self.PGGMKGMBBKJ = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPJGLAJDNAC)?;
                },
                16 => {
                    self.LJILKDFEMCF = is.read_uint32()?;
                },
                8 => {
                    self.EMFIDIGHFCC = is.read_uint32()?;
                },
                48 => {
                    self.AOPKNFPCPBB = is.read_uint32()?;
                },
                104 => {
                    self.DOIMONBNDII = is.read_uint32()?;
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
        if self.HHLBDGMIBNP != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.HHLBDGMIBNP);
        }
        if self.BGFEKHFMEIH != false {
            my_size += 1 + 1;
        }
        if self.PGGMKGMBBKJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.PGGMKGMBBKJ);
        }
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LJILKDFEMCF != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LJILKDFEMCF);
        }
        if self.EMFIDIGHFCC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.EMFIDIGHFCC);
        }
        if self.AOPKNFPCPBB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.AOPKNFPCPBB);
        }
        if self.DOIMONBNDII != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.DOIMONBNDII);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HHLBDGMIBNP != 0 {
            os.write_uint32(7, self.HHLBDGMIBNP)?;
        }
        if self.BGFEKHFMEIH != false {
            os.write_bool(12, self.BGFEKHFMEIH)?;
        }
        if self.PGGMKGMBBKJ != 0 {
            os.write_uint32(3, self.PGGMKGMBBKJ)?;
        }
        if let Some(v) = self.CPJGLAJDNAC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.LJILKDFEMCF != 0 {
            os.write_uint32(2, self.LJILKDFEMCF)?;
        }
        if self.EMFIDIGHFCC != 0 {
            os.write_uint32(1, self.EMFIDIGHFCC)?;
        }
        if self.AOPKNFPCPBB != 0 {
            os.write_uint32(6, self.AOPKNFPCPBB)?;
        }
        if self.DOIMONBNDII != 0 {
            os.write_uint32(13, self.DOIMONBNDII)?;
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

    fn new() -> AJIEDNODEAO {
        AJIEDNODEAO::new()
    }

    fn clear(&mut self) {
        self.HHLBDGMIBNP = 0;
        self.BGFEKHFMEIH = false;
        self.PGGMKGMBBKJ = 0;
        self.CPJGLAJDNAC.clear();
        self.LJILKDFEMCF = 0;
        self.EMFIDIGHFCC = 0;
        self.AOPKNFPCPBB = 0;
        self.DOIMONBNDII = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AJIEDNODEAO {
        static instance: AJIEDNODEAO = AJIEDNODEAO {
            HHLBDGMIBNP: 0,
            BGFEKHFMEIH: false,
            PGGMKGMBBKJ: 0,
            CPJGLAJDNAC: ::protobuf::MessageField::none(),
            LJILKDFEMCF: 0,
            EMFIDIGHFCC: 0,
            AOPKNFPCPBB: 0,
            DOIMONBNDII: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AJIEDNODEAO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AJIEDNODEAO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AJIEDNODEAO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AJIEDNODEAO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AJIEDNODEAO.proto\x1a\x11APKJGAJLLEA.proto\"\xab\x02\n\x0bAJIEDNOD\
    EAO\x12\x20\n\x0bHHLBDGMIBNP\x18\x07\x20\x01(\rR\x0bHHLBDGMIBNP\x12\x20\
    \n\x0bBGFEKHFMEIH\x18\x0c\x20\x01(\x08R\x0bBGFEKHFMEIH\x12\x20\n\x0bPGGM\
    KGMBBKJ\x18\x03\x20\x01(\rR\x0bPGGMKGMBBKJ\x12.\n\x0bCPJGLAJDNAC\x18\x0b\
    \x20\x01(\x0b2\x0c.APKJGAJLLEAR\x0bCPJGLAJDNAC\x12\x20\n\x0bLJILKDFEMCF\
    \x18\x02\x20\x01(\rR\x0bLJILKDFEMCF\x12\x20\n\x0bEMFIDIGHFCC\x18\x01\x20\
    \x01(\rR\x0bEMFIDIGHFCC\x12\x20\n\x0bAOPKNFPCPBB\x18\x06\x20\x01(\rR\x0b\
    AOPKNFPCPBB\x12\x20\n\x0bDOIMONBNDII\x18\r\x20\x01(\rR\x0bDOIMONBNDIIb\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::APKJGAJLLEA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AJIEDNODEAO::generated_message_descriptor_data());
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