use aoc2025::read_file;
use std::{
    collections::HashMap,
    fmt,
    ops::{Deref, DerefMut},
};

const DEBUG: bool = true;

/// A Bitmap used for both the shapes and the regions.
///
/// The implementation is quite inefficient (a vector of vectors of booleans).
#[derive(Clone, Hash, PartialEq, Eq)]
struct Bitmap(Vec<Vec<bool>>);

impl Deref for Bitmap {
    type Target = Vec<Vec<bool>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for &x in row {
                write!(f, "{}", if x { "#" } else { "." })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Bitmap {
    // Return an empty bitmap of the specified sizes.
    fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![false; width]; height])
    }

    /// Compute the area, i.e., the number of filled points in the bitmap.
    fn area(&self) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|&x| *x).count())
            .sum()
    }

    /// Return the width and height of a field.
    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        self[0].len()
    }

    /// Flip the bitmap vertically.
    fn flip_vert(&self) -> Self {
        let mut result = self.clone();
        let size = self.height();
        for i in 0..size / 2 {
            result.swap(i, size - 1 - i);
        }
        result
    }

    /// Rotate the field to the left
    fn rotate_left(&self) -> Self {
        let mut result = Bitmap(vec![vec![false; self.height()]; self.width()]);
        for i in 0..self.width() {
            for j in 0..self.height() {
                result[i][j] = self[j][self.width() - 1 - i];
            }
        }
        result
    }

    /// Return a vector of fields derived through rotations and flipping
    fn augmented(&self) -> Vec<Self> {
        vec![
            self.clone(),
            self.rotate_left(),
            self.rotate_left().rotate_left(),
            self.rotate_left().rotate_left().rotate_left(),
            self.flip_vert(),
            self.flip_vert().rotate_left(),
            self.flip_vert().rotate_left().rotate_left(),
            self.flip_vert().rotate_left().rotate_left().rotate_left(),
        ]
    }

    fn parse_field_row(row: &str) -> Vec<bool> {
        row.chars().map(|ch| ch == '#').collect()
    }

    /// Convert a slice of strings into a bitmap.
    fn parse(content: &[String]) -> Self {
        Self(
            content
                .iter()
                .skip(1) //  skip first row
                .map(|s| Bitmap::parse_field_row(s))
                .collect(),
        )
    }

    /// Determine all the way it is possible to add the shape `additional` in the current
    /// bitmap without collisions.
    fn merge(&self, additional: &Bitmap) -> Vec<Bitmap> {
        let mut result = Vec::new();
        for i in 0..self.height() - additional.height() + 1 {
            for j in 0..self.width() - additional.width() + 1 {
                let mut ok = true;
                'shape: for di in 0..3 {
                    for dj in 0..3 {
                        if additional[di][dj] && self[i + di][j + dj] {
                            ok = false;
                            break 'shape;
                        }
                    }
                }
                if ok {
                    let mut bitmapnew = self.clone();
                    for di in 0..3 {
                        for dj in 0..3 {
                            bitmapnew[i + di][j + dj] |= additional[di][dj];
                        }
                    }
                    result.push(bitmapnew);
                }
            }
        }
        result
    }
}

/// A Shape of the quiz. A shape collects in a single object all the bitmaps obtained
/// by rotating and flipping the original problem's shapes, and the area for each of
/// them.
struct Shape {
    bitmaps: Vec<Bitmap>,
    area: usize,
}

impl Shape {
    /// Create a shape from its original bitmap.
    fn new(bitmap: Bitmap) -> Self {
        let area = bitmap.area();
        let bitmaps = bitmap.augmented();
        Self { bitmaps, area }
    }

    /// Convert a slice of strings into a shape.
    fn parse(content: &[String]) -> Self {
        Self::new(Bitmap::parse(content))
    }
}

/// A region is composed of two sizes (width and heigh) and a list of integers representing
/// how many shapes of each type need to be positional in the region.
#[derive(Debug)]

struct Region {
    width: usize,
    height: usize,
    requirements: Vec<usize>,
}

impl Region {
    /// Convert a string into a region.
    fn parse(row: &str) -> Self {
        let (sizes, shapes) = row.split_once(":").unwrap();
        let (width, height) = sizes.split_once("x").unwrap();
        let requirements = shapes
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            requirements: requirements,
        }
    }

    /// Determine if a region is feasible by just comparing its area with the sum of the area of
    /// the shapes it should contain. This is a necessary but not sufficient condition for the
    /// region to be feasible. However, the only case where this test is positive and the
    /// region is not feasible seems to be the third region in the example input.
    fn is_feasible_fast(&self, shapes: &Vec<Shape>) -> bool {
        let total_area: usize = self
            .requirements
            .iter()
            .enumerate()
            .map(|(s, val)| shapes[s].area * val)
            .sum();
        total_area < self.width * self.height
    }

    /// Determine if the region is feasible looking for a solution, i.e. a way to accomodate the
    /// shapes inside the region. This is much slower, and essentially useless, because all the
    /// feasible region in the input file are classified correctly by the `is_feasible_fast`
    /// method. However, this is still used for two reasons:
    ///   1. check that the solution declared feasible by `is_feasible_fast` is actually feasible;
    ///   2. correctly declare the third region in the example input as not feasible.
    fn is_feasible_slow(&self, shapes: &Vec<Shape>) -> bool {
        if DEBUG {
            println!("{self:?}");
        }
        let bitmap = Bitmap::new(self.width, self.height);
        let shape_list = self
            .requirements
            .iter()
            .enumerate()
            .flat_map(|(i, s)| vec![i; *s as usize])
            .collect();
        Region::is_feasible_slow_aux(shapes, &shape_list, 0, &bitmap, &mut HashMap::new())
    }

    /// Auxiliary function used in `is_feasible_slow`. Note that caching is only useful for the third
    /// region of the example input, and might be removed.
    fn is_feasible_slow_aux(
        shapes: &Vec<Shape>,
        shape_list: &Vec<usize>,
        i: usize,
        region_bitmap: &Bitmap,
        memo: &mut HashMap<(usize, Bitmap), bool>,
    ) -> bool {
        if i >= shape_list.len() {
            return true;
        }
        if let Some(&v) = memo.get(&(i, region_bitmap.clone())) {
            return v;
        }
        for shape_bitmap in &shapes[shape_list[i]].bitmaps {
            for new_bitmap in region_bitmap.merge(shape_bitmap) {
                if Region::is_feasible_slow_aux(shapes, shape_list, i + 1, &new_bitmap, memo) {
                    memo.insert((i, region_bitmap.clone()), true);
                    return true;
                }
            }
        }
        memo.insert((i, region_bitmap.clone()), false);
        false
    }
}

// The problem is a pair made of the list of shapes and the list of regions.
struct Problem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Problem {
    /// Parse the input file. The implementation si quite ad-hox, assuming the presence of six shapes and
    /// a single whiteline between shapes and between the last shape and the first region.
    fn parse(content: &[String]) -> Self {
        let shapes = (0..6)
            .map(|i: usize| Shape::parse(&content[5 * i..5 * i + 4]))
            .collect();
        let regions = (30..content.len())
            .map(|i| Region::parse(&content[i]))
            .collect();
        Self { shapes, regions }
    }
}

fn part1(content: &[String]) -> usize {
    let problem = Problem::parse(&content);
    problem
        .regions
        .iter()
        .filter(|region| {
            region.is_feasible_fast(&problem.shapes) && region.is_feasible_slow(&problem.shapes)
        })
        .count()
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle12_example.txt", "\n", str::to_string);
    assert_eq!(part1(&content), 2);
    let content = read_file("inputs/puzzle12.txt", "\n", str::to_string);
    assert_eq!(part1(&content), 595);
}

fn main() {
    let content = read_file("inputs/puzzle12.txt", "\n", str::to_string);
    println!("Part 1: {}", part1(&content));
}
