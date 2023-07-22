use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum)]
pub enum Endiannes {
    Little,
    Big,
    Unknown,
}
