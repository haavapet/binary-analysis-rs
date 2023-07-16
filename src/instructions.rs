use crate::prelude::*;
use std::ops::Neg;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::iter_instructions::iter_instructions;

pub fn call_candidates(binary: &[u8], config: &Config, endiannes: &Endiannes) -> Vec<(u64, usize)> {
    // Destructure CLI params we need
    let &Config { call_opcode_mask, 
                 call_search_range,
                 .. } = config;

    // OPTION 1, simple solution with FxHasher
    let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
    for instr in iter_instructions(binary, endiannes, config.instr_len) {
        counts.entry(instr & call_opcode_mask).and_modify(|e| *e += 1).or_insert(1);
    }

    counts
        .into_iter()
        .filter(|(c, _)| c > &1)
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(call_search_range[0])
        .take(call_search_range[1])
        .collect()

    // OPTION 2: BtreeMap instead of HashMap, slower by ~40% when testing on curl_aarch64 binary

    // OPTION 3: Slightly faster than BtreeMap, however noticably slower than HashMap implementation
    // instructions
    //     .iter()
    //     .map(|x| x & call_opcode_mask)
    //     .sorted()
    //     .group_by(|&x| x)
    //     .into_iter()
    //     .map(|(key, group)| (key, group.count()))
    //     .sorted_by_key(|&(_, count)| count)
    //     .rev()
    //     .skip(call_search_range[0] as usize)
    //     .take(call_search_range[1] as usize)
    //     .collect()

    // Option 4: Using counter crate. About as efficient as best, but need another crate
    // use counter::Counter;
    // instructions
    //     .iter()
    //     .map(|instr| instr & ret_opcode_mask)
    //     .collect::<Counter<_>>()
    //     .k_most_common_ordered(ret_search_range[1])

    // Option 6: Use internal representation of counter crate
    // This works, keep a binaryheap of fixed size, probably most efficient, but not by a lot
    // from https://docs.rs/counter/latest/src/counter/lib.rs.html#1-1733
    // let mut items = counts.iter().map(|(&k, &c)| (std::cmp::Reverse(c), k));
    // let mut heap: BinaryHeap<_> = items.by_ref().take(ret_search_range[1]).collect();
    // items.for_each(|item| {
    //     let mut root = heap.peek_mut().expect("the heap is empty");
    //     if (*root).0 > item.0 {
    //         *root = item;
    //     }
    // });
    // heap.iter().map(|(std::cmp::Reverse(c), k)| (c.clone(), k.clone())).collect()
}

pub fn ret_candidates(binary: &[u8], config: &Config, endiannes: &Endiannes) -> Vec<(u64, usize)> {
    // Destructure CLI params we need
    let &Config { ret_opcode_mask, 
                 ret_search_range,
                 .. } = config;

    
    // Super nice optimization, look at a subset if big file, find unique instructions in the subset,
    // Only put the ones found here in the count after, skip rest
    // Assumes that a ret instruction will be found in a subsize of the given size.
    let counts: FxHashMap<u64, usize> = if binary.len() > 262144 /* 2 ^ 18 */ {
        let mut potentials: FxHashSet<u64> = FxHashSet::with_capacity_and_hasher(8192, Default::default());
        let index = (8192 * config.instr_len / BYTE_SIZE) as usize;
        for instr in iter_instructions(&binary[index..index*2], endiannes, config.instr_len) {
            potentials.insert(instr & ret_opcode_mask);
        }

        let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
        for instr in iter_instructions(binary, endiannes, config.instr_len) {
            let instr = instr & ret_opcode_mask;
            if potentials.contains(&instr) {
                counts.entry(instr).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        counts
    } else {
        // OPTION 1, simple solution with FxHasher
        let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
        for instr in iter_instructions(binary, endiannes, config.instr_len) {
            counts.entry(instr & ret_opcode_mask).and_modify(|e| *e += 1).or_insert(1);
        }
        counts
    };
    
    counts
        .into_iter()
        .filter(|(_, c)| c > &1) // Optimization, most instructions are unique, not need to consider them as return instruction
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(ret_search_range[0])
        .take(ret_search_range[1])
        .collect()
}

// TODO move to file call_edges.rs and have call edges struct
pub fn find_potential_edges(binary: &[u8], call_candidate: u64, config: &Config, endiannes: &Endiannes) -> Vec<(usize, usize)> {
    // Destructure CLI params we need TODO dereference let &Config...
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
        println!("USE {}", pc_offset);
        unimplemented!();
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