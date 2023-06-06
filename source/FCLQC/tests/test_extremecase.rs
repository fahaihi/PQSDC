use fs::{File, OpenOptions};
use io::{BufRead, BufReader, Write};
use std::{fs, io};

extern crate fqcomp;
use fqcomp::decoder::Decoder;
use fqcomp::encoder::Encoder;
use fqcomp::lookuptable::LookUpTable;

#[cfg(test)]
mod test_extremecase {
    use super::*;

    fn testing_pipeline(
        precision: u8,
        infile: &str,
        file_size: usize,
        encoded: &str,
        decoded: &str,
        prob_lut: &LookUpTable,
    ) {
        let mut encoder = Encoder::new(precision, prob_lut);
        encoder.encode(&infile, encoded);

        let mut decoder = Decoder::new(precision, prob_lut);
        decoder.decode(encoded, decoded, file_size);

        let openfastqfile = File::open(infile).expect("Cannot open the file");
        let fastq_buffer = BufReader::new(openfastqfile);

        let mut opendecodedfile = OpenOptions::new()
            .read(true)
            .append(true)
            .open(decoded)
            .expect("Can't open decoded file");
        opendecodedfile.write_all(&[0xA]).expect("insert error"); // insert \n at the end of the file to use pop method
        let opendecodedfile = File::open(decoded).expect("Can't open file"); // reset cursor into start
        let mut decoded_buffer = BufReader::new(opendecodedfile);

        for line in fastq_buffer.lines() {
            let source = line.unwrap();

            let mut decoded_line = String::new();
            let _ = decoded_buffer.read_line(&mut decoded_line);
            decoded_line.pop(); // Remove \n at the end of the line

            assert_eq!(source, decoded_line); // Compare line by line
        }

        fs::remove_file(encoded).expect("Can't remove file");
        fs::remove_file(decoded).expect("Can't remove file");
    }

    #[test]
    fn test_extreme_low_upper() {
        let precisions = [30, 52];
        let infile = "../fqcomp/sample_data/extreme.qs";
        let q_value: (usize, usize) = (33, 73);
        let file_size = 3;
        let encoded = "encoded_extreme_low_upper.fastq";
        let decoded = "decoded_extreme_low_upper.fastq";
        let prob_output_name = "probability_extreme_low_upper.txt";
        let mut prob_lut = LookUpTable::new(q_value);
        prob_lut.marginal[0] = 1;
        prob_lut.marginal[(prob_lut.q_dim - 1) / 2] = 3;
        prob_lut.marginal[prob_lut.q_dim - 1] = 1;
        prob_lut.total = 5;
        prob_lut.conditional[prob_lut.q_dim - 1][0] = 1;
        prob_lut.conditional[prob_lut.q_dim - 1][(prob_lut.q_dim - 1) / 2] = 3;
        prob_lut.conditional[prob_lut.q_dim - 1][prob_lut.q_dim - 1] = 1;
        prob_lut.cond_total[prob_lut.q_dim - 1] = 5;
        prob_lut.conditional[0][0] = 1;
        prob_lut.conditional[0][(prob_lut.q_dim - 1) / 2] = 3;
        prob_lut.conditional[0][prob_lut.q_dim - 1] = 1;
        prob_lut.cond_total[0] = 5;
        prob_lut.normalize();

        for precision in precisions.iter() {
            prob_lut.write_probability(prob_output_name, 1, file_size, file_size, *precision);
            testing_pipeline(*precision, &infile, file_size, encoded, decoded, &prob_lut);
        }
        fs::remove_file(prob_output_name).unwrap();
    }

    #[test]
    fn test_extreme_probability() {
        let precisions = [52];
        let infile = "../fqcomp/sample_data/extreme_probability.qs";
        let q_value: (usize, usize) = (33, 73);
        let file_size = 3;
        let encoded = "encoded_extreme_probability.fastq";
        let decoded = "decoded_extreme_probability.fastq";
        let prob_output_name = "probability_extreme.txt";
        let occurrences: Vec<u64> = vec![
            30000000,
            300000000,
            3000000000,
            30000000000,
            300000000000,
            3000000000000,
        ];

        for occurrence in occurrences.iter() {
            let mut prob_lut = LookUpTable::new(q_value);
            prob_lut.marginal[0] = 1;
            prob_lut.marginal[prob_lut.q_dim - 1] = *occurrence;
            prob_lut.total = 1 + *occurrence;
            prob_lut.conditional[prob_lut.q_dim - 1][0] = 1;
            prob_lut.conditional[prob_lut.q_dim - 1][prob_lut.q_dim - 1] = *occurrence;
            prob_lut.cond_total[prob_lut.q_dim - 1] = 1 + *occurrence;
            prob_lut.conditional[0][0] = 1;
            prob_lut.conditional[0][prob_lut.q_dim - 1] = *occurrence;
            prob_lut.cond_total[0] = 1 + *occurrence;
            prob_lut.normalize();

            for precision in precisions.iter() {
                prob_lut.write_probability(prob_output_name, 1, file_size, file_size, *precision);
                testing_pipeline(*precision, &infile, file_size, encoded, decoded, &prob_lut);
            }
        }

        fs::remove_file(prob_output_name).unwrap();
    }
}
