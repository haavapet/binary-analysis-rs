use crate::prelude::*;
use std::ops::Neg;

use crate::iter_instructions::{iter_instructions, iter_instructions_and_search};

// TODO have call edges struct
pub fn find_potential_edges(binary: &[u8], call_candidate: u64, config: &Config, endiannes: &Endiannes) -> Vec<(usize, usize)> {
    // Destructure CLI params we need
    let &Config { call_opcode_mask,
                 call_operand_mask,
                 call_operand_signed_mask,
                 pc_offset,
                 left_shift_call_operand,
                 is_absolute_addressing,
                 .. } = config;
                
    let call_operand_signed_mask = call_operand_signed_mask as i64;
    let mut potential_edges = Vec::new();

    if is_absolute_addressing {
        for (i, instr) in iter_instructions(binary, endiannes, config.instr_len).enumerate() {
            if instr & call_opcode_mask == call_candidate {
                let call_operand = instr & call_operand_mask;
                let address = ((call_operand as i64) << left_shift_call_operand) - pc_offset as i64;
                if binary.get(address as usize).is_some() {
                    potential_edges.push((i * (config.instr_len / BYTE_SIZE) as usize, address as usize));
                }
            }
        }
    } else {
        let mut potential_edges: Vec<(usize, usize)> = Vec::new();

        for (i, instr) in iter_instructions(binary, endiannes, config.instr_len).enumerate() {
            if instr & call_opcode_mask == call_candidate {
                let call_operand = (instr & call_operand_mask) as i64;
                let signed_operand = {
                    if call_operand > call_operand_signed_mask {
                        call_operand | call_operand_signed_mask.neg()
                    } else {
                        call_operand
                    }
                };
                // Maybe i.checked_add(rest) ?? Because we know the result should be unsigned

                // TODO, maybe pc_inc is really uneccesarry? it's really always instr_len / byte size, in all archs
                // TODO change all usizes to u64 because usize doesn't work on 64bit

                // Assume that all ISA left-shifts by byte length of instruction, thus we do not need to check 
                // that address % instr_byte_len == 0, since it would be valid for all
                let address = (signed_operand << left_shift_call_operand) + (i as u64 * (config.instr_len / BYTE_SIZE)) as i64;
                if binary.get(address as usize).is_some() 
                    && (i as i64 - address).abs() > 4 * (config.instr_len / BYTE_SIZE) as i64 
                {
                    potential_edges.push((i * (config.instr_len / BYTE_SIZE) as usize, address as usize));
                }
            }
        }
    }
    
    // Performance improvement:
    // sorted makes memory access in for loop in filter_valid_edges much more efficient, reducing runtime by ̃~15%
    // This also seems to be more efficient than using a binary heap, most likely because most lists are small
    potential_edges.sort_unstable_by_key(|&(_, to)| to);
    potential_edges
}

pub fn filter_valid_edges(binary: &[u8], ret_opcode: u64, config: &Config, potential_call_edges: &Vec<(usize, usize)>, endiannes: &Endiannes) -> Vec<(usize, usize)> {
    let &Config { ret_func_dist,
                 ret_opcode_mask,
                 .. } = config;

    let mut valid_call_edges: Vec<(usize, usize)> = Vec::new();
    let distance = (ret_func_dist) * (config.instr_len / BYTE_SIZE) as usize;

    for &(from_edge, to_edge) in potential_call_edges {
        // The first function in the program has no preceding return statement, thus we mark it valid either way.
        // This also works nicely because we know the below for loop has valid indexes because of this check
        // Ideally we should find the index of the first return instruction for the given return candidate, and mark all calls before that as valid
        
        // if (is first function) || (function has preceding return)
        if to_edge <= distance || iter_instructions_and_search(&binary[to_edge - distance..to_edge], endiannes, config.instr_len, ret_opcode_mask, ret_opcode) {
            valid_call_edges.push((from_edge, to_edge));
        }
    }
    valid_call_edges
}