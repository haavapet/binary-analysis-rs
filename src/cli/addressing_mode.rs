use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum)]
pub enum AddressingMode {
    // Register? Not sure if it is feasible to handle that
    Absolute,
    Relative,
    Unknown,
}
