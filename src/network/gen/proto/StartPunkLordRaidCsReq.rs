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

//! Generated file from `StartPunkLordRaidCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartPunkLordRaidCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartPunkLordRaidCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartPunkLordRaidCsReq.HFMDIBOEOKN)
    pub HFMDIBOEOKN: u32,
    // @@protoc_insertion_point(field:StartPunkLordRaidCsReq.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:StartPunkLordRaidCsReq.KEPHAGKCAGK)
    pub KEPHAGKCAGK: bool,
    // special fields
    // @@protoc_insertion_point(special_field:StartPunkLordRaidCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartPunkLordRaidCsReq {
    fn default() -> &'a StartPunkLordRaidCsReq {
        <StartPunkLordRaidCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartPunkLordRaidCsReq {
    pub fn new() -> StartPunkLordRaidCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFMDIBOEOKN",
            |m: &StartPunkLordRaidCsReq| { &m.HFMDIBOEOKN },
            |m: &mut StartPunkLordRaidCsReq| { &mut m.HFMDIBOEOKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &StartPunkLordRaidCsReq| { &m.uid },
            |m: &mut StartPunkLordRaidCsReq| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KEPHAGKCAGK",
            |m: &StartPunkLordRaidCsReq| { &m.KEPHAGKCAGK },
            |m: &mut StartPunkLordRaidCsReq| { &mut m.KEPHAGKCAGK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartPunkLordRaidCsReq>(
            "StartPunkLordRaidCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartPunkLordRaidCsReq {
    const NAME: &'static str = "StartPunkLordRaidCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.HFMDIBOEOKN = is.read_uint32()?;
                },
                64 => {
                    self.uid = is.read_uint32()?;
                },
                48 => {
                    self.KEPHAGKCAGK = is.read_bool()?;
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
        if self.HFMDIBOEOKN != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.HFMDIBOEOKN);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.uid);
        }
        if self.KEPHAGKCAGK != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HFMDIBOEOKN != 0 {
            os.write_uint32(12, self.HFMDIBOEOKN)?;
        }
        if self.uid != 0 {
            os.write_uint32(8, self.uid)?;
        }
        if self.KEPHAGKCAGK != false {
            os.write_bool(6, self.KEPHAGKCAGK)?;
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

    fn new() -> StartPunkLordRaidCsReq {
        StartPunkLordRaidCsReq::new()
    }

    fn clear(&mut self) {
        self.HFMDIBOEOKN = 0;
        self.uid = 0;
        self.KEPHAGKCAGK = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartPunkLordRaidCsReq {
        static instance: StartPunkLordRaidCsReq = StartPunkLordRaidCsReq {
            HFMDIBOEOKN: 0,
            uid: 0,
            KEPHAGKCAGK: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartPunkLordRaidCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartPunkLordRaidCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartPunkLordRaidCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartPunkLordRaidCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cStartPunkLordRaidCsReq.proto\"n\n\x16StartPunkLordRaidCsReq\x12\
    \x20\n\x0bHFMDIBOEOKN\x18\x0c\x20\x01(\rR\x0bHFMDIBOEOKN\x12\x10\n\x03ui\
    d\x18\x08\x20\x01(\rR\x03uid\x12\x20\n\x0bKEPHAGKCAGK\x18\x06\x20\x01(\
    \x08R\x0bKEPHAGKCAGKb\x06proto3\
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
            messages.push(StartPunkLordRaidCsReq::generated_message_descriptor_data());
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
