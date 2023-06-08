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
fn meaning_1_display<T>(num: T) -> DisplayElement
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
#[doc = "Create token_fields: disp_00_11 sdisp_00_11"]
fn token_3(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: disp_00_03 sdisp_00_03 opcode_00_03"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: opcode_00_15"]
fn token_5(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: imm3_00_02"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: opcode_04_07 rm_04_07 rn_04_07"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: opcode_08_15"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 255) as u8)
}
#[doc = "Create token_fields: opcode_03_03"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 1) as u8)
}
#[doc = "Create token_fields: disp_00_07 sdisp_00_07 imm_00_07 simm_00_07 opcode_00_07"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: opcode_12_15"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: opcode_08_11 rm_08_11 rn_08_11 rm_imm_08_11 rn_imm_08_11"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 15) as u8)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1011:1, end:1011:2))"]
#[derive(Clone, Debug)]
struct div0u_instructionVar0 {}
impl div0u_instructionVar0 {
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
struct rts_instructionVar1 {}
impl rts_instructionVar1 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1731:1, end:1731:2))"]
#[derive(Clone, Debug)]
struct clrmac_instructionVar2 {}
impl clrmac_instructionVar2 {
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
struct clrt_instructionVar3 {}
impl clrt_instructionVar3 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1860:1, end:1860:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar4 {}
impl nop_instructionVar4 {
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
struct rte_instructionVar5 {}
impl rte_instructionVar5 {
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
struct sett_instructionVar6 {}
impl sett_instructionVar6 {
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
struct sleep_instructionVar7 {}
impl sleep_instructionVar7 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar8 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar8 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_w_instructionVar9 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar9 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:269:1, end:269:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar10 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar10 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct movt_instructionVar11 {
    rn_08_11: u8,
}
impl movt_instructionVar11 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct cmp_instructionVar12 {
    rn_08_11: u8,
}
impl cmp_instructionVar12 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct cmp_instructionVar13 {
    rn_08_11: u8,
}
impl cmp_instructionVar13 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct tas_b_instructionVar14 {
    rn_08_11: u8,
}
impl tas_b_instructionVar14 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct rotcl_instructionVar15 {
    rn_08_11: u8,
}
impl rotcl_instructionVar15 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct rotcr_instructionVar16 {
    rn_08_11: u8,
}
impl rotcr_instructionVar16 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct rotl_instructionVar17 {
    rn_08_11: u8,
}
impl rotl_instructionVar17 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct rotr_instructionVar18 {
    rn_08_11: u8,
}
impl rotr_instructionVar18 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shal_instructionVar19 {
    rn_08_11: u8,
}
impl shal_instructionVar19 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shar_instructionVar20 {
    rn_08_11: u8,
}
impl shar_instructionVar20 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shll_instructionVar21 {
    rn_08_11: u8,
}
impl shll_instructionVar21 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shll2_instructionVar22 {
    rn_08_11: u8,
}
impl shll2_instructionVar22 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shll8_instructionVar23 {
    rn_08_11: u8,
}
impl shll8_instructionVar23 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shll16_instructionVar24 {
    rn_08_11: u8,
}
impl shll16_instructionVar24 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shlr_instructionVar25 {
    rn_08_11: u8,
}
impl shlr_instructionVar25 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shlr2_instructionVar26 {
    rn_08_11: u8,
}
impl shlr2_instructionVar26 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shlr8_instructionVar27 {
    rn_08_11: u8,
}
impl shlr8_instructionVar27 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct shlr16_instructionVar28 {
    rn_08_11: u8,
}
impl shlr16_instructionVar28 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct jmp_instructionVar29 {
    rm_08_11: u8,
}
impl jmp_instructionVar29 {
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
            meaning_0_display(self.rm_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct jsr_instructionVar30 {
    rm_08_11: u8,
}
impl jsr_instructionVar30 {
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
            meaning_0_display(self.rm_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1775:1, end:1775:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar31 {
    rm_08_11: u8,
}
impl ldc_instructionVar31 {
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
            meaning_0_display(self.rm_08_11),
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
struct ldc_l_instructionVar32 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar32 {
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
            meaning_0_display(self.rm_08_11),
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
struct ldc_instructionVar33 {
    rm_08_11: u8,
}
impl ldc_instructionVar33 {
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
            meaning_0_display(self.rm_08_11),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1799:1, end:1799:2))"]
#[derive(Clone, Debug)]
struct ldc_l_instructionVar34 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar34 {
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
            meaning_0_display(self.rm_08_11),
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
struct ldc_instructionVar35 {
    rm_08_11: u8,
}
impl ldc_instructionVar35 {
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
            meaning_0_display(self.rm_08_11),
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
struct ldc_l_instructionVar36 {
    rm_08_11: u8,
}
impl ldc_l_instructionVar36 {
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
            meaning_0_display(self.rm_08_11),
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
struct lds_instructionVar37 {
    rm_08_11: u8,
}
impl lds_instructionVar37 {
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
            meaning_0_display(self.rm_08_11),
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
struct lds_l_instructionVar38 {
    rm_08_11: u8,
}
impl lds_l_instructionVar38 {
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
            meaning_0_display(self.rm_08_11),
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
struct lds_instructionVar39 {
    rm_08_11: u8,
}
impl lds_instructionVar39 {
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
            meaning_0_display(self.rm_08_11),
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
struct lds_l_instructionVar40 {
    rm_08_11: u8,
}
impl lds_l_instructionVar40 {
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
            meaning_0_display(self.rm_08_11),
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
struct lds_instructionVar41 {
    rm_08_11: u8,
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
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rm_08_11),
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
struct lds_l_instructionVar42 {
    rm_08_11: u8,
}
impl lds_l_instructionVar42 {
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
            meaning_0_display(self.rm_08_11),
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
struct stc_instructionVar43 {
    rn_08_11: u8,
}
impl stc_instructionVar43 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct stc_l_instructionVar44 {
    rn_08_11: u8,
}
impl stc_l_instructionVar44 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct stc_instructionVar45 {
    rn_08_11: u8,
}
impl stc_instructionVar45 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct stc_l_instructionVar46 {
    rn_08_11: u8,
}
impl stc_l_instructionVar46 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct stc_instructionVar47 {
    rn_08_11: u8,
}
impl stc_instructionVar47 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct stc_l_instructionVar48 {
    rn_08_11: u8,
}
impl stc_l_instructionVar48 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_instructionVar49 {
    rn_08_11: u8,
}
impl sts_instructionVar49 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_l_instructionVar50 {
    rn_08_11: u8,
}
impl sts_l_instructionVar50 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_instructionVar51 {
    rn_08_11: u8,
}
impl sts_instructionVar51 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_l_instructionVar52 {
    rn_08_11: u8,
}
impl sts_l_instructionVar52 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_instructionVar53 {
    rn_08_11: u8,
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
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::pr),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sts_l_instructionVar54 {
    rn_08_11: u8,
}
impl sts_l_instructionVar54 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_instructionVar55 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_instructionVar55 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:199:1, end:199:2))"]
#[derive(Clone, Debug)]
struct mova_instructionVar56 {
    disppc4: Tabledisppc4,
}
impl mova_instructionVar56 {
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
struct mov_b_instructionVar57 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar57 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar58 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar58 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_l_instructionVar59 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar59 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar60 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar60 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar61 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar61 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_l_instructionVar62 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar62 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar63 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar63 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_w_instructionVar64 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar64 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
        let opcode_08_11 = token_8(tokens_current);
        let opcode_04_07 = token_7(tokens_current);
        let rm_04_07 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:274:1, end:274:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar65 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar65 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
        let opcode_08_11 = token_8(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rm_04_07, rn_08_11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:280:1, end:280:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar66 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar66 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct mov_w_instructionVar67 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar67 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:292:1, end:292:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar68 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar68 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@-"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct mov_b_instructionVar69 {
    disp_00_03: u8,
    rm_04_07: u8,
}
impl mov_b_instructionVar69 {
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
            meaning_0_display(self.rm_04_07),
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
        let disp_00_03 = token_1(tokens_current);
        let rm_04_07 = token_7(tokens_current);
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
struct mov_w_instructionVar70 {
    rm_04_07: u8,
    disp_00_03: u8,
}
impl mov_w_instructionVar70 {
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
            meaning_0_display(self.rm_04_07),
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
struct mov_b_instructionVar71 {
    disp_00_03: u8,
    rn_04_07: u8,
}
impl mov_b_instructionVar71 {
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
            meaning_0_display(self.rn_04_07),
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
struct mov_w_instructionVar72 {
    rn_04_07: u8,
    disp_00_03: u8,
}
impl mov_w_instructionVar72 {
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
            meaning_0_display(self.rn_04_07),
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
        let rn_04_07 = token_7(tokens_current);
        let disp_00_03 = token_1(tokens_current);
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
struct mov_b_instructionVar73 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar73 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_w_instructionVar74 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar74 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:338:1, end:338:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar75 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar75 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_b_instructionVar76 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_b_instructionVar76 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
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
struct mov_w_instructionVar77 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_w_instructionVar77 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
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
struct mov_l_instructionVar78 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mov_l_instructionVar78 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Register(Register::r0),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
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
struct mov_b_instructionVar79 {
    disp_00_07: u8,
}
impl mov_b_instructionVar79 {
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
struct mov_w_instructionVar80 {
    disp_00_07: u8,
}
impl mov_w_instructionVar80 {
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
struct mov_l_instructionVar81 {
    disp_00_07: u8,
}
impl mov_l_instructionVar81 {
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
struct mov_b_instructionVar82 {
    disp_00_07: u8,
}
impl mov_b_instructionVar82 {
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
struct mov_w_instructionVar83 {
    disp_00_07: u8,
}
impl mov_w_instructionVar83 {
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
struct mov_l_instructionVar84 {
    disp_00_07: u8,
}
impl mov_l_instructionVar84 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:828:1, end:828:2))"]
#[derive(Clone, Debug)]
struct swap_b_instructionVar85 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl swap_b_instructionVar85 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:840:1, end:840:2))"]
#[derive(Clone, Debug)]
struct swap_w_instructionVar86 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl swap_w_instructionVar86 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:850:1, end:850:2))"]
#[derive(Clone, Debug)]
struct xtrct_instructionVar87 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl xtrct_instructionVar87 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct add_instructionVar88 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl add_instructionVar88 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:873:1, end:873:2))"]
#[derive(Clone, Debug)]
struct addc_instructionVar89 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl addc_instructionVar89 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:886:1, end:886:2))"]
#[derive(Clone, Debug)]
struct addv_instructionVar90 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl addv_instructionVar90 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct cmp_instructionVar91 {
    simm_00_07: u8,
}
impl cmp_instructionVar91 {
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
struct cmp_instructionVar92 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar92 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct cmp_instructionVar93 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar93 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:920:1, end:920:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar94 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar94 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:925:1, end:925:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar95 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar95 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct cmp_instructionVar96 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar96 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:945:1, end:945:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar97 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl cmp_instructionVar97 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct div0s_instructionVar98 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl div0s_instructionVar98 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct div1_instructionVar99 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl div1_instructionVar99 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct exts_b_instructionVar100 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl exts_b_instructionVar100 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1094:1, end:1094:2))"]
#[derive(Clone, Debug)]
struct exts_w_instructionVar101 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl exts_w_instructionVar101 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct extu_b_instructionVar102 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl extu_b_instructionVar102 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1105:1, end:1105:2))"]
#[derive(Clone, Debug)]
struct extu_w_instructionVar103 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl extu_w_instructionVar103 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mac_w_instructionVar104 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mac_w_instructionVar104 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("+,@"),
            meaning_0_display(self.rn_08_11),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1306:1, end:1306:2))"]
#[derive(Clone, Debug)]
struct muls_w_instructionVar105 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl muls_w_instructionVar105 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1311:1, end:1311:2))"]
#[derive(Clone, Debug)]
struct mulu_w_instructionVar106 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl mulu_w_instructionVar106 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1316:1, end:1316:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar107 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl neg_instructionVar107 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1321:1, end:1321:2))"]
#[derive(Clone, Debug)]
struct negc_instructionVar108 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl negc_instructionVar108 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct sub_instructionVar109 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl sub_instructionVar109 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1338:1, end:1338:2))"]
#[derive(Clone, Debug)]
struct subc_instructionVar110 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl subc_instructionVar110 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1351:1, end:1351:2))"]
#[derive(Clone, Debug)]
struct subv_instructionVar111 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl subv_instructionVar111 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct and_instructionVar112 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl and_instructionVar112 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct and_instructionVar113 {
    imm_00_07: u8,
}
impl and_instructionVar113 {
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
struct and_b_instructionVar114 {
    imm_00_07: u8,
}
impl and_b_instructionVar114 {
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
struct not_instructionVar115 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl not_instructionVar115 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct or_instructionVar116 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl or_instructionVar116 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1400:1, end:1400:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar117 {
    imm_00_07: u8,
}
impl or_instructionVar117 {
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
struct or_b_instructionVar118 {
    imm_00_07: u8,
}
impl or_b_instructionVar118 {
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
struct tst_instructionVar119 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl tst_instructionVar119 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1429:1, end:1429:2))"]
#[derive(Clone, Debug)]
struct tst_instructionVar120 {
    imm_00_07: u8,
}
impl tst_instructionVar120 {
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
struct tst_b_instructionVar121 {
    imm_00_07: u8,
}
impl tst_b_instructionVar121 {
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
struct xor_instructionVar122 {
    rm_04_07: u8,
    rn_08_11: u8,
}
impl xor_instructionVar122 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1449:1, end:1449:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar123 {
    imm_00_07: u8,
}
impl xor_instructionVar123 {
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
struct xor_b_instructionVar124 {
    imm_00_07: u8,
}
impl xor_b_instructionVar124 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1599:1, end:1599:2))"]
#[derive(Clone, Debug)]
struct bf_instructionVar125 {
    target00_07: Tabletarget00_07,
}
impl bf_instructionVar125 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1613:1, end:1613:2))"]
#[derive(Clone, Debug)]
struct bt_instructionVar126 {
    target00_07: Tabletarget00_07,
}
impl bt_instructionVar126 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:1980:1, end:1980:2))"]
#[derive(Clone, Debug)]
struct trapa_instructionVar127 {
    imm_00_07: u8,
}
impl trapa_instructionVar127 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:186:1, end:186:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar128 {
    rn_08_11: u8,
    imm8: Tableimm8,
}
impl mov_instructionVar128 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_w_instructionVar129 {
    rn_08_11: u8,
    disppc2: Tabledisppc2,
}
impl mov_w_instructionVar129 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_l_instructionVar130 {
    rn_08_11: u8,
    disppc4: Tabledisppc4,
}
impl mov_l_instructionVar130 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct mov_l_instructionVar131 {
    rm_04_07: u8,
    rn_08_11: u8,
    disp_00_03: u8,
}
impl mov_l_instructionVar131 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal("),"),
            meaning_0_display(self.rn_08_11),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/SuperH/data/languages/superh.sinc, start:323:1, end:323:2))"]
#[derive(Clone, Debug)]
struct mov_l_instructionVar132 {
    rm_04_07: u8,
    rn_08_11: u8,
    disp_00_03: u8,
}
impl mov_l_instructionVar132 {
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
            meaning_0_display(self.rm_04_07),
            <DisplayElement>::Literal(",@("),
            <DisplayElement>::Number(true, calc_disp.is_negative(), calc_disp.abs() as u64),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.rn_08_11),
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
        let disp_00_03 = token_1(tokens_current);
        let rn_08_11 = token_8(tokens_current);
        let rm_04_07 = token_7(tokens_current);
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
struct add_instructionVar133 {
    simm_00_07: u8,
    rn_08_11: u8,
}
impl add_instructionVar133 {
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
            meaning_0_display(self.rn_08_11),
        ];
        display.extend_from_slice(&extend);
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
struct bra_instructionVar134 {
    target00_11: Tabletarget00_11,
}
impl bra_instructionVar134 {
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
struct bsr_instructionVar135 {
    target00_11: Tabletarget00_11,
}
impl bsr_instructionVar135 {
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
    Var0(div0u_instructionVar0),
    Var1(rts_instructionVar1),
    Var2(clrmac_instructionVar2),
    Var3(clrt_instructionVar3),
    Var4(nop_instructionVar4),
    Var5(rte_instructionVar5),
    Var6(sett_instructionVar6),
    Var7(sleep_instructionVar7),
    Var8(mov_b_instructionVar8),
    Var9(mov_w_instructionVar9),
    Var10(mov_l_instructionVar10),
    Var11(movt_instructionVar11),
    Var12(cmp_instructionVar12),
    Var13(cmp_instructionVar13),
    Var14(tas_b_instructionVar14),
    Var15(rotcl_instructionVar15),
    Var16(rotcr_instructionVar16),
    Var17(rotl_instructionVar17),
    Var18(rotr_instructionVar18),
    Var19(shal_instructionVar19),
    Var20(shar_instructionVar20),
    Var21(shll_instructionVar21),
    Var22(shll2_instructionVar22),
    Var23(shll8_instructionVar23),
    Var24(shll16_instructionVar24),
    Var25(shlr_instructionVar25),
    Var26(shlr2_instructionVar26),
    Var27(shlr8_instructionVar27),
    Var28(shlr16_instructionVar28),
    Var29(jmp_instructionVar29),
    Var30(jsr_instructionVar30),
    Var31(ldc_instructionVar31),
    Var32(ldc_l_instructionVar32),
    Var33(ldc_instructionVar33),
    Var34(ldc_l_instructionVar34),
    Var35(ldc_instructionVar35),
    Var36(ldc_l_instructionVar36),
    Var37(lds_instructionVar37),
    Var38(lds_l_instructionVar38),
    Var39(lds_instructionVar39),
    Var40(lds_l_instructionVar40),
    Var41(lds_instructionVar41),
    Var42(lds_l_instructionVar42),
    Var43(stc_instructionVar43),
    Var44(stc_l_instructionVar44),
    Var45(stc_instructionVar45),
    Var46(stc_l_instructionVar46),
    Var47(stc_instructionVar47),
    Var48(stc_l_instructionVar48),
    Var49(sts_instructionVar49),
    Var50(sts_l_instructionVar50),
    Var51(sts_instructionVar51),
    Var52(sts_l_instructionVar52),
    Var53(sts_instructionVar53),
    Var54(sts_l_instructionVar54),
    Var55(mov_instructionVar55),
    Var56(mova_instructionVar56),
    Var57(mov_b_instructionVar57),
    Var58(mov_w_instructionVar58),
    Var59(mov_l_instructionVar59),
    Var60(mov_b_instructionVar60),
    Var61(mov_w_instructionVar61),
    Var62(mov_l_instructionVar62),
    Var63(mov_b_instructionVar63),
    Var64(mov_w_instructionVar64),
    Var65(mov_l_instructionVar65),
    Var66(mov_b_instructionVar66),
    Var67(mov_w_instructionVar67),
    Var68(mov_l_instructionVar68),
    Var69(mov_b_instructionVar69),
    Var70(mov_w_instructionVar70),
    Var71(mov_b_instructionVar71),
    Var72(mov_w_instructionVar72),
    Var73(mov_b_instructionVar73),
    Var74(mov_w_instructionVar74),
    Var75(mov_l_instructionVar75),
    Var76(mov_b_instructionVar76),
    Var77(mov_w_instructionVar77),
    Var78(mov_l_instructionVar78),
    Var79(mov_b_instructionVar79),
    Var80(mov_w_instructionVar80),
    Var81(mov_l_instructionVar81),
    Var82(mov_b_instructionVar82),
    Var83(mov_w_instructionVar83),
    Var84(mov_l_instructionVar84),
    Var85(swap_b_instructionVar85),
    Var86(swap_w_instructionVar86),
    Var87(xtrct_instructionVar87),
    Var88(add_instructionVar88),
    Var89(addc_instructionVar89),
    Var90(addv_instructionVar90),
    Var91(cmp_instructionVar91),
    Var92(cmp_instructionVar92),
    Var93(cmp_instructionVar93),
    Var94(cmp_instructionVar94),
    Var95(cmp_instructionVar95),
    Var96(cmp_instructionVar96),
    Var97(cmp_instructionVar97),
    Var98(div0s_instructionVar98),
    Var99(div1_instructionVar99),
    Var100(exts_b_instructionVar100),
    Var101(exts_w_instructionVar101),
    Var102(extu_b_instructionVar102),
    Var103(extu_w_instructionVar103),
    Var104(mac_w_instructionVar104),
    Var105(muls_w_instructionVar105),
    Var106(mulu_w_instructionVar106),
    Var107(neg_instructionVar107),
    Var108(negc_instructionVar108),
    Var109(sub_instructionVar109),
    Var110(subc_instructionVar110),
    Var111(subv_instructionVar111),
    Var112(and_instructionVar112),
    Var113(and_instructionVar113),
    Var114(and_b_instructionVar114),
    Var115(not_instructionVar115),
    Var116(or_instructionVar116),
    Var117(or_instructionVar117),
    Var118(or_b_instructionVar118),
    Var119(tst_instructionVar119),
    Var120(tst_instructionVar120),
    Var121(tst_b_instructionVar121),
    Var122(xor_instructionVar122),
    Var123(xor_instructionVar123),
    Var124(xor_b_instructionVar124),
    Var125(bf_instructionVar125),
    Var126(bt_instructionVar126),
    Var127(trapa_instructionVar127),
    Var128(mov_instructionVar128),
    Var129(mov_w_instructionVar129),
    Var130(mov_l_instructionVar130),
    Var131(mov_l_instructionVar131),
    Var132(mov_l_instructionVar132),
    Var133(add_instructionVar133),
    Var134(bra_instructionVar134),
    Var135(bsr_instructionVar135),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                div0u_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 11
        {
            if let Some((inst_len, parsed)) =
                rts_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 40
        {
            if let Some((inst_len, parsed)) =
                clrmac_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 8 {
            if let Some((inst_len, parsed)) =
                clrt_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 9 {
            if let Some((inst_len, parsed)) =
                nop_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                rte_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                sett_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 255) == 27
        {
            if let Some((inst_len, parsed)) =
                sleep_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 41
        {
            if let Some((inst_len, parsed)) =
                movt_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 21
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 17
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 27
        {
            if let Some((inst_len, parsed)) =
                tas_b_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 36
        {
            if let Some((inst_len, parsed)) =
                rotcl_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 37
        {
            if let Some((inst_len, parsed)) =
                rotcr_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 4
        {
            if let Some((inst_len, parsed)) =
                rotl_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 5
        {
            if let Some((inst_len, parsed)) =
                rotr_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 32
        {
            if let Some((inst_len, parsed)) =
                shal_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 33
        {
            if let Some((inst_len, parsed)) =
                shar_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                shll_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 8
        {
            if let Some((inst_len, parsed)) =
                shll2_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                shll8_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 40
        {
            if let Some((inst_len, parsed)) =
                shll16_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                shlr_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 9
        {
            if let Some((inst_len, parsed)) =
                shlr2_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                shlr8_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 41
        {
            if let Some((inst_len, parsed)) =
                shlr16_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                jmp_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 11
        {
            if let Some((inst_len, parsed)) =
                jsr_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 7
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 23
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 39
        {
            if let Some((inst_len, parsed)) =
                ldc_l_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 10
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 26
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                lds_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                lds_l_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                stc_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 18
        {
            if let Some((inst_len, parsed)) =
                stc_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 19
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                stc_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                stc_l_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 10
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 26
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 18
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                sts_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                sts_l_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 199 {
            if let Some((inst_len, parsed)) =
                mova_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 132 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 133 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 128 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 129 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 0 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 196 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 197 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 192 {
            if let Some((inst_len, parsed)) =
                mov_b_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 193 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 194 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                swap_b_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                swap_w_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                xtrct_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                add_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                addc_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                addv_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 136 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                div0s_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                div1_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                exts_b_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                exts_w_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 12
        {
            if let Some((inst_len, parsed)) =
                extu_b_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 13
        {
            if let Some((inst_len, parsed)) =
                extu_w_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 64 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                mac_w_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 15
        {
            if let Some((inst_len, parsed)) =
                muls_w_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 14
        {
            if let Some((inst_len, parsed)) =
                mulu_w_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                neg_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                negc_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                subc_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 48 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                subv_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                and_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) =
                and_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                and_b_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 96 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                not_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 11
        {
            if let Some((inst_len, parsed)) =
                or_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 {
            if let Some((inst_len, parsed)) =
                or_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 207 {
            if let Some((inst_len, parsed)) =
                or_b_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                tst_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 200 {
            if let Some((inst_len, parsed)) =
                tst_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 204 {
            if let Some((inst_len, parsed)) =
                tst_b_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 32 && (tokens_param[1] & 15) == 10
        {
            if let Some((inst_len, parsed)) =
                xor_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 202 {
            if let Some((inst_len, parsed)) =
                xor_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) =
                xor_b_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 139 {
            if let Some((inst_len, parsed)) =
                bf_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 137 {
            if let Some((inst_len, parsed)) =
                bt_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) =
                trapa_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 224 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 144 {
            if let Some((inst_len, parsed)) =
                mov_w_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 208 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 80 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 16 {
            if let Some((inst_len, parsed)) =
                mov_l_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 112 {
            if let Some((inst_len, parsed)) =
                add_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 160 {
            if let Some((inst_len, parsed)) =
                bra_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 176 {
            if let Some((inst_len, parsed)) =
                bsr_instructionVar135::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
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
