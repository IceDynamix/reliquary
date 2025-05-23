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

//! Generated file from `DCDNIAJCEHN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:DCDNIAJCEHN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DCDNIAJCEHN {
    // message fields
    // @@protoc_insertion_point(field:DCDNIAJCEHN.HDBLELEBKHO)
    pub HDBLELEBKHO: i32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.buff_id)
    pub buff_id: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.time)
    pub time: f32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.KAMIHNEJMFG)
    pub KAMIHNEJMFG: ::protobuf::EnumOrUnknown<super::LKKAJCACIJI::LKKAJCACIJI>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.CIPICLLLIJH)
    pub CIPICLLLIJH: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.JLCIKBLNENH)
    pub JLCIKBLNENH: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.JIJHAAIHNCN)
    pub JIJHAAIHNCN: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.DKPNENBHELH)
    pub DKPNENBHELH: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.text_id)
    pub text_id: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.DJOADECJPOB)
    pub DJOADECJPOB: ::protobuf::EnumOrUnknown<super::PAJNHIAGODD::PAJNHIAGODD>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.MNBEMGNNFOD)
    pub MNBEMGNNFOD: f32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.TURN_FOOD_SWITCH_ATTACK)
    pub TURN_FOOD_SWITCH_ATTACK: i32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.FNIHJJJGOEE)
    pub FNIHJJJGOEE: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.IDABOFPKOKN)
    pub IDABOFPKOKN: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.JLJIGEPLPMH)
    pub JLJIGEPLPMH: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.IAAGGMKGODC)
    pub IAAGGMKGODC: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.level)
    pub level: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.PLFKOCCDBAG)
    pub PLFKOCCDBAG: bool,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.CCLMFABDENA)
    pub CCLMFABDENA: ::protobuf::EnumOrUnknown<super::FightMarbleHpChangeType::FightMarbleHpChangeType>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.GODNAALNOKL)
    pub GODNAALNOKL: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.id)
    pub id: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.LKEFOLCGFGD)
    pub LKEFOLCGFGD: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.NBKELCHILGG)
    pub NBKELCHILGG: bool,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.hp)
    pub hp: i32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.skill_id)
    pub skill_id: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.max_hp)
    pub max_hp: i32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.FDNDMHJOHMO)
    pub FDNDMHJOHMO: ::protobuf::MessageField<super::PFGAIEBGHCP::PFGAIEBGHCP>,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.PNLDLMNKJMK)
    pub PNLDLMNKJMK: u32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.GGBFKENAHOE)
    pub GGBFKENAHOE: f32,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.DMBBMFFEJGI)
    pub DMBBMFFEJGI: bool,
    // @@protoc_insertion_point(field:DCDNIAJCEHN.DHELBCIMLGA)
    pub DHELBCIMLGA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DCDNIAJCEHN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DCDNIAJCEHN {
    fn default() -> &'a DCDNIAJCEHN {
        <DCDNIAJCEHN as ::protobuf::Message>::default_instance()
    }
}

impl DCDNIAJCEHN {
    pub fn new() -> DCDNIAJCEHN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(31);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDBLELEBKHO",
            |m: &DCDNIAJCEHN| { &m.HDBLELEBKHO },
            |m: &mut DCDNIAJCEHN| { &mut m.HDBLELEBKHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buff_id",
            |m: &DCDNIAJCEHN| { &m.buff_id },
            |m: &mut DCDNIAJCEHN| { &mut m.buff_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &DCDNIAJCEHN| { &m.time },
            |m: &mut DCDNIAJCEHN| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KAMIHNEJMFG",
            |m: &DCDNIAJCEHN| { &m.KAMIHNEJMFG },
            |m: &mut DCDNIAJCEHN| { &mut m.KAMIHNEJMFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CIPICLLLIJH",
            |m: &DCDNIAJCEHN| { &m.CIPICLLLIJH },
            |m: &mut DCDNIAJCEHN| { &mut m.CIPICLLLIJH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLCIKBLNENH",
            |m: &DCDNIAJCEHN| { &m.JLCIKBLNENH },
            |m: &mut DCDNIAJCEHN| { &mut m.JLCIKBLNENH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JIJHAAIHNCN",
            |m: &DCDNIAJCEHN| { &m.JIJHAAIHNCN },
            |m: &mut DCDNIAJCEHN| { &mut m.JIJHAAIHNCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKPNENBHELH",
            |m: &DCDNIAJCEHN| { &m.DKPNENBHELH },
            |m: &mut DCDNIAJCEHN| { &mut m.DKPNENBHELH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "text_id",
            |m: &DCDNIAJCEHN| { &m.text_id },
            |m: &mut DCDNIAJCEHN| { &mut m.text_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DJOADECJPOB",
            |m: &DCDNIAJCEHN| { &m.DJOADECJPOB },
            |m: &mut DCDNIAJCEHN| { &mut m.DJOADECJPOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNBEMGNNFOD",
            |m: &DCDNIAJCEHN| { &m.MNBEMGNNFOD },
            |m: &mut DCDNIAJCEHN| { &mut m.MNBEMGNNFOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "TURN_FOOD_SWITCH_ATTACK",
            |m: &DCDNIAJCEHN| { &m.TURN_FOOD_SWITCH_ATTACK },
            |m: &mut DCDNIAJCEHN| { &mut m.TURN_FOOD_SWITCH_ATTACK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "FNIHJJJGOEE",
            |m: &DCDNIAJCEHN| { &m.FNIHJJJGOEE },
            |m: &mut DCDNIAJCEHN| { &mut m.FNIHJJJGOEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDABOFPKOKN",
            |m: &DCDNIAJCEHN| { &m.IDABOFPKOKN },
            |m: &mut DCDNIAJCEHN| { &mut m.IDABOFPKOKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JLJIGEPLPMH",
            |m: &DCDNIAJCEHN| { &m.JLJIGEPLPMH },
            |m: &mut DCDNIAJCEHN| { &mut m.JLJIGEPLPMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "IAAGGMKGODC",
            |m: &DCDNIAJCEHN| { &m.IAAGGMKGODC },
            |m: &mut DCDNIAJCEHN| { &mut m.IAAGGMKGODC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &DCDNIAJCEHN| { &m.level },
            |m: &mut DCDNIAJCEHN| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLFKOCCDBAG",
            |m: &DCDNIAJCEHN| { &m.PLFKOCCDBAG },
            |m: &mut DCDNIAJCEHN| { &mut m.PLFKOCCDBAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCLMFABDENA",
            |m: &DCDNIAJCEHN| { &m.CCLMFABDENA },
            |m: &mut DCDNIAJCEHN| { &mut m.CCLMFABDENA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "GODNAALNOKL",
            |m: &DCDNIAJCEHN| { &m.GODNAALNOKL },
            |m: &mut DCDNIAJCEHN| { &mut m.GODNAALNOKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &DCDNIAJCEHN| { &m.id },
            |m: &mut DCDNIAJCEHN| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "LKEFOLCGFGD",
            |m: &DCDNIAJCEHN| { &m.LKEFOLCGFGD },
            |m: &mut DCDNIAJCEHN| { &mut m.LKEFOLCGFGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBKELCHILGG",
            |m: &DCDNIAJCEHN| { &m.NBKELCHILGG },
            |m: &mut DCDNIAJCEHN| { &mut m.NBKELCHILGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hp",
            |m: &DCDNIAJCEHN| { &m.hp },
            |m: &mut DCDNIAJCEHN| { &mut m.hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_id",
            |m: &DCDNIAJCEHN| { &m.skill_id },
            |m: &mut DCDNIAJCEHN| { &mut m.skill_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_hp",
            |m: &DCDNIAJCEHN| { &m.max_hp },
            |m: &mut DCDNIAJCEHN| { &mut m.max_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PFGAIEBGHCP::PFGAIEBGHCP>(
            "FDNDMHJOHMO",
            |m: &DCDNIAJCEHN| { &m.FDNDMHJOHMO },
            |m: &mut DCDNIAJCEHN| { &mut m.FDNDMHJOHMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNLDLMNKJMK",
            |m: &DCDNIAJCEHN| { &m.PNLDLMNKJMK },
            |m: &mut DCDNIAJCEHN| { &mut m.PNLDLMNKJMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGBFKENAHOE",
            |m: &DCDNIAJCEHN| { &m.GGBFKENAHOE },
            |m: &mut DCDNIAJCEHN| { &mut m.GGBFKENAHOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMBBMFFEJGI",
            |m: &DCDNIAJCEHN| { &m.DMBBMFFEJGI },
            |m: &mut DCDNIAJCEHN| { &mut m.DMBBMFFEJGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHELBCIMLGA",
            |m: &DCDNIAJCEHN| { &m.DHELBCIMLGA },
            |m: &mut DCDNIAJCEHN| { &mut m.DHELBCIMLGA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DCDNIAJCEHN>(
            "DCDNIAJCEHN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DCDNIAJCEHN {
    const NAME: &'static str = "DCDNIAJCEHN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                4616 => {
                    self.HDBLELEBKHO = is.read_int32()?;
                },
                3512 => {
                    self.buff_id = is.read_uint32()?;
                },
                53 => {
                    self.time = is.read_float()?;
                },
                96 => {
                    self.KAMIHNEJMFG = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.CIPICLLLIJH = is.read_uint32()?;
                },
                6216 => {
                    self.JLCIKBLNENH = is.read_uint32()?;
                },
                11208 => {
                    self.JIJHAAIHNCN = is.read_uint32()?;
                },
                4048 => {
                    self.DKPNENBHELH = is.read_uint32()?;
                },
                15072 => {
                    self.text_id = is.read_uint32()?;
                },
                24 => {
                    self.DJOADECJPOB = is.read_enum_or_unknown()?;
                },
                5389 => {
                    self.MNBEMGNNFOD = is.read_float()?;
                },
                88 => {
                    self.TURN_FOOD_SWITCH_ATTACK = is.read_int32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FNIHJJJGOEE)?;
                },
                10024 => {
                    self.IDABOFPKOKN = is.read_uint32()?;
                },
                8 => {
                    self.JLJIGEPLPMH = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IAAGGMKGODC)?;
                },
                6896 => {
                    self.level = is.read_uint32()?;
                },
                16040 => {
                    self.PLFKOCCDBAG = is.read_bool()?;
                },
                11336 => {
                    self.CCLMFABDENA = is.read_enum_or_unknown()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GODNAALNOKL)?;
                },
                80 => {
                    self.id = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LKEFOLCGFGD)?;
                },
                3192 => {
                    self.NBKELCHILGG = is.read_bool()?;
                },
                64 => {
                    self.hp = is.read_int32()?;
                },
                72 => {
                    self.skill_id = is.read_uint32()?;
                },
                32 => {
                    self.max_hp = is.read_int32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FDNDMHJOHMO)?;
                },
                9808 => {
                    self.PNLDLMNKJMK = is.read_uint32()?;
                },
                7261 => {
                    self.GGBFKENAHOE = is.read_float()?;
                },
                14768 => {
                    self.DMBBMFFEJGI = is.read_bool()?;
                },
                7432 => {
                    self.DHELBCIMLGA = is.read_uint32()?;
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
        if self.HDBLELEBKHO != 0 {
            my_size += ::protobuf::rt::int32_size(577, self.HDBLELEBKHO);
        }
        if self.buff_id != 0 {
            my_size += ::protobuf::rt::uint32_size(439, self.buff_id);
        }
        if self.time != 0. {
            my_size += 1 + 4;
        }
        if self.KAMIHNEJMFG != ::protobuf::EnumOrUnknown::new(super::LKKAJCACIJI::LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.KAMIHNEJMFG.value());
        }
        if self.CIPICLLLIJH != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.CIPICLLLIJH);
        }
        if self.JLCIKBLNENH != 0 {
            my_size += ::protobuf::rt::uint32_size(777, self.JLCIKBLNENH);
        }
        if self.JIJHAAIHNCN != 0 {
            my_size += ::protobuf::rt::uint32_size(1401, self.JIJHAAIHNCN);
        }
        if self.DKPNENBHELH != 0 {
            my_size += ::protobuf::rt::uint32_size(506, self.DKPNENBHELH);
        }
        if self.text_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1884, self.text_id);
        }
        if self.DJOADECJPOB != ::protobuf::EnumOrUnknown::new(super::PAJNHIAGODD::PAJNHIAGODD::MARBLE_FRAME_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.DJOADECJPOB.value());
        }
        if self.MNBEMGNNFOD != 0. {
            my_size += 2 + 4;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.TURN_FOOD_SWITCH_ATTACK);
        }
        if let Some(v) = self.FNIHJJJGOEE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IDABOFPKOKN != 0 {
            my_size += ::protobuf::rt::uint32_size(1253, self.IDABOFPKOKN);
        }
        if self.JLJIGEPLPMH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.JLJIGEPLPMH);
        }
        if let Some(v) = self.IAAGGMKGODC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(862, self.level);
        }
        if self.PLFKOCCDBAG != false {
            my_size += 2 + 1;
        }
        if self.CCLMFABDENA != ::protobuf::EnumOrUnknown::new(super::FightMarbleHpChangeType::FightMarbleHpChangeType::MARBLE_HP_CHANGE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(1417, self.CCLMFABDENA.value());
        }
        if let Some(v) = self.GODNAALNOKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.id);
        }
        if let Some(v) = self.LKEFOLCGFGD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NBKELCHILGG != false {
            my_size += 2 + 1;
        }
        if self.hp != 0 {
            my_size += ::protobuf::rt::int32_size(8, self.hp);
        }
        if self.skill_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.skill_id);
        }
        if self.max_hp != 0 {
            my_size += ::protobuf::rt::int32_size(4, self.max_hp);
        }
        if let Some(v) = self.FDNDMHJOHMO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PNLDLMNKJMK != 0 {
            my_size += ::protobuf::rt::uint32_size(1226, self.PNLDLMNKJMK);
        }
        if self.GGBFKENAHOE != 0. {
            my_size += 2 + 4;
        }
        if self.DMBBMFFEJGI != false {
            my_size += 2 + 1;
        }
        if self.DHELBCIMLGA != 0 {
            my_size += ::protobuf::rt::uint32_size(929, self.DHELBCIMLGA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HDBLELEBKHO != 0 {
            os.write_int32(577, self.HDBLELEBKHO)?;
        }
        if self.buff_id != 0 {
            os.write_uint32(439, self.buff_id)?;
        }
        if self.time != 0. {
            os.write_float(6, self.time)?;
        }
        if self.KAMIHNEJMFG != ::protobuf::EnumOrUnknown::new(super::LKKAJCACIJI::LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.KAMIHNEJMFG))?;
        }
        if self.CIPICLLLIJH != 0 {
            os.write_uint32(14, self.CIPICLLLIJH)?;
        }
        if self.JLCIKBLNENH != 0 {
            os.write_uint32(777, self.JLCIKBLNENH)?;
        }
        if self.JIJHAAIHNCN != 0 {
            os.write_uint32(1401, self.JIJHAAIHNCN)?;
        }
        if self.DKPNENBHELH != 0 {
            os.write_uint32(506, self.DKPNENBHELH)?;
        }
        if self.text_id != 0 {
            os.write_uint32(1884, self.text_id)?;
        }
        if self.DJOADECJPOB != ::protobuf::EnumOrUnknown::new(super::PAJNHIAGODD::PAJNHIAGODD::MARBLE_FRAME_TYPE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.DJOADECJPOB))?;
        }
        if self.MNBEMGNNFOD != 0. {
            os.write_float(673, self.MNBEMGNNFOD)?;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0 {
            os.write_int32(11, self.TURN_FOOD_SWITCH_ATTACK)?;
        }
        if let Some(v) = self.FNIHJJJGOEE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.IDABOFPKOKN != 0 {
            os.write_uint32(1253, self.IDABOFPKOKN)?;
        }
        if self.JLJIGEPLPMH != 0 {
            os.write_uint32(1, self.JLJIGEPLPMH)?;
        }
        if let Some(v) = self.IAAGGMKGODC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.level != 0 {
            os.write_uint32(862, self.level)?;
        }
        if self.PLFKOCCDBAG != false {
            os.write_bool(2005, self.PLFKOCCDBAG)?;
        }
        if self.CCLMFABDENA != ::protobuf::EnumOrUnknown::new(super::FightMarbleHpChangeType::FightMarbleHpChangeType::MARBLE_HP_CHANGE_TYPE_NONE) {
            os.write_enum(1417, ::protobuf::EnumOrUnknown::value(&self.CCLMFABDENA))?;
        }
        if let Some(v) = self.GODNAALNOKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.id != 0 {
            os.write_uint32(10, self.id)?;
        }
        if let Some(v) = self.LKEFOLCGFGD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.NBKELCHILGG != false {
            os.write_bool(399, self.NBKELCHILGG)?;
        }
        if self.hp != 0 {
            os.write_int32(8, self.hp)?;
        }
        if self.skill_id != 0 {
            os.write_uint32(9, self.skill_id)?;
        }
        if self.max_hp != 0 {
            os.write_int32(4, self.max_hp)?;
        }
        if let Some(v) = self.FDNDMHJOHMO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.PNLDLMNKJMK != 0 {
            os.write_uint32(1226, self.PNLDLMNKJMK)?;
        }
        if self.GGBFKENAHOE != 0. {
            os.write_float(907, self.GGBFKENAHOE)?;
        }
        if self.DMBBMFFEJGI != false {
            os.write_bool(1846, self.DMBBMFFEJGI)?;
        }
        if self.DHELBCIMLGA != 0 {
            os.write_uint32(929, self.DHELBCIMLGA)?;
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

    fn new() -> DCDNIAJCEHN {
        DCDNIAJCEHN::new()
    }

    fn clear(&mut self) {
        self.HDBLELEBKHO = 0;
        self.buff_id = 0;
        self.time = 0.;
        self.KAMIHNEJMFG = ::protobuf::EnumOrUnknown::new(super::LKKAJCACIJI::LKKAJCACIJI::MARBLE_FACTION_TYPE_NONE);
        self.CIPICLLLIJH = 0;
        self.JLCIKBLNENH = 0;
        self.JIJHAAIHNCN = 0;
        self.DKPNENBHELH = 0;
        self.text_id = 0;
        self.DJOADECJPOB = ::protobuf::EnumOrUnknown::new(super::PAJNHIAGODD::PAJNHIAGODD::MARBLE_FRAME_TYPE_NONE);
        self.MNBEMGNNFOD = 0.;
        self.TURN_FOOD_SWITCH_ATTACK = 0;
        self.FNIHJJJGOEE.clear();
        self.IDABOFPKOKN = 0;
        self.JLJIGEPLPMH = 0;
        self.IAAGGMKGODC.clear();
        self.level = 0;
        self.PLFKOCCDBAG = false;
        self.CCLMFABDENA = ::protobuf::EnumOrUnknown::new(super::FightMarbleHpChangeType::FightMarbleHpChangeType::MARBLE_HP_CHANGE_TYPE_NONE);
        self.GODNAALNOKL.clear();
        self.id = 0;
        self.LKEFOLCGFGD.clear();
        self.NBKELCHILGG = false;
        self.hp = 0;
        self.skill_id = 0;
        self.max_hp = 0;
        self.FDNDMHJOHMO.clear();
        self.PNLDLMNKJMK = 0;
        self.GGBFKENAHOE = 0.;
        self.DMBBMFFEJGI = false;
        self.DHELBCIMLGA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DCDNIAJCEHN {
        static instance: DCDNIAJCEHN = DCDNIAJCEHN {
            HDBLELEBKHO: 0,
            buff_id: 0,
            time: 0.,
            KAMIHNEJMFG: ::protobuf::EnumOrUnknown::from_i32(0),
            CIPICLLLIJH: 0,
            JLCIKBLNENH: 0,
            JIJHAAIHNCN: 0,
            DKPNENBHELH: 0,
            text_id: 0,
            DJOADECJPOB: ::protobuf::EnumOrUnknown::from_i32(0),
            MNBEMGNNFOD: 0.,
            TURN_FOOD_SWITCH_ATTACK: 0,
            FNIHJJJGOEE: ::protobuf::MessageField::none(),
            IDABOFPKOKN: 0,
            JLJIGEPLPMH: 0,
            IAAGGMKGODC: ::protobuf::MessageField::none(),
            level: 0,
            PLFKOCCDBAG: false,
            CCLMFABDENA: ::protobuf::EnumOrUnknown::from_i32(0),
            GODNAALNOKL: ::protobuf::MessageField::none(),
            id: 0,
            LKEFOLCGFGD: ::protobuf::MessageField::none(),
            NBKELCHILGG: false,
            hp: 0,
            skill_id: 0,
            max_hp: 0,
            FDNDMHJOHMO: ::protobuf::MessageField::none(),
            PNLDLMNKJMK: 0,
            GGBFKENAHOE: 0.,
            DMBBMFFEJGI: false,
            DHELBCIMLGA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DCDNIAJCEHN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DCDNIAJCEHN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DCDNIAJCEHN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DCDNIAJCEHN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DCDNIAJCEHN.proto\x1a\x1dFightMarbleHpChangeType.proto\x1a\x11LKKA\
    JCACIJI.proto\x1a\x11PAJNHIAGODD.proto\x1a\x11PFGAIEBGHCP.proto\"\xea\
    \x08\n\x0bDCDNIAJCEHN\x12!\n\x0bHDBLELEBKHO\x18\xc1\x04\x20\x01(\x05R\
    \x0bHDBLELEBKHO\x12\x18\n\x07buff_id\x18\xb7\x03\x20\x01(\rR\x06buffId\
    \x12\x12\n\x04time\x18\x06\x20\x01(\x02R\x04time\x12.\n\x0bKAMIHNEJMFG\
    \x18\x0c\x20\x01(\x0e2\x0c.LKKAJCACIJIR\x0bKAMIHNEJMFG\x12\x20\n\x0bCIPI\
    CLLLIJH\x18\x0e\x20\x01(\rR\x0bCIPICLLLIJH\x12!\n\x0bJLCIKBLNENH\x18\x89\
    \x06\x20\x01(\rR\x0bJLCIKBLNENH\x12!\n\x0bJIJHAAIHNCN\x18\xf9\n\x20\x01(\
    \rR\x0bJIJHAAIHNCN\x12!\n\x0bDKPNENBHELH\x18\xfa\x03\x20\x01(\rR\x0bDKPN\
    ENBHELH\x12\x18\n\x07text_id\x18\xdc\x0e\x20\x01(\rR\x06textId\x12.\n\
    \x0bDJOADECJPOB\x18\x03\x20\x01(\x0e2\x0c.PAJNHIAGODDR\x0bDJOADECJPOB\
    \x12!\n\x0bMNBEMGNNFOD\x18\xa1\x05\x20\x01(\x02R\x0bMNBEMGNNFOD\x125\n\
    \x17TURN_FOOD_SWITCH_ATTACK\x18\x0b\x20\x01(\x05R\x14TURNFOODSWITCHATTAC\
    K\x12.\n\x0bFNIHJJJGOEE\x18\x0f\x20\x01(\x0b2\x0c.PFGAIEBGHCPR\x0bFNIHJJ\
    JGOEE\x12!\n\x0bIDABOFPKOKN\x18\xe5\t\x20\x01(\rR\x0bIDABOFPKOKN\x12\x20\
    \n\x0bJLJIGEPLPMH\x18\x01\x20\x01(\rR\x0bJLJIGEPLPMH\x12.\n\x0bIAAGGMKGO\
    DC\x18\x02\x20\x01(\x0b2\x0c.PFGAIEBGHCPR\x0bIAAGGMKGODC\x12\x15\n\x05le\
    vel\x18\xde\x06\x20\x01(\rR\x05level\x12!\n\x0bPLFKOCCDBAG\x18\xd5\x0f\
    \x20\x01(\x08R\x0bPLFKOCCDBAG\x12;\n\x0bCCLMFABDENA\x18\x89\x0b\x20\x01(\
    \x0e2\x18.FightMarbleHpChangeTypeR\x0bCCLMFABDENA\x12.\n\x0bGODNAALNOKL\
    \x18\x05\x20\x01(\x0b2\x0c.PFGAIEBGHCPR\x0bGODNAALNOKL\x12\x0e\n\x02id\
    \x18\n\x20\x01(\rR\x02id\x12.\n\x0bLKEFOLCGFGD\x18\x07\x20\x01(\x0b2\x0c\
    .PFGAIEBGHCPR\x0bLKEFOLCGFGD\x12!\n\x0bNBKELCHILGG\x18\x8f\x03\x20\x01(\
    \x08R\x0bNBKELCHILGG\x12\x0e\n\x02hp\x18\x08\x20\x01(\x05R\x02hp\x12\x19\
    \n\x08skill_id\x18\t\x20\x01(\rR\x07skillId\x12\x15\n\x06max_hp\x18\x04\
    \x20\x01(\x05R\x05maxHp\x12.\n\x0bFDNDMHJOHMO\x18\r\x20\x01(\x0b2\x0c.PF\
    GAIEBGHCPR\x0bFDNDMHJOHMO\x12!\n\x0bPNLDLMNKJMK\x18\xca\t\x20\x01(\rR\
    \x0bPNLDLMNKJMK\x12!\n\x0bGGBFKENAHOE\x18\x8b\x07\x20\x01(\x02R\x0bGGBFK\
    ENAHOE\x12!\n\x0bDMBBMFFEJGI\x18\xb6\x0e\x20\x01(\x08R\x0bDMBBMFFEJGI\
    \x12!\n\x0bDHELBCIMLGA\x18\xa1\x07\x20\x01(\rR\x0bDHELBCIMLGAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::FightMarbleHpChangeType::file_descriptor().clone());
            deps.push(super::LKKAJCACIJI::file_descriptor().clone());
            deps.push(super::PAJNHIAGODD::file_descriptor().clone());
            deps.push(super::PFGAIEBGHCP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DCDNIAJCEHN::generated_message_descriptor_data());
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
