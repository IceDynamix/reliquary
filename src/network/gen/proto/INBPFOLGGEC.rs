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

//! Generated file from `INBPFOLGGEC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:INBPFOLGGEC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct INBPFOLGGEC {
    // message fields
    // @@protoc_insertion_point(field:INBPFOLGGEC.LELABBBOIKN)
    pub LELABBBOIKN: u32,
    // @@protoc_insertion_point(field:INBPFOLGGEC.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // @@protoc_insertion_point(field:INBPFOLGGEC.FJPEFPBFALC)
    pub FJPEFPBFALC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:INBPFOLGGEC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a INBPFOLGGEC {
    fn default() -> &'a INBPFOLGGEC {
        <INBPFOLGGEC as ::protobuf::Message>::default_instance()
    }
}

impl INBPFOLGGEC {
    pub fn new() -> INBPFOLGGEC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LELABBBOIKN",
            |m: &INBPFOLGGEC| { &m.LELABBBOIKN },
            |m: &mut INBPFOLGGEC| { &mut m.LELABBBOIKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &INBPFOLGGEC| { &m.GCFIIGOLPMF },
            |m: &mut INBPFOLGGEC| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJPEFPBFALC",
            |m: &INBPFOLGGEC| { &m.FJPEFPBFALC },
            |m: &mut INBPFOLGGEC| { &mut m.FJPEFPBFALC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<INBPFOLGGEC>(
            "INBPFOLGGEC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for INBPFOLGGEC {
    const NAME: &'static str = "INBPFOLGGEC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.LELABBBOIKN = is.read_uint32()?;
                },
                40 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                88 => {
                    self.FJPEFPBFALC = is.read_uint32()?;
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
        if self.LELABBBOIKN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LELABBBOIKN);
        }
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GCFIIGOLPMF);
        }
        if self.FJPEFPBFALC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.FJPEFPBFALC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LELABBBOIKN != 0 {
            os.write_uint32(8, self.LELABBBOIKN)?;
        }
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(5, self.GCFIIGOLPMF)?;
        }
        if self.FJPEFPBFALC != 0 {
            os.write_uint32(11, self.FJPEFPBFALC)?;
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

    fn new() -> INBPFOLGGEC {
        INBPFOLGGEC::new()
    }

    fn clear(&mut self) {
        self.LELABBBOIKN = 0;
        self.GCFIIGOLPMF = 0;
        self.FJPEFPBFALC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static INBPFOLGGEC {
        static instance: INBPFOLGGEC = INBPFOLGGEC {
            LELABBBOIKN: 0,
            GCFIIGOLPMF: 0,
            FJPEFPBFALC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for INBPFOLGGEC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("INBPFOLGGEC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for INBPFOLGGEC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INBPFOLGGEC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11INBPFOLGGEC.proto\"s\n\x0bINBPFOLGGEC\x12\x20\n\x0bLELABBBOIKN\x18\
    \x08\x20\x01(\rR\x0bLELABBBOIKN\x12\x20\n\x0bGCFIIGOLPMF\x18\x05\x20\x01\
    (\rR\x0bGCFIIGOLPMF\x12\x20\n\x0bFJPEFPBFALC\x18\x0b\x20\x01(\rR\x0bFJPE\
    FPBFALCb\x06proto3\
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
            messages.push(INBPFOLGGEC::generated_message_descriptor_data());
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
