pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    F,
    A,
    C,
    B,
    E,
    D,
    L,
    H,
    AF,
    BC,
    DE,
    HL,
    A_,
    F_,
    B_,
    C_,
    D_,
    E_,
    H_,
    L_,
    AF_,
    BC_,
    DE_,
    HL_,
    PC,
    SP,
    S_flag,
    Z_flag,
    AC_flag,
    P_flag,
    CY_flag,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::F => "F",
            Self::A => "A",
            Self::C => "C",
            Self::B => "B",
            Self::E => "E",
            Self::D => "D",
            Self::L => "L",
            Self::H => "H",
            Self::AF => "AF",
            Self::BC => "BC",
            Self::DE => "DE",
            Self::HL => "HL",
            Self::A_ => "A_",
            Self::F_ => "F_",
            Self::B_ => "B_",
            Self::C_ => "C_",
            Self::D_ => "D_",
            Self::E_ => "E_",
            Self::H_ => "H_",
            Self::L_ => "L_",
            Self::AF_ => "AF_",
            Self::BC_ => "BC_",
            Self::DE_ => "DE_",
            Self::HL_ => "HL_",
            Self::PC => "PC",
            Self::SP => "SP",
            Self::S_flag => "S_flag",
            Self::Z_flag => "Z_flag",
            Self::AC_flag => "AC_flag",
            Self::P_flag => "P_flag",
            Self::CY_flag => "CY_flag",
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
        0 => Register::B,
        1 => Register::C,
        2 => Register::D,
        3 => Register::E,
        4 => Register::H,
        5 => Register::L,
        7 => Register::A,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::HL,
        3 => Register::SP,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::HL,
        3 => Register::AF,
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
#[doc = "Create token_fields: dRegPair4_2 pRegPair4_2 sRegPair4_2 qRegPair4_2 rRegPair4_2"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 3) as u8)
}
#[doc = "Create token_fields: reg0_3 bits0_3"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: sign8"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 7) & 1) as u8)
}
#[doc = "Create token_fields: sign16"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 15) & 1) as u8)
}
#[doc = "Create token_fields: imm16 simm16"]
fn token_8(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: reg3_3 bits3_3"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 3) & 7) as u8)
}
#[doc = "Create token_fields: op0_8 imm8 simm8"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: op6_2"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 6) & 3) as u8)
}
#[doc = "Create token_fields: bits0_4"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 15) as u8)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:151:1, end:151:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar0 {
    imm8: u8,
}
impl MVI_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVI"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
            <DisplayElement>::Literal("),"),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:156:1, end:156:2))"]
#[derive(Clone, Debug)]
struct LDAX_instructionVar1 {}
impl LDAX_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDAX"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::BC),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:161:1, end:161:2))"]
#[derive(Clone, Debug)]
struct LDAX_instructionVar2 {}
impl LDAX_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDAX"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::DE),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar3 {
    Mem8: TableMem8,
}
impl LDA_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Mem8 = if let Some((len, table)) =
            TableMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct STAX_instructionVar4 {}
impl STAX_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STAX"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::BC),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:175:1, end:175:2))"]
#[derive(Clone, Debug)]
struct STAX_instructionVar5 {}
impl STAX_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STAX"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::DE),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar6 {
    Mem8: TableMem8,
}
impl STA_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Mem8 = if let Some((len, table)) =
            TableMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:188:1, end:188:2))"]
#[derive(Clone, Debug)]
struct LHLD_instructionVar7 {
    Mem16: TableMem16,
}
impl LHLD_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LHLD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:192:1, end:192:2))"]
#[derive(Clone, Debug)]
struct SHLD_instructionVar8 {
    Mem16: TableMem16,
}
impl SHLD_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SHLD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct SPHL_instructionVar9 {}
impl SPHL_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SPHL"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:208:1, end:208:2))"]
#[derive(Clone, Debug)]
struct XCHG_instructionVar10 {}
impl XCHG_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XCHG"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct XTHL_instructionVar11 {}
impl XTHL_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XTHL"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:226:1, end:226:2))"]
#[derive(Clone, Debug)]
struct ADI_instructionVar12 {
    imm8: u8,
}
impl ADI_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:232:1, end:232:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar13 {}
impl ADD_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct ACI_instructionVar14 {
    imm8: u8,
}
impl ACI_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ACI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:251:1, end:251:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar15 {}
impl ADC_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:264:1, end:264:2))"]
#[derive(Clone, Debug)]
struct SUI_instructionVar16 {
    imm8: u8,
}
impl SUI_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:270:1, end:270:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar17 {}
impl SUB_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:283:1, end:283:2))"]
#[derive(Clone, Debug)]
struct SBI_instructionVar18 {
    imm8: u8,
}
impl SBI_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:289:1, end:289:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar19 {}
impl SBB_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct ANI_instructionVar20 {
    imm8: u8,
}
impl ANI_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:312:1, end:312:2))"]
#[derive(Clone, Debug)]
struct ANA_instructionVar21 {}
impl ANA_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANA"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:328:1, end:328:2))"]
#[derive(Clone, Debug)]
struct ORI_instructionVar22 {
    imm8: u8,
}
impl ORI_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ORI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:336:1, end:336:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar23 {}
impl ORA_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ORA"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:352:1, end:352:2))"]
#[derive(Clone, Debug)]
struct XRA_instructionVar24 {}
impl XRA_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XRA"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:360:1, end:360:2))"]
#[derive(Clone, Debug)]
struct XRI_instructionVar25 {
    imm8: u8,
}
impl XRI_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XRI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:374:1, end:374:2))"]
#[derive(Clone, Debug)]
struct CPI_instructionVar26 {
    imm8: u8,
}
impl CPI_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar27 {}
impl CMP_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:393:1, end:393:2))"]
#[derive(Clone, Debug)]
struct INR_instructionVar28 {}
impl INR_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INR"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:407:1, end:407:2))"]
#[derive(Clone, Debug)]
struct DCR_instructionVar29 {}
impl DCR_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DCR"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct DAA_instructionVar30 {}
impl DAA_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DAA"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:421:1, end:421:2))"]
#[derive(Clone, Debug)]
struct CMA_instructionVar31 {}
impl CMA_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMA"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:425:1, end:425:2))"]
#[derive(Clone, Debug)]
struct CMC_instructionVar32 {}
impl CMC_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:429:1, end:429:2))"]
#[derive(Clone, Debug)]
struct STC_instructionVar33 {}
impl STC_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:434:1, end:434:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar34 {}
impl NOP_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NOP"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:437:1, end:437:2))"]
#[derive(Clone, Debug)]
struct HALT_instructionVar35 {}
impl HALT_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("HALT"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:441:1, end:441:2))"]
#[derive(Clone, Debug)]
struct DI_instructionVar36 {}
impl DI_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:447:1, end:447:2))"]
#[derive(Clone, Debug)]
struct EI_instructionVar37 {}
impl EI_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct RIM_instructionVar38 {}
impl RIM_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RIM"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:457:1, end:457:2))"]
#[derive(Clone, Debug)]
struct SIM_instructionVar39 {}
impl SIM_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SIM"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:474:1, end:474:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar40 {}
impl RLC_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:480:1, end:480:2))"]
#[derive(Clone, Debug)]
struct RAL_instructionVar41 {}
impl RAL_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RAL"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:487:1, end:487:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar42 {}
impl RRC_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:493:1, end:493:2))"]
#[derive(Clone, Debug)]
struct RAR_instructionVar43 {}
impl RAR_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RAR"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:500:1, end:500:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar44 {
    Addr16: TableAddr16,
}
impl JMP_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Addr16 = if let Some((len, table)) =
            TableAddr16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:508:1, end:508:2))"]
#[derive(Clone, Debug)]
struct PCHL_instructionVar45 {}
impl PCHL_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PCHL"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:512:1, end:512:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar46 {
    Addr16: TableAddr16,
}
impl CALL_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CALL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Addr16 = if let Some((len, table)) =
            TableAddr16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:525:1, end:525:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar47 {}
impl RET_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RET"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:544:1, end:544:2))"]
#[derive(Clone, Debug)]
struct IN_instructionVar48 {
    IOAddr8: TableIOAddr8,
}
impl IN_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IN"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IOAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:548:1, end:548:2))"]
#[derive(Clone, Debug)]
struct OUT_instructionVar49 {
    IOAddr8: TableIOAddr8,
}
impl OUT_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OUT"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IOAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:184:1, end:184:2))"]
#[derive(Clone, Debug)]
struct LXI_instructionVar50 {
    dRegPair4_2: u8,
    imm16: u16,
}
impl LXI_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LXI"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dRegPair4_2),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.imm16 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let dRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_8(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { dRegPair4_2, imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:200:1, end:200:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar51 {
    qRegPair4_2: u8,
}
impl PUSH_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PUSH"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.qRegPair4_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let qRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:204:1, end:204:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar52 {
    qRegPair4_2: u8,
}
impl POP_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("POP"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.qRegPair4_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let qRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct DAD_instructionVar53 {
    sRegPair4_2: u8,
}
impl DAD_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DAD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::HL),
            <DisplayElement>::Literal(","),
            meaning_1_display(self.sRegPair4_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let sRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:466:1, end:466:2))"]
#[derive(Clone, Debug)]
struct INX_instructionVar54 {
    sRegPair4_2: u8,
}
impl INX_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INX"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.sRegPair4_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let sRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:470:1, end:470:2))"]
#[derive(Clone, Debug)]
struct DCX_instructionVar55 {
    sRegPair4_2: u8,
}
impl DCX_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DCX"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.sRegPair4_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let sRegPair4_2 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:137:1, end:137:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar56 {
    reg3_3: u8,
    imm8: u8,
}
impl MVI_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVI"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
            DisplayElement::Number(true, false, self.imm8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:141:1, end:141:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar57 {
    reg3_3: u8,
}
impl MOV_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(",("),
            <DisplayElement>::Register(Register::HL),
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
        let mut block_0_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:146:1, end:146:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar58 {
    reg0_3: u8,
}
impl MOV_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::HL),
            <DisplayElement>::Literal("),"),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:220:1, end:220:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar59 {
    reg0_3: u8,
}
impl ADD_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:239:1, end:239:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar60 {
    reg0_3: u8,
}
impl ADC_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar61 {
    reg0_3: u8,
}
impl SUB_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:277:1, end:277:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar62 {
    reg0_3: u8,
}
impl SBB_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct ANA_instructionVar63 {
    reg0_3: u8,
}
impl ANA_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANA"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:320:1, end:320:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar64 {
    reg0_3: u8,
}
impl ORA_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ORA"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:344:1, end:344:2))"]
#[derive(Clone, Debug)]
struct XRA_instructionVar65 {
    reg0_3: u8,
}
impl XRA_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XRA"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:368:1, end:368:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar66 {
    reg0_3: u8,
}
impl CMP_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct INR_instructionVar67 {
    reg3_3: u8,
}
impl INR_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:401:1, end:401:2))"]
#[derive(Clone, Debug)]
struct DCR_instructionVar68 {
    reg3_3: u8,
}
impl DCR_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DCR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:504:1, end:504:2))"]
#[derive(Clone, Debug)]
struct J_instructionVar69 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl J_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("J"));
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let cc = if let Some((len, table)) =
            Tablecc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Addr16 = if let Some((len, table)) =
            TableAddr16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc, Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:518:1, end:518:2))"]
#[derive(Clone, Debug)]
struct C_instructionVar70 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl C_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("C"));
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr16
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let cc = if let Some((len, table)) =
            Tablecc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Addr16 = if let Some((len, table)) =
            TableAddr16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc, Addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:531:1, end:531:2))"]
#[derive(Clone, Debug)]
struct R_instructionVar71 {
    cc: Tablecc,
}
impl R_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("R"));
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let cc = if let Some((len, table)) =
            Tablecc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:538:1, end:538:2))"]
#[derive(Clone, Debug)]
struct RST_instructionVar72 {
    RstAddr: TableRstAddr,
}
impl RST_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RST"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RstAddr
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let RstAddr = if let Some((len, table)) =
            TableRstAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RstAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:133:1, end:133:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar73 {
    reg3_3: u8,
    reg0_3: u8,
}
impl MOV_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
            meaning_0_display(self.reg0_3),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let reg0_3 = token_6(tokens_current);
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3, reg0_3 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(MVI_instructionVar0),
    Var1(LDAX_instructionVar1),
    Var2(LDAX_instructionVar2),
    Var3(LDA_instructionVar3),
    Var4(STAX_instructionVar4),
    Var5(STAX_instructionVar5),
    Var6(STA_instructionVar6),
    Var7(LHLD_instructionVar7),
    Var8(SHLD_instructionVar8),
    Var9(SPHL_instructionVar9),
    Var10(XCHG_instructionVar10),
    Var11(XTHL_instructionVar11),
    Var12(ADI_instructionVar12),
    Var13(ADD_instructionVar13),
    Var14(ACI_instructionVar14),
    Var15(ADC_instructionVar15),
    Var16(SUI_instructionVar16),
    Var17(SUB_instructionVar17),
    Var18(SBI_instructionVar18),
    Var19(SBB_instructionVar19),
    Var20(ANI_instructionVar20),
    Var21(ANA_instructionVar21),
    Var22(ORI_instructionVar22),
    Var23(ORA_instructionVar23),
    Var24(XRA_instructionVar24),
    Var25(XRI_instructionVar25),
    Var26(CPI_instructionVar26),
    Var27(CMP_instructionVar27),
    Var28(INR_instructionVar28),
    Var29(DCR_instructionVar29),
    Var30(DAA_instructionVar30),
    Var31(CMA_instructionVar31),
    Var32(CMC_instructionVar32),
    Var33(STC_instructionVar33),
    Var34(NOP_instructionVar34),
    Var35(HALT_instructionVar35),
    Var36(DI_instructionVar36),
    Var37(EI_instructionVar37),
    Var38(RIM_instructionVar38),
    Var39(SIM_instructionVar39),
    Var40(RLC_instructionVar40),
    Var41(RAL_instructionVar41),
    Var42(RRC_instructionVar42),
    Var43(RAR_instructionVar43),
    Var44(JMP_instructionVar44),
    Var45(PCHL_instructionVar45),
    Var46(CALL_instructionVar46),
    Var47(RET_instructionVar47),
    Var48(IN_instructionVar48),
    Var49(OUT_instructionVar49),
    Var50(LXI_instructionVar50),
    Var51(PUSH_instructionVar51),
    Var52(POP_instructionVar52),
    Var53(DAD_instructionVar53),
    Var54(INX_instructionVar54),
    Var55(DCX_instructionVar55),
    Var56(MVI_instructionVar56),
    Var57(MOV_instructionVar57),
    Var58(MOV_instructionVar58),
    Var59(ADD_instructionVar59),
    Var60(ADC_instructionVar60),
    Var61(SUB_instructionVar61),
    Var62(SBB_instructionVar62),
    Var63(ANA_instructionVar63),
    Var64(ORA_instructionVar64),
    Var65(XRA_instructionVar65),
    Var66(CMP_instructionVar66),
    Var67(INR_instructionVar67),
    Var68(DCR_instructionVar68),
    Var69(J_instructionVar69),
    Var70(C_instructionVar70),
    Var71(R_instructionVar71),
    Var72(RST_instructionVar72),
    Var73(MOV_instructionVar73),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 54 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 10 {
            if let Some((inst_len, parsed)) =
                LDAX_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 26 {
            if let Some((inst_len, parsed)) =
                LDAX_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 58 {
            if let Some((inst_len, parsed)) =
                LDA_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                STAX_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 18 {
            if let Some((inst_len, parsed)) =
                STAX_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 50 {
            if let Some((inst_len, parsed)) =
                STA_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 42 {
            if let Some((inst_len, parsed)) =
                LHLD_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 34 {
            if let Some((inst_len, parsed)) =
                SHLD_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 249 {
            if let Some((inst_len, parsed)) =
                SPHL_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 235 {
            if let Some((inst_len, parsed)) =
                XCHG_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 227 {
            if let Some((inst_len, parsed)) =
                XTHL_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) =
                ADI_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 134 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) =
                ACI_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 142 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 214 {
            if let Some((inst_len, parsed)) =
                SUI_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 150 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 222 {
            if let Some((inst_len, parsed)) =
                SBI_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 158 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 230 {
            if let Some((inst_len, parsed)) =
                ANI_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 166 {
            if let Some((inst_len, parsed)) =
                ANA_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 246 {
            if let Some((inst_len, parsed)) =
                ORI_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 182 {
            if let Some((inst_len, parsed)) =
                ORA_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 174 {
            if let Some((inst_len, parsed)) =
                XRA_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 238 {
            if let Some((inst_len, parsed)) =
                XRI_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 254 {
            if let Some((inst_len, parsed)) =
                CPI_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 190 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 52 {
            if let Some((inst_len, parsed)) =
                INR_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 53 {
            if let Some((inst_len, parsed)) =
                DCR_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 39 {
            if let Some((inst_len, parsed)) =
                DAA_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 47 {
            if let Some((inst_len, parsed)) =
                CMA_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 63 {
            if let Some((inst_len, parsed)) =
                CMC_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 55 {
            if let Some((inst_len, parsed)) =
                STC_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 118 {
            if let Some((inst_len, parsed)) =
                HALT_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 243 {
            if let Some((inst_len, parsed)) =
                DI_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 251 {
            if let Some((inst_len, parsed)) =
                EI_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 32 {
            if let Some((inst_len, parsed)) =
                RIM_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 48 {
            if let Some((inst_len, parsed)) =
                SIM_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 23 {
            if let Some((inst_len, parsed)) =
                RAL_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 15 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 31 {
            if let Some((inst_len, parsed)) =
                RAR_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) =
                JMP_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 233 {
            if let Some((inst_len, parsed)) =
                PCHL_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                CALL_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) =
                RET_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 219 {
            if let Some((inst_len, parsed)) =
                IN_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 211 {
            if let Some((inst_len, parsed)) =
                OUT_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 207) == 1 {
            if let Some((inst_len, parsed)) =
                LXI_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 197 {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 193 {
            if let Some((inst_len, parsed)) =
                POP_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 9 {
            if let Some((inst_len, parsed)) =
                DAD_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 3 {
            if let Some((inst_len, parsed)) =
                INX_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 11 {
            if let Some((inst_len, parsed)) =
                DCX_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 199) == 6 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 70 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 112 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 128 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 136 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 144 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 152 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 160 {
            if let Some((inst_len, parsed)) =
                ANA_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 176 {
            if let Some((inst_len, parsed)) =
                ORA_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 168 {
            if let Some((inst_len, parsed)) =
                XRA_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 184 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 4 {
            if let Some((inst_len, parsed)) =
                INR_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 5 {
            if let Some((inst_len, parsed)) =
                DCR_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 199) == 194 {
            if let Some((inst_len, parsed)) =
                J_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 199) == 196 {
            if let Some((inst_len, parsed)) =
                C_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 192 {
            if let Some((inst_len, parsed)) =
                R_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 199 {
            if let Some((inst_len, parsed)) =
                RST_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 192) == 64 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:113:1, end:113:5))"]
#[derive(Clone, Debug)]
struct Mem8Var0 {
    imm16: u16,
}
impl Mem8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("("),
            DisplayElement::Number(true, false, self.imm16 as u64),
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
        let imm16 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableMem8 {
    Var0(Mem8Var0),
}
impl TableMem8 {
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
                Mem8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:114:1, end:114:6))"]
#[derive(Clone, Debug)]
struct Mem16Var0 {
    imm16: u16,
}
impl Mem16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("("),
            DisplayElement::Number(true, false, self.imm16 as u64),
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
        let imm16 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableMem16 {
    Var0(Mem16Var0),
}
impl TableMem16 {
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
                Mem16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:116:1, end:116:7))"]
#[derive(Clone, Debug)]
struct Addr16Var0 {
    imm16: u16,
}
impl Addr16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.imm16 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let imm16 = token_8(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr16 {
    Var0(Addr16Var0),
}
impl TableAddr16 {
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
                Addr16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:118:1, end:118:8))"]
#[derive(Clone, Debug)]
struct RstAddrVar0 {
    bits3_3: u8,
}
impl RstAddrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_loc: i128 = 0;
        calc_loc = u32::try_from(3i128)
            .ok()
            .and_then(|shl| i128::try_from(self.bits3_3).unwrap().checked_shl(shl))
            .unwrap_or(0);
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
        let mut block_0_len = 1;
        calc_loc = u32::try_from(3i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_4(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0);
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[derive(Clone, Debug)]
enum TableRstAddr {
    Var0(RstAddrVar0),
}
impl TableRstAddr {
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
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                RstAddrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:120:1, end:120:8))"]
#[derive(Clone, Debug)]
struct IOAddr8Var0 {
    imm8: u8,
}
impl IOAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("("),
            DisplayElement::Number(true, false, self.imm8 as u64),
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
        let mut block_0_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableIOAddr8 {
    Var0(IOAddr8Var0),
}
impl TableIOAddr8 {
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
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                IOAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:122:1, end:122:3))"]
#[derive(Clone, Debug)]
struct ccVar0 {}
impl ccVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NZ")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:123:1, end:123:3))"]
#[derive(Clone, Debug)]
struct ccVar1 {}
impl ccVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("Z")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:124:1, end:124:3))"]
#[derive(Clone, Debug)]
struct ccVar2 {}
impl ccVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NC")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:125:1, end:125:3))"]
#[derive(Clone, Debug)]
struct ccVar3 {}
impl ccVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("C")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:126:1, end:126:3))"]
#[derive(Clone, Debug)]
struct ccVar4 {}
impl ccVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PO")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:127:1, end:127:3))"]
#[derive(Clone, Debug)]
struct ccVar5 {}
impl ccVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PE")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:128:1, end:128:3))"]
#[derive(Clone, Debug)]
struct ccVar6 {}
impl ccVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("P")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/8085/data/languages/8085.slaspec, start:129:1, end:129:3))"]
#[derive(Clone, Debug)]
struct ccVar7 {}
impl ccVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("M")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecc {
    Var0(ccVar0),
    Var1(ccVar1),
    Var2(ccVar2),
    Var3(ccVar3),
    Var4(ccVar4),
    Var5(ccVar5),
    Var6(ccVar6),
    Var7(ccVar7),
}
impl Tablecc {
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
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 0 {
            if let Some((inst_len, parsed)) =
                ccVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 8 {
            if let Some((inst_len, parsed)) =
                ccVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 16 {
            if let Some((inst_len, parsed)) =
                ccVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 24 {
            if let Some((inst_len, parsed)) =
                ccVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 32 {
            if let Some((inst_len, parsed)) =
                ccVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 40 {
            if let Some((inst_len, parsed)) =
                ccVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 48 {
            if let Some((inst_len, parsed)) =
                ccVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 56 {
            if let Some((inst_len, parsed)) =
                ccVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
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
