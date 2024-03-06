use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use decode::decode;
use encode::encode;
use analyze::analyze;

use crate::lzss_tuple::LzssTuple;

mod lzss_tuple;
mod encode;
mod decode;
mod analyze;

fn read_bytes_into_buffer(file: &mut File, buffer: &mut [u8]) -> io::Result<usize> {
    file.read(buffer)
}

fn run_encoder_and_write(input_file_name: &String, output_file_name: &String, buffer_size: usize, w: i32, n: i32) -> io::Result<()> {
    // Open the file
    let mut input = File::open(input_file_name)?;

    let mut output_file = File::create(output_file_name)?;

    // Create a buffer to hold each chunk of bytes
    let mut buffer = vec![0; buffer_size];

    loop {
        let bytes_read = read_bytes_into_buffer(&mut input, &mut buffer)?;
        // If no bytes were read, we've reached the end of the file
        if bytes_read == 0 {
            break;
        }

        let codes = encode(&mut buffer, bytes_read, w, n);

        // Write each tuple to the output file
        for code in &codes {
            match code {
                LzssTuple::NoPrefix(_, ch) => {
                    write!(output_file, "{}", ch)?;
                }
                LzssTuple::Prefix(num1, num2) => {
                    write!(output_file, "|{}~{}|", num1, num2)?;
                }
            }
        }
    }

    Ok(())
}

fn run_decoder_and_write(input_file_name: &String, output_file_name: &String) -> io::Result<()> {

    // Open the file
    let input_file = File::open(input_file_name)?;

    let mut output_file = File::create(output_file_name)?;

    let mut reader = BufReader::new(input_file);

    // Initialize a vector to store parsed tuples
    let mut codes: Vec<LzssTuple> = Vec::new();

    let mut num = String::new();
    let mut num2 = String::new();
    let mut read_first_number: bool = false;
    let mut reading_tuple = false;
    let mut buffer = [0; 1];

    // Read the file byte by byte
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            // End of file reached
            break;
        }

        let byte = buffer[0];

        if byte == b'~' {
            // If encountering a ~, switch to reading the second number
            read_first_number = true;
        } else if byte == b'|' {
            if !reading_tuple {
                reading_tuple = true;
            } else {
                // If encountering the second pipe separator, construct LzssTuple and reset variables
                let d = num.parse::<i32>().expect("Invalid number");
            
                let lcp = num2.parse::<i32>().expect("Invalid number");
                codes.push(LzssTuple::Prefix(d, lcp));

                read_first_number = false;
                reading_tuple = false;
                num.clear();
                num2.clear();
            }

        } else if byte.is_ascii() && reading_tuple {
            // Reading a tuple

            if !read_first_number {
                num.push(byte as char);
            } else {
                num2.push(byte as char);
            }
        } else if byte.is_ascii() && !reading_tuple {
            // Reading a character

            num2.push(byte as char);
            let ch = num2.parse::<char>().expect("Invalid number");
            codes.push(LzssTuple::NoPrefix(0, ch));
            num2.clear();
        } else {
            // If encountering an unexpected byte, it's invalid input
            eprintln!("Invalid input: unexpected byte");
            break;
        }
    }

    let result = decode(codes);

    write!(output_file, "{}", result)?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // Extract the file name from the arguments
    let input_file_name = &args[1];
    let encoded_file_name = "encoded.bin".into();
    let decoded_file_name = "decoded.bin".into();

    let w = 20;
    let n = 40;

    // Define the buffer size (number of bytes to read at a time)
    let buffer_size = 80; // Adjust this according to your needs

    let encode_result = run_encoder_and_write(input_file_name, &encoded_file_name, buffer_size, w, n);

    if encode_result.is_err() {
        return encode_result;
    }

    let decode_result = run_decoder_and_write(&encoded_file_name, &decoded_file_name);

    if decode_result.is_err() {
        return decode_result;
    }
    analyze()
}