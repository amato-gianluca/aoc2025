use std::collections::VecDeque;
use std::io::{self, Read};

fn parse_line(line: &str) -> (String, Vec<Vec<usize>>, Vec<usize>) {
    let line = line.trim();

    // Indicator pattern in [ .. ]
    let lbr = line.find('[').expect("no [ in line");
    let rbr_rel = line[lbr + 1..]
        .find(']')
        .expect("no ] in line after [");
    let rbr = lbr + 1 + rbr_rel;
    let pattern = line[lbr + 1..rbr].to_string();

    // Joltage targets in { .. }
    let lcur = line
        .find('{')
        .expect("no { in line (joltage requirements)");
    let rcur_rel = line[lcur + 1..]
        .find('}')
        .expect("no } in line after {");
    let rcur = lcur + 1 + rcur_rel;
    let targets_str = &line[lcur + 1..rcur];

    let targets: Vec<usize> = targets_str
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<usize>().expect("bad target number"))
        .collect();

    // Buttons: all (...) before the {
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut idx = 0usize;
    while let Some(rel) = line[idx..].find('(') {
        let sidx = idx + rel;
        if sidx > lcur {
            break; // don't go past the joltage block
        }
        let e_rel = line[sidx..]
            .find(')')
            .expect("no ) to match (");
        let eidx = sidx + e_rel;
        let inner = line[sidx + 1..eidx].trim();
        if inner.is_empty() {
            buttons.push(Vec::new());
        } else {
            let indices = inner
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<usize>().expect("bad button index"))
                .collect::<Vec<_>>();
            buttons.push(indices);
        }
        idx = eidx + 1;
    }

    (pattern, buttons, targets)
}

/// Part 1: BFS over all indicator-light states (bitmasks).
fn min_presses_lights(pattern: &str, buttons: &Vec<Vec<usize>>) -> i64 {
    let l = pattern.chars().count();
    assert!(
        l <= 63,
        "Too many indicator lights ({}), u64 mask would overflow",
        l
    );

    // Target bitmask
    let mut target_mask: u64 = 0;
    for (i, ch) in pattern.chars().enumerate() {
        match ch {
            '#' => target_mask |= 1u64 << i,
            '.' => {}
            _ => panic!("unexpected char in pattern: {}", ch),
        }
    }
    let target_state = target_mask as usize;

    // Precompute button masks
    let mut button_masks: Vec<usize> = Vec::new();
    for b in buttons {
        let mut mask: u64 = 0;
        for &idx in b {
            if idx >= l {
                panic!(
                    "button index {} out of range for {} lights in pattern {}",
                    idx, l, pattern
                );
            }
            mask |= 1u64 << idx;
        }
        button_masks.push(mask as usize);
    }

    let n_states = 1usize << l;
    let mut dist = vec![-1_i32; n_states];
    let mut q = VecDeque::new();

    dist[0] = 0;
    q.push_back(0usize);

    while let Some(state) = q.pop_front() {
        if state == target_state {
            return dist[state] as i64;
        }
        let curd = dist[state];
        for &bm in &button_masks {
            let ns = state ^ bm;
            if dist[ns] == -1 {
                dist[ns] = curd + 1;
                q.push_back(ns);
            }
        }
    }

    panic!("No solution found for lights pattern {}", pattern);
}

/// Part 2: BFS/Dijkstra in counter-space (multi-dimensional grid).
fn min_presses_joltage(buttons: &Vec<Vec<usize>>, target: &Vec<usize>) -> i64 {
    let d = target.len();
    if d == 0 {
        return 0;
    }

    // Quick feasibility check: any counter with target>0 but not affected by any button
    let mut affected = vec![false; d];
    for b in buttons {
        for &idx in b {
            if idx < d {
                affected[idx] = true;
            }
        }
    }
    for i in 0..d {
        if target[i] > 0 && !affected[i] {
            panic!(
                "Counter {} has positive target {} but no button affects it",
                i, target[i]
            );
        }
    }

    // Mixed-radix representation:
    // value[i] ranges 0..=target[i]
    // base[i] = product_{k < i} (target[k] + 1)
    // id = sum_i value[i] * base[i]
    let mut bases = vec![0usize; d];
    let mut mult: usize = 1;
    let mut goal_id: usize = 0;
    for i in 0..d {
        bases[i] = mult;
        goal_id += target[i] * bases[i];
        mult = mult.saturating_mul(target[i] + 1);
    }
    let total_states = mult;

    let mut dist = vec![-1_i32; total_states];
    let mut q = VecDeque::new();

    dist[0] = 0;
    q.push_back(0usize);

    while let Some(id) = q.pop_front() {
        if id == goal_id {
            return dist[id] as i64;
        }
        let curd = dist[id];

        // Decode id -> vector v[]
        let mut v = vec![0usize; d];
        let mut rem = id;
        for i in 0..d {
            let range = target[i] + 1;
            v[i] = rem % range;
            rem /= range;
        }

        // Try pressing each button once
        'button_loop: for b in buttons {
            let mut nid = id;
            // Check if pressing this button keeps all counters <= target
            for &idx in b {
                if idx >= d {
                    // button refers to non-existent counter: invalid problem
                    panic!(
                        "Button references counter {} but there are only {} counters",
                        idx, d
                    );
                }
                if v[idx] + 1 > target[idx] {
                    continue 'button_loop; // would overshoot this counter
                }
            }
            // All good; update nid by adding bases[idx] for each affected counter
            for &idx in b {
                nid += bases[idx];
            }

            if dist[nid] == -1 {
                dist[nid] = curd + 1;
                q.push_back(nid);
            }
        }
    }

    panic!("No solution found for joltage target {:?}", target);
}

fn main() {
    // Read whole stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut part1_total: i64 = 0;
    let mut part2_total: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (pattern, buttons, targets) = parse_line(line);

        let p1 = min_presses_lights(&pattern, &buttons);
        let p2 = min_presses_joltage(&buttons, &targets);

        part1_total += p1;
        part2_total += p2;
    }

    println!("{}", part1_total);
    println!("{}", part2_total);
}
