use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Read};

#[derive(Clone)]
struct VariantBase {
    cells: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

#[derive(Clone)]
struct RegionVariant {
    offsets: Vec<usize>,
    w: usize,
    h: usize,
}

fn generate_variants(cells: &[(i32, i32)]) -> Vec<VariantBase> {
    if cells.is_empty() {
        return Vec::new();
    }

    let mut set = HashSet::new();
    let mut result = Vec::new();

    let transforms: Vec<Box<dyn Fn(i32, i32) -> (i32, i32)>> = vec![
        Box::new(|x, y| (x, y)),
        Box::new(|x, y| (x, -y)),
        Box::new(|x, y| (-x, y)),
        Box::new(|x, y| (-x, -y)),
        Box::new(|x, y| (y, x)),
        Box::new(|x, y| (y, -x)),
        Box::new(|x, y| (-y, x)),
        Box::new(|x, y| (-y, -x)),
    ];

    for t in transforms {
        let mut transformed: Vec<(i32, i32)> = cells.iter().map(|&(x, y)| t(x, y)).collect();
        // Normalize: shift so min x,y is 0,0
        let mut min_x = transformed[0].0;
        let mut min_y = transformed[0].1;
        for &(x, y) in &transformed {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
        }
        for p in &mut transformed {
            p.0 -= min_x;
            p.1 -= min_y;
        }
        transformed.sort();
        let key = transformed
            .iter()
            .map(|(x, y)| format!("{},{}", x, y))
            .collect::<Vec<_>>()
            .join(";");
        if set.insert(key) {
            // New unique variant
            let mut max_x = 0;
            let mut max_y = 0;
            for &(x, y) in &transformed {
                max_x = max(max_x, x);
                max_y = max(max_y, y);
            }
            result.push(VariantBase {
                cells: transformed,
                width: max_x + 1,
                height: max_y + 1,
            });
        }
    }

    result
}

fn dfs(
    piece_idx: usize,
    used_cells: usize,
    board: &mut [bool],
    pieces: &[usize],
    suffix_area: &[usize],
    region_variants: &Vec<Vec<RegionVariant>>,
    shape_area: &[usize],
    w: usize,
    h: usize,
) -> bool {
    if piece_idx == pieces.len() {
        return true;
    }
    let free_cells = w * h - used_cells;
    if suffix_area[piece_idx] > free_cells {
        return false;
    }

    let kind = pieces[piece_idx];
    let variants = &region_variants[kind];
    if variants.is_empty() {
        // Shape has no realizable variant (shouldn't happen if area > 0), but if it does and we
        // need to place one, it's impossible.
        return false;
    }

    let area = shape_area[kind];

    for var in variants {
        let max_oy = h - var.h;
        let max_ox = w - var.w;

        for oy in 0..=max_oy {
            let base_row = oy * w;
            for ox in 0..=max_ox {
                let base = base_row + ox;
                // Check if can place
                let mut ok = true;
                for &off in &var.offsets {
                    let idx = base + off;
                    if board[idx] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                // Place
                for &off in &var.offsets {
                    board[base + off] = true;
                }
                if dfs(
                    piece_idx + 1,
                    used_cells + area,
                    board,
                    pieces,
                    suffix_area,
                    region_variants,
                    shape_area,
                    w,
                    h,
                ) {
                    return true;
                }
                // Unplace
                for &off in &var.offsets {
                    board[base + off] = false;
                }
            }
        }
    }

    false
}

fn can_fit_region(
    w: usize,
    h: usize,
    counts: &[usize],
    all_variants: &Vec<Vec<VariantBase>>,
    shape_area: &[usize],
) -> bool {
    let shapes_len = all_variants.len();
    let mut total_area: usize = 0;
    for k in 0..shapes_len.min(counts.len()) {
        if shape_area[k] == 0 && counts[k] > 0 {
            return false;
        }
        total_area += counts[k] * shape_area[k];
    }
    if total_area > w * h {
        return false;
    }

    // Build list of piece instances
    let mut pieces: Vec<usize> = Vec::new();
    for k in 0..shapes_len.min(counts.len()) {
        for _ in 0..counts[k] {
            pieces.push(k);
        }
    }

    if pieces.is_empty() {
        // No presents to place: always possible
        return true;
    }

    // Sort pieces by descending area (bigger pieces first)
    pieces.sort_by_key(|&k| std::cmp::Reverse(shape_area[k]));

    // Precompute suffix sums of areas
    let mut suffix_area = vec![0usize; pieces.len() + 1];
    for i in (0..pieces.len()).rev() {
        suffix_area[i] = suffix_area[i + 1] + shape_area[pieces[i]];
    }

    // Build region-specific variants (convert (x,y) -> offset)
    let mut region_variants: Vec<Vec<RegionVariant>> = vec![Vec::new(); shapes_len];
    for k in 0..shapes_len {
        let mut rv = Vec::new();
        for vb in &all_variants[k] {
            let mut offsets = Vec::new();
            for &(x, y) in &vb.cells {
                let dx = x as usize;
                let dy = y as usize;
                offsets.push(dy * w + dx);
            }
            rv.push(RegionVariant {
                offsets,
                w: vb.width as usize,
                h: vb.height as usize,
            });
        }
        region_variants[k] = rv;
    }

    let mut board = vec![false; w * h];

    dfs(
        0,
        0,
        &mut board,
        &pieces,
        &suffix_area,
        &region_variants,
        shape_area,
        w,
        h,
    )
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.trim_end().to_string()).collect();

    // Parse shapes
    let mut i = 0usize;
    let mut raw_shapes: Vec<Vec<String>> = Vec::new();
    let mut region_start = lines.len();

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Region line detection: something like "WxH: ..."
        if line.contains('x') && line.contains(':') {
            region_start = i;
            break;
        }

        // Shape header: "N:"
        let mut parts = line.split(':');
        let idx_str = parts
            .next()
            .expect("Shape line should have an index before ':'")
            .trim();
        let idx: usize = idx_str.parse().expect("Invalid shape index");
        if raw_shapes.len() <= idx {
            raw_shapes.resize(idx + 1, Vec::new());
        }

        i += 1;
        let mut rows: Vec<String> = Vec::new();
        while i < lines.len() {
            let l = lines[i].trim();
            if l.is_empty() {
                i += 1;
                break;
            }
            // Grid rows are '#', '.' etc.
            rows.push(l.to_string());
            i += 1;
        }
        raw_shapes[idx] = rows;
    }

    let shape_count = raw_shapes.len();

    // Build cell lists and variants
    let mut base_cells: Vec<Vec<(i32, i32)>> = Vec::with_capacity(shape_count);
    let mut shape_area: Vec<usize> = Vec::with_capacity(shape_count);
    for rows in &raw_shapes {
        let mut cells = Vec::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cells.push((x as i32, y as i32));
                }
            }
        }
        shape_area.push(cells.len());
        base_cells.push(cells);
    }

    let mut all_variants: Vec<Vec<VariantBase>> = Vec::with_capacity(shape_count);
    for cells in &base_cells {
        let variants = generate_variants(cells);
        all_variants.push(variants);
    }

    // Parse regions
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    for j in region_start..lines.len() {
        let line = lines[j].trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(':');
        let dims = parts
            .next()
            .expect("Region line should have dimensions before ':'")
            .trim();
        let counts_str = parts
            .next()
            .unwrap_or("")
            .trim();

        let mut dims_parts = dims.split('x');
        let w_str = dims_parts
            .next()
            .expect("Missing width in region dimensions")
            .trim();
        let h_str = dims_parts
            .next()
            .expect("Missing height in region dimensions")
            .trim();
        let w: usize = w_str.parse().expect("Invalid region width");
        let h: usize = h_str.parse().expect("Invalid region height");

        let mut counts: Vec<usize> = Vec::new();
        if !counts_str.is_empty() {
            for tok in counts_str.split_whitespace() {
                let v: usize = tok.parse().expect("Invalid count in region line");
                counts.push(v);
            }
        }

        // If there are fewer counts than shapes, extend with zeros
        if counts.len() < shape_count {
            counts.resize(shape_count, 0);
        }

        regions.push((w, h, counts));
    }

    let mut fit_count = 0usize;

    for (w, h, counts) in &regions {
        if can_fit_region(*w, *h, counts, &all_variants, &shape_area) {
            fit_count += 1;
        }
    }

    // Part 1: number of regions that can fit all presents
    // Part 2: problem statement doesn't add a new computation, so we output the same value.
    println!("{}", fit_count);
    println!("{}", fit_count);
}
