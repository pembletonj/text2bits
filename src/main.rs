
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::error::Error;



fn main() {
    
    let mut args = env::args();
    let input: BufReader<File>;
    let mut output: File;

    args.next().unwrap();

    let input_arg = match args.next() {

        Some(arg) => arg,
        None => String::from(""),

    };

    if input_arg == "" {
        println!("E: Not enough arguments.");
        return;
    }

    input = BufReader::new(File::open(input_arg).unwrap());



    let output_arg = match args.next() {

        Some(arg) => arg,
        None => String::from(""),

    };

    if output_arg == "" {
        println!("E: Not enough arguments.");
        return;
    }    

    output = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(output_arg) {

            Ok(file) => file,

            Err(e) => {

                println!("E: Could not open output file.\nReason: {}", e.description());
                return;

            }

        };
    
    if args.next().is_some() {
        println!("E: Too many arguments.");
        return;
    }

    let mut bit: u8 = 0;
    let mut byte: u8 = 0;
    let mut comment = false;

    for ch in input.bytes() {

        let ch = ch.unwrap();

        if ch == b'0' && !comment {
            bit += 1;
        }
        else if ch == b'1' && !comment {
            byte += u8::pow(2, 7 - bit as u32);
            bit += 1;
        }
        else if ch == b'\n' {
            comment = false;
        }
        else if ch == b'#' {
            comment = true;
        }

        if bit == 8 {
            // println!("{:08b}", byte);
            output.write(&[byte]).unwrap();
            bit = 0;
            byte = 0;
        }


    }

}
