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

//! Generated file from `StartRaidCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartRaidCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartRaidCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartRaidCsReq.JCPFOPLBOIM)
    pub JCPFOPLBOIM: u32,
    // @@protoc_insertion_point(field:StartRaidCsReq.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:StartRaidCsReq.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:StartRaidCsReq.ODOAJJGMBCL)
    pub ODOAJJGMBCL: u32,
    // @@protoc_insertion_point(field:StartRaidCsReq.KDFKCJLHHAF)
    pub KDFKCJLHHAF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartRaidCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartRaidCsReq {
    fn default() -> &'a StartRaidCsReq {
        <StartRaidCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartRaidCsReq {
    pub fn new() -> StartRaidCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCPFOPLBOIM",
            |m: &StartRaidCsReq| { &m.JCPFOPLBOIM },
            |m: &mut StartRaidCsReq| { &mut m.JCPFOPLBOIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &StartRaidCsReq| { &m.DNMJBNNJLEL },
            |m: &mut StartRaidCsReq| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &StartRaidCsReq| { &m.EMALNMLGANJ },
            |m: &mut StartRaidCsReq| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODOAJJGMBCL",
            |m: &StartRaidCsReq| { &m.ODOAJJGMBCL },
            |m: &mut StartRaidCsReq| { &mut m.ODOAJJGMBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KDFKCJLHHAF",
            |m: &StartRaidCsReq| { &m.KDFKCJLHHAF },
            |m: &mut StartRaidCsReq| { &mut m.KDFKCJLHHAF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartRaidCsReq>(
            "StartRaidCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartRaidCsReq {
    const NAME: &'static str = "StartRaidCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.JCPFOPLBOIM = is.read_uint32()?;
                },
                104 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.EMALNMLGANJ)?;
                },
                120 => {
                    self.EMALNMLGANJ.push(is.read_uint32()?);
                },
                112 => {
                    self.ODOAJJGMBCL = is.read_uint32()?;
                },
                24 => {
                    self.KDFKCJLHHAF = is.read_uint32()?;
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
        if self.JCPFOPLBOIM != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.JCPFOPLBOIM);
        }
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.DNMJBNNJLEL);
        }
        for value in &self.EMALNMLGANJ {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.ODOAJJGMBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.ODOAJJGMBCL);
        }
        if self.KDFKCJLHHAF != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.KDFKCJLHHAF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JCPFOPLBOIM != 0 {
            os.write_uint32(6, self.JCPFOPLBOIM)?;
        }
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(13, self.DNMJBNNJLEL)?;
        }
        for v in &self.EMALNMLGANJ {
            os.write_uint32(15, *v)?;
        };
        if self.ODOAJJGMBCL != 0 {
            os.write_uint32(14, self.ODOAJJGMBCL)?;
        }
        if self.KDFKCJLHHAF != 0 {
            os.write_uint32(3, self.KDFKCJLHHAF)?;
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

    fn new() -> StartRaidCsReq {
        StartRaidCsReq::new()
    }

    fn clear(&mut self) {
        self.JCPFOPLBOIM = 0;
        self.DNMJBNNJLEL = 0;
        self.EMALNMLGANJ.clear();
        self.ODOAJJGMBCL = 0;
        self.KDFKCJLHHAF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartRaidCsReq {
        static instance: StartRaidCsReq = StartRaidCsReq {
            JCPFOPLBOIM: 0,
            DNMJBNNJLEL: 0,
            EMALNMLGANJ: ::std::vec::Vec::new(),
            ODOAJJGMBCL: 0,
            KDFKCJLHHAF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartRaidCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartRaidCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartRaidCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartRaidCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14StartRaidCsReq.proto\"\xba\x01\n\x0eStartRaidCsReq\x12\x20\n\x0bJC\
    PFOPLBOIM\x18\x06\x20\x01(\rR\x0bJCPFOPLBOIM\x12\x20\n\x0bDNMJBNNJLEL\
    \x18\r\x20\x01(\rR\x0bDNMJBNNJLEL\x12\x20\n\x0bEMALNMLGANJ\x18\x0f\x20\
    \x03(\rR\x0bEMALNMLGANJ\x12\x20\n\x0bODOAJJGMBCL\x18\x0e\x20\x01(\rR\x0b\
    ODOAJJGMBCL\x12\x20\n\x0bKDFKCJLHHAF\x18\x03\x20\x01(\rR\x0bKDFKCJLHHAFb\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartRaidCsReq::generated_message_descriptor_data());
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
