//! pi_digits library crate.

use num_bigint::BigInt;
use std::io;
use std::time;

// 0, 10, 3141592653
// 1243, 10, 7678374494
// 79100, 20, 32118636062252701154
// 79552, 20, 37462746172746265824
// 999485, 50, 02833119371611408747270676255856777511995666748615 : ? sec

pub fn binary_split(a: i64, b: i64) -> (BigInt, BigInt, BigInt) {
    let p_ab;
    let q_ab;
    let r_ab;
    if b == a + 1 {
        if a == 0 {
            p_ab = BigInt::from(1);
            q_ab = BigInt::from(1);
        } else {
            p_ab = BigInt::from(6 * a - 5) * BigInt::from(2 * a - 1) * BigInt::from(6 * a - 1);
            q_ab = BigInt::from(10939058860032000i64) * BigInt::from(a * a * a);
        }
        r_ab = &p_ab * BigInt::from(545140134 * a + 13591409) * (1 - 2 * (a & 1));
    } else {
        let m = (a + b) / 2;
        let (p_am, q_am, r_am) = binary_split(a, m);
        let (p_mb, q_mb, r_mb) = binary_split(m, b);
        p_ab = &p_am * &p_mb;
        q_ab = &q_am * &q_mb;
        r_ab = &q_mb * &r_am + &p_am * &r_mb;
    }
    (p_ab, q_ab, r_ab)
}

pub fn chudnovsky(_n: i64) -> BigInt {
    let now = time::Instant::now();
    let (_p_1n, q_1n, r_1n) = binary_split(0, 70518);
    let elapsed = now.elapsed();
    println!("=== Part 1 time spent: {:7} ms", elapsed.as_millis());

    let now = time::Instant::now();
    let z = BigInt::from(100).pow(1000050) * BigInt::from(10005);
    let elapsed = now.elapsed();
    println!("=== Part 2 time spent: {:7} ms", elapsed.as_millis());

    let now = time::Instant::now();
    let pi = (z.sqrt() * BigInt::from(426880) * &q_1n) / r_1n;
    let elapsed = now.elapsed();
    println!("=== Part 3 time spent: {:7} ms", elapsed.as_millis());
    pi
}

/*
{
    [$P1n, $Q1n, $R1n] = binary_split(1, $n);
    return bcdiv(
        bcmul(bcmul('426880', bcsqrt('10005', $n)), $Q1n),
        bcadd(bcmul('13591409', $Q1n), $R1n),
        $n
    ) ?: '0';
}
 */
pub fn pi_digits(from: usize, len: usize) -> String {
    if from + len > 1_000_000 {
        panic!("Works only for first 1 million digits.");
    }
    let pi = chudnovsky((from + len) as i64);
    let digits = pi
        .to_str_radix(10)
        .chars()
        .skip(from)
        .take(len)
        .collect::<String>();
    digits
}

/// Tries to parse CLI arguments to from and len
pub fn parse_args(args: &[String]) -> Result<Option<(usize, usize)>, &'static str> {
    match args.len() {
        1 => Ok(None),
        3 => {
            let from = args[1]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: from must be integer")?;
            let len = args[2]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: length must be integer")?;
            Ok(Some((from, len)))
        }
        _ => Err("Invalid arguments"),
    }
}

pub const PI_FILE_PATH: &str = "./result/pi.txt";
// pub const PI_FILE_PATH: &str = "./result/pi_295k.txt";
pub const ENCODED_FILE_PATH: &str = "./result/encoded_295k.txt";
pub const DECODED_FILE_PATH: &str = "./result/decoded_295k.txt";
pub const MAX_PI_DIGITS: usize = 295_000;
pub const BASE_CP: u32 = 0x5000;

// U+0000 to U+007F     : [      128 code points] : 1 byte in UTF-8, 2 bytes in UTF-16
// U+0080 to U+07FF     : [    1,920 code points] : 2 bytes in UTF-8, 2 bytes in UTF-16
// U+0800 to U+D7FF     : [   53,248 code points] : 3 bytes in UTF-8, 2 bytes in UTF-16
// U+E000 to U+FFFF     : [    8,192 code points] : 3 bytes in UTF-8, 2 bytes in UTF-16
// U+010000 to U+10FFFF : [1,048,576 code points] : 4 bytes in UTF-8, 4 bytes in UTF-16
// Total valid Unicode  : [1,112,064 code points]
// U+D800 to U+DFFF     : [    2,048 code points] : surrogates (invalid code points)

/// Encodes each 4 decimal digits to a single Unicode char (from U+5000)
pub fn encode(pi_digits: &str) -> String {
    if pi_digits.len() % 4 != 0 {
        panic!("No padding implemented");
    }
    let mut ans = String::new();
    let mut chunk = 0;
    let mut counter  = 0;
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

/// Decodes a single Unicode char (from U+5000 to U+5000 + 9999) to 4 decimal digits
pub fn decode(encoded: &str) -> String {
    let mut ans = String::new();
    for c in encoded.chars() {
        let chunk = c as u32;
        if chunk < BASE_CP || chunk > BASE_CP + 9999 {
            panic!("Invalid char");
        }
        let chunk = chunk - BASE_CP;
        let mut div = 1_000;
        for _ in 0..4 {
            let d = (chunk / div) % 10;
            ans.push(char::from_digit(d, 10).unwrap());
            div = div / 10;
        }
    }
    ans
}

pub fn squash_pi() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let index = line.trim().parse::<usize>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<usize>().unwrap();
    let pi_digits = decode(PI_ENCODED);
    println!("{}", pi_digits.chars().skip(index).take(n).collect::<String>());
}

static PI_ENCODED: &str = "";

// --------------------------------------------------------------------
// crate 'rug' not supported for Windows + MSVC target
//
// use rug::{Assign, Integer};
//
// pub fn binary_split(a: i64, b: i64) -> (Integer, Integer, Integer) {
//     let mut Pab = Integer::new();
//     let mut Qab = Integer::new();
//     let mut Rab = Integer::new();
//     if b == a + 1 {
//         Pab.assign(Integer::from(-(6 * a - 5)) * Integer::from(2 * a - 1) * Integer::from(6 * a - 1));
//         Qab.assign(Integer::from(10939058860032000) * Ingeter::from(a * a * a));
//         Rab.assign(Pab * Ingeter::from(545140134 * a + 13591409));
//     } else {
//         let mut m = (a + b) / 2;
//         let (Pam, Qam, Ram) = binary_split(a, m);
//         let (Pmb, Qmb, Rmb) = binary_split(m, b);
//         Pab.assign(Pam * Pmb);
//         Qab.assign(Qam * Qmb);
//         Rab.assign(Qmb * Ram + Pam * Rmb);
//     }
//     (Pab, Qab, Rab)
// }

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_args_invalid_arguments() {
        let args = [String::from("pi_digits"), String::from("0")];
        assert_eq!(parse_args(&args), Err("Invalid arguments"));
        let args = [
            String::from("pi_digits"),
            String::from("a"),
            String::from("10"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: from must be integer")
        );
        let args = [
            String::from("pi_digits"),
            String::from("0"),
            String::from("a"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: length must be integer")
        );
    }

    #[test]
    fn parse_args_valid_arguments() {
        let args = [String::from("pi_digits")];
        assert_eq!(parse_args(&args), Ok(None));
        let args = [
            String::from("pi_digits"),
            String::from("0"),
            String::from("10"),
        ];
        assert_eq!(parse_args(&args), Ok(Some((0, 10))));
    }
}
