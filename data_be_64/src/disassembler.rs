pub type AddrType = u64;
#[derive(Clone, Copy, Debug)]
pub enum Register {
    sp,
    r0,
    contextreg,
}
impl Register {
    fn as_str(&self) -> &'static str {
        match self {
            Self::sp => "sp",
            Self::r0 => "r0",
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
#[derive(Clone, Copy, Default)]
pub struct ContextMemory(pub u8);
impl ContextMemory {
    pub fn read_test(&self) -> u8 {
        (((self.0.reverse_bits() >> 7) & 1) as u8)
    }
    pub fn write_test(&mut self, value: u8) {
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
#[doc = "Constructor at Location(FileSpan(/home/rbran/src/ghidra/Ghidra/Processors/DATA/data/languages/data.sinc, start:26:1, end:26:2))"]
#[derive(Clone, Debug)]
struct nop_instructionVar0 {}
impl nop_instructionVar0 {
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
        let mut block_0_len = 0;
        pattern_len += block_0_len;
        tokens_current = &tokens_current[usize::try_from(block_0_len).unwrap()..];
        *context = context_instance;
        Some((pattern_len, Self {}))
    }
}
#[derive(Clone, Debug)]
enum Tableinstruction {
    Var0(nop_instructionVar0),
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
        }
    }
    fn parse(
        tokens_param: &[u8],
        context_param: &mut ContextMemory,
        inst_start: AddrType,
    ) -> Option<(AddrType, Self)> {
        let mut context_current = context_param.clone();
        if tokens_param.len() >= 0 && context_param.0 & 1 == 1 {
            if let Some((inst_len, parsed)) =
                nop_instructionVar0::parse(tokens_param, &mut context_current, inst_start)
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
) -> Option<(u64, Vec<DisplayElement>)> {
    let (inst_len, instruction) = Tableinstruction::parse(tokens, context, inst_start)?;
    let inst_next = inst_start + inst_len;
    let mut display = vec![];
    instruction.display_extend(&mut display, context, inst_start, inst_next, global_set);
    Some((inst_next, display))
}
