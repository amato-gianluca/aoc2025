use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Collect non-empty lines as the grid
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();

    if lines.is_empty() {
        println!("Part 1: 0");
        println!("Part 2: 0");
        return;
    }

    let h = lines.len();
    let w = lines[0].len();

    // Grid of which cells contain a roll of paper
    let mut is_roll = vec![false; h * w];

    for (r, line) in lines.iter().enumerate() {
        assert_eq!(
            line.len(),
            w,
            "All lines must have the same length (found differing lengths)"
        );
        for (c, ch) in line.chars().enumerate() {
            if ch == '@' {
                is_roll[r * w + c] = true;
            }
        }
    }

    // 8 directions (neighbor offsets)
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let total_cells = h * w;

    // Compute initial neighbor counts (degrees) for each roll
    let mut deg = vec![0u8; total_cells];
    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if !is_roll[idx] {
                continue;
            }
            let mut count = 0u8;
            for (dr, dc) in dirs.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                    continue;
                }
                let nidx = nr as usize * w + nc as usize;
                if is_roll[nidx] {
                    count += 1;
                }
            }
            deg[idx] = count;
        }
    }

    // Part 1: rolls with fewer than 4 neighboring rolls
    let mut part1 = 0usize;
    for idx in 0..total_cells {
        if is_roll[idx] && deg[idx] < 4 {
            part1 += 1;
        }
    }

    // Part 2: iteratively remove accessible rolls (k-core with k=4)
    let mut deg2 = deg.clone();
    let mut removed = vec![false; total_cells];
    let mut queue = VecDeque::new();

    // Initially accessible rolls
    for idx in 0..total_cells {
        if is_roll[idx] && deg2[idx] < 4 {
            queue.push_back(idx);
        }
    }

    let mut removed_count = 0usize;

    while let Some(idx) = queue.pop_front() {
        if removed[idx] || !is_roll[idx] {
            continue;
        }
        // If its degree has risen back to 4+ somehow, skip (shouldn't happen, degrees only go down)
        if deg2[idx] >= 4 {
            continue;
        }

        removed[idx] = true;
        removed_count += 1;

        let r = idx / w;
        let c = idx % w;

        // Removing this roll decreases the degree of its neighboring rolls
        for (dr, dc) in dirs.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                continue;
            }
            let nidx = nr as usize * w + nc as usize;
            if !is_roll[nidx] || removed[nidx] {
                continue;
            }

            if deg2[nidx] > 0 {
                deg2[nidx] -= 1;
                // If a neighbor just dropped below 4, it becomes removable
                if deg2[nidx] == 3 {
                    queue.push_back(nidx);
                }
            }
        }
    }

    let part2 = removed_count;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
