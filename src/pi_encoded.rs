//! Looking up digits of Pi from precalculated, Unicode-encoded static string.

use std::io;

pub const BASE_CP: u32 = 0x5000;

/// Solution code for Codingame puzzle [Squash Pi](https://www.codingame.com/training/easy/squash-pi)
#[expect(dead_code)]
fn squash_pi() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let from = line.trim().parse::<usize>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let len = line.trim().parse::<usize>().unwrap();
    cli_lookup_encoded(from, len);
}

pub fn cli_lookup_encoded(from: usize, len: usize) {
    let pi_digits = decode(PI_ENCODED);
    let ans = pi_digits.chars().skip(from).take(len).collect::<String>();
    println!("{ans}");
    if pi_digits.len() < from + len {
        println!("Error: missing some precalculated, encoded digits");
    }
}

/// Decodes string, where a single Unicode char (from U+5000 to U+5000 + 9999) encodes 4 decimal digits
pub fn decode(encoded: &str) -> String {
    let mut ans = String::new();
    for c in encoded.chars() {
        let chunk = c as u32;
        if !(BASE_CP..=BASE_CP + 9999).contains(&chunk) {
            panic!("Invalid char");
        }
        let chunk = chunk - BASE_CP;
        let mut div = 1_000;
        for _ in 0..4 {
            let d = (chunk / div) % 10;
            ans.push(char::from_digit(d, 10).unwrap());
            div /= 10;
        }
    }
    ans
}

// replace with the result of an encoding run
#[rustfmt::skip]
pub static PI_ENCODED: &str = "居朦擮癁奐桸崷嫫儠恥";
