use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Endiannes {
    Little,
    Big
}

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct Parameters {
    #[arg()]
    pub file_path: String,

    #[arg(short = 'i', long="instr_len")]
    pub instr_len: u32,

    #[arg(short = 'e', value_enum)]
    pub endiannes: Endiannes,

    // Additional parameter left-shift operand of call
}

pub fn parse_parameters() -> Parameters {
    Parameters::parse()
}