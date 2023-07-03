use crate::cli::{Parameters, Endiannes};

// hopefully this will let us lazy evaluate instructions, so we save up to 75% memory (for 32bit instr)
pub fn iter_potential_instructions<'a>(binary: &'a Vec<u8>, cli_params: &'a Parameters) -> impl 'a + Iterator<Item = Vec<u64>> {
    if cli_params.unknown_code_entry.unwrap(){
        // TODO implement possible code starts for unknown code entry
        // let possible_code_start = vec![(file_offset / instr_byte_len, file_offset_end / instr_byte_len)];
        todo!("Implement additional search when code entry unknown is set");
    }

    let instr_byte_len = cli_params.instr_len / 8;

    // Iterates over byte entry points. I.e to get alignment right for a 32 bit instruction length. 
    // possible instructions start at byte 0, 1, 2, 3.
    let iter_closure = |endiannes| (0..instr_byte_len).filter_map(move |i| match i.cmp(&(instr_byte_len)) {
        std::cmp::Ordering::Less => Some(extract_potential_instructions_from_binary(&binary[i as usize..], &endiannes, cli_params.instr_len)),
        _ => None
    });

    // If unknown endiannes we chain both big and little endian iterator
    if let Endiannes::Unknown = cli_params.endiannes {
        return Box::new(iter_closure(Endiannes::Big)
                            .chain(iter_closure(Endiannes::Little))
                        ) as Box<dyn Iterator<Item = Vec<u64>>>
    }

    // Else we just return <ENDIANNES> endiannes iterator
    Box::new(iter_closure(cli_params.endiannes)) as Box<dyn Iterator<Item = Vec<u64>>>
}

pub fn extract_potential_instructions_from_binary(binary: &[u8], endiannes: &Endiannes, instr_len: u32) -> Vec<u64> {
    let extraction_function = match endiannes {
        Endiannes::Big => match instr_len {
            8 => from_be_bytes_8,
            16 => from_be_bytes_16,
            32 => from_be_bytes_32,
            64 => from_be_bytes_64,
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        Endiannes::Little => match instr_len {
            8 => from_le_bytes_8,
            16 => from_le_bytes_16,
            32 => from_le_bytes_32,
            64 => from_le_bytes_64,
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        _ => unreachable!("This function should never be called with <UNKNOWN> endiannes")
    };
    
    binary
    .chunks_exact((instr_len / 8) as usize)
    .map(|x| extraction_function(x))
    .collect()
}

fn from_be_bytes_64(data: &[u8]) -> u64 {
    return u64::from_be_bytes(data.try_into().unwrap()) as u64;
}

fn from_be_bytes_32(data: &[u8]) -> u64 {
    return u32::from_be_bytes(data.try_into().unwrap()) as u64;
}

fn from_be_bytes_16(data: &[u8]) -> u64 {
    return u16::from_be_bytes(data.try_into().unwrap()) as u64;
}

fn from_be_bytes_8(data: &[u8]) -> u64 {
    return u8::from_be_bytes(data.try_into().unwrap()) as u64;
}

fn from_le_bytes_64(data: &[u8]) -> u64 {
    return u64::from_le_bytes(data.try_into().unwrap()) as u64;
}

fn from_le_bytes_32(data: &[u8]) -> u64 {
    return u32::from_le_bytes(data.try_into().unwrap()) as u64;
}

fn from_le_bytes_16(data: &[u8]) -> u64 {
    return u16::from_le_bytes(data.try_into().unwrap()) as u64;
}

fn from_le_bytes_8(data: &[u8]) -> u64 {
    return u8::from_le_bytes(data.try_into().unwrap()) as u64;
}




// Maybe this will work
// https://crates.io/crates/enum_dispatch
// allows to call methods for enum variants
// pub enum InstrLen2 {
//     L16,
//     L32
// }

// trait ToBeBytes2 {
//     fn to_be_bytes(&self, data: &[u8]) -> u64;
// }

// impl ToBeBytes2 for InstrLen2::L16 {
//     fn to_be_bytes(&self, data: &[u8]) -> u64 {
//         return u16::from_be_bytes(data.try_into().unwrap()) as u64;
//     }
// }




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