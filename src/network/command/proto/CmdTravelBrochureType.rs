// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `CmdTravelBrochureType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTravelBrochureType)
pub enum CmdTravelBrochureType {
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureNone)
    CmdTravelBrochureNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetCustomValueScRsp)
    CmdTravelBrochureSetCustomValueScRsp = 6498,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureRemovePasterCsReq)
    CmdTravelBrochureRemovePasterCsReq = 6454,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterCsReq)
    CmdTravelBrochureApplyPasterCsReq = 6470,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureUpdatePasterPosScRsp)
    CmdTravelBrochureUpdatePasterPosScRsp = 6402,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSelectMessageCsReq)
    CmdTravelBrochureSelectMessageCsReq = 6427,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetPasterScNotify)
    CmdTravelBrochureGetPasterScNotify = 6405,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureUpdatePasterPosCsReq)
    CmdTravelBrochureUpdatePasterPosCsReq = 6480,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetPageDescStatusCsReq)
    CmdTravelBrochureSetPageDescStatusCsReq = 6485,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterListScRsp)
    CmdTravelBrochureApplyPasterListScRsp = 6436,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureRemovePasterScRsp)
    CmdTravelBrochureRemovePasterScRsp = 6477,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageResetCsReq)
    CmdTravelBrochurePageResetCsReq = 6452,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterScRsp)
    CmdTravelBrochureApplyPasterScRsp = 6459,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetDataScRsp)
    CmdTravelBrochureGetDataScRsp = 6491,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSelectMessageScRsp)
    CmdTravelBrochureSelectMessageScRsp = 6421,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetPageDescStatusScRsp)
    CmdTravelBrochureSetPageDescStatusScRsp = 6438,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureSetCustomValueCsReq)
    CmdTravelBrochureSetCustomValueCsReq = 6473,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureApplyPasterListCsReq)
    CmdTravelBrochureApplyPasterListCsReq = 6494,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageResetScRsp)
    CmdTravelBrochurePageResetScRsp = 6468,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochureGetDataCsReq)
    CmdTravelBrochureGetDataCsReq = 6420,
    // @@protoc_insertion_point(enum_value:CmdTravelBrochureType.CmdTravelBrochurePageUnlockScNotify)
    CmdTravelBrochurePageUnlockScNotify = 6467,
}

impl ::protobuf::Enum for CmdTravelBrochureType {
    const NAME: &'static str = "CmdTravelBrochureType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTravelBrochureType> {
        match value {
            0 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureNone),
            6498 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp),
            6454 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq),
            6470 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq),
            6402 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp),
            6427 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq),
            6405 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify),
            6480 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq),
            6485 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq),
            6436 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp),
            6477 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp),
            6452 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq),
            6459 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp),
            6491 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp),
            6421 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp),
            6438 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp),
            6473 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq),
            6494 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq),
            6468 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp),
            6420 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq),
            6467 => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTravelBrochureType> {
        match str {
            "CmdTravelBrochureNone" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureNone),
            "CmdTravelBrochureSetCustomValueScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp),
            "CmdTravelBrochureRemovePasterCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq),
            "CmdTravelBrochureApplyPasterCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq),
            "CmdTravelBrochureUpdatePasterPosScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp),
            "CmdTravelBrochureSelectMessageCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq),
            "CmdTravelBrochureGetPasterScNotify" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify),
            "CmdTravelBrochureUpdatePasterPosCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq),
            "CmdTravelBrochureSetPageDescStatusCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq),
            "CmdTravelBrochureApplyPasterListScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp),
            "CmdTravelBrochureRemovePasterScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp),
            "CmdTravelBrochurePageResetCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq),
            "CmdTravelBrochureApplyPasterScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp),
            "CmdTravelBrochureGetDataScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp),
            "CmdTravelBrochureSelectMessageScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp),
            "CmdTravelBrochureSetPageDescStatusScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp),
            "CmdTravelBrochureSetCustomValueCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq),
            "CmdTravelBrochureApplyPasterListCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq),
            "CmdTravelBrochurePageResetScRsp" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp),
            "CmdTravelBrochureGetDataCsReq" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq),
            "CmdTravelBrochurePageUnlockScNotify" => ::std::option::Option::Some(CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTravelBrochureType] = &[
        CmdTravelBrochureType::CmdTravelBrochureNone,
        CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp,
        CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq,
        CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq,
        CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify,
        CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq,
        CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp,
        CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp,
        CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp,
        CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp,
        CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq,
        CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq,
        CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp,
        CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq,
        CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdTravelBrochureType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTravelBrochureType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTravelBrochureType::CmdTravelBrochureNone => 0,
            CmdTravelBrochureType::CmdTravelBrochureSetCustomValueScRsp => 1,
            CmdTravelBrochureType::CmdTravelBrochureRemovePasterCsReq => 2,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterCsReq => 3,
            CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosScRsp => 4,
            CmdTravelBrochureType::CmdTravelBrochureSelectMessageCsReq => 5,
            CmdTravelBrochureType::CmdTravelBrochureGetPasterScNotify => 6,
            CmdTravelBrochureType::CmdTravelBrochureUpdatePasterPosCsReq => 7,
            CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusCsReq => 8,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterListScRsp => 9,
            CmdTravelBrochureType::CmdTravelBrochureRemovePasterScRsp => 10,
            CmdTravelBrochureType::CmdTravelBrochurePageResetCsReq => 11,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterScRsp => 12,
            CmdTravelBrochureType::CmdTravelBrochureGetDataScRsp => 13,
            CmdTravelBrochureType::CmdTravelBrochureSelectMessageScRsp => 14,
            CmdTravelBrochureType::CmdTravelBrochureSetPageDescStatusScRsp => 15,
            CmdTravelBrochureType::CmdTravelBrochureSetCustomValueCsReq => 16,
            CmdTravelBrochureType::CmdTravelBrochureApplyPasterListCsReq => 17,
            CmdTravelBrochureType::CmdTravelBrochurePageResetScRsp => 18,
            CmdTravelBrochureType::CmdTravelBrochureGetDataCsReq => 19,
            CmdTravelBrochureType::CmdTravelBrochurePageUnlockScNotify => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTravelBrochureType {
    fn default() -> Self {
        CmdTravelBrochureType::CmdTravelBrochureNone
    }
}

impl CmdTravelBrochureType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTravelBrochureType>("CmdTravelBrochureType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bCmdTravelBrochureType.proto*\xf1\x06\n\x15CmdTravelBrochureType\
    \x12\x19\n\x15CmdTravelBrochureNone\x10\0\x12)\n$CmdTravelBrochureSetCus\
    tomValueScRsp\x10\xe22\x12'\n\"CmdTravelBrochureRemovePasterCsReq\x10\
    \xb62\x12&\n!CmdTravelBrochureApplyPasterCsReq\x10\xc62\x12*\n%CmdTravel\
    BrochureUpdatePasterPosScRsp\x10\x822\x12(\n#CmdTravelBrochureSelectMess\
    ageCsReq\x10\x9b2\x12'\n\"CmdTravelBrochureGetPasterScNotify\x10\x852\
    \x12*\n%CmdTravelBrochureUpdatePasterPosCsReq\x10\xd02\x12,\n'CmdTravelB\
    rochureSetPageDescStatusCsReq\x10\xd52\x12*\n%CmdTravelBrochureApplyPast\
    erListScRsp\x10\xa42\x12'\n\"CmdTravelBrochureRemovePasterScRsp\x10\xcd2\
    \x12$\n\x1fCmdTravelBrochurePageResetCsReq\x10\xb42\x12&\n!CmdTravelBroc\
    hureApplyPasterScRsp\x10\xbb2\x12\"\n\x1dCmdTravelBrochureGetDataScRsp\
    \x10\xdb2\x12(\n#CmdTravelBrochureSelectMessageScRsp\x10\x952\x12,\n'Cmd\
    TravelBrochureSetPageDescStatusScRsp\x10\xa62\x12)\n$CmdTravelBrochureSe\
    tCustomValueCsReq\x10\xc92\x12*\n%CmdTravelBrochureApplyPasterListCsReq\
    \x10\xde2\x12$\n\x1fCmdTravelBrochurePageResetScRsp\x10\xc42\x12\"\n\x1d\
    CmdTravelBrochureGetDataCsReq\x10\x942\x12(\n#CmdTravelBrochurePageUnloc\
    kScNotify\x10\xc32b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(CmdTravelBrochureType::generated_enum_descriptor_data());
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
