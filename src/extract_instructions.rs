use crate::prelude::*;

// hopefully this will let us lazy evaluate instructions, so we save up to 75% memory (for 32bit instr)
pub fn iter_potential_instructions<'a, T: FromBytes>(binary: &'a [u8], config: &'a Config) -> impl 'a + Iterator<Item = Vec<impl PrimInt>>
        where <T as FromBytes>::Output: PrimInt, T: 'a {
    // Destructure CLI params we need
    let Config { unknown_code_entry, 
                 file_offset,
                 endiannes,
                 instr_len,
                 .. } = config;
    
    if *unknown_code_entry {
        // TODO implement possible code starts for unknown code entry
        // let possible_code_start = vec![(file_offset / instr_byte_len, file_offset_end / instr_byte_len)];
        todo!("Implement additional search when code entry unknown is set");
    }

    // TODO how to handle exact file offset given, do we still iterate over bytes???
    // if file_offset.is_some() {
    //     todo!("Implement file offset")
    // } 

    let instr_byte_len = instr_len / BYTE_SIZE;
    let file_start = file_offset[0];
    let file_end = file_offset[1];

    // Iterates over byte entry points. I.e to get alignment right for a 32 bit instruction length. 
    // possible instructions start at byte 0, 1, 2, 3.                      //TODOOOO CHANGE TO INSTR_BYE_LEN
    let iter_closure = |endiannes| (0..instr_byte_len).filter_map(move |i| match i.cmp(&(instr_byte_len)) {
        std::cmp::Ordering::Less => Some(extract_potential_instructions_from_binary::<T>(&binary[file_start + (i  as usize)..file_end], &endiannes, instr_len)),
        _ => None
    });

    // If unknown endiannes we chain both big and little endian iterator
    // Box is used because of difference between chained and non-chained types
    if let Endiannes::Unknown = endiannes {
        return Box::new(iter_closure(Endiannes::Big)
                            .chain(iter_closure(Endiannes::Little))
                        ) as Box<dyn Iterator<Item = Vec<_>>>
    }

    // Else we just return <ENDIANNES> endiannes iterator
    Box::new(iter_closure(*endiannes)) as Box<dyn Iterator<Item = Vec<_>>>
}

pub fn extract_potential_instructions_from_binary<T: FromBytes>(binary: &[u8], endiannes: &Endiannes, instr_len: &u64) -> Vec<impl PrimInt>
        where <T as FromBytes>::Output: PrimInt {

        match endiannes {
            Endiannes::Big => binary
                                .chunks_exact((instr_len / BYTE_SIZE) as usize)
                                .map(T::from_be_bytes_mine)
                                .collect(),
            Endiannes::Little => binary
                                .chunks_exact((instr_len / BYTE_SIZE) as usize)
                                .map(T::from_le_bytes_mine)
                                .collect(),
        _ => unreachable!("This function should never be called with <UNKNOWN> endiannes")
    }
}  

pub trait FromBytes {
    type Output;
    fn from_be_bytes_mine(data: &[u8]) -> Self::Output;
    fn from_le_bytes_mine(data: &[u8]) -> Self::Output;
    fn from_bytes(data: &[u8], endiannes: &Endiannes) -> Self::Output;
}

impl FromBytes for u8 {
    type Output = u8;
    fn from_bytes(data: &[u8], endiannes: &Endiannes) -> u8 {
        if let Endiannes::Big = endiannes {
            u8::from_be_bytes(data.try_into().unwrap())
        } else {
            u8::from_le_bytes(data.try_into().unwrap())
        }
    }

    fn from_be_bytes_mine(data: &[u8]) -> u8 {
        u8::from_be_bytes(data.try_into().unwrap())
    }

    fn from_le_bytes_mine(data: &[u8]) -> u8 {
        u8::from_le_bytes(data.try_into().unwrap())
    }
}

impl FromBytes for u16 {
    type Output = u16;
    fn from_bytes(data: &[u8], endiannes: &Endiannes) -> u16 {
        if let Endiannes::Big = endiannes {
            u16::from_be_bytes(data.try_into().unwrap())
        } else {
            u16::from_le_bytes(data.try_into().unwrap())
        }
    }
    fn from_be_bytes_mine(data: &[u8]) -> u16 {
        u16::from_be_bytes(data.try_into().unwrap())
    }
    fn from_le_bytes_mine(data: &[u8]) -> u16 {
        u16::from_le_bytes(data.try_into().unwrap())
    }
}

impl FromBytes for u32 {
    type Output = u32;
    fn from_bytes(data: &[u8], endiannes: &Endiannes) -> u32 {
        if let Endiannes::Big = endiannes {
            u32::from_be_bytes(data.try_into().unwrap())
        } else {
            u32::from_le_bytes(data.try_into().unwrap())
        }
    }
    fn from_be_bytes_mine(data: &[u8]) -> u32 {
        u32::from_be_bytes(data.try_into().unwrap())
    }
    fn from_le_bytes_mine(data: &[u8]) -> u32 {
        u32::from_le_bytes(data.try_into().unwrap())
    }
}

impl FromBytes for u64 {
    type Output = u64;
    fn from_bytes(data: &[u8], endiannes: &Endiannes) -> u64 {
        if let Endiannes::Big = endiannes {
            u64::from_be_bytes(data.try_into().unwrap())
        } else {
            u64::from_le_bytes(data.try_into().unwrap())
        }
    }
    fn from_be_bytes_mine(data: &[u8]) -> u64 {
        u64::from_be_bytes(data.try_into().unwrap())
    }
    fn from_le_bytes_mine(data: &[u8]) -> u64 {
        u64::from_le_bytes(data.try_into().unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn trivial() {
        assert_eq!(1, 1)
    }
}