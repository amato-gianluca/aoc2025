use std::cmp::{max, min};

use aoc2025::read_file;

type Coord = (i64, i64, i64);
type Dist = i64;

#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    fn new(items: usize) -> Self {
        Self {
            parents: (0..items).collect(),
            sizes: vec![1; items],
            groups: items,
        }
    }

    fn find(&self, i: usize) -> usize {
        let mut curr = i;
        while self.parents[curr] != curr {
            curr = self.parents[curr];
        }
        curr
    }

    fn union(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        let p1 = min(pi, pj);
        let p2 = max(pi, pj);
        if p1 != p2 {
            self.parents[p2] = p1;
            self.sizes[p1] += self.sizes[p2];
            self.sizes[p2] = 0;
            self.groups -= 1
        }
    }

    fn sorted_groups(&self) -> Vec<usize> {
        let mut v: Vec<usize> = (0..self.parents.len())
            .filter(|&x| self.parents[x] == x)
            .collect();
        v.sort_by_key(|&x| self.sizes[x]);
        v
    }
}

fn parse_coord(row: &str) -> Coord {
    let mut iterator = row.split(',').map(|x| x.parse().unwrap());
    (
        iterator.next().unwrap(),
        iterator.next().unwrap(),
        iterator.next().unwrap(),
    )
}

fn dist(c1: Coord, c2: Coord) -> Dist {
    (c1.0 - c2.0) * (c1.0 - c2.0) + (c1.1 - c2.1) * (c1.1 - c2.1) + (c1.2 - c2.2) * (c1.2 - c2.2)
}

fn distance_matrix(content: &Vec<Coord>) -> Vec<(usize, usize)> {
    let mut matrix = Vec::new();
    for i in 0..content.len() {
        for j in i + 1..content.len() {
            matrix.push((i, j))
        }
    }
    matrix.sort_by_cached_key(|&(i, j)| dist(content[i], content[j]));
    matrix
}

fn part1(content: &Vec<Coord>, joins: usize) -> usize {
    let distances = distance_matrix(&content);
    let mut g = UnionFind::new(content.len());
    for i in 0..joins {
        let (p1, p2) = distances[i];
        g.union(p1, p2);
    }
    let top_groups = g.sorted_groups();
    top_groups
        .iter()
        .rev()
        .take(3)
        .map(|&x| g.sizes[x])
        .product()
}

fn part2(content: &Vec<Coord>) -> i64 {
    let distance = distance_matrix(&content);
    let mut g = UnionFind::new(content.len());
    let mut i = 0;
    loop {
        let (p1, p2) = distance[i];
        g.union(p1, p2);
        if g.groups == 1 {
            return content[p1].0 * content[p2].0;
        }
        i += 1;
    }
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle8_example.txt", "\n", parse_coord);
    assert_eq!(part1(&content, 10), 40);
    assert_eq!(part2(&content), 25272);
    let content = read_file("inputs/puzzle8.txt", "\n", parse_coord);
    assert_eq!(part1(&content, 1000), 84968);
    assert_eq!(part2(&content), 8663467782);
}

fn main() {
    let content = read_file("inputs/puzzle8.txt", "\n", parse_coord);
    println!("Part 1: {}", part1(&content, 1000));
    println!("Part 2: {}", part2(&content));
}
