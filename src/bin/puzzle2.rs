use aoc2025::read_file;

fn parse_interval(interval: &str) -> (u64, u64) {
    let (x, y) = interval.split_once('-').unwrap();
    let first = x.parse().unwrap();
    let second = y.parse().unwrap();
    (first, second)
}

fn is_fake_id1(i: &u64) -> bool {
    let bi = i.to_string().into_bytes();
    bi.len() % 2 == 0 && bi[..bi.len() / 2] == bi[bi.len() / 2..]
}

fn is_fake_id2(i: &u64) -> bool {
    let bi = i.to_string().into_bytes();
    for baselen in 1..=bi.len() / 2 {
        if bi.len() % baselen == 0 {
            let mut is_fake = true;
            for j in 0..bi.len() - baselen {
                if bi[j] != bi[j + baselen] {
                    is_fake = false;
                    break;
                }
            }
            if is_fake {
                return true;
            }
        }
    }
    false
}

fn count_fake_ids(interval: &(u64, u64), checker: fn(&u64) -> bool) -> u64 {
    (interval.0..=interval.1).filter(checker).sum()
}

fn part1(intervals: &Vec<(u64, u64)>) -> u64 {
    intervals
        .into_iter()
        .map(|i| count_fake_ids(i, is_fake_id1))
        .sum()
}

fn part2(intervals: &Vec<(u64, u64)>) -> u64 {
    intervals
        .into_iter()
        .map(|i| count_fake_ids(i, is_fake_id2))
        .sum()
}

#[test]
fn test() {
    let intervals = read_file("inputs/puzzle2_example.txt", ",", parse_interval);
    assert_eq!(part1(&intervals), 1227775554);
    assert_eq!(part2(&intervals), 4174379265);
    let intervals = read_file("inputs/puzzle2.txt", ",", parse_interval);
    assert_eq!(part1(&intervals), 54641809925);
    assert_eq!(part2(&intervals), 73694270688);
}

fn main() {
    let intervals = read_file("inputs/puzzle2.txt", ",", parse_interval);
    println!("Puzzle 1: {}", part1(&intervals));
    println!("Puzzle 2: {}", part2(&intervals));
}
