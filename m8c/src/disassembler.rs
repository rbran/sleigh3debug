pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    A,
    X,
    SP,
    F,
    PC,
    contextreg,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::X => "X",
            Self::SP => "SP",
            Self::F => "F",
            Self::PC => "PC",
            Self::contextreg => "contextreg",
        }
    }
}
impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
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
#[doc = "Create token_fields: op73"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 3) & 31) as u8)
}
#[doc = "Create token_fields: rsb"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: op4"]
fn token_11(tokens: &[u8]) -> u8 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 12) & 15) as u8)
}
#[doc = "Create token_fields: op8 addr8 imm8 simm8 rel"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: sign"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 7) & 1) as u8)
}
#[doc = "Create token_fields: srel12"]
fn token_12(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: op10"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 3) as u8)
}
#[doc = "Create token_fields: op74"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: op20"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 7) as u8)
}
#[doc = "Create token_fields: op0"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: op71"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 1) & 127) as u8)
}
#[doc = "Create token_fields: imm16 simm16 addr16"]
fn token_10(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u8);
impl ContextMemory {
    pub fn read_regbank(&self) -> u8 {
        (((self.0.reverse_bits() >> 7) & 1) as u8)
    }
    pub fn write_regbank(&mut self, value: u8) {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar0 {
    Imm8: TableImm8,
}
impl ADC_instructionVar0 {
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
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:168:1, end:168:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar1 {
    Addr8: TableAddr8,
}
impl ADC_instructionVar1 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:174:1, end:174:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar2 {
    XAddr8: TableXAddr8,
}
impl ADC_instructionVar2 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:180:1, end:180:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar3 {
    Addr8: TableAddr8,
}
impl ADC_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:187:1, end:187:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar4 {
    XAddr8: TableXAddr8,
}
impl ADC_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:194:1, end:194:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar5 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl ADC_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:202:1, end:202:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar6 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl ADC_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar7 {
    Imm8: TableImm8,
}
impl ADD_instructionVar7 {
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
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:216:1, end:216:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar8 {
    Addr8: TableAddr8,
}
impl ADD_instructionVar8 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar9 {
    XAddr8: TableXAddr8,
}
impl ADD_instructionVar9 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:228:1, end:228:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar10 {
    Addr8: TableAddr8,
}
impl ADD_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:234:1, end:234:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar11 {
    XAddr8: TableXAddr8,
}
impl ADD_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:240:1, end:240:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar12 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl ADD_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADD"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:246:1, end:246:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar13 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:252:1, end:252:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar14 {
    simm8: u8,
}
impl ADD_instructionVar14 {
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
            <DisplayElement>::Register(Register::SP),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:257:1, end:257:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar15 {
    Imm8: TableImm8,
}
impl AND_instructionVar15 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar16 {
    Addr8: TableAddr8,
}
impl AND_instructionVar16 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:269:1, end:269:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar17 {
    XAddr8: TableXAddr8,
}
impl AND_instructionVar17 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:275:1, end:275:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar18 {
    Addr8: TableAddr8,
}
impl AND_instructionVar18 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:282:1, end:282:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar19 {
    XAddr8: TableXAddr8,
}
impl AND_instructionVar19 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:289:1, end:289:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar20 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl AND_instructionVar20 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar21 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl AND_instructionVar21 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:303:1, end:303:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar22 {
    RAddr8: TableRAddr8,
    Imm8: TableImm8,
}
impl AND_instructionVar22 {
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
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:310:1, end:310:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar23 {
    RXAddr8: TableRXAddr8,
    Imm8: TableImm8,
}
impl AND_instructionVar23 {
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
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:317:1, end:317:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar24 {
    imm8: u8,
    rsb: u8,
}
impl AND_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_regbank(
                u8::try_from(i128::try_from(context.read_regbank()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("AND"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::F),
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
        context_instance.write_regbank(
            u8::try_from(
                (i128::try_from(context_instance.read_regbank()).unwrap()
                    & i128::try_from(token_8(tokens_current)).unwrap())
                    & 1,
            )
            .unwrap(),
        );
        let imm8 = token_1(tokens_current);
        let rsb = token_8(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8, rsb }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:323:1, end:323:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar25 {}
impl ASL_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASL"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:328:1, end:328:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar26 {
    Addr8: TableAddr8,
}
impl ASL_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:333:1, end:333:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar27 {
    XAddr8: TableXAddr8,
}
impl ASL_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:338:1, end:338:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar28 {}
impl ASR_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASR"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:343:1, end:343:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar29 {
    Addr8: TableAddr8,
}
impl ASR_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:348:1, end:348:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar30 {
    XAddr8: TableXAddr8,
}
impl ASR_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:360:1, end:360:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar31 {
    Imm8: TableImm8,
}
impl CMP_instructionVar31 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:365:1, end:365:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar32 {
    Addr8: TableAddr8,
}
impl CMP_instructionVar32 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:371:1, end:371:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar33 {
    XAddr8: TableXAddr8,
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
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:376:1, end:376:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar34 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl CMP_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:381:1, end:381:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar35 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl CMP_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CMP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct CPL_instructionVar36 {}
impl CPL_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPL"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:398:1, end:398:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar37 {
    Addr8: TableAddr8,
}
impl DEC_instructionVar37 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:404:1, end:404:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar38 {
    XAddr8: TableXAddr8,
}
impl DEC_instructionVar38 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:410:1, end:410:2))"]
#[derive(Clone, Debug)]
struct HALT_instructionVar39 {}
impl HALT_instructionVar39 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:421:1, end:421:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar40 {
    Addr8: TableAddr8,
}
impl INC_instructionVar40 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:427:1, end:427:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar41 {
    XAddr8: TableXAddr8,
}
impl INC_instructionVar41 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:471:1, end:471:2))"]
#[derive(Clone, Debug)]
struct LCALL_instructionVar42 {
    Addr16: TableAddr16,
}
impl LCALL_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LCALL"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:478:1, end:478:2))"]
#[derive(Clone, Debug)]
struct LJMP_instructionVar43 {
    Addr16: TableAddr16,
}
impl LJMP_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LJMP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:483:1, end:483:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar44 {}
impl MOV_instructionVar44 {
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
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SP),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:488:1, end:488:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar45 {
    Imm8: TableImm8,
}
impl MOV_instructionVar45 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:494:1, end:494:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar46 {
    Addr8: TableAddr8,
}
impl MOV_instructionVar46 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:500:1, end:500:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar47 {
    XAddr8: TableXAddr8,
}
impl MOV_instructionVar47 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:506:1, end:506:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar48 {
    Addr8: TableAddr8,
}
impl MOV_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:511:1, end:511:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar49 {
    XAddr8: TableXAddr8,
}
impl MOV_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:516:1, end:516:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar50 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl MOV_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:521:1, end:521:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar51 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl MOV_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:526:1, end:526:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar52 {
    Imm8: TableImm8,
}
impl MOV_instructionVar52 {
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
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:531:1, end:531:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar53 {
    Addr8: TableAddr8,
}
impl MOV_instructionVar53 {
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
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:536:1, end:536:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar54 {
    XAddr8: TableXAddr8,
}
impl MOV_instructionVar54 {
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
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar55 {
    Addr8: TableAddr8,
}
impl MOV_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:546:1, end:546:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar56 {}
impl MOV_instructionVar56 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:552:1, end:552:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar57 {}
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
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:557:1, end:557:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar58 {
    RAddr8: TableRAddr8,
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
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.RAddr8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:563:1, end:563:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar59 {
    RXAddr8: TableRXAddr8,
}
impl MOV_instructionVar59 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.RXAddr8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:569:1, end:569:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar60 {
    Addr8: TableAddr8,
    SAddr8: TableSAddr8,
}
impl MOV_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.SAddr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let SAddr8 = if let Some((len, table)) =
            TableSAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, SAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:574:1, end:574:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar61 {
    RAddr8: TableRAddr8,
}
impl MOV_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:579:1, end:579:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar62 {
    RXAddr8: TableRXAddr8,
}
impl MOV_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:584:1, end:584:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar63 {
    RAddr8: TableRAddr8,
    Imm8: TableImm8,
}
impl MOV_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:589:1, end:589:2))"]
#[derive(Clone, Debug)]
struct MOV_instructionVar64 {
    RXAddr8: TableRXAddr8,
    Imm8: TableImm8,
}
impl MOV_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOV"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:594:1, end:594:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar65 {
    Addr8Incr: TableAddr8Incr,
}
impl MVI_instructionVar65 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8Incr
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
        let Addr8Incr = if let Some((len, table)) =
            TableAddr8Incr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8Incr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:602:1, end:602:2))"]
#[derive(Clone, Debug)]
struct MVI_instructionVar66 {
    Addr8Incr: TableAddr8Incr,
}
impl MVI_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MVI"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8Incr
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8Incr = if let Some((len, table)) =
            TableAddr8Incr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8Incr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:609:1, end:609:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar67 {}
impl NOP_instructionVar67 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:614:1, end:614:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar68 {
    Imm8: TableImm8,
}
impl OR_instructionVar68 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:620:1, end:620:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar69 {
    Addr8: TableAddr8,
}
impl OR_instructionVar69 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:626:1, end:626:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar70 {
    XAddr8: TableXAddr8,
}
impl OR_instructionVar70 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:632:1, end:632:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar71 {
    Addr8: TableAddr8,
}
impl OR_instructionVar71 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:639:1, end:639:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar72 {
    XAddr8: TableXAddr8,
}
impl OR_instructionVar72 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:646:1, end:646:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar73 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl OR_instructionVar73 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:653:1, end:653:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar74 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl OR_instructionVar74 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:660:1, end:660:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar75 {
    RAddr8: TableRAddr8,
    Imm8: TableImm8,
}
impl OR_instructionVar75 {
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
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:667:1, end:667:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar76 {
    RXAddr8: TableRXAddr8,
    Imm8: TableImm8,
}
impl OR_instructionVar76 {
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
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:674:1, end:674:2))"]
#[derive(Clone, Debug)]
struct OR_instructionVar77 {
    imm8: u8,
    rsb: u8,
}
impl OR_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_regbank(
                u8::try_from(i128::try_from(context.read_regbank()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("OR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::F),
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
        context_instance.write_regbank(
            u8::try_from(
                (i128::try_from(context_instance.read_regbank()).unwrap()
                    | i128::try_from(token_8(tokens_current)).unwrap())
                    & 1,
            )
            .unwrap(),
        );
        let imm8 = token_1(tokens_current);
        let rsb = token_8(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8, rsb }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:680:1, end:680:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar78 {}
impl POP_instructionVar78 {
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:685:1, end:685:2))"]
#[derive(Clone, Debug)]
struct POP_instructionVar79 {}
impl POP_instructionVar79 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:690:1, end:690:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar80 {}
impl PUSH_instructionVar80 {
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:695:1, end:695:2))"]
#[derive(Clone, Debug)]
struct PUSH_instructionVar81 {}
impl PUSH_instructionVar81 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:700:1, end:700:2))"]
#[derive(Clone, Debug)]
struct RETI_instructionVar82 {}
impl RETI_instructionVar82 {
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
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:707:1, end:707:2))"]
#[derive(Clone, Debug)]
struct RET_instructionVar83 {}
impl RET_instructionVar83 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:714:1, end:714:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar84 {}
impl RLC_instructionVar84 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:721:1, end:721:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar85 {
    Addr8: TableAddr8,
}
impl RLC_instructionVar85 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:729:1, end:729:2))"]
#[derive(Clone, Debug)]
struct RLC_instructionVar86 {
    XAddr8: TableXAddr8,
}
impl RLC_instructionVar86 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:737:1, end:737:2))"]
#[derive(Clone, Debug)]
struct ROMX_instructionVar87 {}
impl ROMX_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ROMX"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:745:1, end:745:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar88 {}
impl RRC_instructionVar88 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:752:1, end:752:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar89 {
    Addr8: TableAddr8,
}
impl RRC_instructionVar89 {
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
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:760:1, end:760:2))"]
#[derive(Clone, Debug)]
struct RRC_instructionVar90 {
    XAddr8: TableXAddr8,
}
impl RRC_instructionVar90 {
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
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:768:1, end:768:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar91 {
    Imm8: TableImm8,
}
impl SBB_instructionVar91 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:774:1, end:774:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar92 {
    Addr8: TableAddr8,
}
impl SBB_instructionVar92 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:780:1, end:780:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar93 {
    XAddr8: TableXAddr8,
}
impl SBB_instructionVar93 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:786:1, end:786:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar94 {
    Addr8: TableAddr8,
}
impl SBB_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:793:1, end:793:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar95 {
    XAddr8: TableXAddr8,
}
impl SBB_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:800:1, end:800:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar96 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl SBB_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:807:1, end:807:2))"]
#[derive(Clone, Debug)]
struct SBB_instructionVar97 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl SBB_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBB"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:814:1, end:814:2))"]
#[derive(Clone, Debug)]
struct SSC_instructionVar98 {}
impl SSC_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SSC"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:819:1, end:819:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar99 {
    Imm8: TableImm8,
}
impl SUB_instructionVar99 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:825:1, end:825:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar100 {
    Addr8: TableAddr8,
}
impl SUB_instructionVar100 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:831:1, end:831:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar101 {
    XAddr8: TableXAddr8,
}
impl SUB_instructionVar101 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:837:1, end:837:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar102 {
    Addr8: TableAddr8,
}
impl SUB_instructionVar102 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:844:1, end:844:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar103 {
    XAddr8: TableXAddr8,
}
impl SUB_instructionVar103 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:851:1, end:851:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar104 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl SUB_instructionVar104 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:858:1, end:858:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar105 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl SUB_instructionVar105 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:865:1, end:865:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar106 {}
impl SWAP_instructionVar106 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:879:1, end:879:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar107 {}
impl SWAP_instructionVar107 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::SP),
        ];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:886:1, end:886:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar108 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl TST_instructionVar108 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:892:1, end:892:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar109 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl TST_instructionVar109 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:898:1, end:898:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar110 {
    RAddr8: TableRAddr8,
    Imm8: TableImm8,
}
impl TST_instructionVar110 {
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
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:904:1, end:904:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar111 {
    RXAddr8: TableRXAddr8,
    Imm8: TableImm8,
}
impl TST_instructionVar111 {
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
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:910:1, end:910:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar112 {
    Imm8: TableImm8,
}
impl XOR_instructionVar112 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:916:1, end:916:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar113 {
    Addr8: TableAddr8,
}
impl XOR_instructionVar113 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:922:1, end:922:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar114 {
    XAddr8: TableXAddr8,
}
impl XOR_instructionVar114 {
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
            <DisplayElement>::Register(Register::A),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.XAddr8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:928:1, end:928:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar115 {
    Addr8: TableAddr8,
}
impl XOR_instructionVar115 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:935:1, end:935:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar116 {
    XAddr8: TableXAddr8,
}
impl XOR_instructionVar116 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:942:1, end:942:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar117 {
    Addr8: TableAddr8,
    Imm8: TableImm8,
}
impl XOR_instructionVar117 {
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
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:949:1, end:949:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar118 {
    XAddr8: TableXAddr8,
    Imm8: TableImm8,
}
impl XOR_instructionVar118 {
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
        self.XAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let XAddr8 = if let Some((len, table)) =
            TableXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { XAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:956:1, end:956:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar119 {
    RAddr8: TableRAddr8,
    Imm8: TableImm8,
}
impl XOR_instructionVar119 {
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
        self.RAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RAddr8 = if let Some((len, table)) =
            TableRAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:963:1, end:963:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar120 {
    RXAddr8: TableRXAddr8,
    Imm8: TableImm8,
}
impl XOR_instructionVar120 {
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
        self.RXAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Imm8
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
        let RXAddr8 = if let Some((len, table)) =
            TableRXAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let Imm8 = if let Some((len, table)) =
            TableImm8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RXAddr8, Imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:970:1, end:970:2))"]
#[derive(Clone, Debug)]
struct XOR_instructionVar121 {
    imm8: u8,
    rsb: u8,
}
impl XOR_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        global_set.set(Some(inst_next), |context| {
            context.write_regbank(
                u8::try_from(i128::try_from(context.read_regbank()).unwrap() & 1).unwrap(),
            )
        });
        display.push(DisplayElement::Literal("XOR"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Register(Register::F),
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
        context_instance.write_regbank(
            u8::try_from(
                (i128::try_from(context_instance.read_regbank()).unwrap()
                    ^ i128::try_from(token_8(tokens_current)).unwrap())
                    & 1,
            )
            .unwrap(),
        );
        let rsb = token_8(tokens_current);
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8, rsb }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:392:1, end:392:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar122 {
    regAorX: TableregAorX,
}
impl DEC_instructionVar122 {
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
        self.regAorX
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
        let regAorX = if let Some((len, table)) =
            TableregAorX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { regAorX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:415:1, end:415:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar123 {
    regAorX: TableregAorX,
}
impl INC_instructionVar123 {
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
        self.regAorX
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
        let regAorX = if let Some((len, table)) =
            TableregAorX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { regAorX }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:872:1, end:872:2))"]
#[derive(Clone, Debug)]
struct SWAP_instructionVar124 {
    regAorX: TableregAorX,
    Addr8: TableAddr8,
}
impl SWAP_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SWAP"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.regAorX
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.Addr8
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
        let regAorX = if let Some((len, table)) =
            TableregAorX::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { regAorX, Addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:353:1, end:353:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar125 {
    CallAddr: TableCallAddr,
}
impl CALL_instructionVar125 {
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
        self.CallAddr
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
        let CallAddr = if let Some((len, table)) =
            TableCallAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { CallAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:433:1, end:433:2))"]
#[derive(Clone, Debug)]
struct INDEX_instructionVar126 {
    IndexAddr: TableIndexAddr,
}
impl INDEX_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INDEX"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.IndexAddr
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
        let IndexAddr = if let Some((len, table)) =
            TableIndexAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let srel12 = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { IndexAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:439:1, end:439:2))"]
#[derive(Clone, Debug)]
struct JACC_instructionVar127 {
    RelAddr: TableRelAddr,
}
impl JACC_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JACC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:446:1, end:446:2))"]
#[derive(Clone, Debug)]
struct JC_instructionVar128 {
    RelAddr: TableRelAddr,
}
impl JC_instructionVar128 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:451:1, end:451:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar129 {
    RelAddr: TableRelAddr,
}
impl JMP_instructionVar129 {
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
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:456:1, end:456:2))"]
#[derive(Clone, Debug)]
struct JNC_instructionVar130 {
    RelAddr: TableRelAddr,
}
impl JNC_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JNC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct JNZ_instructionVar131 {
    RelAddr: TableRelAddr,
}
impl JNZ_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JNZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:466:1, end:466:2))"]
#[derive(Clone, Debug)]
struct JZ_instructionVar132 {
    RelAddr: TableRelAddr,
}
impl JZ_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.RelAddr
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
        let RelAddr = if let Some((len, table)) =
            TableRelAddr::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { RelAddr }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(ADC_instructionVar0),
    Var1(ADC_instructionVar1),
    Var2(ADC_instructionVar2),
    Var3(ADC_instructionVar3),
    Var4(ADC_instructionVar4),
    Var5(ADC_instructionVar5),
    Var6(ADC_instructionVar6),
    Var7(ADD_instructionVar7),
    Var8(ADD_instructionVar8),
    Var9(ADD_instructionVar9),
    Var10(ADD_instructionVar10),
    Var11(ADD_instructionVar11),
    Var12(ADD_instructionVar12),
    Var13(ADD_instructionVar13),
    Var14(ADD_instructionVar14),
    Var15(AND_instructionVar15),
    Var16(AND_instructionVar16),
    Var17(AND_instructionVar17),
    Var18(AND_instructionVar18),
    Var19(AND_instructionVar19),
    Var20(AND_instructionVar20),
    Var21(AND_instructionVar21),
    Var22(AND_instructionVar22),
    Var23(AND_instructionVar23),
    Var24(AND_instructionVar24),
    Var25(ASL_instructionVar25),
    Var26(ASL_instructionVar26),
    Var27(ASL_instructionVar27),
    Var28(ASR_instructionVar28),
    Var29(ASR_instructionVar29),
    Var30(ASR_instructionVar30),
    Var31(CMP_instructionVar31),
    Var32(CMP_instructionVar32),
    Var33(CMP_instructionVar33),
    Var34(CMP_instructionVar34),
    Var35(CMP_instructionVar35),
    Var36(CPL_instructionVar36),
    Var37(DEC_instructionVar37),
    Var38(DEC_instructionVar38),
    Var39(HALT_instructionVar39),
    Var40(INC_instructionVar40),
    Var41(INC_instructionVar41),
    Var42(LCALL_instructionVar42),
    Var43(LJMP_instructionVar43),
    Var44(MOV_instructionVar44),
    Var45(MOV_instructionVar45),
    Var46(MOV_instructionVar46),
    Var47(MOV_instructionVar47),
    Var48(MOV_instructionVar48),
    Var49(MOV_instructionVar49),
    Var50(MOV_instructionVar50),
    Var51(MOV_instructionVar51),
    Var52(MOV_instructionVar52),
    Var53(MOV_instructionVar53),
    Var54(MOV_instructionVar54),
    Var55(MOV_instructionVar55),
    Var56(MOV_instructionVar56),
    Var57(MOV_instructionVar57),
    Var58(MOV_instructionVar58),
    Var59(MOV_instructionVar59),
    Var60(MOV_instructionVar60),
    Var61(MOV_instructionVar61),
    Var62(MOV_instructionVar62),
    Var63(MOV_instructionVar63),
    Var64(MOV_instructionVar64),
    Var65(MVI_instructionVar65),
    Var66(MVI_instructionVar66),
    Var67(NOP_instructionVar67),
    Var68(OR_instructionVar68),
    Var69(OR_instructionVar69),
    Var70(OR_instructionVar70),
    Var71(OR_instructionVar71),
    Var72(OR_instructionVar72),
    Var73(OR_instructionVar73),
    Var74(OR_instructionVar74),
    Var75(OR_instructionVar75),
    Var76(OR_instructionVar76),
    Var77(OR_instructionVar77),
    Var78(POP_instructionVar78),
    Var79(POP_instructionVar79),
    Var80(PUSH_instructionVar80),
    Var81(PUSH_instructionVar81),
    Var82(RETI_instructionVar82),
    Var83(RET_instructionVar83),
    Var84(RLC_instructionVar84),
    Var85(RLC_instructionVar85),
    Var86(RLC_instructionVar86),
    Var87(ROMX_instructionVar87),
    Var88(RRC_instructionVar88),
    Var89(RRC_instructionVar89),
    Var90(RRC_instructionVar90),
    Var91(SBB_instructionVar91),
    Var92(SBB_instructionVar92),
    Var93(SBB_instructionVar93),
    Var94(SBB_instructionVar94),
    Var95(SBB_instructionVar95),
    Var96(SBB_instructionVar96),
    Var97(SBB_instructionVar97),
    Var98(SSC_instructionVar98),
    Var99(SUB_instructionVar99),
    Var100(SUB_instructionVar100),
    Var101(SUB_instructionVar101),
    Var102(SUB_instructionVar102),
    Var103(SUB_instructionVar103),
    Var104(SUB_instructionVar104),
    Var105(SUB_instructionVar105),
    Var106(SWAP_instructionVar106),
    Var107(SWAP_instructionVar107),
    Var108(TST_instructionVar108),
    Var109(TST_instructionVar109),
    Var110(TST_instructionVar110),
    Var111(TST_instructionVar111),
    Var112(XOR_instructionVar112),
    Var113(XOR_instructionVar113),
    Var114(XOR_instructionVar114),
    Var115(XOR_instructionVar115),
    Var116(XOR_instructionVar116),
    Var117(XOR_instructionVar117),
    Var118(XOR_instructionVar118),
    Var119(XOR_instructionVar119),
    Var120(XOR_instructionVar120),
    Var121(XOR_instructionVar121),
    Var122(DEC_instructionVar122),
    Var123(INC_instructionVar123),
    Var124(SWAP_instructionVar124),
    Var125(CALL_instructionVar125),
    Var126(INDEX_instructionVar126),
    Var127(JACC_instructionVar127),
    Var128(JC_instructionVar128),
    Var129(JMP_instructionVar129),
    Var130(JNC_instructionVar130),
    Var131(JNZ_instructionVar131),
    Var132(JZ_instructionVar132),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 9 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 10 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 11 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 12 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 13 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 14 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 15 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 1 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 2 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 3 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 4 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 5 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 6 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 7 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 56 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 33 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 34 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 35 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 36 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 37 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 38 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 39 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 65 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 66 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 112 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 100 {
            if let Some((inst_len, parsed)) =
                ASL_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 101 {
            if let Some((inst_len, parsed)) =
                ASL_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 102 {
            if let Some((inst_len, parsed)) =
                ASL_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 103 {
            if let Some((inst_len, parsed)) =
                ASR_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 104 {
            if let Some((inst_len, parsed)) =
                ASR_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 105 {
            if let Some((inst_len, parsed)) =
                ASR_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 57 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 58 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 59 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 60 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 61 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 115 {
            if let Some((inst_len, parsed)) =
                CPL_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 122 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 123 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 48 {
            if let Some((inst_len, parsed)) =
                HALT_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 118 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 119 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 124 {
            if let Some((inst_len, parsed)) =
                LCALL_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 125 {
            if let Some((inst_len, parsed)) =
                LJMP_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 79 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 80 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 81 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 82 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 83 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 84 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 85 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 86 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 87 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 88 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 89 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 90 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 91 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 92 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 93 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 94 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 95 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 96 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 97 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 98 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 99 {
            if let Some((inst_len, parsed)) =
                MOV_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 62 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 63 {
            if let Some((inst_len, parsed)) =
                MVI_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 64 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 41 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 42 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 43 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 44 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 45 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 46 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 47 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 67 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 68 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 113 {
            if let Some((inst_len, parsed)) =
                OR_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 32 {
            if let Some((inst_len, parsed)) =
                POP_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 24 {
            if let Some((inst_len, parsed)) =
                POP_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 16 {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 8 {
            if let Some((inst_len, parsed)) =
                PUSH_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 126 {
            if let Some((inst_len, parsed)) =
                RETI_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 127 {
            if let Some((inst_len, parsed)) =
                RET_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 106 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 107 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 108 {
            if let Some((inst_len, parsed)) =
                RLC_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 40 {
            if let Some((inst_len, parsed)) =
                ROMX_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 109 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 110 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 111 {
            if let Some((inst_len, parsed)) =
                RRC_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 25 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 26 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 27 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 28 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 29 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 30 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 31 {
            if let Some((inst_len, parsed)) =
                SBB_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 0 {
            if let Some((inst_len, parsed)) =
                SSC_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 17 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 18 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 19 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 20 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 21 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 22 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 23 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 75 {
            if let Some((inst_len, parsed)) =
                SWAP_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 78 {
            if let Some((inst_len, parsed)) =
                SWAP_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 71 {
            if let Some((inst_len, parsed)) =
                TST_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 72 {
            if let Some((inst_len, parsed)) =
                TST_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 73 {
            if let Some((inst_len, parsed)) =
                TST_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 74 {
            if let Some((inst_len, parsed)) =
                TST_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 49 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 50 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 51 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 52 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 53 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 54 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 55 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 69 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 255) == 70 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 114 {
            if let Some((inst_len, parsed)) =
                XOR_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 254) == 120 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 254) == 116 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 254) == 76 {
            if let Some((inst_len, parsed)) =
                SWAP_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 144 {
            if let Some((inst_len, parsed)) =
                CALL_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 240 {
            if let Some((inst_len, parsed)) =
                INDEX_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 224 {
            if let Some((inst_len, parsed)) =
                JACC_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 192 {
            if let Some((inst_len, parsed)) =
                JC_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 128 {
            if let Some((inst_len, parsed)) =
                JMP_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 208 {
            if let Some((inst_len, parsed)) =
                JNC_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 176 {
            if let Some((inst_len, parsed)) =
                JNZ_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 240) == 160 {
            if let Some((inst_len, parsed)) =
                JZ_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:138:1, end:138:8))"]
#[derive(Clone, Debug)]
struct regAorXVar0 {}
impl regAorXVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::A)];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:139:1, end:139:8))"]
#[derive(Clone, Debug)]
struct regAorXVar1 {}
impl regAorXVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Register(Register::X)];
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
enum TableregAorX {
    Var0(regAorXVar0),
    Var1(regAorXVar1),
}
impl TableregAorX {
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
        if tokens_param.len() >= 1 && (tokens_param[0] & 1) == 0 {
            if let Some((inst_len, parsed)) =
                regAorXVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 1) == 1 {
            if let Some((inst_len, parsed)) =
                regAorXVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:140:1, end:140:6))"]
#[derive(Clone, Debug)]
struct Addr8Var0 {
    addr8: u8,
}
impl Addr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.addr8 as u64)];
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
        let addr8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr8 }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr8 {
    Var0(Addr8Var0),
}
impl TableAddr8 {
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
                Addr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:141:1, end:141:7))"]
#[derive(Clone, Debug)]
struct SAddr8Var0 {
    addr8: u8,
}
impl SAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.addr8 as u64)];
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
        let addr8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr8 }))
    }
}
#[derive(Clone, Debug)]
enum TableSAddr8 {
    Var0(SAddr8Var0),
}
impl TableSAddr8 {
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
                SAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:142:1, end:142:7))"]
#[derive(Clone, Debug)]
struct XAddr8Var0 {
    simm8: u8,
}
impl XAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:143:1, end:143:7))"]
#[derive(Clone, Debug)]
struct XAddr8Var1 {
    simm8: u8,
}
impl XAddr8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableXAddr8 {
    Var0(XAddr8Var0),
    Var1(XAddr8Var1),
}
impl TableXAddr8 {
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
        if tokens_param.len() >= 1 && (tokens_param[0] & 128) == 0 {
            if let Some((inst_len, parsed)) =
                XAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 128) == 128 {
            if let Some((inst_len, parsed)) =
                XAddr8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:144:1, end:144:10))"]
#[derive(Clone, Debug)]
struct Addr8IncrVar0 {
    Addr8: TableAddr8,
}
impl Addr8IncrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("[")];
        display.extend_from_slice(&extend);
        self.Addr8
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("]++")];
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
        let Addr8 = if let Some((len, table)) =
            TableAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Addr8 }))
    }
}
#[derive(Clone, Debug)]
enum TableAddr8Incr {
    Var0(Addr8IncrVar0),
}
impl TableAddr8Incr {
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
                Addr8IncrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:146:1, end:146:7))"]
#[derive(Clone, Debug)]
struct RAddr8Var0 {
    addr8: u8,
}
impl RAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.addr8 as u64)];
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
        let addr8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:147:1, end:147:7))"]
#[derive(Clone, Debug)]
struct RAddr8Var1 {
    addr8: u8,
}
impl RAddr8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.addr8 as u64)];
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
        let addr8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr8 }))
    }
}
#[derive(Clone, Debug)]
enum TableRAddr8 {
    Var0(RAddr8Var0),
    Var1(RAddr8Var1),
}
impl TableRAddr8 {
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
                RAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                RAddr8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:148:1, end:148:8))"]
#[derive(Clone, Debug)]
struct RXAddr8Var0 {
    simm8: u8,
}
impl RXAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:149:1, end:149:8))"]
#[derive(Clone, Debug)]
struct RXAddr8Var1 {
    simm8: u8,
}
impl RXAddr8Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal("["),
            <DisplayElement>::Register(Register::X),
            <DisplayElement>::Literal("+"),
            DisplayElement::Number(
                true,
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8)
                    .is_negative(),
                (if self.simm8 & 128 != 0 { -1 & !127 } else { 0 } | self.simm8 as i8).abs() as u64,
            ),
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
        let mut block_0_len = 1;
        let simm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { simm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableRXAddr8 {
    Var0(RXAddr8Var0),
    Var1(RXAddr8Var1),
}
impl TableRXAddr8 {
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
                RXAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                RXAddr8Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:151:1, end:151:5))"]
#[derive(Clone, Debug)]
struct Imm8Var0 {
    imm8: u8,
}
impl Imm8Var0 {
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
        let imm8 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[derive(Clone, Debug)]
enum TableImm8 {
    Var0(Imm8Var0),
}
impl TableImm8 {
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
                Imm8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:153:1, end:153:7))"]
#[derive(Clone, Debug)]
struct Addr16Var0 {
    addr16: u16,
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
            [DisplayElement::Number(true, false, self.addr16 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let addr16 = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { addr16 }))
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:154:1, end:154:8))"]
#[derive(Clone, Debug)]
struct RelAddrVar0 {
    srel12: u16,
}
impl RelAddrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reladdr: i128 = 0;
        calc_reladdr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(1i128)
            .wrapping_add(
                i128::try_from(
                    (if self.srel12 & 2048 != 0 {
                        -1 & !2047
                    } else {
                        0
                    } | self.srel12 as i16),
                )
                .unwrap(),
            );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_reladdr.is_negative(),
            calc_reladdr.abs() as u64,
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
        let mut calc_reladdr: i128 = 0;
        let mut block_0_len = 2;
        calc_reladdr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(1i128)
            .wrapping_add(i128::try_from(token_12(tokens_current)).unwrap());
        let srel12 = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srel12 }))
    }
}
#[derive(Clone, Debug)]
enum TableRelAddr {
    Var0(RelAddrVar0),
}
impl TableRelAddr {
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
                RelAddrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:155:1, end:155:9))"]
#[derive(Clone, Debug)]
struct CallAddrVar0 {
    srel12: u16,
}
impl CallAddrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_calladdr: i128 = 0;
        calc_calladdr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(
                i128::try_from(
                    (if self.srel12 & 2048 != 0 {
                        -1 & !2047
                    } else {
                        0
                    } | self.srel12 as i16),
                )
                .unwrap(),
            );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_calladdr.is_negative(),
            calc_calladdr.abs() as u64,
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
        let mut calc_calladdr: i128 = 0;
        let mut block_0_len = 2;
        calc_calladdr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(i128::try_from(token_12(tokens_current)).unwrap());
        let srel12 = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srel12 }))
    }
}
#[derive(Clone, Debug)]
enum TableCallAddr {
    Var0(CallAddrVar0),
}
impl TableCallAddr {
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
                CallAddrVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/M8C/data/languages/m8c.slaspec, start:156:1, end:156:10))"]
#[derive(Clone, Debug)]
struct IndexAddrVar0 {
    srel12: u16,
}
impl IndexAddrVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_indexaddr: i128 = 0;
        calc_indexaddr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(
                i128::try_from(
                    (if self.srel12 & 2048 != 0 {
                        -1 & !2047
                    } else {
                        0
                    } | self.srel12 as i16),
                )
                .unwrap(),
            );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_indexaddr.is_negative(),
            calc_indexaddr.abs() as u64,
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
        let mut calc_indexaddr: i128 = 0;
        let mut block_0_len = 2;
        calc_indexaddr = i128::try_from(inst_start)
            .unwrap()
            .wrapping_add(2i128)
            .wrapping_add(i128::try_from(token_12(tokens_current)).unwrap());
        let srel12 = token_12(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srel12 }))
    }
}
#[derive(Clone, Debug)]
enum TableIndexAddr {
    Var0(IndexAddrVar0),
}
impl TableIndexAddr {
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
                IndexAddrVar0::parse(tokens_param, &mut context_current, inst_start)
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
