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

//! Generated file from `ClientDownloadData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClientDownloadData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientDownloadData {
    // message fields
    // @@protoc_insertion_point(field:ClientDownloadData.version)
    pub version: u32,
    // @@protoc_insertion_point(field:ClientDownloadData.time)
    pub time: i64,
    // @@protoc_insertion_point(field:ClientDownloadData.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:ClientDownloadData.NJDDJHAPNBO)
    pub NJDDJHAPNBO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ClientDownloadData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientDownloadData {
    fn default() -> &'a ClientDownloadData {
        <ClientDownloadData as ::protobuf::Message>::default_instance()
    }
}

impl ClientDownloadData {
    pub fn new() -> ClientDownloadData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &ClientDownloadData| { &m.version },
            |m: &mut ClientDownloadData| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &ClientDownloadData| { &m.time },
            |m: &mut ClientDownloadData| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &ClientDownloadData| { &m.data },
            |m: &mut ClientDownloadData| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NJDDJHAPNBO",
            |m: &ClientDownloadData| { &m.NJDDJHAPNBO },
            |m: &mut ClientDownloadData| { &mut m.NJDDJHAPNBO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientDownloadData>(
            "ClientDownloadData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientDownloadData {
    const NAME: &'static str = "ClientDownloadData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.version = is.read_uint32()?;
                },
                16 => {
                    self.time = is.read_int64()?;
                },
                26 => {
                    self.data = is.read_bytes()?;
                },
                32 => {
                    self.NJDDJHAPNBO = is.read_uint32()?;
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
        if self.version != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.version);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.time);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.data);
        }
        if self.NJDDJHAPNBO != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NJDDJHAPNBO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if self.time != 0 {
            os.write_int64(2, self.time)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(3, &self.data)?;
        }
        if self.NJDDJHAPNBO != 0 {
            os.write_uint32(4, self.NJDDJHAPNBO)?;
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

    fn new() -> ClientDownloadData {
        ClientDownloadData::new()
    }

    fn clear(&mut self) {
        self.version = 0;
        self.time = 0;
        self.data.clear();
        self.NJDDJHAPNBO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientDownloadData {
        static instance: ClientDownloadData = ClientDownloadData {
            version: 0,
            time: 0,
            data: ::std::vec::Vec::new(),
            NJDDJHAPNBO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientDownloadData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientDownloadData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientDownloadData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientDownloadData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ClientDownloadData.proto\"x\n\x12ClientDownloadData\x12\x18\n\x07v\
    ersion\x18\x01\x20\x01(\rR\x07version\x12\x12\n\x04time\x18\x02\x20\x01(\
    \x03R\x04time\x12\x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data\x12\x20\n\
    \x0bNJDDJHAPNBO\x18\x04\x20\x01(\rR\x0bNJDDJHAPNBOB\x15\n\x13emu.lunarco\
    re.protob\x06proto3\
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
            messages.push(ClientDownloadData::generated_message_descriptor_data());
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