//! pi-digits binary crate.

use pi_digits::{parse_args, pi_digits};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("Calculating the digits of PI...");
    let args = env::args().collect::<Vec<_>>();
    let mut from = 0;
    let mut len = 10;
    match parse_args(&args) {
        Err(_msg) => {
            return ExitCode::FAILURE;
        }
        Ok(Some((afrom, alen))) => {
            from = afrom;
            len = alen;
        }
        Ok(None) => {}
    }
    let ans = pi_digits(from, len);
    println!("Digits of PI, from: {}, length: {} : {}", from, len, ans);
    ExitCode::SUCCESS
}
