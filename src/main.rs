use crate::instructions::iter_potential_instructions;

mod instructions;
mod file;
mod cli;

fn main() {
    let cli_params = cli::parse_parameters();

    let binary = file::read_file(&cli_params);

    for i in iter_potential_instructions(&binary, &cli_params){
        println!("TEST {:#06x?}", &i[0 as usize..5 as usize])
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
