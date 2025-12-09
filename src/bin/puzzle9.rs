use aoc2025::read_file;

type Point = (i64, i64);
type Segment = (Point, Point);
type Area = i64;

/// Parse a row of the file and return a point.
fn parse_point(row: &str) -> Point {
    let (x, y) = row.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

/// Compute the area of a rectangle, given its opposite corners.
fn area_rectangle((x1, y1): Point, (x2, y2): Point) -> Area {
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}

/// Compute the vector of segments which make the border of the polygon.
/// The extremal points of each segment are ordered according to the lexicographic
/// ordering that, given that all segments are either horizontal or vertical,
/// correspond to the coordinate-wise ordering.
fn corners_to_segments(corners: &Vec<Point>) -> Vec<Segment> {
    let mut result = Vec::new();
    let mut pred: (i64, i64) = corners[corners.len() - 1];
    for &curr in corners {
        // order the extremal points of the segments
        result.push((curr.min(pred), curr.max(pred)));
        pred = curr
    }
    result
}

/// Determine if a point is on the border of the polygon.
fn on_border(segments: &Vec<Segment>, (x, y): Point) -> bool {
    segments
        .iter()
        .any(|&((x1, y1), (x2, y2))| x1 <= x && x <= x2 && y1 <= y && y <= y2)
}

/// Compute the number of vertical segments crossed from a ray, starting at point `(x, y)`
/// and directed leftward. Only the first extremal point is considered part of the segment,
/// since this is the correct way to determin whether a point is inside the polygon.
///
/// It might be useful to memoize the results of this function.
fn count_x_crosses(segments: &Vec<Segment>, (x, y): Point) -> u64 {
    let mut cross_x = 0;
    for &((x1, y1), (x2, y2)) in segments {
        // note that t
        if x1 == x2 && x1 <= x && y1 <= y && y < y2 {
            cross_x += 1
        }
    }
    cross_x
}

/// Compute the number of vertical segments crossed from a ray, starting at point `(x, y)`
/// and directed leftward (1st componen of the result) and the number of horizontal points
/// crossed from a ray starting at the same position and directed upward. Extremal point
/// are not considered part of the segments.
///
/// It might be useful to memoize the results of this function.
fn count_crosses(segments: &Vec<Segment>, (x, y): Point) -> (u64, u64) {
    let mut cross_x = 0;
    let mut cross_y = 0;
    for &((x1, y1), (x2, y2)) in segments {
        if x1 == x2 && x1 <= x && y1 < y && y < y2 {
            cross_x += 1
        }
        if y1 == y2 && y1 <= y && x1 < x && x < x2 {
            cross_y += 1
        }
    }
    (cross_x, cross_y)
}

/// Determine whether `p` is inside the polygon or on the border of the polygon.
fn is_inside(segments: &Vec<Segment>, p: Point) -> bool {
    count_x_crosses(segments, p) % 2 == 1 || on_border(segments, p)
}

/// Determine if the square with opposite corners in `p1` and `p2` is inside
/// the polygon. This only works if there are no adjacent parallel segments
/// in the border of the polygon. This seems to be the case in the input data.
fn is_safe_rectangle(segments: &Vec<Segment>, p1: Point, p2: Point) -> bool {
    // determine corners of the rectangle
    let ul = (p1.0.min(p2.0), p1.1.min(p2.1));
    let ur = (p1.0.max(p2.0), p1.1.min(p2.1));
    let dr = (p1.0.max(p2.0), p1.1.max(p2.1));
    let dl = (p1.0.min(p2.0), p1.1.max(p2.1));

    // determine if all corners of the rectangle are inside the polygon
    let all_inside = is_inside(segments, ul)
        && is_inside(segments, ur)
        && is_inside(segments, dr)
        && is_inside(segments, dl);

    if !all_inside {
        return false;
    }

    // determine if sides of the rectangle do not cross the border of the polygon
    let no_crosses = count_crosses(segments, dl).1 == count_crosses(segments, ul).1
        && count_crosses(segments, dr).1 == count_crosses(segments, ur).1
        && count_crosses(segments, ur).0 == count_crosses(segments, ul).0
        && count_crosses(segments, ur).0 == count_crosses(segments, dl).0;

    no_crosses
}

/// Compute the rectangle with largest area present in the polygon
/// `corners`, subject to the safety condition `safety_check`.
fn largest_rectangle<T>(corners: &Vec<Point>, safety_check: T) -> Area
where
    T: Fn(Point, Point) -> bool,
{
    (0..corners.len())
        .into_iter()
        .map(|i| {
            (i + 1..corners.len())
                .into_iter()
                .filter(|&j| safety_check(corners[i], corners[j]))
                .map(|j| area_rectangle(corners[i], corners[j]))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
}

fn part1(corners: &Vec<Point>) -> Area {
    largest_rectangle(corners, |_p1, _p2| true)
}

fn part2(corners: &Vec<Point>) -> Area {
    let segments = corners_to_segments(corners);
    largest_rectangle(corners, |p1, p2| is_safe_rectangle(&segments, p1, p2))
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle9_example.txt", "\n", parse_point);
    assert_eq!(part1(&content), 50);
    assert_eq!(part2(&content), 24);
    let content = read_file("inputs/puzzle9.txt", "\n", parse_point);
    assert_eq!(part1(&content), 4782896435);
    assert_eq!(part2(&content), 1540060480);
}

fn main() {
    let content = read_file("inputs/puzzle9.txt", "\n", parse_point);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
