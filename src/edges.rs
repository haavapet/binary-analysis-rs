use crate::prelude::*;
use std::ops::Neg;

use crate::iter_instructions::iter_instructions;

// TODO have call edges struct
pub fn find_potential_edges(binary: &[u8], call_candidate: u64, config: &Config, endiannes: &Endiannes) -> Vec<(usize, usize)> {
    // Destructure CLI params we need
    let &Config { call_opcode_mask,
                 call_operand_mask,
                 call_operand_signed_mask,
                 pc_inc,
                 pc_offset,
                 left_shift_call_operand,
                 is_absolute_addressing,
                 .. } = config;
                
    let pc_inc = pc_inc as i64;
    let call_operand_signed_mask = call_operand_signed_mask as i64;
    if is_absolute_addressing {
        let mut potential_edges: Vec<(usize, usize)> = Vec::new();

        for (i, instr) in iter_instructions(binary, endiannes, config.instr_len).enumerate() {
            if instr & call_opcode_mask == call_candidate {
                let call_operand = instr & call_operand_mask;
                let address = (((call_operand as i64) << left_shift_call_operand) - pc_offset as i64) / pc_inc;
                if 0 <= address && (address as usize) < (binary.len() / (config.instr_len / BYTE_SIZE) as usize){
                    potential_edges.push((i, address as usize));
                }
            }
        }

        potential_edges
    } else {
        let itosi = |x: u64, mask: i64| {
            let mut signed: i64 = x as i64;
            if signed > mask {
                signed |= mask.neg();
            }
            signed
        };

        let mut potential_edges: Vec<(usize, usize)> = Vec::new();

        for (i, instr) in iter_instructions(binary, endiannes, config.instr_len).enumerate() {
            if instr & call_opcode_mask == call_candidate {
                let call_operand = instr & call_operand_mask;
                let signed_operand = itosi(call_operand, call_operand_signed_mask);
                // Maybe i.checked_add(rest) ?? Because we know the result should be unsigned
                let address = ((signed_operand << left_shift_call_operand) / pc_inc) + i as i64;
                if 0 <= address && (address as usize) < (binary.len() / (config.instr_len / BYTE_SIZE) as usize)
                    && (i as i64 - address).abs() > 4 {
                        potential_edges.push((i, address as usize));
                }
            }
        }
        potential_edges
    }
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
        if to_edge < ret_func_dist + 1 {
            valid_call_edges.push((from_edge, to_edge));
            continue;
        }

        let adjusted_to_edge = to_edge * (config.instr_len / BYTE_SIZE) as usize;
        for instr in iter_instructions(&binary[adjusted_to_edge - distance..adjusted_to_edge], endiannes, config.instr_len) {
            if instr & ret_opcode_mask == ret_opcode {
                valid_call_edges.push((from_edge, to_edge));
                break;
            }
        }
    }
    valid_call_edges
}