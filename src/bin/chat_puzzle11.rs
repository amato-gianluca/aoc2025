use std::collections::HashMap;
use std::io::{self, Read};

fn dfs_count(
    u: usize,
    target: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
) -> u64 {
    if u == target {
        return 1;
    }

    visited[u] = true;
    let mut total = 0u64;
    for &v in &adj[u] {
        if !visited[v] {
            total += dfs_count(v, target, adj, visited);
        }
    }
    visited[u] = false;

    total
}

fn dfs_with_requirements(
    u: usize,
    target: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    dac_id: Option<usize>,
    fft_id: Option<usize>,
    has_dac: bool,
    has_fft: bool,
    total_paths: &mut u64,
    both_paths: &mut u64,
) {
    // Update flags for current node
    let mut has_dac_now = has_dac;
    let mut has_fft_now = has_fft;

    if let Some(d) = dac_id {
        if u == d {
            has_dac_now = true;
        }
    }
    if let Some(f) = fft_id {
        if u == f {
            has_fft_now = true;
        }
    }

    if u == target {
        *total_paths += 1;
        if has_dac_now && has_fft_now {
            *both_paths += 1;
        }
        return;
    }

    visited[u] = true;
    for &v in &adj[u] {
        if !visited[v] {
            dfs_with_requirements(
                v,
                target,
                adj,
                visited,
                dac_id,
                fft_id,
                has_dac_now,
                has_fft_now,
                total_paths,
                both_paths,
            );
        }
    }
    visited[u] = false;
}

fn main() {
    // Read entire input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Map device names to integer IDs
    let mut name_to_id: HashMap<String, usize> = HashMap::new();
    // Temporarily store edges as (src_id, Vec<dest_name>)
    let mut raw_edges: Vec<(usize, Vec<String>)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(':');
        let left = match parts.next() {
            Some(s) => s.trim(),
            None => continue,
        };
        let right = parts.next().unwrap_or("").trim();

        if left.is_empty() {
            continue;
        }

        // Get or assign ID for source
        let src_id = {
            let len = name_to_id.len();
            *name_to_id.entry(left.to_string()).or_insert(len)
        };

        let mut dest_names = Vec::new();
        if !right.is_empty() {
            for tok in right.split_whitespace() {
                if tok.is_empty() {
                    continue;
                }
                let name = tok.to_string();
                let len = name_to_id.len();
                name_to_id.entry(name.clone()).or_insert(len);
                dest_names.push(name);
            }
        }

        raw_edges.push((src_id, dest_names));
    }

    let n = name_to_id.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Build adjacency list
    for (src_id, dest_names) in raw_edges {
        for dn in dest_names {
            if let Some(&v) = name_to_id.get(&dn) {
                adj[src_id].push(v);
            }
        }
    }

    // Helper to look up IDs
    let you_id = name_to_id.get("you").cloned();
    let out_id = name_to_id.get("out").cloned();
    let svr_id = name_to_id.get("svr").cloned();
    let dac_id = name_to_id.get("dac").cloned();
    let fft_id = name_to_id.get("fft").cloned();

    // Part 1: paths from "you" to "out"
    let part1 = if let (Some(start), Some(target)) = (you_id, out_id) {
        let mut visited = vec![false; n];
        dfs_count(start, target, &adj, &mut visited)
    } else {
        0
    };

    // Part 2: paths from "svr" to "out" that visit both "dac" and "fft"
    let mut part2_both = 0u64;
    if let (Some(start), Some(target)) = (svr_id, out_id) {
        let mut visited = vec![false; n];
        let mut total_paths = 0u64; // not printed, but tracked as per problem text
        dfs_with_requirements(
            start,
            target,
            &adj,
            &mut visited,
            dac_id,
            fft_id,
            false,
            false,
            &mut total_paths,
            &mut part2_both,
        );
    }

    // Output answers: first for part 1, then for part 2
    println!("{}", part1);
    println!("{}", part2_both);
}
