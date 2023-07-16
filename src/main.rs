mod prelude;
mod iter_instructions;
mod file;
mod cli;
mod instructions;
mod config;
mod endiannes;

use itertools::Itertools;
use iter_instructions::iter_potential_instruction_configuration;

use prelude::*;

fn main() {
    let config = Config::get();

    let binary = file::read_file(&config);

    println!("STARTING ITER INSTR {}", binary.len());


    for (binary_slice, endiannes) in iter_potential_instruction_configuration(&binary, &config) {

        let call_cand = instructions::call_candidates(binary_slice, &config, endiannes);
        let ret_cand = instructions::ret_candidates(binary_slice, &config, endiannes);
        
        for &(call_candidate, call_count) in call_cand.iter() {
            let potential_edges = instructions::find_potential_edges(binary_slice, call_candidate, &config, endiannes);
            // if potential_edges.len() < call_count / 3 {
            //     continue                
            // }
            
            for &(ret_candidate, ret_count) in ret_cand.iter() {
                // For relative and absolute addressing? for addressing_mode
                let valid_edges = instructions::filter_valid_edges(binary_slice, ret_candidate, &config, &potential_edges, endiannes);
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