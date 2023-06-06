//#![cfg(feature = "probability")]
extern crate fqcomp;
use fqcomp::lookuptable::LookUpTable;
use fqcomp::{file_management, go_threads};
use std::fs;

const SMALL_SAMPLE_SIZE: usize = 2;
const HUGE_SAMPLE_SIZE: usize = 2500;

#[cfg(test)]
mod test_lookuptable {
    use super::*;

    fn get_lut(
        infile: &str,
        q_value: (usize, usize),
        name: &'static str,
        file_size: usize,
        precision: u8,
    ) -> LookUpTable {
        let thread_num = 1;
        let split_info = file_management::split_file(infile, name, file_size);
        go_threads::count(thread_num, q_value, split_info.index, name);
        let mut prob_lut = LookUpTable::new(q_value);
        prob_lut.aggregate(
            q_value,
            name,
            split_info.index,
            file_size,
            split_info.last_size,
            precision,
        );

        prob_lut
    }

    #[test]
    fn test_lut_small() {
        let infile = "../fqcomp/sample_data/small_sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let name = "test_lut_small";
        let file_size = SMALL_SAMPLE_SIZE;
        let precision = 35;

        let prob_lut = get_lut(infile, q_value, name, file_size, precision);
        let prob_output = file_management::get_probability_fname(name);

        assert!(prob_lut.bias == 33);
        assert!(prob_lut.q_dim == 41);

        // test get_init_probability
        let init_range = prob_lut.get_init_probability(12);
        assert!(init_range == (0.0, 0.5));

        // test get_probability
        let range = prob_lut.get_probability(12, 23);
        assert!(range == (0.0, 1.0));

        fs::remove_file(prob_output).unwrap();
        file_management::remove_format_file(1, name, "qs");
        file_management::remove_format_file(1, name, "id");
        file_management::remove_format_file(1, name, "read");
        file_management::remove_format_file(1, name, "cnt");
    }

    #[test]
    fn test_lut_huge() {
        let infile = "../fqcomp/sample_data/huge_sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let name = "test_lut_huge";
        let file_size = HUGE_SAMPLE_SIZE;
        let precision = 35;

        let prob_lut = get_lut(infile, q_value, name, file_size, precision);
        let prob_output = file_management::get_probability_fname(name);

        assert!(prob_lut.bias == 33);
        assert!(prob_lut.q_dim == 41);

        // test get_init_probability
        let init_range = prob_lut.get_init_probability(12);
        let eps = 1e-8;
        assert!(init_range.0 == 0.0);
        assert!((init_range.1 - 0.3912).abs() < eps);

        // test get_probability
        let range = prob_lut.get_probability(40, 39);
        let target0 = 0.142660669154;
        let target1 = 0.145276605096;

        assert!((range.0 - target0).abs() < eps);
        assert!((range.1 - target1).abs() < eps);

        fs::remove_file(prob_output).unwrap();
        file_management::remove_format_file(1, name, "qs");
        file_management::remove_format_file(1, name, "id");
        file_management::remove_format_file(1, name, "read");
        file_management::remove_format_file(1, name, "cnt");
    }

    #[test]
    fn test_lut_sum() {
        let infile = "../fqcomp/sample_data/huge_sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let name = "test_lut_sum";
        let q_dim = q_value.1 - q_value.0;
        let file_size = HUGE_SAMPLE_SIZE;
        let precision = 35;

        let prob_lut = get_lut(infile, q_value, name, file_size, precision);
        let prob_output = file_management::get_probability_fname(name);

        assert_eq!(prob_lut.p_cumulative[q_dim], 1.0, "cumulative error");

        for condition in 0..q_dim + 1 {
            if prob_lut.p_cond_cumulative[condition][q_dim] != 0.0 {
                assert_eq!(
                    prob_lut.p_cond_cumulative[condition][q_dim], 1.0,
                    "conditional cumulative error"
                );
            }
        }

        fs::remove_file(prob_output).unwrap();
        file_management::remove_format_file(1, name, "qs");
        file_management::remove_format_file(1, name, "id");
        file_management::remove_format_file(1, name, "read");
        file_management::remove_format_file(1, name, "cnt");
    }

    #[test]
    fn test_encoder_decoder_lut() {
        let infile = "../fqcomp/sample_data/huge_sample.fastq";
        let q_value: (usize, usize) = (33, 73);
        let name = "test_enc_dec_lut";
        let q_dim = q_value.1 - q_value.0;
        let file_size = HUGE_SAMPLE_SIZE;
        let precision = 35;

        let encoder_prob_lut = get_lut(infile, q_value, name, file_size, precision);
        let prob_output = file_management::get_probability_fname(name);

        let mut decoder_prob_lut = LookUpTable::new(q_value);
        let (_d_filenum, d_filesize, _d_last, d_precision) =
            decoder_prob_lut.read_lut(&prob_output);

        assert!(d_filesize == file_size);
        assert!(precision == d_precision);

        for previous in 0..q_dim + 1 {
            assert!(encoder_prob_lut.p_marginal[previous] == decoder_prob_lut.p_marginal[previous]);
            for current in 0..q_dim + 1 {
                assert!(
                    encoder_prob_lut.p_conditional[previous][current]
                        == decoder_prob_lut.p_conditional[previous][current]
                );
            }
        }

        fs::remove_file(prob_output).unwrap();
        file_management::remove_format_file(1, name, "qs");
        file_management::remove_format_file(1, name, "id");
        file_management::remove_format_file(1, name, "read");
        file_management::remove_format_file(1, name, "cnt");
    }
}
