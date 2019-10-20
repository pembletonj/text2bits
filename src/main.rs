
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::OpenOptions;



fn main() {
    
    let mut args = env::args();
    let input: BufReader<File>;
    let mut output: File;

    args.next().unwrap();
    input = BufReader::new(File::open(args.next().expect("Not enough arguments.")).unwrap());
    output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(args.next().expect("Not enough arguments."))
        .unwrap();
    
    if args.next().is_some() {
        panic!("Too many arguments.");
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
