use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(parse_rotation)
        .collect::<Result<Vec<i32>, io::Error>>()
        .unwrap();
    lines
}

fn parse_rotation(rot: Result<String, io::Error>) -> Result<i32, io::Error> {
    let rot = rot?;
    let steps: i32 = rot[1..].parse().unwrap();
    Ok(if rot.starts_with("L") { -steps } else { steps })
}

fn part1(rotations: &Vec<i32>) -> i32 {
    let mut dial = 50;
    let mut count_zeros = 0;
    for rot in rotations {
        dial = (dial + rot).rem_euclid(100);
        // alternatively: dial = (dial + rot) % 100
        count_zeros += (dial == 0) as i32;
    }
    count_zeros
}

fn part2(rotations: &Vec<i32>) -> i32 {
    let mut dial = 50;
    let mut count_zeros = 0;
    for rot in rotations {
        let counts = if *rot >= 0 {
            (dial + rot) / 100
        } else {
            let cross_zero = dial > 0 && dial + rot <= 0;
            -(dial + rot) / 100 + cross_zero as i32
            // alternatively: ((100 - dial).rem_euclid(100) - rot)/100
        };
        dial = (dial + rot).rem_euclid(100);
        count_zeros += counts;
    }
    count_zeros
}

#[test]
fn test() {
    let rotations: Vec<i32> = read_file("inputs/puzzle1_example.txt");
    assert_eq!(part1(&rotations), 3);
    assert_eq!(part2(&rotations), 6);
    let rotations = read_file("inputs/puzzle1.txt");
    assert_eq!(part1(&rotations), 992);
    assert_eq!(part2(&rotations), 6133);
}

fn main() {
    let rotations = read_file("inputs/puzzle1.txt");
    println!("Puzzle 1: {}", part1(&rotations));
    println!("Puzzle 2: {}", part2(&rotations));
}
