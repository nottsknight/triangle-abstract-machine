mod codegen;

use std::{fs, fs::OpenOptions};

use byteorder::{WriteBytesExt, BE};
use clap::Parser;
use common::instruction::Instruction;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub tasm);

#[derive(Clone)]
pub struct InstrData {
    label: Option<String>,
    data: Instruction,
    named_dest: Option<String>,
}

impl InstrData {
    pub fn new(op: u8, r: u8, n: u8, d: i16) -> InstrData {
        InstrData {
            label: None,
            data: Instruction { op, r, n, d },
            named_dest: None,
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// Assembly file to compile
    infile: String,

    ///Name of bytecode file to create
    #[arg(short, default_value_t = String::from("a.out"))]
    outfile: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input = fs::read_to_string(args.infile)?;

    let parser = tasm::ProgramParser::new();
    let data = parser.parse(&input).unwrap();
    let code = codegen::gen_code(data);

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&args.outfile)?;

    for instr in code {
        f.write_u32::<BE>(u32::from(instr))?;
    }

    Ok(())
}
