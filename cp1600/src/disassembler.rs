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
    I,
    C,
    O,
    Z,
    S,
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
            Self::I => "I",
            Self::C => "C",
            Self::O => "O",
            Self::Z => "Z",
            Self::S => "S",
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
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
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
        0 => Register::R0,
        1 => Register::R1,
        2 => Register::R2,
        3 => Register::R3,
        4 => Register::R4,
        5 => Register::R5,
        6 => Register::R6,
        7 => Register::R7,
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
        0 => Register::R4,
        1 => Register::R5,
        2 => Register::R6,
        3 => Register::R7,
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
#[doc = "Create token_fields: target24_25 reg24_25"]
fn token_13(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 24) & 3) as u8)
}
#[doc = "Create token_fields: branch_sign"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 1) as u8)
}
#[doc = "Create token_fields: value_hi"]
fn token_18(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: opcode3_9"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 127) as u8)
}
#[doc = "Create token_fields: opcode2_9"]
fn token_10(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 2) & 255) as u8)
}
#[doc = "Create token_fields: branch_condition external_condition"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: imm16 addr16"]
fn token_19(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: target0_2 reg0_2"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: operation_size"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 2) & 1) as u8)
}
#[doc = "Create token_fields: branch_external"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: jump_type"]
fn token_15(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 3) as u8)
}
#[doc = "Create token_fields: target3_5 reg3_5"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 3) & 7) as u8)
}
#[doc = "Create token_fields: opcode1_9"]
fn token_11(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 1) & 511) as u16)
}
#[doc = "Create token_fields: address_hi"]
fn token_14(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 18) & 63) as u8)
}
#[doc = "Create token_fields: reg0_1"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 3) as u8)
}
#[doc = "Create token_fields: address_lo"]
fn token_16(tokens: &[u8]) -> u16 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 0) & 1023) as u16)
}
#[doc = "Create token_fields: value_lo"]
fn token_17(tokens: &[u8]) -> u8 {
    (((u32::from_be_bytes(tokens[0..4].try_into().unwrap()) >> 16) & 255) as u8)
}
#[doc = "Create token_fields: opcode0_9"]
fn token_12(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 1023) as u16)
}
#[doc = "Create token_fields: opcode6_9"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 6) & 15) as u8)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u8);
impl ContextMemory {
    pub fn read_doublebyte(&self) -> u8 {
        (((self.0.reverse_bits() >> 7) & 1) as u8)
    }
    pub fn write_doublebyte(&mut self, value: u8) {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:273:1, end:273:2))"]
#[derive(Clone, Debug)]
struct J_instructionVar0 {
    jmpdest16: Tablejmpdest16,
}
impl J_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("J"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:277:1, end:277:2))"]
#[derive(Clone, Debug)]
struct JD_instructionVar1 {
    jmpdest16: Tablejmpdest16,
}
impl JD_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:282:1, end:282:2))"]
#[derive(Clone, Debug)]
struct JE_instructionVar2 {
    jmpdest16: Tablejmpdest16,
}
impl JE_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JE"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { jmpdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:293:1, end:293:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar3 {
    reg24_25: u8,
    jmpdest16: Tablejmpdest16,
}
impl JSR_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JSR"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.reg24_25),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg24_25 = token_13(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:298:1, end:298:2))"]
#[derive(Clone, Debug)]
struct JSRD_instructionVar4 {
    reg24_25: u8,
    jmpdest16: Tablejmpdest16,
}
impl JSRD_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JSRD"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.reg24_25),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg24_25 = token_13(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:304:1, end:304:2))"]
#[derive(Clone, Debug)]
struct JSRE_instructionVar5 {
    reg24_25: u8,
    jmpdest16: Tablejmpdest16,
}
impl JSRE_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JSRE"));
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            meaning_2_display(self.reg24_25),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.jmpdest16
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
        let jmpdest16 = if let Some((len, table)) =
            Tablejmpdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg24_25 = token_13(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                jmpdest16,
                reg24_25,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:215:1, end:215:2))"]
#[derive(Clone, Debug)]
struct CLRC_instructionVar6 {}
impl CLRC_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct DIS_instructionVar7 {}
impl DIS_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DIS"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:254:1, end:254:2))"]
#[derive(Clone, Debug)]
struct EIS_instructionVar8 {}
impl EIS_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EIS"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct HLT_instructionVar9 {}
impl HLT_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("HLT"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:384:1, end:384:2))"]
#[derive(Clone, Debug)]
struct SDBD_instructionVar10 {}
impl SDBD_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_doublebyte(
                u8::try_from(i128::try_from(context.read_doublebyte()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("SDBD"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        context_instance.write_doublebyte(u8::try_from(1i128 & 1).unwrap());
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct SETC_instructionVar11 {}
impl SETC_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SETC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:411:1, end:411:2))"]
#[derive(Clone, Debug)]
struct TCI_instructionVar12 {}
impl TCI_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TCI"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:361:1, end:361:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar13 {}
impl NOP_instructionVar13 {
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
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:391:1, end:391:2))"]
#[derive(Clone, Debug)]
struct SIN_instructionVar14 {}
impl SIN_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SIN"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct GSWD_instructionVar15 {
    reg0_1: u8,
}
impl GSWD_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("GSWD"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_0_display(self.reg0_1),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_1 = token_3(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:512:1, end:512:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar16 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RLC_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:524:1, end:524:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar17 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl RRC_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:536:1, end:536:2))"]
#[derive(Clone, Debug)]
struct SAR_instructionVar18 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SAR_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SAR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:544:1, end:544:2))"]
#[derive(Clone, Debug)]
struct SARC_instructionVar19 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SARC_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SARC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:556:1, end:556:2))"]
#[derive(Clone, Debug)]
struct SLL_instructionVar20 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLL_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:564:1, end:564:2))"]
#[derive(Clone, Debug)]
struct SLLC_instructionVar21 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLLC_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLLC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:576:1, end:576:2))"]
#[derive(Clone, Debug)]
struct SLR_instructionVar22 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SLR_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:584:1, end:584:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar23 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SWAP_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SWAP"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:626:1, end:626:2))"]
#[derive(Clone, Debug)]
struct ADDI_instructionVar24 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
    splitimm16: Tablesplitimm16,
}
impl ADDI_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
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
                checkbranch,
                regval0_2,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:631:1, end:631:2))"]
#[derive(Clone, Debug)]
struct ANDI_instructionVar25 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl ANDI_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
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
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:637:1, end:637:2))"]
#[derive(Clone, Debug)]
struct CMPI_instructionVar26 {
    reg0_2: u8,
    splitimm16: Tablesplitimm16,
}
impl CMPI_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMPI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { splitimm16, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:641:1, end:641:2))"]
#[derive(Clone, Debug)]
struct MVII_instructionVar27 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl MVII_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVII"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
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
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:646:1, end:646:2))"]
#[derive(Clone, Debug)]
struct SUBI_instructionVar28 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
    splitimm16: Tablesplitimm16,
}
impl SUBI_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
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
                regval0_2,
                checkbranch,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:651:1, end:651:2))"]
#[derive(Clone, Debug)]
struct XORI_instructionVar29 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
    splitimm16: Tablesplitimm16,
}
impl XORI_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
        ];
        display.extend_from_slice(&extend);
        self.splitimm16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let splitimm16 = if let Some((len, table)) =
            Tablesplitimm16::parse(tokens_current, &mut context_instance, inst_start)
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
                checkbranch,
                regval0_2,
                splitimm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar30 {
    addr16: u16,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl ADD_instructionVar30 {
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
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                addr16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:177:1, end:177:2))"]
#[derive(Clone, Debug)]
struct ADCR_instructionVar31 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADCR_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADCR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:188:1, end:188:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar32 {
    addr16: u16,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl AND_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                addr16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:225:1, end:225:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar33 {
    addr16: u16,
    reg0_2: u8,
}
impl CMP_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr16, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:238:1, end:238:2))"]
#[derive(Clone, Debug)]
struct COMR_instructionVar34 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl COMR_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("COMR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:244:1, end:244:2))"]
#[derive(Clone, Debug)]
struct DECR_instructionVar35 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl DECR_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:267:1, end:267:2))"]
#[derive(Clone, Debug)]
struct INCR_instructionVar36 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl INCR_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar37 {
    addr16: u16,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
}
impl MVI_instructionVar37 {
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
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                addr16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar38 {
    reg0_2: u8,
    addr16: u16,
}
impl MVO_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVO"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.addr16 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2, addr16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:332:1, end:332:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar39 {
    reg0_2: u8,
    reg3_5: u8,
    checkbranch: Tablecheckbranch,
}
impl MVO_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVO"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c62 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 4 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 5 {
                    return Some(((), (), 2));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c62(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                reg3_5,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:345:1, end:345:2))"]
#[derive(Clone, Debug)]
struct MVOI_instructionVar40 {
    reg0_2: u8,
}
impl MVOI_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVOI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:350:1, end:350:2))"]
#[derive(Clone, Debug)]
struct NEGR_instructionVar41 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl NEGR_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEGR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:364:1, end:364:2))"]
#[derive(Clone, Debug)]
struct NOPP_instructionVar42 {}
impl NOPP_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NOPP"));
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
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:367:1, end:367:2))"]
#[derive(Clone, Debug)]
struct PSHR_instructionVar43 {
    reg0_2: u8,
}
impl PSHR_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PSHR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:373:1, end:373:2))"]
#[derive(Clone, Debug)]
struct PULR_instructionVar44 {
    reg0_2: u8,
    impliedval16: Tableimpliedval16,
}
impl PULR_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("PULR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:377:1, end:377:2))"]
#[derive(Clone, Debug)]
struct RSWD_instructionVar45 {
    reg0_2: u8,
}
impl RSWD_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RSWD"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:395:1, end:395:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar46 {
    addr16: u16,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUB_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUB"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                addr16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:419:1, end:419:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar47 {
    addr16: u16,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl XOR_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XOR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.addr16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let addr16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                addr16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:438:1, end:438:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar48 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RLC_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:448:1, end:448:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar49 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl RRC_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:458:1, end:458:2))"]
#[derive(Clone, Debug)]
struct SAR_instructionVar50 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SAR_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SAR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:466:1, end:466:2))"]
#[derive(Clone, Debug)]
struct SARC_instructionVar51 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SARC_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SARC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:476:1, end:476:2))"]
#[derive(Clone, Debug)]
struct SLL_instructionVar52 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLL_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLL"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:484:1, end:484:2))"]
#[derive(Clone, Debug)]
struct SLLC_instructionVar53 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLLC_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLLC"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:494:1, end:494:2))"]
#[derive(Clone, Debug)]
struct SLR_instructionVar54 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SLR_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:502:1, end:502:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar55 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SWAP_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SWAP"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
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
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:593:1, end:593:2))"]
#[derive(Clone, Debug)]
struct ADDI_instructionVar56 {
    imm16: u16,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl ADDI_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDI"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                imm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:598:1, end:598:2))"]
#[derive(Clone, Debug)]
struct ANDI_instructionVar57 {
    imm16: u16,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl ANDI_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDI"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                imm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:604:1, end:604:2))"]
#[derive(Clone, Debug)]
struct CMPI_instructionVar58 {
    imm16: u16,
    reg0_2: u8,
}
impl CMPI_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMPI"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:608:1, end:608:2))"]
#[derive(Clone, Debug)]
struct MVII_instructionVar59 {
    imm16: u16,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
}
impl MVII_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVII"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                imm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:613:1, end:613:2))"]
#[derive(Clone, Debug)]
struct SUBI_instructionVar60 {
    imm16: u16,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUBI_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBI"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                imm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:618:1, end:618:2))"]
#[derive(Clone, Debug)]
struct XORI_instructionVar61 {
    imm16: u16,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl XORI_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORI"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Literal("#"),
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                imm16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:211:1, end:211:2))"]
#[derive(Clone, Debug)]
struct BEXT_instructionVar62 {
    external_condition: u8,
    branchdest16: Tablebranchdest16,
}
impl BEXT_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BEXT"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.branchdest16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.external_condition as u64),
        ];
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
        let mut sub_pattern_c43 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if token_8(tokens) != 8 {
                return None;
            }
            if token_6(tokens) != 1 {
                return None;
            }
            let external_condition = token_7(tokens);
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (external_condition), pattern_len))
        };
        let ((), (external_condition), sub_len) =
            sub_pattern_c43(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let branchdest16 = if let Some((len, table)) =
            Tablebranchdest16::parse(tokens_current, &mut context_instance, inst_start)
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
                branchdest16,
                external_condition,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:219:1, end:219:2))"]
#[derive(Clone, Debug)]
struct CLRR_instructionVar63 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
}
impl CLRR_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c45 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if i128::try_from(token_2(tokens)).unwrap() != i128::try_from(token_1(tokens)).unwrap()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c45(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:287:1, end:287:2))"]
#[derive(Clone, Debug)]
struct JR_instructionVar64 {
    reg3_5: u8,
}
impl JR_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct TSTR_instructionVar65 {
    reg0_2: u8,
}
impl TSTR_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TSTR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c45 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if i128::try_from(token_2(tokens)).unwrap() != i128::try_from(token_1(tokens)).unwrap()
            {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c45(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:172:1, end:172:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar66 {
    reg0_2: u8,
    impliedval16: Tableimpliedval16,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl ADD_instructionVar66 {
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
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:183:1, end:183:2))"]
#[derive(Clone, Debug)]
struct ADDR_instructionVar67 {
    reg3_5: u8,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl ADDR_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:195:1, end:195:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar68 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
    regval0_2: Tableregval0_2,
}
impl AND_instructionVar68 {
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
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                impliedval16,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:201:1, end:201:2))"]
#[derive(Clone, Debug)]
struct ANDR_instructionVar69 {
    reg3_5: u8,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl ANDR_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg3_5 = token_1(tokens_current);
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:207:1, end:207:2))"]
#[derive(Clone, Debug)]
struct B_instructionVar70 {
    cc: Tablecc,
    branchdest16: Tablebranchdest16,
}
impl B_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("B"));
        self.cc
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.branchdest16
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
            if token_8(tokens) != 8 {
                return None;
            }
            let cc = if let Some((len, table)) =
                Tablecc::parse(tokens, &mut context_instance, inst_start)
            {
                block_0_len = block_0_len.max(len as AddrType);
                table
            } else {
                return None;
            };
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((cc), (), pattern_len))
        };
        let ((mut cc), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let branchdest16 = if let Some((len, table)) =
            Tablebranchdest16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { cc, branchdest16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:230:1, end:230:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar71 {
    reg0_2: u8,
    impliedval16: Tableimpliedval16,
}
impl CMP_instructionVar71 {
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
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct CMPR_instructionVar72 {
    reg3_5: u8,
    reg0_2: u8,
}
impl CMPR_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMPR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg3_5 = token_1(tokens_current);
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5, reg0_2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:310:1, end:310:2))"]
#[derive(Clone, Debug)]
struct MOVR_instructionVar73 {
    reg3_5: u8,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
}
impl MOVR_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOVR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:322:1, end:322:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar74 {
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    impliedval16: Tableimpliedval16,
}
impl MVI_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVI"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                impliedval16,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:339:1, end:339:2))"]
#[derive(Clone, Debug)]
struct MVO_instructionVar75 {
    reg0_2: u8,
    reg3_5: u8,
    checkbranch: Tablecheckbranch,
}
impl MVO_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVO"));
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                reg0_2,
                reg3_5,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:401:1, end:401:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar76 {
    reg0_2: u8,
    impliedval16: Tableimpliedval16,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl SUB_instructionVar76 {
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
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                impliedval16,
                checkbranch,
                regval0_2,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:406:1, end:406:2))"]
#[derive(Clone, Debug)]
struct SUBR_instructionVar77 {
    reg3_5: u8,
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    checkbranch: Tablecheckbranch,
}
impl SUBR_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg3_5 = token_1(tokens_current);
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                checkbranch,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:426:1, end:426:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar78 {
    reg0_2: u8,
    regval0_2: Tableregval0_2,
    impliedval16: Tableimpliedval16,
    checkbranch: Tablecheckbranch,
}
impl XOR_instructionVar78 {
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
            <DisplayElement>::Literal("@"),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.impliedval16
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let impliedval16 = if let Some((len, table)) =
            Tableimpliedval16::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                regval0_2,
                impliedval16,
                checkbranch,
                reg0_2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:432:1, end:432:2))"]
#[derive(Clone, Debug)]
struct XORR_instructionVar79 {
    reg3_5: u8,
    reg0_2: u8,
    checkbranch: Tablecheckbranch,
    regval0_2: Tableregval0_2,
}
impl XORR_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg3_5),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            meaning_1_display(self.reg0_2),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let regval0_2 = if let Some((len, table)) =
            Tableregval0_2::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let checkbranch = if let Some((len, table)) =
            Tablecheckbranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let reg3_5 = token_1(tokens_current);
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                checkbranch,
                regval0_2,
                reg3_5,
                reg0_2,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(J_instructionVar0),
    Var1(JD_instructionVar1),
    Var2(JE_instructionVar2),
    Var3(JSR_instructionVar3),
    Var4(JSRD_instructionVar4),
    Var5(JSRE_instructionVar5),
    Var6(CLRC_instructionVar6),
    Var7(DIS_instructionVar7),
    Var8(EIS_instructionVar8),
    Var9(HLT_instructionVar9),
    Var10(SDBD_instructionVar10),
    Var11(SETC_instructionVar11),
    Var12(TCI_instructionVar12),
    Var13(NOP_instructionVar13),
    Var14(SIN_instructionVar14),
    Var15(GSWD_instructionVar15),
    Var16(RLC_instructionVar16),
    Var17(RRC_instructionVar17),
    Var18(SAR_instructionVar18),
    Var19(SARC_instructionVar19),
    Var20(SLL_instructionVar20),
    Var21(SLLC_instructionVar21),
    Var22(SLR_instructionVar22),
    Var23(SWAP_instructionVar23),
    Var24(ADDI_instructionVar24),
    Var25(ANDI_instructionVar25),
    Var26(CMPI_instructionVar26),
    Var27(MVII_instructionVar27),
    Var28(SUBI_instructionVar28),
    Var29(XORI_instructionVar29),
    Var30(ADD_instructionVar30),
    Var31(ADCR_instructionVar31),
    Var32(AND_instructionVar32),
    Var33(CMP_instructionVar33),
    Var34(COMR_instructionVar34),
    Var35(DECR_instructionVar35),
    Var36(INCR_instructionVar36),
    Var37(MVI_instructionVar37),
    Var38(MVO_instructionVar38),
    Var39(MVO_instructionVar39),
    Var40(MVOI_instructionVar40),
    Var41(NEGR_instructionVar41),
    Var42(NOPP_instructionVar42),
    Var43(PSHR_instructionVar43),
    Var44(PULR_instructionVar44),
    Var45(RSWD_instructionVar45),
    Var46(SUB_instructionVar46),
    Var47(XOR_instructionVar47),
    Var48(RLC_instructionVar48),
    Var49(RRC_instructionVar49),
    Var50(SAR_instructionVar50),
    Var51(SARC_instructionVar51),
    Var52(SLL_instructionVar52),
    Var53(SLLC_instructionVar53),
    Var54(SLR_instructionVar54),
    Var55(SWAP_instructionVar55),
    Var56(ADDI_instructionVar56),
    Var57(ANDI_instructionVar57),
    Var58(CMPI_instructionVar58),
    Var59(MVII_instructionVar59),
    Var60(SUBI_instructionVar60),
    Var61(XORI_instructionVar61),
    Var62(BEXT_instructionVar62),
    Var63(CLRR_instructionVar63),
    Var64(JR_instructionVar64),
    Var65(TSTR_instructionVar65),
    Var66(ADD_instructionVar66),
    Var67(ADDR_instructionVar67),
    Var68(AND_instructionVar68),
    Var69(ANDR_instructionVar69),
    Var70(B_instructionVar70),
    Var71(CMP_instructionVar71),
    Var72(CMPR_instructionVar72),
    Var73(MOVR_instructionVar73),
    Var74(MVI_instructionVar74),
    Var75(MVO_instructionVar75),
    Var76(SUB_instructionVar76),
    Var77(SUBR_instructionVar77),
    Var78(XOR_instructionVar78),
    Var79(XORR_instructionVar79),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[2] & 3) == 3
            && (tokens_param[3] & 3) == 0
        {
            if let Some((inst_len, parsed)) =
                J_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[2] & 3) == 3
            && (tokens_param[3] & 3) == 2
        {
            if let Some((inst_len, parsed)) =
                JD_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[2] & 3) == 3
            && (tokens_param[3] & 3) == 1
        {
            if let Some((inst_len, parsed)) =
                JE_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[3] & 3) == 0
        {
            if let Some((inst_len, parsed)) =
                JSR_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[3] & 3) == 2
        {
            if let Some((inst_len, parsed)) =
                JSRD_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && (tokens_param[0] & 3) == 0
            && (tokens_param[1] & 255) == 4
            && (tokens_param[3] & 3) == 1
        {
            if let Some((inst_len, parsed)) =
                JSRE_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 6 {
            if let Some((inst_len, parsed)) =
                CLRC_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 3 {
            if let Some((inst_len, parsed)) =
                DIS_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                EIS_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                HLT_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                SDBD_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                SETC_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 255) == 5 {
            if let Some((inst_len, parsed)) =
                TCI_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 254) == 52 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 254) == 54 {
            if let Some((inst_len, parsed)) =
                SIN_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 48 {
            if let Some((inst_len, parsed)) =
                GSWD_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 84 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 116 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 108 {
            if let Some((inst_len, parsed)) =
                SAR_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 124 {
            if let Some((inst_len, parsed)) =
                SARC_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 76 {
            if let Some((inst_len, parsed)) =
                SLL_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 92 {
            if let Some((inst_len, parsed)) =
                SLLC_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 100 {
            if let Some((inst_len, parsed)) =
                SLR_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 252) == 68 {
            if let Some((inst_len, parsed)) =
                SWAP_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 2
            && (tokens_param[1] & 248) == 248
        {
            if let Some((inst_len, parsed)) =
                ADDI_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 3
            && (tokens_param[1] & 248) == 184
        {
            if let Some((inst_len, parsed)) =
                ANDI_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 3
            && (tokens_param[1] & 248) == 120
        {
            if let Some((inst_len, parsed)) =
                CMPI_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 2
            && (tokens_param[1] & 248) == 184
        {
            if let Some((inst_len, parsed)) =
                MVII_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 3
            && (tokens_param[1] & 248) == 56
        {
            if let Some((inst_len, parsed)) =
                SUBI_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1 == 1
            && (tokens_param[0] & 3) == 3
            && (tokens_param[1] & 248) == 248
        {
            if let Some((inst_len, parsed)) =
                XORI_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 192 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 40 {
            if let Some((inst_len, parsed)) =
                ADCR_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 128 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 64 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 24 {
            if let Some((inst_len, parsed)) =
                COMR_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 16 {
            if let Some((inst_len, parsed)) =
                DECR_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 8 {
            if let Some((inst_len, parsed)) =
                INCR_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 128 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 64 {
            if let Some((inst_len, parsed)) =
                MVO_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 240) == 96 {
            if let Some((inst_len, parsed)) =
                MVO_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 120 {
            if let Some((inst_len, parsed)) =
                MVOI_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 32 {
            if let Some((inst_len, parsed)) =
                NEGR_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 223) == 8 {
            if let Some((inst_len, parsed)) =
                NOPP_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 112 {
            if let Some((inst_len, parsed)) =
                PSHR_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 176 {
            if let Some((inst_len, parsed)) =
                PULR_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 56 {
            if let Some((inst_len, parsed)) =
                RSWD_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 0 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 192 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 80 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 112 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 104 {
            if let Some((inst_len, parsed)) =
                SAR_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 120 {
            if let Some((inst_len, parsed)) =
                SARC_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 72 {
            if let Some((inst_len, parsed)) =
                SLL_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 88 {
            if let Some((inst_len, parsed)) =
                SLLC_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 96 {
            if let Some((inst_len, parsed)) =
                SLR_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 248) == 64 {
            if let Some((inst_len, parsed)) =
                SWAP_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 248 {
            if let Some((inst_len, parsed)) =
                ADDI_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 184 {
            if let Some((inst_len, parsed)) =
                ANDI_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 120 {
            if let Some((inst_len, parsed)) =
                CMPI_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 248) == 184 {
            if let Some((inst_len, parsed)) =
                MVII_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 56 {
            if let Some((inst_len, parsed)) =
                SUBI_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 248) == 248 {
            if let Some((inst_len, parsed)) =
                XORI_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 208) == 16 {
            if let Some((inst_len, parsed)) =
                BEXT_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 && (tokens_param[1] & 192) == 192 {
            if let Some((inst_len, parsed)) =
                CLRR_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 199) == 135 {
            if let Some((inst_len, parsed)) =
                JR_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                TSTR_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 192) == 192 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 192) == 192 {
            if let Some((inst_len, parsed)) =
                ADDR_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 && (tokens_param[1] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                ANDR_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 192) == 0 {
            if let Some((inst_len, parsed)) =
                B_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 192) == 64 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 && (tokens_param[1] & 192) == 64 {
            if let Some((inst_len, parsed)) =
                CMPR_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 0 && (tokens_param[1] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                MOVR_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 192) == 128 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 2 && (tokens_param[1] & 192) == 64 {
            if let Some((inst_len, parsed)) =
                MVO_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 192) == 0 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 && (tokens_param[1] & 192) == 0 {
            if let Some((inst_len, parsed)) =
                SUBR_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 3 && (tokens_param[1] & 192) == 192 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 3) == 1 && (tokens_param[1] & 192) == 192 {
            if let Some((inst_len, parsed)) =
                XORR_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:58:1, end:58:10))"]
#[derive(Clone, Debug)]
struct jmpdest16Var0 {
    address_hi: u8,
    address_lo: u16,
}
impl jmpdest16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = u32::try_from(10i128)
            .ok()
            .and_then(|shl| i128::try_from(self.address_hi).unwrap().checked_shl(shl))
            .unwrap_or(0)
            .wrapping_add(i128::try_from(self.address_lo).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 4;
        calc_reloc = u32::try_from(10i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_14(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(token_16(tokens_current)).unwrap());
        let address_lo = token_16(tokens_current);
        let address_hi = token_14(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                address_hi,
                address_lo,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tablejmpdest16 {
    Var0(jmpdest16Var0),
}
impl Tablejmpdest16 {
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
                jmpdest16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:59:1, end:59:13))"]
#[derive(Clone, Debug)]
struct branchdest16Var0 {
    imm16: u16,
}
impl branchdest16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(i128::try_from(self.imm16).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(i128::try_from(token_19(tokens_current)).unwrap());
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:60:1, end:60:13))"]
#[derive(Clone, Debug)]
struct branchdest16Var1 {
    imm16: u16,
}
impl branchdest16Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add((i128::try_from(self.imm16).unwrap() ^ 65535i128));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reloc.is_negative(),
            calc_reloc.abs() as u64,
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
        let mut calc_reloc: i128 = 0;
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        calc_reloc = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add((i128::try_from(token_19(tokens_current)).unwrap() ^ 65535i128));
        let imm16 = token_19(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebranchdest16 {
    Var0(branchdest16Var0),
    Var1(branchdest16Var1),
}
impl Tablebranchdest16 {
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
        if tokens_param.len() >= 4 && (tokens_param[1] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                branchdest16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 4 && (tokens_param[1] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                branchdest16Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:61:1, end:61:11))"]
#[derive(Clone, Debug)]
struct splitimm16Var0 {
    value_hi: u8,
    value_lo: u8,
}
impl splitimm16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_split: i128 = 0;
        calc_split = u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.value_hi).unwrap().checked_shl(shl))
            .unwrap_or(0)
            .wrapping_add(i128::try_from(self.value_lo).unwrap());
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_split.is_negative(),
            calc_split.abs() as u64,
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
        let mut calc_split: i128 = 0;
        let mut block_0_len = 4;
        calc_split = u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_18(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            .wrapping_add(i128::try_from(token_17(tokens_current)).unwrap());
        let value_hi = token_18(tokens_current);
        let value_lo = token_17(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { value_hi, value_lo }))
    }
}
#[derive(Clone, Debug)]
enum Tablesplitimm16 {
    Var0(splitimm16Var0),
}
impl Tablesplitimm16 {
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
                splitimm16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:63:1, end:63:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var0 {
    reg3_5: u8,
}
impl impliedval16Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 0 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 1 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 2 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 3 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 7 {
                    return Some(((), (), 2));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:68:1, end:68:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var1 {
    reg3_5: u8,
}
impl impliedval16Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 4 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 5 {
                    return Some(((), (), 2));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:74:1, end:74:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var2 {
    reg3_5: u8,
}
impl impliedval16Var2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:80:1, end:80:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var3 {
    reg3_5: u8,
}
impl impliedval16Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 4 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 5 {
                    return Some(((), (), 2));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:89:1, end:89:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var4 {
    reg3_5: u8,
}
impl impliedval16Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:98:1, end:98:13))"]
#[derive(Clone, Debug)]
struct impliedval16Var5 {
    reg3_5: u8,
}
impl impliedval16Var5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [meaning_1_display(self.reg3_5)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c34 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 0 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 1 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 2 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 3 {
                    return Some(((), (), 2));
                }
                if token_1(tokens_param) == 7 {
                    return Some(((), (), 2));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c34(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let reg3_5 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { reg3_5 }))
    }
}
#[derive(Clone, Debug)]
enum Tableimpliedval16 {
    Var0(impliedval16Var0),
    Var1(impliedval16Var1),
    Var2(impliedval16Var2),
    Var3(impliedval16Var3),
    Var4(impliedval16Var4),
    Var5(impliedval16Var5),
}
impl Tableimpliedval16 {
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && context_param.0 & 1 == 0 {
            if let Some((inst_len, parsed)) =
                impliedval16Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 1 == 0 && (tokens_param[1] & 48) == 32 {
            if let Some((inst_len, parsed)) =
                impliedval16Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 1 == 0 && (tokens_param[1] & 56) == 48 {
            if let Some((inst_len, parsed)) =
                impliedval16Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 1 == 1 && (tokens_param[1] & 48) == 32 {
            if let Some((inst_len, parsed)) =
                impliedval16Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 1 == 1 && (tokens_param[1] & 56) == 48 {
            if let Some((inst_len, parsed)) =
                impliedval16Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                impliedval16Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:104:1, end:104:12))"]
#[derive(Clone, Debug)]
struct checkbranchVar0 {}
impl checkbranchVar0 {
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:105:1, end:105:12))"]
#[derive(Clone, Debug)]
struct checkbranchVar1 {}
impl checkbranchVar1 {
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
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tablecheckbranch {
    Var0(checkbranchVar0),
    Var1(checkbranchVar1),
}
impl Tablecheckbranch {
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
                checkbranchVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                checkbranchVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:106:1, end:106:10))"]
#[derive(Clone, Debug)]
struct regval0_2Var0 {}
impl regval0_2Var0 {
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:110:1, end:110:10))"]
#[derive(Clone, Debug)]
struct regval0_2Var1 {}
impl regval0_2Var1 {
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
        let reg0_2 = token_2(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tableregval0_2 {
    Var0(regval0_2Var0),
    Var1(regval0_2Var1),
}
impl Tableregval0_2 {
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
                regval0_2Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                regval0_2Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:112:1, end:112:3))"]
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:113:1, end:113:3))"]
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
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:114:1, end:114:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("OV")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:115:1, end:115:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PL")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:116:1, end:116:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("EQ")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:117:1, end:117:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("LT")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:118:1, end:118:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("LE")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:119:1, end:119:3))"]
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("USC")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:120:1, end:120:3))"]
#[derive(Clone, Debug)]
struct ccVar8 {}
impl ccVar8 {
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
        let mut block_0_len = 2;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:121:1, end:121:3))"]
#[derive(Clone, Debug)]
struct ccVar9 {}
impl ccVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NOV")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:122:1, end:122:3))"]
#[derive(Clone, Debug)]
struct ccVar10 {}
impl ccVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("MI")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:123:1, end:123:3))"]
#[derive(Clone, Debug)]
struct ccVar11 {}
impl ccVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("NEQ")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:124:1, end:124:3))"]
#[derive(Clone, Debug)]
struct ccVar12 {}
impl ccVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("GE")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:125:1, end:125:3))"]
#[derive(Clone, Debug)]
struct ccVar13 {}
impl ccVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("GT")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/CP1600/data/languages/CP1600.slaspec, start:126:1, end:126:3))"]
#[derive(Clone, Debug)]
struct ccVar14 {}
impl ccVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("ESC")];
        display.extend_from_slice(&extend);
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
enum Tablecc {
    Var0(ccVar0),
    Var1(ccVar1),
    Var2(ccVar2),
    Var3(ccVar3),
    Var4(ccVar4),
    Var5(ccVar5),
    Var6(ccVar6),
    Var7(ccVar7),
    Var8(ccVar8),
    Var9(ccVar9),
    Var10(ccVar10),
    Var11(ccVar11),
    Var12(ccVar12),
    Var13(ccVar13),
    Var14(ccVar14),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 0 {
            if let Some((inst_len, parsed)) =
                ccVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 1 {
            if let Some((inst_len, parsed)) =
                ccVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 2 {
            if let Some((inst_len, parsed)) =
                ccVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 3 {
            if let Some((inst_len, parsed)) =
                ccVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 4 {
            if let Some((inst_len, parsed)) =
                ccVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 5 {
            if let Some((inst_len, parsed)) =
                ccVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 6 {
            if let Some((inst_len, parsed)) =
                ccVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 7 {
            if let Some((inst_len, parsed)) =
                ccVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 9 {
            if let Some((inst_len, parsed)) =
                ccVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 10 {
            if let Some((inst_len, parsed)) =
                ccVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 11 {
            if let Some((inst_len, parsed)) =
                ccVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 12 {
            if let Some((inst_len, parsed)) =
                ccVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 13 {
            if let Some((inst_len, parsed)) =
                ccVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 14 {
            if let Some((inst_len, parsed)) =
                ccVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 31) == 15 {
            if let Some((inst_len, parsed)) =
                ccVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
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
