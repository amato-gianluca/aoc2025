use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    d2: i64,
    a: usize,
    b: usize,
}

impl Edge {
    fn new(a: usize, b: usize, d2: i64) -> Self {
        Edge { d2, a, b }
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.d2 == other.d2 && self.a == other.a && self.b == other.b
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by distance squared, then by endpoints to break ties
        self.d2
            .cmp(&other.d2)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        DSU { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    fn component_sizes(&mut self) -> Vec<i64> {
        let n = self.parent.len();
        let mut root_size = std::collections::HashMap::<usize, i64>::new();
        for i in 0..n {
            let r = self.find(i);
            *root_size.entry(r).or_insert(0) += 1;
        }
        root_size.values().cloned().collect()
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Expect "x,y,z"
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue; // ignore malformed lines
        }
        let x: i64 = parts[0].trim().parse().unwrap();
        let y: i64 = parts[1].trim().parse().unwrap();
        let z: i64 = parts[2].trim().parse().unwrap();
        points.push(Point { x, y, z });
    }

    let n = points.len();
    if n == 0 {
        // Degenerate case; just print 0s
        println!("0");
        println!("0");
        return;
    }
    if n == 1 {
        // Only one junction box: one circuit and no connection needed
        println!("1");
        println!("0");
        return;
    }

    // Build all edges with squared Euclidean distance
    let mut edges: Vec<Edge> = Vec::new();
    edges.reserve(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let d2 = dx * dx + dy * dy + dz * dz;
            edges.push(Edge::new(i, j, d2));
        }
    }

    // Sort edges by increasing distance
    edges.sort();

    // Part 1: connect the 1000 closest pairs (edges), even if they don't change the circuits
    let mut dsu1 = DSU::new(n);
    let edges_to_use = std::cmp::min(1000usize, edges.len());
    for i in 0..edges_to_use {
        let e = edges[i];
        let _ = dsu1.union(e.a, e.b);
    }

    let mut sizes = dsu1.component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // descending

    let mut part1: i128 = 1;
    for i in 0..3 {
        if i < sizes.len() {
            part1 *= sizes[i] as i128;
        }
    }

    // Part 2: from scratch, keep connecting closest pairs until all are in one circuit
    let mut dsu2 = DSU::new(n);
    let mut components_left = n as i64;
    let mut part2: i128 = 0;

    for e in &edges {
        if dsu2.union(e.a, e.b) {
            components_left -= 1;
            if components_left == 1 {
                // This is the last edge needed to connect all boxes
                let pa = &points[e.a];
                let pb = &points[e.b];
                part2 = (pa.x as i128) * (pb.x as i128);
                break;
            }
        }
    }

    println!("{}", part1);
    println!("{}", part2);
}
