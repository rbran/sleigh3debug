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
    I,
    R,
    AF,
    BC,
    DE,
    HL,
    F_,
    A_,
    C_,
    B_,
    E_,
    D_,
    L_,
    H_,
    AF_,
    BC_,
    DE_,
    HL_,
    PC,
    SP,
    IX,
    IY,
    rCBAR,
    rCBR,
    rBBR,
    DECOMPILE_MODE,
    contextreg,
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
            Self::I => "I",
            Self::R => "R",
            Self::AF => "AF",
            Self::BC => "BC",
            Self::DE => "DE",
            Self::HL => "HL",
            Self::F_ => "F_",
            Self::A_ => "A_",
            Self::C_ => "C_",
            Self::B_ => "B_",
            Self::E_ => "E_",
            Self::D_ => "D_",
            Self::L_ => "L_",
            Self::H_ => "H_",
            Self::AF_ => "AF_",
            Self::BC_ => "BC_",
            Self::DE_ => "DE_",
            Self::HL_ => "HL_",
            Self::PC => "PC",
            Self::SP => "SP",
            Self::IX => "IX",
            Self::IY => "IY",
            Self::rCBAR => "rCBAR",
            Self::rCBR => "rCBR",
            Self::rBBR => "rBBR",
            Self::DECOMPILE_MODE => "DECOMPILE_MODE",
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
fn meaning_3_value<T>(num: T) -> Register
where
    u8: TryFrom<T>,
    <u8 as TryFrom<T>>::Error: core::fmt::Debug,
{
    match u8::try_from(num).unwrap() {
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::IX,
        3 => Register::SP,
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
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::IY,
        3 => Register::SP,
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
#[doc = "Create token_fields: reg3_3 bits3_3"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 3) & 7) as u8)
}
#[doc = "Create token_fields: imm16 simm16"]
fn token_9(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: op6_2"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 6) & 3) as u8)
}
#[doc = "Create token_fields: dRegPair4_2 pRegPair4_2 sRegPair4_2 qRegPair4_2 rRegPair4_2"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 3) as u8)
}
#[doc = "Create token_fields: op0_8 imm8 simm8"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: bits0_4"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: sign16"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 15) & 1) as u8)
}
#[doc = "Create token_fields: timm4"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: sign8"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 7) & 1) as u8)
}
#[doc = "Create token_fields: reg0_3 bits0_3"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u8::from_le_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 7) as u8)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u8);
impl ContextMemory {
    pub fn read_assume8bitIOSpace(&self) -> u8 {
        (((self.0.reverse_bits() >> 7) & 1) as u8)
    }
    pub fn write_assume8bitIOSpace(&mut self, value: u8) {
        self.0 = ((self.0.reverse_bits() & !(1 << 7)) | ((value as u8 & 1) << 7)).reverse_bits();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1360:1, end:1360:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar0 {
    ixMem8: TableixMem8,
}
impl RLC_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1370:1, end:1370:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar1 {
    iyMem8: TableiyMem8,
}
impl RLC_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1404:1, end:1404:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar2 {
    ixMem8: TableixMem8,
}
impl RL_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1416:1, end:1416:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar3 {
    iyMem8: TableiyMem8,
}
impl RL_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1449:1, end:1449:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar4 {
    ixMem8: TableixMem8,
}
impl RRC_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1460:1, end:1460:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar5 {
    iyMem8: TableiyMem8,
}
impl RRC_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1494:1, end:1494:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar6 {
    ixMem8: TableixMem8,
}
impl RR_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1506:1, end:1506:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar7 {
    iyMem8: TableiyMem8,
}
impl RR_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1539:1, end:1539:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar8 {
    ixMem8: TableixMem8,
}
impl SLA_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1550:1, end:1550:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar9 {
    iyMem8: TableiyMem8,
}
impl SLA_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1582:1, end:1582:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar10 {
    ixMem8: TableixMem8,
}
impl SRA_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1593:1, end:1593:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar11 {
    iyMem8: TableiyMem8,
}
impl SRA_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1625:1, end:1625:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar12 {
    ixMem8: TableixMem8,
}
impl SRL_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1636:1, end:1636:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar13 {
    iyMem8: TableiyMem8,
}
impl SRL_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1699:1, end:1699:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar14 {
    bits3_3: u8,
    ixMem8: TableixMem8,
}
impl BIT_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIT"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1707:1, end:1707:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar15 {
    bits3_3: u8,
    iyMem8: TableiyMem8,
}
impl BIT_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIT"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1728:1, end:1728:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar16 {
    bits3_3: u8,
    ixMem8: TableixMem8,
}
impl SET_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SET"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1734:1, end:1734:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar17 {
    bits3_3: u8,
    iyMem8: TableiyMem8,
}
impl SET_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SET"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1753:1, end:1753:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar18 {
    bits3_3: u8,
    ixMem8: TableixMem8,
}
impl RES_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RES"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1759:1, end:1759:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar19 {
    bits3_3: u8,
    iyMem8: TableiyMem8,
}
impl RES_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RES"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:388:1, end:388:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar20 {
    reg3_3: u8,
    ixMem8: TableixMem8,
}
impl LD_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        if token_4(tokens_current) == 6 {
            return None;
        }
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:392:1, end:392:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar21 {
    reg3_3: u8,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        if token_4(tokens_current) == 6 {
            return None;
        }
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:400:1, end:400:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar22 {
    reg0_3: u8,
    ixMem8: TableixMem8,
}
impl LD_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        if token_6(tokens_current) == 6 {
            return None;
        }
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar23 {
    reg0_3: u8,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        if token_6(tokens_current) == 6 {
            return None;
        }
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:413:1, end:413:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar24 {
    imm8: u8,
    ixMem8: TableixMem8,
}
impl LD_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:417:1, end:417:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar25 {
    imm8: u8,
    iyMem8: TableiyMem8,
}
impl LD_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8, imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:445:1, end:445:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar26 {}
impl LD_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::I),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar27 {}
impl LD_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::R),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar28 {}
impl LD_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::I),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::A),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:465:1, end:465:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar29 {}
impl LD_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::R),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::A),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:473:1, end:473:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar30 {
    imm16: u16,
}
impl LD_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IX),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let imm16 = token_9(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:477:1, end:477:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar31 {
    imm16: u16,
}
impl LD_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IY),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let imm16 = token_9(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:489:1, end:489:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar32 {
    Mem16: TableMem16,
}
impl LD_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IX),
            <DisplayElement>::Literal(","),
        ];
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:493:1, end:493:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar33 {
    Mem16: TableMem16,
}
impl LD_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IY),
            <DisplayElement>::Literal(","),
        ];
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:505:1, end:505:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar34 {
    Mem16: TableMem16,
}
impl LD_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:509:1, end:509:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar35 {
    Mem16: TableMem16,
}
impl LD_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:517:1, end:517:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar36 {}
impl LD_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:521:1, end:521:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar37 {}
impl LD_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:529:1, end:529:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar38 {}
impl PUSH_instructionVar38 {
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
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:533:1, end:533:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar39 {}
impl PUSH_instructionVar39 {
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
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar40 {}
impl POP_instructionVar40 {
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
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:546:1, end:546:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar41 {}
impl POP_instructionVar41 {
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
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:578:1, end:578:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar42 {}
impl EX_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EX"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:582:1, end:582:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar43 {}
impl EX_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EX"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct LDI_instructionVar44 {}
impl LDI_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDI"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:599:1, end:599:2))"]
#[derive(Clone, Debug)]
struct LDIR_instructionVar45 {}
impl LDIR_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDIR"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:612:1, end:612:2))"]
#[derive(Clone, Debug)]
struct LDD_instructionVar46 {}
impl LDD_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDD"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:621:1, end:621:2))"]
#[derive(Clone, Debug)]
struct LDDR_instructionVar47 {}
impl LDDR_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDDR"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:634:1, end:634:2))"]
#[derive(Clone, Debug)]
struct CPI_instructionVar48 {}
impl CPI_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPI"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct CPIR_instructionVar49 {}
impl CPIR_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPIR"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:664:1, end:664:2))"]
#[derive(Clone, Debug)]
struct CPD_instructionVar50 {}
impl CPD_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPD"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:678:1, end:678:2))"]
#[derive(Clone, Debug)]
struct CPDR_instructionVar51 {}
impl CPDR_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPDR"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:721:1, end:721:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar52 {
    ixMem8: TableixMem8,
}
impl ADD_instructionVar52 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:730:1, end:730:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar53 {
    iyMem8: TableiyMem8,
}
impl ADD_instructionVar53 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:756:1, end:756:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar54 {
    ixMem8: TableixMem8,
}
impl ADC_instructionVar54 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:762:1, end:762:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar55 {
    iyMem8: TableiyMem8,
}
impl ADC_instructionVar55 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:794:1, end:794:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar56 {
    ixMem8: TableixMem8,
}
impl SUB_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:803:1, end:803:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar57 {
    iyMem8: TableiyMem8,
}
impl SUB_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:829:1, end:829:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar58 {
    ixMem8: TableixMem8,
}
impl SBC_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:835:1, end:835:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar59 {
    iyMem8: TableiyMem8,
}
impl SBC_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:870:1, end:870:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar60 {
    ixMem8: TableixMem8,
}
impl AND_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:879:1, end:879:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar61 {
    iyMem8: TableiyMem8,
}
impl AND_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:917:1, end:917:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar62 {
    ixMem8: TableixMem8,
}
impl OR_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:926:1, end:926:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar63 {
    iyMem8: TableiyMem8,
}
impl OR_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:964:1, end:964:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar64 {
    ixMem8: TableixMem8,
}
impl XOR_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:973:1, end:973:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar65 {
    iyMem8: TableiyMem8,
}
impl XOR_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar66 {
    ixMem8: TableixMem8,
}
impl CP_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1011:1, end:1011:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar67 {
    iyMem8: TableiyMem8,
}
impl CP_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1038:1, end:1038:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar68 {
    ixMem8: TableixMem8,
}
impl INC_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1048:1, end:1048:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar69 {
    iyMem8: TableiyMem8,
}
impl INC_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1077:1, end:1077:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar70 {
    ixMem8: TableixMem8,
}
impl DEC_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ixMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let ixMem8 = if let Some((len, table)) =
            TableixMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ixMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1087:1, end:1087:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar71 {
    iyMem8: TableiyMem8,
}
impl DEC_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.iyMem8
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let iyMem8 = if let Some((len, table)) =
            TableiyMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { iyMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1197:1, end:1197:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar72 {}
impl NEG_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEG"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1235:1, end:1235:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar73 {}
impl IM_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IM"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("0"),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1239:1, end:1239:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar74 {}
impl IM_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IM"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("1"),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1243:1, end:1243:2))"]
#[derive(Clone, Debug)]
struct IM_instructionVar75 {}
impl IM_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IM"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("2"),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1289:1, end:1289:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar76 {}
impl INC_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1293:1, end:1293:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar77 {}
impl INC_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1301:1, end:1301:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar78 {}
impl DEC_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IX),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1305:1, end:1305:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar79 {}
impl DEC_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::IY),
        ];
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1348:1, end:1348:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar80 {}
impl RLC_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1391:1, end:1391:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar81 {}
impl RL_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RL"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1437:1, end:1437:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar82 {}
impl RRC_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1481:1, end:1481:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar83 {}
impl RR_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RR"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1527:1, end:1527:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar84 {}
impl SLA_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLA"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1570:1, end:1570:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar85 {}
impl SRA_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRA"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1613:1, end:1613:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar86 {}
impl SRL_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRL"));
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1647:1, end:1647:2))"]
#[derive(Clone, Debug)]
struct RLD_instructionVar87 {}
impl RLD_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLD"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct RRD_instructionVar88 {}
impl RRD_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRD"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1788:1, end:1788:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar89 {}
impl JP_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JP"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IX),
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1793:1, end:1793:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar90 {}
impl JP_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JP"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IY),
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
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1827:1, end:1827:2))"]
#[derive(Clone, Debug)]
struct RETI_instructionVar91 {}
impl RETI_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RETI"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct RETN_instructionVar92 {}
impl RETN_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RETN"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1857:1, end:1857:2))"]
#[derive(Clone, Debug)]
struct INI_instructionVar93 {
    IOAddrC: TableIOAddrC,
}
impl INI_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1867:1, end:1867:2))"]
#[derive(Clone, Debug)]
struct INIR_instructionVar94 {
    IOAddrC: TableIOAddrC,
}
impl INIR_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INIR"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1878:1, end:1878:2))"]
#[derive(Clone, Debug)]
struct IND_instructionVar95 {
    IOAddrC: TableIOAddrC,
}
impl IND_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IND"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1888:1, end:1888:2))"]
#[derive(Clone, Debug)]
struct INDR_instructionVar96 {
    IOAddrC: TableIOAddrC,
}
impl INDR_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INDR"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1907:1, end:1907:2))"]
#[derive(Clone, Debug)]
struct OUTI_instructionVar97 {
    IOAddrC: TableIOAddrC,
}
impl OUTI_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OUTI"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1917:1, end:1917:2))"]
#[derive(Clone, Debug)]
struct OTIR_instructionVar98 {
    IOAddrC: TableIOAddrC,
}
impl OTIR_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTIR"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1928:1, end:1928:2))"]
#[derive(Clone, Debug)]
struct OUTD_instructionVar99 {
    IOAddrC: TableIOAddrC,
}
impl OUTD_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OUTD"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct OTDR_instructionVar100 {
    IOAddrC: TableIOAddrC,
}
impl OTDR_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTDR"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1965:1, end:1965:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar101 {
    hlMem8: TablehlMem8,
}
impl TST_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TST"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.hlMem8
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
        let hlMem8 = if let Some((len, table)) =
            TablehlMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { hlMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1976:1, end:1976:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar102 {
    imm8: u8,
}
impl TST_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TST"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let imm8 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1985:1, end:1985:2))"]
#[derive(Clone, Debug)]
struct IN0_instructionVar103 {
    Flag: TableFlag,
    IOAddr8: TableIOAddr8,
}
impl IN0_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IN0"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Flag
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
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
        let Flag = if let Some((len, table)) =
            TableFlag::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Flag, IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2006:1, end:2006:2))"]
#[derive(Clone, Debug)]
struct OTDM_instructionVar104 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTDM_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTDM"));
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
        let hlMem8 = if let Some((len, table)) =
            TablehlMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2018:1, end:2018:2))"]
#[derive(Clone, Debug)]
struct OTDMR_instructionVar105 {
    IOAddrC: TableIOAddrC,
    hlMem8: TablehlMem8,
}
impl OTDMR_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTDMR"));
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
        let hlMem8 = if let Some((len, table)) =
            TablehlMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, hlMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2033:1, end:2033:2))"]
#[derive(Clone, Debug)]
struct TSTIO_instructionVar106 {
    IOAddr8: TableIOAddr8,
}
impl TSTIO_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TSTIO"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2044:1, end:2044:2))"]
#[derive(Clone, Debug)]
struct OTIM_instructionVar107 {
    hlMem8: TablehlMem8,
    IOAddrC: TableIOAddrC,
}
impl OTIM_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTIM"));
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
        let hlMem8 = if let Some((len, table)) =
            TablehlMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { hlMem8, IOAddrC }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2059:1, end:2059:2))"]
#[derive(Clone, Debug)]
struct OTIMR_instructionVar108 {
    IOAddrC: TableIOAddrC,
    hlMem8: TablehlMem8,
}
impl OTIMR_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OTIMR"));
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
        let hlMem8 = if let Some((len, table)) =
            TablehlMem8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, hlMem8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2074:1, end:2074:2))"]
#[derive(Clone, Debug)]
struct SLP_instructionVar109 {}
impl SLP_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLP"));
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
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:485:1, end:485:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar110 {
    dRegPair4_2: u8,
    Mem16: TableMem16,
}
impl LD_instructionVar110 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.dRegPair4_2),
            <DisplayElement>::Literal(","),
        ];
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
        let mut block_1_len = 1;
        let dRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16, dRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:501:1, end:501:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar111 {
    dRegPair4_2: u8,
    Mem16: TableMem16,
}
impl LD_instructionVar111 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            meaning_1_display(self.dRegPair4_2),
        ];
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
        let dRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 2;
        let Mem16 = if let Some((len, table)) =
            TableMem16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Mem16, dRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1255:1, end:1255:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar112 {
    sRegPair4_2: u8,
}
impl ADC_instructionVar112 {
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let sRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1264:1, end:1264:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar113 {
    sRegPair4_2: u8,
}
impl SBC_instructionVar113 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let sRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1269:1, end:1269:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar114 {
    pRegPair4_2: u8,
}
impl ADD_instructionVar114 {
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
            <DisplayElement>::Register(Register::IX),
            <DisplayElement>::Literal(","),
            meaning_3_display(self.pRegPair4_2),
        ];
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
        let pRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { pRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1277:1, end:1277:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar115 {
    pRegPair4_2: u8,
}
impl ADD_instructionVar115 {
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
            <DisplayElement>::Register(Register::IY),
            <DisplayElement>::Literal(","),
            meaning_3_display(self.pRegPair4_2),
        ];
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
        let pRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { pRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1339:1, end:1339:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar116 {
    reg0_3: u8,
}
impl RLC_instructionVar116 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1381:1, end:1381:2))"]
#[derive(Clone, Debug)]
struct RL_instructionVar117 {
    reg0_3: u8,
}
impl RL_instructionVar117 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RL"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1428:1, end:1428:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar118 {
    reg0_3: u8,
}
impl RRC_instructionVar118 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1471:1, end:1471:2))"]
#[derive(Clone, Debug)]
struct RR_instructionVar119 {
    reg0_3: u8,
}
impl RR_instructionVar119 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RR"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1518:1, end:1518:2))"]
#[derive(Clone, Debug)]
struct SLA_instructionVar120 {
    reg0_3: u8,
}
impl SLA_instructionVar120 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLA"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1561:1, end:1561:2))"]
#[derive(Clone, Debug)]
struct SRA_instructionVar121 {
    reg0_3: u8,
}
impl SRA_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRA"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1604:1, end:1604:2))"]
#[derive(Clone, Debug)]
struct SRL_instructionVar122 {
    reg0_3: u8,
}
impl SRL_instructionVar122 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SRL"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1950:1, end:1950:2))"]
#[derive(Clone, Debug)]
struct MLT_instructionVar123 {
    qRegPair4_2: u8,
}
impl MLT_instructionVar123 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MLT"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let qRegPair4_2 = token_3(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { qRegPair4_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1690:1, end:1690:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar124 {
    bits3_3: u8,
}
impl BIT_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIT"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1720:1, end:1720:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar125 {
    bits3_3: u8,
}
impl SET_instructionVar125 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SET"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1745:1, end:1745:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar126 {
    bits3_3: u8,
}
impl RES_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RES"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1849:1, end:1849:2))"]
#[derive(Clone, Debug)]
struct IN_instructionVar127 {
    reg3_3: u8,
    IOAddrC: TableIOAddrC,
}
impl IN_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IN"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.IOAddrC
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
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1903:1, end:1903:2))"]
#[derive(Clone, Debug)]
struct OUT_instructionVar128 {
    reg3_3: u8,
    IOAddrC: TableIOAddrC,
}
impl OUT_instructionVar128 {
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
        self.IOAddrC
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
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
        let IOAddrC = if let Some((len, table)) =
            TableIOAddrC::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddrC, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1956:1, end:1956:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar129 {
    reg3_3: u8,
}
impl TST_instructionVar129 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TST"));
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1994:1, end:1994:2))"]
#[derive(Clone, Debug)]
struct IN0_instructionVar130 {
    reg3_3: u8,
    IOAddr8: TableIOAddr8,
}
impl IN0_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IN0"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg3_3),
            <DisplayElement>::Literal(","),
        ];
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
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:2002:1, end:2002:2))"]
#[derive(Clone, Debug)]
struct OUT0_instructionVar131 {
    reg3_3: u8,
    IOAddr8: TableIOAddr8,
}
impl OUT0_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OUT0"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IOAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let reg3_3 = token_4(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let IOAddr8 = if let Some((len, table)) =
            TableIOAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8, reg3_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1683:1, end:1683:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar132 {
    bits3_3: u8,
    reg0_3: u8,
}
impl BIT_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIT"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1715:1, end:1715:2))"]
#[derive(Clone, Debug)]
struct SET_instructionVar133 {
    bits3_3: u8,
    reg0_3: u8,
}
impl SET_instructionVar133 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SET"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1740:1, end:1740:2))"]
#[derive(Clone, Debug)]
struct RES_instructionVar134 {
    bits3_3: u8,
    reg0_3: u8,
}
impl RES_instructionVar134 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RES"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.bits3_3 as u64),
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let bits3_3 = token_4(tokens_current);
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bits3_3, reg0_3 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:408:1, end:408:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar135 {
    imm8: u8,
}
impl LD_instructionVar135 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:421:1, end:421:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar136 {}
impl LD_instructionVar136 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(",("),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:425:1, end:425:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar137 {}
impl LD_instructionVar137 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(",("),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:429:1, end:429:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar138 {
    Mem8: TableMem8,
}
impl LD_instructionVar138 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:433:1, end:433:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar139 {}
impl LD_instructionVar139 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::BC),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::A),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:437:1, end:437:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar140 {}
impl LD_instructionVar140 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::DE),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::A),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:441:1, end:441:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar141 {
    Mem8: TableMem8,
}
impl LD_instructionVar141 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::A),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:481:1, end:481:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar142 {
    Mem16: TableMem16,
}
impl LD_instructionVar142 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::HL),
            <DisplayElement>::Literal(","),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:497:1, end:497:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar143 {
    Mem16: TableMem16,
}
impl LD_instructionVar143 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Mem16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::HL),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:513:1, end:513:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar144 {}
impl LD_instructionVar144 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::HL),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:550:1, end:550:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar145 {}
impl EX_instructionVar145 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EX"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::DE),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::HL),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:556:1, end:556:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar146 {}
impl EX_instructionVar146 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EX"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::AF),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::AF_),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:562:1, end:562:2))"]
#[derive(Clone, Debug)]
struct EXX_instructionVar147 {}
impl EXX_instructionVar147 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EXX"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:574:1, end:574:2))"]
#[derive(Clone, Debug)]
struct EX_instructionVar148 {}
impl EX_instructionVar148 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EX"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal("),"),
            <DisplayElement>::Register(Register::HL),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:703:1, end:703:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar149 {
    imm8: u8,
}
impl ADD_instructionVar149 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:711:1, end:711:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar150 {}
impl ADD_instructionVar150 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:744:1, end:744:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar151 {
    imm8: u8,
}
impl ADC_instructionVar151 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:749:1, end:749:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar152 {}
impl ADC_instructionVar152 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:776:1, end:776:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar153 {
    imm8: u8,
}
impl SUB_instructionVar153 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:784:1, end:784:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar154 {}
impl SUB_instructionVar154 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:817:1, end:817:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar155 {
    imm8: u8,
}
impl SBC_instructionVar155 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:822:1, end:822:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar156 {}
impl SBC_instructionVar156 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 7usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:850:1, end:850:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar157 {
    imm8: u8,
}
impl AND_instructionVar157 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:859:1, end:859:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar158 {}
impl AND_instructionVar158 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:897:1, end:897:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar159 {
    imm8: u8,
}
impl OR_instructionVar159 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:906:1, end:906:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar160 {}
impl OR_instructionVar160 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:944:1, end:944:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar161 {
    imm8: u8,
}
impl XOR_instructionVar161 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:953:1, end:953:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar162 {}
impl XOR_instructionVar162 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:988:1, end:988:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar163 {
    imm8: u8,
}
impl CP_instructionVar163 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:994:1, end:994:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar164 {}
impl CP_instructionVar164 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1027:1, end:1027:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar165 {}
impl INC_instructionVar165 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1066:1, end:1066:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar166 {}
impl DEC_instructionVar166 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1097:1, end:1097:2))"]
#[derive(Clone, Debug)]
struct DAA_instructionVar167 {}
impl DAA_instructionVar167 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1191:1, end:1191:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar168 {}
impl CPL_instructionVar168 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPL"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1205:1, end:1205:2))"]
#[derive(Clone, Debug)]
struct CCF_instructionVar169 {}
impl CCF_instructionVar169 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CCF"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1210:1, end:1210:2))"]
#[derive(Clone, Debug)]
struct SCF_instructionVar170 {}
impl SCF_instructionVar170 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SCF"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1216:1, end:1216:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar171 {}
impl NOP_instructionVar171 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1219:1, end:1219:2))"]
#[derive(Clone, Debug)]
struct HALT_instructionVar172 {}
impl HALT_instructionVar172 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1223:1, end:1223:2))"]
#[derive(Clone, Debug)]
struct DI_instructionVar173 {}
impl DI_instructionVar173 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1229:1, end:1229:2))"]
#[derive(Clone, Debug)]
struct EI_instructionVar174 {}
impl EI_instructionVar174 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1309:1, end:1309:2))"]
#[derive(Clone, Debug)]
struct RLCA_instructionVar175 {}
impl RLCA_instructionVar175 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLCA"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1316:1, end:1316:2))"]
#[derive(Clone, Debug)]
struct RLA_instructionVar176 {}
impl RLA_instructionVar176 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLA"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1324:1, end:1324:2))"]
#[derive(Clone, Debug)]
struct RRCA_instructionVar177 {}
impl RRCA_instructionVar177 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRCA"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1331:1, end:1331:2))"]
#[derive(Clone, Debug)]
struct RRA_instructionVar178 {}
impl RRA_instructionVar178 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRA"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1765:1, end:1765:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar179 {
    Addr16: TableAddr16,
}
impl JP_instructionVar179 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1773:1, end:1773:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar180 {
    RelAddr8: TableRelAddr8,
}
impl JR_instructionVar180 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr8
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
        let RelAddr8 = if let Some((len, table)) =
            TableRelAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1781:1, end:1781:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar181 {}
impl JP_instructionVar181 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1798:1, end:1798:2))"]
#[derive(Clone, Debug)]
struct DJNZ_instructionVar182 {
    RelAddr8: TableRelAddr8,
}
impl DJNZ_instructionVar182 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DJNZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr8
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
        let RelAddr8 = if let Some((len, table)) =
            TableRelAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1803:1, end:1803:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar183 {
    Addr16: TableAddr16,
}
impl CALL_instructionVar183 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1814:1, end:1814:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar184 {}
impl RET_instructionVar184 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1845:1, end:1845:2))"]
#[derive(Clone, Debug)]
struct IN_instructionVar185 {
    IOAddr8a: TableIOAddr8a,
}
impl IN_instructionVar185 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IN"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.IOAddr8a
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
        let IOAddr8a = if let Some((len, table)) =
            TableIOAddr8a::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1899:1, end:1899:2))"]
#[derive(Clone, Debug)]
struct OUT_instructionVar186 {
    IOAddr8a: TableIOAddr8a,
}
impl OUT_instructionVar186 {
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
        self.IOAddr8a
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Register(Register::A),
        ];
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
        let IOAddr8a = if let Some((len, table)) =
            TableIOAddr8a::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IOAddr8a }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:469:1, end:469:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar187 {
    dRegPair4_2: u8,
    imm16: u16,
}
impl LD_instructionVar187 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
        let imm16 = token_9(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { dRegPair4_2, imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:525:1, end:525:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar188 {
    qRegPair4_2: u8,
}
impl PUSH_instructionVar188 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:537:1, end:537:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar189 {
    qRegPair4_2: u8,
}
impl POP_instructionVar189 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1247:1, end:1247:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar190 {
    sRegPair4_2: u8,
}
impl ADD_instructionVar190 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1285:1, end:1285:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar191 {
    sRegPair4_2: u8,
}
impl INC_instructionVar191 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1297:1, end:1297:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar192 {
    sRegPair4_2: u8,
}
impl DEC_instructionVar192 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar193 {
    reg3_3: u8,
    imm8: u8,
}
impl LD_instructionVar193 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:384:1, end:384:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar194 {
    reg3_3: u8,
}
impl LD_instructionVar194 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:396:1, end:396:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar195 {
    reg0_3: u8,
}
impl LD_instructionVar195 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:694:1, end:694:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar196 {
    reg0_3: u8,
}
impl ADD_instructionVar196 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:739:1, end:739:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar197 {
    reg0_3: u8,
}
impl ADC_instructionVar197 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:768:1, end:768:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar198 {
    reg0_3: u8,
}
impl SUB_instructionVar198 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:812:1, end:812:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar199 {
    reg0_3: u8,
}
impl SBC_instructionVar199 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:841:1, end:841:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar200 {
    reg0_3: u8,
}
impl AND_instructionVar200 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:888:1, end:888:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar201 {
    reg0_3: u8,
}
impl OR_instructionVar201 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:935:1, end:935:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar202 {
    reg0_3: u8,
}
impl XOR_instructionVar202 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:982:1, end:982:2))"]
#[derive(Clone, Debug)]
struct CP_instructionVar203 {
    reg0_3: u8,
}
impl CP_instructionVar203 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1019:1, end:1019:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar204 {
    reg3_3: u8,
}
impl INC_instructionVar204 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1058:1, end:1058:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar205 {
    reg3_3: u8,
}
impl DEC_instructionVar205 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DEC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1769:1, end:1769:2))"]
#[derive(Clone, Debug)]
struct JP_instructionVar206 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl JP_instructionVar206 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1777:1, end:1777:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar207 {
    cc2: Tablecc2,
    RelAddr8: TableRelAddr8,
}
impl JR_instructionVar207 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.cc2
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.RelAddr8
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
        let cc2 = if let Some((len, table)) =
            Tablecc2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let RelAddr8 = if let Some((len, table)) =
            TableRelAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc2, RelAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1808:1, end:1808:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar208 {
    cc: Tablecc,
    Addr16: TableAddr16,
}
impl CALL_instructionVar208 {
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
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1820:1, end:1820:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar209 {
    cc: Tablecc,
}
impl RET_instructionVar209 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RET"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:1840:1, end:1840:2))"]
#[derive(Clone, Debug)]
struct RST_instructionVar210 {
    RstAddr: TableRstAddr,
}
impl RST_instructionVar210 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:376:1, end:376:2))"]
#[derive(Clone, Debug)]
struct LD_instructionVar211 {
    reg3_3: u8,
    reg0_3: u8,
}
impl LD_instructionVar211 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LD"));
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
        let reg3_3 = token_4(tokens_current);
        let reg0_3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_3, reg0_3 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(RLC_instructionVar0),
    Var1(RLC_instructionVar1),
    Var2(RL_instructionVar2),
    Var3(RL_instructionVar3),
    Var4(RRC_instructionVar4),
    Var5(RRC_instructionVar5),
    Var6(RR_instructionVar6),
    Var7(RR_instructionVar7),
    Var8(SLA_instructionVar8),
    Var9(SLA_instructionVar9),
    Var10(SRA_instructionVar10),
    Var11(SRA_instructionVar11),
    Var12(SRL_instructionVar12),
    Var13(SRL_instructionVar13),
    Var14(BIT_instructionVar14),
    Var15(BIT_instructionVar15),
    Var16(SET_instructionVar16),
    Var17(SET_instructionVar17),
    Var18(RES_instructionVar18),
    Var19(RES_instructionVar19),
    Var20(LD_instructionVar20),
    Var21(LD_instructionVar21),
    Var22(LD_instructionVar22),
    Var23(LD_instructionVar23),
    Var24(LD_instructionVar24),
    Var25(LD_instructionVar25),
    Var26(LD_instructionVar26),
    Var27(LD_instructionVar27),
    Var28(LD_instructionVar28),
    Var29(LD_instructionVar29),
    Var30(LD_instructionVar30),
    Var31(LD_instructionVar31),
    Var32(LD_instructionVar32),
    Var33(LD_instructionVar33),
    Var34(LD_instructionVar34),
    Var35(LD_instructionVar35),
    Var36(LD_instructionVar36),
    Var37(LD_instructionVar37),
    Var38(PUSH_instructionVar38),
    Var39(PUSH_instructionVar39),
    Var40(POP_instructionVar40),
    Var41(POP_instructionVar41),
    Var42(EX_instructionVar42),
    Var43(EX_instructionVar43),
    Var44(LDI_instructionVar44),
    Var45(LDIR_instructionVar45),
    Var46(LDD_instructionVar46),
    Var47(LDDR_instructionVar47),
    Var48(CPI_instructionVar48),
    Var49(CPIR_instructionVar49),
    Var50(CPD_instructionVar50),
    Var51(CPDR_instructionVar51),
    Var52(ADD_instructionVar52),
    Var53(ADD_instructionVar53),
    Var54(ADC_instructionVar54),
    Var55(ADC_instructionVar55),
    Var56(SUB_instructionVar56),
    Var57(SUB_instructionVar57),
    Var58(SBC_instructionVar58),
    Var59(SBC_instructionVar59),
    Var60(AND_instructionVar60),
    Var61(AND_instructionVar61),
    Var62(OR_instructionVar62),
    Var63(OR_instructionVar63),
    Var64(XOR_instructionVar64),
    Var65(XOR_instructionVar65),
    Var66(CP_instructionVar66),
    Var67(CP_instructionVar67),
    Var68(INC_instructionVar68),
    Var69(INC_instructionVar69),
    Var70(DEC_instructionVar70),
    Var71(DEC_instructionVar71),
    Var72(NEG_instructionVar72),
    Var73(IM_instructionVar73),
    Var74(IM_instructionVar74),
    Var75(IM_instructionVar75),
    Var76(INC_instructionVar76),
    Var77(INC_instructionVar77),
    Var78(DEC_instructionVar78),
    Var79(DEC_instructionVar79),
    Var80(RLC_instructionVar80),
    Var81(RL_instructionVar81),
    Var82(RRC_instructionVar82),
    Var83(RR_instructionVar83),
    Var84(SLA_instructionVar84),
    Var85(SRA_instructionVar85),
    Var86(SRL_instructionVar86),
    Var87(RLD_instructionVar87),
    Var88(RRD_instructionVar88),
    Var89(JP_instructionVar89),
    Var90(JP_instructionVar90),
    Var91(RETI_instructionVar91),
    Var92(RETN_instructionVar92),
    Var93(INI_instructionVar93),
    Var94(INIR_instructionVar94),
    Var95(IND_instructionVar95),
    Var96(INDR_instructionVar96),
    Var97(OUTI_instructionVar97),
    Var98(OTIR_instructionVar98),
    Var99(OUTD_instructionVar99),
    Var100(OTDR_instructionVar100),
    Var101(TST_instructionVar101),
    Var102(TST_instructionVar102),
    Var103(IN0_instructionVar103),
    Var104(OTDM_instructionVar104),
    Var105(OTDMR_instructionVar105),
    Var106(TSTIO_instructionVar106),
    Var107(OTIM_instructionVar107),
    Var108(OTIMR_instructionVar108),
    Var109(SLP_instructionVar109),
    Var110(LD_instructionVar110),
    Var111(LD_instructionVar111),
    Var112(ADC_instructionVar112),
    Var113(SBC_instructionVar113),
    Var114(ADD_instructionVar114),
    Var115(ADD_instructionVar115),
    Var116(RLC_instructionVar116),
    Var117(RL_instructionVar117),
    Var118(RRC_instructionVar118),
    Var119(RR_instructionVar119),
    Var120(SLA_instructionVar120),
    Var121(SRA_instructionVar121),
    Var122(SRL_instructionVar122),
    Var123(MLT_instructionVar123),
    Var124(BIT_instructionVar124),
    Var125(SET_instructionVar125),
    Var126(RES_instructionVar126),
    Var127(IN_instructionVar127),
    Var128(OUT_instructionVar128),
    Var129(TST_instructionVar129),
    Var130(IN0_instructionVar130),
    Var131(OUT0_instructionVar131),
    Var132(BIT_instructionVar132),
    Var133(SET_instructionVar133),
    Var134(RES_instructionVar134),
    Var135(LD_instructionVar135),
    Var136(LD_instructionVar136),
    Var137(LD_instructionVar137),
    Var138(LD_instructionVar138),
    Var139(LD_instructionVar139),
    Var140(LD_instructionVar140),
    Var141(LD_instructionVar141),
    Var142(LD_instructionVar142),
    Var143(LD_instructionVar143),
    Var144(LD_instructionVar144),
    Var145(EX_instructionVar145),
    Var146(EX_instructionVar146),
    Var147(EXX_instructionVar147),
    Var148(EX_instructionVar148),
    Var149(ADD_instructionVar149),
    Var150(ADD_instructionVar150),
    Var151(ADC_instructionVar151),
    Var152(ADC_instructionVar152),
    Var153(SUB_instructionVar153),
    Var154(SUB_instructionVar154),
    Var155(SBC_instructionVar155),
    Var156(SBC_instructionVar156),
    Var157(AND_instructionVar157),
    Var158(AND_instructionVar158),
    Var159(OR_instructionVar159),
    Var160(OR_instructionVar160),
    Var161(XOR_instructionVar161),
    Var162(XOR_instructionVar162),
    Var163(CP_instructionVar163),
    Var164(CP_instructionVar164),
    Var165(INC_instructionVar165),
    Var166(DEC_instructionVar166),
    Var167(DAA_instructionVar167),
    Var168(CPL_instructionVar168),
    Var169(CCF_instructionVar169),
    Var170(SCF_instructionVar170),
    Var171(NOP_instructionVar171),
    Var172(HALT_instructionVar172),
    Var173(DI_instructionVar173),
    Var174(EI_instructionVar174),
    Var175(RLCA_instructionVar175),
    Var176(RLA_instructionVar176),
    Var177(RRCA_instructionVar177),
    Var178(RRA_instructionVar178),
    Var179(JP_instructionVar179),
    Var180(JR_instructionVar180),
    Var181(JP_instructionVar181),
    Var182(DJNZ_instructionVar182),
    Var183(CALL_instructionVar183),
    Var184(RET_instructionVar184),
    Var185(IN_instructionVar185),
    Var186(OUT_instructionVar186),
    Var187(LD_instructionVar187),
    Var188(PUSH_instructionVar188),
    Var189(POP_instructionVar189),
    Var190(ADD_instructionVar190),
    Var191(INC_instructionVar191),
    Var192(DEC_instructionVar192),
    Var193(LD_instructionVar193),
    Var194(LD_instructionVar194),
    Var195(LD_instructionVar195),
    Var196(ADD_instructionVar196),
    Var197(ADC_instructionVar197),
    Var198(SUB_instructionVar198),
    Var199(SBC_instructionVar199),
    Var200(AND_instructionVar200),
    Var201(OR_instructionVar201),
    Var202(XOR_instructionVar202),
    Var203(CP_instructionVar203),
    Var204(INC_instructionVar204),
    Var205(DEC_instructionVar205),
    Var206(JP_instructionVar206),
    Var207(JR_instructionVar207),
    Var208(CALL_instructionVar208),
    Var209(RET_instructionVar209),
    Var210(RST_instructionVar210),
    Var211(LD_instructionVar211),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                RL_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                RL_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                RR_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                RR_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                SLA_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                SLA_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                SRA_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                SRA_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 62
        {
            if let Some((inst_len, parsed)) =
                SRL_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 255) == 62
        {
            if let Some((inst_len, parsed)) =
                SRL_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 70
        {
            if let Some((inst_len, parsed)) =
                BIT_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 70
        {
            if let Some((inst_len, parsed)) =
                BIT_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 198
        {
            if let Some((inst_len, parsed)) =
                SET_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 198
        {
            if let Some((inst_len, parsed)) =
                SET_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 134
        {
            if let Some((inst_len, parsed)) =
                RES_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 203
            && (tokens_param[3] & 199) == 134
        {
            if let Some((inst_len, parsed)) =
                RES_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 199) == 70
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 199) == 70
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 248) == 112
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 248) == 112
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 54
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 54
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 87
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 95
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 71
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 79
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 33
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 33
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 249
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 249
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 229
        {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 229
        {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 225
        {
            if let Some((inst_len, parsed)) =
                POP_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 225
        {
            if let Some((inst_len, parsed)) =
                POP_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 227
        {
            if let Some((inst_len, parsed)) =
                EX_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 227
        {
            if let Some((inst_len, parsed)) =
                EX_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 160
        {
            if let Some((inst_len, parsed)) =
                LDI_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 176
        {
            if let Some((inst_len, parsed)) =
                LDIR_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 168
        {
            if let Some((inst_len, parsed)) =
                LDD_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 184
        {
            if let Some((inst_len, parsed)) =
                LDDR_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 161
        {
            if let Some((inst_len, parsed)) =
                CPI_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 177
        {
            if let Some((inst_len, parsed)) =
                CPIR_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 169
        {
            if let Some((inst_len, parsed)) =
                CPD_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 185
        {
            if let Some((inst_len, parsed)) =
                CPDR_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 134
        {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 134
        {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 142
        {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 142
        {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 150
        {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 150
        {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 158
        {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 158
        {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 166
        {
            if let Some((inst_len, parsed)) =
                AND_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 166
        {
            if let Some((inst_len, parsed)) =
                AND_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 182
        {
            if let Some((inst_len, parsed)) =
                OR_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 182
        {
            if let Some((inst_len, parsed)) =
                OR_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 174
        {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 174
        {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 190
        {
            if let Some((inst_len, parsed)) =
                CP_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 190
        {
            if let Some((inst_len, parsed)) =
                CP_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 52
        {
            if let Some((inst_len, parsed)) =
                INC_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 52
        {
            if let Some((inst_len, parsed)) =
                INC_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 53
        {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 53
        {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 68
        {
            if let Some((inst_len, parsed)) =
                NEG_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 70
        {
            if let Some((inst_len, parsed)) =
                IM_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 86
        {
            if let Some((inst_len, parsed)) =
                IM_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 94
        {
            if let Some((inst_len, parsed)) =
                IM_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                INC_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                INC_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 && (tokens_param[1] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                RL_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                RR_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                SLA_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                SRA_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 255) == 62
        {
            if let Some((inst_len, parsed)) =
                SRL_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 111
        {
            if let Some((inst_len, parsed)) =
                RLD_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 103
        {
            if let Some((inst_len, parsed)) =
                RRD_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 221
            && (tokens_param[1] & 255) == 233
        {
            if let Some((inst_len, parsed)) =
                JP_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 253
            && (tokens_param[1] & 255) == 233
        {
            if let Some((inst_len, parsed)) =
                JP_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 77
        {
            if let Some((inst_len, parsed)) =
                RETI_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 69
        {
            if let Some((inst_len, parsed)) =
                RETN_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 162
        {
            if let Some((inst_len, parsed)) =
                INI_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 178
        {
            if let Some((inst_len, parsed)) =
                INIR_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 170
        {
            if let Some((inst_len, parsed)) =
                IND_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 186
        {
            if let Some((inst_len, parsed)) =
                INDR_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 163
        {
            if let Some((inst_len, parsed)) =
                OUTI_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 179
        {
            if let Some((inst_len, parsed)) =
                OTIR_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                OUTD_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 187
        {
            if let Some((inst_len, parsed)) =
                OTDR_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 52
        {
            if let Some((inst_len, parsed)) =
                TST_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 100
        {
            if let Some((inst_len, parsed)) =
                TST_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 48
        {
            if let Some((inst_len, parsed)) =
                IN0_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 139
        {
            if let Some((inst_len, parsed)) =
                OTDM_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 155
        {
            if let Some((inst_len, parsed)) =
                OTDMR_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 116
        {
            if let Some((inst_len, parsed)) =
                TSTIO_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 131
        {
            if let Some((inst_len, parsed)) =
                OTIM_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 147
        {
            if let Some((inst_len, parsed)) =
                OTIMR_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 255) == 118
        {
            if let Some((inst_len, parsed)) =
                SLP_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 207) == 75
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 207) == 67
        {
            if let Some((inst_len, parsed)) =
                LD_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 207) == 74
        {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 207) == 66
        {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 221 && (tokens_param[1] & 207) == 9
        {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 253 && (tokens_param[1] & 207) == 9
        {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 && (tokens_param[1] & 248) == 0
        {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 248) == 16
        {
            if let Some((inst_len, parsed)) =
                RL_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 203 && (tokens_param[1] & 248) == 8
        {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 248) == 24
        {
            if let Some((inst_len, parsed)) =
                RR_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 248) == 32
        {
            if let Some((inst_len, parsed)) =
                SLA_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 248) == 40
        {
            if let Some((inst_len, parsed)) =
                SRA_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 248) == 56
        {
            if let Some((inst_len, parsed)) =
                SRL_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 207) == 76
        {
            if let Some((inst_len, parsed)) =
                MLT_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 199) == 70
        {
            if let Some((inst_len, parsed)) =
                BIT_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 199) == 198
        {
            if let Some((inst_len, parsed)) =
                SET_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 199) == 134
        {
            if let Some((inst_len, parsed)) =
                RES_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 199) == 64
        {
            if let Some((inst_len, parsed)) =
                IN_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 237
            && (tokens_param[1] & 199) == 65
        {
            if let Some((inst_len, parsed)) =
                OUT_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 237 && (tokens_param[1] & 199) == 4
        {
            if let Some((inst_len, parsed)) =
                TST_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 237 && (tokens_param[1] & 199) == 0
        {
            if let Some((inst_len, parsed)) =
                IN0_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 237 && (tokens_param[1] & 199) == 1
        {
            if let Some((inst_len, parsed)) =
                OUT0_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 192) == 64
        {
            if let Some((inst_len, parsed)) =
                BIT_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 192) == 192
        {
            if let Some((inst_len, parsed)) =
                SET_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && (tokens_param[0] & 255) == 203
            && (tokens_param[1] & 192) == 128
        {
            if let Some((inst_len, parsed)) =
                RES_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 54 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar135::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 10 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar136::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var136(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 26 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar137::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var137(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 58 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar138::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var138(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar139::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var139(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 18 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar140::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var140(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 50 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar141::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var141(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 42 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar142::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var142(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 34 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar143::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var143(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 249 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar144::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var144(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 235 {
            if let Some((inst_len, parsed)) =
                EX_instructionVar145::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var145(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 8 {
            if let Some((inst_len, parsed)) =
                EX_instructionVar146::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var146(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 217 {
            if let Some((inst_len, parsed)) =
                EXX_instructionVar147::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var147(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 227 {
            if let Some((inst_len, parsed)) =
                EX_instructionVar148::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var148(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 198 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar149::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var149(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 134 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar150::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var150(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 206 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar151::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var151(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 142 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar152::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var152(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 214 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar153::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var153(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 150 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar154::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var154(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 222 {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar155::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var155(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 158 {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar156::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var156(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 230 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar157::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var157(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 166 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar158::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var158(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 246 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar159::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var159(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 182 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar160::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var160(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 238 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar161::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var161(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 174 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar162::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var162(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 254 {
            if let Some((inst_len, parsed)) =
                CP_instructionVar163::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var163(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 190 {
            if let Some((inst_len, parsed)) =
                CP_instructionVar164::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var164(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 52 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar165::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var165(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 53 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar166::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var166(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 39 {
            if let Some((inst_len, parsed)) =
                DAA_instructionVar167::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var167(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 47 {
            if let Some((inst_len, parsed)) =
                CPL_instructionVar168::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var168(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 63 {
            if let Some((inst_len, parsed)) =
                CCF_instructionVar169::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var169(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 55 {
            if let Some((inst_len, parsed)) =
                SCF_instructionVar170::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var170(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar171::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var171(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 118 {
            if let Some((inst_len, parsed)) =
                HALT_instructionVar172::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var172(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 243 {
            if let Some((inst_len, parsed)) =
                DI_instructionVar173::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var173(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 251 {
            if let Some((inst_len, parsed)) =
                EI_instructionVar174::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var174(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                RLCA_instructionVar175::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var175(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 23 {
            if let Some((inst_len, parsed)) =
                RLA_instructionVar176::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var176(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 15 {
            if let Some((inst_len, parsed)) =
                RRCA_instructionVar177::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var177(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 31 {
            if let Some((inst_len, parsed)) =
                RRA_instructionVar178::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var178(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 195 {
            if let Some((inst_len, parsed)) =
                JP_instructionVar179::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var179(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 24 {
            if let Some((inst_len, parsed)) =
                JR_instructionVar180::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var180(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 233 {
            if let Some((inst_len, parsed)) =
                JP_instructionVar181::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var181(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 16 {
            if let Some((inst_len, parsed)) =
                DJNZ_instructionVar182::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var182(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 205 {
            if let Some((inst_len, parsed)) =
                CALL_instructionVar183::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var183(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 201 {
            if let Some((inst_len, parsed)) =
                RET_instructionVar184::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var184(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 219 {
            if let Some((inst_len, parsed)) =
                IN_instructionVar185::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var185(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 211 {
            if let Some((inst_len, parsed)) =
                OUT_instructionVar186::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var186(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 207) == 1 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar187::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var187(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 197 {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar188::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var188(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 193 {
            if let Some((inst_len, parsed)) =
                POP_instructionVar189::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var189(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 9 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar190::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var190(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 3 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar191::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var191(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 11 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar192::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var192(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 199) == 6 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar193::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var193(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 70 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar194::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var194(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 112 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar195::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var195(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 128 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar196::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var196(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 136 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar197::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var197(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 144 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar198::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var198(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 152 {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar199::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var199(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 160 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar200::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var200(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 176 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar201::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var201(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 168 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar202::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var202(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 248) == 184 {
            if let Some((inst_len, parsed)) =
                CP_instructionVar203::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var203(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 4 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar204::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var204(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 5 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar205::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var205(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 199) == 194 {
            if let Some((inst_len, parsed)) =
                JP_instructionVar206::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var206(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 199) == 0 {
            if let Some((inst_len, parsed)) =
                JR_instructionVar207::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var207(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 199) == 196 {
            if let Some((inst_len, parsed)) =
                CALL_instructionVar208::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var208(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 192 {
            if let Some((inst_len, parsed)) =
                RET_instructionVar209::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var209(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 199) == 199 {
            if let Some((inst_len, parsed)) =
                RST_instructionVar210::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var210(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 192) == 64 {
            if let Some((inst_len, parsed)) =
                LD_instructionVar211::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var211(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:284:1, end:284:5))"]
#[derive(Clone, Debug)]
struct FlagVar0 {}
impl FlagVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("Flag")];
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
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableFlag {
    Var0(FlagVar0),
}
impl TableFlag {
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
                FlagVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:314:1, end:314:7))"]
#[derive(Clone, Debug)]
struct hlMem8Var0 {}
impl hlMem8Var0 {
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
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TablehlMem8 {
    Var0(hlMem8Var0),
}
impl TablehlMem8 {
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
                hlMem8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:318:1, end:318:7))"]
#[derive(Clone, Debug)]
struct ixMem8Var0 {
    simm8: u8,
}
impl ixMem8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (-i128::try_from(
            (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8),
        )
        .unwrap());
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IX),
            <DisplayElement>::Literal("-"),
            <DisplayElement>::Number(true, calc_val.is_negative(), calc_val.abs() as u64),
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
        let mut calc_val: i128 = 0;
        let mut block_0_len = 1;
        calc_val = (-i128::try_from(token_1(tokens_current)).unwrap());
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:317:1, end:317:7))"]
#[derive(Clone, Debug)]
struct ixMem8Var1 {
    simm8: u8,
}
impl ixMem8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IX),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableixMem8 {
    Var0(ixMem8Var0),
    Var1(ixMem8Var1),
}
impl TableixMem8 {
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
        if tokens_param.len() >= 1 && (tokens_param[0] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                ixMem8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                ixMem8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:321:1, end:321:7))"]
#[derive(Clone, Debug)]
struct iyMem8Var0 {
    simm8: u8,
}
impl iyMem8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_val: i128 = 0;
        calc_val = (-i128::try_from(
            (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8),
        )
        .unwrap());
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IY),
            <DisplayElement>::Literal("-"),
            <DisplayElement>::Number(true, calc_val.is_negative(), calc_val.abs() as u64),
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
        let mut calc_val: i128 = 0;
        let mut block_0_len = 1;
        calc_val = (-i128::try_from(token_1(tokens_current)).unwrap());
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:320:1, end:320:7))"]
#[derive(Clone, Debug)]
struct iyMem8Var1 {
    simm8: u8,
}
impl iyMem8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("("),
            <DisplayElement>::Register(Register::IY),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableiyMem8 {
    Var0(iyMem8Var0),
    Var1(iyMem8Var1),
}
impl TableiyMem8 {
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
        if tokens_param.len() >= 1 && (tokens_param[0] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                iyMem8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 {
            if let Some((inst_len, parsed)) =
                iyMem8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:326:1, end:326:7))"]
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
        let imm16 = token_9(tokens_current);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:328:1, end:328:5))"]
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
        let imm16 = token_9(tokens_current);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:330:1, end:330:6))"]
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
        let imm16 = token_9(tokens_current);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:342:1, end:342:9))"]
#[derive(Clone, Debug)]
struct RelAddr8Var0 {
    simm8: u8,
}
impl RelAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_loc: i128 = 0;
        calc_loc = i128::try_from(inst_next).unwrap().wrapping_add(
            i128::try_from((if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8))
                .unwrap(),
        );
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
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableRelAddr8 {
    Var0(RelAddr8Var0),
}
impl TableRelAddr8 {
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
                RelAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:344:1, end:344:8))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:347:1, end:347:8))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:349:1, end:349:9))"]
#[derive(Clone, Debug)]
struct IOAddr8aVar0 {
    imm8: u8,
}
impl IOAddr8aVar0 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:350:1, end:350:9))"]
#[derive(Clone, Debug)]
struct IOAddr8aVar1 {
    imm8: u8,
}
impl IOAddr8aVar1 {
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
enum TableIOAddr8a {
    Var0(IOAddr8aVar0),
    Var1(IOAddr8aVar1),
}
impl TableIOAddr8a {
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
        if tokens_param.len() >= 1 && context_param.0 & 1 == 0 {
            if let Some((inst_len, parsed)) =
                IOAddr8aVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                IOAddr8aVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:352:1, end:352:8))"]
#[derive(Clone, Debug)]
struct IOAddrCVar0 {}
impl IOAddrCVar0 {
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
            <DisplayElement>::Register(Register::C),
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
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:353:1, end:353:8))"]
#[derive(Clone, Debug)]
struct IOAddrCVar1 {}
impl IOAddrCVar1 {
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
            <DisplayElement>::Register(Register::C),
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
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableIOAddrC {
    Var0(IOAddrCVar0),
    Var1(IOAddrCVar1),
}
impl TableIOAddrC {
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
        if tokens_param.len() >= 0 && context_param.0 & 1 == 0 {
            if let Some((inst_len, parsed)) =
                IOAddrCVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 0 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                IOAddrCVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:361:1, end:361:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:362:1, end:362:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:363:1, end:363:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:364:1, end:364:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:365:1, end:365:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:366:1, end:366:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:367:1, end:367:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:368:1, end:368:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:370:1, end:370:4))"]
#[derive(Clone, Debug)]
struct cc2Var0 {}
impl cc2Var0 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:371:1, end:371:4))"]
#[derive(Clone, Debug)]
struct cc2Var1 {}
impl cc2Var1 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:372:1, end:372:4))"]
#[derive(Clone, Debug)]
struct cc2Var2 {}
impl cc2Var2 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/Z80/data/languages/z80.slaspec, start:373:1, end:373:4))"]
#[derive(Clone, Debug)]
struct cc2Var3 {}
impl cc2Var3 {
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
#[derive(Clone, Debug)]
enum Tablecc2 {
    Var0(cc2Var0),
    Var1(cc2Var1),
    Var2(cc2Var2),
    Var3(cc2Var3),
}
impl Tablecc2 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 32 {
            if let Some((inst_len, parsed)) =
                cc2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 40 {
            if let Some((inst_len, parsed)) =
                cc2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 48 {
            if let Some((inst_len, parsed)) =
                cc2Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 56) == 56 {
            if let Some((inst_len, parsed)) =
                cc2Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
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
