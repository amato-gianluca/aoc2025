use std::{collections::HashMap};

use aoc2025::read_file;

type Server = (String, Vec<String>);

struct Rack {
    connections: HashMap<String, Vec<String>>,
}

impl Rack {
    fn from(servers: &Vec<Server>) -> Rack {
        let connections: HashMap<_, _> = servers.iter().cloned().collect();
        Rack { connections }
    }

    fn compute_paths_aux(&self, origin: &str, to_pass: &Vec<String>, memo: &mut HashMap<(String, Vec<String>), u64> ) -> u64 {
        if origin == "out" {
            if to_pass.is_empty() { 1 } else { 0 }
        } else {
            match memo.get(&(origin.to_string(), to_pass.to_vec())) {
                Some(&v) => v,
                None => {
                    let origin_index = to_pass.iter().position(|x| x == origin);
                    let to_pass_new = match origin_index {
                        None => to_pass.clone(),
                        Some(i) => {
                            let mut v = to_pass.clone();
                            v.remove(i);
                            v
                        }
                    };
                    let result = self.connections[origin]
                        .iter()
                        .map(|dst| self.compute_paths_aux(dst, &to_pass_new, memo))
                        .sum();
                    memo.insert((origin.to_string(), to_pass_new), result);
                    result
                }
            }
        }
    }

    fn compute_paths(&self, to_pass: &Vec<String>, origin: &str) -> u64 {
        self.compute_paths_aux(origin, to_pass, &mut HashMap::new())
    }
}

fn parse_server(row: &str) -> Server {
    let (key, outstring) = row.split_once(":").unwrap();
    let outs = outstring.split_whitespace().map(String::from).collect();
    (key.to_string(), outs)
}

fn part1(servers: &Vec<Server>) -> u64 {
    let rack = Rack::from(servers);
    rack.compute_paths(&vec![], "you")
}

fn part2(servers: &Vec<Server>) -> u64 {
    let rack = Rack::from(servers);
    rack.compute_paths(&vec!["fft".to_string(), "dac".to_string()], "svr")

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
