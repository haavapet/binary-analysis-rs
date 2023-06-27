mod instructions;
mod file;
mod cli;

fn main() {
    let args = cli::parse_parameters();

    let binary = file::read_file(&args);

    // for potential_instructions in binary.extract_potential_instructios(){
    //      for call_opcode in potential_instructions.call_candidates(){
    //            potential_edges = ...
    //            for ret_opcode in potential_instructions.ret_candidates(){
    //                  filter_edges... probability ... addtoheap...
    //            }
    //      }
    // }    
}
