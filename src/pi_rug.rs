//! Calculating digits of Pi with the Chudnovsky algorithm, using the `rug` crate.
//!
//! Note: rug crate does not support Windows MSVC target

#![cfg(not(target_os = "windows"))]

use rug::{Assign, Integer, ops::Pow};
use std::time;

pub fn binary_split(a: i64, b: i64) -> (Integer, Integer, Integer) {
    let mut p_ab = Integer::new();
    let mut q_ab = Integer::new();
    let mut r_ab = Integer::new();
    if b == a + 1 {
        if a == 0 {
            p_ab = Integer::from(1);
            q_ab = Integer::from(1);
        } else {
            p_ab.assign(
                Integer::from(6 * a - 5) * Integer::from(2 * a - 1) * Integer::from(6 * a - 1),
            );
            q_ab.assign(Integer::from(10939058860032000i64) * Integer::from(a * a * a));
        }
        r_ab.assign(p_ab.clone() * Integer::from(545140134 * a + 13591409) * (1 - 2 * (a & 1)));
    } else {
        let m = (a + b) / 2;
        let (p_am, q_am, r_am) = binary_split(a, m);
        let (p_mb, q_mb, r_mb) = binary_split(m, b);
        p_ab.assign(p_am.clone() * p_mb);
        q_ab.assign(q_am * q_mb.clone());
        r_ab.assign(q_mb * r_am + p_am * r_mb);
    }
    (p_ab, q_ab, r_ab)
}

pub fn chudnovsky(_n: i64) -> Integer {
    println!("=== Using rug");

    let now = time::Instant::now();
    let (_p_1n, q_1n, r_1n) = binary_split(0, 70518);
    println!("=== Part 1 time spent: {:7} ms", now.elapsed().as_millis());
    // println!("r = {:?}", r_1n);

    let now = time::Instant::now();
    let z = Integer::from(100).pow(1000050) * Integer::from(10005);
    println!("=== Part 2 time spent: {:7} ms", now.elapsed().as_millis());

    let now = time::Instant::now();
    let pi = (z.sqrt() * Integer::from(426880) * q_1n) / r_1n;
    println!("=== Part 3 time spent: {:7} ms", now.elapsed().as_millis());
    pi
}

pub fn pi_digits(from: usize, len: usize) -> String {
    if from + len > 1_000_000 {
        panic!("Works only for first 1 million digits.");
    }
    let pi = chudnovsky((from + len) as i64);
    pi.to_string_radix(10)
        .chars()
        .skip(from)
        .take(len)
        .collect::<String>()
}
