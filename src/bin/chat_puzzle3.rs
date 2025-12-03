use std::io::{self, Read};

fn max_two_digit(digits: &[u8]) -> u64 {
    let n = digits.len();
    if n < 2 {
        return 0;
    }

    // Suffix maximum digit from each position to the end.
    let mut suf_max = vec![0u8; n];
    let mut current_max = 0u8;
    for i in (0..n).rev() {
        if digits[i] > current_max {
            current_max = digits[i];
        }
        suf_max[i] = current_max;
    }

    // For each possible first digit, pair it with the best possible second digit to its right.
    let mut best = 0u64;
    for i in 0..n - 1 {
        let first = digits[i] as u64;
        let second = suf_max[i + 1] as u64;
        let value = first * 10 + second;
        if value > best {
            best = value;
        }
    }

    best
}

fn max_k_digit(digits: &[u8], k: usize) -> u64 {
    let n = digits.len();
    assert!(k <= n);

    let mut result: u64 = 0;
    let mut start = 0usize;
    let mut remaining = k;

    // Greedily build the lexicographically-largest subsequence of length k.
    while remaining > 0 {
        let end = n - remaining; // inclusive upper bound for this choice
        let mut best_digit = 0u8;
        let mut best_pos = start;

        for i in start..=end {
            let d = digits[i];
            if d > best_digit {
                best_digit = d;
                best_pos = i;
                if d == 9 {
                    // Can't do better than 9 within this window.
                    break;
                }
            }
        }

        result = result * 10 + best_digit as u64;
        start = best_pos + 1;
        remaining -= 1;
    }

    result
}

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    const K: usize = 12;

    for line in input.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        // Convert characters '1'..'9' into digits.
        let digits: Vec<u8> = s
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();

        if digits.len() < 2 {
            continue; // Problem input should not do this, but be defensive.
        }

        // Part 1: best two-digit number from this bank.
        part1 += max_two_digit(&digits);

        // Part 2: best 12-digit number from this bank.
        // The puzzle guarantees there are enough digits; if not, fall back to using all of them.
        if digits.len() >= K {
            part2 += max_k_digit(&digits, K);
        } else {
            part2 += max_k_digit(&digits, digits.len());
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
