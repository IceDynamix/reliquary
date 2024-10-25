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

//! Generated file from `CmdActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdActivityType)
pub enum CmdActivityType {
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdActivityTypeNone)
    CmdActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardScRsp)
    CmdTakeTrialActivityRewardScRsp = 2639,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdCurTrialActivityScNotify)
    CmdCurTrialActivityScNotify = 2686,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardScRsp)
    CmdTakeMaterialSubmitActivityRewardScRsp = 2660,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataScRsp)
    CmdGetMaterialSubmitActivityDataScRsp = 2620,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardCsReq)
    CmdTakeMaterialSubmitActivityRewardCsReq = 2604,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigScRsp)
    CmdGetActivityScheduleConfigScRsp = 2677,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityScRsp)
    CmdStartTrialActivityScRsp = 2603,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardCsReq)
    CmdTakeTrialActivityRewardCsReq = 2676,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialCsReq)
    CmdSubmitMaterialSubmitActivityMaterialCsReq = 2650,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityCsReq)
    CmdLeaveTrialActivityCsReq = 2684,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigCsReq)
    CmdGetActivityScheduleConfigCsReq = 2679,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialScRsp)
    CmdSubmitMaterialSubmitActivityMaterialScRsp = 2631,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageCsReq)
    CmdEnterTrialActivityStageCsReq = 2658,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityCsReq)
    CmdStartTrialActivityCsReq = 2649,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardScRsp)
    CmdTakeLoginActivityRewardScRsp = 2642,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTrialActivityDataChangeScNotify)
    CmdTrialActivityDataChangeScNotify = 2669,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataScRsp)
    CmdGetTrialActivityDataScRsp = 2654,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityScRsp)
    CmdLeaveTrialActivityScRsp = 2632,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataCsReq)
    CmdGetTrialActivityDataCsReq = 2687,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityCsReq)
    CmdGetLoginActivityCsReq = 2698,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityScRsp)
    CmdGetLoginActivityScRsp = 2671,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageScRsp)
    CmdEnterTrialActivityStageScRsp = 2667,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataCsReq)
    CmdGetMaterialSubmitActivityDataCsReq = 2645,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardCsReq)
    CmdTakeLoginActivityRewardCsReq = 2683,
}

impl ::protobuf::Enum for CmdActivityType {
    const NAME: &'static str = "CmdActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            2639 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            2686 => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            2660 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            2620 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            2604 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            2677 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            2603 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            2676 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            2650 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            2684 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            2679 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            2631 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            2658 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            2649 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            2642 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            2669 => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            2654 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            2632 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            2687 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            2698 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            2671 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            2667 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            2645 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            2683 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdActivityType> {
        match str {
            "CmdActivityTypeNone" => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            "CmdTakeTrialActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            "CmdCurTrialActivityScNotify" => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            "CmdTakeMaterialSubmitActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            "CmdGetMaterialSubmitActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            "CmdTakeMaterialSubmitActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            "CmdGetActivityScheduleConfigScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            "CmdStartTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            "CmdTakeTrialActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            "CmdSubmitMaterialSubmitActivityMaterialCsReq" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            "CmdLeaveTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            "CmdGetActivityScheduleConfigCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            "CmdSubmitMaterialSubmitActivityMaterialScRsp" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            "CmdEnterTrialActivityStageCsReq" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            "CmdStartTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            "CmdTakeLoginActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            "CmdTrialActivityDataChangeScNotify" => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            "CmdGetTrialActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            "CmdLeaveTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            "CmdGetTrialActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            "CmdGetLoginActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            "CmdGetLoginActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            "CmdEnterTrialActivityStageScRsp" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            "CmdGetMaterialSubmitActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            "CmdTakeLoginActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdActivityType] = &[
        CmdActivityType::CmdActivityTypeNone,
        CmdActivityType::CmdTakeTrialActivityRewardScRsp,
        CmdActivityType::CmdCurTrialActivityScNotify,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp,
        CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq,
        CmdActivityType::CmdGetActivityScheduleConfigScRsp,
        CmdActivityType::CmdStartTrialActivityScRsp,
        CmdActivityType::CmdTakeTrialActivityRewardCsReq,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq,
        CmdActivityType::CmdLeaveTrialActivityCsReq,
        CmdActivityType::CmdGetActivityScheduleConfigCsReq,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp,
        CmdActivityType::CmdEnterTrialActivityStageCsReq,
        CmdActivityType::CmdStartTrialActivityCsReq,
        CmdActivityType::CmdTakeLoginActivityRewardScRsp,
        CmdActivityType::CmdTrialActivityDataChangeScNotify,
        CmdActivityType::CmdGetTrialActivityDataScRsp,
        CmdActivityType::CmdLeaveTrialActivityScRsp,
        CmdActivityType::CmdGetTrialActivityDataCsReq,
        CmdActivityType::CmdGetLoginActivityCsReq,
        CmdActivityType::CmdGetLoginActivityScRsp,
        CmdActivityType::CmdEnterTrialActivityStageScRsp,
        CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq,
        CmdActivityType::CmdTakeLoginActivityRewardCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdActivityType::CmdActivityTypeNone => 0,
            CmdActivityType::CmdTakeTrialActivityRewardScRsp => 1,
            CmdActivityType::CmdCurTrialActivityScNotify => 2,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp => 3,
            CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp => 4,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq => 5,
            CmdActivityType::CmdGetActivityScheduleConfigScRsp => 6,
            CmdActivityType::CmdStartTrialActivityScRsp => 7,
            CmdActivityType::CmdTakeTrialActivityRewardCsReq => 8,
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq => 9,
            CmdActivityType::CmdLeaveTrialActivityCsReq => 10,
            CmdActivityType::CmdGetActivityScheduleConfigCsReq => 11,
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp => 12,
            CmdActivityType::CmdEnterTrialActivityStageCsReq => 13,
            CmdActivityType::CmdStartTrialActivityCsReq => 14,
            CmdActivityType::CmdTakeLoginActivityRewardScRsp => 15,
            CmdActivityType::CmdTrialActivityDataChangeScNotify => 16,
            CmdActivityType::CmdGetTrialActivityDataScRsp => 17,
            CmdActivityType::CmdLeaveTrialActivityScRsp => 18,
            CmdActivityType::CmdGetTrialActivityDataCsReq => 19,
            CmdActivityType::CmdGetLoginActivityCsReq => 20,
            CmdActivityType::CmdGetLoginActivityScRsp => 21,
            CmdActivityType::CmdEnterTrialActivityStageScRsp => 22,
            CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq => 23,
            CmdActivityType::CmdTakeLoginActivityRewardCsReq => 24,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdActivityType {
    fn default() -> Self {
        CmdActivityType::CmdActivityTypeNone
    }
}

impl CmdActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdActivityType>("CmdActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdActivityType.proto*\xcd\x07\n\x0fCmdActivityType\x12\x17\n\x13C\
    mdActivityTypeNone\x10\0\x12$\n\x1fCmdTakeTrialActivityRewardScRsp\x10\
    \xcf\x14\x12\x20\n\x1bCmdCurTrialActivityScNotify\x10\xfe\x14\x12-\n(Cmd\
    TakeMaterialSubmitActivityRewardScRsp\x10\xe4\x14\x12*\n%CmdGetMaterialS\
    ubmitActivityDataScRsp\x10\xbc\x14\x12-\n(CmdTakeMaterialSubmitActivityR\
    ewardCsReq\x10\xac\x14\x12&\n!CmdGetActivityScheduleConfigScRsp\x10\xf5\
    \x14\x12\x1f\n\x1aCmdStartTrialActivityScRsp\x10\xab\x14\x12$\n\x1fCmdTa\
    keTrialActivityRewardCsReq\x10\xf4\x14\x121\n,CmdSubmitMaterialSubmitAct\
    ivityMaterialCsReq\x10\xda\x14\x12\x1f\n\x1aCmdLeaveTrialActivityCsReq\
    \x10\xfc\x14\x12&\n!CmdGetActivityScheduleConfigCsReq\x10\xf7\x14\x121\n\
    ,CmdSubmitMaterialSubmitActivityMaterialScRsp\x10\xc7\x14\x12$\n\x1fCmdE\
    nterTrialActivityStageCsReq\x10\xe2\x14\x12\x1f\n\x1aCmdStartTrialActivi\
    tyCsReq\x10\xd9\x14\x12$\n\x1fCmdTakeLoginActivityRewardScRsp\x10\xd2\
    \x14\x12'\n\"CmdTrialActivityDataChangeScNotify\x10\xed\x14\x12!\n\x1cCm\
    dGetTrialActivityDataScRsp\x10\xde\x14\x12\x1f\n\x1aCmdLeaveTrialActivit\
    yScRsp\x10\xc8\x14\x12!\n\x1cCmdGetTrialActivityDataCsReq\x10\xff\x14\
    \x12\x1d\n\x18CmdGetLoginActivityCsReq\x10\x8a\x15\x12\x1d\n\x18CmdGetLo\
    ginActivityScRsp\x10\xef\x14\x12$\n\x1fCmdEnterTrialActivityStageScRsp\
    \x10\xeb\x14\x12*\n%CmdGetMaterialSubmitActivityDataCsReq\x10\xd5\x14\
    \x12$\n\x1fCmdTakeLoginActivityRewardCsReq\x10\xfb\x14b\x06proto3\
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
            enums.push(CmdActivityType::generated_enum_descriptor_data());
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
