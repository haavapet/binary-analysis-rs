pub fn read_file(file_path: &str) -> Vec<u8>{
    std::fs::read(file_path)
        .expect("file not found")
}