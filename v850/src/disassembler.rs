pub type AddrType = u32;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r0,
    r1,
    r2,
    sp,
    gp,
    tp,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
    r16,
    r17,
    r18,
    r19,
    r20,
    r21,
    r22,
    r23,
    r24,
    r25,
    r26,
    r27,
    r28,
    r29,
    ep,
    lp,
    EIPC,
    EIPSW,
    FEPC,
    FEPSW,
    ECR,
    PSW,
    FPSR,
    FPEPC,
    FPST,
    FPCC,
    FPCFG,
    SCCFG,
    SCBP,
    EIIC,
    FEIC,
    DBIC,
    CTPC,
    CTPSW,
    DBPC,
    DBPSW,
    CTBP,
    DIR,
    DBG22,
    DBG23,
    DBG24,
    DBG25,
    DBG26,
    DBG27,
    EIWR,
    FEWR,
    DBWR,
    BSEL,
    r0r1,
    r2sp,
    r4r5,
    r6r7,
    r8r9,
    r10r11,
    r12r13,
    r14r15,
    r16r17,
    r18r19,
    r20r21,
    r22r23,
    r24r25,
    r26r27,
    r28r29,
    eplp,
    PC,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::r0 => "r0",
            Self::r1 => "r1",
            Self::r2 => "r2",
            Self::sp => "sp",
            Self::gp => "gp",
            Self::tp => "tp",
            Self::r6 => "r6",
            Self::r7 => "r7",
            Self::r8 => "r8",
            Self::r9 => "r9",
            Self::r10 => "r10",
            Self::r11 => "r11",
            Self::r12 => "r12",
            Self::r13 => "r13",
            Self::r14 => "r14",
            Self::r15 => "r15",
            Self::r16 => "r16",
            Self::r17 => "r17",
            Self::r18 => "r18",
            Self::r19 => "r19",
            Self::r20 => "r20",
            Self::r21 => "r21",
            Self::r22 => "r22",
            Self::r23 => "r23",
            Self::r24 => "r24",
            Self::r25 => "r25",
            Self::r26 => "r26",
            Self::r27 => "r27",
            Self::r28 => "r28",
            Self::r29 => "r29",
            Self::ep => "ep",
            Self::lp => "lp",
            Self::EIPC => "EIPC",
            Self::EIPSW => "EIPSW",
            Self::FEPC => "FEPC",
            Self::FEPSW => "FEPSW",
            Self::ECR => "ECR",
            Self::PSW => "PSW",
            Self::FPSR => "FPSR",
            Self::FPEPC => "FPEPC",
            Self::FPST => "FPST",
            Self::FPCC => "FPCC",
            Self::FPCFG => "FPCFG",
            Self::SCCFG => "SCCFG",
            Self::SCBP => "SCBP",
            Self::EIIC => "EIIC",
            Self::FEIC => "FEIC",
            Self::DBIC => "DBIC",
            Self::CTPC => "CTPC",
            Self::CTPSW => "CTPSW",
            Self::DBPC => "DBPC",
            Self::DBPSW => "DBPSW",
            Self::CTBP => "CTBP",
            Self::DIR => "DIR",
            Self::DBG22 => "DBG22",
            Self::DBG23 => "DBG23",
            Self::DBG24 => "DBG24",
            Self::DBG25 => "DBG25",
            Self::DBG26 => "DBG26",
            Self::DBG27 => "DBG27",
            Self::EIWR => "EIWR",
            Self::FEWR => "FEWR",
            Self::DBWR => "DBWR",
            Self::BSEL => "BSEL",
            Self::r0r1 => "r0r1",
            Self::r2sp => "r2sp",
            Self::r4r5 => "r4r5",
            Self::r6r7 => "r6r7",
            Self::r8r9 => "r8r9",
            Self::r10r11 => "r10r11",
            Self::r12r13 => "r12r13",
            Self::r14r15 => "r14r15",
            Self::r16r17 => "r16r17",
            Self::r18r19 => "r18r19",
            Self::r20r21 => "r20r21",
            Self::r22r23 => "r22r23",
            Self::r24r25 => "r24r25",
            Self::r26r27 => "r26r27",
            Self::r28r29 => "r28r29",
            Self::eplp => "eplp",
            Self::PC => "PC",
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
        0 => Register::r0,
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::sp,
        4 => Register::gp,
        5 => Register::tp,
        6 => Register::r6,
        7 => Register::r7,
        8 => Register::r8,
        9 => Register::r9,
        10 => Register::r10,
        11 => Register::r11,
        12 => Register::r12,
        13 => Register::r13,
        14 => Register::r14,
        15 => Register::r15,
        16 => Register::r16,
        17 => Register::r17,
        18 => Register::r18,
        19 => Register::r19,
        20 => Register::r20,
        21 => Register::r21,
        22 => Register::r22,
        23 => Register::r23,
        24 => Register::r24,
        25 => Register::r25,
        26 => Register::r26,
        27 => Register::r27,
        28 => Register::r28,
        29 => Register::r29,
        30 => Register::ep,
        31 => Register::lp,
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
        0 => Register::r0r1,
        2 => Register::r2sp,
        4 => Register::r4r5,
        6 => Register::r6r7,
        8 => Register::r8r9,
        10 => Register::r10r11,
        12 => Register::r12r13,
        14 => Register::r14r15,
        16 => Register::r16r17,
        18 => Register::r18r19,
        20 => Register::r20r21,
        22 => Register::r22r23,
        24 => Register::r24r25,
        26 => Register::r26r27,
        28 => Register::r28r29,
        30 => Register::eplp,
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
        0 => Register::EIPC,
        1 => Register::EIPSW,
        2 => Register::FEPC,
        3 => Register::FEPSW,
        4 => Register::ECR,
        5 => Register::PSW,
        6 => Register::FPSR,
        7 => Register::FPEPC,
        8 => Register::FPST,
        9 => Register::FPCC,
        10 => Register::FPCFG,
        11 => Register::SCCFG,
        12 => Register::SCBP,
        13 => Register::EIIC,
        14 => Register::FEIC,
        15 => Register::DBIC,
        16 => Register::CTPC,
        17 => Register::CTPSW,
        18 => Register::DBPC,
        19 => Register::DBPSW,
        20 => Register::CTBP,
        21 => Register::DIR,
        22 => Register::DBG22,
        23 => Register::DBG23,
        24 => Register::DBG24,
        25 => Register::DBG25,
        26 => Register::DBG26,
        27 => Register::DBG27,
        28 => Register::EIWR,
        29 => Register::FEWR,
        30 => Register::DBWR,
        31 => Register::BSEL,
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
        0 => Register::r0,
        1 => Register::r2,
        2 => Register::gp,
        3 => Register::r6,
        4 => Register::r8,
        5 => Register::r10,
        6 => Register::r12,
        7 => Register::r14,
        8 => Register::r16,
        9 => Register::r18,
        10 => Register::r20,
        11 => Register::r22,
        12 => Register::r24,
        13 => Register::r26,
        14 => Register::r28,
        15 => Register::ep,
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
        0 => Register::r1,
        1 => Register::sp,
        2 => Register::tp,
        3 => Register::r7,
        4 => Register::r9,
        5 => Register::r11,
        6 => Register::r13,
        7 => Register::r15,
        8 => Register::r17,
        9 => Register::r19,
        10 => Register::r21,
        11 => Register::r23,
        12 => Register::r25,
        13 => Register::r27,
        14 => Register::r29,
        15 => Register::lp,
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
fn meaning_5_display<T>(num: T) -> DisplayElement
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => <DisplayElement>::Literal("f"),
        1 => <DisplayElement>::Literal("un"),
        2 => <DisplayElement>::Literal("eq"),
        3 => <DisplayElement>::Literal("ueq"),
        4 => <DisplayElement>::Literal("olt"),
        5 => <DisplayElement>::Literal("ult"),
        6 => <DisplayElement>::Literal("ole"),
        7 => <DisplayElement>::Literal("ule"),
        8 => <DisplayElement>::Literal("sd"),
        9 => <DisplayElement>::Literal("ngle"),
        10 => <DisplayElement>::Literal("seq"),
        11 => <DisplayElement>::Literal("ngl"),
        12 => <DisplayElement>::Literal("lt"),
        13 => <DisplayElement>::Literal("nge"),
        14 => <DisplayElement>::Literal("le"),
        15 => <DisplayElement>::Literal("ngt"),
        _ => unreachable!("Invalid Attach Value"),
    }
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
#[doc = "Create token_fields: op2323"]
fn token_29(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 7) & 1) as u8)
}
#[doc = "Create token_fields: op0003 op1619"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: op0004 _R0004 SR0004 R0004x2 s0004 op1620 R1620 R1620x2"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 31) as u8)
}
#[doc = "Create token_fields: prep27"]
fn token_43(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 27) & 1) as u8)
}
#[doc = "Create token_fields: prep2527"]
fn token_49(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 25) & 7) as u8)
}
#[doc = "Create token_fields: prep2931"]
fn token_46(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 29) & 7) as u8)
}
#[doc = "Create token_fields: prep21"]
fn token_35(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 21) & 1) as u8)
}
#[doc = "Create token_fields: prep2223"]
fn token_37(tokens: &[u8]) -> u16 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 22) & 1023) as u16)
}
#[doc = "Create token_fields: prep1620"]
fn token_34(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 31) as u8)
}
#[doc = "Create token_fields: prep31"]
fn token_52(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 31) & 1) as u8)
}
#[doc = "Create token_fields: op1720 op0_1720 op1_1720"]
fn token_24(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 15) as u8)
}
#[doc = "Create token_fields: prep26"]
fn token_42(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 26) & 1) as u8)
}
#[doc = "Create token_fields: op0006"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 127) as u8)
}
#[doc = "Create token_fields: op0515"]
fn token_13(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 2047) as u16)
}
#[doc = "Create token_fields: op2426"]
fn token_30(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 7) as u8)
}
#[doc = "Create token_fields: prep0105"]
fn token_32(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 1) & 31) as u8)
}
#[doc = "Create token_fields: op1617"]
fn token_22(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 3) as u8)
}
#[doc = "Create token_fields: op0505"]
fn token_11(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 1) as u8)
}
#[doc = "Create token_fields: prep24"]
fn token_39(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 1) as u8)
}
#[doc = "Create token_fields: fcbit1719"]
fn token_23(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 7) as u8)
}
#[doc = "Create token_fields: prep25"]
fn token_41(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 25) & 1) as u8)
}
#[doc = "Create token_fields: prep3031"]
fn token_47(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 30) & 3) as u8)
}
#[doc = "Create token_fields: op2122"]
fn token_28(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 3) as u8)
}
#[doc = "Create token_fields: op0000 op1616"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: prep22"]
fn token_36(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 22) & 1) as u8)
}
#[doc = "Create token_fields: op1114 fcond2730"]
fn token_18(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 11) & 15) as u8)
}
#[doc = "Create token_fields: prep2627"]
fn token_50(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 26) & 3) as u8)
}
#[doc = "Create token_fields: op0510 op2126"]
fn token_12(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 63) as u8)
}
#[doc = "Create token_fields: op1515 op3131"]
fn token_21(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 15) & 1) as u8)
}
#[doc = "Create token_fields: op0710"]
fn token_16(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 7) & 15) as u8)
}
#[doc = "Create token_fields: prep0615"]
fn token_33(tokens: &[u8]) -> u16 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 6) & 1023) as u16)
}
#[doc = "Create token_fields: op0015 op1631 s1631 op3247 s3247 op4863"]
fn token_7(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: op0410 op2026"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 127) as u8)
}
#[doc = "Create token_fields: op0615"]
fn token_15(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 6) & 1023) as u16)
}
#[doc = "Create token_fields: op2020"]
fn token_27(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: prep2431"]
fn token_40(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 255) as u8)
}
#[doc = "Create token_fields: prep2427"]
fn token_48(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 15) as u8)
}
#[doc = "Create token_fields: op1115 _R1115 SR1115 R1115x2 s1115 op2731 _R2731 R2731x2"]
fn token_19(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 11) & 31) as u8)
}
#[doc = "Create token_fields: prep23"]
fn token_38(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 23) & 1) as u8)
}
#[doc = "Create token_fields: op1415 op3031"]
fn token_20(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 14) & 3) as u8)
}
#[doc = "Create token_fields: s0005 op0005"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 63) as u8)
}
#[doc = "Create token_fields: op0106"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 63) as u8)
}
#[doc = "Create token_fields: op0610 op2226"]
fn token_14(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 6) & 31) as u8)
}
#[doc = "Create token_fields: op1821 s1821"]
fn token_26(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 2) & 15) as u8)
}
#[doc = "Create token_fields: op0406"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 7) as u8)
}
#[doc = "Create token_fields: op1113 op2729"]
fn token_17(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 11) & 7) as u8)
}
#[doc = "Create token_fields: s1731"]
fn token_25(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 32767) as u16)
}
#[doc = "Create token_fields: prep00"]
fn token_31(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: prep28"]
fn token_44(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 28) & 1) as u8)
}
#[doc = "Create token_fields: prep30"]
fn token_51(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 30) & 1) as u8)
}
#[doc = "Create token_fields: prep29"]
fn token_45(tokens: &[u8]) -> u8 {
    (((u32::from_le_bytes(tokens[0..4].try_into().unwrap()) >> 29) & 1) as u8)
}
#[doc = "Create token_fields: op0010 op1626"]
fn token_6(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 2047) as u16)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub ());
#[derive(Clone, Copy, Default)]
pub struct GlobalSet(());
impl GlobalSet {
    pub fn new(_: ContextMemory) -> Self {
        Self(())
    }
    pub fn set(&mut self, _: Option<AddrType>, _: impl FnOnce(&mut ContextMemory)) {
        unreachable!()
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:142:1, end:142:2))"]
#[derive(Clone, Debug)]
struct di_instructionVar0 {}
impl di_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("di"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct ei_instructionVar1 {}
impl ei_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ei"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct halt_instructionVar2 {}
impl halt_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("halt"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:7:1, end:7:2))"]
#[derive(Clone, Debug)]
struct absf_d_instructionVar3 {
    R1115x2: u8,
    R2731x2: u8,
}
impl absf_d_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("absf.d"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct absf_s_instructionVar4 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl absf_s_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("absf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:31:1, end:31:2))"]
#[derive(Clone, Debug)]
struct ceilf_dl_instructionVar5 {
    R1115x2: u8,
    R2731x2: u8,
}
impl ceilf_dl_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.dl"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:38:1, end:38:2))"]
#[derive(Clone, Debug)]
struct ceilf_dul_instructionVar6 {
    R1115x2: u8,
    R2731x2: u8,
}
impl ceilf_dul_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.dul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:45:1, end:45:2))"]
#[derive(Clone, Debug)]
struct ceilf_duw_instructionVar7 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl ceilf_duw_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.duw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct ceilf_dw_instructionVar8 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl ceilf_dw_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.dw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:57:1, end:57:2))"]
#[derive(Clone, Debug)]
struct ceilf_sl_instructionVar9 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl ceilf_sl_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.sl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:64:1, end:64:2))"]
#[derive(Clone, Debug)]
struct ceilf_sul_instructionVar10 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl ceilf_sul_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.sul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:71:1, end:71:2))"]
#[derive(Clone, Debug)]
struct ceilf_sul_instructionVar11 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl ceilf_sul_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.sul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct ceilf_sw_instructionVar12 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl ceilf_sw_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ceilf.sw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:126:1, end:126:2))"]
#[derive(Clone, Debug)]
struct cvtf_dl_instructionVar13 {
    R1115x2: u8,
    R2731x2: u8,
}
impl cvtf_dl_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.dl"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:132:1, end:132:2))"]
#[derive(Clone, Debug)]
struct cvtf_ds_instructionVar14 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl cvtf_ds_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.ds"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:138:1, end:138:2))"]
#[derive(Clone, Debug)]
struct cvtf_dul_instructionVar15 {
    R1115x2: u8,
    R2731x2: u8,
}
impl cvtf_dul_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.dul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:144:1, end:144:2))"]
#[derive(Clone, Debug)]
struct cvtf_duw_instructionVar16 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl cvtf_duw_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.duw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:150:1, end:150:2))"]
#[derive(Clone, Debug)]
struct cvtf_sw_instructionVar17 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl cvtf_sw_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.sw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct cvtf_ls_instructionVar18 {
    R1115x2: u8,
    R2731x2: u8,
}
impl cvtf_ls_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.ls"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct cvtf_ls_instructionVar19 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl cvtf_ls_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.ls"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:168:1, end:168:2))"]
#[derive(Clone, Debug)]
struct cvtf_sd_instructionVar20 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl cvtf_sd_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.sd"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:174:1, end:174:2))"]
#[derive(Clone, Debug)]
struct cvtf_sl_instructionVar21 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl cvtf_sl_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.sl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct cvtf_sul_instructionVar22 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl cvtf_sul_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.sul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:186:1, end:186:2))"]
#[derive(Clone, Debug)]
struct cvtf_suw_instructionVar23 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_suw_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.suw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:192:1, end:192:2))"]
#[derive(Clone, Debug)]
struct cvtf_sw_instructionVar24 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_sw_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.sw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:198:1, end:198:2))"]
#[derive(Clone, Debug)]
struct cvtf_uls_instructionVar25 {
    R1115x2: u8,
    R2731x2: u8,
}
impl cvtf_uls_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.uls"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct cvtf_uls_instructionVar26 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl cvtf_uls_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.uls"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct cvtf_uwd_instructionVar27 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl cvtf_uwd_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.uwd"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:216:1, end:216:2))"]
#[derive(Clone, Debug)]
struct cvtf_uws_instructionVar28 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_uws_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.uws"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct cvtf_wd_instructionVar29 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl cvtf_wd_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.wd"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:228:1, end:228:2))"]
#[derive(Clone, Debug)]
struct cvtf_ws_instructionVar30 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cvtf_ws_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cvtf.ws"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:246:1, end:246:2))"]
#[derive(Clone, Debug)]
struct floorf_dl_instructionVar31 {
    R1115x2: u8,
    R2731x2: u8,
}
impl floorf_dl_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.dl"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:253:1, end:253:2))"]
#[derive(Clone, Debug)]
struct floorf_dul_instructionVar32 {
    R1115x2: u8,
    R2731x2: u8,
}
impl floorf_dul_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.dul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:260:1, end:260:2))"]
#[derive(Clone, Debug)]
struct floorf_duw_instructionVar33 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl floorf_duw_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.duw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:266:1, end:266:2))"]
#[derive(Clone, Debug)]
struct floorf_dw_instructionVar34 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl floorf_dw_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.dw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct floorf_sl_instructionVar35 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl floorf_sl_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.sl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:279:1, end:279:2))"]
#[derive(Clone, Debug)]
struct floorf_sul_instructionVar36 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl floorf_sul_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.sul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct floorf_suw_instructionVar37 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl floorf_suw_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.suw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:292:1, end:292:2))"]
#[derive(Clone, Debug)]
struct floorf_suw_instructionVar38 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl floorf_suw_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("floorf.suw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:374:1, end:374:2))"]
#[derive(Clone, Debug)]
struct negf_d_instructionVar39 {
    R1115x2: u8,
    R2731x2: u8,
}
impl negf_d_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("negf.d"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct negf_s_instructionVar40 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl negf_s_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("negf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:398:1, end:398:2))"]
#[derive(Clone, Debug)]
struct recipf_d_instructionVar41 {
    R1115x2: u8,
    R2731x2: u8,
}
impl recipf_d_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("recipf.d"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct recipf_s_instructionVar42 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl recipf_s_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("recipf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:410:1, end:410:2))"]
#[derive(Clone, Debug)]
struct rsqrtf_d_instructionVar43 {
    R1115x2: u8,
    R2731x2: u8,
}
impl rsqrtf_d_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsqrtf.d"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:416:1, end:416:2))"]
#[derive(Clone, Debug)]
struct rsqrtf_s_instructionVar44 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl rsqrtf_s_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsqrtf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:422:1, end:422:2))"]
#[derive(Clone, Debug)]
struct sqrtf_d_instructionVar45 {
    R1115x2: u8,
    R2731x2: u8,
}
impl sqrtf_d_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sqrtf.d"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:428:1, end:428:2))"]
#[derive(Clone, Debug)]
struct sqrtf_s_instructionVar46 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sqrtf_s_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sqrtf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct trncf_dl_instructionVar47 {
    R1115x2: u8,
    R2731x2: u8,
}
impl trncf_dl_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.dl"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:459:1, end:459:2))"]
#[derive(Clone, Debug)]
struct trncf_dul_instructionVar48 {
    R1115x2: u8,
    R2731x2: u8,
}
impl trncf_dul_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.dul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115x2, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:465:1, end:465:2))"]
#[derive(Clone, Debug)]
struct trncf_duw_instructionVar49 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl trncf_duw_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.duw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:471:1, end:471:2))"]
#[derive(Clone, Debug)]
struct trncf_dw_instructionVar50 {
    R1115x2: u8,
    R2731: TableR2731,
}
impl trncf_dw_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.dw"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R2731, R1115x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct trncf_sl_instructionVar51 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl trncf_sl_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.sl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:483:1, end:483:2))"]
#[derive(Clone, Debug)]
struct trncf_sul_instructionVar52 {
    R2731x2: u8,
    R1115: TableR1115,
}
impl trncf_sul_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.sul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731x2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:489:1, end:489:2))"]
#[derive(Clone, Debug)]
struct trncf_suw_instructionVar53 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl trncf_suw_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.suw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:495:1, end:495:2))"]
#[derive(Clone, Debug)]
struct trncf_sw_instructionVar54 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl trncf_sw_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trncf.sw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:221:1, end:221:2))"]
#[derive(Clone, Debug)]
struct bsh_instructionVar55 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl bsh_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bsh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:235:1, end:235:2))"]
#[derive(Clone, Debug)]
struct bsw_instructionVar56 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl bsw_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bsw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:261:1, end:261:2))"]
#[derive(Clone, Debug)]
struct hsh_instructionVar57 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl hsh_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("hsh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:271:1, end:271:2))"]
#[derive(Clone, Debug)]
struct hsw_instructionVar58 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl hsw_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("hsw"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:299:1, end:299:2))"]
#[derive(Clone, Debug)]
struct sasf_instructionVar59 {
    R1115: TableR1115,
    c0003: Tablec0003,
}
impl sasf_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sasf"));
        self.c0003
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
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
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, c0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:305:1, end:305:2))"]
#[derive(Clone, Debug)]
struct setf_instructionVar60 {
    R1115: TableR1115,
    c0003: Tablec0003,
}
impl setf_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("setf"));
        self.c0003
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
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
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, c0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:133:1, end:133:2))"]
#[derive(Clone, Debug)]
struct ctret_instructionVar61 {}
impl ctret_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ctret"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:173:1, end:173:2))"]
#[derive(Clone, Debug)]
struct eiret_instructionVar62 {}
impl eiret_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("eiret"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct feret_instructionVar63 {}
impl feret_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("feret"));
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:262:1, end:262:2))"]
#[derive(Clone, Debug)]
struct reti_instructionVar64 {}
impl reti_instructionVar64 {
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
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:300:1, end:300:2))"]
#[derive(Clone, Debug)]
struct rie_instructionVar65 {
    op1115: u8,
    op0003: u8,
}
impl rie_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rie"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1115 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0003 as u64),
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
        let op1115 = token_19(tokens_current);
        let op0003 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:361:1, end:361:2))"]
#[derive(Clone, Debug)]
struct trap_instructionVar66 {
    op0004: u8,
}
impl trap_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trap"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0004 as u64),
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
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:281:1, end:281:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar67 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl sar_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sar"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:311:1, end:311:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar68 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl shl_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:329:1, end:329:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar69 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl shr_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct clr1_instructionVar70 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl clr1_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clr1"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:130:1, end:130:2))"]
#[derive(Clone, Debug)]
struct not1_instructionVar71 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl not1_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not1"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:147:1, end:147:2))"]
#[derive(Clone, Debug)]
struct set1_instructionVar72 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl set1_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("set1"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:163:1, end:163:2))"]
#[derive(Clone, Debug)]
struct tst1_instructionVar73 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl tst1_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst1"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct sch0l_instructionVar74 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch0l_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sch0l"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:21:1, end:21:2))"]
#[derive(Clone, Debug)]
struct sch0r_instructionVar75 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch0r_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sch0r"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:29:1, end:29:2))"]
#[derive(Clone, Debug)]
struct sch1l_instructionVar76 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch1l_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sch1l"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:37:1, end:37:2))"]
#[derive(Clone, Debug)]
struct sch1r_instructionVar77 {
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sch1r_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sch1r"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct ldsr_instructionVar78 {
    SR1115: u8,
    R0004: TableR0004,
}
impl ldsr_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldsr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.SR1115),
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let SR1115 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, SR1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:312:1, end:312:2))"]
#[derive(Clone, Debug)]
struct stsr_instructionVar79 {
    SR0004: u8,
    R1115: TableR1115,
}
impl stsr_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stsr"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.SR0004),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let SR0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, SR0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:345:1, end:345:2))"]
#[derive(Clone, Debug)]
struct syscall_instructionVar80 {
    op2729: u8,
    op0004: u8,
}
impl syscall_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_vector8: i128 = 0;
        calc_vector8 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op2729).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op0004).unwrap());
        display.push(DisplayElement::Literal("syscall"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_vector8.is_negative(), calc_vector8.abs() as u64),
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
        let mut calc_vector8: i128 = 0;
        let mut block_0_len = 2;
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_vector8 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_17(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_3(tokens_current)).unwrap());
        let op2729 = token_17(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op2729, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:16:1, end:16:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar81 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl mul_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct mulu_instructionVar82 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulu_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulu"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:199:1, end:199:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar83 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl satadd_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satadd"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:217:1, end:217:2))"]
#[derive(Clone, Debug)]
struct satsub_instructionVar84 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl satsub_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satsub"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:251:1, end:251:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar85 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl div_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct divh_instructionVar86 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl divh_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:284:1, end:284:2))"]
#[derive(Clone, Debug)]
struct divhu_instructionVar87 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divhu_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divhu"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct divu_instructionVar88 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divu_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divu"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:315:1, end:315:2))"]
#[derive(Clone, Debug)]
struct divq_instructionVar89 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divq_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divq"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct divqu_instructionVar90 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divqu_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divqu"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:19:1, end:19:2))"]
#[derive(Clone, Debug)]
struct addf_d_instructionVar91 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl addf_d_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("addf.d"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        let R0004x2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:25:1, end:25:2))"]
#[derive(Clone, Debug)]
struct addf_s_instructionVar92 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl addf_s_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("addf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct divf_s_instructionVar93 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl divf_s_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divf.s"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R0004x2 = token_3(tokens_current);
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:240:1, end:240:2))"]
#[derive(Clone, Debug)]
struct divf_s_instructionVar94 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl divf_s_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct fmaf_s_instructionVar95 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl fmaf_s_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmaf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct fmsf_s_instructionVar96 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl fmsf_s_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmsf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:310:1, end:310:2))"]
#[derive(Clone, Debug)]
struct fnmaf_s_instructionVar97 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl fnmaf_s_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fnmaf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct fnmfs_s_instructionVar98 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl fnmfs_s_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fnmfs.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:328:1, end:328:2))"]
#[derive(Clone, Debug)]
struct maxf_d_instructionVar99 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl maxf_d_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("maxf.d"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R0004x2 = token_3(tokens_current);
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct maxf_s_instructionVar100 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl maxf_s_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("maxf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:342:1, end:342:2))"]
#[derive(Clone, Debug)]
struct minf_d_instructionVar101 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl minf_d_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("minf.d"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        let R0004x2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:349:1, end:349:2))"]
#[derive(Clone, Debug)]
struct minf_s_instructionVar102 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl minf_s_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("minf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:362:1, end:362:2))"]
#[derive(Clone, Debug)]
struct mulf_d_instructionVar103 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl mulf_d_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulf.d"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        let R0004x2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct mulf_s_instructionVar104 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulf_s_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:434:1, end:434:2))"]
#[derive(Clone, Debug)]
struct subf_d_instructionVar105 {
    R0004x2: u8,
    R1115x2: u8,
    R2731x2: u8,
}
impl subf_d_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subf.d"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R0004x2 = token_3(tokens_current);
        let R1115x2 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004x2,
                R1115x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:440:1, end:440:2))"]
#[derive(Clone, Debug)]
struct subf_s_instructionVar106 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl subf_s_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:293:1, end:293:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar107 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl sar_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sar"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:323:1, end:323:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar108 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl shl_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:341:1, end:341:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar109 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl shr_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:123:1, end:123:2))"]
#[derive(Clone, Debug)]
struct caxi_instructionVar110 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl caxi_instructionVar110 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("caxi"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:24:1, end:24:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar111 {
    s1821: u8,
    op0004: u8,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mul_instructionVar111 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_imm9: i128 = 0;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| {
                i128::try_from((if self.s1821 & 8 != 0 { -1 & !7 } else { 0 } | self.s1821 as i8))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op0004).unwrap());
        display.push(DisplayElement::Literal("mul"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_imm9.is_negative(), calc_imm9.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_imm9: i128 = 0;
        let mut block_0_len = 2;
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_26(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_3(tokens_current)).unwrap());
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s1821 = token_26(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R2731,
                s1821,
                op0004,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:446:1, end:446:2))"]
#[derive(Clone, Debug)]
struct trfsr_instructionVar112 {
    fcbit1719: u8,
}
impl trfsr_instructionVar112 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trfsr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.fcbit1719 as u64),
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
        let mut block_1_len = 2;
        let fcbit1719 = token_23(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fcbit1719 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:51:1, end:51:2))"]
#[derive(Clone, Debug)]
struct ld_h_instructionVar113 {
    s3247: u16,
    op2126: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_h_instructionVar113 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op2126).unwrap().checked_shl(shl))
                .unwrap_or(0));
        display.push(DisplayElement::Literal("ld.h"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2126 = token_12(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_12(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0));
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2126,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct ld_w_instructionVar114 {
    s3247: u16,
    op2126: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_w_instructionVar114 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op2126).unwrap().checked_shl(shl))
                .unwrap_or(0));
        display.push(DisplayElement::Literal("ld.w"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2126 = token_12(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_12(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0));
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2126,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:187:1, end:187:2))"]
#[derive(Clone, Debug)]
struct st_h_instructionVar115 {
    s3247: u16,
    op2126: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_h_instructionVar115 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op2126).unwrap().checked_shl(shl))
                .unwrap_or(0));
        display.push(DisplayElement::Literal("st.h"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2126 = token_12(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_12(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0));
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2126,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:205:1, end:205:2))"]
#[derive(Clone, Debug)]
struct st_w_instructionVar116 {
    s3247: u16,
    op2126: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_w_instructionVar116 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op2126).unwrap().checked_shl(shl))
                .unwrap_or(0));
        display.push(DisplayElement::Literal("st.w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2126 = token_12(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_12(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0));
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2126,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:20:1, end:20:2))"]
#[derive(Clone, Debug)]
struct ld_b_instructionVar117 {
    s3247: u16,
    op2026: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_b_instructionVar117 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op2026).unwrap());
        display.push(DisplayElement::Literal("ld.b"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2026 = token_10(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_10(tokens_current)).unwrap());
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2026,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:36:1, end:36:2))"]
#[derive(Clone, Debug)]
struct ld_bu_instructionVar118 {
    s3247: u16,
    op2026: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_bu_instructionVar118 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op2026).unwrap());
        display.push(DisplayElement::Literal("ld.bu"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2026 = token_10(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_10(tokens_current)).unwrap());
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2026,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:67:1, end:67:2))"]
#[derive(Clone, Debug)]
struct ld_hu_instructionVar119 {
    s3247: u16,
    op2026: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl ld_hu_instructionVar119 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op2026).unwrap());
        display.push(DisplayElement::Literal("ld.hu"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2026 = token_10(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_10(tokens_current)).unwrap());
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2026,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct st_b_instructionVar120 {
    s3247: u16,
    op2026: u8,
    R0004: TableR0004,
    R2731: TableR2731,
}
impl st_b_instructionVar120 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp23: i128 = 0;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s3247 & 32768 != 0 {
                        -1 & !32767
                    } else {
                        0
                    } | self.s3247 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op2026).unwrap());
        display.push(DisplayElement::Literal("st.b"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp23.is_negative(), calc_disp23.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp23: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op2026 = token_10(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_disp23 = (u32::try_from(7i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_10(tokens_current)).unwrap());
        let s3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R2731,
                s3247,
                op2026,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar121 {
    R0004: TableR0004,
}
impl jmp_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jmp"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:81:1, end:81:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar122 {
    R0004: TableR0004,
}
impl jmp_instructionVar122 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jmp"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        if token_3(tokens_current) == 31 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:100:1, end:100:2))"]
#[derive(Clone, Debug)]
struct jr_instructionVar123 {
    adr32: Tableadr32,
}
impl jr_instructionVar123 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr32
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let adr32 = if let Some((len, table)) =
            Tableadr32::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { adr32 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:189:1, end:189:2))"]
#[derive(Clone, Debug)]
struct fetrap_instructionVar124 {
    op1114: u8,
}
impl fetrap_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fetrap"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1114 as u64),
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let op1114 = token_18(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1114 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:216:1, end:216:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar125 {}
impl nop_instructionVar125 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:288:1, end:288:2))"]
#[derive(Clone, Debug)]
struct rie_instructionVar126 {}
impl rie_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rie"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct synce_instructionVar127 {}
impl synce_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("synce"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:333:1, end:333:2))"]
#[derive(Clone, Debug)]
struct syncm_instructionVar128 {}
impl syncm_instructionVar128 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("syncm"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:339:1, end:339:2))"]
#[derive(Clone, Debug)]
struct syncp_instructionVar129 {}
impl syncp_instructionVar129 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("syncp"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:33:1, end:33:2))"]
#[derive(Clone, Debug)]
struct mulh_instructionVar130 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl mulh_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:59:1, end:59:2))"]
#[derive(Clone, Debug)]
struct mulu_instructionVar131 {
    op1821: u8,
    op0004: u8,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl mulu_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_imm9: i128 = 0;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op1821).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op0004).unwrap());
        display.push(DisplayElement::Literal("mulu"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_imm9.is_negative(), calc_imm9.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_imm9: i128 = 0;
        let mut block_0_len = 2;
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_imm9 = (u32::try_from(5i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_26(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_3(tokens_current)).unwrap());
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op1821 = token_26(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R2731,
                op1821,
                op0004,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:98:1, end:98:2))"]
#[derive(Clone, Debug)]
struct cmpf_d_instructionVar132 {
    fcond2730: u8,
    R1115x2: u8,
    R0004x2: u8,
    fcbit1719: u8,
}
impl cmpf_d_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpf.d"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            meaning_5_display(self.fcond2730),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.fcbit1719 as u64),
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
        let R1115x2 = token_19(tokens_current);
        let R0004x2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let fcbit1719 = token_23(tokens_current);
        let fcond2730 = token_18(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                fcond2730,
                R1115x2,
                R0004x2,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:115:1, end:115:2))"]
#[derive(Clone, Debug)]
struct cmpf_s_instructionVar133 {
    fcond2730: u8,
    fcbit1719: u8,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl cmpf_s_instructionVar133 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmpf.s"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_5_display(self.fcond2730),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.fcbit1719 as u64),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let fcbit1719 = token_23(tokens_current);
        let fcond2730 = token_18(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                fcond2730,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct cmovf_d_instructionVar134 {
    fcbit1719: u8,
    R1115x2: u8,
    R0004x2: u8,
    R2731x2: u8,
}
impl cmovf_d_instructionVar134 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmovf.d"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.fcbit1719 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1115x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R0004x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
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
        let R1115x2 = token_19(tokens_current);
        let R0004x2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let fcbit1719 = token_23(tokens_current);
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                fcbit1719,
                R1115x2,
                R0004x2,
                R2731x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:91:1, end:91:2))"]
#[derive(Clone, Debug)]
struct cmovf_s_instructionVar135 {
    fcbit1719: u8,
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
}
impl cmovf_s_instructionVar135 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmovf.s"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.fcbit1719 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let fcbit1719 = token_23(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                R2731,
                fcbit1719,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:75:1, end:75:2))"]
#[derive(Clone, Debug)]
struct mac_instructionVar136 {
    R2731x2: u8,
    R1620x2: u8,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl mac_instructionVar136 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mac"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1620x2),
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R1620x2 = token_3(tokens_current);
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                R2731x2,
                R1620x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:81:1, end:81:2))"]
#[derive(Clone, Debug)]
struct macu_instructionVar137 {
    R2731x2: u8,
    R1620x2: u8,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl macu_instructionVar137 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("macu"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R2731x2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.R1620x2),
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R1620x2 = token_3(tokens_current);
        let R2731x2 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                R2731x2,
                R1620x2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:133:1, end:133:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar138 {
    s0004: u8,
    R1115: TableR1115,
}
impl mov_instructionVar138 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:139:1, end:139:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar139 {
    op3247: u16,
    op1631: u16,
    R0004: TableR0004,
}
impl mov_instructionVar139 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_imm32: i128 = 0;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op3247).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op1631).unwrap());
        display.push(DisplayElement::Literal("mov"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_imm32.is_negative(), calc_imm32.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_imm32: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap());
        let op3247 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op3247,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:146:1, end:146:2))"]
#[derive(Clone, Debug)]
struct movea_instructionVar140 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl movea_instructionVar140 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movea"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:152:1, end:152:2))"]
#[derive(Clone, Debug)]
struct movhi_instructionVar141 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl movhi_instructionVar141 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movhi"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar142 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satadd_instructionVar142 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satadd"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:190:1, end:190:2))"]
#[derive(Clone, Debug)]
struct satadd_instructionVar143 {
    s0004: u8,
    R1115: TableR1115,
}
impl satadd_instructionVar143 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satadd"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:208:1, end:208:2))"]
#[derive(Clone, Debug)]
struct satsub_instructionVar144 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satsub_instructionVar144 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satsub"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:226:1, end:226:2))"]
#[derive(Clone, Debug)]
struct satsubi_instructionVar145 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl satsubi_instructionVar145 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satsubi"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:235:1, end:235:2))"]
#[derive(Clone, Debug)]
struct satsubr_instructionVar146 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl satsubr_instructionVar146 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("satsubr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        if token_19(tokens_current) == 0 {
            return None;
        }
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:247:1, end:247:2))"]
#[derive(Clone, Debug)]
struct cmov_instructionVar147 {
    R0004: TableR0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl cmov_instructionVar147 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmov"));
        self.c1720
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                c1720,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:254:1, end:254:2))"]
#[derive(Clone, Debug)]
struct cmov_instructionVar148 {
    s0004: u8,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl cmov_instructionVar148 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmov"));
        self.c1720
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                c1720,
                R2731,
                s0004,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct adf_instructionVar149 {
    R0004: TableR0004,
    R1115: TableR1115,
    R2731: TableR2731,
    c1720: Tablec1720,
}
impl adf_instructionVar149 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("adf"));
        self.c1720
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                R2731,
                c1720,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:24:1, end:24:2))"]
#[derive(Clone, Debug)]
struct sbf_instructionVar150 {
    R0004: TableR0004,
    R1115: TableR1115,
    c1720: Tablec1720,
    R2731: TableR2731,
}
impl sbf_instructionVar150 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sbf"));
        self.c1720
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let c1720 = if let Some((len, table)) =
            Tablec1720::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                c1720,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:322:1, end:322:2))"]
#[derive(Clone, Debug)]
struct maddf_s_instructionVar151 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl maddf_s_instructionVar151 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("maddf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.reg4
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:356:1, end:356:2))"]
#[derive(Clone, Debug)]
struct msubf_s_instructionVar152 {
    R0004: TableR0004,
    R1115: TableR1115,
    reg4: Tablereg4,
    R2731: TableR2731,
}
impl msubf_s_instructionVar152 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("msubf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.reg4
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
                R0004,
                R1115,
                reg4,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:386:1, end:386:2))"]
#[derive(Clone, Debug)]
struct nmaddf_s_instructionVar153 {
    R1115: TableR1115,
    R0004: TableR0004,
    reg4: Tablereg4,
    R2731: TableR2731,
}
impl nmaddf_s_instructionVar153 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("nmaddf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.reg4
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                reg4,
                R2731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Float.sinc, start:392:1, end:392:2))"]
#[derive(Clone, Debug)]
struct nmsubf_s_instructionVar154 {
    R1115: TableR1115,
    R0004: TableR0004,
    R2731: TableR2731,
    reg4: Tablereg4,
}
impl nmsubf_s_instructionVar154 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("nmsubf.s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R2731
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.reg4
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let R2731 = if let Some((len, table)) =
            TableR2731::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg4 = if let Some((len, table)) =
            Tablereg4::parse(tokens_current, &mut context_instance, inst_start)
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
                R1115,
                R0004,
                R2731,
                reg4,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:44:1, end:44:2))"]
#[derive(Clone, Debug)]
struct ld_h_instructionVar155 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_h_instructionVar155 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld.h"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:59:1, end:59:2))"]
#[derive(Clone, Debug)]
struct ld_hu_instructionVar156 {
    s1731: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_hu_instructionVar156 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s1731 & 16384 != 0 {
                        -1 & !16383
                    } else {
                        0
                    } | self.s1731 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0);
        display.push(DisplayElement::Literal("ld.hu"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp16.is_negative(), calc_disp16.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_disp16 = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_25(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let s1731 = token_25(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:75:1, end:75:2))"]
#[derive(Clone, Debug)]
struct ld_w_instructionVar157 {
    s1731: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_w_instructionVar157 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = i128::try_from(
            (if self.s1731 & 16384 != 0 {
                -1 & !16383
            } else {
                0
            } | self.s1731 as i16),
        )
        .unwrap()
        .wrapping_mul(2i128);
        display.push(DisplayElement::Literal("ld.w"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp16.is_negative(), calc_disp16.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_disp16 = i128::try_from(token_25(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let s1731 = token_25(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:98:1, end:98:2))"]
#[derive(Clone, Debug)]
struct sld_bu_instructionVar158 {
    op0003: u8,
    R1115: TableR1115,
}
impl sld_bu_instructionVar158 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sld.bu"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0003 as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0003 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct sld_hu_instructionVar159 {
    op0003: u8,
    R1115: TableR1115,
}
impl sld_hu_instructionVar159 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp5: i128 = 0;
        calc_disp5 = i128::try_from(self.op0003).unwrap().wrapping_mul(2i128);
        display.push(DisplayElement::Literal("sld.hu"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp5.is_negative(), calc_disp5.abs() as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp5: i128 = 0;
        let mut block_0_len = 2;
        calc_disp5 = i128::try_from(token_2(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0003 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0003 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:179:1, end:179:2))"]
#[derive(Clone, Debug)]
struct st_h_instructionVar160 {
    s1631: u16,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl st_h_instructionVar160 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st.h"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct st_w_instructionVar161 {
    s1731: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl st_w_instructionVar161 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = i128::try_from(
            (if self.s1731 & 16384 != 0 {
                -1 & !16383
            } else {
                0
            } | self.s1731 as i16),
        )
        .unwrap()
        .wrapping_mul(2i128);
        display.push(DisplayElement::Literal("st.w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp16.is_negative(), calc_disp16.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_disp16 = i128::try_from(token_25(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let s1731 = token_25(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:347:1, end:347:2))"]
#[derive(Clone, Debug)]
struct sxb_instructionVar162 {
    R0004: TableR0004,
}
impl sxb_instructionVar162 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sxb"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:353:1, end:353:2))"]
#[derive(Clone, Debug)]
struct sxh_instructionVar163 {
    R0004: TableR0004,
}
impl sxh_instructionVar163 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sxh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:359:1, end:359:2))"]
#[derive(Clone, Debug)]
struct zxb_instructionVar164 {
    R0004: TableR0004,
}
impl zxb_instructionVar164 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("zxb"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:365:1, end:365:2))"]
#[derive(Clone, Debug)]
struct zxh_instructionVar165 {
    R0004: TableR0004,
}
impl zxh_instructionVar165 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("zxh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:70:1, end:70:2))"]
#[derive(Clone, Debug)]
struct jarl_instructionVar166 {
    R0004: TableR0004,
    adr32: Tableadr32,
}
impl jarl_instructionVar166 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jarl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr32
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let adr32 = if let Some((len, table)) =
            Tableadr32::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, adr32 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:87:1, end:87:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar167 {
    R0004: TableR0004,
    adr32i: Tableadr32i,
}
impl jmp_instructionVar167 {
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
        self.adr32i
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let adr32i = if let Some((len, table)) =
            Tableadr32i::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, adr32i }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:318:1, end:318:2))"]
#[derive(Clone, Debug)]
struct switch_instructionVar168 {
    R0004: TableR0004,
}
impl switch_instructionVar168 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("switch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:104:1, end:104:2))"]
#[derive(Clone, Debug)]
struct clr1_instructionVar169 {
    op1113: u8,
    s1631: u16,
    R0004: TableR0004,
}
impl clr1_instructionVar169 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clr1"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1113 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op1113 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct not1_instructionVar170 {
    op1113: u8,
    s1631: u16,
    R0004: TableR0004,
}
impl not1_instructionVar170 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not1"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1113 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op1113 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:138:1, end:138:2))"]
#[derive(Clone, Debug)]
struct set1_instructionVar171 {
    op1113: u8,
    s1631: u16,
    R0004: TableR0004,
}
impl set1_instructionVar171 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("set1"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1113 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op1113 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:155:1, end:155:2))"]
#[derive(Clone, Debug)]
struct tst1_instructionVar172 {
    op1113: u8,
    s1631: u16,
    R0004: TableR0004,
}
impl tst1_instructionVar172 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst1"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1113 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op1113 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                op1113,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:39:1, end:39:2))"]
#[derive(Clone, Debug)]
struct mulh_instructionVar173 {
    s0004: u8,
    R1115: TableR1115,
}
impl mulh_instructionVar173 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulh"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:45:1, end:45:2))"]
#[derive(Clone, Debug)]
struct mulhi_instructionVar174 {
    s1631: u16,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl mulhi_instructionVar174 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulhi"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:94:1, end:94:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar175 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl add_instructionVar175 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:101:1, end:101:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar176 {
    s0004: u8,
    R1115: TableR1115,
}
impl add_instructionVar176 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:108:1, end:108:2))"]
#[derive(Clone, Debug)]
struct addi_instructionVar177 {
    s1631: u16,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl addi_instructionVar177 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("addi"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:115:1, end:115:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar178 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl cmp_instructionVar178 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar179 {
    s0004: u8,
    R1115: TableR1115,
}
impl cmp_instructionVar179 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).is_negative(),
                (if self.s0004 & 16 != 0 { -1 & !15 } else { 0 } | self.s0004 as i8).abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let s0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, s0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:127:1, end:127:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar180 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl mov_instructionVar180 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:158:1, end:158:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar181 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl sub_instructionVar181 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct subr_instructionVar182 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl subr_instructionVar182 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Arithmetic.sinc, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct divh_instructionVar183 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl divh_instructionVar183 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divh"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:13:1, end:13:2))"]
#[derive(Clone, Debug)]
struct ld_b_instructionVar184 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_b_instructionVar184 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ld.b"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:149:1, end:149:2))"]
#[derive(Clone, Debug)]
struct dispose_instructionVar185 {
    prep0105: u8,
    DispList: TableDispList,
}
impl dispose_instructionVar185 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dispose"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.DispList
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
        let DispList = if let Some((len, table)) =
            TableDispList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar186 {
    prep0105: u8,
    PrepList: TablePrepList,
}
impl prepare_instructionVar186 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("prepare"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
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
        let mut block_0_len = 4;
        let PrepList = if let Some((len, table)) =
            TablePrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar187 {
    prep0105: u8,
    PrepList: TablePrepList,
}
impl prepare_instructionVar187 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("prepare"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::sp),
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
        let mut block_0_len = 4;
        let PrepList = if let Some((len, table)) =
            TablePrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList, prep0105 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:237:1, end:237:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar188 {
    prep0105: u8,
    s3247: u16,
    PrepList: TablePrepList,
}
impl prepare_instructionVar188 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("prepare"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s3247 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s3247 as i16)
                    .is_negative(),
                (if self.s3247 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s3247 as i16)
                    .abs() as u64,
            ),
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
        let mut block_0_len = 4;
        let PrepList = if let Some((len, table)) =
            TablePrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s3247 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar189 {
    prep0105: u8,
    s3247: u16,
    PrepList: TablePrepList,
}
impl prepare_instructionVar189 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("prepare"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s3247 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s3247 as i16)
                    .is_negative(),
                (if self.s3247 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s3247 as i16)
                    .abs() as u64,
            ),
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
        let mut block_0_len = 4;
        let PrepList = if let Some((len, table)) =
            TablePrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s3247 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                s3247,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:253:1, end:253:2))"]
#[derive(Clone, Debug)]
struct prepare_instructionVar190 {
    prep0105: u8,
    op4863: u16,
    op3247: u16,
    PrepList: TablePrepList,
}
impl prepare_instructionVar190 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_imm32: i128 = 0;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op4863).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op3247).unwrap());
        display.push(DisplayElement::Literal("prepare"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.PrepList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_imm32.is_negative(), calc_imm32.abs() as u64),
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
        let mut calc_imm32: i128 = 0;
        let mut block_0_len = 4;
        let PrepList = if let Some((len, table)) =
            TablePrepList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let op3247 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        calc_imm32 = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap());
        let op4863 = token_7(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                PrepList,
                prep0105,
                op4863,
                op3247,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:28:1, end:28:2))"]
#[derive(Clone, Debug)]
struct ld_bu_instructionVar191 {
    s1731: u16,
    op0505: u8,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ld_bu_instructionVar191 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp16: i128 = 0;
        calc_disp16 = (u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.s1731 & 16384 != 0 {
                        -1 & !16383
                    } else {
                        0
                    } | self.s1731 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op0505).unwrap());
        display.push(DisplayElement::Literal("ld.bu"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp16.is_negative(), calc_disp16.abs() as u64),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp16: i128 = 0;
        let mut block_0_len = 2;
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0505 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_disp16 = (u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_25(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_11(tokens_current)).unwrap());
        let s1731 = token_25(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1731,
                op0505,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:57:1, end:57:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar192 {
    adr9: Tableadr9,
}
impl br_instructionVar192 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("br"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr9
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
        let adr9 = if let Some((len, table)) =
            Tableadr9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { adr9 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct sld_w_instructionVar193 {
    op0106: u8,
    R1115: TableR1115,
}
impl sld_w_instructionVar193 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = i128::try_from(self.op0106).unwrap().wrapping_mul(4i128);
        display.push(DisplayElement::Literal("sld.w"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp8.is_negative(), calc_disp8.abs() as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2;
        calc_disp8 = i128::try_from(token_8(tokens_current))
            .unwrap()
            .wrapping_mul(4i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0106 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:153:1, end:153:2))"]
#[derive(Clone, Debug)]
struct sst_w_instructionVar194 {
    op0106: u8,
    R1115: TableR1115,
}
impl sst_w_instructionVar194 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = i128::try_from(self.op0106).unwrap().wrapping_mul(4i128);
        display.push(DisplayElement::Literal("sst.w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp8.is_negative(), calc_disp8.abs() as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("]"),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2;
        calc_disp8 = i128::try_from(token_8(tokens_current))
            .unwrap()
            .wrapping_mul(4i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0106 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0106 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct st_b_instructionVar195 {
    s1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl st_b_instructionVar195 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("st.b"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .is_negative(),
                (if self.s1631 & 32768 != 0 {
                    -1 & !32767
                } else {
                    0
                } | self.s1631 as i16)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal("["),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]")];
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let s1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                s1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:287:1, end:287:2))"]
#[derive(Clone, Debug)]
struct sar_instructionVar196 {
    op0004: u8,
    R1115: TableR1115,
}
impl sar_instructionVar196 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sar"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0004 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:317:1, end:317:2))"]
#[derive(Clone, Debug)]
struct shl_instructionVar197 {
    op0004: u8,
    R1115: TableR1115,
}
impl shl_instructionVar197 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shl"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0004 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct shr_instructionVar198 {
    op0004: u8,
    R1115: TableR1115,
}
impl shr_instructionVar198 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shr"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0004 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:42:1, end:42:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar199 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl and_instructionVar199 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:49:1, end:49:2))"]
#[derive(Clone, Debug)]
struct andi_instructionVar200 {
    op1631: u16,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl andi_instructionVar200 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("andi"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1631 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:56:1, end:56:2))"]
#[derive(Clone, Debug)]
struct not_instructionVar201 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl not_instructionVar201 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:63:1, end:63:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar202 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl or_instructionVar202 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:70:1, end:70:2))"]
#[derive(Clone, Debug)]
struct ori_instructionVar203 {
    op1631: u16,
    R0004: TableR0004,
    R1115: TableR1115,
}
impl ori_instructionVar203 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ori"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1631 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R0004,
                R1115,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:77:1, end:77:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar204 {
    R1115: TableR1115,
    R0004: TableR0004,
}
impl tst_instructionVar204 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:83:1, end:83:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar205 {
    R0004: TableR0004,
    R1115: TableR1115,
}
impl xor_instructionVar205 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R0004, R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Logic.sinc, start:90:1, end:90:2))"]
#[derive(Clone, Debug)]
struct xori_instructionVar206 {
    op1631: u16,
    R1115: TableR1115,
    R0004: TableR0004,
}
impl xori_instructionVar206 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xori"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op1631 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R0004
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let R0004 = if let Some((len, table)) =
            TableR0004::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                R1115,
                R0004,
                op1631,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:94:1, end:94:2))"]
#[derive(Clone, Debug)]
struct jr_instructionVar207 {
    adr22: Tableadr22,
}
impl jr_instructionVar207 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr22
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
        let adr22 = if let Some((len, table)) =
            Tableadr22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { adr22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:113:1, end:113:2))"]
#[derive(Clone, Debug)]
struct callt_instructionVar208 {
    op0005: u8,
}
impl callt_instructionVar208 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("callt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0005 as u64),
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
        let op0005 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0005 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct dispose_instructionVar209 {
    prep0105: u8,
    prep1620: u8,
    DispList: TableDispList,
}
impl dispose_instructionVar209 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dispose"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.prep0105 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.DispList
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("["),
            meaning_0_display(self.prep1620),
            <DisplayElement>::Literal("]"),
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
        let mut block_0_len = 4;
        let DispList = if let Some((len, table)) =
            TableDispList::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let prep1620 = token_34(tokens_current);
        let prep0105 = token_32(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                DispList,
                prep0105,
                prep1620,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:63:1, end:63:2))"]
#[derive(Clone, Debug)]
struct jarl_instructionVar210 {
    R1115: TableR1115,
    adr22: Tableadr22,
}
impl jarl_instructionVar210 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jarl"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr22
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if token_14(tokens) != 30 {
                return None;
            }
            let R1115 = if let Some((len, table)) =
                TableR1115::parse(tokens, &mut context_instance, inst_start)
            {
                block_0_len = block_0_len.max(len as AddrType);
                table
            } else {
                return None;
            };
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((R1115), (), pattern_len))
        };
        let ((mut R1115), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let adr22 = if let Some((len, table)) =
            Tableadr22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, adr22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:91:1, end:91:2))"]
#[derive(Clone, Debug)]
struct sld_b_instructionVar211 {
    op0006: u8,
    R1115: TableR1115,
}
impl sld_b_instructionVar211 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sld.b"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0006 as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0006 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:105:1, end:105:2))"]
#[derive(Clone, Debug)]
struct sld_h_instructionVar212 {
    op0006: u8,
    R1115: TableR1115,
}
impl sld_h_instructionVar212 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = i128::try_from(self.op0006).unwrap().wrapping_mul(2i128);
        display.push(DisplayElement::Literal("sld.h"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp8.is_negative(), calc_disp8.abs() as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("],"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2;
        calc_disp8 = i128::try_from(token_5(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0006 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:136:1, end:136:2))"]
#[derive(Clone, Debug)]
struct sst_b_instructionVar213 {
    op0006: u8,
    R1115: TableR1115,
}
impl sst_b_instructionVar213 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sst.b"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.op0006 as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("]"),
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
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0006 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Load_Store.sinc, start:144:1, end:144:2))"]
#[derive(Clone, Debug)]
struct sst_h_instructionVar214 {
    op0006: u8,
    R1115: TableR1115,
}
impl sst_h_instructionVar214 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp8: i128 = 0;
        calc_disp8 = i128::try_from(self.op0006).unwrap().wrapping_mul(2i128);
        display.push(DisplayElement::Literal("sst.h"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.R1115
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_disp8.is_negative(), calc_disp8.abs() as u64),
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal("]"),
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
        let mut calc_disp8: i128 = 0;
        let mut block_0_len = 2;
        calc_disp8 = i128::try_from(token_5(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let R1115 = if let Some((len, table)) =
            TableR1115::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let op0006 = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { R1115, op0006 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Instructions/Special.sinc, start:52:1, end:52:2))"]
#[derive(Clone, Debug)]
struct b_instructionVar215 {
    c0003: Tablec0003,
    adr9: Tableadr9,
}
impl b_instructionVar215 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("b"));
        self.c0003
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.adr9
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
        let c0003 = if let Some((len, table)) =
            Tablec0003::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let adr9 = if let Some((len, table)) =
            Tableadr9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { c0003, adr9 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(di_instructionVar0),
    Var1(ei_instructionVar1),
    Var2(halt_instructionVar2),
    Var3(absf_d_instructionVar3),
    Var4(absf_s_instructionVar4),
    Var5(ceilf_dl_instructionVar5),
    Var6(ceilf_dul_instructionVar6),
    Var7(ceilf_duw_instructionVar7),
    Var8(ceilf_dw_instructionVar8),
    Var9(ceilf_sl_instructionVar9),
    Var10(ceilf_sul_instructionVar10),
    Var11(ceilf_sul_instructionVar11),
    Var12(ceilf_sw_instructionVar12),
    Var13(cvtf_dl_instructionVar13),
    Var14(cvtf_ds_instructionVar14),
    Var15(cvtf_dul_instructionVar15),
    Var16(cvtf_duw_instructionVar16),
    Var17(cvtf_sw_instructionVar17),
    Var18(cvtf_ls_instructionVar18),
    Var19(cvtf_ls_instructionVar19),
    Var20(cvtf_sd_instructionVar20),
    Var21(cvtf_sl_instructionVar21),
    Var22(cvtf_sul_instructionVar22),
    Var23(cvtf_suw_instructionVar23),
    Var24(cvtf_sw_instructionVar24),
    Var25(cvtf_uls_instructionVar25),
    Var26(cvtf_uls_instructionVar26),
    Var27(cvtf_uwd_instructionVar27),
    Var28(cvtf_uws_instructionVar28),
    Var29(cvtf_wd_instructionVar29),
    Var30(cvtf_ws_instructionVar30),
    Var31(floorf_dl_instructionVar31),
    Var32(floorf_dul_instructionVar32),
    Var33(floorf_duw_instructionVar33),
    Var34(floorf_dw_instructionVar34),
    Var35(floorf_sl_instructionVar35),
    Var36(floorf_sul_instructionVar36),
    Var37(floorf_suw_instructionVar37),
    Var38(floorf_suw_instructionVar38),
    Var39(negf_d_instructionVar39),
    Var40(negf_s_instructionVar40),
    Var41(recipf_d_instructionVar41),
    Var42(recipf_s_instructionVar42),
    Var43(rsqrtf_d_instructionVar43),
    Var44(rsqrtf_s_instructionVar44),
    Var45(sqrtf_d_instructionVar45),
    Var46(sqrtf_s_instructionVar46),
    Var47(trncf_dl_instructionVar47),
    Var48(trncf_dul_instructionVar48),
    Var49(trncf_duw_instructionVar49),
    Var50(trncf_dw_instructionVar50),
    Var51(trncf_sl_instructionVar51),
    Var52(trncf_sul_instructionVar52),
    Var53(trncf_suw_instructionVar53),
    Var54(trncf_sw_instructionVar54),
    Var55(bsh_instructionVar55),
    Var56(bsw_instructionVar56),
    Var57(hsh_instructionVar57),
    Var58(hsw_instructionVar58),
    Var59(sasf_instructionVar59),
    Var60(setf_instructionVar60),
    Var61(ctret_instructionVar61),
    Var62(eiret_instructionVar62),
    Var63(feret_instructionVar63),
    Var64(reti_instructionVar64),
    Var65(rie_instructionVar65),
    Var66(trap_instructionVar66),
    Var67(sar_instructionVar67),
    Var68(shl_instructionVar68),
    Var69(shr_instructionVar69),
    Var70(clr1_instructionVar70),
    Var71(not1_instructionVar71),
    Var72(set1_instructionVar72),
    Var73(tst1_instructionVar73),
    Var74(sch0l_instructionVar74),
    Var75(sch0r_instructionVar75),
    Var76(sch1l_instructionVar76),
    Var77(sch1r_instructionVar77),
    Var78(ldsr_instructionVar78),
    Var79(stsr_instructionVar79),
    Var80(syscall_instructionVar80),
    Var81(mul_instructionVar81),
    Var82(mulu_instructionVar82),
    Var83(satadd_instructionVar83),
    Var84(satsub_instructionVar84),
    Var85(div_instructionVar85),
    Var86(divh_instructionVar86),
    Var87(divhu_instructionVar87),
    Var88(divu_instructionVar88),
    Var89(divq_instructionVar89),
    Var90(divqu_instructionVar90),
    Var91(addf_d_instructionVar91),
    Var92(addf_s_instructionVar92),
    Var93(divf_s_instructionVar93),
    Var94(divf_s_instructionVar94),
    Var95(fmaf_s_instructionVar95),
    Var96(fmsf_s_instructionVar96),
    Var97(fnmaf_s_instructionVar97),
    Var98(fnmfs_s_instructionVar98),
    Var99(maxf_d_instructionVar99),
    Var100(maxf_s_instructionVar100),
    Var101(minf_d_instructionVar101),
    Var102(minf_s_instructionVar102),
    Var103(mulf_d_instructionVar103),
    Var104(mulf_s_instructionVar104),
    Var105(subf_d_instructionVar105),
    Var106(subf_s_instructionVar106),
    Var107(sar_instructionVar107),
    Var108(shl_instructionVar108),
    Var109(shr_instructionVar109),
    Var110(caxi_instructionVar110),
    Var111(mul_instructionVar111),
    Var112(trfsr_instructionVar112),
    Var113(ld_h_instructionVar113),
    Var114(ld_w_instructionVar114),
    Var115(st_h_instructionVar115),
    Var116(st_w_instructionVar116),
    Var117(ld_b_instructionVar117),
    Var118(ld_bu_instructionVar118),
    Var119(ld_hu_instructionVar119),
    Var120(st_b_instructionVar120),
    Var121(jmp_instructionVar121),
    Var122(jmp_instructionVar122),
    Var123(jr_instructionVar123),
    Var124(fetrap_instructionVar124),
    Var125(nop_instructionVar125),
    Var126(rie_instructionVar126),
    Var127(synce_instructionVar127),
    Var128(syncm_instructionVar128),
    Var129(syncp_instructionVar129),
    Var130(mulh_instructionVar130),
    Var131(mulu_instructionVar131),
    Var132(cmpf_d_instructionVar132),
    Var133(cmpf_s_instructionVar133),
    Var134(cmovf_d_instructionVar134),
    Var135(cmovf_s_instructionVar135),
    Var136(mac_instructionVar136),
    Var137(macu_instructionVar137),
    Var138(mov_instructionVar138),
    Var139(mov_instructionVar139),
    Var140(movea_instructionVar140),
    Var141(movhi_instructionVar141),
    Var142(satadd_instructionVar142),
    Var143(satadd_instructionVar143),
    Var144(satsub_instructionVar144),
    Var145(satsubi_instructionVar145),
    Var146(satsubr_instructionVar146),
    Var147(cmov_instructionVar147),
    Var148(cmov_instructionVar148),
    Var149(adf_instructionVar149),
    Var150(sbf_instructionVar150),
    Var151(maddf_s_instructionVar151),
    Var152(msubf_s_instructionVar152),
    Var153(nmaddf_s_instructionVar153),
    Var154(nmsubf_s_instructionVar154),
    Var155(ld_h_instructionVar155),
    Var156(ld_hu_instructionVar156),
    Var157(ld_w_instructionVar157),
    Var158(sld_bu_instructionVar158),
    Var159(sld_hu_instructionVar159),
    Var160(st_h_instructionVar160),
    Var161(st_w_instructionVar161),
    Var162(sxb_instructionVar162),
    Var163(sxh_instructionVar163),
    Var164(zxb_instructionVar164),
    Var165(zxh_instructionVar165),
    Var166(jarl_instructionVar166),
    Var167(jmp_instructionVar167),
    Var168(switch_instructionVar168),
    Var169(clr1_instructionVar169),
    Var170(not1_instructionVar170),
    Var171(set1_instructionVar171),
    Var172(tst1_instructionVar172),
    Var173(mulh_instructionVar173),
    Var174(mulhi_instructionVar174),
    Var175(add_instructionVar175),
    Var176(add_instructionVar176),
    Var177(addi_instructionVar177),
    Var178(cmp_instructionVar178),
    Var179(cmp_instructionVar179),
    Var180(mov_instructionVar180),
    Var181(sub_instructionVar181),
    Var182(subr_instructionVar182),
    Var183(divh_instructionVar183),
    Var184(ld_b_instructionVar184),
    Var185(dispose_instructionVar185),
    Var186(prepare_instructionVar186),
    Var187(prepare_instructionVar187),
    Var188(prepare_instructionVar188),
    Var189(prepare_instructionVar189),
    Var190(prepare_instructionVar190),
    Var191(ld_bu_instructionVar191),
    Var192(br_instructionVar192),
    Var193(sld_w_instructionVar193),
    Var194(sst_w_instructionVar194),
    Var195(st_b_instructionVar195),
    Var196(sar_instructionVar196),
    Var197(shl_instructionVar197),
    Var198(shr_instructionVar198),
    Var199(and_instructionVar199),
    Var200(andi_instructionVar200),
    Var201(not_instructionVar201),
    Var202(or_instructionVar202),
    Var203(ori_instructionVar203),
    Var204(tst_instructionVar204),
    Var205(xor_instructionVar205),
    Var206(xori_instructionVar206),
    Var207(jr_instructionVar207),
    Var208(callt_instructionVar208),
    Var209(dispose_instructionVar209),
    Var210(jarl_instructionVar210),
    Var211(sld_b_instructionVar211),
    Var212(sld_h_instructionVar212),
    Var213(sst_b_instructionVar213),
    Var214(sst_h_instructionVar214),
    Var215(b_instructionVar215),
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
            Self::Var104(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var105(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var106(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var107(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var108(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var109(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var110(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var111(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var112(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var113(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var114(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var115(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var116(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var117(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var118(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var119(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var120(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var121(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var122(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var123(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var124(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var125(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var126(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var127(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var128(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var129(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var130(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var131(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var132(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var133(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var134(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var135(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var136(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var137(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var138(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var139(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var140(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var141(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var142(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var143(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var144(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var145(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var146(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var147(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var148(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var149(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var150(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var151(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var152(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var153(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var154(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var155(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var156(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var157(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var158(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var159(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var160(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var161(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var162(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var163(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var164(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var165(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var166(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var167(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var168(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var169(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var170(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var171(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var172(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var173(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var174(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var175(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var176(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var177(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var178(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var179(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var180(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var181(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var182(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var183(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var184(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var185(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var186(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var187(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var188(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var189(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var190(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var191(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var192(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var193(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var194(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var195(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var196(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var197(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var198(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var199(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var200(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var201(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var202(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var203(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var204(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var205(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var206(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var207(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var208(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var209(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var210(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var211(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var212(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var213(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var214(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var215(x) => {
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
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 96
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                di_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 255) == 135
            && (tokens_param[2] & 255) == 96
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                ei_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 32
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                halt_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 88
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                absf_d_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 72
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                absf_s_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_dl_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 242
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_dul_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 242
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_duw_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_dw_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_sl_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 242
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_sul_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 242
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_sul_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                ceilf_sw_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 228
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_dl_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 227
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_ds_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 244
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_dul_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 244
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_duw_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 228
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_sw_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_ls_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 66
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_ls_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_sd_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 228
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_sl_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 244
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_sul_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 244
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_suw_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 228
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_sw_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_uls_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 66
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_uls_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 240
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_uwd_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 240
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 66
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_uws_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 82
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_wd_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 66
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cvtf_ws_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 227
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_dl_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 243
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_dul_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 243
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_duw_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 227
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_dw_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 227
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_sl_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 243
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_sul_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 243
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_suw_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 227
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                floorf_suw_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 88
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                negf_d_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 72
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                negf_s_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 94
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                recipf_d_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 78
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                recipf_s_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 94
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                rsqrtf_d_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 226
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 78
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                rsqrtf_s_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 94
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                sqrtf_d_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 78
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                sqrtf_s_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_dl_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 84
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_dul_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_duw_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 80
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_dw_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_sl_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_sul_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 241
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_suw_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 225
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                trncf_sw_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 66
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                bsh_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                bsw_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 70
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                hsh_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                hsw_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 0
            && (tokens_param[3] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                sasf_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 0
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                setf_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 68
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                ctret_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 72
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                eiret_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 74
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                feret_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                reti_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 0
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                rie_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 255) == 0
            && (tokens_param[3] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                trap_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 160
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                sar_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 192
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                shl_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 128
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                shr_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 228
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                clr1_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 226
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                not1_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 224
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                set1_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 230
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                tst1_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 100
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                sch0l_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 96
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                sch0r_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 102
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                sch1l_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 98
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                sch1r_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 32
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                ldsr_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 64
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                stsr_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 255) == 215
            && (tokens_param[2] & 255) == 96
            && (tokens_param[3] & 199) == 1
        {
            if let Some((inst_len, parsed)) =
                syscall_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 32
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                mul_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 34
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                mulu_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 186
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                satadd_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 154
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                satsub_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 192
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                div_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 128
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                divh_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 130
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                divhu_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 194
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                divu_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 252
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                divq_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 254
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                divqu_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 112
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                addf_d_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 96
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                addf_s_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 126
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                divf_s_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 110
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                divf_s_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 224
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                fmaf_s_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 226
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                fmsf_s_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 228
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                fnmaf_s_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 230
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                fnmfs_s_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 120
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                maxf_d_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 104
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                maxf_s_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 122
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                minf_d_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 106
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                minf_s_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 116
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                mulf_d_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 100
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                mulf_s_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 114
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                subf_d_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 98
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                subf_s_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 162
            && (tokens_param[3] & 7) == 0
        {
            if let Some((inst_len, parsed)) =
                sar_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 194
            && (tokens_param[3] & 7) == 0
        {
            if let Some((inst_len, parsed)) =
                shl_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 130
            && (tokens_param[3] & 7) == 0
        {
            if let Some((inst_len, parsed)) =
                shr_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 255) == 238
            && (tokens_param[3] & 7) == 0
        {
            if let Some((inst_len, parsed)) =
                caxi_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 195) == 64
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                mul_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 224
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 241) == 0
            && (tokens_param[3] & 255) == 4
        {
            if let Some((inst_len, parsed)) =
                trfsr_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 7
        {
            if let Some((inst_len, parsed)) =
                ld_h_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 9
        {
            if let Some((inst_len, parsed)) =
                ld_w_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 160
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 13
        {
            if let Some((inst_len, parsed)) =
                st_h_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 15
        {
            if let Some((inst_len, parsed)) =
                st_w_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 15) == 5
        {
            if let Some((inst_len, parsed)) =
                ld_b_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 160
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 15) == 5
        {
            if let Some((inst_len, parsed)) =
                ld_bu_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 160
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 15) == 7
        {
            if let Some((inst_len, parsed)) =
                ld_hu_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 224) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                st_b_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 127 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 96 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 255) == 224 && (tokens_param[1] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                jr_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 64 && (tokens_param[1] & 135) == 0
        {
            if let Some((inst_len, parsed)) =
                fetrap_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                nop_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 64 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                rie_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 29 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                synce_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 30 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                syncm_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 31 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                syncp_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                mulh_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 195) == 66
            && (tokens_param[3] & 7) == 2
        {
            if let Some((inst_len, parsed)) =
                mulu_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 241) == 48
            && (tokens_param[3] & 135) == 4
        {
            if let Some((inst_len, parsed)) =
                cmpf_d_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 241) == 32
            && (tokens_param[3] & 135) == 4
        {
            if let Some((inst_len, parsed)) =
                cmpf_s_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 241) == 16
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cmovf_d_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 241) == 0
            && (tokens_param[3] & 7) == 4
        {
            if let Some((inst_len, parsed)) =
                cmovf_s_instructionVar135::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 192
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                mac_instructionVar136::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var136(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 224
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                macu_instructionVar137::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var137(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar138::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var138(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                mov_instructionVar139::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var139(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                movea_instructionVar140::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var140(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                movhi_instructionVar141::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var141(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                satadd_instructionVar142::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var142(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                satadd_instructionVar143::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var143(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                satsub_instructionVar144::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var144(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 96 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                satsubi_instructionVar145::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var145(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 128 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                satsubr_instructionVar146::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var146(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 32
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                cmov_instructionVar147::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var147(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 0
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                cmov_instructionVar148::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var148(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 160
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                adf_instructionVar149::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var149(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 225) == 128
            && (tokens_param[3] & 7) == 3
        {
            if let Some((inst_len, parsed)) =
                sbf_instructionVar150::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var150(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 97) == 0
            && (tokens_param[3] & 7) == 5
        {
            if let Some((inst_len, parsed)) =
                maddf_s_instructionVar151::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var151(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 97) == 32
            && (tokens_param[3] & 7) == 5
        {
            if let Some((inst_len, parsed)) =
                msubf_s_instructionVar152::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var152(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 97) == 64
            && (tokens_param[3] & 7) == 5
        {
            if let Some((inst_len, parsed)) =
                nmaddf_s_instructionVar153::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var153(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 97) == 96
            && (tokens_param[3] & 7) == 5
        {
            if let Some((inst_len, parsed)) =
                nmsubf_s_instructionVar154::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var154(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 32
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 0
        {
            if let Some((inst_len, parsed)) =
                ld_h_instructionVar155::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var155(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 224
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 1
        {
            if let Some((inst_len, parsed)) =
                ld_hu_instructionVar156::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var156(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 32
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 1
        {
            if let Some((inst_len, parsed)) =
                ld_w_instructionVar157::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var157(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                sld_bu_instructionVar158::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var158(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 112 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                sld_hu_instructionVar159::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var159(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 96
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 0
        {
            if let Some((inst_len, parsed)) =
                st_h_instructionVar160::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var160(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 96
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 1
        {
            if let Some((inst_len, parsed)) =
                st_w_instructionVar161::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var161(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                sxb_instructionVar162::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var162(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                sxh_instructionVar163::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var163(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 128 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                zxb_instructionVar164::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var164(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                zxh_instructionVar165::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var165(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                jarl_instructionVar166::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var166(parsed)));
            }
        }
        if tokens_param.len() >= 6 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar167::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var167(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                switch_instructionVar168::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var168(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 192
            && (tokens_param[1] & 199) == 135
        {
            if let Some((inst_len, parsed)) =
                clr1_instructionVar169::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var169(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 192
            && (tokens_param[1] & 199) == 71
        {
            if let Some((inst_len, parsed)) =
                not1_instructionVar170::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var170(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 199) == 7
        {
            if let Some((inst_len, parsed)) =
                set1_instructionVar171::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var171(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 224) == 192
            && (tokens_param[1] & 199) == 199
        {
            if let Some((inst_len, parsed)) =
                tst1_instructionVar172::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var172(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                mulh_instructionVar173::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var173(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                mulhi_instructionVar174::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var174(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                add_instructionVar175::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var175(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                add_instructionVar176::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var176(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                addi_instructionVar177::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var177(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 224 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar178::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var178(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 96 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar179::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var179(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar180::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var180(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar181::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var181(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 128 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                subr_instructionVar182::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var182(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                divh_instructionVar183::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var183(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 7) == 7 {
            if let Some((inst_len, parsed)) =
                ld_b_instructionVar184::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var184(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 192) == 64
            && (tokens_param[1] & 255) == 6
            && (tokens_param[2] & 31) == 0
        {
            if let Some((inst_len, parsed)) =
                dispose_instructionVar185::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var185(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 1
        {
            if let Some((inst_len, parsed)) =
                prepare_instructionVar186::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var186(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 3
        {
            if let Some((inst_len, parsed)) =
                prepare_instructionVar187::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var187(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 11
        {
            if let Some((inst_len, parsed)) =
                prepare_instructionVar188::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var188(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 19
        {
            if let Some((inst_len, parsed)) =
                prepare_instructionVar189::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var189(parsed)));
            }
        }
        if tokens_param.len() >= 8
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 255) == 7
            && (tokens_param[2] & 31) == 27
        {
            if let Some((inst_len, parsed)) =
                prepare_instructionVar190::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var190(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 192) == 128
            && (tokens_param[1] & 7) == 7
            && (tokens_param[2] & 1) == 1
        {
            if let Some((inst_len, parsed)) =
                ld_bu_instructionVar191::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var191(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 143) == 133 && (tokens_param[1] & 7) == 5 {
            if let Some((inst_len, parsed)) =
                br_instructionVar192::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var192(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 129) == 0 && (tokens_param[1] & 7) == 5 {
            if let Some((inst_len, parsed)) =
                sld_w_instructionVar193::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var193(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 129) == 1 && (tokens_param[1] & 7) == 5 {
            if let Some((inst_len, parsed)) =
                sst_w_instructionVar194::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var194(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 7) == 7 {
            if let Some((inst_len, parsed)) =
                st_b_instructionVar195::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var195(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                sar_instructionVar196::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var196(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                shl_instructionVar197::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var197(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 128 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                shr_instructionVar198::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var198(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                and_instructionVar199::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var199(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                andi_instructionVar200::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var200(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                not_instructionVar201::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var201(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                or_instructionVar202::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var202(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 128 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                ori_instructionVar203::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var203(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 96 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                tst_instructionVar204::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var204(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                xor_instructionVar205::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var205(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                xori_instructionVar206::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var206(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 255) == 7
        {
            if let Some((inst_len, parsed)) =
                jr_instructionVar207::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var207(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 0 && (tokens_param[1] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                callt_instructionVar208::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var208(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 192) == 64 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                dispose_instructionVar209::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var209(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 7) == 7 {
            if let Some((inst_len, parsed)) =
                jarl_instructionVar210::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var210(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 0 && (tokens_param[1] & 7) == 3 {
            if let Some((inst_len, parsed)) =
                sld_b_instructionVar211::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var211(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 0 && (tokens_param[1] & 7) == 4 {
            if let Some((inst_len, parsed)) =
                sld_h_instructionVar212::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var212(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 128 && (tokens_param[1] & 7) == 3 {
            if let Some((inst_len, parsed)) =
                sst_b_instructionVar213::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var213(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 128 && (tokens_param[1] & 7) == 4 {
            if let Some((inst_len, parsed)) =
                sst_h_instructionVar214::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var214(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 128 && (tokens_param[1] & 7) == 5 {
            if let Some((inst_len, parsed)) =
                b_instructionVar215::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var215(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:6:1, end:6:6))"]
#[derive(Clone, Debug)]
struct c0003Var0 {}
impl c0003Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("v")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:7:1, end:7:6))"]
#[derive(Clone, Debug)]
struct c0003Var1 {}
impl c0003Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nv")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:8:1, end:8:6))"]
#[derive(Clone, Debug)]
struct c0003Var2 {}
impl c0003Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("c")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:9:1, end:9:6))"]
#[derive(Clone, Debug)]
struct c0003Var3 {}
impl c0003Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nc")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:10:1, end:10:6))"]
#[derive(Clone, Debug)]
struct c0003Var4 {}
impl c0003Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("e")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:11:1, end:11:6))"]
#[derive(Clone, Debug)]
struct c0003Var5 {}
impl c0003Var5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ne")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:12:1, end:12:6))"]
#[derive(Clone, Debug)]
struct c0003Var6 {}
impl c0003Var6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nh")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:13:1, end:13:6))"]
#[derive(Clone, Debug)]
struct c0003Var7 {}
impl c0003Var7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("h")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:14:1, end:14:6))"]
#[derive(Clone, Debug)]
struct c0003Var8 {}
impl c0003Var8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("n")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:15:1, end:15:6))"]
#[derive(Clone, Debug)]
struct c0003Var9 {}
impl c0003Var9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("p")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:16:1, end:16:6))"]
#[derive(Clone, Debug)]
struct c0003Var10 {}
impl c0003Var10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("t")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:17:1, end:17:6))"]
#[derive(Clone, Debug)]
struct c0003Var11 {}
impl c0003Var11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("sa")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:18:1, end:18:6))"]
#[derive(Clone, Debug)]
struct c0003Var12 {}
impl c0003Var12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("lt")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:19:1, end:19:6))"]
#[derive(Clone, Debug)]
struct c0003Var13 {}
impl c0003Var13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ge")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:20:1, end:20:6))"]
#[derive(Clone, Debug)]
struct c0003Var14 {}
impl c0003Var14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("le")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:21:1, end:21:6))"]
#[derive(Clone, Debug)]
struct c0003Var15 {}
impl c0003Var15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("gt")];
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
#[derive(Clone, Debug)]
enum Tablec0003 {
    Var0(c0003Var0),
    Var1(c0003Var1),
    Var2(c0003Var2),
    Var3(c0003Var3),
    Var4(c0003Var4),
    Var5(c0003Var5),
    Var6(c0003Var6),
    Var7(c0003Var7),
    Var8(c0003Var8),
    Var9(c0003Var9),
    Var10(c0003Var10),
    Var11(c0003Var11),
    Var12(c0003Var12),
    Var13(c0003Var13),
    Var14(c0003Var14),
    Var15(c0003Var15),
}
impl Tablec0003 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                c0003Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                c0003Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                c0003Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                c0003Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                c0003Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                c0003Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                c0003Var6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                c0003Var7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                c0003Var8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                c0003Var9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                c0003Var10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                c0003Var11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                c0003Var12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                c0003Var13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                c0003Var14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                c0003Var15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:23:1, end:23:6))"]
#[derive(Clone, Debug)]
struct c1720Var0 {}
impl c1720Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("v")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:24:1, end:24:6))"]
#[derive(Clone, Debug)]
struct c1720Var1 {}
impl c1720Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nv")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:25:1, end:25:6))"]
#[derive(Clone, Debug)]
struct c1720Var2 {}
impl c1720Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("c")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:26:1, end:26:6))"]
#[derive(Clone, Debug)]
struct c1720Var3 {}
impl c1720Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nc")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:27:1, end:27:6))"]
#[derive(Clone, Debug)]
struct c1720Var4 {}
impl c1720Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("e")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:28:1, end:28:6))"]
#[derive(Clone, Debug)]
struct c1720Var5 {}
impl c1720Var5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ne")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:29:1, end:29:6))"]
#[derive(Clone, Debug)]
struct c1720Var6 {}
impl c1720Var6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("nh")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:30:1, end:30:6))"]
#[derive(Clone, Debug)]
struct c1720Var7 {}
impl c1720Var7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("h")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:31:1, end:31:6))"]
#[derive(Clone, Debug)]
struct c1720Var8 {}
impl c1720Var8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("n")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:32:1, end:32:6))"]
#[derive(Clone, Debug)]
struct c1720Var9 {}
impl c1720Var9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("p")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:33:1, end:33:6))"]
#[derive(Clone, Debug)]
struct c1720Var10 {}
impl c1720Var10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("t")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:34:1, end:34:6))"]
#[derive(Clone, Debug)]
struct c1720Var11 {}
impl c1720Var11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("sa")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:35:1, end:35:6))"]
#[derive(Clone, Debug)]
struct c1720Var12 {}
impl c1720Var12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("lt")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:36:1, end:36:6))"]
#[derive(Clone, Debug)]
struct c1720Var13 {}
impl c1720Var13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ge")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:37:1, end:37:6))"]
#[derive(Clone, Debug)]
struct c1720Var14 {}
impl c1720Var14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("le")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Conditions.sinc, start:38:1, end:38:6))"]
#[derive(Clone, Debug)]
struct c1720Var15 {}
impl c1720Var15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("gt")];
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
#[derive(Clone, Debug)]
enum Tablec1720 {
    Var0(c1720Var0),
    Var1(c1720Var1),
    Var2(c1720Var2),
    Var3(c1720Var3),
    Var4(c1720Var4),
    Var5(c1720Var5),
    Var6(c1720Var6),
    Var7(c1720Var7),
    Var8(c1720Var8),
    Var9(c1720Var9),
    Var10(c1720Var10),
    Var11(c1720Var11),
    Var12(c1720Var12),
    Var13(c1720Var13),
    Var14(c1720Var14),
    Var15(c1720Var15),
}
impl Tablec1720 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 0 {
            if let Some((inst_len, parsed)) =
                c1720Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 16 {
            if let Some((inst_len, parsed)) =
                c1720Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 2 {
            if let Some((inst_len, parsed)) =
                c1720Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 18 {
            if let Some((inst_len, parsed)) =
                c1720Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 4 {
            if let Some((inst_len, parsed)) =
                c1720Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 20 {
            if let Some((inst_len, parsed)) =
                c1720Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 6 {
            if let Some((inst_len, parsed)) =
                c1720Var6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 22 {
            if let Some((inst_len, parsed)) =
                c1720Var7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 8 {
            if let Some((inst_len, parsed)) =
                c1720Var8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 24 {
            if let Some((inst_len, parsed)) =
                c1720Var9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 10 {
            if let Some((inst_len, parsed)) =
                c1720Var10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 26 {
            if let Some((inst_len, parsed)) =
                c1720Var11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 12 {
            if let Some((inst_len, parsed)) =
                c1720Var12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 28 {
            if let Some((inst_len, parsed)) =
                c1720Var13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 14 {
            if let Some((inst_len, parsed)) =
                c1720Var14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 30) == 30 {
            if let Some((inst_len, parsed)) =
                c1720Var15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:7:1, end:7:6))"]
#[derive(Clone, Debug)]
struct R0004Var0 {
    _R0004: u8,
}
impl R0004Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R0004)];
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
        let _R0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:8:1, end:8:6))"]
#[derive(Clone, Debug)]
struct R0004Var1 {
    _R0004: u8,
}
impl R0004Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R0004)];
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
        let _R0004 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R0004 }))
    }
}
#[derive(Clone, Debug)]
enum TableR0004 {
    Var0(R0004Var0),
    Var1(R0004Var1),
}
impl TableR0004 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 0 {
            if let Some((inst_len, parsed)) =
                R0004Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                R0004Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:10:1, end:10:6))"]
#[derive(Clone, Debug)]
struct R1115Var0 {
    _R1115: u8,
}
impl R1115Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R1115)];
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
        let _R1115 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:11:1, end:11:6))"]
#[derive(Clone, Debug)]
struct R1115Var1 {
    _R1115: u8,
}
impl R1115Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R1115)];
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
        let _R1115 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R1115 }))
    }
}
#[derive(Clone, Debug)]
enum TableR1115 {
    Var0(R1115Var0),
    Var1(R1115Var1),
}
impl TableR1115 {
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
        if tokens_param.len() >= 2 && (tokens_param[1] & 248) == 0 {
            if let Some((inst_len, parsed)) =
                R1115Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                R1115Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:13:1, end:13:6))"]
#[derive(Clone, Debug)]
struct R2731Var0 {
    _R2731: u8,
}
impl R2731Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R2731)];
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
        let _R2731 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:14:1, end:14:6))"]
#[derive(Clone, Debug)]
struct R2731Var1 {
    _R2731: u8,
}
impl R2731Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_0_display(self._R2731)];
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
        let _R2731 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { _R2731 }))
    }
}
#[derive(Clone, Debug)]
enum TableR2731 {
    Var0(R2731Var0),
    Var1(R2731Var1),
}
impl TableR2731 {
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
        if tokens_param.len() >= 2 && (tokens_param[1] & 248) == 0 {
            if let Some((inst_len, parsed)) =
                R2731Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                R2731Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:17:1, end:17:5))"]
#[derive(Clone, Debug)]
struct adr9Var0 {
    s1115: u8,
    op0406: u8,
}
impl adr9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| {
                i128::try_from((if self.s1115 & 16 != 0 { -1 & !15 } else { 0 } | self.s1115 as i8))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| i128::try_from(self.op0406).unwrap().checked_shl(shl))
                .unwrap_or(0))
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_res.is_negative(),
            calc_res.abs() as u64,
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
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2;
        calc_res = (u32::try_from(4i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_19(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(1i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_9(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let s1115 = token_19(tokens_current);
        let op0406 = token_9(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { s1115, op0406 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr9 {
    Var0(adr9Var0),
}
impl Tableadr9 {
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
                adr9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:23:1, end:23:6))"]
#[derive(Clone, Debug)]
struct adr22Var0 {
    s0005: u8,
    op1631: u16,
}
impl adr22Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from((if self.s0005 & 32 != 0 { -1 & !31 } else { 0 } | self.s0005 as i8))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.op1631).unwrap())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_res.is_negative(),
            calc_res.abs() as u64,
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
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2;
        let s0005 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_4(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let op1631 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { s0005, op1631 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr22 {
    Var0(adr22Var0),
}
impl Tableadr22 {
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
        if tokens_param.len() >= 4 && (tokens_param[2] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                adr22Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:29:1, end:29:6))"]
#[derive(Clone, Debug)]
struct adr32Var0 {
    op3247: u16,
    op1631: u16,
}
impl adr32Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op3247).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op1631).unwrap())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_res.is_negative(),
            calc_res.abs() as u64,
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
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap())
        .wrapping_add(i128::try_from(inst_start).unwrap());
        let op3247 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op3247, op1631 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr32 {
    Var0(adr32Var0),
}
impl Tableadr32 {
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
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                adr32Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:35:1, end:35:7))"]
#[derive(Clone, Debug)]
struct adr32iVar0 {
    op3247: u16,
    op1631: u16,
}
impl adr32iVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_res: i128 = 0;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| i128::try_from(self.op3247).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.op1631).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_res.is_negative(),
            calc_res.abs() as u64,
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
        let mut calc_res: i128 = 0;
        let mut block_0_len = 2;
        let op1631 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_res = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_7(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap());
        let op3247 = token_7(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op3247, op1631 }))
    }
}
#[derive(Clone, Debug)]
enum Tableadr32i {
    Var0(adr32iVar0),
}
impl Tableadr32i {
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
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                adr32iVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:41:1, end:41:5))"]
#[derive(Clone, Debug)]
struct reg4Var0 {
    op0_1720: u8,
}
impl reg4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_3_display(self.op0_1720)];
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
        let op0_1720 = token_24(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op0_1720 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:42:1, end:42:5))"]
#[derive(Clone, Debug)]
struct reg4Var1 {
    op1_1720: u8,
}
impl reg4Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_4_display(self.op1_1720)];
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
        let op1_1720 = token_24(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { op1_1720 }))
    }
}
#[derive(Clone, Debug)]
enum Tablereg4 {
    Var0(reg4Var0),
    Var1(reg4Var1),
}
impl Tablereg4 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                reg4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                reg4Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:48:1, end:48:11))"]
#[derive(Clone, Debug)]
struct PrepList20Var0 {}
impl PrepList20Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r20)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:49:1, end:49:11))"]
#[derive(Clone, Debug)]
struct PrepList20Var1 {}
impl PrepList20Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList20 {
    Var0(PrepList20Var0),
    Var1(PrepList20Var1),
}
impl TablePrepList20 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                PrepList20Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList20Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:51:1, end:51:11))"]
#[derive(Clone, Debug)]
struct PrepList21Var0 {}
impl PrepList21Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r21)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:52:1, end:52:11))"]
#[derive(Clone, Debug)]
struct PrepList21Var1 {
    PrepList20: TablePrepList20,
}
impl PrepList21Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList20
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r21),
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
        let mut block_0_len = 4;
        let PrepList20 = if let Some((len, table)) =
            TablePrepList20::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:53:1, end:53:11))"]
#[derive(Clone, Debug)]
struct PrepList21Var2 {
    PrepList20: TablePrepList20,
}
impl PrepList21Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList20
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
        let PrepList20 = if let Some((len, table)) =
            TablePrepList20::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList20 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList21 {
    Var0(PrepList21Var0),
    Var1(PrepList21Var1),
    Var2(PrepList21Var2),
}
impl TablePrepList21 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 12) == 4 {
            if let Some((inst_len, parsed)) =
                PrepList21Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 4) == 4 {
            if let Some((inst_len, parsed)) =
                PrepList21Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 4) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList21Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:55:1, end:55:11))"]
#[derive(Clone, Debug)]
struct PrepList22Var0 {}
impl PrepList22Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r22)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:56:1, end:56:11))"]
#[derive(Clone, Debug)]
struct PrepList22Var1 {
    PrepList21: TablePrepList21,
}
impl PrepList22Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList21
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r22),
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
        let mut block_0_len = 4;
        let PrepList21 = if let Some((len, table)) =
            TablePrepList21::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:57:1, end:57:11))"]
#[derive(Clone, Debug)]
struct PrepList22Var2 {
    PrepList21: TablePrepList21,
}
impl PrepList22Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList21
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
        let PrepList21 = if let Some((len, table)) =
            TablePrepList21::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList21 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList22 {
    Var0(PrepList22Var0),
    Var1(PrepList22Var1),
    Var2(PrepList22Var2),
}
impl TablePrepList22 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 14) == 2 {
            if let Some((inst_len, parsed)) =
                PrepList22Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 2) == 2 {
            if let Some((inst_len, parsed)) =
                PrepList22Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 2) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList22Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:59:1, end:59:11))"]
#[derive(Clone, Debug)]
struct PrepList23Var0 {}
impl PrepList23Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r23)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:60:1, end:60:11))"]
#[derive(Clone, Debug)]
struct PrepList23Var1 {
    PrepList22: TablePrepList22,
}
impl PrepList23Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList22
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r23),
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
        let mut block_0_len = 4;
        let PrepList22 = if let Some((len, table)) =
            TablePrepList22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:61:1, end:61:11))"]
#[derive(Clone, Debug)]
struct PrepList23Var2 {
    PrepList22: TablePrepList22,
}
impl PrepList23Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList22
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
        let PrepList22 = if let Some((len, table)) =
            TablePrepList22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList22 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList23 {
    Var0(PrepList23Var0),
    Var1(PrepList23Var1),
    Var2(PrepList23Var2),
}
impl TablePrepList23 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                PrepList23Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                PrepList23Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList23Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:63:1, end:63:11))"]
#[derive(Clone, Debug)]
struct PrepList24Var0 {}
impl PrepList24Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r24)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:64:1, end:64:11))"]
#[derive(Clone, Debug)]
struct PrepList24Var1 {
    PrepList23: TablePrepList23,
}
impl PrepList24Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList23
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r24),
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
        let mut block_0_len = 4;
        let PrepList23 = if let Some((len, table)) =
            TablePrepList23::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:65:1, end:65:11))"]
#[derive(Clone, Debug)]
struct PrepList24Var2 {
    PrepList23: TablePrepList23,
}
impl PrepList24Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList23
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
        let PrepList23 = if let Some((len, table)) =
            TablePrepList23::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList23 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList24 {
    Var0(PrepList24Var0),
    Var1(PrepList24Var1),
    Var2(PrepList24Var2),
}
impl TablePrepList24 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 143) == 128 {
            if let Some((inst_len, parsed)) =
                PrepList24Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                PrepList24Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList24Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:67:1, end:67:11))"]
#[derive(Clone, Debug)]
struct PrepList25Var0 {}
impl PrepList25Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r25)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:68:1, end:68:11))"]
#[derive(Clone, Debug)]
struct PrepList25Var1 {
    PrepList24: TablePrepList24,
}
impl PrepList25Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList24
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r25),
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
        let mut block_0_len = 4;
        let PrepList24 = if let Some((len, table)) =
            TablePrepList24::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:69:1, end:69:11))"]
#[derive(Clone, Debug)]
struct PrepList25Var2 {
    PrepList24: TablePrepList24,
}
impl PrepList25Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList24
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
        let PrepList24 = if let Some((len, table)) =
            TablePrepList24::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList24 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList25 {
    Var0(PrepList25Var0),
    Var1(PrepList25Var1),
    Var2(PrepList25Var2),
}
impl TablePrepList25 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 207) == 64 {
            if let Some((inst_len, parsed)) =
                PrepList25Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 64) == 64 {
            if let Some((inst_len, parsed)) =
                PrepList25Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 64) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList25Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:71:1, end:71:11))"]
#[derive(Clone, Debug)]
struct PrepList26Var0 {}
impl PrepList26Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r26)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:72:1, end:72:11))"]
#[derive(Clone, Debug)]
struct PrepList26Var1 {
    PrepList25: TablePrepList25,
}
impl PrepList26Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList25
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r26),
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
        let mut block_0_len = 4;
        let PrepList25 = if let Some((len, table)) =
            TablePrepList25::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:73:1, end:73:11))"]
#[derive(Clone, Debug)]
struct PrepList26Var2 {
    PrepList25: TablePrepList25,
}
impl PrepList26Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList25
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
        let PrepList25 = if let Some((len, table)) =
            TablePrepList25::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList25 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList26 {
    Var0(PrepList26Var0),
    Var1(PrepList26Var1),
    Var2(PrepList26Var2),
}
impl TablePrepList26 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 239) == 32 {
            if let Some((inst_len, parsed)) =
                PrepList26Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                PrepList26Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList26Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:75:1, end:75:11))"]
#[derive(Clone, Debug)]
struct PrepList27Var0 {}
impl PrepList27Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r27)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:76:1, end:76:11))"]
#[derive(Clone, Debug)]
struct PrepList27Var1 {
    PrepList26: TablePrepList26,
}
impl PrepList27Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList26
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r27),
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
        let mut block_0_len = 4;
        let PrepList26 = if let Some((len, table)) =
            TablePrepList26::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:77:1, end:77:11))"]
#[derive(Clone, Debug)]
struct PrepList27Var2 {
    PrepList26: TablePrepList26,
}
impl PrepList27Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList26
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
        let PrepList26 = if let Some((len, table)) =
            TablePrepList26::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList26 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList27 {
    Var0(PrepList27Var0),
    Var1(PrepList27Var1),
    Var2(PrepList27Var2),
}
impl TablePrepList27 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[3] & 255) == 16 {
            if let Some((inst_len, parsed)) =
                PrepList27Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 16) == 16 {
            if let Some((inst_len, parsed)) =
                PrepList27Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 16) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList27Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:79:1, end:79:11))"]
#[derive(Clone, Debug)]
struct PrepList28Var0 {}
impl PrepList28Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r28)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:80:1, end:80:11))"]
#[derive(Clone, Debug)]
struct PrepList28Var1 {
    PrepList27: TablePrepList27,
}
impl PrepList28Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList27
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r28),
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
        let mut block_0_len = 4;
        let PrepList27 = if let Some((len, table)) =
            TablePrepList27::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:81:1, end:81:11))"]
#[derive(Clone, Debug)]
struct PrepList28Var2 {
    PrepList27: TablePrepList27,
}
impl PrepList28Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList27
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
        let PrepList27 = if let Some((len, table)) =
            TablePrepList27::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList27 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList28 {
    Var0(PrepList28Var0),
    Var1(PrepList28Var1),
    Var2(PrepList28Var2),
}
impl TablePrepList28 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[2] & 128) == 128 && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                PrepList28Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                PrepList28Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList28Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:83:1, end:83:11))"]
#[derive(Clone, Debug)]
struct PrepList29Var0 {}
impl PrepList29Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::r29)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:84:1, end:84:11))"]
#[derive(Clone, Debug)]
struct PrepList29Var1 {
    PrepList28: TablePrepList28,
}
impl PrepList29Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList28
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r29),
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
        let mut block_0_len = 4;
        let PrepList28 = if let Some((len, table)) =
            TablePrepList28::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:85:1, end:85:11))"]
#[derive(Clone, Debug)]
struct PrepList29Var2 {
    PrepList28: TablePrepList28,
}
impl PrepList29Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList28
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
        let PrepList28 = if let Some((len, table)) =
            TablePrepList28::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList28 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList29 {
    Var0(PrepList29Var0),
    Var1(PrepList29Var1),
    Var2(PrepList29Var2),
}
impl TablePrepList29 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4 && (tokens_param[2] & 192) == 64 && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                PrepList29Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 64) == 64 {
            if let Some((inst_len, parsed)) =
                PrepList29Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 64) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList29Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:87:1, end:87:11))"]
#[derive(Clone, Debug)]
struct PrepList30Var0 {}
impl PrepList30Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::ep)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:88:1, end:88:11))"]
#[derive(Clone, Debug)]
struct PrepList30Var1 {
    PrepList29: TablePrepList29,
}
impl PrepList30Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList29
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::ep),
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
        let mut block_0_len = 4;
        let PrepList29 = if let Some((len, table)) =
            TablePrepList29::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:89:1, end:89:11))"]
#[derive(Clone, Debug)]
struct PrepList30Var2 {
    PrepList29: TablePrepList29,
}
impl PrepList30Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.PrepList29
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
        let PrepList29 = if let Some((len, table)) =
            TablePrepList29::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList29 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList30 {
    Var0(PrepList30Var0),
    Var1(PrepList30Var1),
    Var2(PrepList30Var2),
}
impl TablePrepList30 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4
            && (tokens_param[0] & 1) == 1
            && (tokens_param[2] & 192) == 0
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                PrepList30Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                PrepList30Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                PrepList30Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:91:1, end:91:9))"]
#[derive(Clone, Debug)]
struct PrepListVar0 {}
impl PrepListVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("{"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::lp),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("}"),
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
        let mut block_0_len = 4;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:92:1, end:92:9))"]
#[derive(Clone, Debug)]
struct PrepListVar1 {
    PrepList30: TablePrepList30,
}
impl PrepListVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("{"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList30
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::lp),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("}"),
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
        let mut block_0_len = 4;
        let PrepList30 = if let Some((len, table)) =
            TablePrepList30::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:93:1, end:93:9))"]
#[derive(Clone, Debug)]
struct PrepListVar2 {
    PrepList30: TablePrepList30,
}
impl PrepListVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("{"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.PrepList30
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("}"),
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
        let mut block_0_len = 4;
        let PrepList30 = if let Some((len, table)) =
            TablePrepList30::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { PrepList30 }))
    }
}
#[derive(Clone, Debug)]
enum TablePrepList {
    Var0(PrepListVar0),
    Var1(PrepListVar1),
    Var2(PrepListVar2),
}
impl TablePrepList {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4
            && (tokens_param[0] & 1) == 0
            && (tokens_param[2] & 224) == 32
            && (tokens_param[3] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                PrepListVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                PrepListVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                PrepListVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:97:1, end:97:11))"]
#[derive(Clone, Debug)]
struct DispList31Var0 {}
impl DispList31Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::lp)];
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:98:1, end:98:11))"]
#[derive(Clone, Debug)]
struct DispList31Var1 {}
impl DispList31Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 4;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableDispList31 {
    Var0(DispList31Var0),
    Var1(DispList31Var1),
}
impl TableDispList31 {
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
        if tokens_param.len() >= 4 && (tokens_param[2] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                DispList31Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                DispList31Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:100:1, end:100:11))"]
#[derive(Clone, Debug)]
struct DispList30Var0 {
    DispList31: TableDispList31,
}
impl DispList30Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::ep),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList31
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
        let DispList31 = if let Some((len, table)) =
            TableDispList31::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList31 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:101:1, end:101:11))"]
#[derive(Clone, Debug)]
struct DispList30Var1 {
    DispList31: TableDispList31,
}
impl DispList30Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList31
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
        let DispList31 = if let Some((len, table)) =
            TableDispList31::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList31 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList30 {
    Var0(DispList30Var0),
    Var1(DispList30Var1),
}
impl TableDispList30 {
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
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                DispList30Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                DispList30Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:103:1, end:103:11))"]
#[derive(Clone, Debug)]
struct DispList29Var0 {
    DispList30: TableDispList30,
}
impl DispList29Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r29),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList30
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
        let DispList30 = if let Some((len, table)) =
            TableDispList30::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList30 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:104:1, end:104:11))"]
#[derive(Clone, Debug)]
struct DispList29Var1 {
    DispList30: TableDispList30,
}
impl DispList29Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList30
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
        let DispList30 = if let Some((len, table)) =
            TableDispList30::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList30 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList29 {
    Var0(DispList29Var0),
    Var1(DispList29Var1),
}
impl TableDispList29 {
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
        if tokens_param.len() >= 4 && (tokens_param[2] & 64) == 64 {
            if let Some((inst_len, parsed)) =
                DispList29Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 64) == 0 {
            if let Some((inst_len, parsed)) =
                DispList29Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:106:1, end:106:11))"]
#[derive(Clone, Debug)]
struct DispList28Var0 {
    DispList29: TableDispList29,
}
impl DispList28Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r28),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList29
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
        let DispList29 = if let Some((len, table)) =
            TableDispList29::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList29 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:107:1, end:107:11))"]
#[derive(Clone, Debug)]
struct DispList28Var1 {
    DispList29: TableDispList29,
}
impl DispList28Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList29
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
        let DispList29 = if let Some((len, table)) =
            TableDispList29::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList29 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList28 {
    Var0(DispList28Var0),
    Var1(DispList28Var1),
}
impl TableDispList28 {
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
        if tokens_param.len() >= 4 && (tokens_param[2] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                DispList28Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[2] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                DispList28Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:109:1, end:109:11))"]
#[derive(Clone, Debug)]
struct DispList27Var0 {
    DispList28: TableDispList28,
}
impl DispList27Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r27),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList28
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
        let DispList28 = if let Some((len, table)) =
            TableDispList28::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList28 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:110:1, end:110:11))"]
#[derive(Clone, Debug)]
struct DispList27Var1 {
    DispList28: TableDispList28,
}
impl DispList27Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList28
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
        let DispList28 = if let Some((len, table)) =
            TableDispList28::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList28 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList27 {
    Var0(DispList27Var0),
    Var1(DispList27Var1),
}
impl TableDispList27 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 16) == 16 {
            if let Some((inst_len, parsed)) =
                DispList27Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 16) == 0 {
            if let Some((inst_len, parsed)) =
                DispList27Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:112:1, end:112:11))"]
#[derive(Clone, Debug)]
struct DispList26Var0 {
    DispList27: TableDispList27,
}
impl DispList26Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r26),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList27
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
        let DispList27 = if let Some((len, table)) =
            TableDispList27::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList27 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:113:1, end:113:11))"]
#[derive(Clone, Debug)]
struct DispList26Var1 {
    DispList27: TableDispList27,
}
impl DispList26Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList27
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
        let DispList27 = if let Some((len, table)) =
            TableDispList27::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList27 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList26 {
    Var0(DispList26Var0),
    Var1(DispList26Var1),
}
impl TableDispList26 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                DispList26Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                DispList26Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:115:1, end:115:11))"]
#[derive(Clone, Debug)]
struct DispList25Var0 {
    DispList26: TableDispList26,
}
impl DispList25Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r25),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList26
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
        let DispList26 = if let Some((len, table)) =
            TableDispList26::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList26 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:116:1, end:116:11))"]
#[derive(Clone, Debug)]
struct DispList25Var1 {
    DispList26: TableDispList26,
}
impl DispList25Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList26
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
        let DispList26 = if let Some((len, table)) =
            TableDispList26::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList26 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList25 {
    Var0(DispList25Var0),
    Var1(DispList25Var1),
}
impl TableDispList25 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 64) == 64 {
            if let Some((inst_len, parsed)) =
                DispList25Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 64) == 0 {
            if let Some((inst_len, parsed)) =
                DispList25Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:118:1, end:118:11))"]
#[derive(Clone, Debug)]
struct DispList24Var0 {
    DispList25: TableDispList25,
}
impl DispList24Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r24),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList25
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
        let DispList25 = if let Some((len, table)) =
            TableDispList25::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList25 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:119:1, end:119:11))"]
#[derive(Clone, Debug)]
struct DispList24Var1 {
    DispList25: TableDispList25,
}
impl DispList24Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList25
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
        let DispList25 = if let Some((len, table)) =
            TableDispList25::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList25 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList24 {
    Var0(DispList24Var0),
    Var1(DispList24Var1),
}
impl TableDispList24 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                DispList24Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                DispList24Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:121:1, end:121:11))"]
#[derive(Clone, Debug)]
struct DispList23Var0 {
    DispList24: TableDispList24,
}
impl DispList23Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r23),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList24
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
        let DispList24 = if let Some((len, table)) =
            TableDispList24::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList24 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:122:1, end:122:11))"]
#[derive(Clone, Debug)]
struct DispList23Var1 {
    DispList24: TableDispList24,
}
impl DispList23Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList24
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
        let DispList24 = if let Some((len, table)) =
            TableDispList24::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList24 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList23 {
    Var0(DispList23Var0),
    Var1(DispList23Var1),
}
impl TableDispList23 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                DispList23Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                DispList23Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:124:1, end:124:11))"]
#[derive(Clone, Debug)]
struct DispList22Var0 {
    DispList23: TableDispList23,
}
impl DispList22Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r22),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList23
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
        let DispList23 = if let Some((len, table)) =
            TableDispList23::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList23 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:125:1, end:125:11))"]
#[derive(Clone, Debug)]
struct DispList22Var1 {
    DispList23: TableDispList23,
}
impl DispList22Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList23
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
        let DispList23 = if let Some((len, table)) =
            TableDispList23::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList23 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList22 {
    Var0(DispList22Var0),
    Var1(DispList22Var1),
}
impl TableDispList22 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 2) == 2 {
            if let Some((inst_len, parsed)) =
                DispList22Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 2) == 0 {
            if let Some((inst_len, parsed)) =
                DispList22Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:127:1, end:127:11))"]
#[derive(Clone, Debug)]
struct DispList21Var0 {
    DispList22: TableDispList22,
}
impl DispList21Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Register(Register::r21),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList22
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
        let DispList22 = if let Some((len, table)) =
            TableDispList22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList22 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:128:1, end:128:11))"]
#[derive(Clone, Debug)]
struct DispList21Var1 {
    DispList22: TableDispList22,
}
impl DispList21Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.DispList22
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
        let DispList22 = if let Some((len, table)) =
            TableDispList22::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList22 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList21 {
    Var0(DispList21Var0),
    Var1(DispList21Var1),
}
impl TableDispList21 {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 4) == 4 {
            if let Some((inst_len, parsed)) =
                DispList21Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 4) == 0 {
            if let Some((inst_len, parsed)) =
                DispList21Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:130:1, end:130:9))"]
#[derive(Clone, Debug)]
struct DispListVar0 {
    DispList21: TableDispList21,
}
impl DispListVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal("{"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r20),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DispList21
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("}"),
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
        let mut block_0_len = 4;
        let DispList21 = if let Some((len, table)) =
            TableDispList21::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList21 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/V850/data/languages/./Helpers/Extras.sinc, start:131:1, end:131:9))"]
#[derive(Clone, Debug)]
struct DispListVar1 {
    DispList21: TableDispList21,
}
impl DispListVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("{"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.DispList21
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("}"),
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
        let mut block_0_len = 4;
        let DispList21 = if let Some((len, table)) =
            TableDispList21::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DispList21 }))
    }
}
#[derive(Clone, Debug)]
enum TableDispList {
    Var0(DispListVar0),
    Var1(DispListVar1),
}
impl TableDispList {
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
        if tokens_param.len() >= 4 && (tokens_param[3] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                DispListVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[3] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                DispListVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
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
) -> Option<(u32, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
