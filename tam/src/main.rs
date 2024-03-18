mod errors;
mod instruction;
mod machine;

use byteorder::{ReadBytesExt, BE};
use clap::Parser;
use instruction::Instruction;
use machine::TAM;
use std::fs::File;

#[derive(Parser, Debug)]
struct Args {
    /// Bytecode file to load and run
    bytecode: String,

    /// Print the disassembly of the given code instead of running it
    #[arg(short, long)]
    disassemble: bool,

    /// Print each instruction before executing them
    #[arg(short, long)]
    trace: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.disassemble {
        return disassemble(&args.bytecode);
    }

    let mut tam = TAM::new(args.trace);
    tam.load_program(&args.bytecode)?;
    if let Err(e) = tam.run() {
        println!("{}", e);
    }
    Ok(())
}

fn disassemble(filename: &str) -> std::io::Result<()> {
    let mut f = File::open(filename)?;
    let mut addr = 0;

    while let Ok(inst) = f.read_u32::<BE>() {
        let inst1 = Instruction::from(inst);
        println!("{addr:04x}: {inst1}");
        addr += 1;
    }
    Ok(())
}
