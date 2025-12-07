use aoc2025::read_file;
use std::collections::hash_set::HashSet;

fn parse_row(row: &str) -> Vec<u8> {
    row.as_bytes().to_vec()
}

fn part1(manifold: &Vec<Vec<u8>>) -> u64 {
    let start = manifold[0].iter().position(|&x| x == b'S').unwrap();
    let mut beams = HashSet::from([start]);
    let mut newbeams = HashSet::new();
    let mut splits = 0;
    for i in 1..manifold.len() {
        for &beam in &beams {
            if manifold[i][beam] == b'^' {
                splits += 1;
                newbeams.insert(beam-1);
                newbeams.insert(beam+1);
            } else {
                newbeams.insert(beam);
            }
        }
        beams.drain();
        (beams, newbeams) = (newbeams, beams);
    }
    splits
}

fn part2_inner(manifold: &Vec<Vec<u8>>, row: usize, col: usize, cache: &mut Vec<Vec<u64>>) -> u64 {
    if cache[row][col] == 0 {
        if row == manifold.len() - 1 {
            cache[row][col] = 1;
        } else if manifold[row][col] == b'^' {
            let left = part2_inner(manifold, row+1, col-1, cache);
            let right = part2_inner(manifold, row+1, col+1, cache);
            cache[row][col] = left + right
        } else {
            cache[row][col] = part2_inner(manifold, row+1, col, cache)
        }
    }
    cache[row][col]
}

fn part2(manifold: &Vec<Vec<u8>>) -> u64 {
    let mut cache = vec![vec![0; manifold[0].len()] ; manifold.len()];
    let start = manifold[0].iter().position(|&x| x == b'S').unwrap();
    part2_inner(manifold, 0, start, &mut cache)
}


#[test]
fn test() {
    let content = read_file("inputs/puzzle7_example.txt", "\n", parse_row);
    assert_eq!(part1(&content), 21);
    assert_eq!(part2(&content), 40);
    let content = read_file("inputs/puzzle7.txt", "\n", parse_row);
    assert_eq!(part1(&content), 1533);
    assert_eq!(part2(&content), 10733529153890);
}

fn main() {
    let content = read_file("inputs/puzzle7.txt", "\n", parse_row);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
