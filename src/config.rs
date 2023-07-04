use crate::{prelude::*, file};

// This struct is added because the cli.rs struct is bloated. I.e the masks in this struct are a combination
// of fields from the other struct, which we do not need individually.
// Additionally we want to set some runtime default, such as default pc_inc = instr_len / 8
pub struct Config {
    pub file_path: PathBuf,
    pub instr_len: u32,
    pub call_opcode_mask: u32,
    pub call_operand_mask: u32,
    pub ret_opcode_mask: u32,
    pub endiannes: Endiannes,
    pub file_offset: [u32; 2],
    pub pc_offset: u32,
    pub pc_inc: u32,
    pub left_shift_call_operand: u32,
    pub nr_cand: u32,
    pub call_search_range: [u32; 2],
    pub ret_search_range: [u32; 2],
    pub ret_func_dist: u32,
    pub unknown_code_entry: bool,
    pub include_instructions: bool,
    pub is_absolute_addressing: bool,
}

impl Config {
    pub fn get() -> Config {
        // Deconstruct so that if adding more fields we get an error
        let crate::cli::Parameters {file_path, 
                                    call_opcode_len, 
                                    instr_len, 
                                    endiannes, 
                                    ret_opcode_index, 
                                    call_opcode_index,
                                    call_operand_index, 
                                    file_offset, 
                                    pc_offset, 
                                    pc_inc, 
                                    left_shift_call_operand, 
                                    nr_cand, 
                                    call_search_range, 
                                    ret_search_range, 
                                    ret_func_dist, 
                                    unknown_code_entry, 
                                    include_instructions, 
                                    is_absolute_addressing} = crate::cli::parse_parameters();

        if call_opcode_len.is_none() {
            unimplemented!("Need to handle bitmasks when the additional parameters are provided");
        };
        let col = call_opcode_len.unwrap();
        let call_opcode_mask: u32 = ((1 << instr_len) - 1) ^ ((1 << (instr_len - col)) - 1);
        let call_operand_mask: u32 = (1 << (instr_len - col)) - 1;
        let ret_opcode_mask: u32 = (1 << instr_len) - 1;

        let file_offset: [u32; 2] = if let Some(value) = file_offset {
            value.try_into().unwrap()
        } else if unknown_code_entry { 
            unimplemented!();
        } else {
            [0, std::fs::metadata(&file_path).expect("file does not exist").len() as u32]
        };

        Config { 
            file_path: file_path, 
            instr_len: instr_len, 
            call_opcode_mask: call_opcode_mask, 
            call_operand_mask: call_operand_mask, 
            ret_opcode_mask: ret_opcode_mask, 
            endiannes: endiannes, 
            file_offset: file_offset, 
            pc_offset: pc_offset, 
            pc_inc: if let Some(value) = pc_inc {value} else {instr_len / BYTE_SIZE}, 
            left_shift_call_operand: left_shift_call_operand, 
            nr_cand, 
            call_search_range: call_search_range.try_into().unwrap(), 
            ret_search_range: ret_search_range.try_into().unwrap(), 
            ret_func_dist: ret_func_dist, 
            unknown_code_entry: unknown_code_entry, 
            include_instructions: include_instructions, 
            is_absolute_addressing: is_absolute_addressing 
        }
    }
}