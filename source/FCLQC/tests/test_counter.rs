//#![cfg(feature = "probability")]
extern crate fqcomp;
use fqcomp::counter::Counter;
use std::fs;

#[test]
fn test_counter_small() {
    let q_value: (usize, usize) = (33, 73);
    let mut sample_counter = Counter::new(q_value);

    let in_fname = "../fqcomp/sample_data/sample.qs";
    let out_fname = "../fqcomp/result.txt";

    sample_counter.count(in_fname, out_fname);

    let mut out_file = fs::File::open(out_fname).unwrap();
    let decoded: Counter = bincode::deserialize_from(&mut out_file).unwrap();

    assert_eq!(sample_counter, decoded);

    fs::remove_file(out_fname).unwrap();
}
