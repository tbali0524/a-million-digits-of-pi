//! pi_digits library crate.

use num_bigint::BigInt;
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
