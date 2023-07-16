use crate::prelude::*;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::iter_instructions::iter_instructions;

pub fn call_candidates(binary: &[u8], config: &Config, endiannes: &Endiannes) -> Vec<(u64, usize)> {
    // Destructure CLI params we need
    let &Config { call_opcode_mask, 
                 call_search_range,
                 .. } = config;

    let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
    for instr in iter_instructions(binary, endiannes, config.instr_len) {
        counts.entry(instr & call_opcode_mask).and_modify(|e| *e += 1).or_insert(1);
    }

    counts
        .into_iter()
        .filter(|(c, _)| c > &10)
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(call_search_range[0])
        .take(call_search_range[1])
        .collect()
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
        .filter(|(_, c)| c > &10) // Optimization, most instructions are unique, not need to consider them as return instruction
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(ret_search_range[0])
        .take(ret_search_range[1])
        .collect()
}

// Some other options that has been tested

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