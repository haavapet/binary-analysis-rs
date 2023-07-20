use crate::{file::read_file_len, prelude::*};

// This struct is added because the cli.rs struct is bloated. I.e the masks in this struct are a combination
// of fields from the other struct, which we do not need individually.
// Additionally we want to set some runtime default, such as default pc_inc = instr_len / 8
pub struct Config {
    // TODO masks etc need to be u64 probably, because for instR_len = 64, mask will be 64 bits
    pub file_path: PathBuf,
    pub instr_len: u64,
    pub call_opcode_mask: u64,
    pub ret_opcode_mask: u64,
    pub call_operand_mask: u64,
    pub call_operand_signed_mask: u64,
    pub endiannes: Endiannes,
    pub file_offset: [usize; 2],
    pub pc_offset: u64,
    pub pc_inc: u64,
    pub left_shift_call_operand: u64,
    pub nr_cand: usize,
    pub call_search_range: [usize; 2],
    pub ret_search_range: [usize; 2],
    pub ret_func_dist: usize,
    pub parallell: bool,
    pub include_instructions: bool,
    pub is_absolute_addressing: bool,
}

impl Config {
    pub fn get() -> Config {
        // Deconstruct so that if adding more fields we get an error
        let crate::cli::Parameters {
            file_path,
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
            parallell,
            include_instructions,
            is_absolute_addressing,
        } = crate::cli::parse_parameters();

        // TODO: Handle parsing of parameters, i.e file_offset cannot be greater than file length,
        // instr_len only 8,16,32,64. indexes must be valid etc etc.

        if call_opcode_len.is_none() {
            println!(
                "USE {:?} {:?} {:?}",
                call_opcode_index, call_operand_index, ret_opcode_index
            );
            unimplemented!("Need to handle bitmasks when the additional parameters are provided");
        };
        let col = call_opcode_len.unwrap();
        let temp = u64::MAX; //, need to handle overflow, do u64::max if 64 instr_len?
        let call_opcode_mask: u64 = (temp) ^ ((1 << (instr_len - col)) - 1);
        let call_operand_mask: u64 = (1 << (instr_len - col)) - 1;
        let ret_opcode_mask: u64 = temp;
        let call_operand_signed_mask: u64 = (1 << (instr_len - col - 1)) - 1;

        let file_offset: [usize; 2] = if let Some(value) = file_offset {
            value.try_into().unwrap()
        } else {
            [0, read_file_len(&file_path)]
        };

        Config {
            file_path,
            instr_len,
            call_opcode_mask,
            ret_opcode_mask,
            call_operand_mask,
            call_operand_signed_mask,
            endiannes,
            file_offset,
            pc_offset,
            pc_inc: if let Some(value) = pc_inc {
                value
            } else {
                instr_len / BYTE_SIZE
            },
            left_shift_call_operand,
            nr_cand,
            call_search_range: call_search_range.try_into().unwrap(),
            ret_search_range: ret_search_range.try_into().unwrap(),
            ret_func_dist,
            parallell,
            include_instructions,
            is_absolute_addressing,
        }
    }
}
