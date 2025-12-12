use std::collections::VecDeque;

use aoc2025::read_file;

const DEBUG: bool = false;

type Mask = u32;

#[derive(Debug)]
struct Machine {
    bitmap: Mask,
    buttons: Vec<Mask>,
    buttons2: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn parse_bitmap(s_bitmap: &str) -> Mask {
        s_bitmap.as_bytes()[1..s_bitmap.len() - 1]
            .iter()
            .enumerate()
            .map(|(pos, &ch)| ((ch == b'#') as Mask) << pos)
            .sum()
    }

    fn parse_button(s_button: &str) -> (Mask, Vec<usize>) {
        let data: Vec<usize> = s_button[1..s_button.len() - 1]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        let data_mask = data.iter().map(|v| 1 << v).sum();
        (data_mask, data)
    }

    fn parse_joltages(s_joltages: &str) -> Vec<u32> {
        s_joltages[1..s_joltages.len() - 1]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn parse(s_machine: &str) -> Self {
        let data: Vec<&str> = s_machine.split_whitespace().collect();
        let bitmap = Machine::parse_bitmap(data[0]);
        let (buttons, buttons2) = data[1..data.len() - 1]
            .into_iter()
            .map(|&str| Machine::parse_button(str))
            .unzip();
        let joltages = Machine::parse_joltages(data[data.len() - 1]);
        Self {
            bitmap,
            buttons,
            buttons2,
            joltages,
        }
    }
}

fn fewer_buttons_indicators(m: &Machine) -> u32 {
    let mut choices = VecDeque::from([(0 as Mask, 0u32)]);
    while let Some((v, b)) = choices.pop_front() {
        if v == m.bitmap {
            return b.count_ones();
        } else {
            for button_idx in 0..m.buttons.len() {
                if (1 << button_idx) & b == 0 {
                    choices.push_back((v ^ m.buttons[button_idx], b | (1 << button_idx)));
                }
            }
        }
    }
    panic!("No solution found");
}

#[allow(dead_code)]
fn fewer_buttons_joltage(m: &Machine) -> usize {
    let zero = vec![0u32; m.joltages.len()];
    let mut choices: VecDeque<(Vec<u32>, usize, usize)> = VecDeque::from([(zero, 0, 0)]);
    while let Some((v, b, last)) = choices.pop_front() {
        //println!("{:?} {:?} {:?}", v, b, choices);
        if v == m.joltages {
            return b;
        } else {
            if (0..v.len()).all(|i| v[i] <= m.joltages[i]) {
                for button_idx in last..m.buttons.len() {
                    let mut newv = v.clone();
                    for &x in &m.buttons2[button_idx] {
                        newv[x as usize] += 1;
                    }
                    choices.push_back((newv, b + 1, button_idx));
                }
            }
        }
    }
    panic!("No solution found");
}

fn find_promising_counter(buttons: &Vec<Vec<usize>>, joltages: &Vec<u32>) -> usize {
    //println!("find {buttons:?} {joltages:?}");
    joltages
        .iter()
        .enumerate()
        .filter(|&(_index, &x)| x > 0)
        .map(|(index, _x)| {
            (index, buttons
                .iter()
                .map(|b| b.contains(&index) as usize)
                .sum::<usize>())
        })
        .min_by_key(|&(_index, n)| n)
        .unwrap()
        .0
}

fn part1(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(fewer_buttons_indicators).sum()
}

fn possible_sums(num: usize, target: u32) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut data = vec![0; num];
    let mut first = vec![true; num];
    let mut i: usize = 0;
    let mut tot = 0;
    loop {
        if i < num - 1 {
            if first[i] {
                first[i] = false;
                i += 1;
            } else if tot < target {
                data[i] += 1;
                tot += 1;
                i += 1;
            } else {
                tot -= data[i];
                data[i] = 0;
                first[i] = true;
                if i == 0 {
                    return result;
                }
                i -= 1;

            }
        } else {
            data[i] = target - tot;
            result.push(data.clone());
            data[i] = 0;
            if i == 0 {
                    return result;
            }
            i -= 1;
        }

    }
}

fn button_press_to_joltages(
    button_selection: &Vec<&Vec<usize>>,
    joltages: &Vec<u32>,
    presses: &Vec<u32>,
) -> Option<Vec<u32>> {
    let mut data = joltages.clone();
    for (button_idx, &presses) in presses.iter().enumerate() {
        let button = button_selection[button_idx];
        for &counter in button {
            if data[counter] >= presses {
                data[counter] -= presses;
            } else {
                return None;
            }
        }
    }
    Some(data)
}

fn part2aux(buttons: &Vec<Vec<usize>>, joltages: &Vec<u32>) -> Option<u32> {
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?}"); }
    if joltages.iter().all(|&x| x == 0) {
        return Some(0)
    }
    let best_counter: usize = find_promising_counter(buttons, joltages);
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} best counter {best_counter}"); }
    let buttons_selection: Vec<&Vec<usize>> = buttons
        .iter()
        .filter(|&b| b.contains(&best_counter))
        .collect();
    if buttons_selection.is_empty() {
        return None;
    }
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} button selection {buttons_selection:?}"); }
    let remaining_buttons = buttons
        .iter()
        .filter(|&b| !b.contains(&best_counter))
        .map(|x| x.clone())
        .collect();
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} remaining buttons {remaining_buttons:?}"); }
    let target = joltages[best_counter];
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} target {target}"); }
    let choices = possible_sums(buttons_selection.len(), target);
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} choices {choices:?}"); }
    let new_joltages: Vec<Vec<u32>> = choices
        .iter()
        .map(|choice| button_press_to_joltages(&buttons_selection, joltages, choice))
        .filter(|x| *x != None)
        .map(|x| x.unwrap())
        .collect();
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} new_joltages {new_joltages:?}"); }
    let recursive: Vec<u32> = new_joltages
        .iter()
        .map(|j| part2aux(&remaining_buttons, j))
        .filter(|&x| x != None)
        .map(|x| x.unwrap())
        .collect();
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} recursive {recursive:?}"); }
    let result = recursive.iter().min().map(|x| x + target);
    if DEBUG { println!("buttons {buttons:?}  joltages {joltages:?} result {result:?}"); }
    result
}

fn part2(machines: &Vec<Machine>) -> u32 {
    //machines.iter().map(fewer_buttons_joltage).sum()
    machines.iter().map(|m| {
        println!("{m:?}");
        part2aux(&m.buttons2, &m.joltages).unwrap()
    }).sum()
}

#[test]
fn test() {
    let content = read_file("inputs/puzzle10_example.txt", "\n", Machine::parse);
    assert_eq!(part1(&content), 7);
    assert_eq!(part2(&content), 33);
    let content = read_file("inputs/puzzle10.txt", "\n", Machine::parse);
    assert_eq!(part1(&content), 509);
    assert_eq!(part2(&content), 20083);
}

fn main() {
    let content = read_file("inputs/puzzle10.txt", "\n", Machine::parse);
    println!("Part 1: {}", part1(&content));
    println!("Part 2: {}", part2(&content));
}
