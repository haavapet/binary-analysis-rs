use crate::prelude::*;
use itertools::iproduct;

pub fn iter_potential_instruction_configuration<'a>(binary: &'a [u8], config: &'a Config) -> impl Iterator<Item = (&'a [u8], &'a Endiannes)> {
    let instr_byte_len = (config.instr_len / BYTE_SIZE) as usize;
    
    let endiannes = if let Endiannes::Unknown = config.endiannes {
        vec![&Endiannes::Big, &Endiannes::Little]
    } else {
        vec![&config.endiannes]
    };

    // TODO also add addressing mode

    iproduct!((0..instr_byte_len), endiannes)
        .filter_map(move |(byte_index, endiannes)| match byte_index.cmp(&(instr_byte_len)) {
            std::cmp::Ordering::Less => Some((&binary[config.file_offset[0] + byte_index..config.file_offset[1]], endiannes)),
            _ => unreachable!()
        })
}

pub fn iter_instructions<'a>(binary: &'a [u8], endiannes: &Endiannes, instr_len: u64) -> impl Iterator<Item = u64> + 'a {
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
        Endiannes::Unknown => unimplemented!()
    };
    binary
      .chunks_exact((instr_len / BYTE_SIZE) as usize)
      .map(extraction_function)
}

fn from_be_bytes_64(data: &[u8]) -> u64 {
    u64::from_be_bytes(data.try_into().unwrap())
}

fn from_be_bytes_32(data: &[u8]) -> u64 {
    u32::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_be_bytes_16(data: &[u8]) -> u64 {
    u16::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_be_bytes_8(data: &[u8]) -> u64 {
    u8::from_be_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_64(data: &[u8]) -> u64 {
    u64::from_le_bytes(data.try_into().unwrap())
}

fn from_le_bytes_32(data: &[u8]) -> u64 {
    u32::from_le_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_16(data: &[u8]) -> u64 {
    u16::from_le_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_8(data: &[u8]) -> u64 {
    u8::from_le_bytes(data.try_into().unwrap()) as u64
}

// Due to being called from a hot code path, inlining this and having it be a seperate function from iter_instruction
// is more efficient, most likely due to some compiler optimizations
// Seems to be about 15% improvement
pub fn iter_instructions_and_search(binary: &[u8], endiannes: &Endiannes, instr_len: u64, mask: u64, target: u64) -> bool {
    match endiannes {
        Endiannes::Big => match instr_len { 
            8 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u8::from_be_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            16 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u16::from_be_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            32 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u32::from_be_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            64 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u64::from_be_bytes(data.try_into().unwrap()))
                    .any(|x| x & mask == target),
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        Endiannes::Little => match instr_len {
            8 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u8::from_le_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            16 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u16::from_le_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            32 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u32::from_le_bytes(data.try_into().unwrap()))
                    .any(|x| x as u64 & mask == target),
            64 => binary
                    .chunks_exact((instr_len / BYTE_SIZE) as usize)
                    .map(|data| u64::from_le_bytes(data.try_into().unwrap()))
                    .any(|x| x & mask == target),
            _ => unreachable!("Instr_len should only be one of [8, 16, 32, 64]")
        },
        Endiannes::Unknown => unimplemented!()
    }
}





// OTHER OPTIONS, None of them are really faster than the above
    
// unsafe { u32::from_le_bytes(data.try_into().unwrap_unchecked()) as u64 }

// unsafe {
//     // Show why safe
//     std::slice::from_raw_parts(data.as_ptr() as *const u32, 1)[0] as u64
// }

// match data[..] {
//     [a, b, c, d] => u32::from_le_bytes([a, b, c, d]) as u64,
//     _ => unreachable!()
// }

// u32::from_le_bytes(unsafe {
//     *((&data[0..4]).as_ptr() as *const [u8; 4])
// }) as u64

// u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as u64

// u32::from_le_bytes(*arrayref::array_ref!(data, 0, 4)) as u64