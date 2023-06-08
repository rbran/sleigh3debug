pub type AddrType = u32;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    cat2_return_value,
    return_value,
    SP,
    PC,
    switch_target,
    return_address,
    call_target,
    LVA,
    switch_ctrl,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::cat2_return_value => "cat2_return_value",
            Self::return_value => "return_value",
            Self::SP => "SP",
            Self::PC => "PC",
            Self::switch_target => "switch_target",
            Self::return_address => "return_address",
            Self::call_target => "call_target",
            Self::LVA => "LVA",
            Self::switch_ctrl => "switch_ctrl",
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
#[doc = "Create token_fields: n"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 15) as u8)
}
#[doc = "Create token_fields: m"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 15) as u8)
}
#[doc = "Create token_fields: w_op"]
fn token_2(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: op atype byte byte1 byte2 byte3 byte4 sbyte branch branchbyte1 branchbyte2 branchbyte3 branchbyte4 index indexbyte1 indexbyte2 constant constantbyte1 constantbyte2 nargs method defaultbyte1 defaultbyte2 defaultbyte3 defaultbyte4 highbyte1 highbyte2 highbyte3 highbyte4 lowbyte1 lowbyte2 lowbyte3 lowbyte4 npairsbyte1 npairsbyte2 npairsbyte3 npairsbyte4 dimensions blank1 blank2 count pad pad1 pad2 pad3 wide_op matchbyte1 matchbyte2 matchbyte3 matchbyte4 offsetbyte1 offsetbyte2 offsetbyte3 offsetbyte4"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u128);
impl ContextMemory {
    pub fn read_switch_low(&self) -> u32 {
        (((self.0.reverse_bits() >> 96) & 4294967295) as u32)
    }
    pub fn write_switch_low(&mut self, value: u32) {
        self.0 = ((self.0.reverse_bits() & !(4294967295 << 96))
            | ((value as u128 & 4294967295) << 96))
            .reverse_bits();
    }
    pub fn read_switch_high(&self) -> u32 {
        (((self.0.reverse_bits() >> 64) & 4294967295) as u32)
    }
    pub fn write_switch_high(&mut self, value: u32) {
        self.0 = ((self.0.reverse_bits() & !(4294967295 << 64))
            | ((value as u128 & 4294967295) << 64))
            .reverse_bits();
    }
    pub fn read_switch_num(&self) -> u32 {
        (((self.0.reverse_bits() >> 32) & 4294967295) as u32)
    }
    pub fn write_switch_num(&mut self, value: u32) {
        self.0 = ((self.0.reverse_bits() & !(4294967295 << 32))
            | ((value as u128 & 4294967295) << 32))
            .reverse_bits();
    }
    pub fn read_in_table_switch(&self) -> u8 {
        (((self.0.reverse_bits() >> 30) & 3) as u8)
    }
    pub fn write_in_table_switch(&mut self, value: u8) {
        self.0 =
            ((self.0.reverse_bits() & !(3 << 30)) | ((value as u128 & 3) << 30)).reverse_bits();
    }
    pub fn read_in_lookup_switch(&self) -> u8 {
        (((self.0.reverse_bits() >> 28) & 3) as u8)
    }
    pub fn write_in_lookup_switch(&mut self, value: u8) {
        self.0 =
            ((self.0.reverse_bits() & !(3 << 28)) | ((value as u128 & 3) << 28)).reverse_bits();
    }
    pub fn read_alignmentPad(&self) -> u8 {
        (((self.0.reverse_bits() >> 26) & 3) as u8)
    }
    pub fn write_alignmentPad(&mut self, value: u8) {
        self.0 =
            ((self.0.reverse_bits() & !(3 << 26)) | ((value as u128 & 3) << 26)).reverse_bits();
    }
    pub fn read_padVal(&self) -> u8 {
        (((self.0.reverse_bits() >> 26) & 3) as u8)
    }
    pub fn write_padVal(&mut self, value: u8) {
        self.0 =
            ((self.0.reverse_bits() & !(3 << 26)) | ((value as u128 & 3) << 26)).reverse_bits();
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1757:1, end:1757:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar0 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lookupswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dolookupswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 8;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1761:1, end:1761:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar1 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lookupswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dolookupswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 8;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1765:1, end:1765:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar2 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lookupswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dolookupswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 8;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_4_len = block_4_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1769:1, end:1769:2))"]
#[derive(Clone, Debug)]
struct lookupswitch_instructionVar3 {
    dolookupswitch: Tabledolookupswitch,
    instruction: Box<Tableinstruction>,
}
impl lookupswitch_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lookupswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dolookupswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let pad3 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 8;
        let dolookupswitch = if let Some((len, table)) =
            Tabledolookupswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_4_len = block_4_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_5_len = block_5_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_5_len;
        tokens_current = &tokens_current[usize::try_from(block_5_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dolookupswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2048:1, end:2048:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar4 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tableswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dotableswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 12;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2052:1, end:2052:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar5 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tableswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dotableswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 12;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2056:1, end:2056:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar6 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tableswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dotableswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 12;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_3_len = block_3_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_4_len = block_4_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2060:1, end:2060:2))"]
#[derive(Clone, Debug)]
struct tableswitch_instructionVar7 {
    dotableswitch: Tabledotableswitch,
    instruction: Box<Tableinstruction>,
}
impl tableswitch_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("tableswitch"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.dotableswitch
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let pad3 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 12;
        let dotableswitch = if let Some((len, table)) =
            Tabledotableswitch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_4_len = block_4_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_5_len = block_5_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_5_len;
        tokens_current = &tokens_current[usize::try_from(block_5_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                dotableswitch,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2066:1, end:2066:2))"]
#[derive(Clone, Debug)]
struct wide_iload_instructionVar8 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_iload_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_iload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50197 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2073:1, end:2073:2))"]
#[derive(Clone, Debug)]
struct wide_fload_instructionVar9 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_fload_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_fload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50199 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2080:1, end:2080:2))"]
#[derive(Clone, Debug)]
struct wide_aload_instructionVar10 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_aload_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_aload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50201 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2087:1, end:2087:2))"]
#[derive(Clone, Debug)]
struct wide_lload_instructionVar11 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_lload_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_lload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50198 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2094:1, end:2094:2))"]
#[derive(Clone, Debug)]
struct wide_dload_instructionVar12 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_dload_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_dload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50200 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2103:1, end:2103:2))"]
#[derive(Clone, Debug)]
struct wide_istore_instructionVar13 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_istore_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_istore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50230 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2111:1, end:2111:2))"]
#[derive(Clone, Debug)]
struct wide_fstore_instructionVar14 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_fstore_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_fstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50232 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2119:1, end:2119:2))"]
#[derive(Clone, Debug)]
struct wide_astore_instructionVar15 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_astore_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_astore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50234 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2127:1, end:2127:2))"]
#[derive(Clone, Debug)]
struct wide_lstore_instructionVar16 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_lstore_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_lstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50231 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2135:1, end:2135:2))"]
#[derive(Clone, Debug)]
struct wide_dstore_instructionVar17 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_dstore_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_dstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c23 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50233 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c23(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2144:1, end:2144:2))"]
#[derive(Clone, Debug)]
struct wide_ret_instructionVar18 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl wide_ret_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("wide_ret"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50345 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2152:1, end:2152:2))"]
#[derive(Clone, Debug)]
struct iinc_w_instructionVar19 {
    indexbyte1: u8,
    indexbyte2: u8,
    constantbyte1: u8,
    constantbyte2: u8,
}
impl iinc_w_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        let mut calc_constant: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        calc_constant = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.constantbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.constantbyte2).unwrap());
        display.push(DisplayElement::Literal("iinc_w"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(
                true,
                calc_constant.is_negative(),
                calc_constant.abs() as u64,
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
        let mut calc_index: i128 = 0;
        let mut calc_constant: i128 = 0;
        let mut block_0_len = 2;
        let mut sub_pattern_c28 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 2;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_2(tokens) != 50308 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c28(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let constantbyte1 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        calc_constant = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let constantbyte2 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
                constantbyte1,
                constantbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct aaload_instructionVar20 {}
impl aaload_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aaload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 50 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:233:1, end:233:2))"]
#[derive(Clone, Debug)]
struct aastore_instructionVar21 {}
impl aastore_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 83 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:246:1, end:246:2))"]
#[derive(Clone, Debug)]
struct aconst_null_instructionVar22 {}
impl aconst_null_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aconst_null"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 1 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:251:1, end:251:2))"]
#[derive(Clone, Debug)]
struct aload_instructionVar23 {
    index: u8,
}
impl aload_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 25 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct aload_0_instructionVar24 {}
impl aload_0_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aload_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 42 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:265:1, end:265:2))"]
#[derive(Clone, Debug)]
struct aload_1_instructionVar25 {}
impl aload_1_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aload_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 43 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:272:1, end:272:2))"]
#[derive(Clone, Debug)]
struct aload_2_instructionVar26 {}
impl aload_2_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aload_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 44 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:279:1, end:279:2))"]
#[derive(Clone, Debug)]
struct aload_3_instructionVar27 {}
impl aload_3_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("aload_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 45 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:286:1, end:286:2))"]
#[derive(Clone, Debug)]
struct anewarray_instructionVar28 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl anewarray_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("anewarray"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 189 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:296:1, end:296:2))"]
#[derive(Clone, Debug)]
struct areturn_instructionVar29 {}
impl areturn_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("areturn"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 176 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:302:1, end:302:2))"]
#[derive(Clone, Debug)]
struct arraylength_instructionVar30 {}
impl arraylength_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("arraylength"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 190 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:311:1, end:311:2))"]
#[derive(Clone, Debug)]
struct astore_instructionVar31 {
    index: u8,
}
impl astore_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("astore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 58 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:319:1, end:319:2))"]
#[derive(Clone, Debug)]
struct astore_0_instructionVar32 {}
impl astore_0_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("astore_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 75 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:327:1, end:327:2))"]
#[derive(Clone, Debug)]
struct astore_1_instructionVar33 {}
impl astore_1_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("astore_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 76 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:335:1, end:335:2))"]
#[derive(Clone, Debug)]
struct astore_2_instructionVar34 {}
impl astore_2_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("astore_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 77 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:344:1, end:344:2))"]
#[derive(Clone, Debug)]
struct astore_3_instructionVar35 {}
impl astore_3_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("astore_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 78 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:352:1, end:352:2))"]
#[derive(Clone, Debug)]
struct athrow_instructionVar36 {}
impl athrow_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("athrow"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 191 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:362:1, end:362:2))"]
#[derive(Clone, Debug)]
struct baload_instructionVar37 {}
impl baload_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("baload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 51 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:375:1, end:375:2))"]
#[derive(Clone, Debug)]
struct bastore_instructionVar38 {}
impl bastore_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 84 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:387:1, end:387:2))"]
#[derive(Clone, Debug)]
struct bipush_instructionVar39 {
    sbyte: u8,
}
impl bipush_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("bipush"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(
                true,
                (if self.sbyte & 128 != 0 { -1 & !127 } else { 0 } | self.sbyte as i8)
                    .is_negative(),
                (if self.sbyte & 128 != 0 { -1 & !127 } else { 0 } | self.sbyte as i8).abs() as u64,
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 16 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let sbyte = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { sbyte }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:393:1, end:393:2))"]
#[derive(Clone, Debug)]
struct caload_instructionVar40 {}
impl caload_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("caload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 52 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:406:1, end:406:2))"]
#[derive(Clone, Debug)]
struct castore_instructionVar41 {}
impl castore_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("castore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 85 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:418:1, end:418:2))"]
#[derive(Clone, Debug)]
struct checkcast_instructionVar42 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl checkcast_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("checkcast"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 192 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:440:1, end:440:2))"]
#[derive(Clone, Debug)]
struct d2f_instructionVar43 {}
impl d2f_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("d2f"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 144 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:451:1, end:451:2))"]
#[derive(Clone, Debug)]
struct d2i_instructionVar44 {}
impl d2i_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("d2i"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 142 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:461:1, end:461:2))"]
#[derive(Clone, Debug)]
struct d2l_instructionVar45 {}
impl d2l_instructionVar45 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("d2l"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 143 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:470:1, end:470:2))"]
#[derive(Clone, Debug)]
struct dadd_instructionVar46 {}
impl dadd_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dadd"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 99 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:481:1, end:481:2))"]
#[derive(Clone, Debug)]
struct daload_instructionVar47 {}
impl daload_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("daload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 49 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:493:1, end:493:2))"]
#[derive(Clone, Debug)]
struct dastore_instructionVar48 {}
impl dastore_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 82 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:505:1, end:505:2))"]
#[derive(Clone, Debug)]
struct dcmpg_instructionVar49 {}
impl dcmpg_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dcmpg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 152 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:516:1, end:516:2))"]
#[derive(Clone, Debug)]
struct dcmpl_instructionVar50 {}
impl dcmpl_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dcmpl"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 151 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:527:1, end:527:2))"]
#[derive(Clone, Debug)]
struct dconst_0_instructionVar51 {}
impl dconst_0_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dconst_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 14 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:534:1, end:534:2))"]
#[derive(Clone, Debug)]
struct dconst_1_instructionVar52 {}
impl dconst_1_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dconst_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 15 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct ddiv_instructionVar53 {}
impl ddiv_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ddiv"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 111 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:552:1, end:552:2))"]
#[derive(Clone, Debug)]
struct dload_instructionVar54 {
    index: u8,
}
impl dload_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 24 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:559:1, end:559:2))"]
#[derive(Clone, Debug)]
struct dload_0_instructionVar55 {}
impl dload_0_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dload_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 38 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:566:1, end:566:2))"]
#[derive(Clone, Debug)]
struct dload_1_instructionVar56 {}
impl dload_1_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dload_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 39 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:573:1, end:573:2))"]
#[derive(Clone, Debug)]
struct dload_2_instructionVar57 {}
impl dload_2_instructionVar57 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dload_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 40 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:580:1, end:580:2))"]
#[derive(Clone, Debug)]
struct dload_3_instructionVar58 {}
impl dload_3_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dload_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 41 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:587:1, end:587:2))"]
#[derive(Clone, Debug)]
struct dmul_instructionVar59 {}
impl dmul_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dmul"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 107 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:598:1, end:598:2))"]
#[derive(Clone, Debug)]
struct dneg_instructionVar60 {}
impl dneg_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dneg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 119 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:607:1, end:607:2))"]
#[derive(Clone, Debug)]
struct drem_instructionVar61 {}
impl drem_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("drem"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 115 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:618:1, end:618:2))"]
#[derive(Clone, Debug)]
struct dreturn_instructionVar62 {}
impl dreturn_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dreturn"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 175 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:624:1, end:624:2))"]
#[derive(Clone, Debug)]
struct dstore_instructionVar63 {
    index: u8,
}
impl dstore_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 57 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:632:1, end:632:2))"]
#[derive(Clone, Debug)]
struct dstore_0_instructionVar64 {}
impl dstore_0_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dstore_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 71 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:640:1, end:640:2))"]
#[derive(Clone, Debug)]
struct dstore_1_instructionVar65 {}
impl dstore_1_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dstore_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 72 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct dstore_2_instructionVar66 {}
impl dstore_2_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dstore_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 73 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:656:1, end:656:2))"]
#[derive(Clone, Debug)]
struct dstore_3_instructionVar67 {}
impl dstore_3_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dstore_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 74 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:664:1, end:664:2))"]
#[derive(Clone, Debug)]
struct dsub_instructionVar68 {}
impl dsub_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dsub"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 103 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:675:1, end:675:2))"]
#[derive(Clone, Debug)]
struct dup_instructionVar69 {}
impl dup_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 89 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:681:1, end:681:2))"]
#[derive(Clone, Debug)]
struct dup_x1_instructionVar70 {}
impl dup_x1_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup_x1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 90 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:692:1, end:692:2))"]
#[derive(Clone, Debug)]
struct dup_x2_instructionVar71 {}
impl dup_x2_instructionVar71 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup_x2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 91 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:706:1, end:706:2))"]
#[derive(Clone, Debug)]
struct dup2_instructionVar72 {}
impl dup2_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 92 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:718:1, end:718:2))"]
#[derive(Clone, Debug)]
struct dup2_x1_instructionVar73 {}
impl dup2_x1_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup2_x1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 93 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:733:1, end:733:2))"]
#[derive(Clone, Debug)]
struct dup2_x2_instructionVar74 {}
impl dup2_x2_instructionVar74 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("dup2_x2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 94 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:751:1, end:751:2))"]
#[derive(Clone, Debug)]
struct f2d_instructionVar75 {}
impl f2d_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("f2d"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 141 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:761:1, end:761:2))"]
#[derive(Clone, Debug)]
struct f2i_instructionVar76 {}
impl f2i_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("f2i"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 139 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:771:1, end:771:2))"]
#[derive(Clone, Debug)]
struct f2l_instructionVar77 {}
impl f2l_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("f2l"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 140 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:782:1, end:782:2))"]
#[derive(Clone, Debug)]
struct fadd_instructionVar78 {}
impl fadd_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fadd"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 98 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:793:1, end:793:2))"]
#[derive(Clone, Debug)]
struct faload_instructionVar79 {}
impl faload_instructionVar79 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("faload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 48 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:805:1, end:805:2))"]
#[derive(Clone, Debug)]
struct fastore_instructionVar80 {}
impl fastore_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 81 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:817:1, end:817:2))"]
#[derive(Clone, Debug)]
struct fcmpg_instructionVar81 {}
impl fcmpg_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcmpg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 150 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:828:1, end:828:2))"]
#[derive(Clone, Debug)]
struct fcmpl_instructionVar82 {}
impl fcmpl_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fcmpl"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 149 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:839:1, end:839:2))"]
#[derive(Clone, Debug)]
struct fconst_0_instructionVar83 {}
impl fconst_0_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fconst_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 11 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:846:1, end:846:2))"]
#[derive(Clone, Debug)]
struct fconst_1_instructionVar84 {}
impl fconst_1_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fconst_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 12 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:853:1, end:853:2))"]
#[derive(Clone, Debug)]
struct fconst_2_instructionVar85 {}
impl fconst_2_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fconst_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 13 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:860:1, end:860:2))"]
#[derive(Clone, Debug)]
struct fdiv_instructionVar86 {}
impl fdiv_instructionVar86 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fdiv"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 110 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:871:1, end:871:2))"]
#[derive(Clone, Debug)]
struct fload_instructionVar87 {
    index: u8,
}
impl fload_instructionVar87 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 23 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:878:1, end:878:2))"]
#[derive(Clone, Debug)]
struct fload_0_instructionVar88 {}
impl fload_0_instructionVar88 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fload_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 34 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:885:1, end:885:2))"]
#[derive(Clone, Debug)]
struct fload_1_instructionVar89 {}
impl fload_1_instructionVar89 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fload_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 35 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:892:1, end:892:2))"]
#[derive(Clone, Debug)]
struct fload_2_instructionVar90 {}
impl fload_2_instructionVar90 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fload_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 36 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:899:1, end:899:2))"]
#[derive(Clone, Debug)]
struct fload_3_instructionVar91 {}
impl fload_3_instructionVar91 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fload_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 37 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:906:1, end:906:2))"]
#[derive(Clone, Debug)]
struct fmul_instructionVar92 {}
impl fmul_instructionVar92 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fmul"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 106 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:917:1, end:917:2))"]
#[derive(Clone, Debug)]
struct fneg_instructionVar93 {}
impl fneg_instructionVar93 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fneg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 118 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:926:1, end:926:2))"]
#[derive(Clone, Debug)]
struct frem_instructionVar94 {}
impl frem_instructionVar94 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("frem"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 114 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:937:1, end:937:2))"]
#[derive(Clone, Debug)]
struct freturn_instructionVar95 {}
impl freturn_instructionVar95 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("freturn"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 174 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:943:1, end:943:2))"]
#[derive(Clone, Debug)]
struct fstore_instructionVar96 {
    index: u8,
}
impl fstore_instructionVar96 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 56 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:951:1, end:951:2))"]
#[derive(Clone, Debug)]
struct fstore_0_instructionVar97 {}
impl fstore_0_instructionVar97 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fstore_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 67 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:959:1, end:959:2))"]
#[derive(Clone, Debug)]
struct fstore_1_instructionVar98 {}
impl fstore_1_instructionVar98 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fstore_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 68 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:966:1, end:966:2))"]
#[derive(Clone, Debug)]
struct fstore_2_instructionVar99 {}
impl fstore_2_instructionVar99 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fstore_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 69 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:973:1, end:973:2))"]
#[derive(Clone, Debug)]
struct fstore_3_instructionVar100 {}
impl fstore_3_instructionVar100 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fstore_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 70 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:981:1, end:981:2))"]
#[derive(Clone, Debug)]
struct fsub_instructionVar101 {}
impl fsub_instructionVar101 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("fsub"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 102 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:992:1, end:992:2))"]
#[derive(Clone, Debug)]
struct getfield_instructionVar102 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl getfield_instructionVar102 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("getfield"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 180 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:998:1, end:998:2))"]
#[derive(Clone, Debug)]
struct getstatic_instructionVar103 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl getstatic_instructionVar103 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("getstatic"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 178 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1003:1, end:1003:2))"]
#[derive(Clone, Debug)]
struct goto_instructionVar104 {
    Branch: TableBranch,
}
impl goto_instructionVar104 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("goto"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 167 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1008:1, end:1008:2))"]
#[derive(Clone, Debug)]
struct goto_w_instructionVar105 {
    Branch_w: TableBranch_w,
}
impl goto_w_instructionVar105 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("goto_w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch_w
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
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 200 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let Branch_w = if let Some((len, table)) =
            TableBranch_w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch_w }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1013:1, end:1013:2))"]
#[derive(Clone, Debug)]
struct i2b_instructionVar106 {}
impl i2b_instructionVar106 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2b"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 145 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1023:1, end:1023:2))"]
#[derive(Clone, Debug)]
struct i2c_instructionVar107 {}
impl i2c_instructionVar107 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2c"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 146 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1033:1, end:1033:2))"]
#[derive(Clone, Debug)]
struct i2d_instructionVar108 {}
impl i2d_instructionVar108 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2d"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 135 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1044:1, end:1044:2))"]
#[derive(Clone, Debug)]
struct i2f_instructionVar109 {}
impl i2f_instructionVar109 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2f"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 134 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1053:1, end:1053:2))"]
#[derive(Clone, Debug)]
struct i2l_instructionVar110 {}
impl i2l_instructionVar110 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2l"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 133 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1062:1, end:1062:2))"]
#[derive(Clone, Debug)]
struct i2s_instructionVar111 {}
impl i2s_instructionVar111 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("i2s"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c9 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 147 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c9(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1072:1, end:1072:2))"]
#[derive(Clone, Debug)]
struct iadd_instructionVar112 {}
impl iadd_instructionVar112 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iadd"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 96 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1083:1, end:1083:2))"]
#[derive(Clone, Debug)]
struct iaload_instructionVar113 {}
impl iaload_instructionVar113 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iaload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 46 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1095:1, end:1095:2))"]
#[derive(Clone, Debug)]
struct iand_instructionVar114 {}
impl iand_instructionVar114 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iand"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 126 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1106:1, end:1106:2))"]
#[derive(Clone, Debug)]
struct iastore_instructionVar115 {}
impl iastore_instructionVar115 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 79 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1118:1, end:1118:2))"]
#[derive(Clone, Debug)]
struct iconst_m1_instructionVar116 {}
impl iconst_m1_instructionVar116 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_m1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 2 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1124:1, end:1124:2))"]
#[derive(Clone, Debug)]
struct iconst_0_instructionVar117 {}
impl iconst_0_instructionVar117 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 3 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1130:1, end:1130:2))"]
#[derive(Clone, Debug)]
struct iconst_1_instructionVar118 {}
impl iconst_1_instructionVar118 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 4 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1136:1, end:1136:2))"]
#[derive(Clone, Debug)]
struct iconst_2_instructionVar119 {}
impl iconst_2_instructionVar119 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 5 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1142:1, end:1142:2))"]
#[derive(Clone, Debug)]
struct iconst_3_instructionVar120 {}
impl iconst_3_instructionVar120 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 6 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1148:1, end:1148:2))"]
#[derive(Clone, Debug)]
struct iconst_4_instructionVar121 {}
impl iconst_4_instructionVar121 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_4"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 7 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1154:1, end:1154:2))"]
#[derive(Clone, Debug)]
struct iconst_5_instructionVar122 {}
impl iconst_5_instructionVar122 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iconst_5"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 8 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1160:1, end:1160:2))"]
#[derive(Clone, Debug)]
struct idiv_instructionVar123 {}
impl idiv_instructionVar123 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("idiv"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 108 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1171:1, end:1171:2))"]
#[derive(Clone, Debug)]
struct if_acmpeq_instructionVar124 {
    Branch: TableBranch,
}
impl if_acmpeq_instructionVar124 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_acmpeq"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 165 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1180:1, end:1180:2))"]
#[derive(Clone, Debug)]
struct if_acmpne_instructionVar125 {
    Branch: TableBranch,
}
impl if_acmpne_instructionVar125 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_acmpne"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 166 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1189:1, end:1189:2))"]
#[derive(Clone, Debug)]
struct if_icmpeq_instructionVar126 {
    Branch: TableBranch,
}
impl if_icmpeq_instructionVar126 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmpeq"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 159 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1198:1, end:1198:2))"]
#[derive(Clone, Debug)]
struct if_icmpne_instructionVar127 {
    Branch: TableBranch,
}
impl if_icmpne_instructionVar127 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmpne"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 160 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1207:1, end:1207:2))"]
#[derive(Clone, Debug)]
struct if_icmplt_instructionVar128 {
    Branch: TableBranch,
}
impl if_icmplt_instructionVar128 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmplt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 161 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1216:1, end:1216:2))"]
#[derive(Clone, Debug)]
struct if_icmpge_instructionVar129 {
    Branch: TableBranch,
}
impl if_icmpge_instructionVar129 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmpge"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 162 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1225:1, end:1225:2))"]
#[derive(Clone, Debug)]
struct if_icmpgt_instructionVar130 {
    Branch: TableBranch,
}
impl if_icmpgt_instructionVar130 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmpgt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 163 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1234:1, end:1234:2))"]
#[derive(Clone, Debug)]
struct if_icmple_instructionVar131 {
    Branch: TableBranch,
}
impl if_icmple_instructionVar131 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("if_icmple"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 164 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1243:1, end:1243:2))"]
#[derive(Clone, Debug)]
struct ifeq_instructionVar132 {
    Branch: TableBranch,
}
impl ifeq_instructionVar132 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifeq"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 153 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1250:1, end:1250:2))"]
#[derive(Clone, Debug)]
struct ifne_instructionVar133 {
    Branch: TableBranch,
}
impl ifne_instructionVar133 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifne"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 154 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1257:1, end:1257:2))"]
#[derive(Clone, Debug)]
struct iflt_instructionVar134 {
    Branch: TableBranch,
}
impl iflt_instructionVar134 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iflt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 155 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1264:1, end:1264:2))"]
#[derive(Clone, Debug)]
struct ifge_instructionVar135 {
    Branch: TableBranch,
}
impl ifge_instructionVar135 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifge"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 156 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1271:1, end:1271:2))"]
#[derive(Clone, Debug)]
struct ifgt_instructionVar136 {
    Branch: TableBranch,
}
impl ifgt_instructionVar136 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifgt"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 157 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1278:1, end:1278:2))"]
#[derive(Clone, Debug)]
struct ifle_instructionVar137 {
    Branch: TableBranch,
}
impl ifle_instructionVar137 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifle"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 158 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1285:1, end:1285:2))"]
#[derive(Clone, Debug)]
struct ifnonnull_instructionVar138 {
    Branch: TableBranch,
}
impl ifnonnull_instructionVar138 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifnonnull"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 199 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1292:1, end:1292:2))"]
#[derive(Clone, Debug)]
struct ifnull_instructionVar139 {
    Branch: TableBranch,
}
impl ifnull_instructionVar139 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ifnull"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 198 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1299:1, end:1299:2))"]
#[derive(Clone, Debug)]
struct iinc_instructionVar140 {
    index: u8,
    constant: u8,
}
impl iinc_instructionVar140 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iinc"));
        let extend: [DisplayElement; 5usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.constant as u64),
        ];
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
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 132 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let constant = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index, constant }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1307:1, end:1307:2))"]
#[derive(Clone, Debug)]
struct iload_instructionVar141 {
    index: u8,
}
impl iload_instructionVar141 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 21 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1314:1, end:1314:2))"]
#[derive(Clone, Debug)]
struct iload_0_instructionVar142 {}
impl iload_0_instructionVar142 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iload_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 26 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1321:1, end:1321:2))"]
#[derive(Clone, Debug)]
struct iload_1_instructionVar143 {}
impl iload_1_instructionVar143 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iload_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 27 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1328:1, end:1328:2))"]
#[derive(Clone, Debug)]
struct iload_2_instructionVar144 {}
impl iload_2_instructionVar144 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iload_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 28 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1334:1, end:1334:2))"]
#[derive(Clone, Debug)]
struct iload_3_instructionVar145 {}
impl iload_3_instructionVar145 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iload_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 29 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1341:1, end:1341:2))"]
#[derive(Clone, Debug)]
struct imul_instructionVar146 {}
impl imul_instructionVar146 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("imul"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 104 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1352:1, end:1352:2))"]
#[derive(Clone, Debug)]
struct ineg_instructionVar147 {}
impl ineg_instructionVar147 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ineg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 116 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1361:1, end:1361:2))"]
#[derive(Clone, Debug)]
struct instanceof_instructionVar148 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl instanceof_instructionVar148 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("instanceof"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c22 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 193 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c22(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1371:1, end:1371:2))"]
#[derive(Clone, Debug)]
struct invokedynamic_instructionVar149 {
    blank1: u8,
    blank2: u8,
    indexbyte1: u8,
    indexbyte2: u8,
}
impl invokedynamic_instructionVar149 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("invokedynamic"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.blank1 as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.blank2 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c41 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 186 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c41(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let blank1 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let blank2 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                blank1,
                blank2,
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1376:1, end:1376:2))"]
#[derive(Clone, Debug)]
struct invokeinterface_instructionVar150 {
    count: u8,
    blank1: u8,
    indexbyte1: u8,
    indexbyte2: u8,
}
impl invokeinterface_instructionVar150 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("invokeinterface"));
        let extend: [DisplayElement; 8usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.count as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.blank1 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c42 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 185 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c42(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let count = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let blank1 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                count,
                blank1,
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1381:1, end:1381:2))"]
#[derive(Clone, Debug)]
struct invokespecial_instructionVar151 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl invokespecial_instructionVar151 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("invokespecial"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 183 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1386:1, end:1386:2))"]
#[derive(Clone, Debug)]
struct invokestatic_instructionVar152 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl invokestatic_instructionVar152 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("invokestatic"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c24 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 184 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c24(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1391:1, end:1391:2))"]
#[derive(Clone, Debug)]
struct invokevirtual_instructionVar153 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl invokevirtual_instructionVar153 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("invokevirtual"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c25 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 182 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c25(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1397:1, end:1397:2))"]
#[derive(Clone, Debug)]
struct ior_instructionVar154 {}
impl ior_instructionVar154 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ior"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 128 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1409:1, end:1409:2))"]
#[derive(Clone, Debug)]
struct irem_instructionVar155 {}
impl irem_instructionVar155 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("irem"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 112 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1421:1, end:1421:2))"]
#[derive(Clone, Debug)]
struct ireturn_instructionVar156 {}
impl ireturn_instructionVar156 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ireturn"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 172 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1427:1, end:1427:2))"]
#[derive(Clone, Debug)]
struct ishl_instructionVar157 {}
impl ishl_instructionVar157 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ishl"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 120 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1439:1, end:1439:2))"]
#[derive(Clone, Debug)]
struct ishr_instructionVar158 {}
impl ishr_instructionVar158 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ishr"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 122 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1451:1, end:1451:2))"]
#[derive(Clone, Debug)]
struct istore_instructionVar159 {
    index: u8,
}
impl istore_instructionVar159 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("istore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 54 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1459:1, end:1459:2))"]
#[derive(Clone, Debug)]
struct istore_0_instructionVar160 {}
impl istore_0_instructionVar160 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("istore_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 59 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1467:1, end:1467:2))"]
#[derive(Clone, Debug)]
struct istore_1_instructionVar161 {}
impl istore_1_instructionVar161 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("istore_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 60 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1475:1, end:1475:2))"]
#[derive(Clone, Debug)]
struct istore_2_instructionVar162 {}
impl istore_2_instructionVar162 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("istore_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 61 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1483:1, end:1483:2))"]
#[derive(Clone, Debug)]
struct istore_3_instructionVar163 {}
impl istore_3_instructionVar163 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("istore_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 62 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1491:1, end:1491:2))"]
#[derive(Clone, Debug)]
struct isub_instructionVar164 {}
impl isub_instructionVar164 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("isub"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 100 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1503:1, end:1503:2))"]
#[derive(Clone, Debug)]
struct iushr_instructionVar165 {}
impl iushr_instructionVar165 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("iushr"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 124 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1515:1, end:1515:2))"]
#[derive(Clone, Debug)]
struct ixor_instructionVar166 {}
impl ixor_instructionVar166 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ixor"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 130 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1527:1, end:1527:2))"]
#[derive(Clone, Debug)]
struct jsr_instructionVar167 {
    Branch: TableBranch,
}
impl jsr_instructionVar167 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jsr"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch
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
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 168 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c16(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let Branch = if let Some((len, table)) =
            TableBranch::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1534:1, end:1534:2))"]
#[derive(Clone, Debug)]
struct jsr_w_instructionVar168 {
    Branch_w: TableBranch_w,
}
impl jsr_w_instructionVar168 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("jsr_w"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.Branch_w
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
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 201 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let Branch_w = if let Some((len, table)) =
            TableBranch_w::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Branch_w }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1541:1, end:1541:2))"]
#[derive(Clone, Debug)]
struct l2d_instructionVar169 {}
impl l2d_instructionVar169 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("l2d"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 138 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1550:1, end:1550:2))"]
#[derive(Clone, Debug)]
struct l2f_instructionVar170 {}
impl l2f_instructionVar170 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("l2f"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 137 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1559:1, end:1559:2))"]
#[derive(Clone, Debug)]
struct l2i_instructionVar171 {}
impl l2i_instructionVar171 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("l2i"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 136 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1568:1, end:1568:2))"]
#[derive(Clone, Debug)]
struct ladd_instructionVar172 {}
impl ladd_instructionVar172 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ladd"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 97 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1579:1, end:1579:2))"]
#[derive(Clone, Debug)]
struct laload_instructionVar173 {}
impl laload_instructionVar173 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("laload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 47 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1592:1, end:1592:2))"]
#[derive(Clone, Debug)]
struct land_instructionVar174 {}
impl land_instructionVar174 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("land"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 127 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1603:1, end:1603:2))"]
#[derive(Clone, Debug)]
struct lastore_instructionVar175 {}
impl lastore_instructionVar175 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 80 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1616:1, end:1616:2))"]
#[derive(Clone, Debug)]
struct lcmp_instructionVar176 {}
impl lcmp_instructionVar176 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lcmp"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 148 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1627:1, end:1627:2))"]
#[derive(Clone, Debug)]
struct lconst_0_instructionVar177 {}
impl lconst_0_instructionVar177 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lconst_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 9 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1633:1, end:1633:2))"]
#[derive(Clone, Debug)]
struct lconst_1_instructionVar178 {}
impl lconst_1_instructionVar178 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lconst_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 10 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1639:1, end:1639:2))"]
#[derive(Clone, Debug)]
struct ldc_instructionVar179 {
    index: u8,
}
impl ldc_instructionVar179 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldc"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 18 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1644:1, end:1644:2))"]
#[derive(Clone, Debug)]
struct ldc_w_instructionVar180 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl ldc_w_instructionVar180 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("ldc_w"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 19 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1649:1, end:1649:2))"]
#[derive(Clone, Debug)]
struct ldc2_w_instructionVar181 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl ldc2_w_instructionVar181 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("ldc2_w"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 20 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1654:1, end:1654:2))"]
#[derive(Clone, Debug)]
struct ldiv_instructionVar182 {}
impl ldiv_instructionVar182 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ldiv"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 109 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1665:1, end:1665:2))"]
#[derive(Clone, Debug)]
struct lload_instructionVar183 {
    index: u8,
}
impl lload_instructionVar183 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lload"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 22 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1672:1, end:1672:2))"]
#[derive(Clone, Debug)]
struct lload_0_instructionVar184 {}
impl lload_0_instructionVar184 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lload_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 30 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1679:1, end:1679:2))"]
#[derive(Clone, Debug)]
struct lload_1_instructionVar185 {}
impl lload_1_instructionVar185 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lload_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 31 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1686:1, end:1686:2))"]
#[derive(Clone, Debug)]
struct lload_2_instructionVar186 {}
impl lload_2_instructionVar186 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lload_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 32 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1693:1, end:1693:2))"]
#[derive(Clone, Debug)]
struct lload_3_instructionVar187 {}
impl lload_3_instructionVar187 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lload_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 33 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1700:1, end:1700:2))"]
#[derive(Clone, Debug)]
struct lmul_instructionVar188 {}
impl lmul_instructionVar188 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lmul"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 105 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1711:1, end:1711:2))"]
#[derive(Clone, Debug)]
struct lneg_instructionVar189 {}
impl lneg_instructionVar189 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lneg"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 117 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1735:1, end:1735:2))"]
#[derive(Clone, Debug)]
struct instructionVar190 {
    LookupSwitch_match: TableLookupSwitch_match,
}
impl instructionVar190 {
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
        self.LookupSwitch_match
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 0;
        context_instance.write_in_lookup_switch(u8::try_from(0i128 & 3).unwrap());
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0;
            if context_instance.read_in_lookup_switch() != 1 {
                return None;
            }
            if context_instance.read_switch_num() != 1 {
                return None;
            }
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 8;
        let LookupSwitch_match = if let Some((len, table)) =
            TableLookupSwitch_match::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { LookupSwitch_match }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1775:1, end:1775:2))"]
#[derive(Clone, Debug)]
struct lor_instructionVar191 {}
impl lor_instructionVar191 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lor"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 129 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1786:1, end:1786:2))"]
#[derive(Clone, Debug)]
struct lrem_instructionVar192 {}
impl lrem_instructionVar192 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lrem"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 113 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1797:1, end:1797:2))"]
#[derive(Clone, Debug)]
struct lreturn_instructionVar193 {}
impl lreturn_instructionVar193 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lreturn"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 173 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1803:1, end:1803:2))"]
#[derive(Clone, Debug)]
struct lshl_instructionVar194 {}
impl lshl_instructionVar194 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lshl"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 121 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1814:1, end:1814:2))"]
#[derive(Clone, Debug)]
struct lshr_instructionVar195 {}
impl lshr_instructionVar195 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lshr"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 123 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1825:1, end:1825:2))"]
#[derive(Clone, Debug)]
struct lstore_instructionVar196 {
    index: u8,
}
impl lstore_instructionVar196 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lstore"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 55 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1833:1, end:1833:2))"]
#[derive(Clone, Debug)]
struct lstore_0_instructionVar197 {}
impl lstore_0_instructionVar197 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lstore_0"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 63 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1841:1, end:1841:2))"]
#[derive(Clone, Debug)]
struct lstore_1_instructionVar198 {}
impl lstore_1_instructionVar198 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lstore_1"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 64 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1849:1, end:1849:2))"]
#[derive(Clone, Debug)]
struct lstore_2_instructionVar199 {}
impl lstore_2_instructionVar199 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lstore_2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 65 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1857:1, end:1857:2))"]
#[derive(Clone, Debug)]
struct lstore_3_instructionVar200 {}
impl lstore_3_instructionVar200 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lstore_3"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 66 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1865:1, end:1865:2))"]
#[derive(Clone, Debug)]
struct lsub_instructionVar201 {}
impl lsub_instructionVar201 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lsub"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 101 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1876:1, end:1876:2))"]
#[derive(Clone, Debug)]
struct lushr_instructionVar202 {}
impl lushr_instructionVar202 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lushr"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 125 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1887:1, end:1887:2))"]
#[derive(Clone, Debug)]
struct lxor_instructionVar203 {}
impl lxor_instructionVar203 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("lxor"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 131 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1898:1, end:1898:2))"]
#[derive(Clone, Debug)]
struct monitorenter_instructionVar204 {}
impl monitorenter_instructionVar204 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("monitorenter"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c18 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 194 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c18(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1905:1, end:1905:2))"]
#[derive(Clone, Debug)]
struct monitorexit_instructionVar205 {}
impl monitorexit_instructionVar205 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("monitorexit"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 195 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1912:1, end:1912:2))"]
#[derive(Clone, Debug)]
struct multianewarray_instructionVar206 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl multianewarray_instructionVar206 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("multianewarray"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c26 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 197 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c26(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let dimensions = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1917:1, end:1917:2))"]
#[derive(Clone, Debug)]
struct new_instructionVar207 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl new_instructionVar207 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("new"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 187 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1924:1, end:1924:2))"]
#[derive(Clone, Debug)]
struct newarray_instructionVar208 {
    atype: u8,
}
impl newarray_instructionVar208 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("newarray"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.atype as u64),
        ];
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
        let mut sub_pattern_c20 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 188 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c20(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let atype = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { atype }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1934:1, end:1934:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar209 {}
impl nop_instructionVar209 {
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
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1938:1, end:1938:2))"]
#[derive(Clone, Debug)]
struct pop_instructionVar210 {}
impl pop_instructionVar210 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("pop"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c10 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 87 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c10(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1943:1, end:1943:2))"]
#[derive(Clone, Debug)]
struct pop2_instructionVar211 {}
impl pop2_instructionVar211 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("pop2"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c11 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 88 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c11(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1948:1, end:1948:2))"]
#[derive(Clone, Debug)]
struct putfield_instructionVar212 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl putfield_instructionVar212 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("putfield"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 181 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1953:1, end:1953:2))"]
#[derive(Clone, Debug)]
struct putstatic_instructionVar213 {
    indexbyte1: u8,
    indexbyte2: u8,
}
impl putstatic_instructionVar213 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_index: i128 = 0;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.indexbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.indexbyte2).unwrap());
        display.push(DisplayElement::Literal("putstatic"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_index.is_negative(), calc_index.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_index: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 179 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let indexbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_index = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let indexbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                indexbyte1,
                indexbyte2,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1958:1, end:1958:2))"]
#[derive(Clone, Debug)]
struct ret_instructionVar214 {
    index: u8,
}
impl ret_instructionVar214 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ret"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.index as u64),
        ];
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
        let mut sub_pattern_c16 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 169 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c16(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let index = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { index }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1965:1, end:1965:2))"]
#[derive(Clone, Debug)]
struct return_instructionVar215 {}
impl return_instructionVar215 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("return"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 177 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1970:1, end:1970:2))"]
#[derive(Clone, Debug)]
struct saload_instructionVar216 {}
impl saload_instructionVar216 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("saload"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 53 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1983:1, end:1983:2))"]
#[derive(Clone, Debug)]
struct sastore_instructionVar217 {}
impl sastore_instructionVar217 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("sastore"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 86 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1995:1, end:1995:2))"]
#[derive(Clone, Debug)]
struct sipush_instructionVar218 {
    byte1: u8,
    byte2: u8,
}
impl sipush_instructionVar218 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_short: i128 = 0;
        calc_short = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| i128::try_from(self.byte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | i128::try_from(self.byte2).unwrap());
        display.push(DisplayElement::Literal("sipush"));
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_short.is_negative(), calc_short.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_short: i128 = 0;
        let mut block_0_len = 1;
        let mut sub_pattern_c19 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 17 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c19(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let byte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        calc_short = (u32::try_from(8i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | i128::try_from(token_1(tokens_current)).unwrap());
        let byte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { byte1, byte2 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2001:1, end:2001:2))"]
#[derive(Clone, Debug)]
struct swap_instructionVar219 {}
impl swap_instructionVar219 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("swap"));
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 1;
        let mut sub_pattern_c12 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 1;
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if token_1(tokens) != 95 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c12(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2028:1, end:2028:2))"]
#[derive(Clone, Debug)]
struct instructionVar220 {
    Switch_offset: TableSwitch_offset,
}
impl instructionVar220 {
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
        self.Switch_offset
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 0;
        context_instance.write_in_table_switch(u8::try_from(0i128 & 3).unwrap());
        let mut sub_pattern_c21 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0;
            if context_instance.read_in_table_switch() != 1 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            if context_instance.read_switch_num() != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c21(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let Switch_offset = if let Some((len, table)) =
            TableSwitch_offset::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { Switch_offset }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1730:1, end:1730:2))"]
#[derive(Clone, Debug)]
struct instructionVar221 {
    LookupSwitch_match: TableLookupSwitch_match,
    instruction: Box<Tableinstruction>,
}
impl instructionVar221 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.LookupSwitch_match
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 0;
        context_instance.write_switch_num(
            u32::try_from(
                i128::try_from(context_instance.read_switch_num())
                    .unwrap()
                    .wrapping_sub(1i128)
                    & 4294967295,
            )
            .unwrap(),
        );
        let mut sub_pattern_c38 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0;
            if context_instance.read_in_lookup_switch() != 1 {
                return None;
            }
            if context_instance.read_in_table_switch() != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c38(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 8;
        let LookupSwitch_match = if let Some((len, table)) =
            TableLookupSwitch_match::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                LookupSwitch_match,
                instruction,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2023:1, end:2023:2))"]
#[derive(Clone, Debug)]
struct instructionVar222 {
    Switch_offset: TableSwitch_offset,
    instruction: Box<Tableinstruction>,
}
impl instructionVar222 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.Switch_offset
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
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
        let mut block_0_len = 0;
        context_instance.write_switch_num(
            u32::try_from(
                i128::try_from(context_instance.read_switch_num())
                    .unwrap()
                    .wrapping_sub(1i128)
                    & 4294967295,
            )
            .unwrap(),
        );
        let mut sub_pattern_c33 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let mut block_0_len = 0;
            if context_instance.read_in_table_switch() != 1 {
                return None;
            }
            if context_instance.read_in_lookup_switch() != 0 {
                return None;
            }
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c33(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 4;
        let Switch_offset = if let Some((len, table)) =
            TableSwitch_offset::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let instruction = if let Some((len, table)) =
            Tableinstruction::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            Box::new(table)
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                Switch_offset,
                instruction,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(lookupswitch_instructionVar0),
    Var1(lookupswitch_instructionVar1),
    Var2(lookupswitch_instructionVar2),
    Var3(lookupswitch_instructionVar3),
    Var4(tableswitch_instructionVar4),
    Var5(tableswitch_instructionVar5),
    Var6(tableswitch_instructionVar6),
    Var7(tableswitch_instructionVar7),
    Var8(wide_iload_instructionVar8),
    Var9(wide_fload_instructionVar9),
    Var10(wide_aload_instructionVar10),
    Var11(wide_lload_instructionVar11),
    Var12(wide_dload_instructionVar12),
    Var13(wide_istore_instructionVar13),
    Var14(wide_fstore_instructionVar14),
    Var15(wide_astore_instructionVar15),
    Var16(wide_lstore_instructionVar16),
    Var17(wide_dstore_instructionVar17),
    Var18(wide_ret_instructionVar18),
    Var19(iinc_w_instructionVar19),
    Var20(aaload_instructionVar20),
    Var21(aastore_instructionVar21),
    Var22(aconst_null_instructionVar22),
    Var23(aload_instructionVar23),
    Var24(aload_0_instructionVar24),
    Var25(aload_1_instructionVar25),
    Var26(aload_2_instructionVar26),
    Var27(aload_3_instructionVar27),
    Var28(anewarray_instructionVar28),
    Var29(areturn_instructionVar29),
    Var30(arraylength_instructionVar30),
    Var31(astore_instructionVar31),
    Var32(astore_0_instructionVar32),
    Var33(astore_1_instructionVar33),
    Var34(astore_2_instructionVar34),
    Var35(astore_3_instructionVar35),
    Var36(athrow_instructionVar36),
    Var37(baload_instructionVar37),
    Var38(bastore_instructionVar38),
    Var39(bipush_instructionVar39),
    Var40(caload_instructionVar40),
    Var41(castore_instructionVar41),
    Var42(checkcast_instructionVar42),
    Var43(d2f_instructionVar43),
    Var44(d2i_instructionVar44),
    Var45(d2l_instructionVar45),
    Var46(dadd_instructionVar46),
    Var47(daload_instructionVar47),
    Var48(dastore_instructionVar48),
    Var49(dcmpg_instructionVar49),
    Var50(dcmpl_instructionVar50),
    Var51(dconst_0_instructionVar51),
    Var52(dconst_1_instructionVar52),
    Var53(ddiv_instructionVar53),
    Var54(dload_instructionVar54),
    Var55(dload_0_instructionVar55),
    Var56(dload_1_instructionVar56),
    Var57(dload_2_instructionVar57),
    Var58(dload_3_instructionVar58),
    Var59(dmul_instructionVar59),
    Var60(dneg_instructionVar60),
    Var61(drem_instructionVar61),
    Var62(dreturn_instructionVar62),
    Var63(dstore_instructionVar63),
    Var64(dstore_0_instructionVar64),
    Var65(dstore_1_instructionVar65),
    Var66(dstore_2_instructionVar66),
    Var67(dstore_3_instructionVar67),
    Var68(dsub_instructionVar68),
    Var69(dup_instructionVar69),
    Var70(dup_x1_instructionVar70),
    Var71(dup_x2_instructionVar71),
    Var72(dup2_instructionVar72),
    Var73(dup2_x1_instructionVar73),
    Var74(dup2_x2_instructionVar74),
    Var75(f2d_instructionVar75),
    Var76(f2i_instructionVar76),
    Var77(f2l_instructionVar77),
    Var78(fadd_instructionVar78),
    Var79(faload_instructionVar79),
    Var80(fastore_instructionVar80),
    Var81(fcmpg_instructionVar81),
    Var82(fcmpl_instructionVar82),
    Var83(fconst_0_instructionVar83),
    Var84(fconst_1_instructionVar84),
    Var85(fconst_2_instructionVar85),
    Var86(fdiv_instructionVar86),
    Var87(fload_instructionVar87),
    Var88(fload_0_instructionVar88),
    Var89(fload_1_instructionVar89),
    Var90(fload_2_instructionVar90),
    Var91(fload_3_instructionVar91),
    Var92(fmul_instructionVar92),
    Var93(fneg_instructionVar93),
    Var94(frem_instructionVar94),
    Var95(freturn_instructionVar95),
    Var96(fstore_instructionVar96),
    Var97(fstore_0_instructionVar97),
    Var98(fstore_1_instructionVar98),
    Var99(fstore_2_instructionVar99),
    Var100(fstore_3_instructionVar100),
    Var101(fsub_instructionVar101),
    Var102(getfield_instructionVar102),
    Var103(getstatic_instructionVar103),
    Var104(goto_instructionVar104),
    Var105(goto_w_instructionVar105),
    Var106(i2b_instructionVar106),
    Var107(i2c_instructionVar107),
    Var108(i2d_instructionVar108),
    Var109(i2f_instructionVar109),
    Var110(i2l_instructionVar110),
    Var111(i2s_instructionVar111),
    Var112(iadd_instructionVar112),
    Var113(iaload_instructionVar113),
    Var114(iand_instructionVar114),
    Var115(iastore_instructionVar115),
    Var116(iconst_m1_instructionVar116),
    Var117(iconst_0_instructionVar117),
    Var118(iconst_1_instructionVar118),
    Var119(iconst_2_instructionVar119),
    Var120(iconst_3_instructionVar120),
    Var121(iconst_4_instructionVar121),
    Var122(iconst_5_instructionVar122),
    Var123(idiv_instructionVar123),
    Var124(if_acmpeq_instructionVar124),
    Var125(if_acmpne_instructionVar125),
    Var126(if_icmpeq_instructionVar126),
    Var127(if_icmpne_instructionVar127),
    Var128(if_icmplt_instructionVar128),
    Var129(if_icmpge_instructionVar129),
    Var130(if_icmpgt_instructionVar130),
    Var131(if_icmple_instructionVar131),
    Var132(ifeq_instructionVar132),
    Var133(ifne_instructionVar133),
    Var134(iflt_instructionVar134),
    Var135(ifge_instructionVar135),
    Var136(ifgt_instructionVar136),
    Var137(ifle_instructionVar137),
    Var138(ifnonnull_instructionVar138),
    Var139(ifnull_instructionVar139),
    Var140(iinc_instructionVar140),
    Var141(iload_instructionVar141),
    Var142(iload_0_instructionVar142),
    Var143(iload_1_instructionVar143),
    Var144(iload_2_instructionVar144),
    Var145(iload_3_instructionVar145),
    Var146(imul_instructionVar146),
    Var147(ineg_instructionVar147),
    Var148(instanceof_instructionVar148),
    Var149(invokedynamic_instructionVar149),
    Var150(invokeinterface_instructionVar150),
    Var151(invokespecial_instructionVar151),
    Var152(invokestatic_instructionVar152),
    Var153(invokevirtual_instructionVar153),
    Var154(ior_instructionVar154),
    Var155(irem_instructionVar155),
    Var156(ireturn_instructionVar156),
    Var157(ishl_instructionVar157),
    Var158(ishr_instructionVar158),
    Var159(istore_instructionVar159),
    Var160(istore_0_instructionVar160),
    Var161(istore_1_instructionVar161),
    Var162(istore_2_instructionVar162),
    Var163(istore_3_instructionVar163),
    Var164(isub_instructionVar164),
    Var165(iushr_instructionVar165),
    Var166(ixor_instructionVar166),
    Var167(jsr_instructionVar167),
    Var168(jsr_w_instructionVar168),
    Var169(l2d_instructionVar169),
    Var170(l2f_instructionVar170),
    Var171(l2i_instructionVar171),
    Var172(ladd_instructionVar172),
    Var173(laload_instructionVar173),
    Var174(land_instructionVar174),
    Var175(lastore_instructionVar175),
    Var176(lcmp_instructionVar176),
    Var177(lconst_0_instructionVar177),
    Var178(lconst_1_instructionVar178),
    Var179(ldc_instructionVar179),
    Var180(ldc_w_instructionVar180),
    Var181(ldc2_w_instructionVar181),
    Var182(ldiv_instructionVar182),
    Var183(lload_instructionVar183),
    Var184(lload_0_instructionVar184),
    Var185(lload_1_instructionVar185),
    Var186(lload_2_instructionVar186),
    Var187(lload_3_instructionVar187),
    Var188(lmul_instructionVar188),
    Var189(lneg_instructionVar189),
    Var190(instructionVar190),
    Var191(lor_instructionVar191),
    Var192(lrem_instructionVar192),
    Var193(lreturn_instructionVar193),
    Var194(lshl_instructionVar194),
    Var195(lshr_instructionVar195),
    Var196(lstore_instructionVar196),
    Var197(lstore_0_instructionVar197),
    Var198(lstore_1_instructionVar198),
    Var199(lstore_2_instructionVar199),
    Var200(lstore_3_instructionVar200),
    Var201(lsub_instructionVar201),
    Var202(lushr_instructionVar202),
    Var203(lxor_instructionVar203),
    Var204(monitorenter_instructionVar204),
    Var205(monitorexit_instructionVar205),
    Var206(multianewarray_instructionVar206),
    Var207(new_instructionVar207),
    Var208(newarray_instructionVar208),
    Var209(nop_instructionVar209),
    Var210(pop_instructionVar210),
    Var211(pop2_instructionVar211),
    Var212(putfield_instructionVar212),
    Var213(putstatic_instructionVar213),
    Var214(ret_instructionVar214),
    Var215(return_instructionVar215),
    Var216(saload_instructionVar216),
    Var217(sastore_instructionVar217),
    Var218(sipush_instructionVar218),
    Var219(swap_instructionVar219),
    Var220(instructionVar220),
    Var221(instructionVar221),
    Var222(instructionVar222),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 10
            && context_param.0 & 4991374238398653268393268871168 == 0
            && (tokens_param[0] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                lookupswitch_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 11
            && context_param.0 & 4991374238398653268393268871168 == 2535301200456458802993406410752
            && (tokens_param[0] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                lookupswitch_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 12
            && context_param.0 & 4991374238398653268393268871168 == 1267650600228229401496703205376
            && (tokens_param[0] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                lookupswitch_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 13
            && context_param.0 & 4991374238398653268393268871168 == 3802951800684688204490109616128
            && (tokens_param[0] & 255) == 171
        {
            if let Some((inst_len, parsed)) =
                lookupswitch_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 14
            && context_param.0 & 4991374238398653268393268871168 == 0
            && (tokens_param[0] & 255) == 170
        {
            if let Some((inst_len, parsed)) =
                tableswitch_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 15
            && context_param.0 & 4991374238398653268393268871168 == 2535301200456458802993406410752
            && (tokens_param[0] & 255) == 170
        {
            if let Some((inst_len, parsed)) =
                tableswitch_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 16
            && context_param.0 & 4991374238398653268393268871168 == 1267650600228229401496703205376
            && (tokens_param[0] & 255) == 170
        {
            if let Some((inst_len, parsed)) =
                tableswitch_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 17
            && context_param.0 & 4991374238398653268393268871168 == 3802951800684688204490109616128
            && (tokens_param[0] & 255) == 170
        {
            if let Some((inst_len, parsed)) =
                tableswitch_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 21
        {
            if let Some((inst_len, parsed)) =
                wide_iload_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 23
        {
            if let Some((inst_len, parsed)) =
                wide_fload_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                wide_aload_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                wide_lload_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                wide_dload_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 54
        {
            if let Some((inst_len, parsed)) =
                wide_istore_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 56
        {
            if let Some((inst_len, parsed)) =
                wide_fstore_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 58
        {
            if let Some((inst_len, parsed)) =
                wide_astore_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 55
        {
            if let Some((inst_len, parsed)) =
                wide_lstore_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 57
        {
            if let Some((inst_len, parsed)) =
                wide_dstore_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 169
        {
            if let Some((inst_len, parsed)) =
                wide_ret_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 6
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 196
            && (tokens_param[1] & 255) == 132
        {
            if let Some((inst_len, parsed)) =
                iinc_w_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 50
        {
            if let Some((inst_len, parsed)) =
                aaload_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 83
        {
            if let Some((inst_len, parsed)) =
                aastore_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 1
        {
            if let Some((inst_len, parsed)) =
                aconst_null_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 25
        {
            if let Some((inst_len, parsed)) =
                aload_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 42
        {
            if let Some((inst_len, parsed)) =
                aload_0_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 43
        {
            if let Some((inst_len, parsed)) =
                aload_1_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 44
        {
            if let Some((inst_len, parsed)) =
                aload_2_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 45
        {
            if let Some((inst_len, parsed)) =
                aload_3_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 189
        {
            if let Some((inst_len, parsed)) =
                anewarray_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 176
        {
            if let Some((inst_len, parsed)) =
                areturn_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 190
        {
            if let Some((inst_len, parsed)) =
                arraylength_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 58
        {
            if let Some((inst_len, parsed)) =
                astore_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 75
        {
            if let Some((inst_len, parsed)) =
                astore_0_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 76
        {
            if let Some((inst_len, parsed)) =
                astore_1_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 77
        {
            if let Some((inst_len, parsed)) =
                astore_2_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 78
        {
            if let Some((inst_len, parsed)) =
                astore_3_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 191
        {
            if let Some((inst_len, parsed)) =
                athrow_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 51
        {
            if let Some((inst_len, parsed)) =
                baload_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 84
        {
            if let Some((inst_len, parsed)) =
                bastore_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 16
        {
            if let Some((inst_len, parsed)) =
                bipush_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 52
        {
            if let Some((inst_len, parsed)) =
                caload_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 85
        {
            if let Some((inst_len, parsed)) =
                castore_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 192
        {
            if let Some((inst_len, parsed)) =
                checkcast_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 144
        {
            if let Some((inst_len, parsed)) =
                d2f_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 142
        {
            if let Some((inst_len, parsed)) =
                d2i_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 143
        {
            if let Some((inst_len, parsed)) =
                d2l_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 99
        {
            if let Some((inst_len, parsed)) =
                dadd_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 49
        {
            if let Some((inst_len, parsed)) =
                daload_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 82
        {
            if let Some((inst_len, parsed)) =
                dastore_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 152
        {
            if let Some((inst_len, parsed)) =
                dcmpg_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 151
        {
            if let Some((inst_len, parsed)) =
                dcmpl_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 14
        {
            if let Some((inst_len, parsed)) =
                dconst_0_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 15
        {
            if let Some((inst_len, parsed)) =
                dconst_1_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 111
        {
            if let Some((inst_len, parsed)) =
                ddiv_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 24
        {
            if let Some((inst_len, parsed)) =
                dload_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 38
        {
            if let Some((inst_len, parsed)) =
                dload_0_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 39
        {
            if let Some((inst_len, parsed)) =
                dload_1_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 40
        {
            if let Some((inst_len, parsed)) =
                dload_2_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 41
        {
            if let Some((inst_len, parsed)) =
                dload_3_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 107
        {
            if let Some((inst_len, parsed)) =
                dmul_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 119
        {
            if let Some((inst_len, parsed)) =
                dneg_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 115
        {
            if let Some((inst_len, parsed)) =
                drem_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 175
        {
            if let Some((inst_len, parsed)) =
                dreturn_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 57
        {
            if let Some((inst_len, parsed)) =
                dstore_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 71
        {
            if let Some((inst_len, parsed)) =
                dstore_0_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 72
        {
            if let Some((inst_len, parsed)) =
                dstore_1_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 73
        {
            if let Some((inst_len, parsed)) =
                dstore_2_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 74
        {
            if let Some((inst_len, parsed)) =
                dstore_3_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 103
        {
            if let Some((inst_len, parsed)) =
                dsub_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 89
        {
            if let Some((inst_len, parsed)) =
                dup_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 90
        {
            if let Some((inst_len, parsed)) =
                dup_x1_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 91
        {
            if let Some((inst_len, parsed)) =
                dup_x2_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 92
        {
            if let Some((inst_len, parsed)) =
                dup2_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 93
        {
            if let Some((inst_len, parsed)) =
                dup2_x1_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 94
        {
            if let Some((inst_len, parsed)) =
                dup2_x2_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 141
        {
            if let Some((inst_len, parsed)) =
                f2d_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 139
        {
            if let Some((inst_len, parsed)) =
                f2i_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 140
        {
            if let Some((inst_len, parsed)) =
                f2l_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 98
        {
            if let Some((inst_len, parsed)) =
                fadd_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 48
        {
            if let Some((inst_len, parsed)) =
                faload_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 81
        {
            if let Some((inst_len, parsed)) =
                fastore_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 150
        {
            if let Some((inst_len, parsed)) =
                fcmpg_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 149
        {
            if let Some((inst_len, parsed)) =
                fcmpl_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 11
        {
            if let Some((inst_len, parsed)) =
                fconst_0_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 12
        {
            if let Some((inst_len, parsed)) =
                fconst_1_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 13
        {
            if let Some((inst_len, parsed)) =
                fconst_2_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 110
        {
            if let Some((inst_len, parsed)) =
                fdiv_instructionVar86::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var86(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 23
        {
            if let Some((inst_len, parsed)) =
                fload_instructionVar87::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var87(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 34
        {
            if let Some((inst_len, parsed)) =
                fload_0_instructionVar88::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var88(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 35
        {
            if let Some((inst_len, parsed)) =
                fload_1_instructionVar89::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var89(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 36
        {
            if let Some((inst_len, parsed)) =
                fload_2_instructionVar90::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var90(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 37
        {
            if let Some((inst_len, parsed)) =
                fload_3_instructionVar91::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var91(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 106
        {
            if let Some((inst_len, parsed)) =
                fmul_instructionVar92::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var92(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 118
        {
            if let Some((inst_len, parsed)) =
                fneg_instructionVar93::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var93(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 114
        {
            if let Some((inst_len, parsed)) =
                frem_instructionVar94::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var94(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 174
        {
            if let Some((inst_len, parsed)) =
                freturn_instructionVar95::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var95(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 56
        {
            if let Some((inst_len, parsed)) =
                fstore_instructionVar96::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var96(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 67
        {
            if let Some((inst_len, parsed)) =
                fstore_0_instructionVar97::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var97(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 68
        {
            if let Some((inst_len, parsed)) =
                fstore_1_instructionVar98::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var98(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 69
        {
            if let Some((inst_len, parsed)) =
                fstore_2_instructionVar99::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var99(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 70
        {
            if let Some((inst_len, parsed)) =
                fstore_3_instructionVar100::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var100(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 102
        {
            if let Some((inst_len, parsed)) =
                fsub_instructionVar101::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var101(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 180
        {
            if let Some((inst_len, parsed)) =
                getfield_instructionVar102::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var102(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 178
        {
            if let Some((inst_len, parsed)) =
                getstatic_instructionVar103::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var103(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 167
        {
            if let Some((inst_len, parsed)) =
                goto_instructionVar104::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var104(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 200
        {
            if let Some((inst_len, parsed)) =
                goto_w_instructionVar105::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var105(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 145
        {
            if let Some((inst_len, parsed)) =
                i2b_instructionVar106::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var106(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 146
        {
            if let Some((inst_len, parsed)) =
                i2c_instructionVar107::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var107(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 135
        {
            if let Some((inst_len, parsed)) =
                i2d_instructionVar108::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var108(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 134
        {
            if let Some((inst_len, parsed)) =
                i2f_instructionVar109::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var109(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 133
        {
            if let Some((inst_len, parsed)) =
                i2l_instructionVar110::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var110(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 147
        {
            if let Some((inst_len, parsed)) =
                i2s_instructionVar111::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var111(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 96
        {
            if let Some((inst_len, parsed)) =
                iadd_instructionVar112::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var112(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 46
        {
            if let Some((inst_len, parsed)) =
                iaload_instructionVar113::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var113(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 126
        {
            if let Some((inst_len, parsed)) =
                iand_instructionVar114::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var114(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 79
        {
            if let Some((inst_len, parsed)) =
                iastore_instructionVar115::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var115(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 2
        {
            if let Some((inst_len, parsed)) =
                iconst_m1_instructionVar116::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var116(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 3
        {
            if let Some((inst_len, parsed)) =
                iconst_0_instructionVar117::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var117(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 4
        {
            if let Some((inst_len, parsed)) =
                iconst_1_instructionVar118::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var118(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 5
        {
            if let Some((inst_len, parsed)) =
                iconst_2_instructionVar119::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var119(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 6
        {
            if let Some((inst_len, parsed)) =
                iconst_3_instructionVar120::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var120(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 7
        {
            if let Some((inst_len, parsed)) =
                iconst_4_instructionVar121::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var121(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 8
        {
            if let Some((inst_len, parsed)) =
                iconst_5_instructionVar122::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var122(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 108
        {
            if let Some((inst_len, parsed)) =
                idiv_instructionVar123::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var123(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 165
        {
            if let Some((inst_len, parsed)) =
                if_acmpeq_instructionVar124::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var124(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 166
        {
            if let Some((inst_len, parsed)) =
                if_acmpne_instructionVar125::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var125(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 159
        {
            if let Some((inst_len, parsed)) =
                if_icmpeq_instructionVar126::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var126(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 160
        {
            if let Some((inst_len, parsed)) =
                if_icmpne_instructionVar127::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var127(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 161
        {
            if let Some((inst_len, parsed)) =
                if_icmplt_instructionVar128::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var128(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 162
        {
            if let Some((inst_len, parsed)) =
                if_icmpge_instructionVar129::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var129(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 163
        {
            if let Some((inst_len, parsed)) =
                if_icmpgt_instructionVar130::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var130(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 164
        {
            if let Some((inst_len, parsed)) =
                if_icmple_instructionVar131::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var131(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 153
        {
            if let Some((inst_len, parsed)) =
                ifeq_instructionVar132::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var132(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 154
        {
            if let Some((inst_len, parsed)) =
                ifne_instructionVar133::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var133(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 155
        {
            if let Some((inst_len, parsed)) =
                iflt_instructionVar134::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var134(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 156
        {
            if let Some((inst_len, parsed)) =
                ifge_instructionVar135::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var135(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 157
        {
            if let Some((inst_len, parsed)) =
                ifgt_instructionVar136::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var136(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 158
        {
            if let Some((inst_len, parsed)) =
                ifle_instructionVar137::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var137(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 199
        {
            if let Some((inst_len, parsed)) =
                ifnonnull_instructionVar138::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var138(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 198
        {
            if let Some((inst_len, parsed)) =
                ifnull_instructionVar139::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var139(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 132
        {
            if let Some((inst_len, parsed)) =
                iinc_instructionVar140::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var140(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 21
        {
            if let Some((inst_len, parsed)) =
                iload_instructionVar141::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var141(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 26
        {
            if let Some((inst_len, parsed)) =
                iload_0_instructionVar142::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var142(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 27
        {
            if let Some((inst_len, parsed)) =
                iload_1_instructionVar143::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var143(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 28
        {
            if let Some((inst_len, parsed)) =
                iload_2_instructionVar144::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var144(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 29
        {
            if let Some((inst_len, parsed)) =
                iload_3_instructionVar145::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var145(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 104
        {
            if let Some((inst_len, parsed)) =
                imul_instructionVar146::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var146(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 116
        {
            if let Some((inst_len, parsed)) =
                ineg_instructionVar147::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var147(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 193
        {
            if let Some((inst_len, parsed)) =
                instanceof_instructionVar148::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var148(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 186
        {
            if let Some((inst_len, parsed)) = invokedynamic_instructionVar149::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var149(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 185
        {
            if let Some((inst_len, parsed)) = invokeinterface_instructionVar150::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var150(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 183
        {
            if let Some((inst_len, parsed)) = invokespecial_instructionVar151::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var151(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 184
        {
            if let Some((inst_len, parsed)) = invokestatic_instructionVar152::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var152(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 182
        {
            if let Some((inst_len, parsed)) = invokevirtual_instructionVar153::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var153(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 128
        {
            if let Some((inst_len, parsed)) =
                ior_instructionVar154::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var154(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 112
        {
            if let Some((inst_len, parsed)) =
                irem_instructionVar155::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var155(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 172
        {
            if let Some((inst_len, parsed)) =
                ireturn_instructionVar156::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var156(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 120
        {
            if let Some((inst_len, parsed)) =
                ishl_instructionVar157::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var157(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 122
        {
            if let Some((inst_len, parsed)) =
                ishr_instructionVar158::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var158(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 54
        {
            if let Some((inst_len, parsed)) =
                istore_instructionVar159::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var159(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 59
        {
            if let Some((inst_len, parsed)) =
                istore_0_instructionVar160::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var160(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 60
        {
            if let Some((inst_len, parsed)) =
                istore_1_instructionVar161::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var161(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 61
        {
            if let Some((inst_len, parsed)) =
                istore_2_instructionVar162::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var162(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 62
        {
            if let Some((inst_len, parsed)) =
                istore_3_instructionVar163::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var163(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 100
        {
            if let Some((inst_len, parsed)) =
                isub_instructionVar164::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var164(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 124
        {
            if let Some((inst_len, parsed)) =
                iushr_instructionVar165::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var165(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 130
        {
            if let Some((inst_len, parsed)) =
                ixor_instructionVar166::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var166(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 168
        {
            if let Some((inst_len, parsed)) =
                jsr_instructionVar167::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var167(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 201
        {
            if let Some((inst_len, parsed)) =
                jsr_w_instructionVar168::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var168(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 138
        {
            if let Some((inst_len, parsed)) =
                l2d_instructionVar169::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var169(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 137
        {
            if let Some((inst_len, parsed)) =
                l2f_instructionVar170::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var170(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 136
        {
            if let Some((inst_len, parsed)) =
                l2i_instructionVar171::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var171(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 97
        {
            if let Some((inst_len, parsed)) =
                ladd_instructionVar172::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var172(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 47
        {
            if let Some((inst_len, parsed)) =
                laload_instructionVar173::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var173(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 127
        {
            if let Some((inst_len, parsed)) =
                land_instructionVar174::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var174(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 80
        {
            if let Some((inst_len, parsed)) =
                lastore_instructionVar175::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var175(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 148
        {
            if let Some((inst_len, parsed)) =
                lcmp_instructionVar176::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var176(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 9
        {
            if let Some((inst_len, parsed)) =
                lconst_0_instructionVar177::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var177(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 10
        {
            if let Some((inst_len, parsed)) =
                lconst_1_instructionVar178::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var178(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 18
        {
            if let Some((inst_len, parsed)) =
                ldc_instructionVar179::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var179(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 19
        {
            if let Some((inst_len, parsed)) =
                ldc_w_instructionVar180::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var180(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 20
        {
            if let Some((inst_len, parsed)) =
                ldc2_w_instructionVar181::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var181(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 109
        {
            if let Some((inst_len, parsed)) =
                ldiv_instructionVar182::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var182(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 22
        {
            if let Some((inst_len, parsed)) =
                lload_instructionVar183::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var183(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 30
        {
            if let Some((inst_len, parsed)) =
                lload_0_instructionVar184::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var184(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 31
        {
            if let Some((inst_len, parsed)) =
                lload_1_instructionVar185::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var185(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 32
        {
            if let Some((inst_len, parsed)) =
                lload_2_instructionVar186::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var186(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 33
        {
            if let Some((inst_len, parsed)) =
                lload_3_instructionVar187::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var187(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 105
        {
            if let Some((inst_len, parsed)) =
                lmul_instructionVar188::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var188(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 117
        {
            if let Some((inst_len, parsed)) =
                lneg_instructionVar189::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var189(parsed)));
            }
        }
        if tokens_param.len() >= 8
            && context_param.0 & 1267650600209782657422993653760 == 673439381371246869545123577856
        {
            if let Some((inst_len, parsed)) =
                instructionVar190::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var190(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 129
        {
            if let Some((inst_len, parsed)) =
                lor_instructionVar191::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var191(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 113
        {
            if let Some((inst_len, parsed)) =
                lrem_instructionVar192::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var192(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 173
        {
            if let Some((inst_len, parsed)) =
                lreturn_instructionVar193::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var193(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 121
        {
            if let Some((inst_len, parsed)) =
                lshl_instructionVar194::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var194(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 123
        {
            if let Some((inst_len, parsed)) =
                lshr_instructionVar195::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var195(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 55
        {
            if let Some((inst_len, parsed)) =
                lstore_instructionVar196::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var196(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 63
        {
            if let Some((inst_len, parsed)) =
                lstore_0_instructionVar197::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var197(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 64
        {
            if let Some((inst_len, parsed)) =
                lstore_1_instructionVar198::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var198(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 65
        {
            if let Some((inst_len, parsed)) =
                lstore_2_instructionVar199::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var199(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 66
        {
            if let Some((inst_len, parsed)) =
                lstore_3_instructionVar200::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var200(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 101
        {
            if let Some((inst_len, parsed)) =
                lsub_instructionVar201::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var201(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 125
        {
            if let Some((inst_len, parsed)) =
                lushr_instructionVar202::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var202(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 131
        {
            if let Some((inst_len, parsed)) =
                lxor_instructionVar203::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var203(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 194
        {
            if let Some((inst_len, parsed)) = monitorenter_instructionVar204::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var204(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 195
        {
            if let Some((inst_len, parsed)) =
                monitorexit_instructionVar205::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var205(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 197
        {
            if let Some((inst_len, parsed)) = multianewarray_instructionVar206::parse(
                tokens_param,
                &mut context_current,
                inst_start,
            ) {
                *context_param = context_current;
                return Some((inst_len, Self::Var206(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 187
        {
            if let Some((inst_len, parsed)) =
                new_instructionVar207::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var207(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 188
        {
            if let Some((inst_len, parsed)) =
                newarray_instructionVar208::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var208(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 0
        {
            if let Some((inst_len, parsed)) =
                nop_instructionVar209::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var209(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 87
        {
            if let Some((inst_len, parsed)) =
                pop_instructionVar210::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var210(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 88
        {
            if let Some((inst_len, parsed)) =
                pop2_instructionVar211::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var211(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 181
        {
            if let Some((inst_len, parsed)) =
                putfield_instructionVar212::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var212(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 179
        {
            if let Some((inst_len, parsed)) =
                putstatic_instructionVar213::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var213(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 169
        {
            if let Some((inst_len, parsed)) =
                ret_instructionVar214::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var214(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 177
        {
            if let Some((inst_len, parsed)) =
                return_instructionVar215::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var215(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 53
        {
            if let Some((inst_len, parsed)) =
                saload_instructionVar216::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var216(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 86
        {
            if let Some((inst_len, parsed)) =
                sastore_instructionVar217::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var217(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 17
        {
            if let Some((inst_len, parsed)) =
                sipush_instructionVar218::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var218(parsed)));
            }
        }
        if tokens_param.len() >= 1
            && context_param.0 & 1188422437713965063903159255040 == 0
            && (tokens_param[0] & 255) == 95
        {
            if let Some((inst_len, parsed)) =
                swap_instructionVar219::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var219(parsed)));
            }
        }
        if tokens_param.len() >= 4
            && context_param.0 & 1267650600209782657422993653760 == 158456325028528675187087900672
        {
            if let Some((inst_len, parsed)) =
                instructionVar220::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var220(parsed)));
            }
        }
        if tokens_param.len() >= 9
            && context_param.0 & 1188422437713965063903159255040 == 633825300114114700748351602688
        {
            if let Some((inst_len, parsed)) =
                instructionVar221::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var221(parsed)));
            }
        }
        if tokens_param.len() >= 5
            && context_param.0 & 1188422437713965063903159255040 == 158456325028528675187087900672
        {
            if let Some((inst_len, parsed)) =
                instructionVar222::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var222(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:204:1, end:204:7))"]
#[derive(Clone, Debug)]
struct BranchVar0 {
    branchbyte1: u8,
    branchbyte2: u8,
}
impl BranchVar0 {
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
            (u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(
                        (if self.branchbyte1 & 128 != 0 {
                            -1 & !127
                        } else {
                            0
                        } | self.branchbyte1 as i8),
                    )
                    .unwrap()
                    .checked_shl(shl)
                })
                .unwrap_or(0)
                | i128::try_from(self.branchbyte2).unwrap()),
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
        let mut block_0_len = 1;
        let branchbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        let branchbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                branchbyte1,
                branchbyte2,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableBranch {
    Var0(BranchVar0),
}
impl TableBranch {
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
                BranchVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:209:1, end:209:9))"]
#[derive(Clone, Debug)]
struct Branch_wVar0 {
    branchbyte1: u8,
    branchbyte2: u8,
    branchbyte3: u8,
    branchbyte4: u8,
}
impl Branch_wVar0 {
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
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(
                        (if self.branchbyte1 & 128 != 0 {
                            -1 & !127
                        } else {
                            0
                        } | self.branchbyte1 as i8),
                    )
                    .unwrap()
                    .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.branchbyte2).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.branchbyte3).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | i128::try_from(self.branchbyte4).unwrap()),
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
        let mut block_0_len = 1;
        let branchbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let branchbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let branchbyte3 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        let branchbyte4 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                branchbyte1,
                branchbyte2,
                branchbyte3,
                branchbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableBranch_w {
    Var0(Branch_wVar0),
}
impl TableBranch_w {
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
                Branch_wVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:214:1, end:214:8))"]
#[derive(Clone, Debug)]
struct DefaultVar0 {
    defaultbyte1: u8,
    defaultbyte2: u8,
    defaultbyte3: u8,
    defaultbyte4: u8,
}
impl DefaultVar0 {
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
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| i128::try_from(self.defaultbyte1).unwrap().checked_shl(shl))
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.defaultbyte2).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.defaultbyte3).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | i128::try_from(self.defaultbyte4).unwrap()),
        );
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal("default"),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_addr.is_negative(), calc_addr.abs() as u64),
        ];
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
        let mut block_0_len = 1;
        let defaultbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let defaultbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let defaultbyte3 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        calc_addr = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        let defaultbyte4 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                defaultbyte1,
                defaultbyte2,
                defaultbyte3,
                defaultbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableDefault {
    Var0(DefaultVar0),
}
impl TableDefault {
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
                DefaultVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1725:1, end:1725:19))"]
#[derive(Clone, Debug)]
struct LookupSwitch_matchVar0 {
    matchbyte1: u8,
    matchbyte2: u8,
    matchbyte3: u8,
    matchbyte4: u8,
    offsetbyte1: u8,
    offsetbyte2: u8,
    offsetbyte3: u8,
    offsetbyte4: u8,
}
impl LookupSwitch_matchVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_match: i128 = 0;
        let mut calc__offset: i128 = 0;
        calc_match = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| i128::try_from(self.matchbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| i128::try_from(self.matchbyte2).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| i128::try_from(self.matchbyte3).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.matchbyte4).unwrap());
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| i128::try_from(self.offsetbyte1).unwrap().checked_shl(shl))
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.offsetbyte2).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.offsetbyte3).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | i128::try_from(self.offsetbyte4).unwrap()),
        );
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Number(true, calc_match.is_negative(), calc_match.abs() as u64),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc__offset.is_negative(), calc__offset.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_match: i128 = 0;
        let mut calc__offset: i128 = 0;
        let mut block_0_len = 1;
        let matchbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let matchbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let matchbyte3 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        calc_match = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_1(tokens_current)).unwrap());
        let matchbyte4 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let offsetbyte1 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1;
        let offsetbyte2 = token_1(tokens_current);
        pattern_len += block_5_len;
        tokens_current = &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1;
        let offsetbyte3 = token_1(tokens_current);
        pattern_len += block_6_len;
        tokens_current = &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1;
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        let offsetbyte4 = token_1(tokens_current);
        pattern_len += block_7_len;
        tokens_current = &tokens_current[usize::try_from(block_7_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                matchbyte1,
                matchbyte2,
                matchbyte3,
                matchbyte4,
                offsetbyte1,
                offsetbyte2,
                offsetbyte3,
                offsetbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableLookupSwitch_match {
    Var0(LookupSwitch_matchVar0),
}
impl TableLookupSwitch_match {
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
        if tokens_param.len() >= 8 {
            if let Some((inst_len, parsed)) =
                LookupSwitch_matchVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1741:1, end:1741:10))"]
#[derive(Clone, Debug)]
struct padSwitchVar0 {}
impl padSwitchVar0 {
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
        let mut block_0_len = 1;
        let op = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let pad3 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1742:1, end:1742:10))"]
#[derive(Clone, Debug)]
struct padSwitchVar1 {}
impl padSwitchVar1 {
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
        let mut block_0_len = 1;
        let op = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let pad2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1743:1, end:1743:10))"]
#[derive(Clone, Debug)]
struct padSwitchVar2 {}
impl padSwitchVar2 {
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
        let mut block_0_len = 1;
        let op = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let pad1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1744:1, end:1744:10))"]
#[derive(Clone, Debug)]
struct padSwitchVar3 {}
impl padSwitchVar3 {
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
        let mut block_0_len = 1;
        let op = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TablepadSwitch {
    Var0(padSwitchVar0),
    Var1(padSwitchVar1),
    Var2(padSwitchVar2),
    Var3(padSwitchVar3),
}
impl TablepadSwitch {
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
        if tokens_param.len() >= 4
            && context_param.0 & 3802951800684688204490109616128 == 3802951800684688204490109616128
        {
            if let Some((inst_len, parsed)) =
                padSwitchVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 3
            && context_param.0 & 3802951800684688204490109616128 == 1267650600228229401496703205376
        {
            if let Some((inst_len, parsed)) =
                padSwitchVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2
            && context_param.0 & 3802951800684688204490109616128 == 2535301200456458802993406410752
        {
            if let Some((inst_len, parsed)) =
                padSwitchVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 1 && context_param.0 & 3802951800684688204490109616128 == 0 {
            if let Some((inst_len, parsed)) =
                padSwitchVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:1747:1, end:1747:15))"]
#[derive(Clone, Debug)]
struct dolookupswitchVar0 {
    npairsbyte1: u8,
    npairsbyte2: u8,
    npairsbyte3: u8,
    npairsbyte4: u8,
    defaultbyte1: u8,
    defaultbyte2: u8,
    defaultbyte3: u8,
    defaultbyte4: u8,
}
impl dolookupswitchVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_npairs: i128 = 0;
        let mut calc__default: i128 = 0;
        calc_npairs = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| i128::try_from(self.npairsbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| i128::try_from(self.npairsbyte2).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| i128::try_from(self.npairsbyte3).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.npairsbyte4).unwrap());
        calc__default = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| i128::try_from(self.defaultbyte1).unwrap().checked_shl(shl))
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.defaultbyte2).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.defaultbyte3).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | i128::try_from(self.defaultbyte4).unwrap()),
        );
        let extend: [DisplayElement; 4usize] = [
            <DisplayElement>::Number(
                true,
                calc__default.is_negative(),
                calc__default.abs() as u64,
            ),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_npairs.is_negative(), calc_npairs.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_npairs: i128 = 0;
        let mut calc__default: i128 = 0;
        let mut block_0_len = 1;
        let defaultbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let defaultbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let defaultbyte3 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let defaultbyte4 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        let npairsbyte1 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1;
        let npairsbyte2 = token_1(tokens_current);
        pattern_len += block_5_len;
        tokens_current = &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1;
        let npairsbyte3 = token_1(tokens_current);
        pattern_len += block_6_len;
        tokens_current = &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1;
        calc_npairs = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_1(tokens_current)).unwrap());
        calc__default = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        context_instance.write_switch_num(u32::try_from(calc_npairs & 4294967295).unwrap());
        context_instance.write_in_lookup_switch(u8::try_from(1i128 & 3).unwrap());
        let npairsbyte4 = token_1(tokens_current);
        pattern_len += block_7_len;
        tokens_current = &tokens_current[usize::try_from(block_7_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                npairsbyte1,
                npairsbyte2,
                npairsbyte3,
                npairsbyte4,
                defaultbyte1,
                defaultbyte2,
                defaultbyte3,
                defaultbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tabledolookupswitch {
    Var0(dolookupswitchVar0),
}
impl Tabledolookupswitch {
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
        if tokens_param.len() >= 8 {
            if let Some((inst_len, parsed)) =
                dolookupswitchVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2016:1, end:2016:14))"]
#[derive(Clone, Debug)]
struct Switch_offsetVar0 {
    offsetbyte1: u8,
    offsetbyte2: u8,
    offsetbyte3: u8,
    offsetbyte4: u8,
}
impl Switch_offsetVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc__offset: i128 = 0;
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| i128::try_from(self.offsetbyte1).unwrap().checked_shl(shl))
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.offsetbyte2).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| i128::try_from(self.offsetbyte3).unwrap().checked_shl(shl))
                    .unwrap_or(0))
                | i128::try_from(self.offsetbyte4).unwrap()),
        );
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc__offset.is_negative(),
            calc__offset.abs() as u64,
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
        let mut calc__offset: i128 = 0;
        let mut block_0_len = 1;
        let offsetbyte1 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let offsetbyte2 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let offsetbyte3 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        calc__offset = i128::try_from(inst_start).unwrap().wrapping_add(
            (((u32::try_from(24i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0)
                | u32::try_from(16i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | u32::try_from(8i128)
                    .ok()
                    .and_then(|shl| {
                        i128::try_from(token_1(tokens_current))
                            .unwrap()
                            .checked_shl(shl)
                    })
                    .unwrap_or(0))
                | i128::try_from(token_1(tokens_current)).unwrap()),
        );
        let offsetbyte4 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                offsetbyte1,
                offsetbyte2,
                offsetbyte3,
                offsetbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum TableSwitch_offset {
    Var0(Switch_offsetVar0),
}
impl TableSwitch_offset {
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
                Switch_offsetVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/JVM/data/languages/JVM.slaspec, start:2032:1, end:2032:14))"]
#[derive(Clone, Debug)]
struct dotableswitchVar0 {
    lowbyte1: u8,
    lowbyte2: u8,
    lowbyte3: u8,
    lowbyte4: u8,
    highbyte1: u8,
    highbyte2: u8,
    highbyte3: u8,
    highbyte4: u8,
    Default: TableDefault,
}
impl dotableswitchVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_low: i128 = 0;
        let mut calc_high: i128 = 0;
        calc_low = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| i128::try_from(self.lowbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| i128::try_from(self.lowbyte2).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| i128::try_from(self.lowbyte3).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.lowbyte4).unwrap());
        calc_high = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| i128::try_from(self.highbyte1).unwrap().checked_shl(shl))
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| i128::try_from(self.highbyte2).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| i128::try_from(self.highbyte3).unwrap().checked_shl(shl))
                .unwrap_or(0))
            | i128::try_from(self.highbyte4).unwrap());
        self.Default
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 6usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_low.is_negative(), calc_low.abs() as u64),
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
            <DisplayElement>::Number(true, calc_high.is_negative(), calc_high.abs() as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut calc_low: i128 = 0;
        let mut calc_high: i128 = 0;
        let mut block_0_len = 4;
        let Default = if let Some((len, table)) =
            TableDefault::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let lowbyte1 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let lowbyte2 = token_1(tokens_current);
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        let mut block_3_len = 1;
        let lowbyte3 = token_1(tokens_current);
        pattern_len += block_3_len;
        tokens_current = &tokens_current[usize::try_from(block_3_len).unwrap()..];
        let mut block_4_len = 1;
        calc_low = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_1(tokens_current)).unwrap());
        let lowbyte4 = token_1(tokens_current);
        pattern_len += block_4_len;
        tokens_current = &tokens_current[usize::try_from(block_4_len).unwrap()..];
        let mut block_5_len = 1;
        let highbyte1 = token_1(tokens_current);
        pattern_len += block_5_len;
        tokens_current = &tokens_current[usize::try_from(block_5_len).unwrap()..];
        let mut block_6_len = 1;
        let highbyte2 = token_1(tokens_current);
        pattern_len += block_6_len;
        tokens_current = &tokens_current[usize::try_from(block_6_len).unwrap()..];
        let mut block_7_len = 1;
        let highbyte3 = token_1(tokens_current);
        pattern_len += block_7_len;
        tokens_current = &tokens_current[usize::try_from(block_7_len).unwrap()..];
        let mut block_8_len = 1;
        calc_high = (((u32::try_from(24i128)
            .ok()
            .and_then(|shl| {
                i128::try_from(token_1(tokens_current))
                    .unwrap()
                    .checked_shl(shl)
            })
            .unwrap_or(0)
            | u32::try_from(16i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | u32::try_from(8i128)
                .ok()
                .and_then(|shl| {
                    i128::try_from(token_1(tokens_current))
                        .unwrap()
                        .checked_shl(shl)
                })
                .unwrap_or(0))
            | i128::try_from(token_1(tokens_current)).unwrap());
        context_instance.write_switch_low(u32::try_from(calc_low & 4294967295).unwrap());
        context_instance.write_switch_num(
            u32::try_from(calc_high.wrapping_sub(calc_low) & 4294967295).unwrap(),
        );
        context_instance.write_switch_high(u32::try_from(calc_high & 4294967295).unwrap());
        context_instance.write_in_table_switch(u8::try_from(1i128 & 3).unwrap());
        let highbyte4 = token_1(tokens_current);
        pattern_len += block_8_len;
        tokens_current = &tokens_current[usize::try_from(block_8_len).unwrap()..];
        *context = context_instance;
        Some((
            pattern_len,
            Self {
                Default,
                lowbyte1,
                lowbyte2,
                lowbyte3,
                lowbyte4,
                highbyte1,
                highbyte2,
                highbyte3,
                highbyte4,
            },
        ))
    }
}
#[derive(Clone, Debug)]
enum Tabledotableswitch {
    Var0(dotableswitchVar0),
}
impl Tabledotableswitch {
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
        if tokens_param.len() >= 12 {
            if let Some((inst_len, parsed)) =
                dotableswitchVar0::parse(tokens_param, &mut context_current, inst_start)
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
