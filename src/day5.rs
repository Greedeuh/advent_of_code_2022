use regex::Regex;
use std::fs;

pub fn part1() {
    let (crates, instructions) = split_input(&input());
    let crates = parse_crates(&crates);
    let answer: String = instructions
        .split('\n')
        .into_iter()
        .map(parse_instruction)
        .fold(crates, apply_instruction)
        .into_iter()
        .map(|mut column| column.pop())
        .filter(|last| last.is_some())
        .map(|last| last.unwrap())
        .collect();

    println!("day5, part1: {:?}", answer);
}

pub fn part2() {
    let (crates, instructions) = split_input(&input());
    let crates = parse_crates(&crates);
    let answer: String = instructions
        .split('\n')
        .into_iter()
        .map(parse_instruction)
        .fold(crates, apply_insane_instruction)
        .into_iter()
        .map(|mut column| column.pop())
        .filter(|last| last.is_some())
        .map(|last| last.unwrap())
        .collect();

    println!("day5, part2: {:?}", answer);
}

fn split_input<'a>(input: &str) -> (String, String) {
    let regex = Regex::new(r"([\S\s]+)\n 1 .+\n([\S\s]+)").unwrap();
    let groups = regex.captures(input).unwrap();

    (groups[1].to_string(), groups[2].trim().to_string())
}

fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<Vec<char>>> = crates.split('\n').map(parse_crates_row).collect();

    let bottom_row: Vec<Vec<char>> = crates.pop().unwrap();

    crates.into_iter().rev().fold(bottom_row, |acc, next_row| {
        acc.into_iter()
            .enumerate()
            .map(|(index, mut column)| {
                column.extend(next_row.get(index).unwrap());
                column
            })
            .collect::<Vec<Vec<char>>>()
    })
}

fn parse_crates_row(crates_row: &str) -> Vec<Vec<char>> {
    let crates_row = format!("{} ", crates_row);
    crates_row
        .chars()
        .array_chunks()
        .map(|[_, second, _, _]| if second == ' ' { vec![] } else { vec![second] })
        .collect()
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let groups = regex.captures(instruction).unwrap();

    (
        groups[1].parse().unwrap(),
        groups[2].parse().unwrap(),
        groups[3].parse().unwrap(),
    )
}

fn apply_instruction(
    mut chars: Vec<Vec<char>>,
    (mov, from, to): (usize, usize, usize),
) -> Vec<Vec<char>> {
    let from = from - 1;
    let to = to - 1;

    for _ in 0..mov {
        // let c = chars[from].pop().unwrap();
        if let Some(c) = chars[from].pop() {
            chars[to].push(c);
        }
    }

    chars
}

fn apply_insane_instruction(
    mut chars: Vec<Vec<char>>,
    (mov, from, to): (usize, usize, usize),
) -> Vec<Vec<char>> {
    let from = from - 1;
    let to = to - 1;

    let moving_index = chars[from].len() - mov;
    let moving_crates: Vec<_> = chars[from].drain(moving_index..).collect();
    chars[to].extend(moving_crates);

    chars
}

#[cfg(test)]
mod test {
    use crate::day5::{
        apply_insane_instruction, apply_instruction, parse_crates_row, parse_instruction,
    };

    use super::{parse_crates, split_input};

    #[test]
    fn split_input_bim() {
        assert_eq!(
            split_input(
                "[L] [C] [W] [C] [P] [T] [M] [Z] [W]
 1   2   3   4   5   6   7   8   9 

move 6 from 6 to 5",
            ),
            (
                "[L] [C] [W] [C] [P] [T] [M] [Z] [W]".to_string(),
                "move 6 from 6 to 5".to_string()
            )
        );
        assert_eq!(
            split_input(
                "[L] [C] [W] [C] [P] [T] [M] [Z] [W]
[M] [C] [W] [C] [P] [T] [M] [Z] [W]
 1   2   3   4   5   6   7   8   9 

move 6 from 6 to 5",
            ),
            (
                "[L] [C] [W] [C] [P] [T] [M] [Z] [W]
[M] [C] [W] [C] [P] [T] [M] [Z] [W]"
                    .to_string(),
                "move 6 from 6 to 5".to_string()
            )
        );
    }

    #[test]
    fn parse_crates_row_boum() {
        assert_eq!(
            parse_crates_row("[L] [C] [W] [C] [P] [T] [M] [Z] [W]"),
            vec![
                vec!['L'],
                vec!['C'],
                vec!['W'],
                vec!['C'],
                vec!['P'],
                vec!['T'],
                vec!['M'],
                vec!['Z'],
                vec!['W']
            ]
        );
        assert_eq!(
            parse_crates_row("    [C] [W] [C] [P] [T] [M] [Z] [W]"),
            vec![
                vec![],
                vec!['C'],
                vec!['W'],
                vec!['C'],
                vec!['P'],
                vec!['T'],
                vec!['M'],
                vec!['Z'],
                vec!['W']
            ]
        )
    }

    #[test]
    fn parse_crates_bim() {
        assert_eq!(
            parse_crates("[L] [C] [W] [C] [P] [T] [M] [Z] [W]"),
            vec![
                vec!['L'],
                vec!['C'],
                vec!['W'],
                vec!['C'],
                vec!['P'],
                vec!['T'],
                vec!['M'],
                vec!['Z'],
                vec!['W']
            ]
        );
        assert_eq!(
            parse_crates(
                "[L] [C] [W] [C] [P] [T] [M] [Z] [W]
[W] [L] [C] [W] [C] [P] [T] [M] [Z]"
            ),
            vec![
                vec!['W', 'L'],
                vec!['L', 'C'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );

        assert_eq!(
            parse_crates(
                "    [C] [W] [C] [P] [T] [M] [Z] [W]
[W] [L] [C] [W] [C] [P] [T] [M] [Z]"
            ),
            vec![
                vec!['W'],
                vec!['L', 'C'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );
    }

    #[test]
    fn parse_instruction_bam() {
        assert_eq!(parse_instruction("move 4 from 3 to 7"), (4, 3, 7));
    }

    #[test]
    fn apply_instruction_bim() {
        assert_eq!(
            apply_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (1, 1, 1)
            ),
            vec![
                vec!['W', 'L'],
                vec!['L', 'C'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );

        assert_eq!(
            apply_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (1, 1, 2)
            ),
            vec![
                vec!['W'],
                vec!['L', 'C', 'L'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );

        assert_eq!(
            apply_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (2, 1, 2)
            ),
            vec![
                vec![],
                vec!['L', 'C', 'L', 'W'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );
    }

    #[test]
    fn apply_insane_instruction_bim() {
        assert_eq!(
            apply_insane_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (1, 1, 1)
            ),
            vec![
                vec!['W', 'L'],
                vec!['L', 'C'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );

        assert_eq!(
            apply_insane_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (1, 1, 2)
            ),
            vec![
                vec!['W'],
                vec!['L', 'C', 'L'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );

        assert_eq!(
            apply_insane_instruction(
                vec![
                    vec!['W', 'L'],
                    vec!['L', 'C'],
                    vec!['C', 'W'],
                    vec!['W', 'C'],
                    vec!['C', 'P'],
                    vec!['P', 'T'],
                    vec!['T', 'M'],
                    vec!['M', 'Z'],
                    vec!['Z', 'W']
                ],
                (2, 1, 2)
            ),
            vec![
                vec![],
                vec!['L', 'C', 'W', 'L'],
                vec!['C', 'W'],
                vec!['W', 'C'],
                vec!['C', 'P'],
                vec!['P', 'T'],
                vec!['T', 'M'],
                vec!['M', 'Z'],
                vec!['Z', 'W']
            ]
        );
    }
}

fn input() -> String {
    fs::read_to_string("day5").unwrap()
}
