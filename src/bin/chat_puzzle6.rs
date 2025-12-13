use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split into lines, strip possible '\r'
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.trim_end_matches('\r').chars().collect())
        .collect();

    // Remove trailing completely blank rows so the last row is the operator row
    while !lines.is_empty()
        && lines
            .last()
            .unwrap()
            .iter()
            .all(|&ch| ch == ' ')
    {
        lines.pop();
    }

    if lines.is_empty() {
        println!("0\n0");
        return;
    }

    let height = lines.len();
    let width = lines.iter().map(|row| row.len()).max().unwrap_or(0);

    // Pad rows to the same width with spaces
    for row in &mut lines {
        if row.len() < width {
            row.resize(width, ' ');
        }
    }

    // Determine which columns are entirely blank (spaces only)
    let mut blank_cols = vec![true; width];
    for x in 0..width {
        for y in 0..height {
            if lines[y][x] != ' ' {
                blank_cols[x] = false;
                break;
            }
        }
    }

    // Find contiguous column ranges for each problem (separated by fully blank columns)
    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut in_block = false;
    let mut start = 0usize;

    for x in 0..width {
        if !blank_cols[x] {
            if !in_block {
                in_block = true;
                start = x;
            }
        } else {
            if in_block {
                blocks.push((start, x - 1));
                in_block = false;
            }
        }
    }
    if in_block {
        blocks.push((start, width - 1));
    }

    // Helper to find operator in a block
    fn find_op(lines: &Vec<Vec<char>>, start: usize, end: usize) -> char {
        let op_row = lines.len() - 1;
        for x in start..=end {
            let c = lines[op_row][x];
            if c == '+' || c == '*' {
                return c;
            }
        }
        panic!("No operator found in block {}..={}", start, end);
    }

    // Part 1: numbers are horizontal, one per row within the block
    fn eval_block_part1(lines: &Vec<Vec<char>>, start: usize, end: usize) -> u128 {
        let height = lines.len();
        let op = find_op(lines, start, end);

        let mut nums: Vec<u128> = Vec::new();
        for y in 0..height - 1 {
            let mut s = String::new();
            for x in start..=end {
                let ch = lines[y][x];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                let v: u128 = s.parse().unwrap();
                nums.push(v);
            }
        }

        if nums.is_empty() {
            return 0;
        }

        match op {
            '+' => nums.into_iter().sum(),
            '*' => nums.into_iter().fold(1u128, |acc, v| acc.saturating_mul(v)),
            _ => unreachable!(),
        }
    }

    // Part 2: numbers are vertical, one per column within the block
    fn eval_block_part2(lines: &Vec<Vec<char>>, start: usize, end: usize) -> u128 {
        let height = lines.len();
        let op = find_op(lines, start, end);

        let mut nums: Vec<u128> = Vec::new();
        for x in start..=end {
            let mut s = String::new();
            for y in 0..height - 1 {
                let ch = lines[y][x];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                let v: u128 = s.parse().unwrap();
                nums.push(v);
            }
        }

        if nums.is_empty() {
            return 0;
        }

        match op {
            '+' => nums.into_iter().sum(),
            '*' => nums.into_iter().fold(1u128, |acc, v| acc.saturating_mul(v)),
            _ => unreachable!(),
        }
    }

    let part1: u128 = blocks
        .iter()
        .map(|&(s, e)| eval_block_part1(&lines, s, e))
        .sum();

    let part2: u128 = blocks
        .iter()
        .map(|&(s, e)| eval_block_part2(&lines, s, e))
        .sum();

    println!("{}", part1);
    println!("{}", part2);
}
