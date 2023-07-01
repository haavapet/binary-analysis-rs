use crate::cli::Parameters;

pub fn read_file(params: &Parameters) -> Vec<u8>{
    std::fs::read(&params.file_path)
        .expect("file not found")
}