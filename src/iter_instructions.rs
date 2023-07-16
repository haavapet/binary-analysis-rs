use crate::prelude::*;

pub fn iter_potential_instruction_configuration<'a>(binary: &'a [u8], config: &'a Config) -> impl Iterator<Item = (&'a [u8], &'a Endiannes)> {
    let instr_byte_len = (config.instr_len / BYTE_SIZE) as usize;

    // TODO handle file offset, probably as simple as indexing binary below
    (0..instr_byte_len).filter_map(move |i| match i.cmp(&(instr_byte_len)) {
        std::cmp::Ordering::Less => match config.endiannes {
            Endiannes::Big | Endiannes::Little => Some((&binary[i..], &config.endiannes)),
            Endiannes::Unknown => todo!("Handle unknown endiannes")
        }
        _ => None
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
    .chunks_exact((instr_len / 8) as usize)
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

#[inline]
fn from_le_bytes_32(data: &[u8]) -> u64 {
    u32::from_le_bytes(data.try_into().unwrap()) as u64

    
    // OTHER OPTIONS, None of them are really faster than the above
    
    //unsafe { u32::from_le_bytes(data.try_into().unwrap_unchecked()) as u64 }

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
    
    //u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as u64

    //u32::from_le_bytes(*arrayref::array_ref!(data, 0, 4)) as u64
}

fn from_le_bytes_16(data: &[u8]) -> u64 {
    u16::from_le_bytes(data.try_into().unwrap()) as u64
}

fn from_le_bytes_8(data: &[u8]) -> u64 {
    u8::from_le_bytes(data.try_into().unwrap()) as u64
}