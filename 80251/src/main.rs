#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_patterns)]
#[allow(dead_code)]
#[allow(unused_comparisons)]
mod disassembler;
use disassembler::*;

fn main() {
    let tokens = [];
    let address: AddrType = 0;
    let mut context = ContextMemory::default();
    let mut globalset = GlobalSet::new(context.clone());
    let (next_addr, output) =
        parse_instruction(&tokens, &mut context, address, &mut globalset).unwrap();
    println!("next_addr {}", next_addr);
    println!("{:?}", output);
    let str_output: String = output.into_iter().map(|value| value.to_string()).collect();
    println!("disassembled {}", str_output);
}
