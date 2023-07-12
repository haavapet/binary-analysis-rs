mod prelude;
mod extract_instructions;
mod file;
mod cli;
mod instructions;
mod config;
mod endiannes;

use prelude::*;
use crate::extract_instructions::iter_potential_instructions;

fn main() {
    let config = Config::get();

    let binary = file::read_file(&config);

    println!("STARTING ITER INSTR {}", binary.len());
    
    for potential_instructions in iter_potential_instructions(&binary, &config){
        let (call_cand, ret_cand) = instructions::get_candidates(&potential_instructions, &config);

        for &(call_candidate, call_count) in call_cand.iter() {
            let potential_edges = instructions::find_potential_edges(&potential_instructions, call_candidate, &config);
            for &(ret_candidate, _) in ret_cand.iter() {
                // For relative and absolute addressing? for addressing_mode
                
                let valid_edges = instructions::filter_valid_edges(&potential_instructions, ret_candidate, &config, &potential_edges);
                let ratio_valid: f64 = valid_edges.len() as f64 / call_count as f64;
                let ratio_potential = potential_edges.len() as f64 / call_count as f64;
                // Another thing to check is valid_edges.map(|from, to| to).unique() / ret_count
                // I.e amount of returns hit
                // Maybe also have paramter minimum_call_instruction_frequency and ret_frequency, default to i.e 100?
                let probability = ((2.0 * ratio_valid) + ratio_potential) / 3.0;
                
                if probability > 0.5 {
                    println!("FOUND HIGH PROBABILITY {}, {:#06x} {:#06x}, potential {}, valid {}", probability, call_candidate, ret_candidate, potential_edges.len(), valid_edges.len());
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
