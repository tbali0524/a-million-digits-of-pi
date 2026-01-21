//! Calculating digits of Pi with the Chudnovsky algorithm, using the `num_bigint` crate.

#![cfg(target_os = "windows")]

use num_bigint::BigInt;
use std::time;

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
    println!("=== Using num_bigint");

    let now = time::Instant::now();
    let (_p_1n, q_1n, r_1n) = binary_split(0, 70518);
    println!("=== Part 1 time spent: {:7} ms", now.elapsed().as_millis());
    // println!("r = {:?}", r_1n);

    let now = time::Instant::now();
    let z = BigInt::from(100).pow(1000050) * BigInt::from(10005);
    println!("=== Part 2 time spent: {:7} ms", now.elapsed().as_millis());

    let now = time::Instant::now();
    let pi = (z.sqrt() * BigInt::from(426880) * &q_1n) / r_1n;
    println!("=== Part 3 time spent: {:7} ms", now.elapsed().as_millis());
    pi
}

pub fn pi_digits(from: usize, len: usize) -> String {
    if from + len > 1_000_000 {
        panic!("Works only for first 1 million digits.");
    }
    let pi = chudnovsky((from + len) as i64);
    pi.to_str_radix(10)
        .chars()
        .skip(from)
        .take(len)
        .collect::<String>()
}
