use std::cmp::{max, min};
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse red tile coordinates
    let mut points: Vec<(i64, i64)> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split(',');
        let xs = it.next().unwrap().trim();
        let ys = it.next().unwrap().trim();
        let x: i64 = xs.parse().unwrap();
        let y: i64 = ys.parse().unwrap();
        points.push((x, y));
    }

    let n = points.len();
    if n < 2 {
        // Degenerate case
        println!("0");
        println!("0");
        return;
    }

    // ---------------- Part 1 ----------------
    let mut max_area1: i64 = 0;
    for i in 0..n {
        let (x1, y1) = points[i];
        for j in (i + 1)..n {
            let (x2, y2) = points[j];
            if x1 == x2 || y1 == y2 {
                continue; // would have zero area
            }
            let dx = (x1 - x2).abs();
            let dy = (y1 - y2).abs();
            let area = dx * dy;
            if area > max_area1 {
                max_area1 = area;
            }
        }
    }

    // ---------------- Part 2 ----------------
    // Build the red+green region:
    //  - red tiles are given points
    //  - green tiles form a loop along axis-aligned segments between consecutive red tiles
    //  - all tiles inside this loop are green as well

    // Bounding box of all red tiles
    let mut min_x = points[0].0;
    let mut max_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    for &(x, y) in &points {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    // Grid indices:
    //   original tile (x, y) -> inside bounding box indices:
    //     gx = (x - min_x) as usize in [0, width-1]
    //     gy = (y - min_y) as usize in [0, height-1]
    //
    // For flood fill we use extended grid with a 1-tile margin:
    //   extended size: (width + 2) x (height + 2)
    //   extended coords:
    //       ex = gx + 1
    //       ey = gy + 1

    let w2 = width + 2;
    let h2 = height + 2;
    let size2 = w2 * h2;

    // boundary: true for tiles that lie on the red+green loop (ring of tiles)
    let mut boundary = vec![false; size2];

    let idx2 = |x: usize, y: usize, w2: usize| -> usize { y * w2 + x };

    // Mark boundary tiles along each axis-aligned segment between consecutive red tiles
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        if x1 == x2 {
            // Vertical segment
            let x = x1;
            let sy = min(y1, y2);
            let ey = max(y1, y2);
            for y in sy..=ey {
                let gx = (x - min_x + 1) as usize;
                let gy = (y - min_y + 1) as usize;
                let id = idx2(gx, gy, w2);
                boundary[id] = true;
            }
        } else if y1 == y2 {
            // Horizontal segment
            let y = y1;
            let sx = min(x1, x2);
            let ex = max(x1, x2);
            for x in sx..=ex {
                let gx = (x - min_x + 1) as usize;
                let gy = (y - min_y + 1) as usize;
                let id = idx2(gx, gy, w2);
                boundary[id] = true;
            }
        } else {
            // Problem guarantees this shouldn't happen (always axis-aligned)
            panic!("Non-axis-aligned segment in input");
        }
    }

    // Flood fill from "outside" on the extended grid, treating boundary tiles as walls.
    // Any non-boundary tile reachable from the outer frame is "outside".
    let mut visited = vec![false; size2];
    let mut q = VecDeque::<(usize, usize)>::new();

    // Initialize queue with all non-boundary tiles on the outer frame
    for x in 0..w2 {
        for &y in &[0usize, h2 - 1] {
            let id = idx2(x, y, w2);
            if !boundary[id] && !visited[id] {
                visited[id] = true;
                q.push_back((x, y));
            }
        }
    }
    for y in 0..h2 {
        for &x in &[0usize, w2 - 1] {
            let id = idx2(x, y, w2);
            if !boundary[id] && !visited[id] {
                visited[id] = true;
                q.push_back((x, y));
            }
        }
    }

    // BFS on 4-connected grid
    while let Some((x, y)) = q.pop_front() {
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for &(nx, ny) in &neighbors {
            if nx >= w2 || ny >= h2 {
                continue;
            }
            let nid = idx2(nx, ny, w2);
            if visited[nid] || boundary[nid] {
                continue;
            }
            visited[nid] = true;
            q.push_back((nx, ny));
        }
    }

    // Now all tiles in the extended grid that are NOT visited are either
    //  - boundary tiles, or
    //  - interior tiles.
    // Those (restricted back to the original bounding box) are the red+green tiles.
    let mut allowed = vec![false; width * height]; // allowed[y * width + x]

    for gy in 0..height {
        for gx in 0..width {
            let ex = gx + 1;
            let ey = gy + 1;
            let id = idx2(ex, ey, w2);
            if !visited[id] {
                // inside loop (boundary or interior): red or green
                allowed[gy * width + gx] = true;
            }
        }
    }

    // Build 2D prefix sums over "allowed" to quickly test if a rectangle is fully inside.
    // ps has dimensions (height+1) x (width+1)
    let pw = width + 1;
    let ph = height + 1;
    let mut ps = vec![0i64; pw * ph];

    let idx_ps = |x: usize, y: usize, pw: usize| -> usize { y * pw + x };

    for y in 0..height {
        for x in 0..width {
            let val = if allowed[y * width + x] { 1i64 } else { 0i64 };
            let a = idx_ps(x + 1, y + 1, pw);
            let b = idx_ps(x + 1, y, pw);
            let c = idx_ps(x, y + 1, pw);
            let d = idx_ps(x, y, pw);
            ps[a] = val + ps[b] + ps[c] - ps[d];
        }
    }

    // Helper to get sum of allowed tiles in rectangle [x0..x1], [y0..y1], inclusive
    let rect_sum = |x0: usize, y0: usize, x1: usize, y1: usize, ps: &Vec<i64>, pw: usize| -> i64 {
        let a = (y1 + 1) * pw + (x1 + 1);
        let b = y0 * pw + (x1 + 1);
        let c = (y1 + 1) * pw + x0;
        let d = y0 * pw + x0;
        ps[a] - ps[b] - ps[c] + ps[d]
    };

    let mut max_area2: i64 = 0;

    for i in 0..n {
        let (x1, y1) = points[i];
        for j in (i + 1)..n {
            let (x2, y2) = points[j];
            if x1 == x2 || y1 == y2 {
                continue; // zero-area rectangle
            }

            let left = min(x1, x2);
            let right = max(x1, x2);
            let bottom = min(y1, y2);
            let top = max(y1, y2);

            let gx0 = (left - min_x) as usize;
            let gx1 = (right - min_x) as usize;
            let gy0 = (bottom - min_y) as usize;
            let gy1 = (top - min_y) as usize;

            let w_rect = (gx1 - gx0 + 1) as i64;
            let h_rect = (gy1 - gy0 + 1) as i64;
            let area_rect = w_rect * h_rect;

            let sum_allowed = rect_sum(gx0, gy0, gx1, gy1, &ps, pw);

            if sum_allowed == area_rect {
                if area_rect > max_area2 {
                    max_area2 = area_rect;
                }
            }
        }
    }

    println!("{}", max_area1);
    println!("{}", max_area2);
}
