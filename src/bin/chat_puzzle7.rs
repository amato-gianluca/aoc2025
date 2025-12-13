use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Build grid, ignoring completely empty lines
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim_end().chars().collect())
        .collect();

    if grid.is_empty() {
        return;
    }

    let h = grid.len();
    let w = grid[0].len();

    // Sanity: ensure all rows have the same width
    for row in &grid {
        assert_eq!(row.len(), w, "All rows must have the same width");
    }

    // Find starting position 'S'
    let (start_r, start_c) = {
        let mut sr = None;
        for (r, row) in grid.iter().enumerate() {
            if let Some(c) = row.iter().position(|&ch| ch == 'S') {
                sr = Some((r, c));
                break;
            }
        }
        sr.expect("No starting position 'S' found in input")
    };

    // ---------------- Part 1: Classical beams ----------------
    //
    // We track a *set* of beam columns for each row, so overlapping
    // beams are treated as a single classical beam.
    let mut part1_splits: u64 = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_c);

    for r in (start_r + 1)..h {
        if beams.is_empty() {
            break;
        }
        let mut next_beams: HashSet<usize> = HashSet::new();
        let row = &grid[r];

        for &c in beams.iter() {
            if c >= w {
                continue;
            }
            let cell = row[c];
            if cell == '^' {
                // One split event
                part1_splits += 1;
                if c > 0 {
                    next_beams.insert(c - 1);
                }
                if c + 1 < w {
                    next_beams.insert(c + 1);
                }
            } else {
                // Beam continues straight down
                next_beams.insert(c);
            }
        }

        beams = next_beams;
    }

    // ---------------- Part 2: Quantum many-worlds ----------------
    //
    // Now we track *counts* of beams per column (timelines).
    // arr[c] = number of timelines where the particle is just above row r at column c.
    let mut arr = vec![0u128; w];
    arr[start_c] = 1;

    for r in (start_r + 1)..h {
        let mut arr_next = vec![0u128; w];
        let row = &grid[r];

        for c in 0..w {
            let v = arr[c];
            if v == 0 {
                continue;
            }
            let cell = row[c];
            if cell == '^' {
                // Each incoming timeline at this splitter branches into left and right
                if c > 0 {
                    arr_next[c - 1] += v;
                }
                if c + 1 < w {
                    arr_next[c + 1] += v;
                }
            } else {
                // Timeline continues straight down
                arr_next[c] += v;
            }
        }

        arr = arr_next;
    }

    let part2_timelines: u128 = arr.iter().sum();

    // Output answers
    println!("{}", part1_splits);
    println!("{}", part2_timelines);
}
