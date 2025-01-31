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

//! Generated file from `ClockParkUseBuffScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClockParkUseBuffScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClockParkUseBuffScRsp {
    // message fields
    // @@protoc_insertion_point(field:ClockParkUseBuffScRsp.EMCJNOFILKP)
    pub EMCJNOFILKP: u32,
    // @@protoc_insertion_point(field:ClockParkUseBuffScRsp.GOHGIEMLNOM)
    pub GOHGIEMLNOM: u32,
    // @@protoc_insertion_point(field:ClockParkUseBuffScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ClockParkUseBuffScRsp.OBNMBPEKKIN)
    pub OBNMBPEKKIN: ::protobuf::MessageField<super::PNKLPCLEOFC::PNKLPCLEOFC>,
    // message oneof groups
    pub NBKFFJJGCPC: ::std::option::Option<clock_park_use_buff_sc_rsp::NBKFFJJGCPC>,
    // special fields
    // @@protoc_insertion_point(special_field:ClockParkUseBuffScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClockParkUseBuffScRsp {
    fn default() -> &'a ClockParkUseBuffScRsp {
        <ClockParkUseBuffScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ClockParkUseBuffScRsp {
    pub fn new() -> ClockParkUseBuffScRsp {
        ::std::default::Default::default()
    }

    // .GDFGFDNCFDG LKNAGOKLOGI = 324;

    pub fn LKNAGOKLOGI(&self) -> &super::GDFGFDNCFDG::GDFGFDNCFDG {
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(ref v)) => v,
            _ => <super::GDFGFDNCFDG::GDFGFDNCFDG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LKNAGOKLOGI(&mut self) {
        self.NBKFFJJGCPC = ::std::option::Option::None;
    }

    pub fn has_LKNAGOKLOGI(&self) -> bool {
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LKNAGOKLOGI(&mut self, v: super::GDFGFDNCFDG::GDFGFDNCFDG) {
        self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LKNAGOKLOGI(&mut self) -> &mut super::GDFGFDNCFDG::GDFGFDNCFDG {
        if let ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(_)) = self.NBKFFJJGCPC {
        } else {
            self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(super::GDFGFDNCFDG::GDFGFDNCFDG::new()));
        }
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LKNAGOKLOGI(&mut self) -> super::GDFGFDNCFDG::GDFGFDNCFDG {
        if self.has_LKNAGOKLOGI() {
            match self.NBKFFJJGCPC.take() {
                ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GDFGFDNCFDG::GDFGFDNCFDG::new()
        }
    }

    // .EOIAPEFEDGE GCPAFPNDPDA = 99;

    pub fn GCPAFPNDPDA(&self) -> &super::EOIAPEFEDGE::EOIAPEFEDGE {
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(ref v)) => v,
            _ => <super::EOIAPEFEDGE::EOIAPEFEDGE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GCPAFPNDPDA(&mut self) {
        self.NBKFFJJGCPC = ::std::option::Option::None;
    }

    pub fn has_GCPAFPNDPDA(&self) -> bool {
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GCPAFPNDPDA(&mut self, v: super::EOIAPEFEDGE::EOIAPEFEDGE) {
        self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GCPAFPNDPDA(&mut self) -> &mut super::EOIAPEFEDGE::EOIAPEFEDGE {
        if let ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(_)) = self.NBKFFJJGCPC {
        } else {
            self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(super::EOIAPEFEDGE::EOIAPEFEDGE::new()));
        }
        match self.NBKFFJJGCPC {
            ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GCPAFPNDPDA(&mut self) -> super::EOIAPEFEDGE::EOIAPEFEDGE {
        if self.has_GCPAFPNDPDA() {
            match self.NBKFFJJGCPC.take() {
                ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::EOIAPEFEDGE::EOIAPEFEDGE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EMCJNOFILKP",
            |m: &ClockParkUseBuffScRsp| { &m.EMCJNOFILKP },
            |m: &mut ClockParkUseBuffScRsp| { &mut m.EMCJNOFILKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GOHGIEMLNOM",
            |m: &ClockParkUseBuffScRsp| { &m.GOHGIEMLNOM },
            |m: &mut ClockParkUseBuffScRsp| { &mut m.GOHGIEMLNOM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ClockParkUseBuffScRsp| { &m.retcode },
            |m: &mut ClockParkUseBuffScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PNKLPCLEOFC::PNKLPCLEOFC>(
            "OBNMBPEKKIN",
            |m: &ClockParkUseBuffScRsp| { &m.OBNMBPEKKIN },
            |m: &mut ClockParkUseBuffScRsp| { &mut m.OBNMBPEKKIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GDFGFDNCFDG::GDFGFDNCFDG>(
            "LKNAGOKLOGI",
            ClockParkUseBuffScRsp::has_LKNAGOKLOGI,
            ClockParkUseBuffScRsp::LKNAGOKLOGI,
            ClockParkUseBuffScRsp::mut_LKNAGOKLOGI,
            ClockParkUseBuffScRsp::set_LKNAGOKLOGI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::EOIAPEFEDGE::EOIAPEFEDGE>(
            "GCPAFPNDPDA",
            ClockParkUseBuffScRsp::has_GCPAFPNDPDA,
            ClockParkUseBuffScRsp::GCPAFPNDPDA,
            ClockParkUseBuffScRsp::mut_GCPAFPNDPDA,
            ClockParkUseBuffScRsp::set_GCPAFPNDPDA,
        ));
        oneofs.push(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClockParkUseBuffScRsp>(
            "ClockParkUseBuffScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClockParkUseBuffScRsp {
    const NAME: &'static str = "ClockParkUseBuffScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.EMCJNOFILKP = is.read_uint32()?;
                },
                8 => {
                    self.GOHGIEMLNOM = is.read_uint32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OBNMBPEKKIN)?;
                },
                2594 => {
                    self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(is.read_message()?));
                },
                794 => {
                    self.NBKFFJJGCPC = ::std::option::Option::Some(clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(is.read_message()?));
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
        if self.EMCJNOFILKP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.EMCJNOFILKP);
        }
        if self.GOHGIEMLNOM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GOHGIEMLNOM);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.NBKFFJJGCPC {
            match v {
                &clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EMCJNOFILKP != 0 {
            os.write_uint32(3, self.EMCJNOFILKP)?;
        }
        if self.GOHGIEMLNOM != 0 {
            os.write_uint32(1, self.GOHGIEMLNOM)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if let Some(v) = self.OBNMBPEKKIN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.NBKFFJJGCPC {
            match v {
                &clock_park_use_buff_sc_rsp::NBKFFJJGCPC::LKNAGOKLOGI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(324, v, os)?;
                },
                &clock_park_use_buff_sc_rsp::NBKFFJJGCPC::GCPAFPNDPDA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(99, v, os)?;
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

    fn new() -> ClockParkUseBuffScRsp {
        ClockParkUseBuffScRsp::new()
    }

    fn clear(&mut self) {
        self.EMCJNOFILKP = 0;
        self.GOHGIEMLNOM = 0;
        self.retcode = 0;
        self.OBNMBPEKKIN.clear();
        self.NBKFFJJGCPC = ::std::option::Option::None;
        self.NBKFFJJGCPC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClockParkUseBuffScRsp {
        static instance: ClockParkUseBuffScRsp = ClockParkUseBuffScRsp {
            EMCJNOFILKP: 0,
            GOHGIEMLNOM: 0,
            retcode: 0,
            OBNMBPEKKIN: ::protobuf::MessageField::none(),
            NBKFFJJGCPC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClockParkUseBuffScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClockParkUseBuffScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClockParkUseBuffScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClockParkUseBuffScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ClockParkUseBuffScRsp`
pub mod clock_park_use_buff_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:ClockParkUseBuffScRsp.NBKFFJJGCPC)
    pub enum NBKFFJJGCPC {
        // @@protoc_insertion_point(oneof_field:ClockParkUseBuffScRsp.LKNAGOKLOGI)
        LKNAGOKLOGI(super::super::GDFGFDNCFDG::GDFGFDNCFDG),
        // @@protoc_insertion_point(oneof_field:ClockParkUseBuffScRsp.GCPAFPNDPDA)
        GCPAFPNDPDA(super::super::EOIAPEFEDGE::EOIAPEFEDGE),
    }

    impl ::protobuf::Oneof for NBKFFJJGCPC {
    }

    impl ::protobuf::OneofFull for NBKFFJJGCPC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::ClockParkUseBuffScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("NBKFFJJGCPC").unwrap()).clone()
        }
    }

    impl NBKFFJJGCPC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NBKFFJJGCPC>("NBKFFJJGCPC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bClockParkUseBuffScRsp.proto\x1a\x11EOIAPEFEDGE.proto\x1a\x11GDFGFD\
    NCFDG.proto\x1a\x11PNKLPCLEOFC.proto\"\x99\x02\n\x15ClockParkUseBuffScRs\
    p\x12\x20\n\x0bEMCJNOFILKP\x18\x03\x20\x01(\rR\x0bEMCJNOFILKP\x12\x20\n\
    \x0bGOHGIEMLNOM\x18\x01\x20\x01(\rR\x0bGOHGIEMLNOM\x12\x18\n\x07retcode\
    \x18\x08\x20\x01(\rR\x07retcode\x12.\n\x0bOBNMBPEKKIN\x18\x0f\x20\x01(\
    \x0b2\x0c.PNKLPCLEOFCR\x0bOBNMBPEKKIN\x121\n\x0bLKNAGOKLOGI\x18\xc4\x02\
    \x20\x01(\x0b2\x0c.GDFGFDNCFDGH\0R\x0bLKNAGOKLOGI\x120\n\x0bGCPAFPNDPDA\
    \x18c\x20\x01(\x0b2\x0c.EOIAPEFEDGEH\0R\x0bGCPAFPNDPDAB\r\n\x0bNBKFFJJGC\
    PCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EOIAPEFEDGE::file_descriptor().clone());
            deps.push(super::GDFGFDNCFDG::file_descriptor().clone());
            deps.push(super::PNKLPCLEOFC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClockParkUseBuffScRsp::generated_message_descriptor_data());
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
