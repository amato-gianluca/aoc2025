use aoc2025::read_file;

fn parse_bank(bank: &str) -> Vec<u8> {
    bank.as_bytes().iter().map(|x| x - b'0').collect()
}

fn max_bank(bank: &Vec<u8>, digits: usize) -> u64 {
    let mut idx = 0;
    let mut val = 0;
    for i in 0..digits {
        let (maxidx, &maxval) = bank[idx..bank.len() - digits + i + 1]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| std::cmp::Reverse(v))
            .unwrap();
        val = (val * 10) + (maxval as u64);
        idx += maxidx + 1
    }
    val
}

fn part1(banks: &Vec<Vec<u8>>) -> u64 {
    banks.iter().map(|bank| max_bank(bank, 2)).sum()
}

fn part2(banks: &Vec<Vec<u8>>) -> u64 {
    banks.iter().map(|bank| max_bank(bank, 12)).sum()
}

#[test]
fn test() {
    let banks = read_file("inputs/puzzle3_example.txt", "\n", parse_bank);
    assert_eq!(part1(&banks), 357);
    assert_eq!(part2(&banks), 3121910778619);
    let banks = read_file("inputs/puzzle3.txt", "\n", parse_bank);
    assert_eq!(part1(&banks), 17158);
    assert_eq!(part2(&banks), 170449335646486);
}

fn main() {
    let banks = read_file("inputs/puzzle3.txt", "\n", parse_bank);
    println!("Puzzle 1: {}", part1(&banks));
    println!("Puzzle 2: {}", part2(&banks));
}
