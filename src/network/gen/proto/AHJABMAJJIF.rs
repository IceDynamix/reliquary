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

//! Generated file from `AHJABMAJJIF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AHJABMAJJIF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AHJABMAJJIF {
    // message fields
    // @@protoc_insertion_point(field:AHJABMAJJIF.PGHNMIDJOAC)
    pub PGHNMIDJOAC: u32,
    // @@protoc_insertion_point(field:AHJABMAJJIF.EANJBBEKKII)
    pub EANJBBEKKII: ::std::string::String,
    // @@protoc_insertion_point(field:AHJABMAJJIF.NKEANFPKDMN)
    pub NKEANFPKDMN: u32,
    // @@protoc_insertion_point(field:AHJABMAJJIF.JFELPOKMMOA)
    pub JFELPOKMMOA: u32,
    // @@protoc_insertion_point(field:AHJABMAJJIF.FANNOHCHAMP)
    pub FANNOHCHAMP: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:AHJABMAJJIF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AHJABMAJJIF {
    fn default() -> &'a AHJABMAJJIF {
        <AHJABMAJJIF as ::protobuf::Message>::default_instance()
    }
}

impl AHJABMAJJIF {
    pub fn new() -> AHJABMAJJIF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PGHNMIDJOAC",
            |m: &AHJABMAJJIF| { &m.PGHNMIDJOAC },
            |m: &mut AHJABMAJJIF| { &mut m.PGHNMIDJOAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EANJBBEKKII",
            |m: &AHJABMAJJIF| { &m.EANJBBEKKII },
            |m: &mut AHJABMAJJIF| { &mut m.EANJBBEKKII },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKEANFPKDMN",
            |m: &AHJABMAJJIF| { &m.NKEANFPKDMN },
            |m: &mut AHJABMAJJIF| { &mut m.NKEANFPKDMN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFELPOKMMOA",
            |m: &AHJABMAJJIF| { &m.JFELPOKMMOA },
            |m: &mut AHJABMAJJIF| { &mut m.JFELPOKMMOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FANNOHCHAMP",
            |m: &AHJABMAJJIF| { &m.FANNOHCHAMP },
            |m: &mut AHJABMAJJIF| { &mut m.FANNOHCHAMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AHJABMAJJIF>(
            "AHJABMAJJIF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AHJABMAJJIF {
    const NAME: &'static str = "AHJABMAJJIF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.PGHNMIDJOAC = is.read_uint32()?;
                },
                50 => {
                    self.EANJBBEKKII = is.read_string()?;
                },
                120 => {
                    self.NKEANFPKDMN = is.read_uint32()?;
                },
                96 => {
                    self.JFELPOKMMOA = is.read_uint32()?;
                },
                34 => {
                    self.FANNOHCHAMP = is.read_string()?;
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
        if self.PGHNMIDJOAC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PGHNMIDJOAC);
        }
        if !self.EANJBBEKKII.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.EANJBBEKKII);
        }
        if self.NKEANFPKDMN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.NKEANFPKDMN);
        }
        if self.JFELPOKMMOA != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.JFELPOKMMOA);
        }
        if !self.FANNOHCHAMP.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.FANNOHCHAMP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PGHNMIDJOAC != 0 {
            os.write_uint32(8, self.PGHNMIDJOAC)?;
        }
        if !self.EANJBBEKKII.is_empty() {
            os.write_string(6, &self.EANJBBEKKII)?;
        }
        if self.NKEANFPKDMN != 0 {
            os.write_uint32(15, self.NKEANFPKDMN)?;
        }
        if self.JFELPOKMMOA != 0 {
            os.write_uint32(12, self.JFELPOKMMOA)?;
        }
        if !self.FANNOHCHAMP.is_empty() {
            os.write_string(4, &self.FANNOHCHAMP)?;
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

    fn new() -> AHJABMAJJIF {
        AHJABMAJJIF::new()
    }

    fn clear(&mut self) {
        self.PGHNMIDJOAC = 0;
        self.EANJBBEKKII.clear();
        self.NKEANFPKDMN = 0;
        self.JFELPOKMMOA = 0;
        self.FANNOHCHAMP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AHJABMAJJIF {
        static instance: AHJABMAJJIF = AHJABMAJJIF {
            PGHNMIDJOAC: 0,
            EANJBBEKKII: ::std::string::String::new(),
            NKEANFPKDMN: 0,
            JFELPOKMMOA: 0,
            FANNOHCHAMP: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AHJABMAJJIF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AHJABMAJJIF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AHJABMAJJIF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AHJABMAJJIF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AHJABMAJJIF.proto\"\xb7\x01\n\x0bAHJABMAJJIF\x12\x20\n\x0bPGHNMIDJ\
    OAC\x18\x08\x20\x01(\rR\x0bPGHNMIDJOAC\x12\x20\n\x0bEANJBBEKKII\x18\x06\
    \x20\x01(\tR\x0bEANJBBEKKII\x12\x20\n\x0bNKEANFPKDMN\x18\x0f\x20\x01(\rR\
    \x0bNKEANFPKDMN\x12\x20\n\x0bJFELPOKMMOA\x18\x0c\x20\x01(\rR\x0bJFELPOKM\
    MOA\x12\x20\n\x0bFANNOHCHAMP\x18\x04\x20\x01(\tR\x0bFANNOHCHAMPb\x06prot\
    o3\
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
            messages.push(AHJABMAJJIF::generated_message_descriptor_data());
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
