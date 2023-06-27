use std::path::PathBuf;

use crate::cli::Parameters;

pub fn read_file(params: &Parameters) -> Vec<u8>{
    std::fs::read(params.file_path.as_ref().unwrap())
        .expect("file not found")
}