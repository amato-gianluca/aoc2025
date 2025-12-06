use aoc2025::read_file;

fn parse_row(row: &str) -> String {
    row.to_string()
}

fn parse_table(table: &Vec<String>) -> (Vec<Vec<u64>>, Vec<&str>) {
    let values = table[0..table.len() - 1]
        .iter()
        .map(|row| row.split_whitespace().map(|v| v.parse().unwrap()).collect())
        .collect();
    let operators = table.last().unwrap().split_whitespace().collect();
    (values, operators)
}

fn part1(content: &Vec<String>) -> u64 {
    let (values, operators) = parse_table(content);
    let mut result = 0;
    for i in 0..operators.len() {
        let mut problem_result = (operators[i] == "*") as u64;
        for j in 0..values.len() {
            if operators[i] == "+" {
                problem_result += values[j][i]
            } else {
                problem_result *= values[j][i]
            }
        }
        result += problem_result
    }
    result
}

fn part2(content: &Vec<String>) -> u64 {
    let raw = content
        .iter()
        .map(|row| row.as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut result = 0; // the puzzle result
    let mut opcol = true; // true if we are on the first column of a problem
    let mut op = b' '; // the operators to use for the current problem
    let mut problem_result = 0; // the result of the current problem
    let ncols = raw.iter().map(|r| r.len()).max().unwrap();
    // we also cycle on the non-existent column ncol so that we can treat
    // the last problem like all the others
    for j in 0..=ncols {
        if opcol {
            op = raw[raw.len() - 1][j];
            opcol = false;
            problem_result = (op == b'*') as u64; // unit of operation op
        }
        let mut column: u64 = 0; // the value of the current column
        for i in 0..raw.len() - 1 {
            if j < raw[i].len() && raw[i][j] != b' ' {
                column = column * 10 + ((raw[i][j] - b'0') as u64);
            }
        }
        if column == 0 {
            // problem terminated
            opcol = true;
            result += problem_result
        } else {
            // still in the middle of a problem
            if op == b'*' {
                problem_result *= column
            } else {
                problem_result += column
            };
        }
    }
    result
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle6_example.txt", "\n", parse_row);
    assert_eq!(part1(&content), 4277556);
    assert_eq!(part2(&content), 3263827);
    let content = read_file("inputs/puzzle6.txt", "\n", parse_row);
    assert_eq!(part1(&content), 6503327062445);
    assert_eq!(part2(&content), 9640641878593);
}

fn main() {
    let content = read_file("inputs/puzzle6.txt", "\n", parse_row);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
