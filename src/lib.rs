/// Read the file `filename`, splitting its content on the pattern `pat`. Each of the pieces
/// is passed to the `parser` function, and it returns a vector of the elements returned by
/// `parser`.
pub fn read_file<P, R>(filename: &str, pat: &str, parser: P) -> Vec<R>
where
    P: Fn(&str) -> R,
{
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(pat)
        .map(parser)
        .collect()
}

/// Read the file `filename`, splitting its content on an empty line. The part before
/// the empty line is fed to `parser1` one line at a time, the part before the empty line
/// is similarly sent to `parser2`. It returns a pair of vectors, with all the results of
/// the two parsers.
pub fn read_file_split<P1, P2, R, S>(filename: &str, parser1: P1, parser2: P2) -> (Vec<R>, Vec<S>)
where
    P1: Fn(&str) -> R,
    P2: Fn(&str) -> S,
{
    let content = std::fs::read_to_string(filename).unwrap();

    let (part1_str, part2_str) = content
        .split_once("\n\n")
        .expect("file must contain an empty-line separator");

    let part1 = part1_str.lines().map(parser1).collect();
    let part2 = part2_str.lines().map(parser2).collect();

    (part1, part2)
}
