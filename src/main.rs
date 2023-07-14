mod prelude;
mod extract_instructions;
mod file;
mod cli;
mod instructions;
mod config;
mod endiannes;
use itertools::Itertools;

use extract_instructions::FromBytes;
use prelude::*;
use crate::extract_instructions::iter_potential_instructions;


fn main() {
    let config = Config::get();

    // Call into function that is generic over the instruction length provided
    match config.instr_len {
        8 => main_generic_over_instruction_length::<u8>(&config),
        16 => main_generic_over_instruction_length::<u16>(&config),
        32 => main_generic_over_instruction_length::<u32>(&config),
        64 => main_generic_over_instruction_length::<u64>(&config),
        _ => panic!("Analysis not implemented for instructions of length {}", config.instr_len)
    };
}

pub fn main_generic_over_instruction_length<T: PrimInt + FromBytes>(config: &Config) 
        where <T as FromBytes>::Output: PrimInt {

    let binary = file::read_file(&config);

    println!("STARTING ITER INSTR {}", binary.len());


    for potential_instructions in iter_potential_instructions::<T>(&binary, &config){
        // One possibility, move all this code into a function, and match here on 
        let (call_cand, ret_cand) = instructions::get_candidates(&potential_instructions, &config);
        
        for &(call_candidate, call_count) in call_cand.iter() {
            let potential_edges = instructions::find_potential_edges(&potential_instructions, call_candidate, &config);
             for &(ret_candidate, ret_count) in ret_cand.iter() {
                // For relative and absolute addressing? for addressing_mode
                let valid_edges = instructions::filter_valid_edges(&potential_instructions, ret_candidate, &config, &potential_edges);
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
                // if call_candidate == 0x94000000 && ret_candidate.clone() == 0xD65F03C0 {
                //     println!("FOUND IT, prob {}, len_potential {} len_valid{}", probability, potential_edges.len(), valid_edges.len());
                //     // println!("{:?}", &potential_edges[0..10]);
                //     // println!("{:?}", &valid_edges[0..10]);
                //     println!("{:#06x?}", &potential_instructions[32..37]);
                // }
            }
        }
    }
}

