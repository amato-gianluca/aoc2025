use aoc2025::read_file_split;

fn parse_range_id(row: &str) -> (u64, u64) {
    let (start, end) = row
        .split_once('-')
        .map(|(l, r)| (l.parse(), r.parse()))
        .unwrap();
    (start.unwrap(), end.unwrap())
}

fn parse_id(row: &str) -> u64 {
    row.parse().unwrap()
}

fn is_safe(id: u64, safe_ids: &Vec<(u64, u64)>) -> bool {
    safe_ids
        .iter()
        .any(|(l, r)| *l <= id && id <= *r)
}

fn part1(safe_ids: &Vec<(u64, u64)>, available_ids: &Vec<u64>) -> u64 {
    available_ids
        .iter()
        .map(|id| is_safe(*id, safe_ids) as u64)
        .sum()
}

fn part2(safe_ids: &Vec<(u64, u64)>) -> u64 {
    let mut safe_ids_sorted = safe_ids.clone();
    safe_ids_sorted.sort_by_key(|(l, _r)| *l);
    let mut count = 0;
    let mut prev_r = 0;
    for (l, r) in safe_ids_sorted {
        if prev_r < l {
            count += r - l + 1;
            prev_r = r
        } else if prev_r < r {
            // we don't have +1 since prev_r has been counted already
            count += r - prev_r;
            prev_r = r
        }
    }
    count
}

#[test]
fn test() {
    let (safe_ids, available_ids) = read_file_split("inputs/puzzle5_example.txt", parse_range_id, parse_id);
    assert_eq!(part1(&safe_ids, &available_ids), 3);
    assert_eq!(part2(&safe_ids), 14);
    let (safe_ids, available_ids) = read_file_split("inputs/puzzle5.txt", parse_range_id, parse_id);
    assert_eq!(part1(&safe_ids, &available_ids), 888);
    assert_eq!(part2(&safe_ids), 344378119285354);
}

fn main() {
    let (safe_ids, available_ids) = read_file_split("inputs/puzzle5.txt", parse_range_id, parse_id);
    println!("Part 1: {}", part1(&safe_ids, &available_ids));
    println!("Part 2: {}", part2(&safe_ids));
}
