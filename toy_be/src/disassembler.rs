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
    sp,
    lr,
    pc,
    C,
    Z,
    N,
    V,
    r0h,
    r0l,
    r1h,
    r1l,
    r2h,
    r2l,
    r3h,
    r3l,
    r4h,
    r4l,
    r5h,
    r5l,
    r6h,
    r6l,
    r7h,
    r7l,
    r8h,
    r8l,
    r9h,
    r9l,
    r10h,
    r10l,
    r11h,
    r11l,
    r12h,
    r12l,
    sph,
    spl,
    lrh,
    lrl,
    pch,
    pcl,
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
            Self::sp => "sp",
            Self::lr => "lr",
            Self::pc => "pc",
            Self::C => "C",
            Self::Z => "Z",
            Self::N => "N",
            Self::V => "V",
            Self::r0h => "r0h",
            Self::r0l => "r0l",
            Self::r1h => "r1h",
            Self::r1l => "r1l",
            Self::r2h => "r2h",
            Self::r2l => "r2l",
            Self::r3h => "r3h",
            Self::r3l => "r3l",
            Self::r4h => "r4h",
            Self::r4l => "r4l",
            Self::r5h => "r5h",
            Self::r5l => "r5l",
            Self::r6h => "r6h",
            Self::r6l => "r6l",
            Self::r7h => "r7h",
            Self::r7l => "r7l",
            Self::r8h => "r8h",
            Self::r8l => "r8l",
            Self::r9h => "r9h",
            Self::r9l => "r9l",
            Self::r10h => "r10h",
            Self::r10l => "r10l",
            Self::r11h => "r11h",
            Self::r11l => "r11l",
            Self::r12h => "r12h",
            Self::r12l => "r12l",
            Self::sph => "sph",
            Self::spl => "spl",
            Self::lrh => "lrh",
            Self::lrl => "lrl",
            Self::pch => "pch",
            Self::pcl => "pcl",
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
        13 => Register::sp,
        14 => Register::lr,
        15 => Register::pc,
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
#[doc = "Create token_fields: op0003 rt imm0003 simm0003"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: simm0010"]
fn token_11(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 2047) as u16)
}
#[doc = "Create token_fields: simm0411"]
fn token_12(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 255) as u8)
}
#[doc = "Create token_fields: cc0911"]
fn token_13(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 7) as u8)
}
#[doc = "Create token_fields: cc0002"]
fn token_14(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: op1215"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: op1515"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 15) & 1) as u8)
}
#[doc = "Create token_fields: op1111"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 11) & 1) as u8)
}
#[doc = "Create token_fields: op0811 rd"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 15) as u8)
}
#[doc = "Create token_fields: op0407 rs"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: op0007 imm0007 simm0007"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: op0303"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 1) as u8)
}
#[doc = "Create token_fields: imm1213 simm1213"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 3) as u8)
}
#[doc = "Create token_fields: op1415"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 14) & 3) as u8)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:217:1, end:217:2))"]
#[derive(Clone, Debug)]
struct ret_instructionVar0 {}
impl ret_instructionVar0 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:227:1, end:227:2))"]
#[derive(Clone, Debug)]
struct user_three_instructionVar1 {}
impl user_three_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_three"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:232:1, end:232:2))"]
#[derive(Clone, Debug)]
struct unimpl_instructionVar2 {}
impl unimpl_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("unimpl"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:197:1, end:197:2))"]
#[derive(Clone, Debug)]
struct inv_instructionVar3 {
    rs: u8,
}
impl inv_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("inv"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:198:1, end:198:2))"]
#[derive(Clone, Debug)]
struct neg_instructionVar4 {
    rs: u8,
}
impl neg_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("neg"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct sk_instructionVar5 {
    Rel82: TableRel82,
    CC: TableCC,
}
impl sk_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sk"));
        self.CC
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
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel82, CC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct push_instructionVar6 {
    rs: u8,
}
impl push_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("push"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:215:1, end:215:2))"]
#[derive(Clone, Debug)]
struct pop_instructionVar7 {
    rs: u8,
}
impl pop_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("pop"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar8 {
    rs: u8,
}
impl call_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("call"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:225:1, end:225:2))"]
#[derive(Clone, Debug)]
struct user_one_instructionVar9 {
    rs: u8,
}
impl user_one_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_one"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:226:1, end:226:2))"]
#[derive(Clone, Debug)]
struct user_two_instructionVar10 {
    rs: u8,
}
impl user_two_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_two"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:230:1, end:230:2))"]
#[derive(Clone, Debug)]
struct user_six_instructionVar11 {
    rs: u8,
}
impl user_six_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_six"));
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:208:1, end:208:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar12 {
    rs: u8,
    COND: TableCOND,
}
impl br_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("br"));
        self.COND
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:209:1, end:209:2))"]
#[derive(Clone, Debug)]
struct brds_instructionVar13 {
    rs: u8,
    COND: TableCOND,
}
impl brds_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("brds"));
        self.COND
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:223:1, end:223:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar14 {
    rs: u8,
    COND: TableCOND,
}
impl call_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("call"));
        self.COND
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] =
            [<DisplayElement>::Literal(" "), meaning_0_display(self.rs)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { COND, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:158:1, end:158:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar15 {
    rs: u8,
    rt: u8,
}
impl add_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("add"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:159:1, end:159:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar16 {
    rs: u8,
    rt: u8,
}
impl sub_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sub"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:160:1, end:160:2))"]
#[derive(Clone, Debug)]
struct rsub_instructionVar17 {
    rs: u8,
    rt: u8,
}
impl rsub_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsub"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:161:1, end:161:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar18 {
    rs: u8,
    rt: u8,
}
impl mul_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar19 {
    rs: u8,
    rt: u8,
}
impl div_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:163:1, end:163:2))"]
#[derive(Clone, Debug)]
struct mod_instructionVar20 {
    rs: u8,
    rt: u8,
}
impl mod_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mod"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:164:1, end:164:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar21 {
    rs: u8,
    rt: u8,
}
impl cmp_instructionVar21 {
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
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rt = token_8(tokens_current);
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct ucmp_instructionVar22 {
    rs: u8,
    rt: u8,
}
impl ucmp_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ucmp"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:167:1, end:167:2))"]
#[derive(Clone, Debug)]
struct add_instructionVar23 {
    rs: u8,
    Simm4: TableSimm4,
}
impl add_instructionVar23 {
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
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:168:1, end:168:2))"]
#[derive(Clone, Debug)]
struct sub_instructionVar24 {
    rs: u8,
    Simm4: TableSimm4,
}
impl sub_instructionVar24 {
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
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:169:1, end:169:2))"]
#[derive(Clone, Debug)]
struct rsub_instructionVar25 {
    rs: u8,
    Simm4: TableSimm4,
}
impl rsub_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("rsub"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct mul_instructionVar26 {
    rs: u8,
    Simm4: TableSimm4,
}
impl mul_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mul"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:171:1, end:171:2))"]
#[derive(Clone, Debug)]
struct div_instructionVar27 {
    rs: u8,
    Simm4: TableSimm4,
}
impl div_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("div"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:172:1, end:172:2))"]
#[derive(Clone, Debug)]
struct mod_instructionVar28 {
    rs: u8,
    Imm4: TableImm4,
}
impl mod_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mod"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4
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
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:173:1, end:173:2))"]
#[derive(Clone, Debug)]
struct cmp_instructionVar29 {
    rs: u8,
    Simm4: TableSimm4,
}
impl cmp_instructionVar29 {
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
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm4
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
        let Simm4 = if let Some((len, table)) =
            TableSimm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:174:1, end:174:2))"]
#[derive(Clone, Debug)]
struct ucmp_instructionVar30 {
    rs: u8,
    Imm4: TableImm4,
}
impl ucmp_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ucmp"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4
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
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:176:1, end:176:2))"]
#[derive(Clone, Debug)]
struct and_instructionVar31 {
    rs: u8,
    rt: u8,
}
impl and_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("and"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:177:1, end:177:2))"]
#[derive(Clone, Debug)]
struct or_instructionVar32 {
    rs: u8,
    rt: u8,
}
impl or_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("or"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:178:1, end:178:2))"]
#[derive(Clone, Debug)]
struct xor_instructionVar33 {
    rs: u8,
    rt: u8,
}
impl xor_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("xor"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:179:1, end:179:2))"]
#[derive(Clone, Debug)]
struct lsr_instructionVar34 {
    rs: u8,
    rt: u8,
}
impl lsr_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsr"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct asr_instructionVar35 {
    rs: u8,
    rt: u8,
}
impl asr_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("asr"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:181:1, end:181:2))"]
#[derive(Clone, Debug)]
struct lsl_instructionVar36 {
    rs: u8,
    rt: u8,
}
impl lsl_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsl"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rt = token_8(tokens_current);
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:192:1, end:192:2))"]
#[derive(Clone, Debug)]
struct saa_instructionVar37 {
    rs: u8,
    rt: u8,
}
impl saa_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("saa"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:194:1, end:194:2))"]
#[derive(Clone, Debug)]
struct lsr_instructionVar38 {
    rs: u8,
    Imm4: TableImm4,
}
impl lsr_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsr"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4
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
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:195:1, end:195:2))"]
#[derive(Clone, Debug)]
struct asr_instructionVar39 {
    rs: u8,
    Imm4: TableImm4,
}
impl asr_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("asr"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4
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
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct lsl_instructionVar40 {
    rs: u8,
    Imm4: TableImm4,
}
impl lsl_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsl"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm4
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
        let Imm4 = if let Some((len, table)) =
            TableImm4::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm4, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:200:1, end:200:2))"]
#[derive(Clone, Debug)]
struct load_instructionVar41 {
    rs: u8,
    RT: TableRT,
}
impl load_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("load"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.RT
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
        let RT = if let Some((len, table)) =
            TableRT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RT, rs }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:201:1, end:201:2))"]
#[derive(Clone, Debug)]
struct store_instructionVar42 {
    rt: u8,
    RS: TableRS,
}
impl store_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("store"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RS
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let RS = if let Some((len, table)) =
            TableRS::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RS, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:202:1, end:202:2))"]
#[derive(Clone, Debug)]
struct mov_instructionVar43 {
    rs: u8,
    rt: u8,
}
impl mov_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("mov"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:206:1, end:206:2))"]
#[derive(Clone, Debug)]
struct br_instructionVar44 {
    Rel82: TableRel82,
    COND: TableCOND,
}
impl br_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("br"));
        self.COND
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel82
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
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel82, COND }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:207:1, end:207:2))"]
#[derive(Clone, Debug)]
struct brds_instructionVar45 {
    Rel82: TableRel82,
    COND: TableCOND,
}
impl brds_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("brds"));
        self.COND
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel82
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
        let COND = if let Some((len, table)) =
            TableCOND::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let Rel82 = if let Some((len, table)) =
            TableRel82::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel82, COND }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:218:1, end:218:2))"]
#[derive(Clone, Debug)]
struct callds_instructionVar46 {
    Rel8: TableRel8,
}
impl callds_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("callds"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8
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
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:228:1, end:228:2))"]
#[derive(Clone, Debug)]
struct user_four_instructionVar47 {
    rs: u8,
    rt: u8,
}
impl user_four_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_four"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rs),
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rt),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let rs = token_6(tokens_current);
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs, rt }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:229:1, end:229:2))"]
#[derive(Clone, Debug)]
struct user_five_instructionVar48 {
    Rel8: TableRel8,
}
impl user_five_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("user_five"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Rel8
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
        let Rel8 = if let Some((len, table)) =
            TableRel8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:220:1, end:220:2))"]
#[derive(Clone, Debug)]
struct call_instructionVar49 {
    Rel11: TableRel11,
}
impl call_instructionVar49 {
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
        self.Rel11
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
        let Rel11 = if let Some((len, table)) =
            TableRel11::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Rel11 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct simm_instructionVar50 {
    rd: u8,
    Simm10: TableSimm10,
}
impl simm_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("simm"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rd),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Simm10
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
        let Simm10 = if let Some((len, table)) =
            TableSimm10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rd = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Simm10, rd }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:155:1, end:155:2))"]
#[derive(Clone, Debug)]
struct imm_instructionVar51 {
    rd: u8,
    Imm10: TableImm10,
}
impl imm_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("imm"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.rd),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm10
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
        let Imm10 = if let Some((len, table)) =
            TableImm10::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let rd = token_5(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm10, rd }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(ret_instructionVar0),
    Var1(user_three_instructionVar1),
    Var2(unimpl_instructionVar2),
    Var3(inv_instructionVar3),
    Var4(neg_instructionVar4),
    Var5(sk_instructionVar5),
    Var6(push_instructionVar6),
    Var7(pop_instructionVar7),
    Var8(call_instructionVar8),
    Var9(user_one_instructionVar9),
    Var10(user_two_instructionVar10),
    Var11(user_six_instructionVar11),
    Var12(br_instructionVar12),
    Var13(brds_instructionVar13),
    Var14(call_instructionVar14),
    Var15(add_instructionVar15),
    Var16(sub_instructionVar16),
    Var17(rsub_instructionVar17),
    Var18(mul_instructionVar18),
    Var19(div_instructionVar19),
    Var20(mod_instructionVar20),
    Var21(cmp_instructionVar21),
    Var22(ucmp_instructionVar22),
    Var23(add_instructionVar23),
    Var24(sub_instructionVar24),
    Var25(rsub_instructionVar25),
    Var26(mul_instructionVar26),
    Var27(div_instructionVar27),
    Var28(mod_instructionVar28),
    Var29(cmp_instructionVar29),
    Var30(ucmp_instructionVar30),
    Var31(and_instructionVar31),
    Var32(or_instructionVar32),
    Var33(xor_instructionVar33),
    Var34(lsr_instructionVar34),
    Var35(asr_instructionVar35),
    Var36(lsl_instructionVar36),
    Var37(saa_instructionVar37),
    Var38(lsr_instructionVar38),
    Var39(asr_instructionVar39),
    Var40(lsl_instructionVar40),
    Var41(load_instructionVar41),
    Var42(store_instructionVar42),
    Var43(mov_instructionVar43),
    Var44(br_instructionVar44),
    Var45(brds_instructionVar45),
    Var46(callds_instructionVar46),
    Var47(user_four_instructionVar47),
    Var48(user_five_instructionVar48),
    Var49(call_instructionVar49),
    Var50(simm_instructionVar50),
    Var51(imm_instructionVar51),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 244 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                ret_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 163 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                user_three_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 168 && (tokens_param[1] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                unimpl_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 222 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                inv_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 222 && (tokens_param[1] & 15) == 1
        {
            if let Some((inst_len, parsed)) =
                neg_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 128 && (tokens_param[1] & 248) == 0
        {
            if let Some((inst_len, parsed)) =
                sk_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 242 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                push_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 243 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                pop_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 246 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                call_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 161 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                user_one_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 162 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                user_two_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 166 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                user_six_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 240 && (tokens_param[1] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                br_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 241 && (tokens_param[1] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                brds_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 246 && (tokens_param[1] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                call_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 192 {
            if let Some((inst_len, parsed)) =
                add_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 193 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 194 {
            if let Some((inst_len, parsed)) =
                rsub_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) =
                mul_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 196 {
            if let Some((inst_len, parsed)) =
                div_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 197 {
            if let Some((inst_len, parsed)) =
                mod_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 199 {
            if let Some((inst_len, parsed)) =
                ucmp_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 200 {
            if let Some((inst_len, parsed)) =
                add_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) =
                sub_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 202 {
            if let Some((inst_len, parsed)) =
                rsub_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 {
            if let Some((inst_len, parsed)) =
                mul_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 204 {
            if let Some((inst_len, parsed)) =
                div_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                mod_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) =
                cmp_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 207 {
            if let Some((inst_len, parsed)) =
                ucmp_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 208 {
            if let Some((inst_len, parsed)) =
                and_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 209 {
            if let Some((inst_len, parsed)) =
                or_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 210 {
            if let Some((inst_len, parsed)) =
                xor_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 211 {
            if let Some((inst_len, parsed)) =
                lsr_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 212 {
            if let Some((inst_len, parsed)) =
                asr_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 213 {
            if let Some((inst_len, parsed)) =
                lsl_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 216 {
            if let Some((inst_len, parsed)) =
                saa_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 219 {
            if let Some((inst_len, parsed)) =
                lsr_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 220 {
            if let Some((inst_len, parsed)) =
                asr_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 221 {
            if let Some((inst_len, parsed)) =
                lsl_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 214 {
            if let Some((inst_len, parsed)) =
                load_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 215 {
            if let Some((inst_len, parsed)) =
                store_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 223 {
            if let Some((inst_len, parsed)) =
                mov_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 224 && (tokens_param[1] & 8) == 0 {
            if let Some((inst_len, parsed)) =
                br_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 224 && (tokens_param[1] & 8) == 8 {
            if let Some((inst_len, parsed)) =
                brds_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 245 {
            if let Some((inst_len, parsed)) =
                callds_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 164 {
            if let Some((inst_len, parsed)) =
                user_four_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 165 {
            if let Some((inst_len, parsed)) =
                user_five_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 248) == 248 {
            if let Some((inst_len, parsed)) =
                call_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                simm_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                imm_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:107:1, end:107:6))"]
#[derive(Clone, Debug)]
struct Simm4Var0 {
    simm0003: u8,
}
impl Simm4Var0 {
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
                (if self.simm0003 & 8 != 0 { -1 & !7 } else { 0 } | self.simm0003 as i8)
                    .is_negative(),
                (if self.simm0003 & 8 != 0 { -1 & !7 } else { 0 } | self.simm0003 as i8).abs()
                    as u64,
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
        let simm0003 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0003 }))
    }
}
#[derive(Clone, Debug)]
enum TableSimm4 {
    Var0(Simm4Var0),
}
impl TableSimm4 {
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
                Simm4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:108:1, end:108:7))"]
#[derive(Clone, Debug)]
struct Simm10Var0 {
    simm1213: u8,
    imm0007: u8,
}
impl Simm10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_computed: i128 = 0;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(
                    (if self.simm1213 & 2 != 0 { -1 & !1 } else { 0 } | self.simm1213 as i8),
                )
                .unwrap()
                .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(self.imm0007).unwrap());
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("#"),
            <DisplayElement>::Number(
                true,
                calc_computed.is_negative(),
                calc_computed.abs() as u64,
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
        let mut calc_computed: i128 = 0;
        let mut block_0_len = 2;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_10(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap());
        let simm1213 = token_10(tokens_current);
        let imm0007 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm1213, imm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableSimm10 {
    Var0(Simm10Var0),
}
impl TableSimm10 {
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
                Simm10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:110:1, end:110:5))"]
#[derive(Clone, Debug)]
struct Imm4Var0 {
    imm0003: u8,
}
impl Imm4Var0 {
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
            DisplayElement::Number(true, false, self.imm0003 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let imm0003 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm0003 }))
    }
}
#[derive(Clone, Debug)]
enum TableImm4 {
    Var0(Imm4Var0),
}
impl TableImm4 {
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
                Imm4Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:111:1, end:111:6))"]
#[derive(Clone, Debug)]
struct Imm10Var0 {
    imm1213: u8,
    imm0007: u8,
}
impl Imm10Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_computed: i128 = 0;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.imm1213).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.imm0007).unwrap());
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("#"),
            <DisplayElement>::Number(
                true,
                calc_computed.is_negative(),
                calc_computed.abs() as u64,
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
        let mut calc_computed: i128 = 0;
        let mut block_0_len = 2;
        calc_computed = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_10(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_7(tokens_current)).unwrap());
        let imm0007 = token_7(tokens_current);
        let imm1213 = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm1213, imm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableImm10 {
    Var0(Imm10Var0),
}
impl TableImm10 {
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
                Imm10Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:113:1, end:113:5))"]
#[derive(Clone, Debug)]
struct Rel8Var0 {
    simm0007: u8,
}
impl Rel8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.simm0007 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm0007 as i8),
            )
            .unwrap(),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_addr.is_negative(),
            calc_addr.abs() as u64,
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
        let mut calc_addr: i128 = 0;
        let mut block_0_len = 2;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(i128::try_from(token_7(tokens_current)).unwrap());
        let simm0007 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0007 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel8 {
    Var0(Rel8Var0),
}
impl TableRel8 {
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
                Rel8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:114:1, end:114:6))"]
#[derive(Clone, Debug)]
struct Rel82Var0 {
    simm0411: u8,
}
impl Rel82Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.simm0411 & 128 != 0 {
                    -1 & !127
                } else {
                    0
                } | self.simm0411 as i8),
            )
            .unwrap(),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_addr.is_negative(),
            calc_addr.abs() as u64,
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
        let mut calc_addr: i128 = 0;
        let mut block_0_len = 2;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(i128::try_from(token_12(tokens_current)).unwrap());
        let simm0411 = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0411 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel82 {
    Var0(Rel82Var0),
}
impl TableRel82 {
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
                Rel82Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:115:1, end:115:6))"]
#[derive(Clone, Debug)]
struct Rel11Var0 {
    simm0010: u16,
}
impl Rel11Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_addr: i128 = 0;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            i128::try_from(
                (if self.simm0010 & 1024 != 0 {
                    -1 & !1023
                } else {
                    0
                } | self.simm0010 as i16),
            )
            .unwrap(),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_addr.is_negative(),
            calc_addr.abs() as u64,
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
        let mut calc_addr: i128 = 0;
        let mut block_0_len = 2;
        calc_addr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(i128::try_from(token_11(tokens_current)).unwrap());
        let simm0010 = token_11(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm0010 }))
    }
}
#[derive(Clone, Debug)]
enum TableRel11 {
    Var0(Rel11Var0),
}
impl TableRel11 {
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
                Rel11Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:117:1, end:117:3))"]
#[derive(Clone, Debug)]
struct RSVar0 {
    rs: u8,
}
impl RSVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("["),
            meaning_0_display(self.rs),
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
        let rs = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rs }))
    }
}
#[derive(Clone, Debug)]
enum TableRS {
    Var0(RSVar0),
}
impl TableRS {
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
                RSVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:118:1, end:118:3))"]
#[derive(Clone, Debug)]
struct RTVar0 {
    rt: u8,
}
impl RTVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("["),
            meaning_0_display(self.rt),
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
        let rt = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rt }))
    }
}
#[derive(Clone, Debug)]
enum TableRT {
    Var0(RTVar0),
}
impl TableRT {
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
                RTVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:120:1, end:120:3))"]
#[derive(Clone, Debug)]
struct CCVar0 {}
impl CCVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("eq")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:121:1, end:121:3))"]
#[derive(Clone, Debug)]
struct CCVar1 {}
impl CCVar1 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:122:1, end:122:3))"]
#[derive(Clone, Debug)]
struct CCVar2 {}
impl CCVar2 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:123:1, end:123:3))"]
#[derive(Clone, Debug)]
struct CCVar3 {}
impl CCVar3 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:124:1, end:124:3))"]
#[derive(Clone, Debug)]
struct CCVar4 {}
impl CCVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("lo")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:125:1, end:125:3))"]
#[derive(Clone, Debug)]
struct CCVar5 {}
impl CCVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("mi")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:126:1, end:126:3))"]
#[derive(Clone, Debug)]
struct CCVar6 {}
impl CCVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("vs")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:127:1, end:127:3))"]
#[derive(Clone, Debug)]
struct CCVar7 {}
impl CCVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("")];
        display.extend_from_slice(&extend);
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
enum TableCC {
    Var0(CCVar0),
    Var1(CCVar1),
    Var2(CCVar2),
    Var3(CCVar3),
    Var4(CCVar4),
    Var5(CCVar5),
    Var6(CCVar6),
    Var7(CCVar7),
}
impl TableCC {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 0 {
            if let Some((inst_len, parsed)) =
                CCVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 1 {
            if let Some((inst_len, parsed)) =
                CCVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 2 {
            if let Some((inst_len, parsed)) =
                CCVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 3 {
            if let Some((inst_len, parsed)) =
                CCVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 4 {
            if let Some((inst_len, parsed)) =
                CCVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 5 {
            if let Some((inst_len, parsed)) =
                CCVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 6 {
            if let Some((inst_len, parsed)) =
                CCVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 7 {
            if let Some((inst_len, parsed)) =
                CCVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:130:1, end:130:5))"]
#[derive(Clone, Debug)]
struct CONDVar0 {
    CC: TableCC,
}
impl CONDVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.CC
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
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Toy/data/languages/toyInstructions.sinc, start:129:1, end:129:5))"]
#[derive(Clone, Debug)]
struct CONDVar1 {
    CC: TableCC,
}
impl CONDVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.CC
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
        let CC = if let Some((len, table)) =
            TableCC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CC }))
    }
}
#[derive(Clone, Debug)]
enum TableCOND {
    Var0(CONDVar0),
    Var1(CONDVar1),
}
impl TableCOND {
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
        if tokens_param.len() >= 2 && (tokens_param[1] & 7) == 7 {
            if let Some((inst_len, parsed)) =
                CONDVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                CONDVar1::parse(tokens_param, &mut context_current, inst_start)
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
