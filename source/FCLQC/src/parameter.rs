use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Parameters {
    pub precision: u8,     // number of precision needed for encoder
    pub file_size: usize,  // number of quality score to compose of sub files
    pub thread_num: usize, // number of thread to run compressor and decompressor
    pub first_line: usize,
    pub last_line: usize,
}

impl Parameters {
    pub fn read(infile: &str) -> Parameters {
        let mut file = File::open(infile).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let parameter: Parameters = serde_json::from_str(&buff).unwrap();

        parameter
    }
}
