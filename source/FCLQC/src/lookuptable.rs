use crate::counter::Counter;
use crate::file_management::{get_probability_fname, naming};
use bitbit::reader::Bit;
use bitbit::{BitReader, BitWriter, MSB};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::mem;

#[derive(Debug, Clone)]
pub struct LookUpTable {
    pub marginal: Vec<u64>,           // marginal count of initial quality values
    pub total: u64,                   // total count of initial quality values
    pub cond_total: Vec<u64>,         // total conditional count of quality values
    pub conditional: Vec<Vec<u64>>,   // conditional count of quality values
    pub p_marginal: Vec<f64>,         // marginal distribution of initial quality values
    pub p_cumulative: Vec<f64>,       // cumulative distribution of initial quality values
    pub p_conditional: Vec<Vec<f64>>, // conditional distributions of quality values
    pub p_cond_cumulative: Vec<Vec<f64>>, // cumulative conditional distributions of quality values
    pub bias: usize,                  // bias of ascii value of quality values (33 in most cases)
    pub q_dim: usize,                 // number of different characters of quality scores
}

impl LookUpTable {
    pub fn new(qvalue: (usize, usize)) -> Self {
        /*
           Count reads the fastq file and extract empirical marginal and conditional distributions

           Input:
            qvalue : minimum and maximum ascii values of quality values
        */

        let symbol_len = qvalue.1 - qvalue.0 + 1;

        // Set 0 vector for conditional distributions and cumulative conditional distributions
        Self {
            marginal: vec![0; symbol_len],
            total: 0,
            cond_total: vec![0; symbol_len],
            conditional: vec![vec![0u64; symbol_len]; symbol_len],
            p_marginal: vec![0.0; symbol_len],
            p_cumulative: vec![0.0; symbol_len],
            p_conditional: vec![vec![0f64; symbol_len]; symbol_len],
            p_cond_cumulative: vec![vec![0f64; symbol_len]; symbol_len],
            bias: qvalue.0,
            q_dim: symbol_len,
        }
    }

    pub fn generate(&mut self, name: &str) {
        let mut subfile = File::open(name).unwrap();
        let decoded: Counter = bincode::deserialize_from(&mut subfile).unwrap();
        let mut marginal_cumulative: f64 = 0.0;
        let epsilon = 1e-15;

        // get total and cond_total
        for idx1 in 0..self.q_dim {
            self.marginal[idx1] = decoded.marginal[idx1];
            self.total += self.marginal[idx1];
            for idx2 in 0..self.q_dim {
                self.conditional[idx1][idx2] = decoded.conditional[idx1][idx2];
                self.cond_total[idx1] += self.conditional[idx1][idx2];
            }
        }

        self.normalize();

        for (index, marginal) in self.p_marginal.iter().enumerate() {
            marginal_cumulative += *marginal;
            self.p_cumulative[index] = marginal_cumulative;
        }
        if (1.0 - epsilon) <= self.p_cumulative[self.q_dim - 1] {
            self.p_cumulative[self.q_dim - 1] = 1.0;
        }

        let mut conditional_cumulative: f64 = 0.0;
        for (idx1, condition) in self.p_conditional.iter().enumerate() {
            for (idx2, prob) in condition.iter().enumerate() {
                conditional_cumulative += *prob;
                self.p_cond_cumulative[idx1][idx2] = conditional_cumulative;
            }
            if (1.0 - epsilon) <= self.p_cond_cumulative[idx1][self.q_dim - 1] {
                self.p_cond_cumulative[idx1][self.q_dim - 1] = 1.0;
            }
            conditional_cumulative = 0.0;
        }
    }

    pub fn aggregate(
        &mut self,
        qvalue: (usize, usize),
        name: &str,
        divided_file_num: usize,
        filesize: usize,
        last_file_size: usize,
        precision: u8,
    ) {
        /*
         */

        let mut counter = Counter::new(qvalue);
        for file_idx in 1..divided_file_num + 1 {
            let subfile_name = naming(name, "cnt", file_idx);
            let mut subfile = File::open(&subfile_name).unwrap();
            let decoded: Counter = bincode::deserialize_from(&mut subfile).unwrap();
            counter.merge(&decoded);
        }

        // get total and cond_total
        for idx1 in 0..self.q_dim {
            self.marginal[idx1] = counter.marginal[idx1];
            self.total += self.marginal[idx1];
            for idx2 in 0..self.q_dim {
                self.conditional[idx1][idx2] = counter.conditional[idx1][idx2];
                self.cond_total[idx1] += self.conditional[idx1][idx2];
            }
        }

        self.normalize();

        let prob_fname = get_probability_fname(name);
        self.write_probability(
            &prob_fname,
            divided_file_num,
            filesize,
            last_file_size,
            precision,
        );
    }

    pub fn count_update(&mut self, input: &str) {
        /*
            Counts the number of occurences of initial quality value (marginal),
            and counts the number of occurences of pair of quality values (conditional).

            Input:
             input: line that contains quality scores from fastq file
        */

        let sym = input.as_bytes();
        for idx in 0..input.len() - 1 {
            self.conditional[sym[idx] as usize - self.bias][sym[idx + 1] as usize - self.bias] += 1;
            self.cond_total[sym[idx] as usize - self.bias] += 1;
        }
        self.marginal[sym[0] as usize - self.bias] += 1;
        self.total += 1;
    }

    pub fn normalize(&mut self) {
        /*
            After all the count updates
            it computes marginal distribution, cumulative distribution,
            conditional distributions, and cumulative conditional distributions
            by normalization.
        */

        let total = self.total as f64;
        for previous_symbol in 0..self.q_dim {
            // normalize marignal distribution
            self.p_marginal[previous_symbol] = self.marginal[previous_symbol] as f64 / total;

            // normalize conditional distribution
            let cond_total = self.cond_total[previous_symbol] as f64;
            for current_symbol in 0..self.q_dim {
                if cond_total == 0.0 {
                    break;
                } else {
                    self.p_conditional[previous_symbol][current_symbol] =
                        self.conditional[previous_symbol][current_symbol] as f64 / cond_total;
                }
            }
        }
    }

    pub fn write_probability(
        &mut self,
        output_name: &str,
        divided_file_num: usize,
        filesize: usize,
        last_filesize: usize,
        precision: u8,
    ) {
        /*
            After normalization, it writes marginal and conditional probability to the file
            and updates marginal distribution, cumulative distribution,
            conditional distributions, and cumulative conditional distributions
            Input:
             output_name: file name to save Probability
        */

        let outfile = File::create(output_name).unwrap();
        let mut outfile = BufWriter::new(outfile);
        let mut bit_buffer = BitWriter::new(&mut outfile);
        let mut marginal_cumulative: f64 = 0.0;
        let epsilon = 1e-15;

        write_bitnum(divided_file_num, 10, &mut bit_buffer);
        write_bitnum(filesize, 64, &mut bit_buffer);
        write_bitnum(last_filesize, 64, &mut bit_buffer);
        write_bitnum(precision as usize, 8, &mut bit_buffer);

        for (index, marginal) in self.p_marginal.iter().enumerate() {
            probability_to_file(*marginal, &mut bit_buffer);
            marginal_cumulative += *marginal;
            self.p_cumulative[index] = marginal_cumulative;
        }
        if (1.0 - epsilon) <= self.p_cumulative[self.q_dim - 1] {
            self.p_cumulative[self.q_dim - 1] = 1.0;
        }

        let mut conditional_cumulative: f64 = 0.0;
        for (idx1, condition) in self.p_conditional.iter().enumerate() {
            for (idx2, prob) in condition.iter().enumerate() {
                probability_to_file(*prob, &mut bit_buffer);
                conditional_cumulative += *prob;
                self.p_cond_cumulative[idx1][idx2] = conditional_cumulative;
            }
            if (1.0 - epsilon) <= self.p_cond_cumulative[idx1][self.q_dim - 1] {
                self.p_cond_cumulative[idx1][self.q_dim - 1] = 1.0;
            }
            conditional_cumulative = 0.0;
        }

        bit_buffer.pad_to_byte().unwrap();
        outfile.flush().unwrap();
    }

    pub fn get_probability(&self, previous_symbol: usize, current_symbol: usize) -> (f64, f64) {
        /*
            After all the updates (count_update, cumulative_update) and normalization
            it gets conditional distributions

            Input:
             previous_symbol: previous quality score
             current_symbol: current quality score
            Output:
             (range_low, range_high): cumulative probabilities for arithmetic coding
             range_low is P(x<current_symbol|previous_symbol)
             range_high is P(x<=current_symbol|previous_symbol)
        */

        let high = self.p_cond_cumulative[previous_symbol][current_symbol];
        let mut low = 0.0;
        if current_symbol != 0 {
            low = self.p_cond_cumulative[previous_symbol][current_symbol - 1];
        }

        (low, high)
    }

    pub fn get_init_probability(&self, current_symbol: usize) -> (f64, f64) {
        /*
            After all the updates (count_update, cumulative_update) and normazliation
            it gets the marginal distribution

            Input:
             current_symbol: current quality score
            Output:
             (range_low, range_high): cumulative probabilities for arithmetic coding of the first quality value
             range_low is P(x<current_symbol)
             range_high is P(x<=current_symbol)
        */

        let high = self.p_cumulative[current_symbol];
        let mut low = 0.0;
        if current_symbol != 0 {
            low = self.p_cumulative[current_symbol - 1];
        }

        (low, high)
    }

    pub fn read_lut(&mut self, lut_file: &str) -> (usize, usize, usize, u8) {
        /*
            Read probability from the file and update marginal and conditional fraction.

            Input:
                lut_file: file that stores the probability
        */

        let input = File::open(lut_file).expect("Can't open file");
        let input = BufReader::new(input);
        let mut in_reader: BitReader<_, MSB> = BitReader::new(input);
        let epsilon = 1e-15;

        let filenum = get_bitnum(10, &mut in_reader) as usize;
        let filesize = get_bitnum(64, &mut in_reader) as usize;
        let last_size = get_bitnum(64, &mut in_reader) as usize;
        let precision = get_bitnum(8, &mut in_reader) as u8;

        let mut marginal_cumulative: f64 = 0.0;
        for index in 0..self.q_dim {
            self.p_marginal[index] = unsafe { mem::transmute(read_bit(&mut in_reader)) };
            marginal_cumulative += self.p_marginal[index];
            self.p_cumulative[index] = marginal_cumulative;
        }
        if (1.0 - epsilon) <= self.p_cumulative[self.q_dim - 1] {
            self.p_cumulative[self.q_dim - 1] = 1.0;
        }

        let mut conditional_cumulative: f64 = 0.0;
        for idx1 in 0..self.q_dim {
            for idx2 in 0..self.q_dim {
                self.p_conditional[idx1][idx2] =
                    unsafe { mem::transmute(read_bit(&mut in_reader)) };
                conditional_cumulative += self.p_conditional[idx1][idx2];
                self.p_cond_cumulative[idx1][idx2] = conditional_cumulative;
            }
            if (1.0 - epsilon) <= self.p_cond_cumulative[idx1][self.q_dim - 1] {
                self.p_cond_cumulative[idx1][self.q_dim - 1] = 1.0;
            }
            conditional_cumulative = 0.0;
        }
        (filenum, filesize, last_size, precision)
    }
}

fn probability_to_file<T: Write>(input: f64, output_buffer: &mut BitWriter<T>) {
    /*
        writes probabilities to the file.
        Because probability is nonnegative number,
        If the probability of a quality score is 0, then output is only a bit(0).
        Otherwise, output starts with 1(true).

        Input:
            input: probability writtened
            output_buffer: buffer to contain output bits
    */

    if input == 0.0 {
        write_bit(0, output_buffer);
    } else {
        let input_to_int: u64 = unsafe { mem::transmute(input) };
        write_bit(input_to_int, output_buffer);
    }
}

fn write_bit<T: Write>(input: u64, output_buffer: &mut BitWriter<T>) {
    /*
       Writes bits to output file

       Input:
        input: bit encoder writes
        output_buffer: buffer to contain output bits
    */

    if input == 0 {
        output_buffer.write_bit(false).unwrap(); // flag to notify that the probability is 0
    } else {
        output_buffer.write_bit(true).unwrap(); // flag to notify that the probability is not 0
        let mut bit_mask: i8 = 62; // to avoid what bit_flag become negative, i8 conversion needed

        while bit_mask >= 0 {
            if (input & 1 << bit_mask) != 0 {
                output_buffer.write_bit(true).unwrap();
            } else {
                output_buffer.write_bit(false).unwrap();
            }
            bit_mask -= 1;
        }
    }
}

pub fn write_bitnum<T: Write>(input: usize, number: usize, output_writer: &mut BitWriter<T>) {
    for shift in 0..number {
        let temp = (input >> shift) & 1;
        if temp == 1 {
            output_writer.write_bit(true).expect("Fail to Write");
        } else {
            output_writer.write_bit(false).expect("Fail to Write");
        }
    }
}

fn read_bit<R: Read, B: Bit>(bit_source: &mut BitReader<R, B>) -> u64 {
    /*
       Reads and loads bits from the file.
       If MSB iS 0, output is 0(that probability is 0).
       Otherwise, it reads 63 bits from the file

       Input:
        bit_source: bitreader to load a bit from compressed quality scores
    */

    let mut probability: u64 = 0;
    if get_bit(bit_source) == 0 {
        return 0;
    } else {
        for _i in 0..63 {
            probability = (probability << 1) | get_bit(bit_source);
        }
        return probability;
    }
}

fn get_bit<R: Read, B: Bit>(bit_source: &mut BitReader<R, B>) -> u64 {
    /*
       loads a bit from file

       Input:
        bit_source: bitreader to load a bit from file
       Output:
        bit: loaded bit  *Because this bit will be caculated with u64 type, it returns u64
    */
    bit_source.read_bit().expect("Decoder Can't read the file") as u64
}

pub fn get_bitnum<R: Read, B: Bit>(number: usize, bit_source: &mut BitReader<R, B>) -> u64 {
    let mut output = 0;
    for i in 0..number {
        output = output | (bit_source.read_bit().unwrap() as u64) << i;
    }
    output
}
