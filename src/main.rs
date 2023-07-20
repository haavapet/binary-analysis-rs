mod candidates;
mod cli;
mod config;
mod edges;
mod endiannes;
mod file;
mod iter_instructions;
mod prelude;

use ordered_float::NotNan;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::{cmp::Reverse, collections::BinaryHeap};

use candidates::{call_candidates, ret_candidates};
use edges::{filter_valid_edges, find_potential_edges};
use iter_instructions::iter_potential_instruction_configuration;
use prelude::*;

fn main() {
    // TODO iterate over both relative and absolute addressing?

    let config = Config::get();

    let binary = file::read_file(&config);

    println!("Starting analysis of file length: {}", binary.len());

    // TODO this type should be abstracted away, and not use arc mutex stuff when not parallell
    let top_candidates: Arc<Mutex<BinaryHeap<Reverse<NotNan<f64>>>>> =
        Arc::new(Mutex::new(Default::default()));

    // Synchronous
    if !config.parallell {
        for (binary_slice, endiannes) in iter_potential_instruction_configuration(&binary, &config)
        {
            analyse_instructions(binary_slice, &config, endiannes, &top_candidates)
        }
    }
    // Parralell, speedup ~ min(num_cores, instr_byte_len), i.e given 32 bit instr and modern pc => 4x speedup
    else {
        iter_potential_instruction_configuration(&binary, &config)
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|(binary_slice, endiannes)| {
                analyse_instructions(binary_slice, &config, endiannes, &top_candidates)
            });
    }

    // We now have the top candidates, we can create a call graph, print the top candidates etc etc.
    println!(
        "{:?}",
        top_candidates.lock().unwrap().clone().into_sorted_vec()
    );
}

// TOdo move this to "analyse_binary.rs"
fn analyse_instructions(
    binary_slice: &[u8],
    config: &Config,
    endiannes: &Endiannes,
    top_candidates: &Arc<Mutex<BinaryHeap<Reverse<NotNan<f64>>>>>,
) {
    // We assume call instruction is among call candidates, and ret instruction for ret_candidates
    let call_cand = call_candidates(binary_slice, config, endiannes);
    let ret_cand = ret_candidates(binary_slice, config, endiannes);

    for &(call_candidate, call_count) in call_cand.iter() {
        // Valid addresses for instructions of the given call candidates
        let potential_edges = find_potential_edges(binary_slice, call_candidate, config, endiannes);

        for &(ret_candidate, ret_count) in ret_cand.iter() {
            // valid addresses where there is a return preceding it
            let valid_edges = filter_valid_edges(
                binary_slice,
                ret_candidate,
                config,
                &potential_edges,
                endiannes,
            );

            // Calculate probability stuff
            let ratio_valid: f64 = valid_edges.len() as f64 / call_count as f64;
            let ratio_potential = potential_edges.len() as f64 / call_count as f64;
            let probability = ((2.0 * ratio_valid) + ratio_potential) / 3.0;

            // TODO abstract this away and only have a 'top_candidates.maybe_add(prob)'
            {
                let mut heap = top_candidates.lock().unwrap();

                heap.push(Reverse(NotNan::new(probability).unwrap()));

                if heap.len() > config.nr_cand {
                    heap.pop();
                }
            }

            // if probability > 0.5 {
            //     // Another thing to check is valid_edges.map(|from, to| to).unique() / ret_count
            //     // I.e amount of returns hit
            //     use itertools::Itertools;
            //     let ret_hits = valid_edges.iter().map(|(_, to)| to).unique().count();
            //     println!("FOUND HIGH PROBABILITY {}, {:#06x} {:#06x}, potential {}, valid {}, ret_hits {}", probability, call_candidate, ret_candidate, potential_edges.len(), valid_edges.len(), ret_hits as f64 / ret_count as f64);
            // }
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
