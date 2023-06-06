use fs::{File, OpenOptions};
use io::{BufRead, BufReader, Write};
use std::{fs, io};

extern crate fqcomp;
use fqcomp::lookuptable::LookUpTable;
use fqcomp::{file_management, go_threads};

#[cfg(test)]
mod test_pipeline {
    use super::*;

    fn testing_newthreadpipeline(
        precision: u8,
        infile: &str,
        q_value: (usize, usize),
        thread_num: usize,
        file_size: usize,
        name: &'static str,
    ) {
        let split_info = file_management::split_file(infile, name, file_size);
        let divided_file_num = split_info.index;
        let last_file_size = split_info.last_size;
        let outname = name;

        go_threads::count(thread_num, q_value, divided_file_num, name);

        let mut prob_lut = LookUpTable::new(q_value);
        prob_lut.aggregate(
            q_value,
            name,
            divided_file_num,
            file_size,
            last_file_size,
            precision,
        );

        // Encoding
        go_threads::encode(thread_num, precision, divided_file_num, &prob_lut, name);

        // Decoding
        go_threads::decode(
            thread_num,
            divided_file_num,
            file_size,
            last_file_size,
            precision,
            &prob_lut,
            name,
            name,
        );

        file_management::merge_file(name, outname, divided_file_num);

        let openfastqfile = File::open(infile).expect("Can't open the file");
        let fastq_buffer = BufReader::new(openfastqfile);
        let decoded_merge_file = outname.to_owned() + "decoded.qs";

        let mut opendecodedfile = OpenOptions::new()
            .read(true)
            .append(true)
            .open(decoded_merge_file.clone())
            .expect("Can't open decoded file");
        opendecodedfile.write_all(&[0xA]).expect("insert error"); // insert \n at the end of the file to remove below
        let opendecodedfile = File::open(decoded_merge_file).expect("Can't open decoded file");
        let mut decoded_buffer = BufReader::new(opendecodedfile);

        for (index, line) in fastq_buffer.lines().enumerate() {
            if (index + 1) % 4 == 0 {
                let source = line.unwrap();

                let mut decoded_line = String::new();
                let _ = decoded_buffer.read_line(&mut decoded_line);
                decoded_line.pop(); // Remove \n at the end of the line

                assert_eq!(source, decoded_line); // Compare line by line
            }
        }

        file_management::remove_enc_sub_file(divided_file_num, name);
        file_management::remove_all_file(divided_file_num, name);
    }

    #[test]
    fn test_sample_newthreadpipeline() {
        let precision = 35;
        let infile = "../fqcomp/sample_data/sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let thread_num = 1;
        let file_size = 2;
        let name = "newthread_sample";

        testing_newthreadpipeline(precision, &infile, q_value, thread_num, file_size, name);
    }

    #[test]
    fn test_huge_newthreadpipeline() {
        let precision = 52;
        let infile = "../fqcomp/sample_data/huge_sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let thread_num = 4;
        let file_size = 250;
        let name = "newthread_huge";

        testing_newthreadpipeline(precision, &infile, q_value, thread_num, file_size, name);
    }
}
