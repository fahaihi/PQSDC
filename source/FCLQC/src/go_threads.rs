use crate::concurrency::counterhandler::CounterHandler;
use crate::concurrency::decoderhandler::DecoderHandler;
use crate::concurrency::encoderhandler::EncoderHandler;
use crate::lookuptable::LookUpTable;

pub fn count(
    thread_num: usize,
    q_value: (usize, usize),
    divided_file_num: usize,
    name: &'static str,
) {
    /*
    Encode quality scores

    Input:
        thread_num: number of threads to use
        q_value: minimum and maximum ascii values of quality values
        divided_file_num: number of divided files
        name: name of the run
    */

    let threads = CounterHandler::create_counters(thread_num, q_value, name);

    // Assume that lengths of all reads are the same
    for file_idx in 1..divided_file_num + 1 {
        threads.go_count(file_idx);
    }
}

pub fn encode(
    thread_num: usize,
    precision: u8,
    divided_file_num: usize,
    lut: &LookUpTable,
    name: &'static str,
) {
    /*
    Encode quality scores

    Input:
        thread_num: number of threads to use
        precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
        divided_file_num: number of divided files
        lut: look up table of probabilities
        name: name of the run
    */

    let threads = EncoderHandler::create_encoders_order(thread_num, precision, lut, name);

    // Assume that lengths of all reads are the same
    for file_idx in 1..divided_file_num + 1 {
        threads.go_encode(file_idx);
    }
}

pub fn decode(
    thread_num: usize,
    divided_file_num: usize,
    file_size: usize,
    last_file_size: usize,
    precision: u8,
    lut: &LookUpTable,
    name: &'static str,
    outname: &'static str,
) {
    /*
    Decode compressed data to quality scores

    Input:
        thread_num: number of threads to useores
        divided_file_num: number of divided files
        file_size: number of quality scores in each file
        last_file_size: number of quality scores in last file
        lut: look up table of probabilities
        name: name of the run
    */

    let threads = DecoderHandler::create_decoders(precision, thread_num, lut, name, outname);

    for file_idx in 1..divided_file_num + 1 {
        if file_idx != divided_file_num {
            threads.go_decode(file_idx, 1, file_size, false);
        } else if file_idx == divided_file_num {
            threads.go_decode(file_idx, 1, last_file_size, false);
        } else {
            println!("file index error");
        }
    }
}

pub fn random_access(
    thread_num: usize,
    first_index: usize,
    last_index: usize,
    first_line: usize,
    last_line: usize,
    file_size: usize,
    precision: u8,
    lut: &LookUpTable,
    name: &'static str,
    outname: &'static str,
) {
    /*
    Decode compressed data to quality scores

    Input:
        thread_num: number of threads to useores
        divided_file_num: number of divided files
        file_size: number of quality scores in each file
        last_file_size: number of quality scores in last file
        lut: look up table of probabilities
        name: name of the run
    */

    let threads = DecoderHandler::create_decoders(precision, thread_num, lut, name, outname);

    if first_index == last_index {
        threads.go_decode(first_index + 1, first_line, last_line, true);
    } else {
        for file_idx in (first_index + 1)..(last_index + 2) {
            if first_index + 1 == file_idx {
                threads.go_decode(file_idx, first_line, file_size, true);
            } else if last_index + 1 == file_idx {
                threads.go_decode(file_idx, 1, last_line, true);
            } else {
                threads.go_decode(file_idx, 1, file_size, true);
            }
        }
    }
}
