use super::lookuptable::{write_bitnum, LookUpTable};
use super::range::Range;

use bitbit::BitWriter;
use fs::File;
use io::{BufRead, BufReader, BufWriter, Write};
use std::{fs, io};

const QVALUE: (usize, usize) = (33, 83);

pub struct Encoder {
    pending: u8, // number of bits pendded when range is in middle-range
    range: Range,
    lut: LookUpTable, // Probabilities look up table used by each encoder
}

impl Encoder {
    pub fn new(precision: u8, prob_lut: &LookUpTable) -> Self {
        /*
            Create constructor named Encoder and initialize parameters of Encoder

           Input:
            precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
            prob_lut : probabilities look up table to be copied
        */

        Self {
            pending: 0,
            range: Range::new(precision),
            lut: prob_lut.clone(),
        }
    }

    pub fn encode(&mut self, infile: &str, outfile: &str) {
        /*
            Opens a FATSQ file and reads quality scores line by line and passes quality scores to encoder.
            when the last quality score is encoded, this funciton terminates end of encoded bit sequence
            and pads zeros to terminate current bytes.

           Input:
            infile: qs file to be encoded
            outfile: encoded file
        */

        // read qs file
        let input_file = File::open(infile).expect("Can't open the file");
        let input_file = BufReader::new(input_file);
        let output_file = File::create(outfile).unwrap();
        let mut output_file = BufWriter::new(output_file);
        let mut output_buffer = BitWriter::new(&mut output_file);
        let mut header: bool = true;

        for line in input_file.lines() {
            let source = line.unwrap();
            let source_byte = source.as_bytes();
            let mut bit_buffer: Vec<bool> = Vec::new();

            //write quality scores length
            if header == true {
                write_bitnum(source_byte.len(), 10, &mut output_buffer);
                header = false;
            }

            // encode first symbol
            self.init_encoding(source_byte[0] as usize - self.lut.bias, &mut bit_buffer);

            // encode from 2'symbols
            for idx in 1..source_byte.len() {
                self.encoding(
                    source_byte[idx - 1] as usize - self.lut.bias,
                    source_byte[idx] as usize - self.lut.bias,
                    &mut bit_buffer,
                );
            }

            // terminate encoded sequence
            self.eof(&mut bit_buffer);

            write_bitnum(bit_buffer.len(), 10, &mut output_buffer);
            write_vector_to_bit(bit_buffer, &mut output_buffer);
        }

        // zero pads to terminate current bytes
        output_buffer.pad_to_byte().unwrap();
        // write buffer to file
        output_file.flush().unwrap();
    }

    fn init_encoding(&mut self, symbol: usize, bit_buffer: &mut Vec<bool>) {
        /*
           Encodes and initializes range using marginal probabilities for the first quality score

           Input:
            source: look up table of probabilities
            symbol: the first score of quality scores
            output_buffer: buffer to contain output bits
        */

        self.range.init_range(&self.lut, symbol);

        // find prefix low and high share
        while self.range.in_lower() || self.range.in_upper() {
            if self.range.in_upper() {
                self.output(true, bit_buffer);
                self.range.upper_renormalize();
            } else {
                self.output(false, bit_buffer);
                self.range.lower_renormalize();
            }
        }

        // find numbers of pendded bits
        while self.range.in_middle() {
            self.range.middle_renormalize();
            self.pending += 1;
        }
    }

    fn encoding(&mut self, pre_symbol: usize, current_symbol: usize, bit_buffer: &mut Vec<bool>) {
        /*
           Encodes and update range using conditional probabilities for the other scores except the first quality scores

           Input:
            source: look up table of probabilities
            pre_symbol: previous quality score
            current_symbol: current quality score
            output_buffer: buffer to contain output bits
        */

        self.range
            .update_range(&self.lut, pre_symbol, current_symbol);

        // find prefix low and high share
        while self.range.in_lower() || self.range.in_upper() {
            if self.range.in_upper() {
                self.output(true, bit_buffer);
                self.range.upper_renormalize();
            } else {
                self.output(false, bit_buffer);
                self.range.lower_renormalize();
            }
        }

        // find numbers of pendded bits
        while self.range.in_middle() {
            self.range.middle_renormalize();
            self.pending += 1;
        }
    }

    fn eof(&mut self, bit_buffer: &mut Vec<bool>) {
        /*
           At end of encoding, terminates end of encoded bits sequence
           and pads zeros in order to distinguish each encoded qulaity scores

           Input:
            output_buffer: buffer to contain output bits
        */

        self.pending += 1;

        // terminate sequence
        if self.range.in_quarter() {
            self.output(false, bit_buffer);
        } else {
            self.output(true, bit_buffer);
        }
    }

    fn output(&mut self, input: bool, bit_buffer: &mut Vec<bool>) {
        /*
           Writes bits to output file

           Input:
            input: bit encoder writes
            output_buffer: buffer to contain output bits
        */

        bit_buffer.push(input);
        while self.pending > 0 {
            bit_buffer.push(!input);
            self.pending -= 1;
        }
    }
}

fn write_vector_to_bit<T: Write>(input_vec: Vec<bool>, output_writer: &mut BitWriter<T>) {
    for bit in input_vec.iter() {
        output_writer.write_bit(*bit).expect("Fail to Write");
    }
}
