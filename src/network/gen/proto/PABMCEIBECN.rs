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

//! Generated file from `PABMCEIBECN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PABMCEIBECN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PABMCEIBECN {
    // message fields
    // @@protoc_insertion_point(field:PABMCEIBECN.LOIFEADKBKB)
    pub LOIFEADKBKB: u32,
    // @@protoc_insertion_point(field:PABMCEIBECN.FODEHAMPGJB)
    pub FODEHAMPGJB: u32,
    // @@protoc_insertion_point(field:PABMCEIBECN.JLOIJKNCEAM)
    pub JLOIJKNCEAM: u32,
    // @@protoc_insertion_point(field:PABMCEIBECN.HHLNGHKNBFE)
    pub HHLNGHKNBFE: u32,
    // @@protoc_insertion_point(field:PABMCEIBECN.EEBNCINFDMB)
    pub EEBNCINFDMB: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PABMCEIBECN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PABMCEIBECN {
    fn default() -> &'a PABMCEIBECN {
        <PABMCEIBECN as ::protobuf::Message>::default_instance()
    }
}

impl PABMCEIBECN {
    pub fn new() -> PABMCEIBECN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOIFEADKBKB",
            |m: &PABMCEIBECN| { &m.LOIFEADKBKB },
            |m: &mut PABMCEIBECN| { &mut m.LOIFEADKBKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FODEHAMPGJB",
            |m: &PABMCEIBECN| { &m.FODEHAMPGJB },
            |m: &mut PABMCEIBECN| { &mut m.FODEHAMPGJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLOIJKNCEAM",
            |m: &PABMCEIBECN| { &m.JLOIJKNCEAM },
            |m: &mut PABMCEIBECN| { &mut m.JLOIJKNCEAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHLNGHKNBFE",
            |m: &PABMCEIBECN| { &m.HHLNGHKNBFE },
            |m: &mut PABMCEIBECN| { &mut m.HHLNGHKNBFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "EEBNCINFDMB",
            |m: &PABMCEIBECN| { &m.EEBNCINFDMB },
            |m: &mut PABMCEIBECN| { &mut m.EEBNCINFDMB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PABMCEIBECN>(
            "PABMCEIBECN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PABMCEIBECN {
    const NAME: &'static str = "PABMCEIBECN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.LOIFEADKBKB = is.read_uint32()?;
                },
                56 => {
                    self.FODEHAMPGJB = is.read_uint32()?;
                },
                8 => {
                    self.JLOIJKNCEAM = is.read_uint32()?;
                },
                80 => {
                    self.HHLNGHKNBFE = is.read_uint32()?;
                },
                66 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.EEBNCINFDMB.insert(key, value);
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
        if self.LOIFEADKBKB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.LOIFEADKBKB);
        }
        if self.FODEHAMPGJB != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FODEHAMPGJB);
        }
        if self.JLOIJKNCEAM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.JLOIJKNCEAM);
        }
        if self.HHLNGHKNBFE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.HHLNGHKNBFE);
        }
        for (k, v) in &self.EEBNCINFDMB {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LOIFEADKBKB != 0 {
            os.write_uint32(2, self.LOIFEADKBKB)?;
        }
        if self.FODEHAMPGJB != 0 {
            os.write_uint32(7, self.FODEHAMPGJB)?;
        }
        if self.JLOIJKNCEAM != 0 {
            os.write_uint32(1, self.JLOIJKNCEAM)?;
        }
        if self.HHLNGHKNBFE != 0 {
            os.write_uint32(10, self.HHLNGHKNBFE)?;
        }
        for (k, v) in &self.EEBNCINFDMB {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(66)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
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

    fn new() -> PABMCEIBECN {
        PABMCEIBECN::new()
    }

    fn clear(&mut self) {
        self.LOIFEADKBKB = 0;
        self.FODEHAMPGJB = 0;
        self.JLOIJKNCEAM = 0;
        self.HHLNGHKNBFE = 0;
        self.EEBNCINFDMB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PABMCEIBECN {
        static instance: ::protobuf::rt::Lazy<PABMCEIBECN> = ::protobuf::rt::Lazy::new();
        instance.get(PABMCEIBECN::new)
    }
}

impl ::protobuf::MessageFull for PABMCEIBECN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PABMCEIBECN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PABMCEIBECN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PABMCEIBECN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PABMCEIBECN.proto\"\x96\x02\n\x0bPABMCEIBECN\x12\x20\n\x0bLOIFEADK\
    BKB\x18\x02\x20\x01(\rR\x0bLOIFEADKBKB\x12\x20\n\x0bFODEHAMPGJB\x18\x07\
    \x20\x01(\rR\x0bFODEHAMPGJB\x12\x20\n\x0bJLOIJKNCEAM\x18\x01\x20\x01(\rR\
    \x0bJLOIJKNCEAM\x12\x20\n\x0bHHLNGHKNBFE\x18\n\x20\x01(\rR\x0bHHLNGHKNBF\
    E\x12?\n\x0bEEBNCINFDMB\x18\x08\x20\x03(\x0b2\x1d.PABMCEIBECN.EEBNCINFDM\
    BEntryR\x0bEEBNCINFDMB\x1a>\n\x10EEBNCINFDMBEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\
    \x028\x01b\x06proto3\
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
            messages.push(PABMCEIBECN::generated_message_descriptor_data());
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