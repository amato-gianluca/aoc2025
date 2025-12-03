use std::io::{self, Read};

fn zeros_hit_during_rotation(start: i64, dist: i64, dir: char) -> i64 {
    if dist <= 0 {
        return 0;
    }

    // We want to count k in [1, dist] such that the position after k clicks is 0.
    // Dial positions are modulo 100.
    // For R: position_k = start + k (mod 100) == 0  ->  k ≡ (100 - start) (mod 100)
    // For L: position_k = start - k (mod 100) == 0  ->  k ≡ start (mod 100)
    let base = match dir {
        'R' => (100 - start).rem_euclid(100),
        'L' => start.rem_euclid(100),
        _ => panic!("Invalid direction"),
    };

    // Smallest positive k that satisfies k ≡ base (mod 100):
    // if base == 0, then k = 100; otherwise k = base.
    let first = if base == 0 { 100 } else { base };

    if dist < first {
        0
    } else {
        1 + (dist - first) / 100
    }
}

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut pos: i64 = 50; // starting position
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let dir = chars.next().unwrap(); // 'L' or 'R'
        let dist_str: String = chars.collect();
        let dist: i64 = dist_str.parse().expect("Invalid distance");

        // Part 2: count every click that lands on 0 during this rotation
        part2 += zeros_hit_during_rotation(pos, dist, dir);

        // Update position for both parts
        match dir {
            'R' => {
                pos = (pos + dist).rem_euclid(100);
            }
            'L' => {
                pos = (pos - dist).rem_euclid(100);
            }
            _ => panic!("Invalid direction in input"),
        }

        // Part 1: count if we end this rotation at 0
        if pos == 0 {
            part1 += 1;
        }
    }

    // Print answers, one per line
    println!("{}", part1);
    println!("{}", part2);
}
