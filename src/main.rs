//! pi-digits binary crate.

use pi_digits::{parse_args, pi_digits, encode, decode, PI_FILE_PATH, ENCODED_FILE_PATH, DECODED_FILE_PATH};
use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    match parse_args(&args) {
        Err(_msg) => {
            return ExitCode::FAILURE;
        }
        Ok(Some((from, len))) => {
            println!("Calculating the digits of PI...");
            let pi_digits = pi_digits(from, len);
            fs::write(PI_FILE_PATH, &pi_digits).expect("Unable to write file");
            println!("Digits of PI, from: {from}, length: {len} : {pi_digits}");
            println!("Result written to file: {PI_FILE_PATH}");
        }
        Ok(None) => {
            println!("Encoding precalculated digits of PI.");
            println!("The first 295,000 digits of PI shall be in the file: {PI_FILE_PATH}");
            let pi_digits = fs::read_to_string(PI_FILE_PATH).expect("Unable to read file");
            let encoded = encode(&pi_digits);
            fs::write(ENCODED_FILE_PATH, &encoded).expect("Unable to write file");
            println!("Result written to file: {ENCODED_FILE_PATH}");
            let encoded_digits = fs::read_to_string(ENCODED_FILE_PATH).expect("Unable to read file");
            let decoded_digits = decode(&encoded_digits);
            fs::write(DECODED_FILE_PATH, &decoded_digits).expect("Unable to write file");
            println!("Decoding back, result written to file: {DECODED_FILE_PATH}");
            if decoded_digits == pi_digits {
                println!("OK: Result of encode + decoded matches with the original");
            } else {
                println!("ERROR: Result of encode + decoded does not match with the original");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}
