mod prelude;
mod extract_instructions;
mod file;
mod cli;
mod instructions;
mod config;
mod endiannes;
use std::io::Bytes;

use itertools::Itertools;

use extract_instructions::FromBytes;
use num_traits::ToPrimitive;
use prelude::*;
use crate::extract_instructions::iter_potential_instructions;


// /// warning: The resulting byte view is system-specific
// fn raw_byte_access<'a, T: PrimInt + 'a>(s8: &'a mut [u8]) -> &'a mut [impl PrimInt] {
//     // TODO correct length, not divide by 4, only for u32
//     unsafe { std::slice::from_raw_parts_mut(s8.as_mut_ptr() as *mut T, s8.len() / 4) }
// }

// let test = raw_byte_access::<T>(&mut binary);
// println!("{}", test.len());
// for e in &test[0 as usize..10 as usize] {
//     println!{"{}", parse::<T>(*e)};
// }
// pub fn parse<T: PrimInt>(instr: impl PrimInt) -> u64 {
//     // TODO need to handle both endiannes of instr, and endiannes of architecture, since fromrawparts doesn't
//     instr.to_le().to_u64().unwrap()
// }

fn main() {
    let config = Config::get();

    let binary = file::read_file(&config);

    println!("STARTING ITER INSTR {}", binary.len());


    for (byte_index, endiannes) in potential_instruction_configuration(&config) {
        let binary_slice = &binary[byte_index..];
        let instr_count = binary_slice.len() / (config.instr_len / BYTE_SIZE) as usize;
        // TODO remove this iter and instead pass binary to functions
        let instructions_iter = || parse_instructions(&binary[byte_index..], &endiannes, config.instr_len);
        let call_cand = instructions::call_candidates(instructions_iter(), &config);
        let ret_cand = instructions::ret_candidates(instructions_iter(), &config);
        
        for &(call_candidate, call_count) in call_cand.iter() {
            let potential_edges = instructions::find_potential_edges(instructions_iter(), call_candidate, &config, instr_count);
             for &(ret_candidate, ret_count) in ret_cand.iter() {
                // For relative and absolute addressing? for addressing_mode
                let valid_edges = instructions::filter_valid_edges(binary_slice, ret_candidate, &config, &potential_edges, instr_count);
                let ratio_valid: f64 = valid_edges.len() as f64 / call_count as f64;
                let ratio_potential = potential_edges.len() as f64 / call_count as f64;

                // Maybe also have paramter minimum_call_instruction_frequency and ret_frequency, default to i.e 100?
                let probability = ((2.0 * ratio_valid) + ratio_potential) / 3.0;
                if probability > 0.5 {
                    // Another thing to check is valid_edges.map(|from, to| to).unique() / ret_count
                    // I.e amount of returns hit
                    let ret_hits = valid_edges.iter().map(|(_, to)| to).unique().count();
                    println!("FOUND HIGH PROBABILITY {}, {:#06x} {:#06x}, potential {}, valid {}, ret_hits {}", probability, call_candidate, ret_candidate, potential_edges.len(), valid_edges.len(), ret_hits as f64 / ret_count as f64);
                }
                // if call_candidate == 0x94000000 && ret_candidate == 0xD65F03C0 {
                //     println!("FOUND IT, prob {}, len_potential {} len_valid{}", probability, potential_edges.len(), valid_edges.len());
                //     println!("{:?}", &potential_edges[0..10]);
                //     println!("{:?}", &valid_edges[0..10]);
                //     //println!("{:#06x?}", &potential_instructions[32..37]);
                // }
            }
        }
    }
}

pub fn parse_instructions<'a>(binary: &'a [u8], endiannes: &Endiannes, instr_len: u64) -> impl Iterator<Item = u64> + 'a {
    let extraction_function = match endiannes {
        Endiannes::Big => match instr_len {
            8 => from_be_bytes_8,
            16 => from_be_bytes_16,
            32 => from_be_bytes_32,
            64 => from_be_bytes_64,
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        Endiannes::Little => match instr_len {
            8 => from_le_bytes_8,
            16 => from_le_bytes_16,
            32 => from_le_bytes_32,
            64 => from_le_bytes_64,
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        Endiannes::Unknown => unimplemented!()
    };
    binary
    .chunks_exact((instr_len / 8) as usize)
    .map(extraction_function)
}

fn from_be_bytes_64(data: &[u8]) -> u64 {
    u64::from_be_bytes(data.try_into().unwrap())
}

fn from_be_bytes_32(data: &[u8]) -> u64 {
    u32::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_be_bytes_16(data: &[u8]) -> u64 {
    u16::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_be_bytes_8(data: &[u8]) -> u64 {
    u8::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_64(data: &[u8]) -> u64 {
    u64::from_le_bytes(data.try_into().unwrap())
}

fn from_le_bytes_32(data: &[u8]) -> u64 {
    u32::from_le_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_16(data: &[u8]) -> u64 {
    u16::from_le_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_8(data: &[u8]) -> u64 {
    u8::from_le_bytes(data.try_into().unwrap()) as u64
}



pub fn potential_instruction_configuration(config: &Config) -> impl Iterator<Item = (usize, Endiannes)> + '_ {
    let instr_byte_len = (config.instr_len / BYTE_SIZE) as usize;

    (0..instr_byte_len).filter_map(move |i| match i.cmp(&(instr_byte_len)) {
        std::cmp::Ordering::Less => match config.endiannes {
            Endiannes::Big | Endiannes::Little => Some((i, config.endiannes)),
            Endiannes::Unknown => todo!("Handle unknown endiannes")
        }
        _ => None
    })
}