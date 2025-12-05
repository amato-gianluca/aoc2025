use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();

    // First section: ranges, then a blank line, then IDs
    let mut in_ranges = true;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            in_ranges = false;
            continue;
        }

        if in_ranges {
            // Expect "start-end"
            if let Some((a, b)) = line.split_once('-') {
                let mut start: i64 = a.trim().parse().unwrap();
                let mut end: i64 = b.trim().parse().unwrap();
                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }
                ranges.push((start, end));
            } else {
                // Fallback: treat a single number as a degenerate range
                let v: i64 = line.parse().unwrap();
                ranges.push((v, v));
            }
        } else {
            // Second section: individual IDs
            let v: i64 = line.parse().unwrap();
            ids.push(v);
        }
    }

    if ranges.is_empty() {
        println!("0");
        println!("0");
        return;
    }

    // Merge overlapping ranges
    ranges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (s, e) in ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 {
                // Overlapping: extend the last range
                last.1 = max(last.1, e);
            } else {
                // Disjoint: start a new merged range
                merged.push((s, e));
            }
        } else {
            merged.push((s, e));
        }
    }

    // Part 1: count how many available IDs fall into any range
    let mut fresh_count: i64 = 0;
    'outer: for id in ids {
        for &(s, e) in &merged {
            if id < s {
                // Ranges are sorted; no later range can contain this id
                continue 'outer;
            }
            if id <= e {
                fresh_count += 1;
                continue 'outer;
            }
        }
    }

    // Part 2: total number of integer IDs covered by the union of ranges
    let mut total_fresh_ids: i64 = 0;
    for (s, e) in &merged {
        if e >= s {
            total_fresh_ids += e - s + 1;
        }
    }

    // Output answers
    println!("{}", fresh_count);
    println!("{}", total_fresh_ids);
}
