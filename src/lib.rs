pub fn read_file<R>(filename: &str, pat: &str, parser: fn(&str)-> R) -> Vec<R>
{
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(pat)
        .map(parser)
        .collect()
}
