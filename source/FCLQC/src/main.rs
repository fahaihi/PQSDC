extern crate fqcomp;
use fqcomp::lookuptable::LookUpTable;
use fqcomp::parameter::Parameters;
use fqcomp::{file_management, go_threads};
use std::io::Write;
use std::time::Instant;
use std::{env, fs, process};

#[allow(dead_code)]
const QVALUE: (usize, usize) = (33, 83);

macro_rules! println_err {
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
}

macro_rules! exit_with_error(
    ($code:expr, $($arg:tt)*) => {
        println_err!($($arg)*);
        process::exit($code);
    }
);

fn main() {
    let options = match Options::from_args() {
        Some(o) => o,
        None => {
            exit_with_error!(
                1,
                "Usage: \n
            cargo.exe [Cargo OPTIONS] [MAIN OPTIONS] [INPUT FILE |OUTPUT NAME| PARAMETER FILE] \n\n
            MAINT OPTIONS: [-c | -d ]  
                    -c : run compressor[default] \n
                    -d : run decompressor\n
                    -h : help
                    
                    Example:\n 
                    run compressor in release mode \n
                    cargo run --release -- -c input.fastq outputname parameter.json\n
                    cargo run --release -- -d input'name' outputname parameter.json\n
                    \n
                    when compressing, precision, size of subfile, and number of thread are required for parameter.serde_json.\n
                    when decompressiong, only number of thread is required.\n
                    "
            );
        }
    };

    if options.compressor_flag.unwrap() {
        run_compressor(
            &options.input.unwrap(),
            options.output.unwrap(),
            &options.parameter.unwrap(),
        );
    } else {
        if !options.rand_flag.unwrap() {
            run_decompressor(
                options.input.unwrap(),
                options.output.unwrap(),
                &options.parameter.unwrap(),
            );
        } else {
            run_randomacess(
                options.input.unwrap(),
                options.output.unwrap(),
                &options.parameter.unwrap(),
            );
        }
    }
}

fn run_compressor(infile: &str, outfile: String, parameter_file: &str) {
    /*
       Run quality score compression and decompression pipeline.

       Input:
        infile: input quality score file (currently Fastq file)
        parameter_file : json file that contains parameters needed for running encoder and decoder.
        remove_flag :  flag to remove the generated file  or not
    */

    let parameter = Parameters::read(parameter_file);
    let out_name = string_to_static_str(outfile);

    // Split origin Fast file to sub files composed of 'file_size' quality scores and update prob_lut
    let start = Instant::now();
    let split_info = file_management::split_file(infile, out_name, parameter.file_size);
    let divided_file_num = split_info.index;
    let last_file_size = split_info.last_size;
    let end = start.elapsed();
    println!("split time {:?}", end);

    let start = Instant::now();
    go_threads::count(parameter.thread_num, QVALUE, divided_file_num, out_name);
    let mut prob_lut = LookUpTable::new(QVALUE);
    prob_lut.aggregate(
        QVALUE,
        out_name,
        divided_file_num,
        parameter.file_size,
        last_file_size,
        parameter.precision,
    );

    // Encoding
    go_threads::encode(
        parameter.thread_num,
        parameter.precision,
        divided_file_num,
        &prob_lut,
        out_name,
    );
    let end = start.elapsed();
    println!("encoding time {:?}", end);

    println!(
        "encoded size = {} BYTES",
        file_management::get_encoded_size(out_name, divided_file_num)
    );

    file_management::remove_enc_sub_file(divided_file_num, out_name);
}

fn run_decompressor(infile: String, outfile: String, parameter_file: &str) {
    let parameter = Parameters::read(parameter_file);
    let thread_num = parameter.thread_num;
    let in_name = string_to_static_str(infile);
    let out_name = string_to_static_str(outfile);
    let mut decoder_lut = LookUpTable::new(QVALUE);
    let prob_fname = file_management::get_probability_fname(in_name);
    let (divided_file_num, file_size, last_file_size, precision) =
        decoder_lut.read_lut(&prob_fname);

    let start = Instant::now();

    // Decoding
    go_threads::decode(
        thread_num,
        divided_file_num,
        file_size,
        last_file_size,
        precision,
        &decoder_lut,
        in_name,
        out_name,
    );
    let end = start.elapsed();
    println!("decoding time {:?}", end);
}

fn run_randomacess(infile: String, outfile: String, parameter_file: &str) {
    let parameter = Parameters::read(parameter_file);
    let thread_num = parameter.thread_num;
    let in_name = string_to_static_str(infile);
    let out_name = string_to_static_str(outfile);
    let mut decoder_lut = LookUpTable::new(QVALUE);
    let prob_fname = file_management::get_probability_fname(in_name);
    let (divided_file_num, file_size, last_file_size, precision) =
        decoder_lut.read_lut(&prob_fname);

    let mut first_line = parameter.first_line;
    let mut last_line = parameter.last_line;
    let first_index = first_line / file_size;
    let last_index;
    if last_line > (divided_file_num - 1) * file_size + last_file_size {
        last_line = last_file_size;
        last_index = divided_file_num;
    } else {
        last_index = last_line / file_size;
        last_line = last_line % file_size;
    }
    first_line = first_line % file_size;

    let start = Instant::now();
    // Decoding
    go_threads::random_access(
        thread_num,
        first_index,
        last_index,
        first_line,
        last_line,
        file_size,
        precision,
        &decoder_lut,
        in_name,
        out_name,
    );
    let end = start.elapsed();
    println!(
        "{} to {} random access time {:?}",
        parameter.first_line, parameter.last_line, end
    );

    if first_index != last_index {
        file_management::merge_randfile(out_name, out_name, first_index, last_index);
    } else {
        let from = format!("{}{:03}.dec", out_name, first_index + 1);
        let to = format!("{}.dec", out_name);
        fs::rename(from, to).unwrap();
    }
}

struct Options {
    input: Option<String>,         // input  file
    output: Option<String>,        // output file
    parameter: Option<String>,     // parameter json file
    compressor_flag: Option<bool>, // flag to run quality score compression or decompression
    rand_flag: Option<bool>,
}

impl Options {
    fn from_args() -> Option<Options> {
        let mut options = Options {
            input: None,
            output: None,
            parameter: None,
            compressor_flag: Some(true),
            rand_flag: Some(false),
        };

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "-c" => {
                    options.compressor_flag = Some(true);
                    if let Some(val) = args.next() {
                        options.input = Some(val);
                        if let Some(val) = args.next() {
                            options.output = Some(val);
                            if let Some(val) = args.next() {
                                options.parameter = Some(val);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                "-d" => {
                    options.compressor_flag = Some(false);
                    if let Some(val) = args.next() {
                        options.input = Some(val);
                        if let Some(val) = args.next() {
                            options.output = Some(val);
                            if let Some(val) = args.next() {
                                options.parameter = Some(val);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                "-r" => {
                    options.compressor_flag = Some(false);
                    options.rand_flag = Some(true);
                    if let Some(val) = args.next() {
                        options.input = Some(val);
                        if let Some(val) = args.next() {
                            options.output = Some(val);
                            if let Some(val) = args.next() {
                                options.parameter = Some(val);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                "-h" => {
                    exit_with_error!(
                        1,
                        "Usage: \n
                    cargo.exe [Cargo OPTIONS] [MAIN OPTIONS] [INPUT FILE |OUTPUT FILE NAME| PARAMETER FILE] \n\n
                    MAINT OPTIONS: [-c | -d | -h]
                            -c : run compressor[default] \n
                            -d : run decompressor\n
                            -h : help
                            
                            Example:\n 
                            1.run compressor in release mode \n
                            cargo run --release -- -c input.fastq outputname parameter.json\n
                            
                            2.run decompressor in release mod \n
                            cargo run --release -- -d inputname outputname parameter.json

                            when compressing, precision, size of subfile, and number of thread are required for parameter.serde_json.
                            when decompressiong, only number of thread is required.
                            "
                    );
                }
                _ => {
                    return None;
                }
            }
        }

        if let None = options.input {
            None
        } else {
            if let None = options.output {
                None
            } else {
                if let None = options.parameter {
                    None
                } else {
                    Some(options)
                }
            }
        }
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
