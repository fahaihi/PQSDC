use fs::File;
use io::{BufRead, BufReader, Write};
use std::{fs, io};

pub struct SplitInfo {
    pub index: usize,
    pub last_size: usize,
}

struct Writer {
    pub buffer: String,
    pub fname: String,
    pub fformat: String,
    pub fpath: String,
    pub target_file: File,
}

impl Writer {
    pub fn new(fname_in: &str, fformat: &str, file_idx: usize) -> Self {
        let fpath = naming(fname_in, fformat, file_idx);
        let target_file = File::create(fpath.clone()).expect("Can't create file");
        let buffer = String::from("");

        Self {
            buffer: buffer,
            fname: String::from(fname_in),
            fformat: String::from(fformat),
            fpath: fpath,
            target_file: target_file,
        }
    }

    pub fn renew(&mut self, file_idx: usize) {
        self.fpath = naming(&self.fname, &self.fformat, file_idx);
        self.buffer = String::from("");
        self.target_file = File::create(self.fpath.clone()).expect("Can't create file");
    }

    pub fn push_line(&mut self, new_line: &str) {
        self.buffer.push_str(new_line);
        self.buffer.push('\n');
    }

    pub fn pop_last_symbol(&mut self) {
        self.buffer.pop();
    }

    pub fn write_buffer_to_file(&mut self) {
        match self.target_file.write_all(self.buffer.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", self.fname, why),
            Ok(_) => {}
        }
    }
    pub fn remove_last_file(&mut self) {
        fs::remove_file(self.fpath.clone()).unwrap();
    }
}

struct FastqBuffer {
    pub qs_writer: Writer,
    pub read_writer: Writer,
    pub id_writer: Writer,
    pub file_idx: usize,
}

impl FastqBuffer {
    pub fn new(name: &str) -> Self {
        let file_idx = 1;
        let qs_writer = Writer::new(name, "qs", file_idx);
        let read_writer = Writer::new(name, "read", file_idx);
        let id_writer = Writer::new(name, "id", file_idx);

        Self {
            qs_writer: qs_writer,
            read_writer: read_writer,
            id_writer: id_writer,
            file_idx: file_idx,
        }
    }

    pub fn renew(&mut self) {
        self.file_idx += 1;
        self.qs_writer.renew(self.file_idx);
        self.read_writer.renew(self.file_idx);
        self.id_writer.renew(self.file_idx);
    }

    pub fn pop_last_symbol(&mut self) {
        self.qs_writer.pop_last_symbol();
        self.read_writer.pop_last_symbol();
        self.id_writer.pop_last_symbol();
    }

    pub fn push_id_line(&mut self, new_line: &str) {
        self.id_writer.push_line(new_line);
    }

    pub fn push_read_line(&mut self, new_line: &str) {
        self.read_writer.push_line(new_line);
    }

    pub fn push_qs_line(&mut self, new_line: &str) {
        self.qs_writer.push_line(new_line);
    }

    pub fn write_buffer_to_file(&mut self) {
        self.qs_writer.write_buffer_to_file();
        self.read_writer.write_buffer_to_file();
        self.id_writer.write_buffer_to_file();
    }

    pub fn remove_last_file(&mut self) {
        self.file_idx -= 1;
        self.qs_writer.remove_last_file();
        self.read_writer.remove_last_file();
        self.id_writer.remove_last_file();
    }
}

pub fn get_probability_fname(name: &str) -> String {
    /*
    Name the file that contains probability look up table

    Input:
        name: name of the run
    Output:
        String: File name with index
    */

    format!("{}.prob", name)
}

pub fn naming(file_name: &str, file_format: &str, file_index: usize) -> String {
    /*
    Name the file according to the specific format

    Input:
        file_name: name of th file
        file_index: index of the subfile
        file_format: format of the subfile
    Output:
        String: File name with index
    */

    format!("{}{:03}.{}", file_name, file_index, file_format)
}

pub fn split_file(infile: &str, name: &str, file_size: usize) -> SplitInfo {
    /*
    Split the input file into subfiles

    Input:
        infile: input file name
        name: name of the run
        file_size: size of each subfile
    Output:
        split info that contains number of files and the last file size
    */

    let openfile = File::open(infile).expect("Can't open the file");
    let openfile = BufReader::new(openfile);

    let mut fqbuffer = FastqBuffer::new(name);

    let mut last_file_size = 0;

    // read FATQ File line by line and split id, name, and quality values
    for (index, line) in openfile.lines().enumerate() {
        let message = line.unwrap();

        if index % 4 == 0 {
            fqbuffer.push_id_line(&message);
        } else if index % 4 == 1 {
            fqbuffer.push_read_line(&message);
        } else if index % 4 == 2 {
            // do nothing
        } else if index % 4 == 3 {
            fqbuffer.push_qs_line(&message);
        }

        last_file_size += 1;
        if last_file_size == 4 * file_size {
            fqbuffer.pop_last_symbol();
            fqbuffer.write_buffer_to_file();
            fqbuffer.renew();
            last_file_size = 0;
        }
    }

    // delete the last created file, when file_size is divisor of origin size of FASTQ-file
    if last_file_size == 0 {
        fqbuffer.remove_last_file();
        last_file_size = file_size;
    } else {
        fqbuffer.pop_last_symbol();
        fqbuffer.write_buffer_to_file();
        last_file_size = last_file_size / 4;
    }

    SplitInfo {
        index: fqbuffer.file_idx,
        last_size: last_file_size,
    }
}

#[warn(unused_variables)]
pub fn merge_file(name: &str, outname: &str, divided_file_num: usize) {
    /*
    Merge subfiles into a file and delete subfile

    Input:
        name: name of the run
        outname: output name
        divided_file_num: number of divided file
    */

    let outfile_name = outname.to_owned() + "decoded.qs";
    let mut outfile = File::create(outfile_name).unwrap();
    let mut start_file = true;

    for file_idx in 1..divided_file_num + 1 {
        let subfile_name = naming(name, "dec", file_idx);
        let subfile = File::open(&subfile_name).unwrap();
        let subfile = BufReader::new(subfile);

        for line in subfile.lines() {
            if !start_file {
                outfile.write_all(&[0xA]).unwrap(); // write \n except the first line of files
            }

            outfile.write_all(line.unwrap().as_bytes()).unwrap();
            start_file = false;
        }
    }
}

#[warn(unused_variables)]
pub fn merge_randfile(name: &str, outname: &str, first_index: usize, last_index: usize) {
    let outfile_name = outname.to_owned() + ".dec";
    let mut outfile = File::create(outfile_name).unwrap();
    let mut start_file = true;

    for file_idx in first_index + 1..last_index + 2 {
        let subfile_name = naming(name, "dec", file_idx);
        let subfile = File::open(&subfile_name).unwrap();
        let subfile = BufReader::new(subfile);

        for line in subfile.lines() {
            if !start_file {
                outfile.write_all(&[0xA]).unwrap(); // write \n except the first line of files
            }

            outfile.write_all(line.unwrap().as_bytes()).unwrap();
            start_file = false;
        }
    }

    for file_index in first_index + 1..last_index + 2 {
        let format_file = naming(name, "dec", file_index);
        fs::remove_file(format_file).expect(&format!("Can't remove dec file"));
    }
}

pub fn remove_format_file(divided_file_num: usize, name: &str, format: &str) {
    /*
    Remove files created during split

    Input:
        divided_file_num: number(index) of divided files
        name: name of run
        format: format of file
    */

    for file_index in 1..divided_file_num + 1 {
        let format_file = naming(name, format, file_index);
        fs::remove_file(format_file).expect(&format!("Can't remove {} file", format));
    }
}

pub fn remove_enc_sub_file(divided_file_num: usize, name: &str) {
    /*
    Remove files created during split, encoding, and decoding

    Input:
        divided_file_num: number(index) of divided files
        name: name of run
    */
    remove_format_file(divided_file_num, name, "qs");
    remove_format_file(divided_file_num, name, "read");
    remove_format_file(divided_file_num, name, "id");
    remove_format_file(divided_file_num, name, "cnt");
}

pub fn remove_dec_sub_file(divided_file_num: usize, name: &str) {
    /*
    Remove files created during split, encoding, and decoding

    Input:
        divided_file_num: number(index) of divided files
        name: name of run
    */

    remove_format_file(divided_file_num, name, "dec");
}

pub fn remove_all_file(divided_file_num: usize, name: &str) {
    /*
    Remove files created during split, encoding, and decoding

    Input:
        divided_file_num: number(index) of divided files
        name: name of run
    */

    remove_format_file(divided_file_num, name, "enc");
    remove_format_file(divided_file_num, name, "dec");
    fs::remove_file(name.to_owned() + "decoded.qs").expect("Can't remove decoded file");
    let prob_fname = get_probability_fname(name);
    fs::remove_file(prob_fname).expect("Can't remove lut file");
}

pub fn get_encoded_size(name: &str, divided_file_num: usize) -> u64 {
    /*
    Adds the size of encoded subfiles and returns the size of an encoded file.

    Input:
        name: name of the run
        divided_file_num: number of divided files
    Output:
        encoded_size_buffer: size of the encoded file.
    */
    let mut encoded_size_buffer = 0;
    for idx in 1..divided_file_num + 1 {
        let file_name = naming(name, "enc", idx);
        let encoded = File::open(file_name).unwrap();
        encoded_size_buffer += encoded.metadata().unwrap().len();
    }
    encoded_size_buffer
}

pub fn get_metadata(infile: &str) -> (usize, usize) {
    /*
    Reads file and returns the length of quality scores and file size
    Input:
        infile: FASTQ File
    Output:
        (quality_scores_len, file_len)
        quality_scores_len: legth of quality scores
        file_len: number of lines in FASTQ file
    */
    let file = File::open(infile).unwrap();
    let file = BufReader::new(file);
    let mut file_len: usize = 0;
    let mut quality_scores_len: usize = 0;
    for (idx, line) in file.lines().enumerate() {
        file_len += 1;
        if idx == 3 {
            quality_scores_len = line.unwrap().len();
        }
    }

    (quality_scores_len, file_len / 4)
}
