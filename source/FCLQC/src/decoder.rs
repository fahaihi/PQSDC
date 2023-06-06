use super::lookuptable::{get_bitnum, LookUpTable};
use super::range::Range;
use bitbit::reader::Bit;
use bitbit::{BitReader, MSB};
use std::fs::File;
use std::io::{BufReader, LineWriter, Read, Write};

pub struct Decoder {
    range: Range, //upper and lower range of arithmetic code
    precision: u8,
    quality_scores_len: usize, //lengh of a quality socres
    cur_sym: usize,            //current quality score
    init: bool,                //flag to notify start of decoding
    input: u64,                //value of the input within the range
    symbol_count: usize, //Number to which current score belongs in quality scores, this is used to notify end of decoding
    lut: LookUpTable,    // probabilities look up table used by decoder
    reset: u64,
}

impl Decoder {
    pub fn new(precision: u8, prob_lut: &LookUpTable) -> Self {
        /*
           Creates constructor named Decoder and initializes parameters of Decoder

           Input:
           precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
           qaulity_scores_len: length of a quality scores
           prob_lut : probabilities look up table to be copied
        */

        Self {
            range: Range::new(precision),
            input: 0,
            precision,
            quality_scores_len: 0,
            init: true,
            cur_sym: 0,
            symbol_count: 0,
            lut: prob_lut.clone(),
            reset: 0,
        }
    }

    pub fn decode(&mut self, infile: &str, outfile_name: &str, file_size: usize) {
        /*
            Opens a file of compressed quality scores and reads and passes file to line decoder.
            when the last quality score is decoded, this funciton write 0xA(ascii of 'ENTER') to distinguish each quality scores

           Input:
           infile: encoded file name
           outfile_name: decoded file name
           file_size: number of quality score lines
        */

        let input = File::open(infile).expect("Can't open file");
        let input = BufReader::new(input);
        let mut in_reader: BitReader<_, MSB> = BitReader::new(input);
        let outfile = File::create(outfile_name).expect("Can't create file");
        let mut outfile = LineWriter::new(outfile);

        //read quality score's length
        self.quality_scores_len = get_bitnum(10, &mut in_reader) as usize;
        let mut bitnum = get_bitnum(10, &mut in_reader);

        for _ in 0..self.quality_scores_len {
            let buffer = [self.decoding(bitnum, &mut in_reader)];
            outfile.write_all(&buffer).expect("decoding write error");
        }
        self.reset = 0;

        for _ in 1..file_size {
            outfile.write_all(b"\n").expect("decoding write \n error");
            bitnum = get_bitnum(10, &mut in_reader);
            for _ in 0..self.quality_scores_len {
                let buffer = [self.decoding(bitnum, &mut in_reader)];
                outfile.write_all(&buffer).expect("decoding write error");
            }
            self.reset = 0;
        }

        outfile.flush().unwrap();
    }

    pub fn randomacess_decode(
        &mut self,
        infile: &str,
        outfile_name: &str,
        first_pos: usize,
        file_size: usize,
    ) {
        let input = File::open(infile).expect("Can't open file");
        let input = BufReader::new(input);
        let mut in_reader: BitReader<_, MSB> = BitReader::new(input);
        let outfile = File::create(outfile_name).expect("Can't create file");
        let mut outfile = LineWriter::new(outfile);
        let mut flag = false;

        //read quality score's length
        self.quality_scores_len = get_bitnum(10, &mut in_reader) as usize;

        if first_pos == 1 {
            let bitnum = get_bitnum(10, &mut in_reader);
            for _ in 0..self.quality_scores_len {
                let buffer = [self.decoding(bitnum, &mut in_reader)];
                outfile.write_all(&buffer).expect("decoding write error");
            }
        } else {
            for _ in 0..first_pos - 1 {
                skip_bit(get_bitnum(10, &mut in_reader), &mut in_reader);
            }
            flag = true;
        }

        self.reset = 0;

        for _ in first_pos..file_size {
            if !flag {
                outfile.write_all(b"\n").expect("decoding write \n error");
            }
            let bitnum = get_bitnum(10, &mut in_reader);
            for _ in 0..self.quality_scores_len {
                let buffer = [self.decoding(bitnum, &mut in_reader)];
                outfile.write_all(&buffer).expect("decoding write error");
            }
            self.reset = 0;
            flag = false;
        }

        outfile.flush().unwrap();
    }

    fn decoding<R: Read, B: Bit>(&mut self, bitnum: u64, bit_source: &mut BitReader<R, B>) -> u8 {
        /*
            Initializes bit stream and passes bit stread to each symbol decoder and offer decoding process

           Input:
              bit_source: bitreader to load a bit from compressed quality scores
           Output: decoded symbol
        */

        if self.init {
            self.symbol_init(bitnum, bit_source);
            self.cur_sym = self.first_symbol_decoding();
            self.symbol_count += 1;
            self.init = false;
        } else {
            let pre_sym = self.cur_sym - self.lut.bias;
            self.cur_sym = self.symbol_decoding(pre_sym);
            self.symbol_count += 1;
        }

        if self.symbol_count == self.quality_scores_len {
            self.input = 0;
            self.symbol_count = 0;
            self.init = true;
            return self.cur_sym as u8;
        }

        while self.range.in_lower() || self.range.in_upper() {
            if self.range.in_lower() {
                self.range.lower_renormalize();
                self.input = (self.input * 2) | self.get_bit(bitnum, bit_source);
            } else if self.range.in_upper() {
                self.range.upper_renormalize();
                self.input =
                    ((self.input - self.range.half) * 2) | self.get_bit(bitnum, bit_source);
            }
        }

        while self.range.in_middle() {
            self.range.middle_renormalize();
            self.input =
                ((self.input - self.range.quarter) << 1) | self.get_bit(bitnum, bit_source);
        }

        return self.cur_sym as u8;
    }

    fn symbol_init<R: Read, B: Bit>(&mut self, bitnum: u64, bit_source: &mut BitReader<R, B>) {
        /*
           Reads and loads as many as 'precision' bits from file

           Input:
            bit_source: bitreader to load a bit from compressed quality scores
        */

        for _i in 0..self.precision {
            self.input = (self.input << 1) | self.get_bit(bitnum, bit_source);
        }
    }

    fn first_symbol_decoding(&mut self) -> usize {
        /*
            Decodes the first quality score using binary search based on marginal probabilities and updates range

           Output:
              sym_idx: index of the first quality score
        */

        let mut sym_low = 0;
        let mut sym_high = self.lut.q_dim - 1;
        let mut sym_idx;
        loop {
            sym_idx = (sym_high + sym_low) / 2;
            self.range.init_range(&self.lut, sym_idx);
            if self.range.low <= self.input && self.input < self.range.high {
                return sym_idx + self.lut.bias;
            } else if self.input >= self.range.high {
                sym_low = sym_idx + 1;
            } else {
                sym_high = sym_idx - 1;
            }
        }
    }

    fn symbol_decoding(&mut self, pre_symbol: usize) -> usize {
        /*
           Decodes the other symbols except the first quality score using binary search based on conditional probabilities
           and updates range

           Input:
             pre_symbol: decoded previous quality score of current quality score
           Output:
             sym_idx: index of the first quality score
        */

        let mut sym_low = 0;
        let mut sym_high = self.lut.q_dim - 1;
        let mut sym_idx;
        loop {
            sym_idx = (sym_high + sym_low) / 2;
            let cur_range = self.range.find_range(&self.lut, pre_symbol, sym_idx);
            if cur_range.0 <= self.input && self.input < cur_range.1 {
                self.range.update_range(&self.lut, pre_symbol, sym_idx);
                return sym_idx + self.lut.bias;
            } else if self.input >= cur_range.1 {
                sym_low = sym_idx + 1;
            } else {
                sym_high = sym_idx - 1;
            }
        }
    }

    fn get_bit<R: Read, B: Bit>(&mut self, bitnum: u64, bit_source: &mut BitReader<R, B>) -> u64 {
        /*
           loads a bit from compressed quality scores

           Input:
            bitnum: the number of bits
            bit_source: bitreader to load a bit from compressed quality scores
           Output:
            bit: loaded bit  *Because this bit will be caculated with u64 type, it returns u64
        */

        if self.reset == bitnum {
            return 0;
        } else {
            self.reset += 1;
            let output = bit_source.read_bit().expect("Decoder Can't read the file") as u64;
            output
        }
    }
}

fn skip_bit<R: Read, B: Bit>(number: u64, bit_source: &mut BitReader<R, B>) {
    for _ in 0..number {
        bit_source.read_bit().unwrap();
    }
}
