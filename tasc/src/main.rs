use byteorder::{WriteBytesExt, BE};
use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::{fs, fs::OpenOptions};

lalrpop_mod!(pub tasm);

pub type InstrData = (u8, u8, u8, i16);

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
    let instrs = parser.parse(&input).unwrap();

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&args.outfile)?;

    for (op, r, n, d) in instrs {
        let data =
            ((op as u32) << 28) | ((r as u32) << 24) | ((n as u32) << 16) | ((d as u32) & 0xffff);
        f.write_u32::<BE>(data)?;
    }

    Ok(())
}
