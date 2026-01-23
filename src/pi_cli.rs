//! Command line argument parsing and execution.

use crate::pi_encoded::cli_lookup_encoded;
use crate::pi_generate::{
    MAX_DIGITS, cli_generate_encoded, cli_generate_hardcoded, cli_generate_pi,
};
use crate::pi_hardcoded::cli_lookup_hardcoded;
use std::env;

#[derive(Debug, PartialEq)]
pub enum CLIArgs {
    Help,
    GeneratePi { from: usize, len: usize },
    GenerateEncoded,
    GenerateHardcoded,
    LookupEncoded { from: usize, len: usize },
    LookupHardcoded { from: usize, len: usize },
}

pub fn print_help() {
    println!(
        "\
        Usage:\n\
        \t pi_digits.exe --generate         [from] [length]     generate digits of Pi to file\n\
        \t pi_digits.exe --encode                               encode Pi digits to a compressed unicode string\n\
        \t pi_digits.exe --hardcode                             generate hardcoded lookup values for Pi slices\n\
        \t pi_digits.exe --lookup-encoded   [from] [length]     lookup Pi from encoded string\n\
        \t pi_digits.exe --lookup-hardcoded [from] [length]     lookup Pi from hardcoded slices\n\
        \n\
        Constraints:\n\
        \t from >= 0, length >= 1, from + length <= 1,000,000\n\
        "
    );
}

/// Tries to parse CLI arguments to from and len
pub fn parse_args(args: &[String]) -> Result<CLIArgs, &'static str> {
    match args.len() {
        1 => Ok(CLIArgs::Help),
        2 => {
            let command = args[1].to_ascii_lowercase();
            if command == "--encode" {
                Ok(CLIArgs::GenerateEncoded)
            } else if command == "--hardcode" {
                Ok(CLIArgs::GenerateHardcoded)
            } else {
                Err("Invalid command")
            }
        }
        4 => {
            let from = args[2]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: `from` must be a non-negative integer")?;
            let len = args[3]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: `length` must be a non-negative integer")?;
            if from + len > MAX_DIGITS {
                return Err("Invalid argument: `from` + `len` must be below 1,000,000");
            }
            let command = args[1].to_ascii_lowercase();
            if command == "--generate" {
                Ok(CLIArgs::GeneratePi { from, len })
            } else if command == "--lookup-encoded" {
                Ok(CLIArgs::LookupEncoded { from, len })
            } else if command == "--lookup-hardcoded" {
                Ok(CLIArgs::LookupHardcoded { from, len })
            } else {
                Err("Invalid command")
            }
        }
        _ => Err("Invalid number of arguments"),
    }
}

pub fn run() {
    println!("A-Million-Digits-of-Pi : Pi digits generator\n");
    let args = env::args().collect::<Vec<_>>();
    match parse_args(&args) {
        Err(msg) => {
            print_help();
            println!("\n[ERROR] {msg}");
        }
        Ok(CLIArgs::Help) => print_help(),
        Ok(CLIArgs::GeneratePi { from, len }) => cli_generate_pi(from, len),
        Ok(CLIArgs::GenerateEncoded) => cli_generate_encoded(),
        Ok(CLIArgs::GenerateHardcoded) => cli_generate_hardcoded(),
        Ok(CLIArgs::LookupEncoded { from, len }) => cli_lookup_encoded(from, len),
        Ok(CLIArgs::LookupHardcoded { from, len }) => cli_lookup_hardcoded(from, len),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_args_invalid_arguments() {
        let args = [
            String::from("pi_digits"),
            String::from("--generate"),
            String::from("0"),
        ];
        assert_eq!(parse_args(&args), Err("Invalid number of arguments"));

        let args = [
            String::from("pi_digits"),
            String::from("--generate"),
            String::from("a"),
            String::from("10"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: `from` must be a non-negative integer")
        );

        let args = [
            String::from("pi_digits"),
            String::from("--generate"),
            String::from("0"),
            String::from("a"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: `length` must be a non-negative integer")
        );

        let args = [
            String::from("pi_digits"),
            String::from("--generate"),
            String::from("900000"),
            String::from("100001"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: `from` + `len` must be below 1,000,000")
        );

        let args = [
            String::from("pi_digits"),
            String::from("--unknown-command"),
            String::from("0"),
            String::from("10"),
        ];
        assert_eq!(parse_args(&args), Err("Invalid command"));
    }

    #[test]
    fn parse_args_valid_arguments() {
        let args = [String::from("pi_digits")];
        assert_eq!(parse_args(&args), Ok(CLIArgs::Help));

        let args = [
            String::from("pi_digits"),
            String::from("--generate"),
            String::from("0"),
            String::from("10"),
        ];
        assert_eq!(
            parse_args(&args),
            Ok(CLIArgs::GeneratePi { from: 0, len: 10 })
        );

        let args = [String::from("pi_digits"), String::from("--encode")];
        assert_eq!(parse_args(&args), Ok(CLIArgs::GenerateEncoded));

        let args = [String::from("pi_digits"), String::from("--hardcode")];
        assert_eq!(parse_args(&args), Ok(CLIArgs::GenerateHardcoded));

        let args = [
            String::from("pi_digits"),
            String::from("--lookup-encoded"),
            String::from("0"),
            String::from("10"),
        ];
        assert_eq!(
            parse_args(&args),
            Ok(CLIArgs::LookupEncoded { from: 0, len: 10 })
        );

        let args = [
            String::from("pi_digits"),
            String::from("--lookup-hardcoded"),
            String::from("0"),
            String::from("10"),
        ];
        assert_eq!(
            parse_args(&args),
            Ok(CLIArgs::LookupHardcoded { from: 0, len: 10 })
        );
    }
}
