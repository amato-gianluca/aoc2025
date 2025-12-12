use aoc2025::read_file;

type JunctionBox = (i64, i64, i64);
type Dist = i64;

/// This struct implements the Union-Find algorithm in Rust over elements
/// of type `usize`.
#[derive(Debug)]
struct UnionFind {
    /// Record the parent of each element.
    parents: Vec<usize>,
    /// Record the size of the group of each element.
    sizes: Vec<usize>,
    /// Number of groups in the structure.
    groups: usize,
}

impl UnionFind {
    /// Create a new Union-Find structures with numbers from `0` to `items`.
    fn new(items: usize) -> Self {
        Self {
            parents: (0..items).collect(),
            sizes: vec![1; items],
            groups: items,
        }
    }

    /// Find the root of the group of the element `i`.
    fn find(&mut self, i: usize) -> usize {
        let mut curr = i;
        while self.parents[curr] != curr {
            // compact parents while traversng upwards
            let newcurr = self.parents[curr];
            self.parents[curr] = self.parents[newcurr];
            curr = newcurr;
        }
        curr
    }

    /// Merge groups corresponding to elements `i` and `j`.
    fn union(&mut self, i: usize, j: usize) {
        let mut p1 = self.find(i);
        let mut p2 = self.find(j);
        if p1 != p2 {
            if self.sizes[p2] > self.sizes[p1] {
                std::mem::swap(&mut p1, &mut p2);
            }
            self.parents[p2] = p1;
            self.sizes[p1] += self.sizes[p2];
            self.groups -= 1
        }
    }

    /// Return a vector with the roots of the groups, sorted by group sizes.
    fn sorted_groups(&self) -> Vec<usize> {
        let mut v: Vec<usize> = (0..self.parents.len())
            .filter(|&x| self.parents[x] == x)
            .collect();
        v.sort_by_key(|&x| self.sizes[x]);
        v
    }
}

/// Parse a row of the file into a triple of coordinates.
fn parse_junction_box(row: &str) -> JunctionBox {
    let mut iterator = row.split(',').map(|x| x.parse().unwrap());
    (
        iterator.next().unwrap(),
        iterator.next().unwrap(),
        iterator.next().unwrap(),
    )
}

/// Computes the (square of the) distance between two junction boxes.
fn dist(c1: JunctionBox, c2: JunctionBox) -> Dist {
    (c1.0 - c2.0) * (c1.0 - c2.0) + (c1.1 - c2.1) * (c1.1 - c2.1) + (c1.2 - c2.2) * (c1.2 - c2.2)
}

/// Returns the list of pairs of junction box indices, ordered according to the
/// distance between the corresponding junction boxes.
fn sorted_pairs(content: &Vec<JunctionBox>) -> Vec<(usize, usize)> {
    let mut matrix = Vec::new();
    for i in 0..content.len() {
        for j in i + 1..content.len() {
            matrix.push((i, j))
        }
    }
    matrix.sort_by_cached_key(|&(i, j)| dist(content[i], content[j]));
    matrix
}

fn part1(junction_boxes: &Vec<JunctionBox>, joins: usize) -> usize {
    let sorted_pairs = sorted_pairs(&junction_boxes);
    let mut g = UnionFind::new(junction_boxes.len());
    for &(j1, j2) in sorted_pairs.iter().take(joins) {
        g.union(j1, j2);
    }
    g.sorted_groups()
        .iter()
        .rev()
        .take(3)
        .map(|&x| g.sizes[x])
        .product()
}

fn part2(junction_boxes: &Vec<JunctionBox>) -> i64 {
    let sorted_pairs = sorted_pairs(&junction_boxes);
    let mut g = UnionFind::new(junction_boxes.len());
    for (p1, p2) in sorted_pairs {
        g.union(p1, p2);
        if g.groups == 1 {
            return junction_boxes[p1].0 * junction_boxes[p2].0;
        }
    }
    panic!("This shouldn't happen!");
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle8_example.txt", "\n", parse_junction_box);
    assert_eq!(part1(&content, 10), 40);
    assert_eq!(part2(&content), 25272);
    let content = read_file("inputs/puzzle8.txt", "\n", parse_junction_box);
    assert_eq!(part1(&content, 1000), 84968);
    assert_eq!(part2(&content), 8663467782);
}

fn main() {
    let content = read_file("inputs/puzzle8.txt", "\n", parse_junction_box);
    println!("Part 1: {}", part1(&content, 1000));
    println!("Part 2: {}", part2(&content));
}
