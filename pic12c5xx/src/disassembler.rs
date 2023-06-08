pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    PC,
    STKPTR,
    W,
    PCL,
    FSR,
    STATUS,
    PA,
    Z,
    DC,
    C,
    OPTION,
    INDF,
    TMR0,
    PCL_0,
    STATUS_0,
    FSR_0,
    OSCCAL,
    GPIO,
    TRIS,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::PC => "PC",
            Self::STKPTR => "STKPTR",
            Self::W => "W",
            Self::PCL => "PCL",
            Self::FSR => "FSR",
            Self::STATUS => "STATUS",
            Self::PA => "PA",
            Self::Z => "Z",
            Self::DC => "DC",
            Self::C => "C",
            Self::OPTION => "OPTION",
            Self::INDF => "INDF",
            Self::TMR0 => "TMR0",
            Self::PCL_0 => "PCL.0",
            Self::STATUS_0 => "STATUS.0",
            Self::FSR_0 => "FSR.0",
            Self::OSCCAL => "OSCCAL",
            Self::GPIO => "GPIO",
            Self::TRIS => "TRIS",
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
#[doc = "Create token_fields: op3"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 9) & 7) as u8)
}
#[doc = "Create token_fields: f5"]
fn token_7(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 31) as u8)
}
#[doc = "Create token_fields: k9"]
fn token_10(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 511) as u16)
}
#[doc = "Create token_fields: d"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 1) as u8)
}
#[doc = "Create token_fields: op6"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 6) & 63) as u8)
}
#[doc = "Create token_fields: op4"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 8) & 15) as u8)
}
#[doc = "Create token_fields: b3"]
fn token_6(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 5) & 7) as u8)
}
#[doc = "Create token_fields: f5h"]
fn token_8(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 4) & 1) as u8)
}
#[doc = "Create token_fields: op12"]
fn token_1(tokens: &[u8]) -> u16 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 4095) as u16)
}
#[doc = "Create token_fields: k8"]
fn token_9(tokens: &[u8]) -> u8 {
    (((u16::from_le_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 255) as u8)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:407:1, end:407:2))"]
#[derive(Clone, Debug)]
struct CLRW_instructionVar0 {}
impl CLRW_instructionVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRW"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:414:1, end:414:2))"]
#[derive(Clone, Debug)]
struct CLRWDT_instructionVar1 {}
impl CLRWDT_instructionVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRWDT"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:532:1, end:532:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar2 {}
impl NOP_instructionVar2 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:536:1, end:536:2))"]
#[derive(Clone, Debug)]
struct OPTION_instructionVar3 {}
impl OPTION_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("OPTION"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:578:1, end:578:2))"]
#[derive(Clone, Debug)]
struct SLEEP_instructionVar4 {}
impl SLEEP_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SLEEP"));
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:249:1, end:249:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar5 {
    bit: Tablebit,
    status: Tablestatus,
}
impl BCF_instructionVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:257:1, end:257:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar6 {
    status: Tablestatus,
    bit: Tablebit,
}
impl BCF_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { status, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:265:1, end:265:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar7 {
    status: Tablestatus,
    bit: Tablebit,
}
impl BCF_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { status, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:273:1, end:273:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar8 {
    status: Tablestatus,
    bit: Tablebit,
}
impl BCF_instructionVar8 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { status, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:281:1, end:281:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar9 {
    bit: Tablebit,
    status: Tablestatus,
}
impl BCF_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:297:1, end:297:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar10 {
    bit: Tablebit,
    status: Tablestatus,
}
impl BSF_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:305:1, end:305:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar11 {
    status: Tablestatus,
    bit: Tablebit,
}
impl BSF_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { status, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:313:1, end:313:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar12 {
    status: Tablestatus,
    bit: Tablebit,
}
impl BSF_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { status, bit }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:321:1, end:321:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar13 {
    bit: Tablebit,
    status: Tablestatus,
}
impl BSF_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:329:1, end:329:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar14 {
    bit: Tablebit,
    status: Tablestatus,
}
impl BSF_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, status }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:346:1, end:346:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar15 {
    status: Tablestatus,
    skipInst: TableskipInst,
    bit: Tablebit,
}
impl BTFSC_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                status,
                skipInst,
                bit,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:352:1, end:352:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar16 {
    skipInst: TableskipInst,
    status: Tablestatus,
    bit: Tablebit,
}
impl BTFSC_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                skipInst,
                status,
                bit,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:358:1, end:358:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar17 {
    status: Tablestatus,
    bit: Tablebit,
    skipInst: TableskipInst,
}
impl BTFSC_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                status,
                bit,
                skipInst,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:373:1, end:373:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar18 {
    bit: Tablebit,
    skipInst: TableskipInst,
    status: Tablestatus,
}
impl BTFSS_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                bit,
                skipInst,
                status,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:379:1, end:379:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar19 {
    skipInst: TableskipInst,
    bit: Tablebit,
    status: Tablestatus,
}
impl BTFSS_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                skipInst,
                bit,
                status,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:385:1, end:385:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar20 {
    status: Tablestatus,
    skipInst: TableskipInst,
    bit: Tablebit,
}
impl BTFSS_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.status
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let status = if let Some((len, table)) =
            Tablestatus::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                status,
                skipInst,
                bit,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:399:1, end:399:2))"]
#[derive(Clone, Debug)]
struct CLRF_instructionVar21 {
    srcREG: TablesrcREG,
}
impl CLRF_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:517:1, end:517:2))"]
#[derive(Clone, Debug)]
struct MOVWF_instructionVar22 {
    srcREG: TablesrcREG,
}
impl MOVWF_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOVWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec, start:24:1, end:24:2))"]
#[derive(Clone, Debug)]
struct TRIS_instructionVar23 {
    trisREG: TabletrisREG,
}
impl TRIS_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TRIS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.trisREG
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let trisREG = if let Some((len, table)) =
            TabletrisREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { trisREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct ADDWF_instructionVar24 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ADDWF_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:209:1, end:209:2))"]
#[derive(Clone, Debug)]
struct ADDWF_instructionVar25 {
    D: TableD,
    pcl: Tablepcl,
}
impl ADDWF_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ADDWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.pcl
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let pcl = if let Some((len, table)) =
            Tablepcl::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { D, pcl }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:230:1, end:230:2))"]
#[derive(Clone, Debug)]
struct ANDWF_instructionVar26 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl ANDWF_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:420:1, end:420:2))"]
#[derive(Clone, Debug)]
struct COMF_instructionVar27 {
    destREG: TabledestREG,
    D: TableD,
    srcREG: TablesrcREG,
}
impl COMF_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("COMF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, D, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:431:1, end:431:2))"]
#[derive(Clone, Debug)]
struct DECF_instructionVar28 {
    destREG: TabledestREG,
    srcREG: TablesrcREG,
    D: TableD,
}
impl DECF_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, srcREG, D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:442:1, end:442:2))"]
#[derive(Clone, Debug)]
struct DECFSZ_instructionVar29 {
    skipInst: TableskipInst,
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl DECFSZ_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECFSZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                skipInst,
                srcREG,
                D,
                destREG,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:460:1, end:460:2))"]
#[derive(Clone, Debug)]
struct INCF_instructionVar30 {
    D: TableD,
    srcREG: TablesrcREG,
    destREG: TabledestREG,
}
impl INCF_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { D, srcREG, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:471:1, end:471:2))"]
#[derive(Clone, Debug)]
struct INCFSZ_instructionVar31 {
    skipInst: TableskipInst,
    D: TableD,
    destREG: TabledestREG,
    srcREG: TablesrcREG,
}
impl INCFSZ_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCFSZ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                skipInst,
                D,
                destREG,
                srcREG,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:489:1, end:489:2))"]
#[derive(Clone, Debug)]
struct IORWF_instructionVar32 {
    destREG: TabledestREG,
    srcREG: TablesrcREG,
    D: TableD,
}
impl IORWF_instructionVar32 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IORWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, srcREG, D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:506:1, end:506:2))"]
#[derive(Clone, Debug)]
struct MOVF_instructionVar33 {
    srcREG: TablesrcREG,
    D: TableD,
    destREG: TabledestREG,
}
impl MOVF_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOVF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG, D, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:524:1, end:524:2))"]
#[derive(Clone, Debug)]
struct MOVWF_instructionVar34 {
    pcl: Tablepcl,
}
impl MOVWF_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOVWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.pcl
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let pcl = if let Some((len, table)) =
            Tablepcl::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { pcl }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:550:1, end:550:2))"]
#[derive(Clone, Debug)]
struct RLF_instructionVar35 {
    D: TableD,
    srcREG: TablesrcREG,
    destREG: TabledestREG,
}
impl RLF_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RLF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { D, srcREG, destREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:564:1, end:564:2))"]
#[derive(Clone, Debug)]
struct RRF_instructionVar36 {
    destREG: TabledestREG,
    srcREG: TablesrcREG,
    D: TableD,
}
impl RRF_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RRF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, srcREG, D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:584:1, end:584:2))"]
#[derive(Clone, Debug)]
struct SUBWF_instructionVar37 {
    destREG: TabledestREG,
    D: TableD,
    srcREG: TablesrcREG,
}
impl SUBWF_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SUBWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, D, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:597:1, end:597:2))"]
#[derive(Clone, Debug)]
struct SWAPF_instructionVar38 {
    srcREG: TablesrcREG,
    destREG: TabledestREG,
    D: TableD,
}
impl SWAPF_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SWAPF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG, destREG, D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:614:1, end:614:2))"]
#[derive(Clone, Debug)]
struct XORWF_instructionVar39 {
    destREG: TabledestREG,
    srcREG: TablesrcREG,
    D: TableD,
}
impl XORWF_instructionVar39 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORWF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.D
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let D = if let Some((len, table)) =
            TableD::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let destREG = if let Some((len, table)) =
            TabledestREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { destREG, srcREG, D }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:223:1, end:223:2))"]
#[derive(Clone, Debug)]
struct ANDLW_instructionVar40 {
    imm8: Tableimm8,
}
impl ANDLW_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ANDLW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:241:1, end:241:2))"]
#[derive(Clone, Debug)]
struct BCF_instructionVar41 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BCF_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:289:1, end:289:2))"]
#[derive(Clone, Debug)]
struct BSF_instructionVar42 {
    bit: Tablebit,
    srcREG: TablesrcREG,
}
impl BSF_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSF"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { bit, srcREG }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:337:1, end:337:2))"]
#[derive(Clone, Debug)]
struct BTFSC_instructionVar43 {
    srcREG: TablesrcREG,
    skipInst: TableskipInst,
    bit: Tablebit,
}
impl BTFSC_instructionVar43 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                srcREG,
                skipInst,
                bit,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:364:1, end:364:2))"]
#[derive(Clone, Debug)]
struct BTFSS_instructionVar44 {
    bit: Tablebit,
    skipInst: TableskipInst,
    srcREG: TablesrcREG,
}
impl BTFSS_instructionVar44 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BTFSS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.srcREG
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 2usize] = [
            <DisplayElement>::Literal(","),
            <DisplayElement>::Literal(" "),
        ];
        display.extend_from_slice(&extend);
        self.bit
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let bit = if let Some((len, table)) =
            Tablebit::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        let skipInst = if let Some((len, table)) =
            TableskipInst::parse(tokens_current, &mut context_instance, inst_start)
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
                bit,
                skipInst,
                srcREG,
            },
        ))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:391:1, end:391:2))"]
#[derive(Clone, Debug)]
struct CALL_instructionVar45 {
    absAddr8: TableabsAddr8,
}
impl CALL_instructionVar45 {
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
        self.absAddr8
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let absAddr8 = if let Some((len, table)) =
            TableabsAddr8::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { absAddr8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:482:1, end:482:2))"]
#[derive(Clone, Debug)]
struct IORLW_instructionVar46 {
    imm8: Tableimm8,
}
impl IORLW_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("IORLW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:500:1, end:500:2))"]
#[derive(Clone, Debug)]
struct MOVLW_instructionVar47 {
    imm8: Tableimm8,
}
impl MOVLW_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MOVLW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:541:1, end:541:2))"]
#[derive(Clone, Debug)]
struct RETLW_instructionVar48 {
    imm8: Tableimm8,
}
impl RETLW_instructionVar48 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RETLW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:607:1, end:607:2))"]
#[derive(Clone, Debug)]
struct XORLW_instructionVar49 {
    imm8: Tableimm8,
}
impl XORLW_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("XORLW"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.imm8
            .display_extend(display, context, inst_start, inst_next, global_set);
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
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:453:1, end:453:2))"]
#[derive(Clone, Debug)]
struct GOTO_instructionVar50 {
    absAddr9: TableabsAddr9,
}
impl GOTO_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("GOTO"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.absAddr9
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let absAddr9 = if let Some((len, table)) =
            TableabsAddr9::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { absAddr9 }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(CLRW_instructionVar0),
    Var1(CLRWDT_instructionVar1),
    Var2(NOP_instructionVar2),
    Var3(OPTION_instructionVar3),
    Var4(SLEEP_instructionVar4),
    Var5(BCF_instructionVar5),
    Var6(BCF_instructionVar6),
    Var7(BCF_instructionVar7),
    Var8(BCF_instructionVar8),
    Var9(BCF_instructionVar9),
    Var10(BSF_instructionVar10),
    Var11(BSF_instructionVar11),
    Var12(BSF_instructionVar12),
    Var13(BSF_instructionVar13),
    Var14(BSF_instructionVar14),
    Var15(BTFSC_instructionVar15),
    Var16(BTFSC_instructionVar16),
    Var17(BTFSC_instructionVar17),
    Var18(BTFSS_instructionVar18),
    Var19(BTFSS_instructionVar19),
    Var20(BTFSS_instructionVar20),
    Var21(CLRF_instructionVar21),
    Var22(MOVWF_instructionVar22),
    Var23(TRIS_instructionVar23),
    Var24(ADDWF_instructionVar24),
    Var25(ADDWF_instructionVar25),
    Var26(ANDWF_instructionVar26),
    Var27(COMF_instructionVar27),
    Var28(DECF_instructionVar28),
    Var29(DECFSZ_instructionVar29),
    Var30(INCF_instructionVar30),
    Var31(INCFSZ_instructionVar31),
    Var32(IORWF_instructionVar32),
    Var33(MOVF_instructionVar33),
    Var34(MOVWF_instructionVar34),
    Var35(RLF_instructionVar35),
    Var36(RRF_instructionVar36),
    Var37(SUBWF_instructionVar37),
    Var38(SWAPF_instructionVar38),
    Var39(XORWF_instructionVar39),
    Var40(ANDLW_instructionVar40),
    Var41(BCF_instructionVar41),
    Var42(BSF_instructionVar42),
    Var43(BTFSC_instructionVar43),
    Var44(BTFSS_instructionVar44),
    Var45(CALL_instructionVar45),
    Var46(IORLW_instructionVar46),
    Var47(MOVLW_instructionVar47),
    Var48(RETLW_instructionVar48),
    Var49(XORLW_instructionVar49),
    Var50(GOTO_instructionVar50),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 64 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                CLRW_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 4 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                CLRWDT_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 0 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 2 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                OPTION_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 3 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                SLEEP_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 15) == 4
        {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 15) == 4
        {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 160 && (tokens_param[1] & 15) == 5
        {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 192 && (tokens_param[1] & 15) == 5
        {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                BTFSC_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                BTFSC_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                BTFSC_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                BTFSS_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                BTFSS_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 64 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                BTFSS_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 96 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                CLRF_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 32 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MOVWF_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 224) == 0 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                TRIS_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 192 && (tokens_param[1] & 15) == 1
        {
            if let Some((inst_len, parsed)) =
                ADDWF_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 192 && (tokens_param[1] & 15) == 1
        {
            if let Some((inst_len, parsed)) =
                ADDWF_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 64 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                ANDWF_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 64 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                COMF_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 192 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                DECF_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 192 && (tokens_param[1] & 15) == 2
        {
            if let Some((inst_len, parsed)) =
                DECFSZ_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 15) == 2
        {
            if let Some((inst_len, parsed)) =
                INCF_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 192 && (tokens_param[1] & 15) == 3
        {
            if let Some((inst_len, parsed)) =
                INCFSZ_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 0 && (tokens_param[1] & 15) == 1 {
            if let Some((inst_len, parsed)) =
                IORWF_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 0 && (tokens_param[1] & 15) == 2 {
            if let Some((inst_len, parsed)) =
                MOVF_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 0 && (tokens_param[1] & 15) == 0 {
            if let Some((inst_len, parsed)) =
                MOVWF_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 64 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                RLF_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 0 && (tokens_param[1] & 15) == 3 {
            if let Some((inst_len, parsed)) =
                RRF_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 15) == 0
        {
            if let Some((inst_len, parsed)) =
                SUBWF_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 15) == 3
        {
            if let Some((inst_len, parsed)) =
                SWAPF_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 192) == 128 && (tokens_param[1] & 15) == 1
        {
            if let Some((inst_len, parsed)) =
                XORWF_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 14 {
            if let Some((inst_len, parsed)) =
                ANDLW_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 4 {
            if let Some((inst_len, parsed)) =
                BCF_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 5 {
            if let Some((inst_len, parsed)) =
                BSF_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 6 {
            if let Some((inst_len, parsed)) =
                BTFSC_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 7 {
            if let Some((inst_len, parsed)) =
                BTFSS_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 9 {
            if let Some((inst_len, parsed)) =
                CALL_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 13 {
            if let Some((inst_len, parsed)) =
                IORLW_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 12 {
            if let Some((inst_len, parsed)) =
                MOVLW_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 8 {
            if let Some((inst_len, parsed)) =
                RETLW_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 15) == 15 {
            if let Some((inst_len, parsed)) =
                XORLW_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[1] & 14) == 10 {
            if let Some((inst_len, parsed)) =
                GOTO_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:120:1, end:120:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar0 {}
impl fREGLocVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("INDF")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:130:1, end:130:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar1 {}
impl fREGLocVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("STATUS")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:131:1, end:131:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar2 {}
impl fREGLocVar2 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("FSR")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:132:1, end:132:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar3 {}
impl fREGLocVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PCL")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:126:1, end:126:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar4 {
    f5: u8,
}
impl fREGLocVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.f5 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let f5 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { f5 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:114:1, end:114:8))"]
#[derive(Clone, Debug)]
struct fREGLocVar5 {
    f5: u8,
}
impl fREGLocVar5 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.f5 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let f5 = token_7(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { f5 }))
    }
}
#[derive(Clone, Debug)]
enum TablefREGLoc {
    Var0(fREGLocVar0),
    Var1(fREGLocVar1),
    Var2(fREGLocVar2),
    Var3(fREGLocVar3),
    Var4(fREGLocVar4),
    Var5(fREGLocVar5),
}
impl TablefREGLoc {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 0 {
            if let Some((inst_len, parsed)) =
                fREGLocVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 3 {
            if let Some((inst_len, parsed)) =
                fREGLocVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 4 {
            if let Some((inst_len, parsed)) =
                fREGLocVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 2 {
            if let Some((inst_len, parsed)) =
                fREGLocVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 16) == 0 {
            if let Some((inst_len, parsed)) =
                fREGLocVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                fREGLocVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:138:1, end:138:7))"]
#[derive(Clone, Debug)]
struct srcREGVar0 {}
impl srcREGVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PCL")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:135:1, end:135:7))"]
#[derive(Clone, Debug)]
struct srcREGVar1 {
    fREGLoc: TablefREGLoc,
}
impl srcREGVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        self.fREGLoc
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let fREGLoc = if let Some((len, table)) =
            TablefREGLoc::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { fREGLoc }))
    }
}
#[derive(Clone, Debug)]
enum TablesrcREG {
    Var0(srcREGVar0),
    Var1(srcREGVar1),
}
impl TablesrcREG {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 2 {
            if let Some((inst_len, parsed)) =
                srcREGVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 {
            if let Some((inst_len, parsed)) =
                srcREGVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:148:1, end:148:8))"]
#[derive(Clone, Debug)]
struct destREGVar0 {}
impl destREGVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("0")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:149:1, end:149:8))"]
#[derive(Clone, Debug)]
struct destREGVar1 {
    srcREG: TablesrcREG,
}
impl destREGVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("1")];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let srcREG = if let Some((len, table)) =
            TablesrcREG::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { srcREG }))
    }
}
#[derive(Clone, Debug)]
enum TabledestREG {
    Var0(destREGVar0),
    Var1(destREGVar1),
}
impl TabledestREG {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                destREGVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                destREGVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:164:1, end:164:2))"]
#[derive(Clone, Debug)]
struct DVar0 {}
impl DVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("w")];
        display.extend_from_slice(&extend);
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:165:1, end:165:2))"]
#[derive(Clone, Debug)]
struct DVar1 {}
impl DVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("f")];
        display.extend_from_slice(&extend);
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
enum TableD {
    Var0(DVar0),
    Var1(DVar1),
}
impl TableD {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 32) == 0 {
            if let Some((inst_len, parsed)) =
                DVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 32) == 32 {
            if let Some((inst_len, parsed)) =
                DVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:168:1, end:168:9))"]
#[derive(Clone, Debug)]
struct absAddr8Var0 {
    k8: u8,
}
impl absAddr8Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.k8 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let k8 = token_9(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k8 }))
    }
}
#[derive(Clone, Debug)]
enum TableabsAddr8 {
    Var0(absAddr8Var0),
}
impl TableabsAddr8 {
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
                absAddr8Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:172:1, end:172:9))"]
#[derive(Clone, Debug)]
struct absAddr9Var0 {
    k9: u16,
}
impl absAddr9Var0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.k9 as u64)];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let k9 = token_10(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k9 }))
    }
}
#[derive(Clone, Debug)]
enum TableabsAddr9 {
    Var0(absAddr9Var0),
}
impl TableabsAddr9 {
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
                absAddr9Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:178:1, end:178:9))"]
#[derive(Clone, Debug)]
struct skipInstVar0 {}
impl skipInstVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_inst_skip: i128 = 0;
        calc_inst_skip = i128::try_from(inst_next).unwrap().wrapping_add(1i128);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Number(
            true,
            calc_inst_skip.is_negative(),
            calc_inst_skip.abs() as u64,
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
        let mut calc_inst_skip: i128 = 0;
        let mut block_0_len = 2;
        let op12 = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum TableskipInst {
    Var0(skipInstVar0),
}
impl TableskipInst {
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
                skipInstVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:181:1, end:181:5))"]
#[derive(Clone, Debug)]
struct imm8Var0 {
    k8: u8,
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
            DisplayElement::Number(true, false, self.k8 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let k8 = token_9(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { k8 }))
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:184:1, end:184:4))"]
#[derive(Clone, Debug)]
struct bitVar0 {
    b3: u8,
}
impl bitVar0 {
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
            DisplayElement::Number(true, false, self.b3 as u64),
        ];
        display.extend_from_slice(&extend);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let b3 = token_6(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { b3 }))
    }
}
#[derive(Clone, Debug)]
enum Tablebit {
    Var0(bitVar0),
}
impl Tablebit {
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
                bitVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:187:1, end:187:4))"]
#[derive(Clone, Debug)]
struct pclVar0 {}
impl pclVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("PC")];
        display.extend_from_slice(&extend);
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
enum Tablepcl {
    Var0(pclVar0),
}
impl Tablepcl {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 2 {
            if let Some((inst_len, parsed)) =
                pclVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12_instructions.sinc, start:190:1, end:190:7))"]
#[derive(Clone, Debug)]
struct statusVar0 {}
impl statusVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("STATUS")];
        display.extend_from_slice(&extend);
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
enum Tablestatus {
    Var0(statusVar0),
}
impl Tablestatus {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 3 {
            if let Some((inst_len, parsed)) =
                statusVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/PIC/data/languages/pic12c5xx.slaspec, start:22:1, end:22:8))"]
#[derive(Clone, Debug)]
struct trisREGVar0 {}
impl trisREGVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal("6")];
        display.extend_from_slice(&extend);
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
enum TabletrisREG {
    Var0(trisREGVar0),
}
impl TabletrisREG {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 31) == 6 {
            if let Some((inst_len, parsed)) =
                trisREGVar0::parse(tokens_param, &mut context_current, inst_start)
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
