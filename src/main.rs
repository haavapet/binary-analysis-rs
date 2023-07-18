mod prelude;
mod iter_instructions;
mod candidates;
mod file;
mod cli;
mod edges;
mod config;
mod endiannes;

use rayon::prelude::*;
use itertools::Itertools;

use prelude::*;
use iter_instructions::iter_potential_instruction_configuration;
use candidates::{call_candidates, ret_candidates};
use edges::{filter_valid_edges, find_potential_edges};

fn main() {
    // TODO iterate over both relative and absolute addressing?

    let config = Config::get();

    let binary = file::read_file(&config);

    println!("Starting analysis of file length: {}", binary.len());

    // Synchronous
    if !config.parallell {
        for (binary_slice, endiannes) in iter_potential_instruction_configuration(&binary, &config) {
            analyse_instructions(binary_slice, &config, endiannes)
        }
    } 
    // Parralell, speedup ~ min(num_cores, instr_byte_len), i.e given 32 bit instr and modern pc => 4x speedup
    else {
        iter_potential_instruction_configuration(&binary, &config).collect::<Vec<_>>().par_iter().for_each(|(binary_slice, endiannes)| {
            analyse_instructions(binary_slice, &config, endiannes)
        })
    }

}

fn analyse_instructions(binary_slice: &[u8], config: &Config, endiannes: &Endiannes) {

    // We assume call instruction is among call candidates, and ret instruction for ret_candidates
    let call_cand = call_candidates(binary_slice, config, endiannes);
    let ret_cand = ret_candidates(binary_slice, config, endiannes);
    
    for &(call_candidate, call_count) in call_cand.iter() {

        // Valid addresses for instructions of the given call candidates
        let potential_edges = find_potential_edges(binary_slice, call_candidate, config, endiannes);
        
        for &(ret_candidate, ret_count) in ret_cand.iter() {

            // valid addresses where there is a return preceding it
            let valid_edges = filter_valid_edges(binary_slice, ret_candidate, config, &potential_edges, endiannes);
            
            // Calculate probability stuff
            let ratio_valid: f64 = valid_edges.len() as f64 / call_count as f64;
            let ratio_potential = potential_edges.len() as f64 / call_count as f64;
            let probability = ((2.0 * ratio_valid) + ratio_potential) / 3.0;
            if probability > 0.5 {
                // Another thing to check is valid_edges.map(|from, to| to).unique() / ret_count
                // I.e amount of returns hit
                let ret_hits = valid_edges.iter().map(|(_, to)| to).unique().count();
                println!("FOUND HIGH PROBABILITY {}, {:#06x} {:#06x}, potential {}, valid {}, ret_hits {}", probability, call_candidate, ret_candidate, potential_edges.len(), valid_edges.len(), ret_hits as f64 / ret_count as f64);
            }
        }
    }
}


// AARCH64 CORRECT
// if call_candidate == 0x94000000 && ret_candidate == 0xD65F03C0 {
//     println!("FOUND IT, prob {}, len_potential {} len_valid{}", probability, potential_edges.len(), valid_edges.len());
//     println!("{:?}", &potential_edges[0..10]);
//     println!("{:?}", &valid_edges[0..10]);
//     //println!("{:#06x?}", &potential_instructions[32..37]);
// }

// MIPS CORRECT
// if call_candidate == 0x0c000000 && ret_candidate == 0x03e00008 {
//     println!("FOUND IT, prob {}, len_potential {} len_valid{}", probability, potential_edges.len(), valid_edges.len());
//     println!("{}", config.is_absolute_addressing);
// }