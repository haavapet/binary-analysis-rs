
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
    
    for potential_instructions in iter_potential_instructions(&binary, &config){
        for call_candidates in instructions::call_candidates(&potential_instructions, &config){
            // potential_edges()     pc_inc, pc_offet, is_relative, call_operand_mask 
            // ret_candidates()      ret_search_range
            // filter_edges          ret_func_dist
            // 
        }
    }

    // for potential_instructions in binary.extract_potential_instructios(){
    //      for call_opcode in potential_instructions.call_candidates(){
    //            potential_edges = ...
    //            for ret_opcode in potential_instructions.ret_candidates(){
    //                  filter_edges... probability ... addtoheap...
    //            }
    //      }
    // }    
}
