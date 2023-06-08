pub type AddrType = u16;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    A,
    X,
    PC,
    SP,
    H,
    I,
    N,
    Z,
    C,
    SWI_VECTOR,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::X => "X",
            Self::PC => "PC",
            Self::SP => "SP",
            Self::H => "H",
            Self::I => "I",
            Self::N => "N",
            Self::Z => "Z",
            Self::C => "C",
            Self::SWI_VECTOR => "SWI_VECTOR",
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
#[doc = "Create token_fields: op imm8 rel"]
fn token_1(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 255) as u8)
}
#[doc = "Create token_fields: imm16"]
fn token_6(tokens: &[u8]) -> u16 {
    (((u16::from_be_bytes(tokens[0..2].try_into().unwrap()) >> 0) & 65535) as u16)
}
#[doc = "Create token_fields: op4_6"]
fn token_3(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 7) as u8)
}
#[doc = "Create token_fields: bit_0"]
fn token_5(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 0) & 1) as u8)
}
#[doc = "Create token_fields: n"]
fn token_4(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 1) & 7) as u8)
}
#[doc = "Create token_fields: op4_7"]
fn token_2(tokens: &[u8]) -> u8 {
    (((u8::from_be_bytes(tokens[0..1].try_into().unwrap()) >> 4) & 15) as u8)
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:56:1, end:56:2))"]
#[derive(Clone, Debug)]
struct ADC_instructionVar0 {
    OP1: TableOP1,
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 169 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 185 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 201 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 217 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 233 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 249 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:75:1, end:75:2))"]
#[derive(Clone, Debug)]
struct ADD_instructionVar1 {
    OP1: TableOP1,
}
impl ADD_instructionVar1 {
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
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 171 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 187 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 203 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 219 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 235 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 251 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:91:1, end:91:2))"]
#[derive(Clone, Debug)]
struct AND_instructionVar2 {
    OP1: TableOP1,
}
impl AND_instructionVar2 {
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
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 164 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 180 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 196 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 212 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 228 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 244 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:98:1, end:98:2))"]
#[derive(Clone, Debug)]
struct ASLA_instructionVar3 {}
impl ASLA_instructionVar3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASLA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:105:1, end:105:2))"]
#[derive(Clone, Debug)]
struct ASLX_instructionVar4 {}
impl ASLX_instructionVar4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASLX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:112:1, end:112:2))"]
#[derive(Clone, Debug)]
struct ASL_instructionVar5 {
    OP1: TableOP1,
}
impl ASL_instructionVar5 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 56 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 104 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 120 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:121:1, end:121:2))"]
#[derive(Clone, Debug)]
struct ASRA_instructionVar6 {}
impl ASRA_instructionVar6 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASRA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:128:1, end:128:2))"]
#[derive(Clone, Debug)]
struct ASRX_instructionVar7 {}
impl ASRX_instructionVar7 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ASRX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:135:1, end:135:2))"]
#[derive(Clone, Debug)]
struct ASR_instructionVar8 {
    OP1: TableOP1,
}
impl ASR_instructionVar8 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 55 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 103 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 119 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:145:1, end:145:2))"]
#[derive(Clone, Debug)]
struct BCC_instructionVar9 {
    REL: TableREL,
}
impl BCC_instructionVar9 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:154:1, end:154:2))"]
#[derive(Clone, Debug)]
struct BCS_instructionVar10 {
    REL: TableREL,
}
impl BCS_instructionVar10 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:158:1, end:158:2))"]
#[derive(Clone, Debug)]
struct BEQ_instructionVar11 {
    REL: TableREL,
}
impl BEQ_instructionVar11 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BEQ"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:162:1, end:162:2))"]
#[derive(Clone, Debug)]
struct BHCC_instructionVar12 {
    REL: TableREL,
}
impl BHCC_instructionVar12 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BHCC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:166:1, end:166:2))"]
#[derive(Clone, Debug)]
struct BHCS_instructionVar13 {
    REL: TableREL,
}
impl BHCS_instructionVar13 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BHCS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:170:1, end:170:2))"]
#[derive(Clone, Debug)]
struct BHI_instructionVar14 {
    REL: TableREL,
}
impl BHI_instructionVar14 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BHI"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:178:1, end:178:2))"]
#[derive(Clone, Debug)]
struct BIH_instructionVar15 {
    REL: TableREL,
}
impl BIH_instructionVar15 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIH"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:183:1, end:183:2))"]
#[derive(Clone, Debug)]
struct BIL_instructionVar16 {
    REL: TableREL,
}
impl BIL_instructionVar16 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:188:1, end:188:2))"]
#[derive(Clone, Debug)]
struct BIT_instructionVar17 {
    OP1: TableOP1,
}
impl BIT_instructionVar17 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BIT"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 165 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 181 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 197 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 213 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 229 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 245 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:196:1, end:196:2))"]
#[derive(Clone, Debug)]
struct BLS_instructionVar18 {
    REL: TableREL,
}
impl BLS_instructionVar18 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BLS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:202:1, end:202:2))"]
#[derive(Clone, Debug)]
struct BMC_instructionVar19 {
    REL: TableREL,
}
impl BMC_instructionVar19 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BMC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:206:1, end:206:2))"]
#[derive(Clone, Debug)]
struct BMI_instructionVar20 {
    REL: TableREL,
}
impl BMI_instructionVar20 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BMI"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:210:1, end:210:2))"]
#[derive(Clone, Debug)]
struct BMS_instructionVar21 {
    REL: TableREL,
}
impl BMS_instructionVar21 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BMS"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:214:1, end:214:2))"]
#[derive(Clone, Debug)]
struct BNE_instructionVar22 {
    REL: TableREL,
}
impl BNE_instructionVar22 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BNE"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:218:1, end:218:2))"]
#[derive(Clone, Debug)]
struct BPL_instructionVar23 {
    REL: TableREL,
}
impl BPL_instructionVar23 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BPL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:222:1, end:222:2))"]
#[derive(Clone, Debug)]
struct BRA_instructionVar24 {
    REL: TableREL,
}
impl BRA_instructionVar24 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BRA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:227:1, end:227:2))"]
#[derive(Clone, Debug)]
struct BRN_instructionVar25 {
    REL: TableREL,
}
impl BRN_instructionVar25 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BRN"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:250:1, end:250:2))"]
#[derive(Clone, Debug)]
struct BSR_instructionVar26 {
    REL: TableREL,
}
impl BSR_instructionVar26 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.REL
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
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { REL }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:258:1, end:258:2))"]
#[derive(Clone, Debug)]
struct CLC_instructionVar27 {}
impl CLC_instructionVar27 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLC"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:263:1, end:263:2))"]
#[derive(Clone, Debug)]
struct CLI_instructionVar28 {}
impl CLI_instructionVar28 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLI"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:267:1, end:267:2))"]
#[derive(Clone, Debug)]
struct CLRA_instructionVar29 {}
impl CLRA_instructionVar29 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:273:1, end:273:2))"]
#[derive(Clone, Debug)]
struct CLRX_instructionVar30 {}
impl CLRX_instructionVar30 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLRX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:279:1, end:279:2))"]
#[derive(Clone, Debug)]
struct CLR_instructionVar31 {
    OP1: TableOP1,
}
impl CLR_instructionVar31 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CLR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 63 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 111 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 127 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:285:1, end:285:2))"]
#[derive(Clone, Debug)]
struct CMP_instructionVar32 {
    OP1: TableOP1,
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
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 161 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 177 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 193 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 209 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 225 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 241 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:294:1, end:294:2))"]
#[derive(Clone, Debug)]
struct COMA_instructionVar33 {}
impl COMA_instructionVar33 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("COMA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:301:1, end:301:2))"]
#[derive(Clone, Debug)]
struct COMX_instructionVar34 {}
impl COMX_instructionVar34 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("COMX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:308:1, end:308:2))"]
#[derive(Clone, Debug)]
struct COM_instructionVar35 {
    OP1: TableOP1,
}
impl COM_instructionVar35 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("COM"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 51 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 99 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 115 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:316:1, end:316:2))"]
#[derive(Clone, Debug)]
struct CPX_instructionVar36 {
    OP1: TableOP1,
}
impl CPX_instructionVar36 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("CPX"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 163 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 179 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 195 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 211 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 227 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 243 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:324:1, end:324:2))"]
#[derive(Clone, Debug)]
struct DECA_instructionVar37 {}
impl DECA_instructionVar37 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:330:1, end:330:2))"]
#[derive(Clone, Debug)]
struct DECX_instructionVar38 {}
impl DECX_instructionVar38 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("DECX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:336:1, end:336:2))"]
#[derive(Clone, Debug)]
struct DEC_instructionVar39 {
    OP1: TableOP1,
}
impl DEC_instructionVar39 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 58 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 106 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 122 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:343:1, end:343:2))"]
#[derive(Clone, Debug)]
struct EOR_instructionVar40 {
    OP1: TableOP1,
}
impl EOR_instructionVar40 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("EOR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 168 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 184 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 200 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 216 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 232 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 248 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:350:1, end:350:2))"]
#[derive(Clone, Debug)]
struct INCA_instructionVar41 {}
impl INCA_instructionVar41 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:356:1, end:356:2))"]
#[derive(Clone, Debug)]
struct INCX_instructionVar42 {}
impl INCX_instructionVar42 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("INCX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:362:1, end:362:2))"]
#[derive(Clone, Debug)]
struct INC_instructionVar43 {
    OP1: TableOP1,
}
impl INC_instructionVar43 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 60 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 108 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 124 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:369:1, end:369:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar44 {
    ADDR: TableADDR,
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
        self.ADDR
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 188 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 204 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDR = if let Some((len, table)) =
            TableADDR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADDR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:374:1, end:374:2))"]
#[derive(Clone, Debug)]
struct JMP_instructionVar45 {
    ADDRI: TableADDRI,
}
impl JMP_instructionVar45 {
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
        self.ADDRI
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
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 220 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 236 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 252 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDRI = if let Some((len, table)) =
            TableADDRI::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADDRI }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:380:1, end:380:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar46 {
    ADDR: TableADDR,
}
impl JSR_instructionVar46 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JSR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ADDR
            .display_extend(display, context, inst_start, inst_next, global_set);
    }
    fn parse(
        mut tokens_current: &[u8],
        context: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut pattern_len = 0;
        let mut context_instance = context.clone();
        let mut block_0_len = 2;
        let mut sub_pattern_c14 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 189 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 205 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c14(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDR = if let Some((len, table)) =
            TableADDR::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADDR }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:386:1, end:386:2))"]
#[derive(Clone, Debug)]
struct JSR_instructionVar47 {
    ADDRI: TableADDRI,
}
impl JSR_instructionVar47 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("JSR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.ADDRI
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
        let mut sub_pattern_c15 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 221 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 237 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 253 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c15(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let ADDRI = if let Some((len, table)) =
            TableADDRI::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { ADDRI }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:393:1, end:393:2))"]
#[derive(Clone, Debug)]
struct LDA_instructionVar48 {
    OP1: TableOP1,
}
impl LDA_instructionVar48 {
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
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 166 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 182 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 198 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 214 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 230 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 246 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:400:1, end:400:2))"]
#[derive(Clone, Debug)]
struct LDX_instructionVar49 {
    OP1: TableOP1,
}
impl LDX_instructionVar49 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LDX"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 174 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 190 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 206 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 222 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 238 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 254 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:411:1, end:411:2))"]
#[derive(Clone, Debug)]
struct LSRA_instructionVar50 {}
impl LSRA_instructionVar50 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LSRA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:418:1, end:418:2))"]
#[derive(Clone, Debug)]
struct LSRX_instructionVar51 {}
impl LSRX_instructionVar51 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LSRX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:425:1, end:425:2))"]
#[derive(Clone, Debug)]
struct LSR_instructionVar52 {
    OP1: TableOP1,
}
impl LSR_instructionVar52 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("LSR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 52 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 100 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 116 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:435:1, end:435:2))"]
#[derive(Clone, Debug)]
struct MUL_instructionVar53 {}
impl MUL_instructionVar53 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("MUL"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:445:1, end:445:2))"]
#[derive(Clone, Debug)]
struct NEGA_instructionVar54 {}
impl NEGA_instructionVar54 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEGA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:452:1, end:452:2))"]
#[derive(Clone, Debug)]
struct NEGX_instructionVar55 {}
impl NEGX_instructionVar55 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEGX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:459:1, end:459:2))"]
#[derive(Clone, Debug)]
struct NEG_instructionVar56 {
    OP1: TableOP1,
}
impl NEG_instructionVar56 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("NEG"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 48 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 96 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 112 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:468:1, end:468:2))"]
#[derive(Clone, Debug)]
struct NOP_instructionVar57 {}
impl NOP_instructionVar57 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:472:1, end:472:2))"]
#[derive(Clone, Debug)]
struct ORA_instructionVar58 {
    OP1: TableOP1,
}
impl ORA_instructionVar58 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ORA"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 170 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 186 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 202 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 218 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 234 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 250 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:479:1, end:479:2))"]
#[derive(Clone, Debug)]
struct ROLA_instructionVar59 {}
impl ROLA_instructionVar59 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ROLA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:488:1, end:488:2))"]
#[derive(Clone, Debug)]
struct ROLX_instructionVar60 {}
impl ROLX_instructionVar60 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ROLX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:497:1, end:497:2))"]
#[derive(Clone, Debug)]
struct ROL_instructionVar61 {
    OP1: TableOP1,
}
impl ROL_instructionVar61 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ROL"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 57 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 105 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 121 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:508:1, end:508:2))"]
#[derive(Clone, Debug)]
struct RORA_instructionVar62 {}
impl RORA_instructionVar62 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RORA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:517:1, end:517:2))"]
#[derive(Clone, Debug)]
struct RORX_instructionVar63 {}
impl RORX_instructionVar63 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RORX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:526:1, end:526:2))"]
#[derive(Clone, Debug)]
struct ROR_instructionVar64 {
    OP1: TableOP1,
}
impl ROR_instructionVar64 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("ROR"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 54 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 102 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 118 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:538:1, end:538:2))"]
#[derive(Clone, Debug)]
struct RSP_instructionVar65 {}
impl RSP_instructionVar65 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RSP"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:543:1, end:543:2))"]
#[derive(Clone, Debug)]
struct RTI_instructionVar66 {}
impl RTI_instructionVar66 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RTI"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:565:1, end:565:2))"]
#[derive(Clone, Debug)]
struct RTS_instructionVar67 {}
impl RTS_instructionVar67 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("RTS"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:574:1, end:574:2))"]
#[derive(Clone, Debug)]
struct SBC_instructionVar68 {
    OP1: TableOP1,
}
impl SBC_instructionVar68 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SBC"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 162 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 178 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 194 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 210 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 226 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 242 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:584:1, end:584:2))"]
#[derive(Clone, Debug)]
struct SEC_instructionVar69 {}
impl SEC_instructionVar69 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SEC"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:589:1, end:589:2))"]
#[derive(Clone, Debug)]
struct SEI_instructionVar70 {}
impl SEI_instructionVar70 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SEI"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:594:1, end:594:2))"]
#[derive(Clone, Debug)]
struct STA_instructionVar71 {
    OP1: TableOP1,
}
impl STA_instructionVar71 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 183 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 199 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 215 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 231 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 247 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:601:1, end:601:2))"]
#[derive(Clone, Debug)]
struct STOP_instructionVar72 {}
impl STOP_instructionVar72 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STOP"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:606:1, end:606:2))"]
#[derive(Clone, Debug)]
struct STX_instructionVar73 {
    OP1: TableOP1,
}
impl STX_instructionVar73 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("STX"));
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(" ")];
        display.extend_from_slice(&extend);
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 191 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 207 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 223 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 239 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 255 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:613:1, end:613:2))"]
#[derive(Clone, Debug)]
struct SUB_instructionVar74 {
    OP1: TableOP1,
}
impl SUB_instructionVar74 {
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
        self.OP1
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
        let mut sub_pattern_c17 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 160 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 176 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 192 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 208 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 224 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 240 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c17(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:623:1, end:623:2))"]
#[derive(Clone, Debug)]
struct SWI_instructionVar75 {}
impl SWI_instructionVar75 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("SWI"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:638:1, end:638:2))"]
#[derive(Clone, Debug)]
struct TAX_instructionVar76 {}
impl TAX_instructionVar76 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TAX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:643:1, end:643:2))"]
#[derive(Clone, Debug)]
struct TSTA_instructionVar77 {}
impl TSTA_instructionVar77 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TSTA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:648:1, end:648:2))"]
#[derive(Clone, Debug)]
struct TSTX_instructionVar78 {}
impl TSTX_instructionVar78 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TSTX"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:653:1, end:653:2))"]
#[derive(Clone, Debug)]
struct TST_instructionVar79 {
    OP1: TableOP1,
}
impl TST_instructionVar79 {
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
        self.OP1
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
        let mut sub_pattern_c13 = |tokens: &[u8], context_param: &mut ContextMemory| {
            let mut pattern_len = 0;
            let mut context_instance = context_param.clone();
            let mut tokens = tokens;
            let block_0 = |tokens_param: &[u8], context_param: &mut ContextMemory| {
                if token_1(tokens_param) == 61 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 109 {
                    return Some(((), (), 1));
                }
                if token_1(tokens_param) == 125 {
                    return Some(((), (), 1));
                }
                None
            };
            let ((), (), block_0_len) = block_0(tokens, &mut context_instance)?;
            pattern_len += block_0_len;
            tokens = &tokens[usize::try_from(block_0_len).unwrap()..];
            *context_param = context_instance;
            Some(((), (), pattern_len))
        };
        let ((), (), sub_len) = sub_pattern_c13(tokens_current, &mut context_instance)?;
        block_0_len = block_0_len.max(sub_len);
        let OP1 = if let Some((len, table)) =
            TableOP1::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_0_len = block_0_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { OP1 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:660:1, end:660:2))"]
#[derive(Clone, Debug)]
struct TXA_instructionVar80 {}
impl TXA_instructionVar80 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("TXA"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:665:1, end:665:2))"]
#[derive(Clone, Debug)]
struct WAIT_instructionVar81 {}
impl WAIT_instructionVar81 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("WAIT"));
    }
    fn parse(
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:150:1, end:150:2))"]
#[derive(Clone, Debug)]
struct BCLR_instructionVar82 {
    n: u8,
    DIRECT: TableDIRECT,
}
impl BCLR_instructionVar82 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BCLR"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.n as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DIRECT
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
        let n = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let DIRECT = if let Some((len, table)) =
            TableDIRECT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DIRECT, n }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:231:1, end:231:2))"]
#[derive(Clone, Debug)]
struct BRCLR_instructionVar83 {
    n: u8,
    DIRECT: TableDIRECT,
    REL: TableREL,
}
impl BRCLR_instructionVar83 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BRCLR"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.n as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DIRECT
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.REL
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
        let n = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let DIRECT = if let Some((len, table)) =
            TableDIRECT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DIRECT, REL, n }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:238:1, end:238:2))"]
#[derive(Clone, Debug)]
struct BRSET_instructionVar84 {
    n: u8,
    DIRECT: TableDIRECT,
    REL: TableREL,
}
impl BRSET_instructionVar84 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BRSET"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.n as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DIRECT
            .display_extend(display, context, inst_start, inst_next, global_set);
        let extend: [DisplayElement; 1usize] = [<DisplayElement>::Literal(",")];
        display.extend_from_slice(&extend);
        self.REL
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
        let n = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let DIRECT = if let Some((len, table)) =
            TableDIRECT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        let mut block_2_len = 1;
        let REL = if let Some((len, table)) =
            TableREL::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_2_len = block_2_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_2_len;
        tokens_current = &tokens_current[usize::try_from(block_2_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DIRECT, REL, n }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:245:1, end:245:2))"]
#[derive(Clone, Debug)]
struct BSET_instructionVar85 {
    n: u8,
    DIRECT: TableDIRECT,
}
impl BSET_instructionVar85 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        display.push(DisplayElement::Literal("BSET"));
        let extend: [DisplayElement; 3usize] = [
            <DisplayElement>::Literal(" "),
            DisplayElement::Number(true, false, self.n as u64),
            <DisplayElement>::Literal(","),
        ];
        display.extend_from_slice(&extend);
        self.DIRECT
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
        let n = token_4(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 1;
        let DIRECT = if let Some((len, table)) =
            TableDIRECT::parse(tokens_current, &mut context_instance, inst_start)
        {
            block_1_len = block_1_len.max(len as AddrType);
            table
        } else {
            return None;
        };
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { DIRECT, n }))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(ADC_instructionVar0),
    Var1(ADD_instructionVar1),
    Var2(AND_instructionVar2),
    Var3(ASLA_instructionVar3),
    Var4(ASLX_instructionVar4),
    Var5(ASL_instructionVar5),
    Var6(ASRA_instructionVar6),
    Var7(ASRX_instructionVar7),
    Var8(ASR_instructionVar8),
    Var9(BCC_instructionVar9),
    Var10(BCS_instructionVar10),
    Var11(BEQ_instructionVar11),
    Var12(BHCC_instructionVar12),
    Var13(BHCS_instructionVar13),
    Var14(BHI_instructionVar14),
    Var15(BIH_instructionVar15),
    Var16(BIL_instructionVar16),
    Var17(BIT_instructionVar17),
    Var18(BLS_instructionVar18),
    Var19(BMC_instructionVar19),
    Var20(BMI_instructionVar20),
    Var21(BMS_instructionVar21),
    Var22(BNE_instructionVar22),
    Var23(BPL_instructionVar23),
    Var24(BRA_instructionVar24),
    Var25(BRN_instructionVar25),
    Var26(BSR_instructionVar26),
    Var27(CLC_instructionVar27),
    Var28(CLI_instructionVar28),
    Var29(CLRA_instructionVar29),
    Var30(CLRX_instructionVar30),
    Var31(CLR_instructionVar31),
    Var32(CMP_instructionVar32),
    Var33(COMA_instructionVar33),
    Var34(COMX_instructionVar34),
    Var35(COM_instructionVar35),
    Var36(CPX_instructionVar36),
    Var37(DECA_instructionVar37),
    Var38(DECX_instructionVar38),
    Var39(DEC_instructionVar39),
    Var40(EOR_instructionVar40),
    Var41(INCA_instructionVar41),
    Var42(INCX_instructionVar42),
    Var43(INC_instructionVar43),
    Var44(JMP_instructionVar44),
    Var45(JMP_instructionVar45),
    Var46(JSR_instructionVar46),
    Var47(JSR_instructionVar47),
    Var48(LDA_instructionVar48),
    Var49(LDX_instructionVar49),
    Var50(LSRA_instructionVar50),
    Var51(LSRX_instructionVar51),
    Var52(LSR_instructionVar52),
    Var53(MUL_instructionVar53),
    Var54(NEGA_instructionVar54),
    Var55(NEGX_instructionVar55),
    Var56(NEG_instructionVar56),
    Var57(NOP_instructionVar57),
    Var58(ORA_instructionVar58),
    Var59(ROLA_instructionVar59),
    Var60(ROLX_instructionVar60),
    Var61(ROL_instructionVar61),
    Var62(RORA_instructionVar62),
    Var63(RORX_instructionVar63),
    Var64(ROR_instructionVar64),
    Var65(RSP_instructionVar65),
    Var66(RTI_instructionVar66),
    Var67(RTS_instructionVar67),
    Var68(SBC_instructionVar68),
    Var69(SEC_instructionVar69),
    Var70(SEI_instructionVar70),
    Var71(STA_instructionVar71),
    Var72(STOP_instructionVar72),
    Var73(STX_instructionVar73),
    Var74(SUB_instructionVar74),
    Var75(SWI_instructionVar75),
    Var76(TAX_instructionVar76),
    Var77(TSTA_instructionVar77),
    Var78(TSTX_instructionVar78),
    Var79(TST_instructionVar79),
    Var80(TXA_instructionVar80),
    Var81(WAIT_instructionVar81),
    Var82(BCLR_instructionVar82),
    Var83(BRCLR_instructionVar83),
    Var84(BRSET_instructionVar84),
    Var85(BSET_instructionVar85),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 137 {
            if let Some((inst_len, parsed)) =
                ADC_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 139 {
            if let Some((inst_len, parsed)) =
                ADD_instructionVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 132 {
            if let Some((inst_len, parsed)) =
                AND_instructionVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 72 {
            if let Some((inst_len, parsed)) =
                ASLA_instructionVar3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 88 {
            if let Some((inst_len, parsed)) =
                ASLX_instructionVar4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 40 {
            if let Some((inst_len, parsed)) =
                ASL_instructionVar5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 71 {
            if let Some((inst_len, parsed)) =
                ASRA_instructionVar6::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var6(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 87 {
            if let Some((inst_len, parsed)) =
                ASRX_instructionVar7::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var7(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 39 {
            if let Some((inst_len, parsed)) =
                ASR_instructionVar8::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var8(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 36 {
            if let Some((inst_len, parsed)) =
                BCC_instructionVar9::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var9(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 37 {
            if let Some((inst_len, parsed)) =
                BCS_instructionVar10::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var10(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 39 {
            if let Some((inst_len, parsed)) =
                BEQ_instructionVar11::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var11(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 40 {
            if let Some((inst_len, parsed)) =
                BHCC_instructionVar12::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var12(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 41 {
            if let Some((inst_len, parsed)) =
                BHCS_instructionVar13::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var13(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 34 {
            if let Some((inst_len, parsed)) =
                BHI_instructionVar14::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var14(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 47 {
            if let Some((inst_len, parsed)) =
                BIH_instructionVar15::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var15(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 46 {
            if let Some((inst_len, parsed)) =
                BIL_instructionVar16::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var16(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 133 {
            if let Some((inst_len, parsed)) =
                BIT_instructionVar17::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var17(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 35 {
            if let Some((inst_len, parsed)) =
                BLS_instructionVar18::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var18(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 44 {
            if let Some((inst_len, parsed)) =
                BMC_instructionVar19::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var19(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 43 {
            if let Some((inst_len, parsed)) =
                BMI_instructionVar20::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var20(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 45 {
            if let Some((inst_len, parsed)) =
                BMS_instructionVar21::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var21(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 38 {
            if let Some((inst_len, parsed)) =
                BNE_instructionVar22::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var22(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 42 {
            if let Some((inst_len, parsed)) =
                BPL_instructionVar23::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var23(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 32 {
            if let Some((inst_len, parsed)) =
                BRA_instructionVar24::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var24(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 33 {
            if let Some((inst_len, parsed)) =
                BRN_instructionVar25::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var25(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 255) == 173 {
            if let Some((inst_len, parsed)) =
                BSR_instructionVar26::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var26(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 152 {
            if let Some((inst_len, parsed)) =
                CLC_instructionVar27::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var27(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 154 {
            if let Some((inst_len, parsed)) =
                CLI_instructionVar28::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var28(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 79 {
            if let Some((inst_len, parsed)) =
                CLRA_instructionVar29::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var29(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 95 {
            if let Some((inst_len, parsed)) =
                CLRX_instructionVar30::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var30(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 47 {
            if let Some((inst_len, parsed)) =
                CLR_instructionVar31::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var31(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 129 {
            if let Some((inst_len, parsed)) =
                CMP_instructionVar32::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var32(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 67 {
            if let Some((inst_len, parsed)) =
                COMA_instructionVar33::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var33(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 83 {
            if let Some((inst_len, parsed)) =
                COMX_instructionVar34::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var34(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 35 {
            if let Some((inst_len, parsed)) =
                COM_instructionVar35::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var35(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 131 {
            if let Some((inst_len, parsed)) =
                CPX_instructionVar36::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var36(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 74 {
            if let Some((inst_len, parsed)) =
                DECA_instructionVar37::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var37(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 90 {
            if let Some((inst_len, parsed)) =
                DECX_instructionVar38::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var38(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 42 {
            if let Some((inst_len, parsed)) =
                DEC_instructionVar39::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var39(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 136 {
            if let Some((inst_len, parsed)) =
                EOR_instructionVar40::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var40(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 76 {
            if let Some((inst_len, parsed)) =
                INCA_instructionVar41::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var41(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 92 {
            if let Some((inst_len, parsed)) =
                INCX_instructionVar42::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var42(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 44 {
            if let Some((inst_len, parsed)) =
                INC_instructionVar43::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var43(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 143) == 140 {
            if let Some((inst_len, parsed)) =
                JMP_instructionVar44::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var44(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 204 {
            if let Some((inst_len, parsed)) =
                JMP_instructionVar45::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var45(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 143) == 141 {
            if let Some((inst_len, parsed)) =
                JSR_instructionVar46::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var46(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 207) == 205 {
            if let Some((inst_len, parsed)) =
                JSR_instructionVar47::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var47(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 134 {
            if let Some((inst_len, parsed)) =
                LDA_instructionVar48::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var48(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 142 {
            if let Some((inst_len, parsed)) =
                LDX_instructionVar49::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var49(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 68 {
            if let Some((inst_len, parsed)) =
                LSRA_instructionVar50::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var50(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 84 {
            if let Some((inst_len, parsed)) =
                LSRX_instructionVar51::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var51(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 36 {
            if let Some((inst_len, parsed)) =
                LSR_instructionVar52::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var52(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 66 {
            if let Some((inst_len, parsed)) =
                MUL_instructionVar53::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var53(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 64 {
            if let Some((inst_len, parsed)) =
                NEGA_instructionVar54::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var54(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 80 {
            if let Some((inst_len, parsed)) =
                NEGX_instructionVar55::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var55(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 32 {
            if let Some((inst_len, parsed)) =
                NEG_instructionVar56::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var56(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 157 {
            if let Some((inst_len, parsed)) =
                NOP_instructionVar57::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var57(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 138 {
            if let Some((inst_len, parsed)) =
                ORA_instructionVar58::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var58(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 73 {
            if let Some((inst_len, parsed)) =
                ROLA_instructionVar59::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var59(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 89 {
            if let Some((inst_len, parsed)) =
                ROLX_instructionVar60::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var60(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 41 {
            if let Some((inst_len, parsed)) =
                ROL_instructionVar61::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var61(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 70 {
            if let Some((inst_len, parsed)) =
                RORA_instructionVar62::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var62(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 86 {
            if let Some((inst_len, parsed)) =
                RORX_instructionVar63::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var63(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 38 {
            if let Some((inst_len, parsed)) =
                ROR_instructionVar64::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var64(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 156 {
            if let Some((inst_len, parsed)) =
                RSP_instructionVar65::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var65(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 128 {
            if let Some((inst_len, parsed)) =
                RTI_instructionVar66::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var66(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 129 {
            if let Some((inst_len, parsed)) =
                RTS_instructionVar67::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var67(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 130 {
            if let Some((inst_len, parsed)) =
                SBC_instructionVar68::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var68(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 153 {
            if let Some((inst_len, parsed)) =
                SEC_instructionVar69::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var69(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 155 {
            if let Some((inst_len, parsed)) =
                SEI_instructionVar70::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var70(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 135 {
            if let Some((inst_len, parsed)) =
                STA_instructionVar71::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var71(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 142 {
            if let Some((inst_len, parsed)) =
                STOP_instructionVar72::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var72(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 143 {
            if let Some((inst_len, parsed)) =
                STX_instructionVar73::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var73(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 143) == 128 {
            if let Some((inst_len, parsed)) =
                SUB_instructionVar74::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var74(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 131 {
            if let Some((inst_len, parsed)) =
                SWI_instructionVar75::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var75(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 151 {
            if let Some((inst_len, parsed)) =
                TAX_instructionVar76::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var76(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 77 {
            if let Some((inst_len, parsed)) =
                TSTA_instructionVar77::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var77(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 93 {
            if let Some((inst_len, parsed)) =
                TSTX_instructionVar78::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var78(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 175) == 45 {
            if let Some((inst_len, parsed)) =
                TST_instructionVar79::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var79(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 159 {
            if let Some((inst_len, parsed)) =
                TXA_instructionVar80::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var80(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 255) == 143 {
            if let Some((inst_len, parsed)) =
                WAIT_instructionVar81::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var81(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 241) == 17 {
            if let Some((inst_len, parsed)) =
                BCLR_instructionVar82::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var82(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 241) == 1 {
            if let Some((inst_len, parsed)) =
                BRCLR_instructionVar83::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var83(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 241) == 0 {
            if let Some((inst_len, parsed)) =
                BRSET_instructionVar84::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var84(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 241) == 16 {
            if let Some((inst_len, parsed)) =
                BSET_instructionVar85::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var85(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:38:1, end:38:4))"]
#[derive(Clone, Debug)]
struct RELVar0 {
    rel: u8,
}
impl RELVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let mut calc_reloc: i128 = 0;
        calc_reloc = i128::try_from(inst_next).unwrap().wrapping_add(
            i128::try_from((if self.rel & 128 != 0 { -1 & !127 } else { 0 } | self.rel as i8))
                .unwrap(),
        );
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
        let mut block_0_len = 1;
        let rel = token_1(tokens_current);
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { rel }))
    }
}
#[derive(Clone, Debug)]
enum TableREL {
    Var0(RELVar0),
}
impl TableREL {
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
                RELVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:40:1, end:40:4))"]
#[derive(Clone, Debug)]
struct OP1Var0 {
    imm8: u8,
}
impl OP1Var0 {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:41:1, end:41:4))"]
#[derive(Clone, Debug)]
struct OP1Var1 {
    imm8: u8,
}
impl OP1Var1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.imm8 as u64)];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:42:1, end:42:4))"]
#[derive(Clone, Debug)]
struct OP1Var2 {
    imm16: u16,
}
impl OP1Var2 {
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:43:1, end:43:4))"]
#[derive(Clone, Debug)]
struct OP1Var3 {
    imm16: u16,
}
impl OP1Var3 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
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
        let mut block_1_len = 2;
        let imm16 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:44:1, end:44:4))"]
#[derive(Clone, Debug)]
struct OP1Var4 {
    imm8: u8,
}
impl OP1Var4 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, false, self.imm8 as u64),
            <DisplayElement>::Literal(","),
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
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:45:1, end:45:4))"]
#[derive(Clone, Debug)]
struct OP1Var5 {}
impl OP1Var5 {
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
enum TableOP1 {
    Var0(OP1Var0),
    Var1(OP1Var1),
    Var2(OP1Var2),
    Var3(OP1Var3),
    Var4(OP1Var4),
    Var5(OP1Var5),
}
impl TableOP1 {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 112) == 32 {
            if let Some((inst_len, parsed)) =
                OP1Var0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 112) == 48 {
            if let Some((inst_len, parsed)) =
                OP1Var1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 112) == 64 {
            if let Some((inst_len, parsed)) =
                OP1Var2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 112) == 80 {
            if let Some((inst_len, parsed)) =
                OP1Var3::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var3(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 112) == 96 {
            if let Some((inst_len, parsed)) =
                OP1Var4::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var4(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 112) == 112 {
            if let Some((inst_len, parsed)) =
                OP1Var5::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var5(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:47:1, end:47:5))"]
#[derive(Clone, Debug)]
struct ADDRVar0 {
    imm8: u8,
}
impl ADDRVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.imm8 as u64)];
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:48:1, end:48:5))"]
#[derive(Clone, Debug)]
struct ADDRVar1 {
    imm16: u16,
}
impl ADDRVar1 {
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
        let mut block_0_len = 1;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        let mut block_1_len = 2;
        let imm16 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[derive(Clone, Debug)]
enum TableADDR {
    Var0(ADDRVar0),
    Var1(ADDRVar1),
}
impl TableADDR {
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
        if tokens_param.len() >= 2 && (tokens_param[0] & 112) == 48 {
            if let Some((inst_len, parsed)) =
                ADDRVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 3 && (tokens_param[0] & 112) == 64 {
            if let Some((inst_len, parsed)) =
                ADDRVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:49:1, end:49:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar0 {
    imm16: u16,
}
impl ADDRIVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, false, self.imm16 as u64),
            <DisplayElement>::Literal(","),
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
        let mut block_1_len = 2;
        let imm16 = token_6(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm16 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:50:1, end:50:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar1 {
    imm8: u8,
}
impl ADDRIVar1 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 3usize] = [
            DisplayElement::Number(true, false, self.imm8 as u64),
            <DisplayElement>::Literal(","),
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
        let imm8 = token_1(tokens_current);
        pattern_len += block_1_len;
        tokens_current = &tokens_current[usize::try_from(block_1_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self { imm8 }))
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:51:1, end:51:6))"]
#[derive(Clone, Debug)]
struct ADDRIVar2 {}
impl ADDRIVar2 {
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
enum TableADDRI {
    Var0(ADDRIVar0),
    Var1(ADDRIVar1),
    Var2(ADDRIVar2),
}
impl TableADDRI {
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
        if tokens_param.len() >= 3 && (tokens_param[0] & 112) == 80 {
            if let Some((inst_len, parsed)) =
                ADDRIVar0::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var0(parsed)));
            }
        }
        if tokens_param.len() >= 2 && (tokens_param[0] & 112) == 96 {
            if let Some((inst_len, parsed)) =
                ADDRIVar1::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var1(parsed)));
            }
        }
        if tokens_param.len() >= 1 && (tokens_param[0] & 112) == 112 {
            if let Some((inst_len, parsed)) =
                ADDRIVar2::parse(tokens_param, &mut context_current, inst_start)
            {
                *context_param = context_current;
                return Some((inst_len, Self::Var2(parsed)));
            }
        }
        None
    }
}
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/MC6800/data/languages/6805.slaspec, start:54:1, end:54:7))"]
#[derive(Clone, Debug)]
struct DIRECTVar0 {
    imm8: u8,
}
impl DIRECTVar0 {
    fn display_extend(
        &self,
        display: &mut Vec<DisplayElement>,
        context: &ContextMemory,
        inst_start: AddrType,
        inst_next: AddrType,
        global_set: &mut GlobalSet,
    ) {
        let extend: [DisplayElement; 1usize] =
            [DisplayElement::Number(true, false, self.imm8 as u64)];
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
enum TableDIRECT {
    Var0(DIRECTVar0),
}
impl TableDIRECT {
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
                DIRECTVar0::parse(tokens_param, &mut context_current, inst_start)
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
