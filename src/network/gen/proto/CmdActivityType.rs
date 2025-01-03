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
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialScRsp)
    CmdSubmitMaterialSubmitActivityMaterialScRsp = 2625,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardScRsp)
    CmdTakeMaterialSubmitActivityRewardScRsp = 2633,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigCsReq)
    CmdGetActivityScheduleConfigCsReq = 2639,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityCsReq)
    CmdLeaveTrialActivityCsReq = 2682,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityCsReq)
    CmdGetLoginActivityCsReq = 2659,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardScRsp)
    CmdTakeLoginActivityRewardScRsp = 2646,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialCsReq)
    CmdSubmitMaterialSubmitActivityMaterialCsReq = 2661,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageCsReq)
    CmdEnterTrialActivityStageCsReq = 2612,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTrialActivityDataChangeScNotify)
    CmdTrialActivityDataChangeScNotify = 2624,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataScRsp)
    CmdGetMaterialSubmitActivityDataScRsp = 2619,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityScRsp)
    CmdGetLoginActivityScRsp = 2620,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageScRsp)
    CmdEnterTrialActivityStageScRsp = 2660,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdCurTrialActivityScNotify)
    CmdCurTrialActivityScNotify = 2684,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataCsReq)
    CmdGetMaterialSubmitActivityDataCsReq = 2679,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigScRsp)
    CmdGetActivityScheduleConfigScRsp = 2653,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardCsReq)
    CmdTakeMaterialSubmitActivityRewardCsReq = 2629,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityScRsp)
    CmdLeaveTrialActivityScRsp = 2662,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityCsReq)
    CmdStartTrialActivityCsReq = 2631,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityScRsp)
    CmdStartTrialActivityScRsp = 2671,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataScRsp)
    CmdGetTrialActivityDataScRsp = 2672,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataCsReq)
    CmdGetTrialActivityDataCsReq = 2678,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardScRsp)
    CmdTakeTrialActivityRewardScRsp = 2696,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardCsReq)
    CmdTakeTrialActivityRewardCsReq = 2632,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardCsReq)
    CmdTakeLoginActivityRewardCsReq = 2603,
}

impl ::protobuf::Enum for CmdActivityType {
    const NAME: &'static str = "CmdActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            2625 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            2633 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            2639 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            2682 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            2659 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            2646 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            2661 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            2612 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            2624 => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            2619 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            2620 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            2660 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            2684 => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            2679 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            2653 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            2629 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            2662 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            2631 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            2671 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            2672 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            2678 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            2696 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            2632 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            2603 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdActivityType> {
        match str {
            "CmdActivityTypeNone" => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            "CmdSubmitMaterialSubmitActivityMaterialScRsp" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            "CmdTakeMaterialSubmitActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            "CmdGetActivityScheduleConfigCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            "CmdLeaveTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            "CmdGetLoginActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            "CmdTakeLoginActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            "CmdSubmitMaterialSubmitActivityMaterialCsReq" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            "CmdEnterTrialActivityStageCsReq" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            "CmdTrialActivityDataChangeScNotify" => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            "CmdGetMaterialSubmitActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            "CmdGetLoginActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            "CmdEnterTrialActivityStageScRsp" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            "CmdCurTrialActivityScNotify" => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            "CmdGetMaterialSubmitActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            "CmdGetActivityScheduleConfigScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            "CmdTakeMaterialSubmitActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            "CmdLeaveTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            "CmdStartTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            "CmdStartTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            "CmdGetTrialActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            "CmdGetTrialActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            "CmdTakeTrialActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            "CmdTakeTrialActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            "CmdTakeLoginActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdActivityType] = &[
        CmdActivityType::CmdActivityTypeNone,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp,
        CmdActivityType::CmdGetActivityScheduleConfigCsReq,
        CmdActivityType::CmdLeaveTrialActivityCsReq,
        CmdActivityType::CmdGetLoginActivityCsReq,
        CmdActivityType::CmdTakeLoginActivityRewardScRsp,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq,
        CmdActivityType::CmdEnterTrialActivityStageCsReq,
        CmdActivityType::CmdTrialActivityDataChangeScNotify,
        CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp,
        CmdActivityType::CmdGetLoginActivityScRsp,
        CmdActivityType::CmdEnterTrialActivityStageScRsp,
        CmdActivityType::CmdCurTrialActivityScNotify,
        CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq,
        CmdActivityType::CmdGetActivityScheduleConfigScRsp,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq,
        CmdActivityType::CmdLeaveTrialActivityScRsp,
        CmdActivityType::CmdStartTrialActivityCsReq,
        CmdActivityType::CmdStartTrialActivityScRsp,
        CmdActivityType::CmdGetTrialActivityDataScRsp,
        CmdActivityType::CmdGetTrialActivityDataCsReq,
        CmdActivityType::CmdTakeTrialActivityRewardScRsp,
        CmdActivityType::CmdTakeTrialActivityRewardCsReq,
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
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp => 1,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp => 2,
            CmdActivityType::CmdGetActivityScheduleConfigCsReq => 3,
            CmdActivityType::CmdLeaveTrialActivityCsReq => 4,
            CmdActivityType::CmdGetLoginActivityCsReq => 5,
            CmdActivityType::CmdTakeLoginActivityRewardScRsp => 6,
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq => 7,
            CmdActivityType::CmdEnterTrialActivityStageCsReq => 8,
            CmdActivityType::CmdTrialActivityDataChangeScNotify => 9,
            CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp => 10,
            CmdActivityType::CmdGetLoginActivityScRsp => 11,
            CmdActivityType::CmdEnterTrialActivityStageScRsp => 12,
            CmdActivityType::CmdCurTrialActivityScNotify => 13,
            CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq => 14,
            CmdActivityType::CmdGetActivityScheduleConfigScRsp => 15,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq => 16,
            CmdActivityType::CmdLeaveTrialActivityScRsp => 17,
            CmdActivityType::CmdStartTrialActivityCsReq => 18,
            CmdActivityType::CmdStartTrialActivityScRsp => 19,
            CmdActivityType::CmdGetTrialActivityDataScRsp => 20,
            CmdActivityType::CmdGetTrialActivityDataCsReq => 21,
            CmdActivityType::CmdTakeTrialActivityRewardScRsp => 22,
            CmdActivityType::CmdTakeTrialActivityRewardCsReq => 23,
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
    mdActivityTypeNone\x10\0\x121\n,CmdSubmitMaterialSubmitActivityMaterialS\
    cRsp\x10\xc1\x14\x12-\n(CmdTakeMaterialSubmitActivityRewardScRsp\x10\xc9\
    \x14\x12&\n!CmdGetActivityScheduleConfigCsReq\x10\xcf\x14\x12\x1f\n\x1aC\
    mdLeaveTrialActivityCsReq\x10\xfa\x14\x12\x1d\n\x18CmdGetLoginActivityCs\
    Req\x10\xe3\x14\x12$\n\x1fCmdTakeLoginActivityRewardScRsp\x10\xd6\x14\
    \x121\n,CmdSubmitMaterialSubmitActivityMaterialCsReq\x10\xe5\x14\x12$\n\
    \x1fCmdEnterTrialActivityStageCsReq\x10\xb4\x14\x12'\n\"CmdTrialActivity\
    DataChangeScNotify\x10\xc0\x14\x12*\n%CmdGetMaterialSubmitActivityDataSc\
    Rsp\x10\xbb\x14\x12\x1d\n\x18CmdGetLoginActivityScRsp\x10\xbc\x14\x12$\n\
    \x1fCmdEnterTrialActivityStageScRsp\x10\xe4\x14\x12\x20\n\x1bCmdCurTrial\
    ActivityScNotify\x10\xfc\x14\x12*\n%CmdGetMaterialSubmitActivityDataCsRe\
    q\x10\xf7\x14\x12&\n!CmdGetActivityScheduleConfigScRsp\x10\xdd\x14\x12-\
    \n(CmdTakeMaterialSubmitActivityRewardCsReq\x10\xc5\x14\x12\x1f\n\x1aCmd\
    LeaveTrialActivityScRsp\x10\xe6\x14\x12\x1f\n\x1aCmdStartTrialActivityCs\
    Req\x10\xc7\x14\x12\x1f\n\x1aCmdStartTrialActivityScRsp\x10\xef\x14\x12!\
    \n\x1cCmdGetTrialActivityDataScRsp\x10\xf0\x14\x12!\n\x1cCmdGetTrialActi\
    vityDataCsReq\x10\xf6\x14\x12$\n\x1fCmdTakeTrialActivityRewardScRsp\x10\
    \x88\x15\x12$\n\x1fCmdTakeTrialActivityRewardCsReq\x10\xc8\x14\x12$\n\
    \x1fCmdTakeLoginActivityRewardCsReq\x10\xab\x14b\x06proto3\
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
