extern crate fqcomp;
use fqcomp::file_management;
use fqcomp::go_threads;
use fqcomp::lookuptable::LookUpTable;
use std::fs;

#[cfg(test)]
mod test_counterhandler {
    use super::*;

    fn test_counterhandler(
        thread_num: usize,
        infile: &str,
        name: &'static str,
        file_size: usize,
        precision: u8,
    ) {
        let split_info = file_management::split_file(infile, name, file_size);
        let q_value = (33, 73);

        go_threads::count(thread_num, q_value, split_info.index, name);

        let mut lut = LookUpTable::new(q_value);
        lut.aggregate(
            q_value,
            name,
            split_info.index,
            file_size,
            split_info.last_size,
            precision,
        );

        file_management::remove_format_file(split_info.index, name, "cnt");
        file_management::remove_format_file(split_info.index, name, "read");
        file_management::remove_format_file(split_info.index, name, "qs");
        file_management::remove_format_file(split_info.index, name, "id");

        let prob_outfile = file_management::get_probability_fname(name);
        fs::remove_file(prob_outfile).expect("Can't remove probability out file");
    }

    #[test]
    fn test_even_counterhandler() {
        let thread_num = 2;
        let infile = "../fqcomp/sample_data/sample.fastq";
        let name = "test_even_counterhandler";
        let file_size = 10;
        let precision = 35;

        test_counterhandler(thread_num, infile, name, file_size, precision);
    }

    #[test]
    fn test_uneven_counterhandler() {
        let thread_num = 2;
        let infile = "../fqcomp/sample_data/sample.fastq";
        let name = "test_uneven_counterhandler";
        let file_size = 11;
        let precision = 35;

        test_counterhandler(thread_num, infile, name, file_size, precision);
    }
}
