//! Looking up digits of Pi from precalculated, hardcoded slices.

use std::io;

/// Solution code for Codingame puzzle [A Million Digits of Pi](https://www.codingame.com/training/expert/a-million-digits-of-pi)
#[expect(dead_code)]
fn million_digits_of_pi() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let from = line.trim().parse::<usize>().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let len = line.trim().parse::<usize>().unwrap();
    cli_lookup_hardcoded(from, len);
}

pub fn cli_lookup_hardcoded(from: usize, len: usize) {
    let mut ans = String::new();
    for &(slice_from, slice_len, result) in &PI_HARDCODED {
        if (slice_from..slice_from + slice_len).contains(&from)
            && (slice_from..slice_from + slice_len).contains(&(from + len - 1))
        {
            ans = result[from - slice_from..from - slice_from + len].to_string();
            break;
        }
    }
    if ans.is_empty() {
        println!("Error: missing some precalculated, hardcoded digits");
    } else {
        println!("{ans}");
    }
}

// replace with the result of a hardcoding run
#[rustfmt::skip]
pub static PI_HARDCODED: [(usize, usize, &str); 14] = [
    (0, 10, "3141592653"),
    (79100, 20, "32118636062252701154"),
    (163587, 30, "088077855566632397283342252706"),
    (294741, 40, "9037059562142587514293266878284788078762"),
    (555631, 50, "05496374484069145828464261242049616787491900287079"),
    (718341, 50, "34015238428243963913256867292722487084779696809273"),
    (999485, 50, "02833119371611408747270676255856777511995666748615"),
    (1243, 10, "7678374494"),
    (79552, 20, "37462746172746265824"),
    (213155, 30, "348018765356776441151647710438"),
    (292480, 40, "1804744803809438397548267445519775059366"),
    (560849, 50, "85941080031965524777784431018487536837769304812085"),
    (753823, 50, "50664375400366598915712966903738260586886220943158"),
    (998581, 50, "24509992532017874996366404734770389855873065076038"),
];
