use core::time;

pub enum Endiannes {
    Little,
    Big
}

// Create a custom iterator struct and trait and impl
// hopefully this will let us lazy evaluate instructions, so we save up to 75% memory (for 32bit instr)

pub fn it1() -> impl Iterator<Item = Vec<u64>> {
    // change 3 to instr_byte_len
    // let possible_code_start = vec![(file_offset / instr_byte_len, file_offset_end / instr_byte_len)];
    
    (0..3).filter_map(move |i| match i.cmp(&3) {
        std::cmp::Ordering::Less => None,      // Code A, does not emit item
        _ => unreachable!(),
    })
}

pub trait InstrInfo{
    fn call_candidates(&self);
}

impl InstrInfo for Vec<u64> {
    fn call_candidates(&self){
        println!("Call candidate called")
    }
}


// WORKING HARDCODED
pub fn extract_potential_instructions_from_binary(binary: &[u8], endiannes: &Endiannes, instr_byte_len: u32) -> Vec<u64> {

    let extraction_function = match endiannes {
        Endiannes::Big => u16::from_be_bytes,
        Endiannes::Little => u16::from_le_bytes,
    };
    binary
        .chunks_exact(instr_byte_len as usize)
        .map(|x| extraction_function(x.try_into().unwrap()) as u64)
        .collect()
}





// Idea how to use enum instead for length of vec
// pub enum InstrLen { // RENAME INSTRUCTIONLENGTH
//     L8(u8), 
//     L16(u16),
//     L32(u32),
//     L64(u64)
// }

// trait Serializer{
//     fn from_be_bytes(data: &[u8], instr_len: InstrLen) -> InstrLen;
// }
// impl Serializer for InstrLen
// {
//     fn from_be_bytes(data: &[u8], instr_len: InstrLen) -> InstrLen
//     {
//         match instr_len {
//             InstrLen::L16(_) => InstrLen::L16(u16::from_be_bytes(data.try_into().unwrap())),
//             _ => panic!("Not implemented")
//         }
//         //InstrLen::L16(u16::from_be_bytes(data.try_into().unwrap()))
//     }
// }

// pub fn read_instructions_from_file(file_path: &str) -> Vec<InstrLen> {
//     std::fs::read(file_path)
//         .unwrap()
//         .chunks_exact(2)
//         .map(|x| InstrLen::from_be_bytes(x, InstrLen::L16(0)))
//         .collect()
// }