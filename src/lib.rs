pub fn read_file<R>(filename: &str, pat: &str, parser: fn(&str)-> R) -> Vec<R>
{
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(pat)
        .map(parser)
        .collect()
}

pub fn read_file_split<R,S>(filename: &str, parser1: fn(&str)-> R, parser2: fn(&str)-> S) -> (Vec<R>, Vec<S>)
{
    let content = std::fs::read_to_string(filename).unwrap();

    let (part1_str, part2_str) = content
        .split_once("\n\n")
        .expect("file must contain an empty-line separator");

    let part1 = part1_str.lines().map(parser1).collect();
    let part2 = part2_str.lines().map(parser2).collect();

    (part1, part2)
}
