use std::io::{self, Read};

fn pow10(exp: usize) -> u64 {
    let mut p = 1u64;
    for _ in 0..exp {
        p *= 10;
    }
    p
}

// Part 1: numbers that are some digit string repeated exactly twice.
fn gen_twice(max_val: u64) -> Vec<u64> {
    let max_digits = max_val.to_string().len();
    let mut res = Vec::new();

    for m in 1..=max_digits / 2 {
        let pow10_m = pow10(m);
        let start = pow10(m - 1);
        let end = pow10(m) - 1;
        let mut base = start;
        while base <= end {
            let n = base * pow10_m + base;
            if n > max_val {
                break;
            }
            res.push(n);
            base += 1;
        }
    }

    res.sort_unstable();
    res
}

// Part 2: numbers that are some digit string repeated k >= 2 times.
fn gen_repeat(max_val: u64) -> Vec<u64> {
    let max_digits = max_val.to_string().len();
    let mut v = Vec::new();

    for len_digits in 2..=max_digits {
        for d in 1..=len_digits / 2 {
            if len_digits % d != 0 {
                continue;
            }
            let repeat = len_digits / d;
            if repeat < 2 {
                continue;
            }

            let pow10_d = pow10(d);
            let start = pow10(d - 1);
            let end = pow10(d) - 1;

            let mut base = start;
            while base <= end {
                // build number = base repeated `repeat` times
                let mut n = 0u64;
                for _ in 0..repeat {
                    n = n * pow10_d + base;
                }
                if n > max_val {
                    break;
                }
                v.push(n);
                base += 1;
            }
        }
    }

    v.sort_unstable();
    v.dedup();
    v
}

fn lower_bound(v: &[u64], x: u64) -> usize {
    let mut l = 0usize;
    let mut r = v.len();
    while l < r {
        let m = (l + r) / 2;
        if v[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn upper_bound(v: &[u64], x: u64) -> usize {
    let mut l = 0usize;
    let mut r = v.len();
    while l < r {
        let m = (l + r) / 2;
        if v[m] <= x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn sum_over_ranges(cands: &[u64], ranges: &[(u64, u64)]) -> u128 {
    let mut total: u128 = 0;
    for &(lo, hi) in ranges {
        let l = lower_bound(cands, lo);
        let r = upper_bound(cands, hi);
        for i in l..r {
            total += cands[i] as u128;
        }
    }
    total
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse ranges: tokens like "a-b" separated by commas/whitespace/newlines
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for part in input.split(|c: char| c == ',' || c.is_whitespace()) {
        if part.is_empty() {
            continue;
        }
        let mut it = part.split('-');
        let a: u64 = it.next().unwrap().parse().unwrap();
        let b: u64 = it.next().unwrap().parse().unwrap();
        ranges.push((a, b));
    }

    if ranges.is_empty() {
        return;
    }

    let mut max_val = ranges[0].1;
    for &(_, b) in &ranges {
        if b > max_val {
            max_val = b;
        }
    }

    let invalid_twice = gen_twice(max_val);
    let invalid_repeat = gen_repeat(max_val);

    let sum1 = sum_over_ranges(&invalid_twice, &ranges);
    let sum2 = sum_over_ranges(&invalid_repeat, &ranges);

    println!("{}", sum1);
    println!("{}", sum2);
}
