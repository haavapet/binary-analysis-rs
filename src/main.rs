mod candidates_opcodes;
mod cli;
mod edges;
mod file;
mod iter_instructions;
mod min_heap;
mod prelude;

use rayon::prelude::*;

use candidates_opcodes::{call_candidates, ret_candidates};
use edges::{filter_valid_edges, find_potential_edges};
use iter_instructions::iter_potential_instruction_configuration;
use min_heap::{Candidate, MinHeap};
use prelude::*;

fn main() {
    let config = Config::get();

    let binary = file::read_file(&config);

    let mut top_candidates: MinHeap = Default::default();

    // Synchronous
    if !config.parallell {
        for (binary_slice, endiannes, addressing_mode) in
            iter_potential_instruction_configuration(&binary, &config)
        {
            analyse_instructions(
                binary_slice,
                &config,
                endiannes,
                addressing_mode,
                &top_candidates,
            )
        }
    }
    // Parralell, speedup ~ min(num_cores, instr_byte_len), i.e given 32 bit instr and modern pc => 4x speedup
    else {
        iter_potential_instruction_configuration(&binary, &config)
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|(binary_slice, endiannes, addressing_mode)| {
                analyse_instructions(
                    binary_slice,
                    &config,
                    endiannes,
                    addressing_mode,
                    &top_candidates,
                )
            });
    }

    // We now have the top candidates, we can create a call graph, print the top candidates etc etc.
    println!("RESULTS:");
    for candidate in top_candidates.get_result() {
        println!(
            "Prob: {:.4} \tCall: {:#08x}\tRet: {:#08x}",
            candidate.probability, candidate.call_opcode, candidate.ret_opcode
        )
    }
}

// TOdo move this to "analyse_binary.rs"
fn analyse_instructions(
    binary_slice: &[u8],
    config: &Config,
    endiannes: &Endiannes,
    addressing_mode: &AddressingMode,
    top_candidates: &MinHeap,
) {
    // We assume call instruction is among call candidates, and ret instruction for ret_candidates
    let call_cand = call_candidates(binary_slice, config, endiannes);
    let ret_cand = ret_candidates(binary_slice, config, endiannes);

    for &(call_candidate, call_count) in call_cand.iter() {
        // Valid addresses for instructions of the given call candidates
        let potential_edges = find_potential_edges(
            binary_slice,
            call_candidate,
            config,
            endiannes,
            addressing_mode,
        );

        for &(ret_candidate, _ret_count) in ret_cand.iter() {
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
            //  use itertools::Itertools;
            //  let ret_hits = valid_edges.iter().map(|(_, to)| to).unique().count();

            // Add to heap if high probability
            top_candidates.add_maybe(
                config.nr_cand,
                Candidate {
                    probability,
                    call_opcode: call_candidate,
                    ret_opcode: ret_candidate,
                },
            )
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
