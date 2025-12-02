fn read_file(filename: &str) -> Vec<(u64, u64)> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split(',')
        .map(parse_interval)
        .collect()
}

fn parse_interval(str_interval: &str) -> (u64, u64) {
    let (x, y) = str_interval.trim().split_once('-').unwrap();
    let first = x.parse().unwrap();
    let second = y.parse().unwrap();
    (first, second)
}

fn count_fake_id(interval: &(u64, u64)) -> u64 {
    let mut fakes = 0;
    for i in interval.0..(interval.1 + 1) {
        let si = i.to_string().into_bytes();
        if si.len() % 2 == 0 {
            if si[0..si.len() / 2] == si[si.len() / 2..] {
                fakes += i;
            }
        }
    }
    fakes
}

fn count_fake_id2(interval: &(u64, u64)) -> u64 {
    let mut fakes = 0;
    for i in interval.0..(interval.1 + 1) {
        let bi = i.to_string().into_bytes();
        let bi_len = bi.len();
        for baselen in 1..(bi_len / 2 + 1) {
            if bi_len % baselen != 0 {
                continue;
            }
            let mut is_fake = true;
            for j in 0..(bi_len - baselen) {
                if bi[j] != bi[j + baselen] {
                    is_fake = false;
                    break;
                }
            }
            if is_fake {
                fakes += i;
                break;
            }
        }
    }
    fakes
}

fn part1(intervals: &Vec<(u64, u64)>) -> u64 {
    intervals.into_iter().map(count_fake_id).sum()
}

fn part2(intervals: &Vec<(u64, u64)>) -> u64 {
    intervals.into_iter().map(count_fake_id2).sum()
}

#[test]
fn test() {
    let intervals = read_file("inputs/puzzle2_example.txt");
    assert_eq!(part1(&intervals), 1227775554);
    assert_eq!(part2(&intervals), 4174379265);
    let intervals = read_file("inputs/puzzle2.txt");
    assert_eq!(part1(&intervals), 54641809925);
    assert_eq!(part2(&intervals), 73694270688);
}

fn main() {
    let intervals = read_file("inputs/puzzle2.txt");
    println!("Puzzle 1: {}", part1(&intervals));
    println!("Puzzle 2: {}", part2(&intervals));
}
