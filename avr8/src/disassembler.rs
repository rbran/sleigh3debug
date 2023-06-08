pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    Xlo,
    Xhi,
    Ylo,
    Yhi,
    Zlo,
    Zhi,
    R1R0,
    R3R2,
    R5R4,
    R7R6,
    R9R8,
    R11R10,
    R13R12,
    R15R14,
    R17R16,
    R19R18,
    R21R20,
    R23R22,
    R25R24,
    X,
    Y,
    Z,
    R25R24R23R22R21R20R19R18,
    R7R6R5R4R3R2R1R0,
    R15R14R13R12R11R10R9R8,
    SPL,
    SPH,
    SP,
    PC,
    Cflg,
    Zflg,
    Nflg,
    Vflg,
    Sflg,
    Hflg,
    Tflg,
    Iflg,
    SKIP,
    RAMPD,
    RAMPX,
    RAMPY,
    RAMPZ,
    SREG,
    contextreg,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::R0 => "R0",
            Self::R1 => "R1",
            Self::R2 => "R2",
            Self::R3 => "R3",
            Self::R4 => "R4",
            Self::R5 => "R5",
            Self::R6 => "R6",
            Self::R7 => "R7",
            Self::R8 => "R8",
            Self::R9 => "R9",
            Self::R10 => "R10",
            Self::R11 => "R11",
            Self::R12 => "R12",
            Self::R13 => "R13",
            Self::R14 => "R14",
            Self::R15 => "R15",
            Self::R16 => "R16",
            Self::R17 => "R17",
            Self::R18 => "R18",
            Self::R19 => "R19",
            Self::R20 => "R20",
            Self::R21 => "R21",
            Self::R22 => "R22",
            Self::R23 => "R23",
            Self::R24 => "R24",
            Self::R25 => "R25",
            Self::Xlo => "Xlo",
            Self::Xhi => "Xhi",
            Self::Ylo => "Ylo",
            Self::Yhi => "Yhi",
            Self::Zlo => "Zlo",
            Self::Zhi => "Zhi",
            Self::R1R0 => "R1R0",
            Self::R3R2 => "R3R2",
            Self::R5R4 => "R5R4",
            Self::R7R6 => "R7R6",
            Self::R9R8 => "R9R8",
            Self::R11R10 => "R11R10",
            Self::R13R12 => "R13R12",
            Self::R15R14 => "R15R14",
            Self::R17R16 => "R17R16",
            Self::R19R18 => "R19R18",
            Self::R21R20 => "R21R20",
            Self::R23R22 => "R23R22",
            Self::R25R24 => "R25R24",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::R25R24R23R22R21R20R19R18 => "R25R24R23R22R21R20R19R18",
            Self::R7R6R5R4R3R2R1R0 => "R7R6R5R4R3R2R1R0",
            Self::R15R14R13R12R11R10R9R8 => "R15R14R13R12R11R10R9R8",
            Self::SPL => "SPL",
            Self::SPH => "SPH",
            Self::SP => "SP",
            Self::PC => "PC",
            Self::Cflg => "Cflg",
            Self::Zflg => "Zflg",
            Self::Nflg => "Nflg",
            Self::Vflg => "Vflg",
            Self::Sflg => "Sflg",
            Self::Hflg => "Hflg",
            Self::Tflg => "Tflg",
            Self::Iflg => "Iflg",
            Self::SKIP => "SKIP",
            Self::RAMPD => "RAMPD",
            Self::RAMPX => "RAMPX",
            Self::RAMPY => "RAMPY",
            Self::RAMPZ => "RAMPZ",
            Self::SREG => "SREG",
            Self::contextreg => "contextreg",
        }
    }
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
fn meaning_0_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::Cflg,
        1 => Register::Zflg,
        2 => Register::Nflg,
        3 => Register::Vflg,
        4 => Register::Sflg,
        5 => Register::Hflg,
        6 => Register::Tflg,
        7 => Register::Iflg,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_0_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_0_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_1_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R16,
        1 => Register::R17,
        2 => Register::R18,
        3 => Register::R19,
        4 => Register::R20,
        5 => Register::R21,
        6 => Register::R22,
        7 => Register::R23,
        8 => Register::R24,
        9 => Register::R25,
        10 => Register::Xlo,
        11 => Register::Xhi,
        12 => Register::Ylo,
        13 => Register::Yhi,
        14 => Register::Zlo,
        15 => Register::Zhi,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_1_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_1_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_2_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R16,
        1 => Register::R17,
        2 => Register::R18,
        3 => Register::R19,
        4 => Register::R20,
        5 => Register::R21,
        6 => Register::R22,
        7 => Register::R23,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_2_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_2_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_3_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        4 => Register::R4,
        5 => Register::R5,
        6 => Register::R6,
        7 => Register::R7,
        8 => Register::R8,
        9 => Register::R9,
        10 => Register::R10,
        11 => Register::R11,
        12 => Register::R12,
        13 => Register::R13,
        14 => Register::R14,
        15 => Register::R15,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_3_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_3_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_4_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        4 => Register::R4,
        5 => Register::R5,
        6 => Register::R6,
        7 => Register::R7,
        8 => Register::R8,
        9 => Register::R9,
        10 => Register::R10,
        11 => Register::R11,
        12 => Register::R12,
        13 => Register::R13,
        14 => Register::R14,
        15 => Register::R15,
        16 => Register::R16,
        17 => Register::R17,
        18 => Register::R18,
        19 => Register::R19,
        20 => Register::R20,
        21 => Register::R21,
        22 => Register::R22,
        23 => Register::R23,
        24 => Register::R24,
        25 => Register::R25,
        26 => Register::Xlo,
        27 => Register::Xhi,
        28 => Register::Ylo,
        29 => Register::Yhi,
        30 => Register::Zlo,
        31 => Register::Zhi,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_4_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_4_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_5_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R25R24,
        1 => Register::X,
        2 => Register::Y,
        3 => Register::Z,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_5_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_5_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_6_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::Z,
        1 => Register::Y,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_6_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_6_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_7_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::Z,
        2 => Register::Y,
        3 => Register::X,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_7_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_7_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_8_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R1R0,
        1 => Register::R3R2,
        2 => Register::R5R4,
        3 => Register::R7R6,
        4 => Register::R9R8,
        5 => Register::R11R10,
        6 => Register::R13R12,
        7 => Register::R15R14,
        8 => Register::R17R16,
        9 => Register::R19R18,
        10 => Register::R21R20,
        11 => Register::R23R22,
        12 => Register::R25R24,
        13 => Register::X,
        14 => Register::Y,
        15 => Register::Z,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_8_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_8_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_9_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R1R0,
        1 => Register::R3R2,
        2 => Register::R5R4,
        3 => Register::R7R6,
        4 => Register::R9R8,
        5 => Register::R11R10,
        6 => Register::R13R12,
        7 => Register::R15R14,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_9_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_9_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
fn meaning_10_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::R17R16,
        1 => Register::R19R18,
        2 => Register::R21R20,
        3 => Register::R23R22,
        4 => Register::R25R24,
        5 => Register::X,
        6 => Register::Y,
        7 => Register::Z,
        _ => unreachable!("Invalid Attach Value"),
    }
}
fn meaning_10_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    let value = meaning_10_value(num.try_into().unwrap());
    <DisplayElement>::Register(value)
}
#[derive(Clone, Copy, Debug)]
pub enum DisplayElement {
    Literal(&'static str),
    Register(Register),
    Number(bool, bool, u64),
}
impl core::fmt::Display for DisplayElement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Literal(lit) => lit.fmt(f),
            Self::Register(reg) => reg.fmt(f),
            Self::Number(true, false, value) => {
                write!(f, "0x{:x}", value)
            }
            Self::Number(true, true, value) => {
                write!(f, "-0x{:x}", value)
            }
            Self::Number(false, false, value) => value.fmt(f),
            Self::Number(false, true, value) => {
                write!(f, "-{:x}", value)
            }
        }
    }
}
#[doc = "Create token_fields: op2bit0"]
fn token_52(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 1) as u8)
}
#[doc = "Create token_fields: opbit9 RrHiLowSel"]
fn token_12(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 1) as u8)
}
#[doc = "Create token_fields: f3op1hi4"]
fn token_57(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 12) & 15) as u8)
}
#[doc = "Create token_fields: op2bits8to11"]
fn token_50(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 15) as u8)
}
#[doc = "Create token_fields: ldswop2low4"]
fn token_78(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 32) & 15) as u8)
}
#[doc = "Create token_fields: f3op1bits5to7 f3op1RdPairHi"]
fn token_67(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 5) & 7) as u8)
}
#[doc = "Create token_fields: ophi8"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 255) as u8)
}
#[doc = "Create token_fields: ophi6"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 10) & 63) as u8)
}
#[doc = "Create token_fields: opbit13"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 13) & 1) as u8)
}
#[doc = "Create token_fields: opbit12"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 1) as u8)
}
#[doc = "Create token_fields: ldswop2imm16"]
fn token_90(tokens: &[u8]) -> u16 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 48) & 65535) as u16)
}
#[doc = "Create token_fields: ldswop1bit4"]
fn token_81(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: ldswop1bit16"]
fn token_83(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 16) & 1) as u8)
}
#[doc = "Create token_fields: op2bits1to3"]
fn token_42(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 17) & 7) as u8)
}
#[doc = "Create token_fields: ldswop2bit4"]
fn token_82(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 36) & 1) as u8)
}
#[doc = "Create token_fields: op2bits4to8"]
fn token_44(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 20) & 31) as u8)
}
#[doc = "Create token_fields: op1bit4"]
fn token_53(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: f3op1hi6"]
fn token_60(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 10) & 63) as u8)
}
#[doc = "Create token_fields: opbit0"]
fn token_17(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: f3op1bits8to11"]
fn token_69(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 8) & 15) as u8)
}
#[doc = "Create token_fields: f3op3bit9"]
fn token_74(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 41) & 1) as u8)
}
#[doc = "Create token_fields: oplow3_flag oplow3 RrHi3"]
fn token_20(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: op1bits5to7 op1RdPairHi"]
fn token_45(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 5) & 7) as u8)
}
#[doc = "Create token_fields: ldswop1imm6"]
fn token_87(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 17) & 63) as u8)
}
#[doc = "Create token_fields: op8to11"]
fn token_34(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 15) as u8)
}
#[doc = "Create token_fields: ldswop2imm15"]
fn token_86(tokens: &[u8]) -> u16 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 49) & 32767) as u16)
}
#[doc = "Create token_fields: op1bits1to3 op1RrPairLow op1RrPairHi"]
fn token_41(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 1) & 7) as u8)
}
#[doc = "Create token_fields: f3op2bits4to7 f3op2RdHi"]
fn token_66(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 20) & 15) as u8)
}
#[doc = "Create token_fields: op2hi4"]
fn token_36(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 28) & 15) as u8)
}
#[doc = "Create token_fields: ophi7"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 127) as u8)
}
#[doc = "Create token_fields: f3op2bits0to3"]
fn token_64(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 16) & 15) as u8)
}
#[doc = "Create token_fields: op2low4 op2bits0to3"]
fn token_40(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 15) as u8)
}
#[doc = "Create token_fields: op3to9signed"]
fn token_33(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 127) as u8)
}
#[doc = "Create token_fields: opbit2"]
fn token_16(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 2) & 1) as u8)
}
#[doc = "Create token_fields: op1bit0"]
fn token_51(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: f3op3hi4"]
fn token_59(tokens: &[u8]) -> u16 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 34) & 16383) as u16)
}
#[doc = "Create token_fields: op8to10"]
fn token_28(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 7) as u8)
}
#[doc = "Create token_fields: f3op3bits5to7"]
fn token_68(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 37) & 7) as u8)
}
#[doc = "Create token_fields: ldswop2hi7"]
fn token_76(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 41) & 127) as u8)
}
#[doc = "Create token_fields: oplow12 oplow12signed"]
fn token_18(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: opbit8"]
fn token_13(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 1) as u8)
}
#[doc = "Create token_fields: RdHi Rdw4 op4to7"]
fn token_31(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: ldswop1imm15"]
fn token_85(tokens: &[u8]) -> u16 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 17) & 32767) as u16)
}
#[doc = "Create token_fields: op2bit4"]
fn token_54(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 20) & 1) as u8)
}
#[doc = "Create token_fields: op4to6 op4to6_flag RdHi3"]
fn token_26(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 7) as u8)
}
#[doc = "Create token_fields: ophi4"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: op6to7"]
fn token_27(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 6) & 3) as u8)
}
#[doc = "Create token_fields: f3op2hi6"]
fn token_61(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 26) & 63) as u8)
}
#[doc = "Create token_fields: ophi9"]
fn token_2(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 7) & 511) as u16)
}
#[doc = "Create token_fields: ldswop1hi7"]
fn token_75(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 9) & 127) as u8)
}
#[doc = "Create token_fields: f3op3bit8"]
fn token_73(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 40) & 1) as u8)
}
#[doc = "Create token_fields: f3op2bits8to11"]
fn token_70(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 24) & 15) as u8)
}
#[doc = "Create token_fields: op10to11"]
fn token_30(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 10) & 3) as u8)
}
#[doc = "Create token_fields: opbit3 Rstq"]
fn token_15(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 1) as u8)
}
#[doc = "Create token_fields: op1bits4to8"]
fn token_43(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 4) & 31) as u8)
}
#[doc = "Create token_fields: opbit10"]
fn token_11(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 10) & 1) as u8)
}
#[doc = "Create token_fields: op1hi6"]
fn token_37(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 10) & 63) as u8)
}
#[doc = "Create token_fields: ldswop1bits5to8 ldswop1RdPair"]
fn token_79(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 5) & 15) as u8)
}
#[doc = "Create token_fields: ldswop1imm16"]
fn token_89(tokens: &[u8]) -> u16 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 16) & 65535) as u16)
}
#[doc = "Create token_fields: op9to10"]
fn token_29(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 3) as u8)
}
#[doc = "Create token_fields: opbit7"]
fn token_14(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 7) & 1) as u8)
}
#[doc = "Create token_fields: ldswop2bit16"]
fn token_84(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 48) & 1) as u8)
}
#[doc = "Create token_fields: op1bits5to8 op1RdPair"]
fn token_47(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 5) & 15) as u8)
}
#[doc = "Create token_fields: oplow4 RrHi RrLow Rrw4 op0to3"]
fn token_19(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: ophi16 next16"]
fn token_1(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: ophi2"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 14) & 3) as u8)
}
#[doc = "Create token_fields: op4to8 RdFull"]
fn token_25(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 31) as u8)
}
#[doc = "Create token_fields: op2hi6"]
fn token_38(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 26) & 63) as u8)
}
#[doc = "Create token_fields: op1low4 op1bits0to3"]
fn token_39(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: op1bits8to11"]
fn token_49(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 8) & 15) as u8)
}
#[doc = "Create token_fields: op2bit9"]
fn token_56(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 25) & 1) as u8)
}
#[doc = "Create token_fields: f3op3bits0to3"]
fn token_65(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 32) & 15) as u8)
}
#[doc = "Create token_fields: op2to3 RstPtr"]
fn token_23(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 2) & 3) as u8)
}
#[doc = "Create token_fields: f3op3bit4"]
fn token_72(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 36) & 1) as u8)
}
#[doc = "Create token_fields: f3op3hi6"]
fn token_62(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 42) & 63) as u8)
}
#[doc = "Create token_fields: f3op1bit4"]
fn token_71(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 4) & 1) as u8)
}
#[doc = "Create token_fields: op1to3"]
fn token_22(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 7) as u8)
}
#[doc = "Create token_fields: oplow2"]
fn token_21(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 3) as u8)
}
#[doc = "Create token_fields: ldswop2bits5to8 stswop2RdPair"]
fn token_80(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 37) & 15) as u8)
}
#[doc = "Create token_fields: Rdw2"]
fn token_32(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 3) as u8)
}
#[doc = "Create token_fields: op2bits5to7"]
fn token_46(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 21) & 7) as u8)
}
#[doc = "Create token_fields: f3op1bits0to3"]
fn token_63(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 0) & 15) as u8)
}
#[doc = "Create token_fields: ophi5"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 11) & 31) as u8)
}
#[doc = "Create token_fields: ldswop1low4"]
fn token_77(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: op1hi4"]
fn token_35(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: op1bit9 op1RrPairSel"]
fn token_55(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 9) & 1) as u8)
}
#[doc = "Create token_fields: ldswop2imm6"]
fn token_88(tokens: &[u8]) -> u8 {
    (((u64::from_le_bytes(tokens[0..8].try_into().unwrap()) >> 49) & 63) as u8)
}
#[doc = "Create token_fields: op2bits5to8"]
fn token_48(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 21) & 15) as u8)
}
#[doc = "Create token_fields: op3to7"]
fn token_24(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 31) as u8)
}
#[doc = "Create token_fields: f3op2hi4"]
fn token_58(tokens: &[u8]) -> u8 {
    let mut bytes = [0u8; 8];
    bytes[0..6].copy_from_slice(&tokens[0..6]);
    let value = u64::from_le_bytes(bytes);
    (((value >> 28) & 15) as u8)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u8);
impl ContextMemory {
    pub fn read_useSkipCond(&self) -> u8 {
        (((self.0.reverse_bits() >> 7) & 1) as u8)
    }
    pub fn write_useSkipCond(&mut self, value: u8) {
        self.0 = ((self.0.reverse_bits() & !(1 << 7)) | ((value as u8 & 1) << 7)).reverse_bits();
    }
    pub fn read_phase(&self) -> u8 {
        (((self.0.reverse_bits() >> 6) & 1) as u8)
    }
    pub fn write_phase(&mut self, value: u8) {
        self.0 = ((self.0.reverse_bits() & !(1 << 6)) | ((value as u8 & 1) << 6)).reverse_bits();
    }
}
#[derive(Clone)]
pub struct GlobalSet {
    default: ContextMemory,
    branches: std::collections::HashMap<AddrType, ContextMemory>,
}
impl GlobalSet {
    pub fn new(default: ContextMemory) -> Self {
        Self {
            default,
            branches: std::collections::HashMap::new(),
        }
    }
    pub fn set(&mut self, address: Option<AddrType>, set: impl FnOnce(&mut ContextMemory)) {
        let Some (address) = address else { return } ;
        let entry = self
            .branches
            .entry(address)
            .or_insert_with(|| self.default.clone());
        set(entry);
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:435:1, end:435:2))"]
#[derive(Clone, Debug)]
struct instructionVar0 {
    instruction: Box<Tableinstruction>,
}
impl instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.instruction
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_phase(u8::try_from(1i128 & 1).unwrap());
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { instruction }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:436:1, end:436:2))"]
#[derive(Clone, Debug)]
struct instructionVar1 {
    instruction: Box<Tableinstruction>,
}
impl instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.instruction
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_phase(u8::try_from(1i128 & 1).unwrap());
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { instruction }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:622:1, end:622:2))"]
#[derive(Clone, Debug)]
struct break_instructionVar2 {}
impl break_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("break"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:653:1, end:653:2))"]
#[derive(Clone, Debug)]
struct clc_instructionVar3 {}
impl clc_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clc"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:656:1, end:656:2))"]
#[derive(Clone, Debug)]
struct clh_instructionVar4 {}
impl clh_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clh"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:659:1, end:659:2))"]
#[derive(Clone, Debug)]
struct cli_instructionVar5 {}
impl cli_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cli"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:662:1, end:662:2))"]
#[derive(Clone, Debug)]
struct cln_instructionVar6 {}
impl cln_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cln"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:665:1, end:665:2))"]
#[derive(Clone, Debug)]
struct cls_instructionVar7 {}
impl cls_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cls"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:668:1, end:668:2))"]
#[derive(Clone, Debug)]
struct clt_instructionVar8 {}
impl clt_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clt"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:671:1, end:671:2))"]
#[derive(Clone, Debug)]
struct clv_instructionVar9 {}
impl clv_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clv"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:674:1, end:674:2))"]
#[derive(Clone, Debug)]
struct clz_instructionVar10 {}
impl clz_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clz"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:794:1, end:794:2))"]
#[derive(Clone, Debug)]
struct icall_instructionVar11 {}
impl icall_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("icall"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:800:1, end:800:2))"]
#[derive(Clone, Debug)]
struct ijmp_instructionVar12 {}
impl ijmp_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ijmp"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:915:1, end:915:2))"]
#[derive(Clone, Debug)]
struct lpm_instructionVar13 {}
impl lpm_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lpm"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::R0),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:973:1, end:973:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar14 {}
impl nop_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("nop"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1008:1, end:1008:2))"]
#[derive(Clone, Debug)]
struct rcall_instructionVar15 {}
impl rcall_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rcall"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("."),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1020:1, end:1020:2))"]
#[derive(Clone, Debug)]
struct ret_instructionVar16 {}
impl ret_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ret"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1025:1, end:1025:2))"]
#[derive(Clone, Debug)]
struct reti_instructionVar17 {}
impl reti_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("reti"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1102:1, end:1102:2))"]
#[derive(Clone, Debug)]
struct sleep_instructionVar18 {}
impl sleep_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sleep"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1108:1, end:1108:2))"]
#[derive(Clone, Debug)]
struct spm_instructionVar19 {}
impl spm_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("spm"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::Z),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1116:1, end:1116:2))"]
#[derive(Clone, Debug)]
struct spm_instructionVar20 {
    SpmPlus: TableSpmPlus,
}
impl spm_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("spm"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.SpmPlus
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let SpmPlus = if let Some((len, table)) =
            TableSpmPlus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { SpmPlus }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1184:1, end:1184:2))"]
#[derive(Clone, Debug)]
struct wdr_instructionVar21 {}
impl wdr_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("wdr"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:601:1, end:601:2))"]
#[derive(Clone, Debug)]
struct bclr_instructionVar22 {
    op4to6_flag: u8,
}
impl bclr_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bclr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.op4to6_flag),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let op4to6_flag = token_26(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op4to6_flag }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:629:1, end:629:2))"]
#[derive(Clone, Debug)]
struct bset_instructionVar23 {
    op4to6_flag: u8,
}
impl bset_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bset"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.op4to6_flag),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        if i128::try_from(token_2(tokens_current)).unwrap()
            != u32::try_from(1i128)
                .ok()
                .and_then(|shl| 148i128.checked_shl(shl))
                .unwrap_or(0)
        {
            return None;
        }
        let op4to6_flag = token_26(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op4to6_flag }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:734:1, end:734:2))"]
#[derive(Clone, Debug)]
struct des_instructionVar24 {
    op4to7: u8,
}
impl des_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("des"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op4to7 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let op4to7 = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op4to7 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:790:1, end:790:2))"]
#[derive(Clone, Debug)]
struct fracmul_instructionVar25 {
    RdHi: u8,
    RrHi: u8,
}
impl fracmul_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fracmul"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.RrHi),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdHi = token_31(tokens_current);
        let RrHi = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi, RrHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:791:1, end:791:2))"]
#[derive(Clone, Debug)]
struct fracmuls_instructionVar26 {
    RdHi: u8,
    RrHi: u8,
}
impl fracmuls_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fracmuls"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.RrHi),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrHi = token_19(tokens_current);
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi, RrHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:792:1, end:792:2))"]
#[derive(Clone, Debug)]
struct fracmulsu_instructionVar27 {
    RdHi: u8,
    RrHi: u8,
}
impl fracmulsu_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fracmulsu"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.RrHi),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrHi = token_19(tokens_current);
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi, RrHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:961:1, end:961:2))"]
#[derive(Clone, Debug)]
struct mulsu_instructionVar28 {
    RdHi3: u8,
    RrHi3: u8,
}
impl mulsu_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulsu"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.RdHi3),
            <DisplayElement>::Literal(","),
            meaning_2_display(self.RrHi3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdHi3 = token_26(tokens_current);
        let RrHi3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi3, RrHi3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1096:1, end:1096:2))"]
#[derive(Clone, Debug)]
struct ser_instructionVar29 {
    RdHi: u8,
}
impl ser_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ser"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_1_display(self.RdHi)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:572:1, end:572:2))"]
#[derive(Clone, Debug)]
struct adiw_instructionVar30 {
    Rdw2: u8,
    K6: TableK6,
}
impl adiw_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("adiw"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_5_display(self.Rdw2),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K6
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K6 = if let Some((len, table)) =
            TableK6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rdw2 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K6, Rdw2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:592:1, end:592:2))"]
#[derive(Clone, Debug)]
struct asr_instructionVar31 {
    RdFull: u8,
}
impl asr_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("asr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:724:1, end:724:2))"]
#[derive(Clone, Debug)]
struct dec_instructionVar32 {
    RdFull: u8,
}
impl dec_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dec"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:808:1, end:808:2))"]
#[derive(Clone, Debug)]
struct in_instructionVar33 {
    RdFull: u8,
}
impl in_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("in"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::SPL),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:811:1, end:811:2))"]
#[derive(Clone, Debug)]
struct in_instructionVar34 {
    RdFull: u8,
}
impl in_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("in"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::SPH),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:814:1, end:814:2))"]
#[derive(Clone, Debug)]
struct in_instructionVar35 {
    RdFull: u8,
}
impl in_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("in"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::SREG),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:818:1, end:818:2))"]
#[derive(Clone, Debug)]
struct inc_instructionVar36 {
    RdFull: u8,
}
impl inc_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("inc"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:829:1, end:829:2))"]
#[derive(Clone, Debug)]
struct lac_instructionVar37 {
    RdFull: u8,
}
impl lac_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lac"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::Z),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:836:1, end:836:2))"]
#[derive(Clone, Debug)]
struct las_instructionVar38 {
    RdFull: u8,
}
impl las_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("las"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::Z),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:843:1, end:843:2))"]
#[derive(Clone, Debug)]
struct lat_instructionVar39 {
    RdFull: u8,
}
impl lat_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lat"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::Z),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:852:1, end:852:2))"]
#[derive(Clone, Debug)]
struct ld_instructionVar40 {
    RdFull: u8,
}
impl ld_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::X),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:891:1, end:891:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar41 {
    RdFull: u8,
    next16memPtrVal1: Tablenext16memPtrVal1,
}
impl lds_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.next16memPtrVal1
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let next16memPtrVal1 = if let Some((len, table)) =
            Tablenext16memPtrVal1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                next16memPtrVal1,
                RdFull,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:921:1, end:921:2))"]
#[derive(Clone, Debug)]
struct lpm_instructionVar42 {
    RdFull: u8,
}
impl lpm_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lpm"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::Z),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:928:1, end:928:2))"]
#[derive(Clone, Debug)]
struct lpm_instructionVar43 {
    RdFull: u8,
    LpmPlus: TableLpmPlus,
}
impl lpm_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lpm"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.LpmPlus
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let LpmPlus = if let Some((len, table)) =
            TableLpmPlus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LpmPlus, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:935:1, end:935:2))"]
#[derive(Clone, Debug)]
struct lsr_instructionVar44 {
    RdFull: u8,
}
impl lsr_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:967:1, end:967:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar45 {
    RdFull: u8,
}
impl neg_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:989:1, end:989:2))"]
#[derive(Clone, Debug)]
struct out_instructionVar46 {
    RdFull: u8,
}
impl out_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("out"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SPL),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:992:1, end:992:2))"]
#[derive(Clone, Debug)]
struct out_instructionVar47 {
    RdFull: u8,
}
impl out_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("out"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SPH),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:995:1, end:995:2))"]
#[derive(Clone, Debug)]
struct out_instructionVar48 {
    RdFull: u8,
}
impl out_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("out"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SREG),
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:999:1, end:999:2))"]
#[derive(Clone, Debug)]
struct pop_instructionVar49 {
    RdFull: u8,
}
impl pop_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("pop"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct push_instructionVar50 {
    RdFull: u8,
}
impl push_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("push"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1035:1, end:1035:2))"]
#[derive(Clone, Debug)]
struct ror_instructionVar51 {
    RdFull: u8,
}
impl ror_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ror"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1124:1, end:1124:2))"]
#[derive(Clone, Debug)]
struct st_instructionVar52 {
    RdFull: u8,
}
impl st_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1155:1, end:1155:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar53 {
    RdFull: u8,
    next16memPtrVal1: Tablenext16memPtrVal1,
}
impl sts_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.next16memPtrVal1
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let next16memPtrVal1 = if let Some((len, table)) =
            Tablenext16memPtrVal1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                next16memPtrVal1,
                RdFull,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1177:1, end:1177:2))"]
#[derive(Clone, Debug)]
struct swap_instructionVar54 {
    RdFull: u8,
}
impl swap_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("swap"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1187:1, end:1187:2))"]
#[derive(Clone, Debug)]
struct xch_instructionVar55 {
    RdFull: u8,
}
impl xch_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xch"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:637:1, end:637:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar56 {
    abs22dst: Tableabs22dst,
}
impl call_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("call"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.abs22dst
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        let mut sub_pattern_c47 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if token_4(tokens) != 74 {
                return None;
            }
            if token_22(tokens) != 7 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c47(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let abs22dst = if let Some((len, table)) =
            Tableabs22dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { abs22dst }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:824:1, end:824:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar57 {
    abs22dst: Tableabs22dst,
}
impl jmp_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jmp"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.abs22dst
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        let mut sub_pattern_c47 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if token_4(tokens) != 74 {
                return None;
            }
            if token_22(tokens) != 6 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c47(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let abs22dst = if let Some((len, table)) =
            Tableabs22dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { abs22dst }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:606:1, end:606:2))"]
#[derive(Clone, Debug)]
struct bld_instructionVar58 {
    RdFull: u8,
    oplow3: u8,
}
impl bld_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bld"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let oplow3 = token_20(tokens_current);
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:633:1, end:633:2))"]
#[derive(Clone, Debug)]
struct bst_instructionVar59 {
    RdFull: u8,
    oplow3: u8,
}
impl bst_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bst"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:644:1, end:644:2))"]
#[derive(Clone, Debug)]
struct cbi_instructionVar60 {
    oplow3: u8,
    Aio5: TableAio5,
}
impl cbi_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cbi"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Aio5
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let Aio5 = if let Some((len, table)) =
            TableAio5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio5, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:858:1, end:858:2))"]
#[derive(Clone, Debug)]
struct ld_instructionVar61 {
    RdFull: u8,
    RstPtr: u8,
}
impl ld_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            meaning_7_display(self.RstPtr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull, RstPtr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1129:1, end:1129:2))"]
#[derive(Clone, Debug)]
struct st_instructionVar62 {
    RstPtr: u8,
    RdFull: u8,
}
impl st_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_7_display(self.RstPtr),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstPtr, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:865:1, end:865:2))"]
#[derive(Clone, Debug)]
struct ld_instructionVar63 {
    RdFull: u8,
    LdPlus: TableLdPlus,
}
impl ld_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.LdPlus
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let LdPlus = if let Some((len, table)) =
            TableLdPlus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LdPlus, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:872:1, end:872:2))"]
#[derive(Clone, Debug)]
struct ld_instructionVar64 {
    RdFull: u8,
    LdPredec: TableLdPredec,
}
impl ld_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.LdPredec
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let LdPredec = if let Some((len, table)) =
            TableLdPredec::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LdPredec, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:946:1, end:946:2))"]
#[derive(Clone, Debug)]
struct movw_instructionVar65 {
    Rdw4: u8,
    Rrw4: u8,
}
impl movw_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_8_display(self.Rdw4),
            <DisplayElement>::Literal(","),
            meaning_8_display(self.Rrw4),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let Rrw4 = token_19(tokens_current);
        let Rdw4 = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rdw4, Rrw4 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:955:1, end:955:2))"]
#[derive(Clone, Debug)]
struct muls_instructionVar66 {
    RdHi: u8,
    RrHi: u8,
}
impl muls_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("muls"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.RrHi),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrHi = token_19(tokens_current);
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdHi, RrHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1067:1, end:1067:2))"]
#[derive(Clone, Debug)]
struct sbi_instructionVar67 {
    oplow3: u8,
    Aio5: TableAio5,
}
impl sbi_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sbi"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Aio5
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let Aio5 = if let Some((len, table)) =
            TableAio5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio5, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1071:1, end:1071:2))"]
#[derive(Clone, Debug)]
struct sbic_instructionVar68 {
    oplow3: u8,
    Aio5: TableAio5,
}
impl sbic_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_useSkipCond(
                u8::try_from(i128::try_from(context.read_useSkipCond()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("sbic"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Aio5
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_useSkipCond(u8::try_from(1i128 & 1).unwrap());
        let Aio5 = if let Some((len, table)) =
            TableAio5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio5, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1074:1, end:1074:2))"]
#[derive(Clone, Debug)]
struct sbis_instructionVar69 {
    oplow3: u8,
    Aio5: TableAio5,
}
impl sbis_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_useSkipCond(
                u8::try_from(i128::try_from(context.read_useSkipCond()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("sbis"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Aio5
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_useSkipCond(u8::try_from(1i128 & 1).unwrap());
        let Aio5 = if let Some((len, table)) =
            TableAio5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio5, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1078:1, end:1078:2))"]
#[derive(Clone, Debug)]
struct sbiw_instructionVar70 {
    Rdw2: u8,
    K6: TableK6,
}
impl sbiw_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sbiw"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_5_display(self.Rdw2),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K6
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K6 = if let Some((len, table)) =
            TableK6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rdw2 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K6, Rdw2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1087:1, end:1087:2))"]
#[derive(Clone, Debug)]
struct sbrc_instructionVar71 {
    RdFull: u8,
    oplow3: u8,
}
impl sbrc_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_useSkipCond(
                u8::try_from(i128::try_from(context.read_useSkipCond()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("sbrc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_useSkipCond(u8::try_from(1i128 & 1).unwrap());
        let RdFull = token_25(tokens_current);
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1090:1, end:1090:2))"]
#[derive(Clone, Debug)]
struct sbrs_instructionVar72 {
    RdFull: u8,
    oplow3: u8,
}
impl sbrs_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_useSkipCond(
                u8::try_from(i128::try_from(context.read_useSkipCond()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("sbrs"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.oplow3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_useSkipCond(u8::try_from(1i128 & 1).unwrap());
        let oplow3 = token_20(tokens_current);
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull, oplow3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1136:1, end:1136:2))"]
#[derive(Clone, Debug)]
struct st_instructionVar73 {
    RdFull: u8,
    StPlus: TableStPlus,
}
impl st_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.StPlus
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let StPlus = if let Some((len, table)) =
            TableStPlus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { StPlus, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1143:1, end:1143:2))"]
#[derive(Clone, Debug)]
struct st_instructionVar74 {
    RdFull: u8,
    StPredec: TableStPredec,
}
impl st_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.StPredec
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let StPredec = if let Some((len, table)) =
            TableStPredec::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { StPredec, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:678:1, end:678:2))"]
#[derive(Clone, Debug)]
struct com_instructionVar75 {
    RdFull: u8,
}
impl com_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("com"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:556:1, end:556:2))"]
#[derive(Clone, Debug)]
struct adc_instructionVar76 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl adc_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("adc"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:564:1, end:564:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar77 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl add_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:580:1, end:580:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar78 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl and_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:612:1, end:612:2))"]
#[derive(Clone, Debug)]
struct brbc_instructionVar79 {
    oplow3_flag: u8,
    rel7dst: Tablerel7dst,
}
impl brbc_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("brbc"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.rel7dst
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_0_display(self.oplow3_flag),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rel7dst = if let Some((len, table)) =
            Tablerel7dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3_flag = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rel7dst,
                oplow3_flag,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:617:1, end:617:2))"]
#[derive(Clone, Debug)]
struct brbs_instructionVar80 {
    oplow3_flag: u8,
    rel7dst: Tablerel7dst,
}
impl brbs_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("brbs"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.rel7dst
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_0_display(self.oplow3_flag),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rel7dst = if let Some((len, table)) =
            Tablerel7dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow3_flag = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rel7dst,
                oplow3_flag,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:684:1, end:684:2))"]
#[derive(Clone, Debug)]
struct cp_instructionVar81 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl cp_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cp"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:691:1, end:691:2))"]
#[derive(Clone, Debug)]
struct cpc_instructionVar82 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl cpc_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cpc"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:720:1, end:720:2))"]
#[derive(Clone, Debug)]
struct cpse_instructionVar83 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl cpse_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_useSkipCond(
                u8::try_from(i128::try_from(context.read_useSkipCond()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("cpse"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_useSkipCond(u8::try_from(1i128 & 1).unwrap());
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:783:1, end:783:2))"]
#[derive(Clone, Debug)]
struct eor_instructionVar84 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl eor_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("eor"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:942:1, end:942:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar85 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl mov_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:949:1, end:949:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar86 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl mul_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:975:1, end:975:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar87 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl or_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1045:1, end:1045:2))"]
#[derive(Clone, Debug)]
struct sbc_instructionVar88 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl sbc_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sbc"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1170:1, end:1170:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar89 {
    RdFull: u8,
    RrFull: TableRrFull,
}
impl sub_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.RrFull
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrFull = if let Some((len, table)) =
            TableRrFull::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrFull, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:805:1, end:805:2))"]
#[derive(Clone, Debug)]
struct in_instructionVar90 {
    RdFull: u8,
    Aio6: TableAio6,
}
impl in_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("in"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.Aio6
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let Aio6 = if let Some((len, table)) =
            TableAio6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio6, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:910:1, end:910:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar91 {
    RdHi: u8,
    K7addr: TableK7addr,
}
impl lds_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K7addr
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K7addr = if let Some((len, table)) =
            TableK7addr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K7addr, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:986:1, end:986:2))"]
#[derive(Clone, Debug)]
struct out_instructionVar92 {
    RdFull: u8,
    Aio6: TableAio6,
}
impl out_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("out"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Aio6
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let Aio6 = if let Some((len, table)) =
            TableAio6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Aio6, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1167:1, end:1167:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar93 {
    RdHi: u8,
    K7addr: TableK7addr,
}
impl sts_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.K7addr
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K7addr = if let Some((len, table)) =
            TableK7addr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K7addr, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:586:1, end:586:2))"]
#[derive(Clone, Debug)]
struct andi_instructionVar94 {
    RdHi: u8,
    K8: TableK8,
}
impl andi_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("andi"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:696:1, end:696:2))"]
#[derive(Clone, Debug)]
struct cpi_instructionVar95 {
    RdHi: u8,
    K8: TableK8,
}
impl cpi_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cpi"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:880:1, end:880:2))"]
#[derive(Clone, Debug)]
struct ldd_instructionVar96 {
    RdFull: u8,
    LddYZq: TableLddYZq,
}
impl ldd_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldd"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.LddYZq
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let LddYZq = if let Some((len, table)) =
            TableLddYZq::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let opbit3 = token_15(tokens_current);
        let RdFull = token_25(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LddYZq, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:885:1, end:885:2))"]
#[derive(Clone, Debug)]
struct ldi_instructionVar97 {
    RdHi: u8,
    K8: TableK8,
}
impl ldi_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldi"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:980:1, end:980:2))"]
#[derive(Clone, Debug)]
struct ori_instructionVar98 {
    RdHi: u8,
    K8: TableK8,
}
impl ori_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ori"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1013:1, end:1013:2))"]
#[derive(Clone, Debug)]
struct rcall_instructionVar99 {
    rel12dst: Tablerel12dst,
}
impl rcall_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rcall"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.rel12dst
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rel12dst = if let Some((len, table)) =
            Tablerel12dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel12dst }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1031:1, end:1031:2))"]
#[derive(Clone, Debug)]
struct rjmp_instructionVar100 {
    rel12dst: Tablerel12dst,
}
impl rjmp_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rjmp"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.rel12dst
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rel12dst = if let Some((len, table)) =
            Tablerel12dst::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel12dst }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1049:1, end:1049:2))"]
#[derive(Clone, Debug)]
struct sbci_instructionVar101 {
    RdHi: u8,
    K8: TableK8,
}
impl sbci_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sbci"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1151:1, end:1151:2))"]
#[derive(Clone, Debug)]
struct std_instructionVar102 {
    RdFull: u8,
    StdYZq: TableStdYZq,
}
impl std_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("std"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.StdYZq
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.RdFull),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let StdYZq = if let Some((len, table)) =
            TableStdYZq::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdFull = token_25(tokens_current);
        let opbit3 = token_15(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { StdYZq, RdFull }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1174:1, end:1174:2))"]
#[derive(Clone, Debug)]
struct subi_instructionVar103 {
    RdHi: u8,
    K8: TableK8,
}
impl subi_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subi"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.RdHi),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.K8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let K8 = if let Some((len, table)) =
            TableK8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let RdHi = token_31(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { K8, RdHi }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(instructionVar0),
    Var1(instructionVar1),
    Var2(break_instructionVar2),
    Var3(clc_instructionVar3),
    Var4(clh_instructionVar4),
    Var5(cli_instructionVar5),
    Var6(cln_instructionVar6),
    Var7(cls_instructionVar7),
    Var8(clt_instructionVar8),
    Var9(clv_instructionVar9),
    Var10(clz_instructionVar10),
    Var11(icall_instructionVar11),
    Var12(ijmp_instructionVar12),
    Var13(lpm_instructionVar13),
    Var14(nop_instructionVar14),
    Var15(rcall_instructionVar15),
    Var16(ret_instructionVar16),
    Var17(reti_instructionVar17),
    Var18(sleep_instructionVar18),
    Var19(spm_instructionVar19),
    Var20(spm_instructionVar20),
    Var21(wdr_instructionVar21),
    Var22(bclr_instructionVar22),
    Var23(bset_instructionVar23),
    Var24(des_instructionVar24),
    Var25(fracmul_instructionVar25),
    Var26(fracmuls_instructionVar26),
    Var27(fracmulsu_instructionVar27),
    Var28(mulsu_instructionVar28),
    Var29(ser_instructionVar29),
    Var30(adiw_instructionVar30),
    Var31(asr_instructionVar31),
    Var32(dec_instructionVar32),
    Var33(in_instructionVar33),
    Var34(in_instructionVar34),
    Var35(in_instructionVar35),
    Var36(inc_instructionVar36),
    Var37(lac_instructionVar37),
    Var38(las_instructionVar38),
    Var39(lat_instructionVar39),
    Var40(ld_instructionVar40),
    Var41(lds_instructionVar41),
    Var42(lpm_instructionVar42),
    Var43(lpm_instructionVar43),
    Var44(lsr_instructionVar44),
    Var45(neg_instructionVar45),
    Var46(out_instructionVar46),
    Var47(out_instructionVar47),
    Var48(out_instructionVar48),
    Var49(pop_instructionVar49),
    Var50(push_instructionVar50),
    Var51(ror_instructionVar51),
    Var52(st_instructionVar52),
    Var53(sts_instructionVar53),
    Var54(swap_instructionVar54),
    Var55(xch_instructionVar55),
    Var56(call_instructionVar56),
    Var57(jmp_instructionVar57),
    Var58(bld_instructionVar58),
    Var59(bst_instructionVar59),
    Var60(cbi_instructionVar60),
    Var61(ld_instructionVar61),
    Var62(st_instructionVar62),
    Var63(ld_instructionVar63),
    Var64(ld_instructionVar64),
    Var65(movw_instructionVar65),
    Var66(muls_instructionVar66),
    Var67(sbi_instructionVar67),
    Var68(sbic_instructionVar68),
    Var69(sbis_instructionVar69),
    Var70(sbiw_instructionVar70),
    Var71(sbrc_instructionVar71),
    Var72(sbrs_instructionVar72),
    Var73(st_instructionVar73),
    Var74(st_instructionVar74),
    Var75(com_instructionVar75),
    Var76(adc_instructionVar76),
    Var77(add_instructionVar77),
    Var78(and_instructionVar78),
    Var79(brbc_instructionVar79),
    Var80(brbs_instructionVar80),
    Var81(cp_instructionVar81),
    Var82(cpc_instructionVar82),
    Var83(cpse_instructionVar83),
    Var84(eor_instructionVar84),
    Var85(mov_instructionVar85),
    Var86(mul_instructionVar86),
    Var87(or_instructionVar87),
    Var88(sbc_instructionVar88),
    Var89(sub_instructionVar89),
    Var90(in_instructionVar90),
    Var91(lds_instructionVar91),
    Var92(out_instructionVar92),
    Var93(sts_instructionVar93),
    Var94(andi_instructionVar94),
    Var95(cpi_instructionVar95),
    Var96(ldd_instructionVar96),
    Var97(ldi_instructionVar97),
    Var98(ori_instructionVar98),
    Var99(rcall_instructionVar99),
    Var100(rjmp_instructionVar100),
    Var101(sbci_instructionVar101),
    Var102(std_instructionVar102),
    Var103(subi_instructionVar103),
}
impl Tableinstruction {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var2(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var3(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var4(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var5(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var6(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var7(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var8(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var9(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var10(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var11(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var12(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var13(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var14(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var15(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var16(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var17(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var18(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var19(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var20(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var21(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var22(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var23(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var24(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var25(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var26(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var27(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var28(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var29(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var30(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var31(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var32(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var33(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var34(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var35(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var36(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var37(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var38(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var39(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var40(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var41(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var42(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var43(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var44(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var45(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var46(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var47(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var48(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var49(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var50(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var51(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var52(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var53(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var54(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var55(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var56(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var57(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var58(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var59(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var60(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var61(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var62(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var63(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var64(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var65(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var66(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var67(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var68(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var69(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var70(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var71(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var72(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var73(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var74(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var75(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var76(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var77(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var78(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var79(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var80(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var81(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var82(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var83(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var84(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var85(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var86(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var87(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var88(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var89(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var90(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var91(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var92(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var93(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var94(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var95(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var96(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var97(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var98(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var99(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var100(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var101(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var102(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var103(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && context_param.0 & 3 == 0 {
            if let Some((inst_len, parsed)) =
                instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 3 == 1 {
            if let Some((inst_len, parsed)) =
                instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 152
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                break_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 136
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                clc_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 216
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                clh_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 248
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                cli_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 168
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                cln_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 200
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                cls_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 232
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                clt_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 184
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                clv_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 152
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                clz_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 9
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                icall_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 9
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                ijmp_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 200
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                lpm_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 0
            && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                nop_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 0
            && (tokens_param[1] & 255) == 208
        {
            if let Some((inst_len, parsed)) =
                rcall_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 8
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                ret_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 24
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                reti_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 136
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                sleep_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 232
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                spm_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 248
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                spm_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 255) == 168
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                wdr_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 143) == 132
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                bclr_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                bset_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 11
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                des_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 136) == 8
            && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                fracmul_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 136) == 128
            && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                fracmuls_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 136) == 136
            && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                fracmulsu_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 136) == 0
            && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                mulsu_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 15
            && (tokens_param[1] & 255) == 239
        {
            if let Some((inst_len, parsed)) =
                ser_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 150 {
            if let Some((inst_len, parsed)) =
                adiw_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 5
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                asr_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 10
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                dec_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 13
            && (tokens_param[1] & 254) == 182
        {
            if let Some((inst_len, parsed)) =
                in_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 14
            && (tokens_param[1] & 254) == 182
        {
            if let Some((inst_len, parsed)) =
                in_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 15
            && (tokens_param[1] & 254) == 182
        {
            if let Some((inst_len, parsed)) =
                in_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 3
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                inc_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 6
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                lac_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 5
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                las_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 7
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                lat_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 12
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                ld_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 0
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 4
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                lpm_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 5
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                lpm_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 6
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                lsr_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 1
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                neg_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 13
            && (tokens_param[1] & 254) == 190
        {
            if let Some((inst_len, parsed)) =
                out_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 14
            && (tokens_param[1] & 254) == 190
        {
            if let Some((inst_len, parsed)) =
                out_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 15
            && (tokens_param[1] & 254) == 190
        {
            if let Some((inst_len, parsed)) =
                out_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 15
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                pop_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 15
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                push_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 7
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                ror_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 12
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                st_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 0
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 2
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                swap_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 15) == 4
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                xch_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 14) == 14
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                call_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 14) == 12
            && (tokens_param[1] & 254) == 148
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 8) == 0
            && (tokens_param[1] & 254) == 248
        {
            if let Some((inst_len, parsed)) =
                bld_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 8) == 0
            && (tokens_param[1] & 254) == 250
        {
            if let Some((inst_len, parsed)) =
                bst_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 152 {
            if let Some((inst_len, parsed)) =
                cbi_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 7) == 0
            && (tokens_param[1] & 254) == 128
        {
            if let Some((inst_len, parsed)) =
                ld_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 7) == 0
            && (tokens_param[1] & 254) == 130
        {
            if let Some((inst_len, parsed)) =
                st_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 3) == 1
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                ld_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 3) == 2
            && (tokens_param[1] & 254) == 144
        {
            if let Some((inst_len, parsed)) =
                ld_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                movw_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                muls_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 154 {
            if let Some((inst_len, parsed)) =
                sbi_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 153 {
            if let Some((inst_len, parsed)) =
                sbic_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 155 {
            if let Some((inst_len, parsed)) =
                sbis_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 255) == 151 {
            if let Some((inst_len, parsed)) =
                sbiw_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 8) == 0
            && (tokens_param[1] & 254) == 252
        {
            if let Some((inst_len, parsed)) =
                sbrc_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 8) == 0
            && (tokens_param[1] & 254) == 254
        {
            if let Some((inst_len, parsed)) =
                sbrs_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 3) == 1
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                st_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 2 == 2
            && (tokens_param[0] & 3) == 2
            && (tokens_param[1] & 254) == 146
        {
            if let Some((inst_len, parsed)) =
                st_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 254) == 148 {
            if let Some((inst_len, parsed)) =
                com_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 28 {
            if let Some((inst_len, parsed)) =
                adc_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 12 {
            if let Some((inst_len, parsed)) =
                add_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 32 {
            if let Some((inst_len, parsed)) =
                and_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 244 {
            if let Some((inst_len, parsed)) =
                brbc_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 240 {
            if let Some((inst_len, parsed)) =
                brbs_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 20 {
            if let Some((inst_len, parsed)) =
                cp_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 4 {
            if let Some((inst_len, parsed)) =
                cpc_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 16 {
            if let Some((inst_len, parsed)) =
                cpse_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 36 {
            if let Some((inst_len, parsed)) =
                eor_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 44 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 156 {
            if let Some((inst_len, parsed)) =
                mul_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 40 {
            if let Some((inst_len, parsed)) =
                or_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 8 {
            if let Some((inst_len, parsed)) =
                sbc_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 252) == 24 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 248) == 176 {
            if let Some((inst_len, parsed)) =
                in_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 248) == 160 {
            if let Some((inst_len, parsed)) =
                lds_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 248) == 184 {
            if let Some((inst_len, parsed)) =
                out_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 248) == 168 {
            if let Some((inst_len, parsed)) =
                sts_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 112 {
            if let Some((inst_len, parsed)) =
                andi_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 48 {
            if let Some((inst_len, parsed)) =
                cpi_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 210) == 128 {
            if let Some((inst_len, parsed)) =
                ldd_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 224 {
            if let Some((inst_len, parsed)) =
                ldi_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 96 {
            if let Some((inst_len, parsed)) =
                ori_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 208 {
            if let Some((inst_len, parsed)) =
                rcall_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 192 {
            if let Some((inst_len, parsed)) =
                rjmp_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 64 {
            if let Some((inst_len, parsed)) =
                sbci_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 210) == 130 {
            if let Some((inst_len, parsed)) =
                std_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 && (tokens_param[1] & 240) == 80 {
            if let Some((inst_len, parsed)) =
                subi_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:324:1, end:324:7))"]
#[derive(Clone, Debug)]
struct RrFullVar0 {
    RrHi: u8,
}
impl RrFullVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.RrHi)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrHi = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:325:1, end:325:7))"]
#[derive(Clone, Debug)]
struct RrFullVar1 {
    RrLow: u8,
}
impl RrFullVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_3_display(self.RrLow)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RrLow = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RrLow }))
    }
}
#[derive(Clone, Debug)]
enum TableRrFull {
    Var0(RrFullVar0),
    Var1(RrFullVar1),
}
impl TableRrFull {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[1] & 2) == 2 {
            if let Some((inst_len, parsed)) =
                RrFullVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 2) == 0 {
            if let Some((inst_len, parsed)) =
                RrFullVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:328:1, end:328:10))"]
#[derive(Clone, Debug)]
struct op1RrPairVar0 {
    op1RrPairHi: u8,
}
impl op1RrPairVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_10_display(self.op1RrPairHi)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        let op1RrPairHi = token_41(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1RrPairHi }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:329:1, end:329:10))"]
#[derive(Clone, Debug)]
struct op1RrPairVar1 {
    op1RrPairLow: u8,
}
impl op1RrPairVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_9_display(self.op1RrPairLow)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        let op1RrPairLow = token_41(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1RrPairLow }))
    }
}
#[derive(Clone, Debug)]
enum Tableop1RrPair {
    Var0(op1RrPairVar0),
    Var1(op1RrPairVar1),
}
impl Tableop1RrPair {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var1(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[1] & 2) == 2 {
            if let Some((inst_len, parsed)) =
                op1RrPairVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[1] & 2) == 0 {
            if let Some((inst_len, parsed)) =
                op1RrPairVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:442:1, end:442:3))"]
#[derive(Clone, Debug)]
struct K8Var0 {
    op8to11: u8,
    op0to3: u8,
}
impl K8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op8to11).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op0to3).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_34(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_19(tokens_current)).unwrap());
        let op0to3 = token_19(tokens_current);
        let op8to11 = token_34(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op8to11, op0to3 }))
    }
}
#[derive(Clone, Debug)]
enum TableK8 {
    Var0(K8Var0),
}
impl TableK8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                K8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:453:1, end:453:9))"]
#[derive(Clone, Debug)]
struct rel7addrVar0 {
    op3to9signed: u8,
}
impl rel7addrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_rel: i128 = 0;
        calc_rel = i128::try_from(
            (if self.op3to9signed & 64 != 0 {
                -1 & !63
            } else {
                0
            } | self.op3to9signed as i8),
        )
        .unwrap()
        .wrapping_add(i128::try_from(inst_next).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_rel.is_negative(),
            calc_rel.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_rel: i128 = 0;
        let mut block_0_len = 2;
        let op3to9signed = token_33(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op3to9signed }))
    }
}
#[derive(Clone, Debug)]
enum Tablerel7addr {
    Var0(rel7addrVar0),
}
impl Tablerel7addr {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                rel7addrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:457:1, end:457:8))"]
#[derive(Clone, Debug)]
struct rel7dstVar0 {
    op3to9signed: u8,
    rel7addr: Tablerel7addr,
}
impl rel7dstVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_byteOffset: i128 = 0;
        calc_byteOffset = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.op3to9signed & 64 != 0 {
                        -1 & !63
                    } else {
                        0
                    } | self.op3to9signed as i8),
                )
                .unwrap()
                .wrapping_add(i128::try_from(inst_next).unwrap())
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_byteOffset.is_negative(),
            calc_byteOffset.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_byteOffset: i128 = 0;
        let mut block_0_len = 2;
        let rel7addr = if let Some((len, table)) =
            Tablerel7addr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op3to9signed = token_33(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rel7addr,
                op3to9signed,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablerel7dst {
    Var0(rel7dstVar0),
}
impl Tablerel7dst {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                rel7dstVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:461:1, end:461:10))"]
#[derive(Clone, Debug)]
struct rel12addrVar0 {
    oplow12signed: u16,
}
impl rel12addrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_rel: i128 = 0;
        calc_rel = i128::try_from(
            (if self.oplow12signed & 2048 != 0 {
                -1 & !2047
            } else {
                0
            } | self.oplow12signed as i16),
        )
        .unwrap()
        .wrapping_add(i128::try_from(inst_start).unwrap())
        .wrapping_add(1i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_rel.is_negative(),
            calc_rel.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_rel: i128 = 0;
        let mut block_0_len = 2;
        calc_rel = i128::try_from(token_18(tokens_current))
            .unwrap()
            .wrapping_add(i128::try_from(inst_start).unwrap())
            .wrapping_add(1i128);
        let oplow12signed = token_18(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { oplow12signed }))
    }
}
#[derive(Clone, Debug)]
enum Tablerel12addr {
    Var0(rel12addrVar0),
}
impl Tablerel12addr {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                rel12addrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:465:1, end:465:9))"]
#[derive(Clone, Debug)]
struct rel12dstVar0 {
    oplow12signed: u16,
    rel12addr: Tablerel12addr,
}
impl rel12dstVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_byteOffset: i128 = 0;
        calc_byteOffset = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.oplow12signed & 2048 != 0 {
                        -1 & !2047
                    } else {
                        0
                    } | self.oplow12signed as i16),
                )
                .unwrap()
                .wrapping_add(i128::try_from(inst_start).unwrap())
                .wrapping_add(1i128)
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_byteOffset.is_negative(),
            calc_byteOffset.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_byteOffset: i128 = 0;
        let mut block_0_len = 2;
        calc_byteOffset = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_18(tokens_current))
                    .unwrap()
                    .wrapping_add(i128::try_from(inst_start).unwrap())
                    .wrapping_add(1i128)
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let rel12addr = if let Some((len, table)) =
            Tablerel12addr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let oplow12signed = token_18(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rel12addr,
                oplow12signed,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablerel12dst {
    Var0(rel12dstVar0),
}
impl Tablerel12dst {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                rel12dstVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:469:1, end:469:10))"]
#[derive(Clone, Debug)]
struct abs22addrVar0 {
    op4to8: u8,
    opbit0: u8,
    next16: u16,
}
impl abs22addrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_loc: i128 = 0;
        calc_loc = ((u32::try_from(17i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op4to8).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| i128::try_from(self.opbit0).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.next16).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_loc.is_negative(),
            calc_loc.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_loc: i128 = 0;
        let mut block_0_len = 2;
        let op4to8 = token_25(tokens_current);
        let opbit0 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_loc = ((u32::try_from(17i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_25(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_17(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_1(tokens_current)).unwrap());
        let next16 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                op4to8,
                opbit0,
                next16,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableabs22addr {
    Var0(abs22addrVar0),
}
impl Tableabs22addr {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 {
            if let Some((inst_len, parsed)) =
                abs22addrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:473:1, end:473:9))"]
#[derive(Clone, Debug)]
struct abs22dstVar0 {
    op4to8: u8,
    opbit0: u8,
    next16: u16,
    abs22addr: Tableabs22addr,
}
impl abs22dstVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_byteOffset: i128 = 0;
        calc_byteOffset = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                ((u32::try_from(17i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.op4to8).unwrap().checked_shl(shl))
                    .unwrap_or(0)
                    | u32::try_from(16i128)
                        .ok()
                        .and_then(|shl| i128::try_from(self.opbit0).unwrap().checked_shl(shl))
                        .unwrap_or(0))
                    | i128::try_from(self.next16).unwrap())
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_byteOffset.is_negative(),
            calc_byteOffset.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_byteOffset: i128 = 0;
        let mut block_0_len = 4;
        calc_byteOffset = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                ((u32::try_from(17i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_25(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0)
                    | u32::try_from(16i128)
                        .ok()
                        .and_then(|shl| {
                            i128::try_from(token_17(tokens_current))
                                .unwrap()
                                .checked_shl(shl)
                        })
                        .unwrap_or(0))
                    | i128::try_from(token_1(tokens_current)).unwrap())
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            let op4to8 = token_25(tokens);
            let opbit0 = token_17(tokens);
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            let mut block_1_len = 2;
            let next16 = token_1(tokens);
            pattern_len += block_1_len;
            tokens = &tokens[usize::try_from(block_1_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (op4to8, opbit0, next16), pattern_len))
        };
        let ((), (op4to8, opbit0, next16), sub_len) =
            sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let abs22addr = if let Some((len, table)) =
            Tableabs22addr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                abs22addr,
                op4to8,
                opbit0,
                next16,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableabs22dst {
    Var0(abs22dstVar0),
}
impl Tableabs22dst {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 {
            if let Some((inst_len, parsed)) =
                abs22dstVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:477:1, end:477:17))"]
#[derive(Clone, Debug)]
struct next16memPtrVal1Var0 {
    next16: u16,
}
impl next16memPtrVal1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.next16 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let next16 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { next16 }))
    }
}
#[derive(Clone, Debug)]
enum Tablenext16memPtrVal1 {
    Var0(next16memPtrVal1Var0),
}
impl Tablenext16memPtrVal1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                next16memPtrVal1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:492:1, end:492:3))"]
#[derive(Clone, Debug)]
struct K6Var0 {
    op6to7: u8,
    oplow4: u8,
}
impl K6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op6to7).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.oplow4).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_27(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_19(tokens_current)).unwrap());
        let op6to7 = token_27(tokens_current);
        let oplow4 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op6to7, oplow4 }))
    }
}
#[derive(Clone, Debug)]
enum TableK6 {
    Var0(K6Var0),
}
impl TableK6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                K6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:495:1, end:495:7))"]
#[derive(Clone, Debug)]
struct K7addrVar0 {
    opbit8: u8,
    op9to10: u8,
    oplow4: u8,
}
impl K7addrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (((u32::try_from(7i128)
            .ok()
            .and_then(|shl| (1i128 ^ i128::try_from(self.opbit8).unwrap()).checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(6i128)
                .ok()
                .and_then(|shl| i128::try_from(self.opbit8).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | u32::try_from(4i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op9to10).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.oplow4).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = (((u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                (1i128 ^ i128::try_from(token_13(tokens_current)).unwrap()).checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(6i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_13(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | u32::try_from(4i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_29(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_19(tokens_current)).unwrap());
        let op9to10 = token_29(tokens_current);
        let opbit8 = token_13(tokens_current);
        let oplow4 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                opbit8,
                op9to10,
                oplow4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableK7addr {
    Var0(K7addrVar0),
}
impl TableK7addr {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                K7addrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:506:1, end:506:5))"]
#[derive(Clone, Debug)]
struct Aio6Var0 {
    op9to10: u8,
    oplow4: u8,
}
impl Aio6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op9to10).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.oplow4).unwrap())
        .wrapping_add(32i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_29(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_19(tokens_current)).unwrap())
        .wrapping_add(32i128);
        let op9to10 = token_29(tokens_current);
        let oplow4 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op9to10, oplow4 }))
    }
}
#[derive(Clone, Debug)]
enum TableAio6 {
    Var0(Aio6Var0),
}
impl TableAio6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                Aio6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:507:1, end:507:5))"]
#[derive(Clone, Debug)]
struct Aio5Var0 {
    op3to7: u8,
}
impl Aio5Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (i128::try_from(self.op3to7).unwrap() | 0i128).wrapping_add(32i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = (i128::try_from(token_24(tokens_current)).unwrap() | 0i128).wrapping_add(32i128);
        let op3to7 = token_24(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op3to7 }))
    }
}
#[derive(Clone, Debug)]
enum TableAio5 {
    Var0(Aio5Var0),
}
impl TableAio5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                Aio5Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:509:1, end:509:3))"]
#[derive(Clone, Debug)]
struct q6Var0 {
    opbit13: u8,
    op10to11: u8,
    oplow3: u8,
}
impl q6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = ((u32::try_from(5i128)
            .ok()
            .and_then(|shl| i128::try_from(self.opbit13).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(3i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op10to11).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.oplow3).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_val.is_negative(),
            calc_val.abs() as u64,
        )];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_val: i128 = 0;
        let mut block_0_len = 2;
        calc_val = ((u32::try_from(5i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_9(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(3i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_30(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_20(tokens_current)).unwrap());
        let opbit13 = token_9(tokens_current);
        let op10to11 = token_30(tokens_current);
        let oplow3 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                opbit13,
                op10to11,
                oplow3,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableq6 {
    Var0(q6Var0),
}
impl Tableq6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                q6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:864:1, end:864:7))"]
#[derive(Clone, Debug)]
struct LdPlusVar0 {
    RstPtr: u8,
}
impl LdPlusVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            meaning_7_display(self.RstPtr),
            <DisplayElement>::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstPtr }))
    }
}
#[derive(Clone, Debug)]
enum TableLdPlus {
    Var0(LdPlusVar0),
}
impl TableLdPlus {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                LdPlusVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:870:1, end:870:9))"]
#[derive(Clone, Debug)]
struct LdPredecVar0 {
    RstPtr: u8,
}
impl LdPredecVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("-"),
            meaning_7_display(self.RstPtr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstPtr }))
    }
}
#[derive(Clone, Debug)]
enum TableLdPredec {
    Var0(LdPredecVar0),
}
impl TableLdPredec {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                LdPredecVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:879:1, end:879:7))"]
#[derive(Clone, Debug)]
struct LddYZqVar0 {
    Rstq: u8,
    q6: Tableq6,
}
impl LddYZqVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] =
            [meaning_6_display(self.Rstq), <DisplayElement>::Literal("+")];
        display.extend_from_slice(&extend);
        self.q6
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let q6 = if let Some((len, table)) =
            Tableq6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rstq = token_15(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { q6, Rstq }))
    }
}
#[derive(Clone, Debug)]
enum TableLddYZq {
    Var0(LddYZqVar0),
}
impl TableLddYZq {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && context_param.0 & 2 == 2 {
            if let Some((inst_len, parsed)) =
                LddYZqVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:927:1, end:927:8))"]
#[derive(Clone, Debug)]
struct LpmPlusVar0 {}
impl LpmPlusVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::Z),
            <DisplayElement>::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableLpmPlus {
    Var0(LpmPlusVar0),
}
impl TableLpmPlus {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 0 {
            if let Some((inst_len, parsed)) =
                LpmPlusVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1115:1, end:1115:8))"]
#[derive(Clone, Debug)]
struct SpmPlusVar0 {}
impl SpmPlusVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::Z),
            <DisplayElement>::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableSpmPlus {
    Var0(SpmPlusVar0),
}
impl TableSpmPlus {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 0 {
            if let Some((inst_len, parsed)) =
                SpmPlusVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1135:1, end:1135:7))"]
#[derive(Clone, Debug)]
struct StPlusVar0 {
    RstPtr: u8,
}
impl StPlusVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            meaning_7_display(self.RstPtr),
            <DisplayElement>::Literal("+"),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstPtr }))
    }
}
#[derive(Clone, Debug)]
enum TableStPlus {
    Var0(StPlusVar0),
}
impl TableStPlus {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                StPlusVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1141:1, end:1141:9))"]
#[derive(Clone, Debug)]
struct StPredecVar0 {
    RstPtr: u8,
}
impl StPredecVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("-"),
            meaning_7_display(self.RstPtr),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RstPtr = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstPtr }))
    }
}
#[derive(Clone, Debug)]
enum TableStPredec {
    Var0(StPredecVar0),
}
impl TableStPredec {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                StPredecVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Atmel/data/languages/avr8.sinc, start:1150:1, end:1150:7))"]
#[derive(Clone, Debug)]
struct StdYZqVar0 {
    Rstq: u8,
    q6: Tableq6,
}
impl StdYZqVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] =
            [meaning_6_display(self.Rstq), <DisplayElement>::Literal("+")];
        display.extend_from_slice(&extend);
        self.q6
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let q6 = if let Some((len, table)) =
            Tableq6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rstq = token_15(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { q6, Rstq }))
    }
}
#[derive(Clone, Debug)]
enum TableStdYZq {
    Var0(StdYZqVar0),
}
impl TableStdYZq {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set_param: &mut GlobalSet,
    ) {
        match self {
            Self::Var0(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                StdYZqVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
pub fn parse_instruction(
    tokens: &[u8],
    context: &mut ContextMemory,
    inst_start: AddrType,
    global_set: &mut GlobalSet,
) -> Option<(u16, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
