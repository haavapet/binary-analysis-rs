use crate::prelude::*;
use std::ops::Neg;
use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn call_candidates(instructions: &Vec<u64>, config: &Config) -> Vec<(u64, usize)> {
    // Destructure CLI params we need
    let Config { call_opcode_mask, 
                 call_search_range,
                 .. } = config;
    
    // OPTION 1, simple solution with FxHasher
    let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(1024, Default::default());
    for instr in instructions {
        counts.entry(instr & call_opcode_mask).and_modify(|e| *e += 1).or_insert(1);
    }

    counts
        .into_iter()
        .filter(|(c, _)| c > &1)
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(call_search_range[0] as usize)
        .take(call_search_range[1] as usize)
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

pub fn ret_candidates(instructions: &Vec<u64>, config: &Config) -> Vec<(u64, usize)> {
    // Destructure CLI params we need
    let Config { ret_opcode_mask, 
                 ret_search_range,
                 .. } = config;


    let mut counts: FxHashMap<u64, usize> = FxHashMap::with_capacity_and_hasher(8192, Default::default());
    for instr in instructions {
        counts.entry(instr & ret_opcode_mask).and_modify(|e| *e += 1).or_insert(1);
    }
    counts
        .into_iter()
        .filter(|(_, c)| c > &1) // Optimization, most instructions are unique, not need to consider them as return instruction
        .sorted_by_key(|&(_, count)| count)
        .rev()
        .skip(ret_search_range[0] as usize)
        .take(ret_search_range[1] as usize)
        .collect()
}

// TODO move to file call_edges.rs and have call edges struct
pub fn find_potential_edges(instructions: &Vec<u64>, call_candidate: u64, config: &Config) -> Vec<(usize, usize)> {
    // Destructure CLI params we need
    let Config { call_opcode_mask,
                 call_operand_mask,
                 call_operand_signed_mask,
                 pc_inc,
                 pc_offset,
                 left_shift_call_operand,
                 is_absolute_addressing,
                 .. } = config;
    let pc_inc: i64 = pc_inc.clone() as i64;
    let call_operand_mask = call_operand_mask.clone() as u64;
    let call_operand_signed_mask = call_operand_signed_mask.clone() as i64;
    if *is_absolute_addressing {
        println!("USE {}", pc_offset);
        unimplemented!();
    } else {
        let itosi = |x: u64, mask: i64| {
            let mut signed: i64 = x.clone() as i64;
            if signed > mask {
                signed |= mask.neg();
            }
            signed
        };

        let mut potential_edges: Vec<(usize, usize)> = Vec::new();
        for (i, instr) in instructions.iter().enumerate() {
            if instr & call_opcode_mask == call_candidate {
                let call_operand = instr & call_operand_mask;
                let signed_operand = itosi(call_operand, call_operand_signed_mask);
                // Maybe i.checked_add(rest) ?? Because we know the result should be unsigned
                let address = ((signed_operand << left_shift_call_operand) / pc_inc) + i as i64;
                if 0 <= address && address < instructions.len() as i64
                    && (i as i64 - address).abs() > 4 {
                        potential_edges.push((i as usize, address as usize));
                    }
            }
        }
        potential_edges
    }
}

pub fn filter_valid_edges(instructions: &Vec<u64>, ret_opcode: u64, config: &Config, potential_call_edges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let Config { ret_func_dist,
                 ret_opcode_mask,
                 .. } = config;
    
    let mut valid_call_edges: Vec<(usize, usize)> = Vec::new();

    for (from_edge, to_edge) in potential_call_edges {
        let to_edge = to_edge.clone();
        let from_edge = from_edge.clone();
        let is_first_instruction = to_edge == 0;
        for i in 1..(ret_func_dist + 1) {
            if ((to_edge as i64) - i as i64 >= 0) && (instructions[to_edge - i] & ret_opcode_mask == ret_opcode) || is_first_instruction {
                valid_call_edges.push((from_edge, to_edge));
                break;
            }
        }
    }
    valid_call_edges
}