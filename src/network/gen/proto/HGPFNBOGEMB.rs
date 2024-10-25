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

//! Generated file from `HGPFNBOGEMB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HGPFNBOGEMB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HGPFNBOGEMB {
    // message fields
    // @@protoc_insertion_point(field:HGPFNBOGEMB.GCFIIGOLPMF)
    pub GCFIIGOLPMF: u32,
    // @@protoc_insertion_point(field:HGPFNBOGEMB.DPIKDBPKAOK)
    pub DPIKDBPKAOK: bool,
    // @@protoc_insertion_point(field:HGPFNBOGEMB.LFFHDKHBHNN)
    pub LFFHDKHBHNN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HGPFNBOGEMB.EJFFFEFLHGK)
    pub EJFFFEFLHGK: i64,
    // special fields
    // @@protoc_insertion_point(special_field:HGPFNBOGEMB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HGPFNBOGEMB {
    fn default() -> &'a HGPFNBOGEMB {
        <HGPFNBOGEMB as ::protobuf::Message>::default_instance()
    }
}

impl HGPFNBOGEMB {
    pub fn new() -> HGPFNBOGEMB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCFIIGOLPMF",
            |m: &HGPFNBOGEMB| { &m.GCFIIGOLPMF },
            |m: &mut HGPFNBOGEMB| { &mut m.GCFIIGOLPMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DPIKDBPKAOK",
            |m: &HGPFNBOGEMB| { &m.DPIKDBPKAOK },
            |m: &mut HGPFNBOGEMB| { &mut m.DPIKDBPKAOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LFFHDKHBHNN",
            |m: &HGPFNBOGEMB| { &m.LFFHDKHBHNN },
            |m: &mut HGPFNBOGEMB| { &mut m.LFFHDKHBHNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EJFFFEFLHGK",
            |m: &HGPFNBOGEMB| { &m.EJFFFEFLHGK },
            |m: &mut HGPFNBOGEMB| { &mut m.EJFFFEFLHGK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HGPFNBOGEMB>(
            "HGPFNBOGEMB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HGPFNBOGEMB {
    const NAME: &'static str = "HGPFNBOGEMB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.GCFIIGOLPMF = is.read_uint32()?;
                },
                16 => {
                    self.DPIKDBPKAOK = is.read_bool()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.LFFHDKHBHNN)?;
                },
                120 => {
                    self.LFFHDKHBHNN.push(is.read_uint32()?);
                },
                104 => {
                    self.EJFFFEFLHGK = is.read_int64()?;
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
        if self.GCFIIGOLPMF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GCFIIGOLPMF);
        }
        if self.DPIKDBPKAOK != false {
            my_size += 1 + 1;
        }
        for value in &self.LFFHDKHBHNN {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.EJFFFEFLHGK != 0 {
            my_size += ::protobuf::rt::int64_size(13, self.EJFFFEFLHGK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GCFIIGOLPMF != 0 {
            os.write_uint32(5, self.GCFIIGOLPMF)?;
        }
        if self.DPIKDBPKAOK != false {
            os.write_bool(2, self.DPIKDBPKAOK)?;
        }
        for v in &self.LFFHDKHBHNN {
            os.write_uint32(15, *v)?;
        };
        if self.EJFFFEFLHGK != 0 {
            os.write_int64(13, self.EJFFFEFLHGK)?;
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

    fn new() -> HGPFNBOGEMB {
        HGPFNBOGEMB::new()
    }

    fn clear(&mut self) {
        self.GCFIIGOLPMF = 0;
        self.DPIKDBPKAOK = false;
        self.LFFHDKHBHNN.clear();
        self.EJFFFEFLHGK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HGPFNBOGEMB {
        static instance: HGPFNBOGEMB = HGPFNBOGEMB {
            GCFIIGOLPMF: 0,
            DPIKDBPKAOK: false,
            LFFHDKHBHNN: ::std::vec::Vec::new(),
            EJFFFEFLHGK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HGPFNBOGEMB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HGPFNBOGEMB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HGPFNBOGEMB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HGPFNBOGEMB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HGPFNBOGEMB.proto\"\x95\x01\n\x0bHGPFNBOGEMB\x12\x20\n\x0bGCFIIGOL\
    PMF\x18\x05\x20\x01(\rR\x0bGCFIIGOLPMF\x12\x20\n\x0bDPIKDBPKAOK\x18\x02\
    \x20\x01(\x08R\x0bDPIKDBPKAOK\x12\x20\n\x0bLFFHDKHBHNN\x18\x0f\x20\x03(\
    \rR\x0bLFFHDKHBHNN\x12\x20\n\x0bEJFFFEFLHGK\x18\r\x20\x01(\x03R\x0bEJFFF\
    EFLHGKb\x06proto3\
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
            messages.push(HGPFNBOGEMB::generated_message_descriptor_data());
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
