use std::fs::File;
use std::io::Write;

use sleigh2rust;

pub fn transpile(slaspec: &str, rust_code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ghidra_file = format!("{}/{}", std::env::var("GHIDRA_SRC")?, slaspec,);
    let disassembler = sleigh2rust::generate_disassembler(ghidra_file)?;

    let mut file = File::create(rust_code)?;
    file.write_all(disassembler.to_string().as_bytes())?;
    drop(file);

    std::process::Command::new("rustfmt")
        .arg(rust_code)
        .status()
        .unwrap()
        .success()
        .then_some(())
        .ok_or(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unable to `rustfmt` output",
        )))
}
