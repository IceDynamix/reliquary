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

//! Generated file from `SelectChessRogueSubStoryScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SelectChessRogueSubStoryScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SelectChessRogueSubStoryScRsp {
    // message fields
    // @@protoc_insertion_point(field:SelectChessRogueSubStoryScRsp.ICCDOBIGCLN)
    pub ICCDOBIGCLN: u32,
    // @@protoc_insertion_point(field:SelectChessRogueSubStoryScRsp.PGHAPHCHHDA)
    pub PGHAPHCHHDA: u32,
    // @@protoc_insertion_point(field:SelectChessRogueSubStoryScRsp.NMEHGPBPMIG)
    pub NMEHGPBPMIG: u32,
    // @@protoc_insertion_point(field:SelectChessRogueSubStoryScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:SelectChessRogueSubStoryScRsp.IBGBNAJCIHD)
    pub IBGBNAJCIHD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SelectChessRogueSubStoryScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SelectChessRogueSubStoryScRsp {
    fn default() -> &'a SelectChessRogueSubStoryScRsp {
        <SelectChessRogueSubStoryScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SelectChessRogueSubStoryScRsp {
    pub fn new() -> SelectChessRogueSubStoryScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICCDOBIGCLN",
            |m: &SelectChessRogueSubStoryScRsp| { &m.ICCDOBIGCLN },
            |m: &mut SelectChessRogueSubStoryScRsp| { &mut m.ICCDOBIGCLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGHAPHCHHDA",
            |m: &SelectChessRogueSubStoryScRsp| { &m.PGHAPHCHHDA },
            |m: &mut SelectChessRogueSubStoryScRsp| { &mut m.PGHAPHCHHDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMEHGPBPMIG",
            |m: &SelectChessRogueSubStoryScRsp| { &m.NMEHGPBPMIG },
            |m: &mut SelectChessRogueSubStoryScRsp| { &mut m.NMEHGPBPMIG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &SelectChessRogueSubStoryScRsp| { &m.ADADHIHDHJC },
            |m: &mut SelectChessRogueSubStoryScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBGBNAJCIHD",
            |m: &SelectChessRogueSubStoryScRsp| { &m.IBGBNAJCIHD },
            |m: &mut SelectChessRogueSubStoryScRsp| { &mut m.IBGBNAJCIHD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SelectChessRogueSubStoryScRsp>(
            "SelectChessRogueSubStoryScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SelectChessRogueSubStoryScRsp {
    const NAME: &'static str = "SelectChessRogueSubStoryScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.ICCDOBIGCLN = is.read_uint32()?;
                },
                88 => {
                    self.PGHAPHCHHDA = is.read_uint32()?;
                },
                72 => {
                    self.NMEHGPBPMIG = is.read_uint32()?;
                },
                104 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                112 => {
                    self.IBGBNAJCIHD = is.read_uint32()?;
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
        if self.ICCDOBIGCLN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.ICCDOBIGCLN);
        }
        if self.PGHAPHCHHDA != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.PGHAPHCHHDA);
        }
        if self.NMEHGPBPMIG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NMEHGPBPMIG);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.ADADHIHDHJC);
        }
        if self.IBGBNAJCIHD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IBGBNAJCIHD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ICCDOBIGCLN != 0 {
            os.write_uint32(7, self.ICCDOBIGCLN)?;
        }
        if self.PGHAPHCHHDA != 0 {
            os.write_uint32(11, self.PGHAPHCHHDA)?;
        }
        if self.NMEHGPBPMIG != 0 {
            os.write_uint32(9, self.NMEHGPBPMIG)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(13, self.ADADHIHDHJC)?;
        }
        if self.IBGBNAJCIHD != 0 {
            os.write_uint32(14, self.IBGBNAJCIHD)?;
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

    fn new() -> SelectChessRogueSubStoryScRsp {
        SelectChessRogueSubStoryScRsp::new()
    }

    fn clear(&mut self) {
        self.ICCDOBIGCLN = 0;
        self.PGHAPHCHHDA = 0;
        self.NMEHGPBPMIG = 0;
        self.ADADHIHDHJC = 0;
        self.IBGBNAJCIHD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SelectChessRogueSubStoryScRsp {
        static instance: SelectChessRogueSubStoryScRsp = SelectChessRogueSubStoryScRsp {
            ICCDOBIGCLN: 0,
            PGHAPHCHHDA: 0,
            NMEHGPBPMIG: 0,
            ADADHIHDHJC: 0,
            IBGBNAJCIHD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SelectChessRogueSubStoryScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SelectChessRogueSubStoryScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SelectChessRogueSubStoryScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectChessRogueSubStoryScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#SelectChessRogueSubStoryScRsp.proto\"\xc9\x01\n\x1dSelectChessRogueSu\
    bStoryScRsp\x12\x20\n\x0bICCDOBIGCLN\x18\x07\x20\x01(\rR\x0bICCDOBIGCLN\
    \x12\x20\n\x0bPGHAPHCHHDA\x18\x0b\x20\x01(\rR\x0bPGHAPHCHHDA\x12\x20\n\
    \x0bNMEHGPBPMIG\x18\t\x20\x01(\rR\x0bNMEHGPBPMIG\x12\x20\n\x0bADADHIHDHJ\
    C\x18\r\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bIBGBNAJCIHD\x18\x0e\x20\
    \x01(\rR\x0bIBGBNAJCIHDb\x06proto3\
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
            messages.push(SelectChessRogueSubStoryScRsp::generated_message_descriptor_data());
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
