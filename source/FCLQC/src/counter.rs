use std::fs::File;
use std::io::{BufRead, BufReader};

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Counter {
    pub marginal: Vec<u64>,         // marginal count of initial quality values
    pub conditional: Vec<Vec<u64>>, // conditional count of quality values
    pub bias: usize,                // bias of ascii value of quality values (33 in most cases)
    pub q_dim: usize,               // number of different characters of quality scores
}

impl Counter {
    pub fn new(qvalue: (usize, usize)) -> Self {
        /*
           Count quality scores from qs files

           Input:
            qvalue : minimum and maximum ascii values of quality values
           Output:
            new Counter object
        */

        let symbol_len = qvalue.1 - qvalue.0 + 1;

        Self {
            marginal: vec![0; symbol_len],
            conditional: vec![vec![0u64; symbol_len]; symbol_len],
            bias: qvalue.0,
            q_dim: symbol_len,
        }
    }

    pub fn count(&mut self, in_fname: &str, out_fname: &str) {
        /*
           Count quality scores from qs files

           Input:
            in_fname: name of the file that stores quality score
            out_fname: name of the file to write cnt result
        */

        let input_file = File::open(in_fname).expect("Can't open the file");
        let input_buffer = BufReader::new(input_file);

        for line in input_buffer.lines() {
            let message = line.unwrap().clone();
            let qsline: Vec<_> = message.chars().collect();
            for idx in 1..qsline.len() {
                self.conditional[qsline[idx - 1] as usize - self.bias]
                    [qsline[idx] as usize - self.bias] += 1;
            }
            self.marginal[qsline[0] as usize - self.bias] += 1;
        }

        self.write_count(out_fname);
    }

    fn write_count(&mut self, out_fname: &str) {
        /*
           Write count result to cnt file

           Input:
            out_fname: name of the file to write cnt result
        */

        let mut out_file = File::create(out_fname).unwrap();
        bincode::serialize_into(&mut out_file, &self).unwrap();
    }

    pub fn merge(&mut self, counter: &Counter) {
        /*
           Merge cnt results

           Input:
            counter: counter object to be merged
        */
        for idx1 in 0..self.q_dim {
            self.marginal[idx1] += counter.marginal[idx1];
            for idx2 in 0..self.q_dim {
                self.conditional[idx1][idx2] += counter.conditional[idx1][idx2];
            }
        }
    }
}
