use std::collections::HashMap;

use aoc2025::read_file;

/// A Server is a line in the input file.
type Server = (String, Vec<String>);

/// A Rack represents the entire set of servers, encoded as an HashMap. The key is the name
/// of a server, and the content is the list of connected servers.
type Rack = HashMap<String, Vec<String>>;

/// Generates a rack from the vector of servers.
fn rack_from_servers(servers: &Vec<Server>) -> Rack {
    servers.iter().cloned().collect()
}

/// Memoized recursive implementation of `compute_paths`.
fn compute_paths_aux<'a, 'b>(
    rack: &'a Rack,
    origin: &'a str,
    intermediates: &Vec<&'b str>,
    memo: &mut HashMap<(&'a str, Vec<&'b str>), u64>,
) -> u64 {
    if origin == "out" {
        if intermediates.is_empty() { 1 } else { 0 }
    } else {
        // I don't like cloning intermediates here, but there is not a simple workaround
        // since building a tuple requires taking owenership of the object.
        match memo.get(&(origin, intermediates.clone())) {
            Some(&v) => v,
            None => {
                let origin_index = intermediates.iter().position(|&x| x == origin);
                let mut intermediates_new = intermediates.clone();
                if let Some(i) = origin_index {
                    intermediates_new.remove(i);
                }
                let result = rack[origin]
                    .iter()
                    .map(|dst| compute_paths_aux(rack, dst, &intermediates_new, memo))
                    .sum();
                memo.insert((origin, intermediates_new), result);
                result
            }
        }
    }
}

/// Computes the number of paths in the rack from `origin` to the "out" servers,
/// only considered those paths which traverse the servers in `intermediates`.
fn compute_paths(rack: &Rack, intermediates: &Vec<&str>, origin: &str) -> u64 {
    compute_paths_aux(rack, origin, intermediates, &mut HashMap::new())
}

/// Parse on row of the input file.
fn parse_server(row: &str) -> Server {
    let (key, outstring) = row.split_once(":").unwrap();
    let outs = outstring.split_whitespace().map(String::from).collect();
    (key.to_string(), outs)
}

fn part1(servers: &Vec<Server>) -> u64 {
    let rack = rack_from_servers(servers);
    compute_paths(&rack, &vec![], "you")
}

fn part2(servers: &Vec<Server>) -> u64 {
    let rack = rack_from_servers(servers);
    compute_paths(&rack, &vec!["fft", "dac"], "svr")
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle11_example.txt", "\n", parse_server);
    assert_eq!(part1(&content), 5);
    let content = read_file("inputs/puzzle11_example2.txt", "\n", parse_server);
    assert_eq!(part2(&content), 2);
    let content = read_file("inputs/puzzle11.txt", "\n", parse_server);
    assert_eq!(part1(&content), 701);
    assert_eq!(part2(&content), 390108778818526);
}

fn main() {
    let content = read_file("inputs/puzzle11.txt", "\n", parse_server);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
