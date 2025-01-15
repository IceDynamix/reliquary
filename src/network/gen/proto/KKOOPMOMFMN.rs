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

//! Generated file from `KKOOPMOMFMN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KKOOPMOMFMN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KKOOPMOMFMN {
    // message fields
    // @@protoc_insertion_point(field:KKOOPMOMFMN.IKFOHIMFFIO)
    pub IKFOHIMFFIO: u32,
    // message oneof groups
    pub JGBFPDICMNF: ::std::option::Option<kkoopmomfmn::JGBFPDICMNF>,
    // special fields
    // @@protoc_insertion_point(special_field:KKOOPMOMFMN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KKOOPMOMFMN {
    fn default() -> &'a KKOOPMOMFMN {
        <KKOOPMOMFMN as ::protobuf::Message>::default_instance()
    }
}

impl KKOOPMOMFMN {
    pub fn new() -> KKOOPMOMFMN {
        ::std::default::Default::default()
    }

    // .ItemList IHKKJKHFIHH = 15;

    pub fn IHKKJKHFIHH(&self) -> &super::ItemList::ItemList {
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(ref v)) => v,
            _ => <super::ItemList::ItemList as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IHKKJKHFIHH(&mut self) {
        self.JGBFPDICMNF = ::std::option::Option::None;
    }

    pub fn has_IHKKJKHFIHH(&self) -> bool {
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IHKKJKHFIHH(&mut self, v: super::ItemList::ItemList) {
        self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IHKKJKHFIHH(&mut self) -> &mut super::ItemList::ItemList {
        if let ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(_)) = self.JGBFPDICMNF {
        } else {
            self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(super::ItemList::ItemList::new()));
        }
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IHKKJKHFIHH(&mut self) -> super::ItemList::ItemList {
        if self.has_IHKKJKHFIHH() {
            match self.JGBFPDICMNF.take() {
                ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ItemList::ItemList::new()
        }
    }

    // .DFKHPDONJLJ FNOPMFBBPPO = 4;

    pub fn FNOPMFBBPPO(&self) -> &super::DFKHPDONJLJ::DFKHPDONJLJ {
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(ref v)) => v,
            _ => <super::DFKHPDONJLJ::DFKHPDONJLJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FNOPMFBBPPO(&mut self) {
        self.JGBFPDICMNF = ::std::option::Option::None;
    }

    pub fn has_FNOPMFBBPPO(&self) -> bool {
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FNOPMFBBPPO(&mut self, v: super::DFKHPDONJLJ::DFKHPDONJLJ) {
        self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FNOPMFBBPPO(&mut self) -> &mut super::DFKHPDONJLJ::DFKHPDONJLJ {
        if let ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(_)) = self.JGBFPDICMNF {
        } else {
            self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(super::DFKHPDONJLJ::DFKHPDONJLJ::new()));
        }
        match self.JGBFPDICMNF {
            ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FNOPMFBBPPO(&mut self) -> super::DFKHPDONJLJ::DFKHPDONJLJ {
        if self.has_FNOPMFBBPPO() {
            match self.JGBFPDICMNF.take() {
                ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::DFKHPDONJLJ::DFKHPDONJLJ::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IKFOHIMFFIO",
            |m: &KKOOPMOMFMN| { &m.IKFOHIMFFIO },
            |m: &mut KKOOPMOMFMN| { &mut m.IKFOHIMFFIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ItemList::ItemList>(
            "IHKKJKHFIHH",
            KKOOPMOMFMN::has_IHKKJKHFIHH,
            KKOOPMOMFMN::IHKKJKHFIHH,
            KKOOPMOMFMN::mut_IHKKJKHFIHH,
            KKOOPMOMFMN::set_IHKKJKHFIHH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::DFKHPDONJLJ::DFKHPDONJLJ>(
            "FNOPMFBBPPO",
            KKOOPMOMFMN::has_FNOPMFBBPPO,
            KKOOPMOMFMN::FNOPMFBBPPO,
            KKOOPMOMFMN::mut_FNOPMFBBPPO,
            KKOOPMOMFMN::set_FNOPMFBBPPO,
        ));
        oneofs.push(kkoopmomfmn::JGBFPDICMNF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KKOOPMOMFMN>(
            "KKOOPMOMFMN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KKOOPMOMFMN {
    const NAME: &'static str = "KKOOPMOMFMN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.IKFOHIMFFIO = is.read_uint32()?;
                },
                122 => {
                    self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(is.read_message()?));
                },
                34 => {
                    self.JGBFPDICMNF = ::std::option::Option::Some(kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(is.read_message()?));
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
        if self.IKFOHIMFFIO != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.IKFOHIMFFIO);
        }
        if let ::std::option::Option::Some(ref v) = self.JGBFPDICMNF {
            match v {
                &kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IKFOHIMFFIO != 0 {
            os.write_uint32(9, self.IKFOHIMFFIO)?;
        }
        if let ::std::option::Option::Some(ref v) = self.JGBFPDICMNF {
            match v {
                &kkoopmomfmn::JGBFPDICMNF::IHKKJKHFIHH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &kkoopmomfmn::JGBFPDICMNF::FNOPMFBBPPO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
            };
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

    fn new() -> KKOOPMOMFMN {
        KKOOPMOMFMN::new()
    }

    fn clear(&mut self) {
        self.IKFOHIMFFIO = 0;
        self.JGBFPDICMNF = ::std::option::Option::None;
        self.JGBFPDICMNF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KKOOPMOMFMN {
        static instance: KKOOPMOMFMN = KKOOPMOMFMN {
            IKFOHIMFFIO: 0,
            JGBFPDICMNF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KKOOPMOMFMN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KKOOPMOMFMN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KKOOPMOMFMN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KKOOPMOMFMN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `KKOOPMOMFMN`
pub mod kkoopmomfmn {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:KKOOPMOMFMN.JGBFPDICMNF)
    pub enum JGBFPDICMNF {
        // @@protoc_insertion_point(oneof_field:KKOOPMOMFMN.IHKKJKHFIHH)
        IHKKJKHFIHH(super::super::ItemList::ItemList),
        // @@protoc_insertion_point(oneof_field:KKOOPMOMFMN.FNOPMFBBPPO)
        FNOPMFBBPPO(super::super::DFKHPDONJLJ::DFKHPDONJLJ),
    }

    impl ::protobuf::Oneof for JGBFPDICMNF {
    }

    impl ::protobuf::OneofFull for JGBFPDICMNF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::KKOOPMOMFMN as ::protobuf::MessageFull>::descriptor().oneof_by_name("JGBFPDICMNF").unwrap()).clone()
        }
    }

    impl JGBFPDICMNF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<JGBFPDICMNF>("JGBFPDICMNF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KKOOPMOMFMN.proto\x1a\x11DFKHPDONJLJ.proto\x1a\x0eItemList.proto\"\
    \x9f\x01\n\x0bKKOOPMOMFMN\x12\x20\n\x0bIKFOHIMFFIO\x18\t\x20\x01(\rR\x0b\
    IKFOHIMFFIO\x12-\n\x0bIHKKJKHFIHH\x18\x0f\x20\x01(\x0b2\t.ItemListH\0R\
    \x0bIHKKJKHFIHH\x120\n\x0bFNOPMFBBPPO\x18\x04\x20\x01(\x0b2\x0c.DFKHPDON\
    JLJH\0R\x0bFNOPMFBBPPOB\r\n\x0bJGBFPDICMNFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::DFKHPDONJLJ::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KKOOPMOMFMN::generated_message_descriptor_data());
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