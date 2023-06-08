pub type AddrType = u32;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
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
    sr,
    gbr,
    vbr,
    mach,
    macl,
    pr,
    pc,
    tbr,
    fr0,
    fr1,
    fr2,
    fr3,
    fr4,
    fr5,
    fr6,
    fr7,
    fr8,
    fr9,
    fr10,
    fr11,
    fr12,
    fr13,
    fr14,
    fr15,
    dr0,
    dr2,
    dr4,
    dr6,
    dr8,
    dr10,
    dr12,
    dr14,
    fpscr,
    fpul,
    resbank_base,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::r0 => "r0",
            Self::r1 => "r1",
            Self::r2 => "r2",
            Self::r3 => "r3",
            Self::r4 => "r4",
            Self::r5 => "r5",
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
            Self::sr => "sr",
            Self::gbr => "gbr",
            Self::vbr => "vbr",
            Self::mach => "mach",
            Self::macl => "macl",
            Self::pr => "pr",
            Self::pc => "pc",
            Self::tbr => "tbr",
            Self::fr0 => "fr0",
            Self::fr1 => "fr1",
            Self::fr2 => "fr2",
            Self::fr3 => "fr3",
            Self::fr4 => "fr4",
            Self::fr5 => "fr5",
            Self::fr6 => "fr6",
            Self::fr7 => "fr7",
            Self::fr8 => "fr8",
            Self::fr9 => "fr9",
            Self::fr10 => "fr10",
            Self::fr11 => "fr11",
            Self::fr12 => "fr12",
            Self::fr13 => "fr13",
            Self::fr14 => "fr14",
            Self::fr15 => "fr15",
            Self::dr0 => "dr0",
            Self::dr2 => "dr2",
            Self::dr4 => "dr4",
            Self::dr6 => "dr6",
            Self::dr8 => "dr8",
            Self::dr10 => "dr10",
            Self::dr12 => "dr12",
            Self::dr14 => "dr14",
            Self::fpscr => "fpscr",
            Self::fpul => "fpul",
            Self::resbank_base => "resbank_base",
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
        3 => Register::r3,
        4 => Register::r4,
        5 => Register::r5,
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
        0 => Register::r0,
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::r3,
        4 => Register::r4,
        5 => Register::r5,
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
        0 => Register::fr0,
        1 => Register::fr1,
        2 => Register::fr2,
        3 => Register::fr3,
        4 => Register::fr4,
        5 => Register::fr5,
        6 => Register::fr6,
        7 => Register::fr7,
        8 => Register::fr8,
        9 => Register::fr9,
        10 => Register::fr10,
        11 => Register::fr11,
        12 => Register::fr12,
        13 => Register::fr13,
        14 => Register::fr14,
        15 => Register::fr15,
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
        1 => Register::r1,
        2 => Register::r2,
        3 => Register::r3,
        4 => Register::r4,
        5 => Register::r5,
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
        0 => Register::dr0,
        1 => Register::dr2,
        2 => Register::dr4,
        3 => Register::dr6,
        4 => Register::dr8,
        5 => Register::dr10,
        6 => Register::dr12,
        7 => Register::dr14,
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
        0 => <DisplayElement>::Literal("r0"),
        1 => <DisplayElement>::Literal("r1"),
        2 => <DisplayElement>::Literal("r2"),
        3 => <DisplayElement>::Literal("r3"),
        4 => <DisplayElement>::Literal("r4"),
        5 => <DisplayElement>::Literal("r5"),
        6 => <DisplayElement>::Literal("r6"),
        7 => <DisplayElement>::Literal("r7"),
        8 => <DisplayElement>::Literal("r8"),
        9 => <DisplayElement>::Literal("r9"),
        10 => <DisplayElement>::Literal("r10"),
        11 => <DisplayElement>::Literal("r11"),
        12 => <DisplayElement>::Literal("r12"),
        13 => <DisplayElement>::Literal("r13"),
        14 => <DisplayElement>::Literal("r14"),
        15 => <DisplayElement>::Literal("pr"),
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
#[doc = "Create token_fields: l_disp_00_11 lfdisp_00_11"]
fn token_11(tokens: &[u8]) -> u16 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: l_opcode_12_15"]
fn token_12(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: opcode_08_15"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 255) as u8)
}
#[doc = "Create token_fields: l_rm_20_23 l_simm20_20_23 lffrm_20_23 lf_rm_20_23 lffrn_20_23"]
fn token_16(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 20) & 15) as u8)
}
#[doc = "Create token_fields: l_imm3_20_22"]
fn token_20(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 20) & 7) as u8)
}
#[doc = "Create token_fields: fop_04_04"]
fn token_22(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: lfop_12_19"]
fn token_25(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 12) & 255) as u8)
}
#[doc = "Create token_fields: fop_08_08"]
fn token_21(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 1) as u8)
}
#[doc = "Create token_fields: l_opcode_23_23"]
fn token_14(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 23) & 1) as u8)
}
#[doc = "Create token_fields: disp_00_07 sdisp_00_07 imm_00_07 simm_00_07 opcode_00_07 fop_00_07"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: opcode_03_03"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 1) as u8)
}
#[doc = "Create token_fields: opcode_04_07 rm_04_07 rn_04_07 fop_04_07 ffrn_04_07 ffrm_04_07 f_rm_04_07"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: l_opcode_28_31 lfop_28_31"]
fn token_18(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 28) & 15) as u8)
}
#[doc = "Create token_fields: fdrn_09_11 fdrm_09_11"]
fn token_23(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 7) as u8)
}
#[doc = "Create token_fields: disp_00_11 sdisp_00_11"]
fn token_3(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: l_opcode_24_31"]
fn token_15(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 255) as u8)
}
#[doc = "Create token_fields: fdrn_05_07 fdrm_05_07"]
fn token_24(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 7) as u8)
}
#[doc = "Create token_fields: opcode_12_15 fop_12_15"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: disp_00_03 sdisp_00_03 opcode_00_03 fop_00_03"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: l_imm20_00_15"]
fn token_19(tokens: &[u8]) -> u16 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: l_rn_24_27 lffrm_24_27 lffrn_24_27 lf_rn_24_27"]
fn token_17(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 15) as u8)
}
#[doc = "Create token_fields: l_opcode_16_19"]
fn token_13(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 15) as u8)
}
#[doc = "Create token_fields: opcode_08_11 rm_08_11 rn_08_11 rm_imm_08_11 rn_imm_08_11 ffrn_08_11 ffrm_08_11 f_rn_08_11"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 15) as u8)
}
#[doc = "Create token_fields: imm3_00_02"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: opcode_00_15 fop_00_15"]
fn token_5(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:821:1, end:821:2))"]
#[derive(Clone, Debug)]
struct nott_instructionVar0 {}
impl nott_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("nott"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1011:1, end:1011:2))"]
#[derive(Clone, Debug)]
struct div0u_instructionVar1 {}
impl div0u_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div0u"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1685:1, end:1685:2))"]
#[derive(Clone, Debug)]
struct rts_instructionVar2 {}
impl rts_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rts"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1713:1, end:1713:2))"]
#[derive(Clone, Debug)]
struct rts_instructionVar3 {}
impl rts_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rts"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("/n")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1731:1, end:1731:2))"]
#[derive(Clone, Debug)]
struct clrmac_instructionVar4 {}
impl clrmac_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clrmac"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1737:1, end:1737:2))"]
#[derive(Clone, Debug)]
struct clrt_instructionVar5 {}
impl clrt_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clrt"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1767:1, end:1767:2))"]
#[derive(Clone, Debug)]
struct resbank_instructionVar6 {}
impl resbank_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("resbank"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1860:1, end:1860:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar7 {}
impl nop_instructionVar7 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1866:1, end:1866:2))"]
#[derive(Clone, Debug)]
struct rte_instructionVar8 {}
impl rte_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rte"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1882:1, end:1882:2))"]
#[derive(Clone, Debug)]
struct sett_instructionVar9 {}
impl sett_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sett"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1889:1, end:1889:2))"]
#[derive(Clone, Debug)]
struct sleep_instructionVar10 {}
impl sleep_instructionVar10 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2177:1, end:2177:2))"]
#[derive(Clone, Debug)]
struct fschg_instructionVar11 {}
impl fschg_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fschg"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2031:1, end:2031:2))"]
#[derive(Clone, Debug)]
struct fcnvds_instructionVar12 {
    fdrm_09_11: u8,
}
impl fcnvds_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcnvds"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.fdrm_09_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
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
        let fdrm_09_11 = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fdrm_09_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2037:1, end:2037:2))"]
#[derive(Clone, Debug)]
struct fcnvsd_instructionVar13 {
    fdrn_09_11: u8,
}
impl fcnvsd_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcnvsd"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_4_display(self.fdrn_09_11),
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
        let fdrn_09_11 = token_23(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fdrn_09_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar14 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let mut sub_pattern_c99 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if i128::try_from(token_7(tokens)).unwrap() != i128::try_from(token_8(tokens)).unwrap()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c99(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:257:1, end:257:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar15 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let mut sub_pattern_c99 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if i128::try_from(token_7(tokens)).unwrap() != i128::try_from(token_8(tokens)).unwrap()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c99(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:269:1, end:269:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar16 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let mut sub_pattern_c99 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if i128::try_from(token_7(tokens)).unwrap() != i128::try_from(token_8(tokens)).unwrap()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c99(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:388:1, end:388:2))"]
#[derive(Clone, Debug)]
struct movt_instructionVar17 {
    rn_08_11: u8,
}
impl movt_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:396:1, end:396:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar18 {
    rn_08_11: u8,
}
impl mov_b_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar19 {
    rn_08_11: u8,
}
impl mov_w_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:412:1, end:412:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar20 {
    rn_08_11: u8,
}
impl mov_l_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:420:1, end:420:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar21 {
    rm_08_11: u8,
}
impl mov_b_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:428:1, end:428:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar22 {
    rm_08_11: u8,
}
impl mov_w_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:436:1, end:436:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar23 {
    rm_08_11: u8,
}
impl mov_l_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2291:1, end:2291:2))"]
#[derive(Clone, Debug)]
struct band_b_instructionVar24 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl band_b_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("band.b"));
        let extend: [DisplayElement; 10usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_imm3_20_22 = token_20(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2299:1, end:2299:2))"]
#[derive(Clone, Debug)]
struct bandnot_b_instructionVar25 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bandnot_b_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bandnot.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_imm3_20_22 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2307:1, end:2307:2))"]
#[derive(Clone, Debug)]
struct bclr_b_instructionVar26 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bclr_b_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bclr.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_imm3_20_22 = token_20(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2324:1, end:2324:2))"]
#[derive(Clone, Debug)]
struct bld_b_instructionVar27 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bld_b_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bld.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_imm3_20_22 = token_20(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2340:1, end:2340:2))"]
#[derive(Clone, Debug)]
struct bldnot_b_instructionVar28 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bldnot_b_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bldnot.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_imm3_20_22 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2348:1, end:2348:2))"]
#[derive(Clone, Debug)]
struct bor_b_instructionVar29 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bor_b_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bor.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_imm3_20_22 = token_20(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2357:1, end:2357:2))"]
#[derive(Clone, Debug)]
struct bornot_b_instructionVar30 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bornot_b_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bornot.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_imm3_20_22 = token_20(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2366:1, end:2366:2))"]
#[derive(Clone, Debug)]
struct bset_b_instructionVar31 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bset_b_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bset.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_imm3_20_22 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2382:1, end:2382:2))"]
#[derive(Clone, Debug)]
struct bst_b_instructionVar32 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bst_b_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bst.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_imm3_20_22 = token_20(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2400:1, end:2400:2))"]
#[derive(Clone, Debug)]
struct bxor_b_instructionVar33 {
    l_imm3_20_22: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl bxor_b_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bxor.b"));
        let extend: [DisplayElement; 11usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.l_imm3_20_22 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_imm3_20_22 = token_20(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_imm3_20_22,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:444:1, end:444:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar34 {
    l_rm_20_23: u8,
    l_disp_00_11: u16,
    l_rn_24_27: u8,
}
impl mov_b_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 10usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let l_rm_20_23 = token_16(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_disp_00_11,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:450:1, end:450:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar35 {
    l_rm_20_23: u8,
    l_rn_24_27: u8,
    l_disp_00_11: u16,
}
impl mov_w_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = 2i128.wrapping_mul(i128::try_from(self.l_disp_00_11).unwrap());
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 10usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 4;
        calc_disp = 2i128.wrapping_mul(i128::try_from(token_11(tokens_current)).unwrap());
        let l_rn_24_27 = token_17(tokens_current);
        let l_rm_20_23 = token_16(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_rn_24_27,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:458:1, end:458:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar36 {
    l_rm_20_23: u8,
    l_rn_24_27: u8,
    l_disp_00_11: u16,
}
impl mov_l_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = 4i128.wrapping_mul(i128::try_from(self.l_disp_00_11).unwrap());
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 10usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 4;
        calc_disp = 4i128.wrapping_mul(i128::try_from(token_11(tokens_current)).unwrap());
        let l_rm_20_23 = token_16(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_rn_24_27,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:466:1, end:466:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar37 {
    l_disp_00_11: u16,
    l_rm_20_23: u8,
    l_rn_24_27: u8,
}
impl mov_b_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let l_rn_24_27 = token_17(tokens_current);
        let l_rm_20_23 = token_16(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_disp_00_11,
                l_rm_20_23,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:472:1, end:472:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar38 {
    l_rm_20_23: u8,
    l_rn_24_27: u8,
    l_disp_00_11: u16,
}
impl mov_w_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = 2i128.wrapping_mul(i128::try_from(self.l_disp_00_11).unwrap());
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 4;
        calc_disp = 2i128.wrapping_mul(i128::try_from(token_11(tokens_current)).unwrap());
        let l_rn_24_27 = token_17(tokens_current);
        let l_rm_20_23 = token_16(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_rn_24_27,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:480:1, end:480:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar39 {
    l_rm_20_23: u8,
    l_rn_24_27: u8,
    l_disp_00_11: u16,
}
impl mov_l_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = 4i128.wrapping_mul(i128::try_from(self.l_disp_00_11).unwrap());
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 4;
        calc_disp = 4i128.wrapping_mul(i128::try_from(token_11(tokens_current)).unwrap());
        let l_disp_00_11 = token_11(tokens_current);
        let l_rm_20_23 = token_16(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_rn_24_27,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:488:1, end:488:2))"]
#[derive(Clone, Debug)]
struct movu_b_instructionVar40 {
    l_disp_00_11: u16,
    l_rm_20_23: u8,
    l_rn_24_27: u8,
}
impl movu_b_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movu.b"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.l_disp_00_11 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let l_disp_00_11 = token_11(tokens_current);
        let l_rm_20_23 = token_16(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_disp_00_11,
                l_rm_20_23,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:495:1, end:495:2))"]
#[derive(Clone, Debug)]
struct movu_w_instructionVar41 {
    l_rm_20_23: u8,
    l_rn_24_27: u8,
    l_disp_00_11: u16,
}
impl movu_w_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = i128::try_from(self.l_disp_00_11)
            .unwrap()
            .wrapping_mul(2i128);
        display.push(DisplayElement::Literal("movu.w"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 4;
        calc_disp = i128::try_from(token_11(tokens_current))
            .unwrap()
            .wrapping_mul(2i128);
        let l_rm_20_23 = token_16(tokens_current);
        let l_disp_00_11 = token_11(tokens_current);
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_rm_20_23,
                l_rn_24_27,
                l_disp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:606:1, end:606:2))"]
#[derive(Clone, Debug)]
struct movml_l_instructionVar42 {
    MovMLReg1: TableMovMLReg1,
}
impl movml_l_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movml.l"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.MovMLReg1
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            <DisplayElement>::Register(Register::r15),
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
        let MovMLReg1 = if let Some((len, table)) =
            TableMovMLReg1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:691:1, end:691:2))"]
#[derive(Clone, Debug)]
struct movml_l_instructionVar43 {
    MovMLReg2: TableMovMLReg2,
}
impl movml_l_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movml.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Register(Register::r15),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.MovMLReg2
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
        let MovMLReg2 = if let Some((len, table)) =
            TableMovMLReg2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rn_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:749:1, end:749:2))"]
#[derive(Clone, Debug)]
struct movmu_l_instructionVar44 {
    MovMUReg1: TableMovMUReg1,
}
impl movmu_l_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movmu.l"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.MovMUReg1
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            <DisplayElement>::Register(Register::r15),
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
        let MovMUReg1 = if let Some((len, table)) =
            TableMovMUReg1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:807:1, end:807:2))"]
#[derive(Clone, Debug)]
struct movmu_l_instructionVar45 {
    MovMUReg2: TableMovMUReg2,
}
impl movmu_l_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movmu.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Register(Register::r15),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.MovMUReg2
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
        let MovMUReg2 = if let Some((len, table)) =
            TableMovMUReg2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rn_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:815:1, end:815:2))"]
#[derive(Clone, Debug)]
struct movrt_instructionVar46 {
    rn_08_11: u8,
}
impl movrt_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movrt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:935:1, end:935:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar47 {
    rn_08_11: u8,
}
impl cmp_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("/pl"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:940:1, end:940:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar48 {
    rn_08_11: u8,
}
impl cmp_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("/pz"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:967:1, end:967:2))"]
#[derive(Clone, Debug)]
struct clips_b_instructionVar49 {
    rn_08_11: u8,
}
impl clips_b_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clips.b"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:976:1, end:976:2))"]
#[derive(Clone, Debug)]
struct clips_w_instructionVar50 {
    rn_08_11: u8,
}
impl clips_w_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clips.w"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:985:1, end:985:2))"]
#[derive(Clone, Debug)]
struct clipu_b_instructionVar51 {
    rn_08_11: u8,
}
impl clipu_b_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clipu.b"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:993:1, end:993:2))"]
#[derive(Clone, Debug)]
struct clipu_w_instructionVar52 {
    rn_08_11: u8,
}
impl clipu_w_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("clipu.w"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1048:1, end:1048:2))"]
#[derive(Clone, Debug)]
struct divs_instructionVar53 {
    rn_08_11: u8,
}
impl divs_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divs"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1053:1, end:1053:2))"]
#[derive(Clone, Debug)]
struct divu_instructionVar54 {
    rn_08_11: u8,
}
impl divu_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("divu"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1081:1, end:1081:2))"]
#[derive(Clone, Debug)]
struct dt_instructionVar55 {
    rn_08_11: u8,
}
impl dt_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1300:1, end:1300:2))"]
#[derive(Clone, Debug)]
struct mulr_instructionVar56 {
    rn_08_11: u8,
}
impl mulr_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulr"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1413:1, end:1413:2))"]
#[derive(Clone, Debug)]
struct tas_b_instructionVar57 {
    rn_08_11: u8,
}
impl tas_b_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tas.b"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1465:1, end:1465:2))"]
#[derive(Clone, Debug)]
struct rotcl_instructionVar58 {
    rn_08_11: u8,
}
impl rotcl_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rotcl"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1476:1, end:1476:2))"]
#[derive(Clone, Debug)]
struct rotcr_instructionVar59 {
    rn_08_11: u8,
}
impl rotcr_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rotcr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1487:1, end:1487:2))"]
#[derive(Clone, Debug)]
struct rotl_instructionVar60 {
    rn_08_11: u8,
}
impl rotl_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rotl"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1494:1, end:1494:2))"]
#[derive(Clone, Debug)]
struct rotr_instructionVar61 {
    rn_08_11: u8,
}
impl rotr_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rotr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1518:1, end:1518:2))"]
#[derive(Clone, Debug)]
struct shal_instructionVar62 {
    rn_08_11: u8,
}
impl shal_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shal"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1526:1, end:1526:2))"]
#[derive(Clone, Debug)]
struct shar_instructionVar63 {
    rn_08_11: u8,
}
impl shar_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shar"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1550:1, end:1550:2))"]
#[derive(Clone, Debug)]
struct shll_instructionVar64 {
    rn_08_11: u8,
}
impl shll_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shll"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1558:1, end:1558:2))"]
#[derive(Clone, Debug)]
struct shll2_instructionVar65 {
    rn_08_11: u8,
}
impl shll2_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shll2"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1563:1, end:1563:2))"]
#[derive(Clone, Debug)]
struct shll8_instructionVar66 {
    rn_08_11: u8,
}
impl shll8_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shll8"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1568:1, end:1568:2))"]
#[derive(Clone, Debug)]
struct shll16_instructionVar67 {
    rn_08_11: u8,
}
impl shll16_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shll16"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1573:1, end:1573:2))"]
#[derive(Clone, Debug)]
struct shlr_instructionVar68 {
    rn_08_11: u8,
}
impl shlr_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shlr"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1581:1, end:1581:2))"]
#[derive(Clone, Debug)]
struct shlr2_instructionVar69 {
    rn_08_11: u8,
}
impl shlr2_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shlr2"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1586:1, end:1586:2))"]
#[derive(Clone, Debug)]
struct shlr8_instructionVar70 {
    rn_08_11: u8,
}
impl shlr8_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shlr8"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1591:1, end:1591:2))"]
#[derive(Clone, Debug)]
struct shlr16_instructionVar71 {
    rn_08_11: u8,
}
impl shlr16_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shlr16"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1634:1, end:1634:2))"]
#[derive(Clone, Debug)]
struct braf_instructionVar72 {
    rm_08_11: u8,
}
impl braf_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("braf"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1653:1, end:1653:2))"]
#[derive(Clone, Debug)]
struct bsrf_instructionVar73 {
    rm_08_11: u8,
}
impl bsrf_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bsrf"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar74 {
    rm_08_11: u8,
}
impl jmp_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jmp"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1673:1, end:1673:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar75 {
    rm_08_11: u8,
}
impl jsr_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jsr"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1696:1, end:1696:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar76 {
    rm_08_11: u8,
}
impl jsr_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jsr"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal("/n"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1719:1, end:1719:2))"]
#[derive(Clone, Debug)]
struct rtv_instructionVar77 {
    rm_08_11: u8,
}
impl rtv_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rtv"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("/n"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1743:1, end:1743:2))"]
#[derive(Clone, Debug)]
struct ldbank_instructionVar78 {
    rm_08_11: u8,
}
impl ldbank_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldbank"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1755:1, end:1755:2))"]
#[derive(Clone, Debug)]
struct stbank_instructionVar79 {
    rn_08_11: u8,
}
impl stbank_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stbank"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1775:1, end:1775:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar80 {
    rm_08_11: u8,
}
impl ldc_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::sr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1780:1, end:1780:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar81 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::sr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1787:1, end:1787:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar82 {
    rm_08_11: u8,
}
impl ldc_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1793:1, end:1793:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar83 {
    rm_08_11: u8,
}
impl ldc_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::tbr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1799:1, end:1799:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar84 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::gbr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1805:1, end:1805:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar85 {
    rm_08_11: u8,
}
impl ldc_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::vbr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1810:1, end:1810:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar86 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::vbr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1816:1, end:1816:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar87 {
    rm_08_11: u8,
}
impl lds_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::mach),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1826:1, end:1826:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar88 {
    rm_08_11: u8,
}
impl lds_l_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::mach),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1838:1, end:1838:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar89 {
    rm_08_11: u8,
}
impl lds_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::macl),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1843:1, end:1843:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar90 {
    rm_08_11: u8,
}
impl lds_l_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::macl),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1849:1, end:1849:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar91 {
    rm_08_11: u8,
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
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::pr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1854:1, end:1854:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar92 {
    rm_08_11: u8,
}
impl lds_l_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Register(Register::pr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1894:1, end:1894:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar93 {
    rn_08_11: u8,
}
impl stc_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::sr),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1899:1, end:1899:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar94 {
    rn_08_11: u8,
}
impl stc_l_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::sr),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1905:1, end:1905:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar95 {
    rn_08_11: u8,
}
impl stc_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1911:1, end:1911:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar96 {
    rn_08_11: u8,
}
impl stc_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::tbr),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1917:1, end:1917:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar97 {
    rn_08_11: u8,
}
impl stc_l_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1923:1, end:1923:2))"]
#[derive(Clone, Debug)]
struct stc_instructionVar98 {
    rn_08_11: u8,
}
impl stc_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::vbr),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1928:1, end:1928:2))"]
#[derive(Clone, Debug)]
struct stc_l_instructionVar99 {
    rn_08_11: u8,
}
impl stc_l_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("stc.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::vbr),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1934:1, end:1934:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar100 {
    rn_08_11: u8,
}
impl sts_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::mach),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1944:1, end:1944:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar101 {
    rn_08_11: u8,
}
impl sts_l_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::mach),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1958:1, end:1958:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar102 {
    rn_08_11: u8,
}
impl sts_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::macl),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1963:1, end:1963:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar103 {
    rn_08_11: u8,
}
impl sts_l_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::macl),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1969:1, end:1969:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar104 {
    rn_08_11: u8,
}
impl sts_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::pr),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1974:1, end:1974:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar105 {
    rn_08_11: u8,
}
impl sts_l_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::pr),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1999:1, end:1999:2))"]
#[derive(Clone, Debug)]
struct fabs_instructionVar106 {
    ffrn_08_11: u8,
}
impl fabs_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fabs"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2051:1, end:2051:2))"]
#[derive(Clone, Debug)]
struct fldi0_instructionVar107 {
    ffrn_08_11: u8,
}
impl fldi0_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fldi0"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2057:1, end:2057:2))"]
#[derive(Clone, Debug)]
struct fldi1_instructionVar108 {
    ffrn_08_11: u8,
}
impl fldi1_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fldi1"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2064:1, end:2064:2))"]
#[derive(Clone, Debug)]
struct flds_instructionVar109 {
    ffrm_08_11: u8,
}
impl flds_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("flds"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
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
        let ffrm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2070:1, end:2070:2))"]
#[derive(Clone, Debug)]
struct float_instructionVar110 {
    ffrn_08_11: u8,
}
impl float_instructionVar110 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("float"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2117:1, end:2117:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar111 {
    lf_rm_20_23: u8,
    lffrn_24_27: u8,
    lfdisp_00_11: u16,
}
impl fmov_s_instructionVar111 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp12: i128 = 0;
        calc_disp12 = i128::try_from(self.lfdisp_00_11)
            .unwrap()
            .wrapping_mul(4i128);
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp12.is_negative(), calc_disp12.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lf_rm_20_23),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.lffrn_24_27),
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
        let mut calc_disp12: i128 = 0;
        let mut block_0_len = 4;
        calc_disp12 = i128::try_from(token_11(tokens_current))
            .unwrap()
            .wrapping_mul(4i128);
        let lfdisp_00_11 = token_11(tokens_current);
        let lf_rm_20_23 = token_16(tokens_current);
        let lffrn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                lf_rm_20_23,
                lffrn_24_27,
                lfdisp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2152:1, end:2152:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar112 {
    lffrm_20_23: u8,
    lf_rn_24_27: u8,
    lfdisp_00_11: u16,
}
impl fmov_s_instructionVar112 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp12: i128 = 0;
        calc_disp12 = i128::try_from(self.lfdisp_00_11)
            .unwrap()
            .wrapping_mul(4i128);
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 10usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.lffrm_20_23),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp12.is_negative(), calc_disp12.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.lf_rn_24_27),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp12: i128 = 0;
        let mut block_0_len = 4;
        calc_disp12 = i128::try_from(token_11(tokens_current))
            .unwrap()
            .wrapping_mul(4i128);
        let lffrm_20_23 = token_16(tokens_current);
        let lfdisp_00_11 = token_11(tokens_current);
        let lf_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                lffrm_20_23,
                lf_rn_24_27,
                lfdisp_00_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2170:1, end:2170:2))"]
#[derive(Clone, Debug)]
struct fneg_instructionVar113 {
    ffrn_08_11: u8,
}
impl fneg_instructionVar113 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fneg"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2183:1, end:2183:2))"]
#[derive(Clone, Debug)]
struct fsqrt_instructionVar114 {
    ffrn_08_11: u8,
}
impl fsqrt_instructionVar114 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fsqrt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2190:1, end:2190:2))"]
#[derive(Clone, Debug)]
struct fsts_instructionVar115 {
    ffrn_08_11: u8,
}
impl fsts_instructionVar115 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fsts"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2204:1, end:2204:2))"]
#[derive(Clone, Debug)]
struct ftrc_instructionVar116 {
    ffrm_08_11: u8,
}
impl ftrc_instructionVar116 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ftrc"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
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
        let ffrm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ffrm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2221:1, end:2221:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar117 {
    rm_08_11: u8,
}
impl lds_instructionVar117 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpscr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2227:1, end:2227:2))"]
#[derive(Clone, Debug)]
struct lds_instructionVar118 {
    rm_08_11: u8,
}
impl lds_instructionVar118 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2233:1, end:2233:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar119 {
    rm_08_11: u8,
}
impl lds_l_instructionVar119 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpscr),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2240:1, end:2240:2))"]
#[derive(Clone, Debug)]
struct lds_l_instructionVar120 {
    rm_08_11: u8,
}
impl lds_l_instructionVar120 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lds.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_08_11),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
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
        let rm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2247:1, end:2247:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar121 {
    rn_08_11: u8,
}
impl sts_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpscr),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2253:1, end:2253:2))"]
#[derive(Clone, Debug)]
struct sts_instructionVar122 {
    rn_08_11: u8,
}
impl sts_instructionVar122 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2259:1, end:2259:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar123 {
    rn_08_11: u8,
}
impl sts_l_instructionVar123 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpscr),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2266:1, end:2266:2))"]
#[derive(Clone, Debug)]
struct sts_l_instructionVar124 {
    rn_08_11: u8,
}
impl sts_l_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sts.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fpul),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:179:1, end:179:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar125 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_instructionVar125 {
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
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2316:1, end:2316:2))"]
#[derive(Clone, Debug)]
struct bclr_instructionVar126 {
    imm3_00_02: u8,
    rn_04_07: u8,
}
impl bclr_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bclr"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm3_00_02 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_04_07),
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
        let imm3_00_02 = token_4(tokens_current);
        let rn_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                imm3_00_02,
                rn_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2332:1, end:2332:2))"]
#[derive(Clone, Debug)]
struct bld_instructionVar127 {
    imm3_00_02: u8,
    rn_04_07: u8,
}
impl bld_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bld"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm3_00_02 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_04_07),
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
        let rn_04_07 = token_7(tokens_current);
        let imm3_00_02 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                imm3_00_02,
                rn_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2375:1, end:2375:2))"]
#[derive(Clone, Debug)]
struct bset_instructionVar128 {
    imm3_00_02: u8,
    rn_04_07: u8,
}
impl bset_instructionVar128 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bset"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm3_00_02 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_04_07),
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
        let imm3_00_02 = token_4(tokens_current);
        let rn_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                imm3_00_02,
                rn_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2392:1, end:2392:2))"]
#[derive(Clone, Debug)]
struct bst_instructionVar129 {
    imm3_00_02: u8,
    rn_04_07: u8,
}
impl bst_instructionVar129 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bst"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm3_00_02 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_04_07),
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
        let rn_04_07 = token_7(tokens_current);
        let imm3_00_02 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                imm3_00_02,
                rn_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:199:1, end:199:2))"]
#[derive(Clone, Debug)]
struct mova_instructionVar130 {
    disppc4: Tabledisppc4,
}
impl mova_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mova"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.disppc4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let disppc4 = if let Some((len, table)) =
            Tabledisppc4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc4 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar131 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar132 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:224:1, end:224:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar133 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar133 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar134 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar134 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar135 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar135 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:239:1, end:239:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar136 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar136 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar137 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar137 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:262:1, end:262:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar138 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar138 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let opcode_04_07 = token_7(tokens_current);
        let opcode_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:274:1, end:274:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar139 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar139 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_1_display(self.rn_08_11),
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
        let opcode_04_07 = token_7(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        let opcode_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:280:1, end:280:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar140 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar140 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar141 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar141 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:292:1, end:292:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar142 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar142 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar143 {
    disp_00_03: u8,
    rm_04_07: u8,
}
impl mov_b_instructionVar143 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.disp_00_03 as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::r0),
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
        let rm_04_07 = token_7(tokens_current);
        let disp_00_03 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                disp_00_03,
                rm_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:303:1, end:303:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar144 {
    rm_04_07: u8,
    disp_00_03: u8,
}
impl mov_w_instructionVar144 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_03).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::r0),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let rm_04_07 = token_7(tokens_current);
        let disp_00_03 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rm_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:313:1, end:313:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar145 {
    disp_00_03: u8,
    rn_04_07: u8,
}
impl mov_b_instructionVar145 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(",@("),
            DisplayElement::Number(true, false, self.disp_00_03 as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_04_07),
            <DisplayElement>::Literal(")"),
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
        let rn_04_07 = token_7(tokens_current);
        let disp_00_03 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                disp_00_03,
                rn_04_07,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:318:1, end:318:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar146 {
    rn_04_07: u8,
    disp_00_03: u8,
}
impl mov_w_instructionVar146 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_03).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_04_07),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let disp_00_03 = token_1(tokens_current);
        let rn_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rn_04_07,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:328:1, end:328:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar147 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar147 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:333:1, end:333:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar148 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar148 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:338:1, end:338:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar149 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar149 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:343:1, end:343:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar150 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar150 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
            <DisplayElement>::Literal(")"),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:348:1, end:348:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar151 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar151 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
            <DisplayElement>::Literal(")"),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:353:1, end:353:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar152 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar152 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
            <DisplayElement>::Literal(")"),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:358:1, end:358:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar153 {
    disp_00_07: u8,
}
impl mov_b_instructionVar153 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            DisplayElement::Number(true, false, self.disp_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::r0),
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
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:363:1, end:363:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar154 {
    disp_00_07: u8,
}
impl mov_w_instructionVar154 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::r0),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar155 {
    disp_00_07: u8,
}
impl mov_l_instructionVar155 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::r0),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:373:1, end:373:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar156 {
    disp_00_07: u8,
}
impl mov_b_instructionVar156 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(",@("),
            DisplayElement::Number(true, false, self.disp_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:378:1, end:378:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar157 {
    disp_00_07: u8,
}
impl mov_w_instructionVar157 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:383:1, end:383:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar158 {
    disp_00_07: u8,
}
impl mov_l_instructionVar158 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:512:1, end:512:2))"]
#[derive(Clone, Debug)]
struct movi20_instructionVar159 {
    l_rn_24_27: u8,
    simm20: Tablesimm20,
}
impl movi20_instructionVar159 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movi20"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.simm20
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let simm20 = if let Some((len, table)) =
            Tablesimm20::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm20, l_rn_24_27 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:519:1, end:519:2))"]
#[derive(Clone, Debug)]
struct movi20s_instructionVar160 {
    l_rn_24_27: u8,
    simm20s: Tablesimm20s,
}
impl movi20s_instructionVar160 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("movi20s"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.simm20s
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.l_rn_24_27),
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
        let simm20s = if let Some((len, table)) =
            Tablesimm20s::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let l_rn_24_27 = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                simm20s,
                l_rn_24_27,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:828:1, end:828:2))"]
#[derive(Clone, Debug)]
struct swap_b_instructionVar161 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl swap_b_instructionVar161 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("swap.b"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:840:1, end:840:2))"]
#[derive(Clone, Debug)]
struct swap_w_instructionVar162 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl swap_w_instructionVar162 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("swap.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:850:1, end:850:2))"]
#[derive(Clone, Debug)]
struct xtrct_instructionVar163 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl xtrct_instructionVar163 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xtrct"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:863:1, end:863:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar164 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl add_instructionVar164 {
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
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:873:1, end:873:2))"]
#[derive(Clone, Debug)]
struct addc_instructionVar165 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl addc_instructionVar165 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("addc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:886:1, end:886:2))"]
#[derive(Clone, Debug)]
struct addv_instructionVar166 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl addv_instructionVar166 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("addv"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:905:1, end:905:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar167 {
    simm_00_07: u8,
}
impl cmp_instructionVar167 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/eq"),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
                    .is_negative(),
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let simm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:910:1, end:910:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar168 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar168 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/eq"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:915:1, end:915:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar169 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar169 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/hs"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:920:1, end:920:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar170 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar170 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/ge"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:925:1, end:925:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar171 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar171 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/hi"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:930:1, end:930:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar172 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar172 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/gt"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:945:1, end:945:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar173 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar173 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("cmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("/str"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct div0s_instructionVar174 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl div0s_instructionVar174 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div0s"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1018:1, end:1018:2))"]
#[derive(Clone, Debug)]
struct div1_instructionVar175 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl div1_instructionVar175 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div1"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1061:1, end:1061:2))"]
#[derive(Clone, Debug)]
struct dmuls_l_instructionVar176 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl dmuls_l_instructionVar176 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dmuls.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1071:1, end:1071:2))"]
#[derive(Clone, Debug)]
struct dmulu_l_instructionVar177 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl dmulu_l_instructionVar177 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dmulu.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1088:1, end:1088:2))"]
#[derive(Clone, Debug)]
struct exts_b_instructionVar178 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl exts_b_instructionVar178 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("exts.b"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1094:1, end:1094:2))"]
#[derive(Clone, Debug)]
struct exts_w_instructionVar179 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl exts_w_instructionVar179 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("exts.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1100:1, end:1100:2))"]
#[derive(Clone, Debug)]
struct extu_b_instructionVar180 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl extu_b_instructionVar180 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("extu.b"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1105:1, end:1105:2))"]
#[derive(Clone, Debug)]
struct extu_w_instructionVar181 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl extu_w_instructionVar181 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("extu.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1111:1, end:1111:2))"]
#[derive(Clone, Debug)]
struct mac_l_instructionVar182 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mac_l_instructionVar182 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mac.l"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,@"),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1220:1, end:1220:2))"]
#[derive(Clone, Debug)]
struct mac_w_instructionVar183 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mac_w_instructionVar183 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mac.w"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("+,@"),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1293:1, end:1293:2))"]
#[derive(Clone, Debug)]
struct mul_l_instructionVar184 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mul_l_instructionVar184 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul.l"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1306:1, end:1306:2))"]
#[derive(Clone, Debug)]
struct muls_w_instructionVar185 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl muls_w_instructionVar185 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("muls.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1311:1, end:1311:2))"]
#[derive(Clone, Debug)]
struct mulu_w_instructionVar186 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mulu_w_instructionVar186 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mulu.w"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1316:1, end:1316:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar187 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl neg_instructionVar187 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1321:1, end:1321:2))"]
#[derive(Clone, Debug)]
struct negc_instructionVar188 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl negc_instructionVar188 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("negc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1333:1, end:1333:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar189 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl sub_instructionVar189 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1338:1, end:1338:2))"]
#[derive(Clone, Debug)]
struct subc_instructionVar190 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl subc_instructionVar190 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subc"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1351:1, end:1351:2))"]
#[derive(Clone, Debug)]
struct subv_instructionVar191 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl subv_instructionVar191 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("subv"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1372:1, end:1372:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar192 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl and_instructionVar192 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1377:1, end:1377:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar193 {
    imm_00_07: u8,
}
impl and_instructionVar193 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1382:1, end:1382:2))"]
#[derive(Clone, Debug)]
struct and_b_instructionVar194 {
    imm_00_07: u8,
}
impl and_b_instructionVar194 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1390:1, end:1390:2))"]
#[derive(Clone, Debug)]
struct not_instructionVar195 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl not_instructionVar195 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("not"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1395:1, end:1395:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar196 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl or_instructionVar196 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1400:1, end:1400:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar197 {
    imm_00_07: u8,
}
impl or_instructionVar197 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1405:1, end:1405:2))"]
#[derive(Clone, Debug)]
struct or_b_instructionVar198 {
    imm_00_07: u8,
}
impl or_b_instructionVar198 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1424:1, end:1424:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar199 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl tst_instructionVar199 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1429:1, end:1429:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar200 {
    imm_00_07: u8,
}
impl tst_instructionVar200 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1436:1, end:1436:2))"]
#[derive(Clone, Debug)]
struct tst_b_instructionVar201 {
    imm_00_07: u8,
}
impl tst_b_instructionVar201 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tst.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1444:1, end:1444:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar202 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl xor_instructionVar202 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1449:1, end:1449:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar203 {
    imm_00_07: u8,
}
impl xor_instructionVar203 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::r0),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1454:1, end:1454:2))"]
#[derive(Clone, Debug)]
struct xor_b_instructionVar204 {
    imm_00_07: u8,
}
impl xor_b_instructionVar204 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor.b"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::gbr),
            <DisplayElement>::Literal(")"),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1502:1, end:1502:2))"]
#[derive(Clone, Debug)]
struct shad_instructionVar205 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl shad_instructionVar205 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shad"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1534:1, end:1534:2))"]
#[derive(Clone, Debug)]
struct shld_instructionVar206 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl shld_instructionVar206 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("shld"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rn_08_11),
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
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1599:1, end:1599:2))"]
#[derive(Clone, Debug)]
struct bf_instructionVar207 {
    target00_07: Tabletarget00_07,
}
impl bf_instructionVar207 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bf"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_07
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
        let target00_07 = if let Some((len, table)) =
            Tabletarget00_07::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1605:1, end:1605:2))"]
#[derive(Clone, Debug)]
struct bf_instructionVar208 {
    target00_07: Tabletarget00_07,
}
impl bf_instructionVar208 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bf"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("/s"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.target00_07
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
        let target00_07 = if let Some((len, table)) =
            Tabletarget00_07::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1613:1, end:1613:2))"]
#[derive(Clone, Debug)]
struct bt_instructionVar209 {
    target00_07: Tabletarget00_07,
}
impl bt_instructionVar209 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_07
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
        let target00_07 = if let Some((len, table)) =
            Tabletarget00_07::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1619:1, end:1619:2))"]
#[derive(Clone, Debug)]
struct bt_instructionVar210 {
    target00_07: Tabletarget00_07,
}
impl bt_instructionVar210 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bt"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("/s"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.target00_07
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
        let target00_07 = if let Some((len, table)) =
            Tabletarget00_07::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1704:1, end:1704:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar211 {
    disp_00_07: u8,
}
impl jsr_instructionVar211 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = i128::try_from(self.disp_00_07).unwrap().wrapping_mul(4i128);
        display.push(DisplayElement::Literal("jsr"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal("/n"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::tbr),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = i128::try_from(token_2(tokens_current))
            .unwrap()
            .wrapping_mul(4i128);
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1980:1, end:1980:2))"]
#[derive(Clone, Debug)]
struct trapa_instructionVar212 {
    imm_00_07: u8,
}
impl trapa_instructionVar212 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("trapa"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm_00_07 as u64),
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
        let imm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm_00_07 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2007:1, end:2007:2))"]
#[derive(Clone, Debug)]
struct fadd_instructionVar213 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fadd_instructionVar213 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fadd"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2015:1, end:2015:2))"]
#[derive(Clone, Debug)]
struct fcmp_instructionVar214 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fcmp_instructionVar214 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcmp"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal("/eq"),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrm_04_07 = token_7(tokens_current);
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2023:1, end:2023:2))"]
#[derive(Clone, Debug)]
struct fcmp_instructionVar215 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fcmp_instructionVar215 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcmp"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal("/gt"),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2043:1, end:2043:2))"]
#[derive(Clone, Debug)]
struct fdiv_instructionVar216 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fdiv_instructionVar216 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fdiv"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2078:1, end:2078:2))"]
#[derive(Clone, Debug)]
struct fmac_instructionVar217 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmac_instructionVar217 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmac"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::fr0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrm_04_07 = token_7(tokens_current);
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2084:1, end:2084:2))"]
#[derive(Clone, Debug)]
struct fmov_instructionVar218 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmov_instructionVar218 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrm_04_07 = token_7(tokens_current);
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2092:1, end:2092:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar219 {
    f_rm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmov_s_instructionVar219 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 9usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.f_rm_04_07),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let f_rm_04_07 = token_7(tokens_current);
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2100:1, end:2100:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar220 {
    f_rm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmov_s_instructionVar220 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_3_display(self.f_rm_04_07),
            <DisplayElement>::Literal("+,"),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let f_rm_04_07 = token_7(tokens_current);
        let ffrn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2109:1, end:2109:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar221 {
    f_rm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmov_s_instructionVar221 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_3_display(self.f_rm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let f_rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                f_rm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2127:1, end:2127:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar222 {
    ffrm_04_07: u8,
    f_rn_08_11: u8,
}
impl fmov_s_instructionVar222 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 12usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_3_display(self.f_rn_08_11),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal(")"),
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
        let f_rn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                f_rn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2135:1, end:2135:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar223 {
    ffrm_04_07: u8,
    f_rn_08_11: u8,
}
impl fmov_s_instructionVar223 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@-"),
            meaning_3_display(self.f_rn_08_11),
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
        let f_rn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                f_rn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2144:1, end:2144:2))"]
#[derive(Clone, Debug)]
struct fmov_s_instructionVar224 {
    ffrm_04_07: u8,
    f_rn_08_11: u8,
}
impl fmov_s_instructionVar224 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmov.s"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@"),
            meaning_3_display(self.f_rn_08_11),
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
        let ffrm_04_07 = token_7(tokens_current);
        let f_rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                f_rn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2163:1, end:2163:2))"]
#[derive(Clone, Debug)]
struct fmul_instructionVar225 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fmul_instructionVar225 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:2196:1, end:2196:2))"]
#[derive(Clone, Debug)]
struct fsub_instructionVar226 {
    ffrm_04_07: u8,
    ffrn_08_11: u8,
}
impl fsub_instructionVar226 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fsub"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrm_04_07),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.ffrn_08_11),
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
        let ffrn_08_11 = token_8(tokens_current);
        let ffrm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                ffrm_04_07,
                ffrn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:186:1, end:186:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar227 {
    rn_08_11: u8,
    imm8: Tableimm8,
}
impl mov_instructionVar227 {
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
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let imm8 = if let Some((len, table)) =
            Tableimm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar228 {
    rn_08_11: u8,
    disppc2: Tabledisppc2,
}
impl mov_w_instructionVar228 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.disppc2
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let disppc2 = if let Some((len, table)) =
            Tabledisppc2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc2, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:209:1, end:209:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar229 {
    rn_08_11: u8,
    disppc4: Tabledisppc4,
}
impl mov_l_instructionVar229 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.disppc4
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let disppc4 = if let Some((len, table)) =
            Tabledisppc4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disppc4, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:308:1, end:308:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar230 {
    rm_04_07: u8,
    rn_08_11: u8,
    disp_00_03: u8,
}
impl mov_l_instructionVar230 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_03).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_1_display(self.rn_08_11),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let rm_04_07 = token_7(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        let disp_00_03 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rm_04_07,
                rn_08_11,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:323:1, end:323:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar231 {
    rm_04_07: u8,
    rn_08_11: u8,
    disp_00_03: u8,
}
impl mov_l_instructionVar231 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_03).unwrap().checked_shl(shl))
            .unwrap_or(0);
        display.push(DisplayElement::Literal("mov.l"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        let disp_00_03 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                rm_04_07,
                rn_08_11,
                disp_00_03,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:868:1, end:868:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar232 {
    simm_00_07: u8,
    rn_08_11: u8,
}
impl add_instructionVar232 {
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
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
                    .is_negative(),
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
                    .abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.rn_08_11),
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
        let simm_00_07 = token_2(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                simm_00_07,
                rn_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1627:1, end:1627:2))"]
#[derive(Clone, Debug)]
struct bra_instructionVar233 {
    target00_11: Tabletarget00_11,
}
impl bra_instructionVar233 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bra"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_11
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
        let target00_11 = if let Some((len, table)) =
            Tabletarget00_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1642:1, end:1642:2))"]
#[derive(Clone, Debug)]
struct bsr_instructionVar234 {
    target00_11: Tabletarget00_11,
}
impl bsr_instructionVar234 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bsr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.target00_11
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
        let target00_11 = if let Some((len, table)) =
            Tabletarget00_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { target00_11 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(nott_instructionVar0),
    Var1(div0u_instructionVar1),
    Var2(rts_instructionVar2),
    Var3(rts_instructionVar3),
    Var4(clrmac_instructionVar4),
    Var5(clrt_instructionVar5),
    Var6(resbank_instructionVar6),
    Var7(nop_instructionVar7),
    Var8(rte_instructionVar8),
    Var9(sett_instructionVar9),
    Var10(sleep_instructionVar10),
    Var11(fschg_instructionVar11),
    Var12(fcnvds_instructionVar12),
    Var13(fcnvsd_instructionVar13),
    Var14(mov_b_instructionVar14),
    Var15(mov_w_instructionVar15),
    Var16(mov_l_instructionVar16),
    Var17(movt_instructionVar17),
    Var18(mov_b_instructionVar18),
    Var19(mov_w_instructionVar19),
    Var20(mov_l_instructionVar20),
    Var21(mov_b_instructionVar21),
    Var22(mov_w_instructionVar22),
    Var23(mov_l_instructionVar23),
    Var24(band_b_instructionVar24),
    Var25(bandnot_b_instructionVar25),
    Var26(bclr_b_instructionVar26),
    Var27(bld_b_instructionVar27),
    Var28(bldnot_b_instructionVar28),
    Var29(bor_b_instructionVar29),
    Var30(bornot_b_instructionVar30),
    Var31(bset_b_instructionVar31),
    Var32(bst_b_instructionVar32),
    Var33(bxor_b_instructionVar33),
    Var34(mov_b_instructionVar34),
    Var35(mov_w_instructionVar35),
    Var36(mov_l_instructionVar36),
    Var37(mov_b_instructionVar37),
    Var38(mov_w_instructionVar38),
    Var39(mov_l_instructionVar39),
    Var40(movu_b_instructionVar40),
    Var41(movu_w_instructionVar41),
    Var42(movml_l_instructionVar42),
    Var43(movml_l_instructionVar43),
    Var44(movmu_l_instructionVar44),
    Var45(movmu_l_instructionVar45),
    Var46(movrt_instructionVar46),
    Var47(cmp_instructionVar47),
    Var48(cmp_instructionVar48),
    Var49(clips_b_instructionVar49),
    Var50(clips_w_instructionVar50),
    Var51(clipu_b_instructionVar51),
    Var52(clipu_w_instructionVar52),
    Var53(divs_instructionVar53),
    Var54(divu_instructionVar54),
    Var55(dt_instructionVar55),
    Var56(mulr_instructionVar56),
    Var57(tas_b_instructionVar57),
    Var58(rotcl_instructionVar58),
    Var59(rotcr_instructionVar59),
    Var60(rotl_instructionVar60),
    Var61(rotr_instructionVar61),
    Var62(shal_instructionVar62),
    Var63(shar_instructionVar63),
    Var64(shll_instructionVar64),
    Var65(shll2_instructionVar65),
    Var66(shll8_instructionVar66),
    Var67(shll16_instructionVar67),
    Var68(shlr_instructionVar68),
    Var69(shlr2_instructionVar69),
    Var70(shlr8_instructionVar70),
    Var71(shlr16_instructionVar71),
    Var72(braf_instructionVar72),
    Var73(bsrf_instructionVar73),
    Var74(jmp_instructionVar74),
    Var75(jsr_instructionVar75),
    Var76(jsr_instructionVar76),
    Var77(rtv_instructionVar77),
    Var78(ldbank_instructionVar78),
    Var79(stbank_instructionVar79),
    Var80(ldc_instructionVar80),
    Var81(ldc_l_instructionVar81),
    Var82(ldc_instructionVar82),
    Var83(ldc_instructionVar83),
    Var84(ldc_l_instructionVar84),
    Var85(ldc_instructionVar85),
    Var86(ldc_l_instructionVar86),
    Var87(lds_instructionVar87),
    Var88(lds_l_instructionVar88),
    Var89(lds_instructionVar89),
    Var90(lds_l_instructionVar90),
    Var91(lds_instructionVar91),
    Var92(lds_l_instructionVar92),
    Var93(stc_instructionVar93),
    Var94(stc_l_instructionVar94),
    Var95(stc_instructionVar95),
    Var96(stc_instructionVar96),
    Var97(stc_l_instructionVar97),
    Var98(stc_instructionVar98),
    Var99(stc_l_instructionVar99),
    Var100(sts_instructionVar100),
    Var101(sts_l_instructionVar101),
    Var102(sts_instructionVar102),
    Var103(sts_l_instructionVar103),
    Var104(sts_instructionVar104),
    Var105(sts_l_instructionVar105),
    Var106(fabs_instructionVar106),
    Var107(fldi0_instructionVar107),
    Var108(fldi1_instructionVar108),
    Var109(flds_instructionVar109),
    Var110(float_instructionVar110),
    Var111(fmov_s_instructionVar111),
    Var112(fmov_s_instructionVar112),
    Var113(fneg_instructionVar113),
    Var114(fsqrt_instructionVar114),
    Var115(fsts_instructionVar115),
    Var116(ftrc_instructionVar116),
    Var117(lds_instructionVar117),
    Var118(lds_instructionVar118),
    Var119(lds_l_instructionVar119),
    Var120(lds_l_instructionVar120),
    Var121(sts_instructionVar121),
    Var122(sts_instructionVar122),
    Var123(sts_l_instructionVar123),
    Var124(sts_l_instructionVar124),
    Var125(mov_instructionVar125),
    Var126(bclr_instructionVar126),
    Var127(bld_instructionVar127),
    Var128(bset_instructionVar128),
    Var129(bst_instructionVar129),
    Var130(mova_instructionVar130),
    Var131(mov_b_instructionVar131),
    Var132(mov_w_instructionVar132),
    Var133(mov_l_instructionVar133),
    Var134(mov_b_instructionVar134),
    Var135(mov_w_instructionVar135),
    Var136(mov_l_instructionVar136),
    Var137(mov_b_instructionVar137),
    Var138(mov_w_instructionVar138),
    Var139(mov_l_instructionVar139),
    Var140(mov_b_instructionVar140),
    Var141(mov_w_instructionVar141),
    Var142(mov_l_instructionVar142),
    Var143(mov_b_instructionVar143),
    Var144(mov_w_instructionVar144),
    Var145(mov_b_instructionVar145),
    Var146(mov_w_instructionVar146),
    Var147(mov_b_instructionVar147),
    Var148(mov_w_instructionVar148),
    Var149(mov_l_instructionVar149),
    Var150(mov_b_instructionVar150),
    Var151(mov_w_instructionVar151),
    Var152(mov_l_instructionVar152),
    Var153(mov_b_instructionVar153),
    Var154(mov_w_instructionVar154),
    Var155(mov_l_instructionVar155),
    Var156(mov_b_instructionVar156),
    Var157(mov_w_instructionVar157),
    Var158(mov_l_instructionVar158),
    Var159(movi20_instructionVar159),
    Var160(movi20s_instructionVar160),
    Var161(swap_b_instructionVar161),
    Var162(swap_w_instructionVar162),
    Var163(xtrct_instructionVar163),
    Var164(add_instructionVar164),
    Var165(addc_instructionVar165),
    Var166(addv_instructionVar166),
    Var167(cmp_instructionVar167),
    Var168(cmp_instructionVar168),
    Var169(cmp_instructionVar169),
    Var170(cmp_instructionVar170),
    Var171(cmp_instructionVar171),
    Var172(cmp_instructionVar172),
    Var173(cmp_instructionVar173),
    Var174(div0s_instructionVar174),
    Var175(div1_instructionVar175),
    Var176(dmuls_l_instructionVar176),
    Var177(dmulu_l_instructionVar177),
    Var178(exts_b_instructionVar178),
    Var179(exts_w_instructionVar179),
    Var180(extu_b_instructionVar180),
    Var181(extu_w_instructionVar181),
    Var182(mac_l_instructionVar182),
    Var183(mac_w_instructionVar183),
    Var184(mul_l_instructionVar184),
    Var185(muls_w_instructionVar185),
    Var186(mulu_w_instructionVar186),
    Var187(neg_instructionVar187),
    Var188(negc_instructionVar188),
    Var189(sub_instructionVar189),
    Var190(subc_instructionVar190),
    Var191(subv_instructionVar191),
    Var192(and_instructionVar192),
    Var193(and_instructionVar193),
    Var194(and_b_instructionVar194),
    Var195(not_instructionVar195),
    Var196(or_instructionVar196),
    Var197(or_instructionVar197),
    Var198(or_b_instructionVar198),
    Var199(tst_instructionVar199),
    Var200(tst_instructionVar200),
    Var201(tst_b_instructionVar201),
    Var202(xor_instructionVar202),
    Var203(xor_instructionVar203),
    Var204(xor_b_instructionVar204),
    Var205(shad_instructionVar205),
    Var206(shld_instructionVar206),
    Var207(bf_instructionVar207),
    Var208(bf_instructionVar208),
    Var209(bt_instructionVar209),
    Var210(bt_instructionVar210),
    Var211(jsr_instructionVar211),
    Var212(trapa_instructionVar212),
    Var213(fadd_instructionVar213),
    Var214(fcmp_instructionVar214),
    Var215(fcmp_instructionVar215),
    Var216(fdiv_instructionVar216),
    Var217(fmac_instructionVar217),
    Var218(fmov_instructionVar218),
    Var219(fmov_s_instructionVar219),
    Var220(fmov_s_instructionVar220),
    Var221(fmov_s_instructionVar221),
    Var222(fmov_s_instructionVar222),
    Var223(fmov_s_instructionVar223),
    Var224(fmov_s_instructionVar224),
    Var225(fmul_instructionVar225),
    Var226(fsub_instructionVar226),
    Var227(mov_instructionVar227),
    Var228(mov_w_instructionVar228),
    Var229(mov_l_instructionVar229),
    Var230(mov_l_instructionVar230),
    Var231(mov_l_instructionVar231),
    Var232(add_instructionVar232),
    Var233(bra_instructionVar233),
    Var234(bsr_instructionVar234),
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
            Self::Var216(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var217(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var218(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var219(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var220(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var221(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var222(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var223(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var224(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var225(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var226(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var227(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var228(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var229(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var230(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var231(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var232(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var233(x) => {
                x.display_extend(display, context, inst_start, inst_next, global_set_param)
            }
            Self::Var234(x) => {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 104
        {
            if let Some((inst_len, parsed)) =
                nott_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                div0u_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 11
        {
            if let Some((inst_len, parsed)) =
                rts_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 107
        {
            if let Some((inst_len, parsed)) =
                rts_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 40
        {
            if let Some((inst_len, parsed)) =
                clrmac_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 8 {
            if let Some((inst_len, parsed)) =
                clrt_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 91
        {
            if let Some((inst_len, parsed)) =
                resbank_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 9 {
            if let Some((inst_len, parsed)) =
                nop_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                rte_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                sett_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 27
        {
            if let Some((inst_len, parsed)) =
                sleep_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 243
            && (tokens_param[1] & 255) == 253
        {
            if let Some((inst_len, parsed)) =
                fschg_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 241) == 240
            && (tokens_param[1] & 255) == 189
        {
            if let Some((inst_len, parsed)) =
                fcnvds_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 241) == 240
            && (tokens_param[1] & 255) == 173
        {
            if let Some((inst_len, parsed)) =
                fcnvsd_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 41
        {
            if let Some((inst_len, parsed)) =
                movt_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 139
        {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 155
        {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 203
        {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 219
        {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 235
        {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 64
        {
            if let Some((inst_len, parsed)) =
                band_b_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 192
        {
            if let Some((inst_len, parsed)) =
                bandnot_b_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                bclr_b_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 48
        {
            if let Some((inst_len, parsed)) =
                bld_b_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 176
        {
            if let Some((inst_len, parsed)) =
                bldnot_b_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 80
        {
            if let Some((inst_len, parsed)) =
                bor_b_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 208
        {
            if let Some((inst_len, parsed)) =
                bornot_b_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 16
        {
            if let Some((inst_len, parsed)) =
                bset_b_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 32
        {
            if let Some((inst_len, parsed)) =
                bst_b_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 143) == 9
            && (tokens_param[2] & 240) == 96
        {
            if let Some((inst_len, parsed)) =
                bxor_b_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 0
        {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 16
        {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 32
        {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 64
        {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 80
        {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 96
        {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 128
        {
            if let Some((inst_len, parsed)) =
                movu_b_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 144
        {
            if let Some((inst_len, parsed)) =
                movu_w_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 241
        {
            if let Some((inst_len, parsed)) =
                movml_l_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 245
        {
            if let Some((inst_len, parsed)) =
                movml_l_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 240
        {
            if let Some((inst_len, parsed)) =
                movmu_l_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 244
        {
            if let Some((inst_len, parsed)) =
                movmu_l_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 57
        {
            if let Some((inst_len, parsed)) =
                movrt_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 21
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 17
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 145
        {
            if let Some((inst_len, parsed)) =
                clips_b_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                clips_w_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 129
        {
            if let Some((inst_len, parsed)) =
                clipu_b_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 133
        {
            if let Some((inst_len, parsed)) =
                clipu_w_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                divs_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 132
        {
            if let Some((inst_len, parsed)) =
                divu_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 16
        {
            if let Some((inst_len, parsed)) =
                dt_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 128
        {
            if let Some((inst_len, parsed)) =
                mulr_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 27
        {
            if let Some((inst_len, parsed)) =
                tas_b_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 36
        {
            if let Some((inst_len, parsed)) =
                rotcl_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 37
        {
            if let Some((inst_len, parsed)) =
                rotcr_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 4
        {
            if let Some((inst_len, parsed)) =
                rotl_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 5
        {
            if let Some((inst_len, parsed)) =
                rotr_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 32
        {
            if let Some((inst_len, parsed)) =
                shal_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 33
        {
            if let Some((inst_len, parsed)) =
                shar_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                shll_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 8
        {
            if let Some((inst_len, parsed)) =
                shll2_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                shll8_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 40
        {
            if let Some((inst_len, parsed)) =
                shll16_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                shlr_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 9
        {
            if let Some((inst_len, parsed)) =
                shlr2_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                shlr8_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 41
        {
            if let Some((inst_len, parsed)) =
                shlr16_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                braf_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 3 {
            if let Some((inst_len, parsed)) =
                bsrf_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 11
        {
            if let Some((inst_len, parsed)) =
                jsr_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 75
        {
            if let Some((inst_len, parsed)) =
                jsr_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 123
        {
            if let Some((inst_len, parsed)) =
                rtv_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 229
        {
            if let Some((inst_len, parsed)) =
                ldbank_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 225
        {
            if let Some((inst_len, parsed)) =
                stbank_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 7
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 74
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 23
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 39
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 10
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 26
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                stc_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 18
        {
            if let Some((inst_len, parsed)) =
                stc_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 74
        {
            if let Some((inst_len, parsed)) =
                stc_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 19
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                stc_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 10
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 26
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 18
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 93
        {
            if let Some((inst_len, parsed)) =
                fabs_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 141
        {
            if let Some((inst_len, parsed)) =
                fldi0_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 157
        {
            if let Some((inst_len, parsed)) =
                fldi1_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 29
        {
            if let Some((inst_len, parsed)) =
                flds_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 45
        {
            if let Some((inst_len, parsed)) =
                float_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 112
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 240) == 48
            && (tokens_param[1] & 15) == 1
            && (tokens_param[2] & 240) == 48
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 77
        {
            if let Some((inst_len, parsed)) =
                fneg_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 109
        {
            if let Some((inst_len, parsed)) =
                fsqrt_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 13
        {
            if let Some((inst_len, parsed)) =
                fsts_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 240
            && (tokens_param[1] & 255) == 61
        {
            if let Some((inst_len, parsed)) =
                ftrc_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 106
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 90
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 240) == 64
            && (tokens_param[1] & 255) == 102
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 86
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 106
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 90
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 98
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 82
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 134 && (tokens_param[1] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                bclr_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 135 && (tokens_param[1] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                bld_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 134 && (tokens_param[1] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                bset_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 135 && (tokens_param[1] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                bst_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 199 {
            if let Some((inst_len, parsed)) =
                mova_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar135::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar136::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var136(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar137::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var137(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar138::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var138(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar139::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var139(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar140::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var140(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar141::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var141(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar142::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var142(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 132 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar143::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var143(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 133 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar144::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var144(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 128 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar145::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var145(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 129 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar146::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var146(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar147::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var147(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar148::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var148(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar149::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var149(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar150::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var150(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar151::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var151(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar152::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var152(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 196 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar153::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var153(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 197 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar154::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var154(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar155::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var155(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 192 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar156::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var156(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 193 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar157::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var157(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 194 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar158::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var158(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                movi20_instructionVar159::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var159(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                movi20s_instructionVar160::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var160(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                swap_b_instructionVar161::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var161(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                swap_w_instructionVar162::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var162(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                xtrct_instructionVar163::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var163(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                add_instructionVar164::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var164(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                addc_instructionVar165::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var165(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                addv_instructionVar166::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var166(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 136 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar167::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var167(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar168::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var168(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar169::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var169(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar170::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var170(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar171::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var171(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar172::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var172(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar173::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var173(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                div0s_instructionVar174::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var174(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                div1_instructionVar175::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var175(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                dmuls_l_instructionVar176::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var176(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                dmulu_l_instructionVar177::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var177(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                exts_b_instructionVar178::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var178(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                exts_w_instructionVar179::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var179(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                extu_b_instructionVar180::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var180(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                extu_w_instructionVar181::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var181(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                mac_l_instructionVar182::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var182(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                mac_w_instructionVar183::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var183(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                mul_l_instructionVar184::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var184(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                muls_w_instructionVar185::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var185(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                mulu_w_instructionVar186::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var186(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                neg_instructionVar187::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var187(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                negc_instructionVar188::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var188(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar189::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var189(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                subc_instructionVar190::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var190(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                subv_instructionVar191::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var191(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                and_instructionVar192::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var192(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) =
                and_instructionVar193::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var193(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                and_b_instructionVar194::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var194(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                not_instructionVar195::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var195(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                or_instructionVar196::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var196(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 {
            if let Some((inst_len, parsed)) =
                or_instructionVar197::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var197(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 207 {
            if let Some((inst_len, parsed)) =
                or_b_instructionVar198::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var198(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                tst_instructionVar199::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var199(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 200 {
            if let Some((inst_len, parsed)) =
                tst_instructionVar200::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var200(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 204 {
            if let Some((inst_len, parsed)) =
                tst_b_instructionVar201::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var201(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                xor_instructionVar202::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var202(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 202 {
            if let Some((inst_len, parsed)) =
                xor_instructionVar203::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var203(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) =
                xor_b_instructionVar204::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var204(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                shad_instructionVar205::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var205(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                shld_instructionVar206::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var206(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 139 {
            if let Some((inst_len, parsed)) =
                bf_instructionVar207::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var207(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 143 {
            if let Some((inst_len, parsed)) =
                bf_instructionVar208::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var208(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 137 {
            if let Some((inst_len, parsed)) =
                bt_instructionVar209::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var209(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 141 {
            if let Some((inst_len, parsed)) =
                bt_instructionVar210::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var210(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 131 {
            if let Some((inst_len, parsed)) =
                jsr_instructionVar211::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var211(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) =
                trapa_instructionVar212::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var212(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                fadd_instructionVar213::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var213(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 4
        {
            if let Some((inst_len, parsed)) =
                fcmp_instructionVar214::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var214(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 5
        {
            if let Some((inst_len, parsed)) =
                fcmp_instructionVar215::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var215(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 3
        {
            if let Some((inst_len, parsed)) =
                fdiv_instructionVar216::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var216(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                fmac_instructionVar217::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var217(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                fmov_instructionVar218::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var218(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 6
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar219::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var219(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 9
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar220::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var220(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 8
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar221::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var221(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 7
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar222::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var222(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar223::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var223(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                fmov_s_instructionVar224::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var224(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 2
        {
            if let Some((inst_len, parsed)) =
                fmul_instructionVar225::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var225(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 && (tokens_param[1] & 15) == 1
        {
            if let Some((inst_len, parsed)) =
                fsub_instructionVar226::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var226(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 224 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar227::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var227(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 144 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar228::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var228(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 208 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar229::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var229(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 80 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar230::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var230(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 16 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar231::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var231(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 112 {
            if let Some((inst_len, parsed)) =
                add_instructionVar232::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var232(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 160 {
            if let Some((inst_len, parsed)) =
                bra_instructionVar233::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var233(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 176 {
            if let Some((inst_len, parsed)) =
                bsr_instructionVar234::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var234(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:166:1, end:166:12))"]
#[derive(Clone, Debug)]
struct target00_07Var0 {
    sdisp_00_07: u8,
}
impl target00_07Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_target: i128 = 0;
        calc_target = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.sdisp_00_07 & 128 != 0 {
                        -1 & !127
                    } else {
                        0
                    } | self.sdisp_00_07 as i8),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap())
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_target.is_negative(),
            calc_target.abs() as u64,
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
        let mut calc_target: i128 = 0;
        let mut block_0_len = 2;
        calc_target = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap())
            .wrapping_add(4i128);
        let sdisp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sdisp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabletarget00_07 {
    Var0(target00_07Var0),
}
impl Tabletarget00_07 {
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
                target00_07Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:170:1, end:170:12))"]
#[derive(Clone, Debug)]
struct target00_11Var0 {
    sdisp_00_11: u16,
}
impl target00_11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_target: i128 = 0;
        calc_target = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.sdisp_00_11 & 2048 != 0 {
                        -1 & !2047
                    } else {
                        0
                    } | self.sdisp_00_11 as i16),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap())
            .wrapping_add(4i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_target.is_negative(),
            calc_target.abs() as u64,
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
        let mut calc_target: i128 = 0;
        let mut block_0_len = 2;
        calc_target = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_3(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap())
            .wrapping_add(4i128);
        let sdisp_00_11 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sdisp_00_11 }))
    }
}
#[derive(Clone, Debug)]
enum Tabletarget00_11 {
    Var0(target00_11Var0),
}
impl Tabletarget00_11 {
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
                target00_11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:184:1, end:184:5))"]
#[derive(Clone, Debug)]
struct imm8Var0 {
    simm_00_07: u8,
}
impl imm8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(
                true,
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
                    .is_negative(),
                (if self.simm_00_07 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm_00_07 as i8)
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
        let mut block_0_len = 2;
        let simm_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimm8 {
    Var0(imm8Var0),
}
impl Tableimm8 {
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
                imm8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:191:1, end:191:8))"]
#[derive(Clone, Debug)]
struct disppc4Var0 {
    disp_00_07: u8,
}
impl disppc4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0)
            .wrapping_add(
                (i128::try_from(inst_start).unwrap().wrapping_add(4i128) & 4294967292i128),
            );
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::pc),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(2i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(
                (i128::try_from(inst_start).unwrap().wrapping_add(4i128) & 4294967292i128),
            );
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabledisppc4 {
    Var0(disppc4Var0),
}
impl Tabledisppc4 {
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
                disppc4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:195:1, end:195:8))"]
#[derive(Clone, Debug)]
struct disppc2Var0 {
    disp_00_07: u8,
}
impl disppc2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_disp: i128 = 0;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| i128::try_from(self.disp_00_07).unwrap().checked_shl(shl))
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap().wrapping_add(4i128));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::pc),
            <DisplayElement>::Literal(")"),
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
        let mut calc_disp: i128 = 0;
        let mut block_0_len = 2;
        calc_disp = u32::try_from(1i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_2(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(inst_start).unwrap().wrapping_add(4i128));
        let disp_00_07 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { disp_00_07 }))
    }
}
#[derive(Clone, Debug)]
enum Tabledisppc2 {
    Var0(disppc2Var0),
}
impl Tabledisppc2 {
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
                disppc2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:502:1, end:502:7))"]
#[derive(Clone, Debug)]
struct simm20Var0 {
    l_simm20_20_23: u8,
    l_imm20_00_15: u16,
}
impl simm20Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_value: i128 = 0;
        calc_value = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.l_simm20_20_23 & 8 != 0 {
                        -1 & !7
                    } else {
                        0
                    } | self.l_simm20_20_23 as i8),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.l_imm20_00_15).unwrap());
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("#"),
            <DisplayElement>::Number(true, calc_value.is_negative(), calc_value.abs() as u64),
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
        let mut calc_value: i128 = 0;
        let mut block_0_len = 4;
        calc_value = (u32::try_from(16i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_16(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_19(tokens_current)).unwrap());
        let l_simm20_20_23 = token_16(tokens_current);
        let l_imm20_00_15 = token_19(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_simm20_20_23,
                l_imm20_00_15,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablesimm20 {
    Var0(simm20Var0),
}
impl Tablesimm20 {
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
                simm20Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:506:1, end:506:8))"]
#[derive(Clone, Debug)]
struct simm20sVar0 {
    l_simm20_20_23: u8,
    l_imm20_00_15: u16,
}
impl simm20sVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_value: i128 = 0;
        calc_value = u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                (u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(
                            (if self.l_simm20_20_23 & 8 != 0 {
                                -1 & !7
                            } else {
                                0
                            } | self.l_simm20_20_23 as i8),
                        )
                        .unwrap()
                        .checked_shl(shl)
                    })
                    .unwrap_or(0)
                    | i128::try_from(self.l_imm20_00_15).unwrap())
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("#"),
            <DisplayElement>::Number(true, calc_value.is_negative(), calc_value.abs() as u64),
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
        let mut calc_value: i128 = 0;
        let mut block_0_len = 4;
        calc_value = u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                (u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_16(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0)
                    | i128::try_from(token_19(tokens_current)).unwrap())
                .checked_shl(shl)
            })
            .unwrap_or(0);
        let l_imm20_00_15 = token_19(tokens_current);
        let l_simm20_20_23 = token_16(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                l_simm20_20_23,
                l_imm20_00_15,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablesimm20s {
    Var0(simm20sVar0),
}
impl Tablesimm20s {
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
                simm20sVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:539:1, end:539:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_0Var0 {
    rm_imm_08_11: u8,
}
impl MovMLReg1_0Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_0 {
    Var0(MovMLReg1_0Var0),
}
impl TableMovMLReg1_0 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_0Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:540:1, end:540:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_0storeVar0 {}
impl MovMLReg1_0storeVar0 {
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
        let mut block_0_len = 2;
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_0store {
    Var0(MovMLReg1_0storeVar0),
}
impl TableMovMLReg1_0store {
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
                MovMLReg1_0storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:543:1, end:543:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_1Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_0store: TableMovMLReg1_0store,
}
impl MovMLReg1_1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_0store = if let Some((len, table)) =
            TableMovMLReg1_0store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_0store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:542:1, end:542:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_1Var1 {
    MovMLReg1_0: TableMovMLReg1_0,
}
impl MovMLReg1_1Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_0
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
        let MovMLReg1_0 = if let Some((len, table)) =
            TableMovMLReg1_0::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_1 {
    Var0(MovMLReg1_1Var0),
    Var1(MovMLReg1_1Var1),
}
impl TableMovMLReg1_1 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_1Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:544:1, end:544:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_1storeVar0 {
    MovMLReg1_0store: TableMovMLReg1_0store,
}
impl MovMLReg1_1storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_0store = if let Some((len, table)) =
            TableMovMLReg1_0store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_0store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_1store {
    Var0(MovMLReg1_1storeVar0),
}
impl TableMovMLReg1_1store {
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
                MovMLReg1_1storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:547:1, end:547:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_2Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_1store: TableMovMLReg1_1store,
}
impl MovMLReg1_2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_1store = if let Some((len, table)) =
            TableMovMLReg1_1store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_1store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:546:1, end:546:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_2Var1 {
    MovMLReg1_1: TableMovMLReg1_1,
}
impl MovMLReg1_2Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_1
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
        let MovMLReg1_1 = if let Some((len, table)) =
            TableMovMLReg1_1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_2 {
    Var0(MovMLReg1_2Var0),
    Var1(MovMLReg1_2Var1),
}
impl TableMovMLReg1_2 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:548:1, end:548:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_2storeVar0 {
    MovMLReg1_1store: TableMovMLReg1_1store,
}
impl MovMLReg1_2storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_1store = if let Some((len, table)) =
            TableMovMLReg1_1store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_1store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_2store {
    Var0(MovMLReg1_2storeVar0),
}
impl TableMovMLReg1_2store {
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
                MovMLReg1_2storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:551:1, end:551:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_3Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_2store: TableMovMLReg1_2store,
}
impl MovMLReg1_3Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_2store = if let Some((len, table)) =
            TableMovMLReg1_2store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_2store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:550:1, end:550:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_3Var1 {
    MovMLReg1_2: TableMovMLReg1_2,
}
impl MovMLReg1_3Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_2
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
        let MovMLReg1_2 = if let Some((len, table)) =
            TableMovMLReg1_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_3 {
    Var0(MovMLReg1_3Var0),
    Var1(MovMLReg1_3Var1),
}
impl TableMovMLReg1_3 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_3Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_3Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:552:1, end:552:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_3storeVar0 {
    MovMLReg1_2store: TableMovMLReg1_2store,
}
impl MovMLReg1_3storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_2store = if let Some((len, table)) =
            TableMovMLReg1_2store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_2store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_3store {
    Var0(MovMLReg1_3storeVar0),
}
impl TableMovMLReg1_3store {
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
                MovMLReg1_3storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:555:1, end:555:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_4Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_3store: TableMovMLReg1_3store,
}
impl MovMLReg1_4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_3store = if let Some((len, table)) =
            TableMovMLReg1_3store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_3store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:554:1, end:554:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_4Var1 {
    MovMLReg1_3: TableMovMLReg1_3,
}
impl MovMLReg1_4Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_3
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
        let MovMLReg1_3 = if let Some((len, table)) =
            TableMovMLReg1_3::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_4 {
    Var0(MovMLReg1_4Var0),
    Var1(MovMLReg1_4Var1),
}
impl TableMovMLReg1_4 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_4Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:556:1, end:556:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_4storeVar0 {
    MovMLReg1_3store: TableMovMLReg1_3store,
}
impl MovMLReg1_4storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_3store = if let Some((len, table)) =
            TableMovMLReg1_3store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_3store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_4store {
    Var0(MovMLReg1_4storeVar0),
}
impl TableMovMLReg1_4store {
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
                MovMLReg1_4storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:559:1, end:559:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_5Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_4store: TableMovMLReg1_4store,
}
impl MovMLReg1_5Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_4store = if let Some((len, table)) =
            TableMovMLReg1_4store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_4store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:558:1, end:558:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_5Var1 {
    MovMLReg1_4: TableMovMLReg1_4,
}
impl MovMLReg1_5Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_4
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
        let MovMLReg1_4 = if let Some((len, table)) =
            TableMovMLReg1_4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_4 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_5 {
    Var0(MovMLReg1_5Var0),
    Var1(MovMLReg1_5Var1),
}
impl TableMovMLReg1_5 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_5Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_5Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:560:1, end:560:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_5storeVar0 {
    MovMLReg1_4store: TableMovMLReg1_4store,
}
impl MovMLReg1_5storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_4store = if let Some((len, table)) =
            TableMovMLReg1_4store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_4store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_5store {
    Var0(MovMLReg1_5storeVar0),
}
impl TableMovMLReg1_5store {
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
                MovMLReg1_5storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:563:1, end:563:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_6Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_5store: TableMovMLReg1_5store,
}
impl MovMLReg1_6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_5store = if let Some((len, table)) =
            TableMovMLReg1_5store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_5store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:562:1, end:562:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_6Var1 {
    MovMLReg1_5: TableMovMLReg1_5,
}
impl MovMLReg1_6Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_5
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
        let MovMLReg1_5 = if let Some((len, table)) =
            TableMovMLReg1_5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_5 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_6 {
    Var0(MovMLReg1_6Var0),
    Var1(MovMLReg1_6Var1),
}
impl TableMovMLReg1_6 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_6Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:564:1, end:564:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_6storeVar0 {
    MovMLReg1_5store: TableMovMLReg1_5store,
}
impl MovMLReg1_6storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_5store = if let Some((len, table)) =
            TableMovMLReg1_5store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_5store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_6store {
    Var0(MovMLReg1_6storeVar0),
}
impl TableMovMLReg1_6store {
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
                MovMLReg1_6storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:567:1, end:567:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_7Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_6store: TableMovMLReg1_6store,
}
impl MovMLReg1_7Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_6store = if let Some((len, table)) =
            TableMovMLReg1_6store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_6store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:566:1, end:566:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_7Var1 {
    MovMLReg1_6: TableMovMLReg1_6,
}
impl MovMLReg1_7Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_6
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
        let MovMLReg1_6 = if let Some((len, table)) =
            TableMovMLReg1_6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_6 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_7 {
    Var0(MovMLReg1_7Var0),
    Var1(MovMLReg1_7Var1),
}
impl TableMovMLReg1_7 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_7Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_7Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:568:1, end:568:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_7storeVar0 {
    MovMLReg1_6store: TableMovMLReg1_6store,
}
impl MovMLReg1_7storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_6store = if let Some((len, table)) =
            TableMovMLReg1_6store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_6store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_7store {
    Var0(MovMLReg1_7storeVar0),
}
impl TableMovMLReg1_7store {
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
                MovMLReg1_7storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:571:1, end:571:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_8Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_7store: TableMovMLReg1_7store,
}
impl MovMLReg1_8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_7store = if let Some((len, table)) =
            TableMovMLReg1_7store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_7store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:570:1, end:570:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_8Var1 {
    MovMLReg1_7: TableMovMLReg1_7,
}
impl MovMLReg1_8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_7
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
        let MovMLReg1_7 = if let Some((len, table)) =
            TableMovMLReg1_7::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_8 {
    Var0(MovMLReg1_8Var0),
    Var1(MovMLReg1_8Var1),
}
impl TableMovMLReg1_8 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:572:1, end:572:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_8storeVar0 {
    MovMLReg1_7store: TableMovMLReg1_7store,
}
impl MovMLReg1_8storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_7store = if let Some((len, table)) =
            TableMovMLReg1_7store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_7store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_8store {
    Var0(MovMLReg1_8storeVar0),
}
impl TableMovMLReg1_8store {
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
                MovMLReg1_8storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:575:1, end:575:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_9Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_8store: TableMovMLReg1_8store,
}
impl MovMLReg1_9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_8store = if let Some((len, table)) =
            TableMovMLReg1_8store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_8store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:574:1, end:574:12))"]
#[derive(Clone, Debug)]
struct MovMLReg1_9Var1 {
    MovMLReg1_8: TableMovMLReg1_8,
}
impl MovMLReg1_9Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_8
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
        let MovMLReg1_8 = if let Some((len, table)) =
            TableMovMLReg1_8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_8 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_9 {
    Var0(MovMLReg1_9Var0),
    Var1(MovMLReg1_9Var1),
}
impl TableMovMLReg1_9 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_9Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:576:1, end:576:17))"]
#[derive(Clone, Debug)]
struct MovMLReg1_9storeVar0 {
    MovMLReg1_8store: TableMovMLReg1_8store,
}
impl MovMLReg1_9storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_8store = if let Some((len, table)) =
            TableMovMLReg1_8store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_8store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_9store {
    Var0(MovMLReg1_9storeVar0),
}
impl TableMovMLReg1_9store {
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
                MovMLReg1_9storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:579:1, end:579:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_10Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_9store: TableMovMLReg1_9store,
}
impl MovMLReg1_10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_9store = if let Some((len, table)) =
            TableMovMLReg1_9store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_9store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:578:1, end:578:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_10Var1 {
    MovMLReg1_9: TableMovMLReg1_9,
}
impl MovMLReg1_10Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_9
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
        let MovMLReg1_9 = if let Some((len, table)) =
            TableMovMLReg1_9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_9 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_10 {
    Var0(MovMLReg1_10Var0),
    Var1(MovMLReg1_10Var1),
}
impl TableMovMLReg1_10 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_10Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:580:1, end:580:18))"]
#[derive(Clone, Debug)]
struct MovMLReg1_10storeVar0 {
    MovMLReg1_9store: TableMovMLReg1_9store,
}
impl MovMLReg1_10storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_9store = if let Some((len, table)) =
            TableMovMLReg1_9store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_9store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_10store {
    Var0(MovMLReg1_10storeVar0),
}
impl TableMovMLReg1_10store {
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
                MovMLReg1_10storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:583:1, end:583:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_11Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_10store: TableMovMLReg1_10store,
}
impl MovMLReg1_11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_10store = if let Some((len, table)) =
            TableMovMLReg1_10store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_10store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:582:1, end:582:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_11Var1 {
    MovMLReg1_10: TableMovMLReg1_10,
}
impl MovMLReg1_11Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_10
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
        let MovMLReg1_10 = if let Some((len, table)) =
            TableMovMLReg1_10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_10 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_11 {
    Var0(MovMLReg1_11Var0),
    Var1(MovMLReg1_11Var1),
}
impl TableMovMLReg1_11 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_11Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:584:1, end:584:18))"]
#[derive(Clone, Debug)]
struct MovMLReg1_11storeVar0 {
    MovMLReg1_10store: TableMovMLReg1_10store,
}
impl MovMLReg1_11storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_10store = if let Some((len, table)) =
            TableMovMLReg1_10store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_10store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_11store {
    Var0(MovMLReg1_11storeVar0),
}
impl TableMovMLReg1_11store {
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
                MovMLReg1_11storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:587:1, end:587:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_12Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_11store: TableMovMLReg1_11store,
}
impl MovMLReg1_12Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_11store = if let Some((len, table)) =
            TableMovMLReg1_11store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_11store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:586:1, end:586:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_12Var1 {
    MovMLReg1_11: TableMovMLReg1_11,
}
impl MovMLReg1_12Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_11
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
        let MovMLReg1_11 = if let Some((len, table)) =
            TableMovMLReg1_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_12 {
    Var0(MovMLReg1_12Var0),
    Var1(MovMLReg1_12Var1),
}
impl TableMovMLReg1_12 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_12Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_12Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:588:1, end:588:18))"]
#[derive(Clone, Debug)]
struct MovMLReg1_12storeVar0 {
    MovMLReg1_11store: TableMovMLReg1_11store,
}
impl MovMLReg1_12storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_11store = if let Some((len, table)) =
            TableMovMLReg1_11store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_11store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_12store {
    Var0(MovMLReg1_12storeVar0),
}
impl TableMovMLReg1_12store {
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
                MovMLReg1_12storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:591:1, end:591:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_13Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_12store: TableMovMLReg1_12store,
}
impl MovMLReg1_13Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_12store = if let Some((len, table)) =
            TableMovMLReg1_12store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_12store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:590:1, end:590:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_13Var1 {
    MovMLReg1_12: TableMovMLReg1_12,
}
impl MovMLReg1_13Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_12
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
        let MovMLReg1_12 = if let Some((len, table)) =
            TableMovMLReg1_12::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_12 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_13 {
    Var0(MovMLReg1_13Var0),
    Var1(MovMLReg1_13Var1),
}
impl TableMovMLReg1_13 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_13Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_13Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:592:1, end:592:18))"]
#[derive(Clone, Debug)]
struct MovMLReg1_13storeVar0 {
    MovMLReg1_12store: TableMovMLReg1_12store,
}
impl MovMLReg1_13storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_12store = if let Some((len, table)) =
            TableMovMLReg1_12store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_12store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_13store {
    Var0(MovMLReg1_13storeVar0),
}
impl TableMovMLReg1_13store {
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
                MovMLReg1_13storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:595:1, end:595:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_14Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_13store: TableMovMLReg1_13store,
}
impl MovMLReg1_14Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_13store = if let Some((len, table)) =
            TableMovMLReg1_13store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_13store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:594:1, end:594:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_14Var1 {
    MovMLReg1_13: TableMovMLReg1_13,
}
impl MovMLReg1_14Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_13
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
        let MovMLReg1_13 = if let Some((len, table)) =
            TableMovMLReg1_13::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_13 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_14 {
    Var0(MovMLReg1_14Var0),
    Var1(MovMLReg1_14Var1),
}
impl TableMovMLReg1_14 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_14Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_14Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:596:1, end:596:18))"]
#[derive(Clone, Debug)]
struct MovMLReg1_14storeVar0 {
    MovMLReg1_13store: TableMovMLReg1_13store,
}
impl MovMLReg1_14storeVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg1_13store = if let Some((len, table)) =
            TableMovMLReg1_13store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_13store }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_14store {
    Var0(MovMLReg1_14storeVar0),
}
impl TableMovMLReg1_14store {
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
                MovMLReg1_14storeVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:599:1, end:599:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_15Var0 {
    rm_imm_08_11: u8,
    MovMLReg1_14store: TableMovMLReg1_14store,
}
impl MovMLReg1_15Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg1_14store = if let Some((len, table)) =
            TableMovMLReg1_14store::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg1_14store,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:598:1, end:598:13))"]
#[derive(Clone, Debug)]
struct MovMLReg1_15Var1 {
    MovMLReg1_14: TableMovMLReg1_14,
}
impl MovMLReg1_15Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_14
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
        let MovMLReg1_14 = if let Some((len, table)) =
            TableMovMLReg1_14::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_14 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1_15 {
    Var0(MovMLReg1_15Var0),
    Var1(MovMLReg1_15Var1),
}
impl TableMovMLReg1_15 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_15Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg1_15Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:601:1, end:601:10))"]
#[derive(Clone, Debug)]
struct MovMLReg1Var0 {
    MovMLReg1_15: TableMovMLReg1_15,
}
impl MovMLReg1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg1_15
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
        let MovMLReg1_15 = if let Some((len, table)) =
            TableMovMLReg1_15::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg1_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg1 {
    Var0(MovMLReg1Var0),
}
impl TableMovMLReg1 {
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
                MovMLReg1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:613:1, end:613:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_0Var0 {
    rm_imm_08_11: u8,
}
impl MovMLReg2_0Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_0 {
    Var0(MovMLReg2_0Var0),
}
impl TableMovMLReg2_0 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_0Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:615:1, end:615:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_0loadVar0 {}
impl MovMLReg2_0loadVar0 {
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
        let mut block_0_len = 2;
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_0load {
    Var0(MovMLReg2_0loadVar0),
}
impl TableMovMLReg2_0load {
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
                MovMLReg2_0loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:618:1, end:618:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_1Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_0load: TableMovMLReg2_0load,
}
impl MovMLReg2_1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_0load = if let Some((len, table)) =
            TableMovMLReg2_0load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_0load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:617:1, end:617:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_1Var1 {
    MovMLReg2_0: TableMovMLReg2_0,
}
impl MovMLReg2_1Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_0
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
        let MovMLReg2_0 = if let Some((len, table)) =
            TableMovMLReg2_0::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_1 {
    Var0(MovMLReg2_1Var0),
    Var1(MovMLReg2_1Var1),
}
impl TableMovMLReg2_1 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_1Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:619:1, end:619:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_1loadVar0 {
    MovMLReg2_0load: TableMovMLReg2_0load,
}
impl MovMLReg2_1loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_0load = if let Some((len, table)) =
            TableMovMLReg2_0load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_0load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_1load {
    Var0(MovMLReg2_1loadVar0),
}
impl TableMovMLReg2_1load {
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
                MovMLReg2_1loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:622:1, end:622:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_2Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_1load: TableMovMLReg2_1load,
}
impl MovMLReg2_2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_1load = if let Some((len, table)) =
            TableMovMLReg2_1load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_1load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:621:1, end:621:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_2Var1 {
    MovMLReg2_1: TableMovMLReg2_1,
}
impl MovMLReg2_2Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_1
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
        let MovMLReg2_1 = if let Some((len, table)) =
            TableMovMLReg2_1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_2 {
    Var0(MovMLReg2_2Var0),
    Var1(MovMLReg2_2Var1),
}
impl TableMovMLReg2_2 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:623:1, end:623:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_2loadVar0 {
    MovMLReg2_1load: TableMovMLReg2_1load,
}
impl MovMLReg2_2loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_1load = if let Some((len, table)) =
            TableMovMLReg2_1load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_1load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_2load {
    Var0(MovMLReg2_2loadVar0),
}
impl TableMovMLReg2_2load {
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
                MovMLReg2_2loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:626:1, end:626:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_3Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_2load: TableMovMLReg2_2load,
}
impl MovMLReg2_3Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_2load = if let Some((len, table)) =
            TableMovMLReg2_2load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_2load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:625:1, end:625:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_3Var1 {
    MovMLReg2_2: TableMovMLReg2_2,
}
impl MovMLReg2_3Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_2
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
        let MovMLReg2_2 = if let Some((len, table)) =
            TableMovMLReg2_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_3 {
    Var0(MovMLReg2_3Var0),
    Var1(MovMLReg2_3Var1),
}
impl TableMovMLReg2_3 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_3Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_3Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:627:1, end:627:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_3loadVar0 {
    MovMLReg2_2load: TableMovMLReg2_2load,
}
impl MovMLReg2_3loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_2load = if let Some((len, table)) =
            TableMovMLReg2_2load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_2load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_3load {
    Var0(MovMLReg2_3loadVar0),
}
impl TableMovMLReg2_3load {
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
                MovMLReg2_3loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:630:1, end:630:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_4Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_3load: TableMovMLReg2_3load,
}
impl MovMLReg2_4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_3load = if let Some((len, table)) =
            TableMovMLReg2_3load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_3load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:629:1, end:629:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_4Var1 {
    MovMLReg2_3: TableMovMLReg2_3,
}
impl MovMLReg2_4Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_3
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
        let MovMLReg2_3 = if let Some((len, table)) =
            TableMovMLReg2_3::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_4 {
    Var0(MovMLReg2_4Var0),
    Var1(MovMLReg2_4Var1),
}
impl TableMovMLReg2_4 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_4Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:632:1, end:632:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_4loadVar0 {
    MovMLReg2_3load: TableMovMLReg2_3load,
}
impl MovMLReg2_4loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_3load = if let Some((len, table)) =
            TableMovMLReg2_3load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_3load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_4load {
    Var0(MovMLReg2_4loadVar0),
}
impl TableMovMLReg2_4load {
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
                MovMLReg2_4loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:635:1, end:635:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_5Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_4load: TableMovMLReg2_4load,
}
impl MovMLReg2_5Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_4load = if let Some((len, table)) =
            TableMovMLReg2_4load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_4load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:634:1, end:634:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_5Var1 {
    MovMLReg2_4: TableMovMLReg2_4,
}
impl MovMLReg2_5Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_4
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
        let MovMLReg2_4 = if let Some((len, table)) =
            TableMovMLReg2_4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_4 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_5 {
    Var0(MovMLReg2_5Var0),
    Var1(MovMLReg2_5Var1),
}
impl TableMovMLReg2_5 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_5Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_5Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:637:1, end:637:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_5loadVar0 {
    MovMLReg2_4load: TableMovMLReg2_4load,
}
impl MovMLReg2_5loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_4load = if let Some((len, table)) =
            TableMovMLReg2_4load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_4load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_5load {
    Var0(MovMLReg2_5loadVar0),
}
impl TableMovMLReg2_5load {
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
                MovMLReg2_5loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:640:1, end:640:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_6Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_5load: TableMovMLReg2_5load,
}
impl MovMLReg2_6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_5load = if let Some((len, table)) =
            TableMovMLReg2_5load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_5load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:639:1, end:639:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_6Var1 {
    MovMLReg2_5: TableMovMLReg2_5,
}
impl MovMLReg2_6Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_5
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
        let MovMLReg2_5 = if let Some((len, table)) =
            TableMovMLReg2_5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_5 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_6 {
    Var0(MovMLReg2_6Var0),
    Var1(MovMLReg2_6Var1),
}
impl TableMovMLReg2_6 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_6Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:642:1, end:642:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_6loadVar0 {
    MovMLReg2_5load: TableMovMLReg2_5load,
}
impl MovMLReg2_6loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_5load = if let Some((len, table)) =
            TableMovMLReg2_5load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_5load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_6load {
    Var0(MovMLReg2_6loadVar0),
}
impl TableMovMLReg2_6load {
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
                MovMLReg2_6loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:645:1, end:645:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_7Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_6load: TableMovMLReg2_6load,
}
impl MovMLReg2_7Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_6load = if let Some((len, table)) =
            TableMovMLReg2_6load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_6load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:644:1, end:644:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_7Var1 {
    MovMLReg2_6: TableMovMLReg2_6,
}
impl MovMLReg2_7Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_6
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
        let MovMLReg2_6 = if let Some((len, table)) =
            TableMovMLReg2_6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_6 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_7 {
    Var0(MovMLReg2_7Var0),
    Var1(MovMLReg2_7Var1),
}
impl TableMovMLReg2_7 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_7Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_7Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:647:1, end:647:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_7loadVar0 {
    MovMLReg2_6load: TableMovMLReg2_6load,
}
impl MovMLReg2_7loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_6load = if let Some((len, table)) =
            TableMovMLReg2_6load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_6load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_7load {
    Var0(MovMLReg2_7loadVar0),
}
impl TableMovMLReg2_7load {
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
                MovMLReg2_7loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:650:1, end:650:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_8Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_7load: TableMovMLReg2_7load,
}
impl MovMLReg2_8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_7load = if let Some((len, table)) =
            TableMovMLReg2_7load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_7load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:649:1, end:649:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_8Var1 {
    MovMLReg2_7: TableMovMLReg2_7,
}
impl MovMLReg2_8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_7
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
        let MovMLReg2_7 = if let Some((len, table)) =
            TableMovMLReg2_7::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_8 {
    Var0(MovMLReg2_8Var0),
    Var1(MovMLReg2_8Var1),
}
impl TableMovMLReg2_8 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:652:1, end:652:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_8loadVar0 {
    MovMLReg2_7load: TableMovMLReg2_7load,
}
impl MovMLReg2_8loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_7load = if let Some((len, table)) =
            TableMovMLReg2_7load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_7load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_8load {
    Var0(MovMLReg2_8loadVar0),
}
impl TableMovMLReg2_8load {
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
                MovMLReg2_8loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:655:1, end:655:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_9Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_8load: TableMovMLReg2_8load,
}
impl MovMLReg2_9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_8load = if let Some((len, table)) =
            TableMovMLReg2_8load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_8load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:654:1, end:654:12))"]
#[derive(Clone, Debug)]
struct MovMLReg2_9Var1 {
    MovMLReg2_8: TableMovMLReg2_8,
}
impl MovMLReg2_9Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_8
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
        let MovMLReg2_8 = if let Some((len, table)) =
            TableMovMLReg2_8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_8 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_9 {
    Var0(MovMLReg2_9Var0),
    Var1(MovMLReg2_9Var1),
}
impl TableMovMLReg2_9 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_9Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:657:1, end:657:16))"]
#[derive(Clone, Debug)]
struct MovMLReg2_9loadVar0 {
    MovMLReg2_8load: TableMovMLReg2_8load,
}
impl MovMLReg2_9loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_8load = if let Some((len, table)) =
            TableMovMLReg2_8load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_8load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_9load {
    Var0(MovMLReg2_9loadVar0),
}
impl TableMovMLReg2_9load {
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
                MovMLReg2_9loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:660:1, end:660:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_10Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_9load: TableMovMLReg2_9load,
}
impl MovMLReg2_10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_9load = if let Some((len, table)) =
            TableMovMLReg2_9load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_9load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:659:1, end:659:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_10Var1 {
    MovMLReg2_9: TableMovMLReg2_9,
}
impl MovMLReg2_10Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_9
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
        let MovMLReg2_9 = if let Some((len, table)) =
            TableMovMLReg2_9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_9 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_10 {
    Var0(MovMLReg2_10Var0),
    Var1(MovMLReg2_10Var1),
}
impl TableMovMLReg2_10 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_10Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:662:1, end:662:17))"]
#[derive(Clone, Debug)]
struct MovMLReg2_10loadVar0 {
    MovMLReg2_9load: TableMovMLReg2_9load,
}
impl MovMLReg2_10loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_9load = if let Some((len, table)) =
            TableMovMLReg2_9load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_9load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_10load {
    Var0(MovMLReg2_10loadVar0),
}
impl TableMovMLReg2_10load {
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
                MovMLReg2_10loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:665:1, end:665:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_11Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_10load: TableMovMLReg2_10load,
}
impl MovMLReg2_11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_10load = if let Some((len, table)) =
            TableMovMLReg2_10load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_10load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:664:1, end:664:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_11Var1 {
    MovMLReg2_10: TableMovMLReg2_10,
}
impl MovMLReg2_11Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_10
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
        let MovMLReg2_10 = if let Some((len, table)) =
            TableMovMLReg2_10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_10 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_11 {
    Var0(MovMLReg2_11Var0),
    Var1(MovMLReg2_11Var1),
}
impl TableMovMLReg2_11 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_11Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:666:1, end:666:17))"]
#[derive(Clone, Debug)]
struct MovMLReg2_11loadVar0 {
    MovMLReg2_10load: TableMovMLReg2_10load,
}
impl MovMLReg2_11loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_10load = if let Some((len, table)) =
            TableMovMLReg2_10load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_10load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_11load {
    Var0(MovMLReg2_11loadVar0),
}
impl TableMovMLReg2_11load {
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
                MovMLReg2_11loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:669:1, end:669:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_12Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_11load: TableMovMLReg2_11load,
}
impl MovMLReg2_12Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_11load = if let Some((len, table)) =
            TableMovMLReg2_11load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_11load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:668:1, end:668:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_12Var1 {
    MovMLReg2_11: TableMovMLReg2_11,
}
impl MovMLReg2_12Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_11
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
        let MovMLReg2_11 = if let Some((len, table)) =
            TableMovMLReg2_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_12 {
    Var0(MovMLReg2_12Var0),
    Var1(MovMLReg2_12Var1),
}
impl TableMovMLReg2_12 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_12Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_12Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:671:1, end:671:17))"]
#[derive(Clone, Debug)]
struct MovMLReg2_12loadVar0 {
    MovMLReg2_11load: TableMovMLReg2_11load,
}
impl MovMLReg2_12loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_11load = if let Some((len, table)) =
            TableMovMLReg2_11load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_11load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_12load {
    Var0(MovMLReg2_12loadVar0),
}
impl TableMovMLReg2_12load {
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
                MovMLReg2_12loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:674:1, end:674:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_13Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_12load: TableMovMLReg2_12load,
}
impl MovMLReg2_13Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_12load = if let Some((len, table)) =
            TableMovMLReg2_12load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_12load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:673:1, end:673:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_13Var1 {
    MovMLReg2_12: TableMovMLReg2_12,
}
impl MovMLReg2_13Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_12
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
        let MovMLReg2_12 = if let Some((len, table)) =
            TableMovMLReg2_12::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_12 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_13 {
    Var0(MovMLReg2_13Var0),
    Var1(MovMLReg2_13Var1),
}
impl TableMovMLReg2_13 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_13Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_13Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:676:1, end:676:17))"]
#[derive(Clone, Debug)]
struct MovMLReg2_13loadVar0 {
    MovMLReg2_12load: TableMovMLReg2_12load,
}
impl MovMLReg2_13loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_12load = if let Some((len, table)) =
            TableMovMLReg2_12load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_12load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_13load {
    Var0(MovMLReg2_13loadVar0),
}
impl TableMovMLReg2_13load {
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
                MovMLReg2_13loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:679:1, end:679:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_14Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_13load: TableMovMLReg2_13load,
}
impl MovMLReg2_14Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_13load = if let Some((len, table)) =
            TableMovMLReg2_13load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_13load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:678:1, end:678:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_14Var1 {
    MovMLReg2_13: TableMovMLReg2_13,
}
impl MovMLReg2_14Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_13
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
        let MovMLReg2_13 = if let Some((len, table)) =
            TableMovMLReg2_13::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_13 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_14 {
    Var0(MovMLReg2_14Var0),
    Var1(MovMLReg2_14Var1),
}
impl TableMovMLReg2_14 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_14Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_14Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:681:1, end:681:17))"]
#[derive(Clone, Debug)]
struct MovMLReg2_14loadVar0 {
    MovMLReg2_13load: TableMovMLReg2_13load,
}
impl MovMLReg2_14loadVar0 {
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
        let mut block_0_len = 2;
        let MovMLReg2_13load = if let Some((len, table)) =
            TableMovMLReg2_13load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_13load }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_14load {
    Var0(MovMLReg2_14loadVar0),
}
impl TableMovMLReg2_14load {
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
                MovMLReg2_14loadVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:684:1, end:684:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_15Var0 {
    rm_imm_08_11: u8,
    MovMLReg2_14load: TableMovMLReg2_14load,
}
impl MovMLReg2_15Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let MovMLReg2_14load = if let Some((len, table)) =
            TableMovMLReg2_14load::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                MovMLReg2_14load,
                rm_imm_08_11,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:683:1, end:683:13))"]
#[derive(Clone, Debug)]
struct MovMLReg2_15Var1 {
    MovMLReg2_14: TableMovMLReg2_14,
}
impl MovMLReg2_15Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_14
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
        let MovMLReg2_14 = if let Some((len, table)) =
            TableMovMLReg2_14::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_14 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2_15 {
    Var0(MovMLReg2_15Var0),
    Var1(MovMLReg2_15Var1),
}
impl TableMovMLReg2_15 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_15Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMLReg2_15Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:686:1, end:686:10))"]
#[derive(Clone, Debug)]
struct MovMLReg2Var0 {
    MovMLReg2_15: TableMovMLReg2_15,
}
impl MovMLReg2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMLReg2_15
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
        let MovMLReg2_15 = if let Some((len, table)) =
            TableMovMLReg2_15::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMLReg2_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMLReg2 {
    Var0(MovMLReg2Var0),
}
impl TableMovMLReg2 {
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
                MovMLReg2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:697:1, end:697:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_0Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_0Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_0 {
    Var0(MovMUReg1_0Var0),
}
impl TableMovMUReg1_0 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_0Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:700:1, end:700:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_1Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:699:1, end:699:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_1Var1 {
    MovMUReg1_0: TableMovMUReg1_0,
}
impl MovMUReg1_1Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_0
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
        let MovMUReg1_0 = if let Some((len, table)) =
            TableMovMUReg1_0::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_1 {
    Var0(MovMUReg1_1Var0),
    Var1(MovMUReg1_1Var1),
}
impl TableMovMUReg1_1 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_1Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:703:1, end:703:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_2Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:702:1, end:702:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_2Var1 {
    MovMUReg1_1: TableMovMUReg1_1,
}
impl MovMUReg1_2Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_1
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
        let MovMUReg1_1 = if let Some((len, table)) =
            TableMovMUReg1_1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_2 {
    Var0(MovMUReg1_2Var0),
    Var1(MovMUReg1_2Var1),
}
impl TableMovMUReg1_2 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:706:1, end:706:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_3Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_3Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:705:1, end:705:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_3Var1 {
    MovMUReg1_2: TableMovMUReg1_2,
}
impl MovMUReg1_3Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_2
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
        let MovMUReg1_2 = if let Some((len, table)) =
            TableMovMUReg1_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_3 {
    Var0(MovMUReg1_3Var0),
    Var1(MovMUReg1_3Var1),
}
impl TableMovMUReg1_3 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_3Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_3Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:709:1, end:709:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_4Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:708:1, end:708:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_4Var1 {
    MovMUReg1_3: TableMovMUReg1_3,
}
impl MovMUReg1_4Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_3
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
        let MovMUReg1_3 = if let Some((len, table)) =
            TableMovMUReg1_3::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_4 {
    Var0(MovMUReg1_4Var0),
    Var1(MovMUReg1_4Var1),
}
impl TableMovMUReg1_4 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_4Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:712:1, end:712:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_5Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_5Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:711:1, end:711:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_5Var1 {
    MovMUReg1_4: TableMovMUReg1_4,
}
impl MovMUReg1_5Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_4
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
        let MovMUReg1_4 = if let Some((len, table)) =
            TableMovMUReg1_4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_4 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_5 {
    Var0(MovMUReg1_5Var0),
    Var1(MovMUReg1_5Var1),
}
impl TableMovMUReg1_5 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_5Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_5Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:715:1, end:715:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_6Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:714:1, end:714:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_6Var1 {
    MovMUReg1_5: TableMovMUReg1_5,
}
impl MovMUReg1_6Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_5
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
        let MovMUReg1_5 = if let Some((len, table)) =
            TableMovMUReg1_5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_5 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_6 {
    Var0(MovMUReg1_6Var0),
    Var1(MovMUReg1_6Var1),
}
impl TableMovMUReg1_6 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_6Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:718:1, end:718:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_7Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_7Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:717:1, end:717:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_7Var1 {
    MovMUReg1_6: TableMovMUReg1_6,
}
impl MovMUReg1_7Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_6
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
        let MovMUReg1_6 = if let Some((len, table)) =
            TableMovMUReg1_6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_6 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_7 {
    Var0(MovMUReg1_7Var0),
    Var1(MovMUReg1_7Var1),
}
impl TableMovMUReg1_7 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_7Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_7Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:721:1, end:721:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_8Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:720:1, end:720:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_8Var1 {
    MovMUReg1_7: TableMovMUReg1_7,
}
impl MovMUReg1_8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_7
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
        let MovMUReg1_7 = if let Some((len, table)) =
            TableMovMUReg1_7::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_8 {
    Var0(MovMUReg1_8Var0),
    Var1(MovMUReg1_8Var1),
}
impl TableMovMUReg1_8 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:724:1, end:724:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_9Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:723:1, end:723:12))"]
#[derive(Clone, Debug)]
struct MovMUReg1_9Var1 {
    MovMUReg1_8: TableMovMUReg1_8,
}
impl MovMUReg1_9Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_8
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
        let MovMUReg1_8 = if let Some((len, table)) =
            TableMovMUReg1_8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_8 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_9 {
    Var0(MovMUReg1_9Var0),
    Var1(MovMUReg1_9Var1),
}
impl TableMovMUReg1_9 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_9Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:727:1, end:727:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_10Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:726:1, end:726:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_10Var1 {
    MovMUReg1_9: TableMovMUReg1_9,
}
impl MovMUReg1_10Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_9
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
        let MovMUReg1_9 = if let Some((len, table)) =
            TableMovMUReg1_9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_9 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_10 {
    Var0(MovMUReg1_10Var0),
    Var1(MovMUReg1_10Var1),
}
impl TableMovMUReg1_10 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_10Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:730:1, end:730:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_11Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:729:1, end:729:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_11Var1 {
    MovMUReg1_10: TableMovMUReg1_10,
}
impl MovMUReg1_11Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_10
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
        let MovMUReg1_10 = if let Some((len, table)) =
            TableMovMUReg1_10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_10 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_11 {
    Var0(MovMUReg1_11Var0),
    Var1(MovMUReg1_11Var1),
}
impl TableMovMUReg1_11 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_11Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:733:1, end:733:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_12Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_12Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:732:1, end:732:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_12Var1 {
    MovMUReg1_11: TableMovMUReg1_11,
}
impl MovMUReg1_12Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_11
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
        let MovMUReg1_11 = if let Some((len, table)) =
            TableMovMUReg1_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_12 {
    Var0(MovMUReg1_12Var0),
    Var1(MovMUReg1_12Var1),
}
impl TableMovMUReg1_12 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_12Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_12Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:736:1, end:736:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_13Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_13Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:735:1, end:735:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_13Var1 {
    MovMUReg1_12: TableMovMUReg1_12,
}
impl MovMUReg1_13Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_12
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
        let MovMUReg1_12 = if let Some((len, table)) =
            TableMovMUReg1_12::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_12 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_13 {
    Var0(MovMUReg1_13Var0),
    Var1(MovMUReg1_13Var1),
}
impl TableMovMUReg1_13 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_13Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_13Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:739:1, end:739:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_14Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_14Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:738:1, end:738:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_14Var1 {
    MovMUReg1_13: TableMovMUReg1_13,
}
impl MovMUReg1_14Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_13
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
        let MovMUReg1_13 = if let Some((len, table)) =
            TableMovMUReg1_13::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_13 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_14 {
    Var0(MovMUReg1_14Var0),
    Var1(MovMUReg1_14Var1),
}
impl TableMovMUReg1_14 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_14Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_14Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:742:1, end:742:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_15Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg1_15Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:741:1, end:741:13))"]
#[derive(Clone, Debug)]
struct MovMUReg1_15Var1 {
    MovMUReg1_14: TableMovMUReg1_14,
}
impl MovMUReg1_15Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_14
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
        let MovMUReg1_14 = if let Some((len, table)) =
            TableMovMUReg1_14::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_14 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1_15 {
    Var0(MovMUReg1_15Var0),
    Var1(MovMUReg1_15Var1),
}
impl TableMovMUReg1_15 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_15Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg1_15Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:744:1, end:744:10))"]
#[derive(Clone, Debug)]
struct MovMUReg1Var0 {
    MovMUReg1_15: TableMovMUReg1_15,
}
impl MovMUReg1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg1_15
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
        let MovMUReg1_15 = if let Some((len, table)) =
            TableMovMUReg1_15::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg1_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg1 {
    Var0(MovMUReg1Var0),
}
impl TableMovMUReg1 {
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
                MovMUReg1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:755:1, end:755:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_0Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_0Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_0 {
    Var0(MovMUReg2_0Var0),
}
impl TableMovMUReg2_0 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_0Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:758:1, end:758:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_1Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_1Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:757:1, end:757:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_1Var1 {
    MovMUReg2_0: TableMovMUReg2_0,
}
impl MovMUReg2_1Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_0
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
        let MovMUReg2_0 = if let Some((len, table)) =
            TableMovMUReg2_0::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_0 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_1 {
    Var0(MovMUReg2_1Var0),
    Var1(MovMUReg2_1Var1),
}
impl TableMovMUReg2_1 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_1Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:761:1, end:761:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_2Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:760:1, end:760:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_2Var1 {
    MovMUReg2_1: TableMovMUReg2_1,
}
impl MovMUReg2_2Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_1
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
        let MovMUReg2_1 = if let Some((len, table)) =
            TableMovMUReg2_1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_1 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_2 {
    Var0(MovMUReg2_2Var0),
    Var1(MovMUReg2_2Var1),
}
impl TableMovMUReg2_2 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:764:1, end:764:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_3Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_3Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:763:1, end:763:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_3Var1 {
    MovMUReg2_2: TableMovMUReg2_2,
}
impl MovMUReg2_3Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_2
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
        let MovMUReg2_2 = if let Some((len, table)) =
            TableMovMUReg2_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_2 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_3 {
    Var0(MovMUReg2_3Var0),
    Var1(MovMUReg2_3Var1),
}
impl TableMovMUReg2_3 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_3Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_3Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:767:1, end:767:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_4Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_4Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:766:1, end:766:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_4Var1 {
    MovMUReg2_3: TableMovMUReg2_3,
}
impl MovMUReg2_4Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_3
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
        let MovMUReg2_3 = if let Some((len, table)) =
            TableMovMUReg2_3::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_4 {
    Var0(MovMUReg2_4Var0),
    Var1(MovMUReg2_4Var1),
}
impl TableMovMUReg2_4 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_4Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:770:1, end:770:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_5Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_5Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:769:1, end:769:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_5Var1 {
    MovMUReg2_4: TableMovMUReg2_4,
}
impl MovMUReg2_5Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_4
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
        let MovMUReg2_4 = if let Some((len, table)) =
            TableMovMUReg2_4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_4 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_5 {
    Var0(MovMUReg2_5Var0),
    Var1(MovMUReg2_5Var1),
}
impl TableMovMUReg2_5 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_5Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_5Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:773:1, end:773:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_6Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_6Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:772:1, end:772:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_6Var1 {
    MovMUReg2_5: TableMovMUReg2_5,
}
impl MovMUReg2_6Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_5
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
        let MovMUReg2_5 = if let Some((len, table)) =
            TableMovMUReg2_5::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_5 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_6 {
    Var0(MovMUReg2_6Var0),
    Var1(MovMUReg2_6Var1),
}
impl TableMovMUReg2_6 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_6Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_6Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:776:1, end:776:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_7Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_7Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:775:1, end:775:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_7Var1 {
    MovMUReg2_6: TableMovMUReg2_6,
}
impl MovMUReg2_7Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_6
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
        let MovMUReg2_6 = if let Some((len, table)) =
            TableMovMUReg2_6::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_6 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_7 {
    Var0(MovMUReg2_7Var0),
    Var1(MovMUReg2_7Var1),
}
impl TableMovMUReg2_7 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_7Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_7Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:779:1, end:779:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_8Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:778:1, end:778:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_8Var1 {
    MovMUReg2_7: TableMovMUReg2_7,
}
impl MovMUReg2_8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_7
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
        let MovMUReg2_7 = if let Some((len, table)) =
            TableMovMUReg2_7::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_7 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_8 {
    Var0(MovMUReg2_8Var0),
    Var1(MovMUReg2_8Var1),
}
impl TableMovMUReg2_8 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:782:1, end:782:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_9Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:781:1, end:781:12))"]
#[derive(Clone, Debug)]
struct MovMUReg2_9Var1 {
    MovMUReg2_8: TableMovMUReg2_8,
}
impl MovMUReg2_9Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_8
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
        let MovMUReg2_8 = if let Some((len, table)) =
            TableMovMUReg2_8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_8 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_9 {
    Var0(MovMUReg2_9Var0),
    Var1(MovMUReg2_9Var1),
}
impl TableMovMUReg2_9 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_9Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:785:1, end:785:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_10Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:784:1, end:784:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_10Var1 {
    MovMUReg2_9: TableMovMUReg2_9,
}
impl MovMUReg2_10Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_9
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
        let MovMUReg2_9 = if let Some((len, table)) =
            TableMovMUReg2_9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_9 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_10 {
    Var0(MovMUReg2_10Var0),
    Var1(MovMUReg2_10Var1),
}
impl TableMovMUReg2_10 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 10 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_10Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:788:1, end:788:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_11Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:787:1, end:787:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_11Var1 {
    MovMUReg2_10: TableMovMUReg2_10,
}
impl MovMUReg2_11Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_10
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
        let MovMUReg2_10 = if let Some((len, table)) =
            TableMovMUReg2_10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_10 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_11 {
    Var0(MovMUReg2_11Var0),
    Var1(MovMUReg2_11Var1),
}
impl TableMovMUReg2_11 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 11 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_11Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:791:1, end:791:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_12Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_12Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:790:1, end:790:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_12Var1 {
    MovMUReg2_11: TableMovMUReg2_11,
}
impl MovMUReg2_12Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_11
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
        let MovMUReg2_11 = if let Some((len, table)) =
            TableMovMUReg2_11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_11 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_12 {
    Var0(MovMUReg2_12Var0),
    Var1(MovMUReg2_12Var1),
}
impl TableMovMUReg2_12 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_12Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_12Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:794:1, end:794:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_13Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_13Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:793:1, end:793:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_13Var1 {
    MovMUReg2_12: TableMovMUReg2_12,
}
impl MovMUReg2_13Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_12
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
        let MovMUReg2_12 = if let Some((len, table)) =
            TableMovMUReg2_12::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_12 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_13 {
    Var0(MovMUReg2_13Var0),
    Var1(MovMUReg2_13Var1),
}
impl TableMovMUReg2_13 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_13Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_13Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:797:1, end:797:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_14Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_14Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:796:1, end:796:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_14Var1 {
    MovMUReg2_13: TableMovMUReg2_13,
}
impl MovMUReg2_14Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_13
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
        let MovMUReg2_13 = if let Some((len, table)) =
            TableMovMUReg2_13::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_13 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_14 {
    Var0(MovMUReg2_14Var0),
    Var1(MovMUReg2_14Var1),
}
impl TableMovMUReg2_14 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_14Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_14Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:800:1, end:800:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_15Var0 {
    rm_imm_08_11: u8,
}
impl MovMUReg2_15Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_5_display(self.rm_imm_08_11)];
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
        let rm_imm_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_imm_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:799:1, end:799:13))"]
#[derive(Clone, Debug)]
struct MovMUReg2_15Var1 {
    MovMUReg2_14: TableMovMUReg2_14,
}
impl MovMUReg2_15Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_14
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
        let MovMUReg2_14 = if let Some((len, table)) =
            TableMovMUReg2_14::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_14 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2_15 {
    Var0(MovMUReg2_15Var0),
    Var1(MovMUReg2_15Var1),
}
impl TableMovMUReg2_15 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_15Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                MovMUReg2_15Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:802:1, end:802:10))"]
#[derive(Clone, Debug)]
struct MovMUReg2Var0 {
    MovMUReg2_15: TableMovMUReg2_15,
}
impl MovMUReg2Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.MovMUReg2_15
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
        let MovMUReg2_15 = if let Some((len, table)) =
            TableMovMUReg2_15::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { MovMUReg2_15 }))
    }
}
#[derive(Clone, Debug)]
enum TableMovMUReg2 {
    Var0(MovMUReg2Var0),
}
impl TableMovMUReg2 {
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
                MovMUReg2Var0::parse(tokens_param, &mut context_current, inst_start)
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
) -> Option<(u32, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
