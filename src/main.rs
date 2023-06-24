mod instructions;
mod file;
mod cli;

use instructions::InstrInfo;


fn main() {
    let args = cli::parse_parameters();
    println!("{}", args.file_path);
    println!("{}", args.instr_len);

    // PARAMETERS, TODO factor to CLI input
    let file_path = String::from("binaries/chipquarium.ch8");
    let instr_len: u32 = 16;
    let instr_byte_len: u32 = instr_len / 8;
    let endiannes = instructions::Endiannes::Big;
    let file_offset: u32 = 0;
    let file_offset_end: u32 = 0x430;

    let binary = file::read_file(&file_path);

    for byte_index in 0..instr_byte_len
    {
        let binary_slice = &binary[byte_index as usize..file_offset_end as usize];
        let instructions = instructions::extract_potential_instructions_from_binary(binary_slice, &endiannes, instr_byte_len);
        instructions.call_candidates();
        println!("{}", instructions.len());
    }

    for a in instructions::it1(){
        
    }



    // for e in instructions
    // {
    //     println!("v: {:#06x}", e);
    // }


    // instructions: Instructions = parse_instructions()

    // call candidates = instructions.call_candidates()

}
