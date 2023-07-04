use crate::prelude::*;
use std::collections::BTreeMap;
use itertools::Itertools;

pub fn call_candidates(instructions: &Vec<u64>, config: &Config) -> Vec<(u64, usize)> {
    let mut counts = BTreeMap::new();
    for instr in instructions {
        counts.entry(instr & config.call_opcode_mask as u64).and_modify(|e| *e += 1).or_insert(1);
    }

    // replace (0, 5) with (call_search_range[0], call_search_range[1])
    counts.into_iter().sorted_by_key(|&(_, count)| count).rev().skip(0).take(5).collect()
}

fn ret_candidates(instructions: &Vec<u64>, config: &Config) -> Vec<u64> {
    vec![0]
}