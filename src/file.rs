use crate::prelude::*;

pub fn read_file(config: &Config) -> Vec<u8> {
    // Destructure CLI params we need
    let Config { file_path, .. } = config;

    std::fs::read(file_path).expect("file not found")
}

pub fn read_file_len(file_path: &PathBuf) -> usize {
    std::fs::metadata(file_path)
        .expect("file does not exist")
        .len() as usize
}
