use aoc2025::read_file;

fn parse_rotation(rot: &str) -> i32 {
    let steps: i32 = rot[1..].parse().unwrap();
    if rot.starts_with("L") { -steps } else { steps }
}

fn part1(rotations: &Vec<i32>) -> u32 {
    let mut dial = 50;
    let mut count_zeros = 0;
    for rot in rotations {
        dial = (dial + rot).rem_euclid(100);
        // alternatively: dial = (dial + rot) % 100
        count_zeros += (dial == 0) as u32;
    }
    count_zeros
}

fn part2(rotations: &Vec<i32>) -> u32 {
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
        count_zeros += counts as u32;
    }
    count_zeros
}

#[test]
fn test() {
    let rotations = read_file("inputs/puzzle1_example.txt", "\n", parse_rotation);
    assert_eq!(part1(&rotations), 3);
    assert_eq!(part2(&rotations), 6);
    let rotations = read_file("inputs/puzzle1.txt", "\n", parse_rotation);
    assert_eq!(part1(&rotations), 992);
    assert_eq!(part2(&rotations), 6133);
}

fn main() {
    let rotations = read_file("inputs/puzzle1.txt", "\n", parse_rotation);
    println!("Part 1: {}", part1(&rotations));
    println!("Part 2: {}", part2(&rotations));
}
