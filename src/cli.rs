use crate::prelude::*;
use clap::{Parser};
use clap_num::maybe_hex;

#[derive(Parser)]
#[command(author, 
          version, 
          about, 
          override_usage="binary-analysis-rs <FILE_PATH> -i <INSTR_LEN> -o <OPCODE_LEN>",
          after_help="TODO How to use this program"
         )]
pub struct Parameters {
    // TODO GIVE all values more explicit names, and just deconstruct them in function prologues instead
    // REQUIRED
    #[arg()]
    pub file_path: PathBuf,
    
    #[arg(short = 'i', long, required=true, value_name="int", help="Instruction Length")]
    pub instr_len: u32,

    #[arg(short = 'c', required_unless_present_all=["ret_opcode_index", "call_opcode_index", "call_operand_index"])]
    pub call_opcode_len: Option<u32>,

    // OPTIONAL
    #[arg(short = 'e', long, default_value="unknown", value_enum)]
    pub endiannes: Endiannes,
    
    // If one needs to be more explicit about which bits are part of return opcode
    #[arg(long, number_of_values=2, required=false, conflicts_with="call_opcode_len")]
    pub ret_opcode_index: Option<Vec<u32>>,

    // If one needs to be more explicit about which bits are part of call opcode
    #[arg(long, number_of_values=2, required=false, conflicts_with="call_opcode_len")]
    pub call_opcode_index: Option<Vec<u32>>,

    // If one needs to be more explicit about which bits are part of call operand
    #[arg(long, number_of_values=2, required=false, conflicts_with="call_opcode_len")]
    pub call_operand_index: Option<Vec<u32>>,

    // start, end offset of .text segment of binary file
    #[arg(long, number_of_values=2, required=false, value_parser=maybe_hex::<u32>, conflicts_with="unknown_code_entry")]
    pub file_offset: Option<Vec<u32>>,

    // position of first instruction in virtual memory, needed for absolute addressing
    #[arg(long, default_value="0x4000000", value_parser=maybe_hex::<u32>)]
    pub pc_offset: u32,

    // Relative adress difference between consecutive instructions, defaults to instr_len / 8
    // TODO set default to instr_len / 8
    #[arg(long, required=false)]
    pub pc_inc: Option<u32>,

    // I.e MIPS and Aarch64 left shifts by 2 since otherwise the last two bits of addressing are unused
    #[arg(long, default_value="0")]
    pub left_shift_call_operand: u32,

    // nr of candidate opcodes to return
    #[arg(long, default_value="3")]
    pub nr_cand: u32,

    // (x, y) -> call is known to be between the x and y most popular instructions
    #[arg(long, number_of_values=2, default_value="0 20", value_delimiter = ' ')]
    pub call_search_range: Vec<u32>,

    // (x, y) -> ret is known to be between the x and y most popular instructions
    #[arg(long, number_of_values=2, default_value="0 15", value_delimiter = ' ')]
    pub ret_search_range: Vec<u32>,

    // Distance from function prologue to prvious ret
    #[arg(long, default_value="5")]
    pub ret_func_dist: u32,
    
    #[arg(long, default_value="false", conflicts_with="file_offset")] // num_args=0??
    pub unknown_code_entry: bool,

    #[arg(long, default_value="false")]
    pub include_instructions: bool,

    #[arg(long, default_value="false")]
    pub is_absolute_addressing: bool,
}

pub fn parse_parameters() -> Parameters {
    Parameters::parse()
}