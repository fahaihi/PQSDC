extern crate fqcomp;
use fqcomp::file_management;

#[cfg(test)]
mod test_split {
    use super::*;

    #[test]
    fn testing_split_even() {
        let infile = "../fqcomp/sample_data/sample.fastq";
        let name = "uneven";
        let file_size = 10;

        let split_info = file_management::split_file(infile, name, file_size);
        file_management::remove_format_file(split_info.index, name, "qs");
        file_management::remove_format_file(split_info.index, name, "read");
        file_management::remove_format_file(split_info.index, name, "id");
    }
    #[test]
    fn testing_split_uneven() {
        let infile = "../fqcomp/sample_data/sample.fastq";
        let name = "sample_uneven";
        let file_size = 11;

        let split_info = file_management::split_file(infile, name, file_size);
        file_management::remove_format_file(split_info.index, name, "qs");
        file_management::remove_format_file(split_info.index, name, "read");
        file_management::remove_format_file(split_info.index, name, "id");
    }
}
