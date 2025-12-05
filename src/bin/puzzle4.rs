use aoc2025::read_file;
use std::cmp::min;

fn parse_row(row: &str) -> Vec<u8> {
    row.as_bytes().to_vec()
}

fn roll_is_free(maze: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let mut count = 0u32;
    let mini = if i == 0 { 0 } else { i-1 };
    let maxi = min(i+1, maze.len()-1);
    let minj = if j == 0 { 0 } else { j-1 };
    let maxj = min(j+1, maze[i].len()-1);
    for i1 in mini..=maxi {
        for j1 in minj..=maxj {
            if (i != i1 || j != j1) && maze[i1][j1] != b'.' {
                count += 1;
            }
        }
    }
    count < 4
}

fn mark_remove(maze: &mut Vec<Vec<u8>>, debug: bool) -> u32 {
    let mut count = 0;
    for i in  0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == b'@' {
                if roll_is_free(maze, i, j) {
                    maze[i][j] = b'x';
                    count += 1
                }
                if debug  {
                    print!("{}", if roll_is_free(maze, i, j) { 'x' } else { '@' });
                }
            } else {
                if debug { (print!(".")); }
            }
        }
        if  debug { println!(); }
    }
    count
}

fn remove_marked(maze: &mut Vec<Vec<u8>>) {
    for i in  0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == b'x' {
                maze[i][j] = b'.'
            }
        }
    }
}

fn part1(maze: &Vec<Vec<u8>>, debug: bool) -> u32 {
    let mut maze_copy: Vec<Vec<u8>> = maze.clone();
    mark_remove(&mut maze_copy, debug)
}

fn part2(maze: &Vec<Vec<u8>>, debug: bool) -> u32 {
    let mut maze_copy: Vec<Vec<u8>> = maze.clone();
    let mut count = 0;
    loop {
        if debug { println!("---- STEP ----") }
        let removed = mark_remove(&mut maze_copy, debug);
        if removed == 0 { break; }
        count += removed;
        remove_marked(&mut maze_copy);
    }
    count
}


#[test]
fn test() {
    let maze = read_file("inputs/puzzle4_example.txt", "\n", parse_row);
    assert_eq!(part1(&maze, false), 13);
    assert_eq!(part2(&maze, false), 43);
    let maze = read_file("inputs/puzzle4.txt", "\n", parse_row);
    assert_eq!(part1(&maze, false), 1569);
    assert_eq!(part2(&maze, false), 9280);
}

fn main() {
    let maze = read_file("inputs/puzzle4.txt", "\n", parse_row);
    println!("Part 1: {}", part1(&maze, false));
    println!("Part 2: {}", part2(&maze, false));
}
