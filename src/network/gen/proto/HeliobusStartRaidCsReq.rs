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

//! Generated file from `HeliobusStartRaidCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HeliobusStartRaidCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeliobusStartRaidCsReq {
    // message fields
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.MEJPGIDEBMI)
    pub MEJPGIDEBMI: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.LNEDFBLNHEN)
    pub LNEDFBLNHEN: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.BEDDOLCOMPO)
    pub BEDDOLCOMPO: bool,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.OGIOAKBPMAE)
    pub OGIOAKBPMAE: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.JFPOBLGOPCP)
    pub JFPOBLGOPCP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HeliobusStartRaidCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeliobusStartRaidCsReq {
    fn default() -> &'a HeliobusStartRaidCsReq {
        <HeliobusStartRaidCsReq as ::protobuf::Message>::default_instance()
    }
}

impl HeliobusStartRaidCsReq {
    pub fn new() -> HeliobusStartRaidCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MEJPGIDEBMI",
            |m: &HeliobusStartRaidCsReq| { &m.MEJPGIDEBMI },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.MEJPGIDEBMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNEDFBLNHEN",
            |m: &HeliobusStartRaidCsReq| { &m.LNEDFBLNHEN },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.LNEDFBLNHEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BEDDOLCOMPO",
            |m: &HeliobusStartRaidCsReq| { &m.BEDDOLCOMPO },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.BEDDOLCOMPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGIOAKBPMAE",
            |m: &HeliobusStartRaidCsReq| { &m.OGIOAKBPMAE },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.OGIOAKBPMAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &HeliobusStartRaidCsReq| { &m.avatar_list },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFPOBLGOPCP",
            |m: &HeliobusStartRaidCsReq| { &m.JFPOBLGOPCP },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.JFPOBLGOPCP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeliobusStartRaidCsReq>(
            "HeliobusStartRaidCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeliobusStartRaidCsReq {
    const NAME: &'static str = "HeliobusStartRaidCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.MEJPGIDEBMI = is.read_uint32()?;
                },
                120 => {
                    self.LNEDFBLNHEN = is.read_uint32()?;
                },
                48 => {
                    self.BEDDOLCOMPO = is.read_bool()?;
                },
                24 => {
                    self.OGIOAKBPMAE = is.read_uint32()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_list)?;
                },
                80 => {
                    self.avatar_list.push(is.read_uint32()?);
                },
                40 => {
                    self.JFPOBLGOPCP = is.read_uint32()?;
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
        if self.MEJPGIDEBMI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.MEJPGIDEBMI);
        }
        if self.LNEDFBLNHEN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LNEDFBLNHEN);
        }
        if self.BEDDOLCOMPO != false {
            my_size += 1 + 1;
        }
        if self.OGIOAKBPMAE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OGIOAKBPMAE);
        }
        for value in &self.avatar_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.JFPOBLGOPCP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.JFPOBLGOPCP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MEJPGIDEBMI != 0 {
            os.write_uint32(2, self.MEJPGIDEBMI)?;
        }
        if self.LNEDFBLNHEN != 0 {
            os.write_uint32(15, self.LNEDFBLNHEN)?;
        }
        if self.BEDDOLCOMPO != false {
            os.write_bool(6, self.BEDDOLCOMPO)?;
        }
        if self.OGIOAKBPMAE != 0 {
            os.write_uint32(3, self.OGIOAKBPMAE)?;
        }
        for v in &self.avatar_list {
            os.write_uint32(10, *v)?;
        };
        if self.JFPOBLGOPCP != 0 {
            os.write_uint32(5, self.JFPOBLGOPCP)?;
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

    fn new() -> HeliobusStartRaidCsReq {
        HeliobusStartRaidCsReq::new()
    }

    fn clear(&mut self) {
        self.MEJPGIDEBMI = 0;
        self.LNEDFBLNHEN = 0;
        self.BEDDOLCOMPO = false;
        self.OGIOAKBPMAE = 0;
        self.avatar_list.clear();
        self.JFPOBLGOPCP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeliobusStartRaidCsReq {
        static instance: HeliobusStartRaidCsReq = HeliobusStartRaidCsReq {
            MEJPGIDEBMI: 0,
            LNEDFBLNHEN: 0,
            BEDDOLCOMPO: false,
            OGIOAKBPMAE: 0,
            avatar_list: ::std::vec::Vec::new(),
            JFPOBLGOPCP: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeliobusStartRaidCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeliobusStartRaidCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeliobusStartRaidCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeliobusStartRaidCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cHeliobusStartRaidCsReq.proto\"\xe3\x01\n\x16HeliobusStartRaidCsReq\
    \x12\x20\n\x0bMEJPGIDEBMI\x18\x02\x20\x01(\rR\x0bMEJPGIDEBMI\x12\x20\n\
    \x0bLNEDFBLNHEN\x18\x0f\x20\x01(\rR\x0bLNEDFBLNHEN\x12\x20\n\x0bBEDDOLCO\
    MPO\x18\x06\x20\x01(\x08R\x0bBEDDOLCOMPO\x12\x20\n\x0bOGIOAKBPMAE\x18\
    \x03\x20\x01(\rR\x0bOGIOAKBPMAE\x12\x1f\n\x0bavatar_list\x18\n\x20\x03(\
    \rR\navatarList\x12\x20\n\x0bJFPOBLGOPCP\x18\x05\x20\x01(\rR\x0bJFPOBLGO\
    PCPb\x06proto3\
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
            messages.push(HeliobusStartRaidCsReq::generated_message_descriptor_data());
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
