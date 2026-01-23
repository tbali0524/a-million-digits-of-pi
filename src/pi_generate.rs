//! Generating digits of Pi in cleartext file, encoded file or hardcoded slices.

#[cfg(target_os = "windows")]
use crate::pi_num_bigint::pi_digits;
#[cfg(not(target_os = "windows"))]
use crate::pi_rug::pi_digits;

use crate::pi_encoded::{BASE_CP, decode};
use crate::pi_hardcoded::PI_HARDCODED;
use std::fs;

pub const MAX_DIGITS: usize = 1_000_000;
pub const MAX_DIGITS_TO_ENCODE: usize = 295_000;
pub const PI_FILE_PATH: &str = "./result/pi.txt";
pub const ENCODED_FILE_PATH: &str = "./result/encoded_295k.txt";
pub const DECODED_FILE_PATH: &str = "./result/decoded_295k.txt";
pub const HARDCODED_FILE_PATH: &str = "./result/hardcoded.txt";

pub fn cli_generate_pi(from: usize, len: usize) {
    println!("Calculating the digits of Pi...");
    let pi_digits = pi_digits(from, len);
    fs::write(PI_FILE_PATH, &pi_digits).expect("Unable to write file");
    if len <= 100 {
        println!("Digits of Pi, from: {from}, length: {len}");
        println!("{pi_digits}");
    }
    println!("Result written to file: {PI_FILE_PATH}");
}

pub fn cli_generate_encoded() {
    println!("Encoding digits of Pi as compressed Unicode string...");
    println!("The precalculated digits of Pi shall be in the file: {PI_FILE_PATH}");
    let mut pi_digits = fs::read_to_string(PI_FILE_PATH).expect("Unable to read file");
    if pi_digits.len() < MAX_DIGITS_TO_ENCODE {
        println!("Error: missing some precalculated digits");
    }
    pi_digits.truncate(MAX_DIGITS_TO_ENCODE);
    let encoded = encode(&pi_digits);
    fs::write(ENCODED_FILE_PATH, &encoded).expect("Unable to write file");
    println!("Result written to file: {ENCODED_FILE_PATH}");
    let encoded_digits = fs::read_to_string(ENCODED_FILE_PATH).expect("Unable to read file");
    let decoded_digits = decode(&encoded_digits);
    fs::write(DECODED_FILE_PATH, &decoded_digits).expect("Unable to write file");
    println!("Decoding back, result written to file: {DECODED_FILE_PATH}");
    if decoded_digits == pi_digits {
        println!("OK: Result of encode + decode operations matches the original");
    } else {
        println!("ERROR: Result of encode + decode operations does not match the original");
    }
}

pub fn cli_generate_hardcoded() {
    println!("Creating hardcoded slices from precalculated digits of Pi...");
    println!("The precalculated digits of Pi shall be in the file: {PI_FILE_PATH}");
    let pi_digits = fs::read_to_string(PI_FILE_PATH).expect("Unable to read file");
    if pi_digits.len() < MAX_DIGITS {
        println!("Error: missing some precalculated digits");
        return;
    }
    let mut content = String::new();
    content.push_str("// Rust:\n");
    content.push_str(&format!(
        "pub static PI_HARDCODED: [(usize, usize, &str); {}] = [\n",
        PI_HARDCODED.len()
    ));
    for &(from, len, _) in &PI_HARDCODED {
        content.push_str(&format!(
            "    ({}, {}, \"{}\"),\n",
            from,
            len,
            &pi_digits[from..from + len]
        ));
    }
    content.push_str("];\n");
    content.push_str("\n\n// PHP:\n");
    content.push_str("const PI_HARDCODED = [\n");
    for &(from, len, _) in &PI_HARDCODED {
        content.push_str(&format!(
            "    [{}, {}, '{}'],\n",
            from,
            len,
            &pi_digits[from..from + len]
        ));
    }
    content.push_str("];\n");
    fs::write(HARDCODED_FILE_PATH, &content).expect("Unable to write file");
    println!("Result written to file: {HARDCODED_FILE_PATH}");
}

// -- Unicode sections overview:
// U+0000 to U+007F     : [      128 code points] : 1 byte in UTF-8, 2 bytes in UTF-16
// U+0080 to U+07FF     : [    1,920 code points] : 2 bytes in UTF-8, 2 bytes in UTF-16
// U+0800 to U+D7FF     : [   53,248 code points] : 3 bytes in UTF-8, 2 bytes in UTF-16
// U+E000 to U+FFFF     : [    8,192 code points] : 3 bytes in UTF-8, 2 bytes in UTF-16
// U+010000 to U+10FFFF : [1,048,576 code points] : 4 bytes in UTF-8, 4 bytes in UTF-16
// Total valid Unicode  : [1,112,064 code points]
// U+D800 to U+DFFF     : [    2,048 code points] : surrogates (invalid code points)

/// Encodes each 4 decimal digits to a single Unicode char (from U+5000)
pub fn encode(pi_digits: &str) -> String {
    if !pi_digits.len().is_multiple_of(4) {
        panic!("No padding implemented");
    }
    let mut ans = String::new();
    let mut chunk = 0;
    let mut counter = 0;
    for pi_digit in pi_digits.chars() {
        if let Some(d) = pi_digit.to_digit(10) {
            chunk = chunk * 10 + d;
            counter += 1;
            if counter < 4 {
                continue;
            }
            let c = char::from_u32(BASE_CP + chunk).unwrap();
            ans.push(c);
            counter = 0;
            chunk = 0;
        }
    }
    ans
}
