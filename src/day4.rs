use regex::Regex;
use std::{fs, ops::RangeInclusive};

pub fn part1() {
    let answer: u16 = input()
        .lines()
        .map(parse_ranges)
        .map(|(a, b)| first_contain_second(&a, &b) || first_contain_second(&b, &a))
        .filter(|is_fully_contained| *is_fully_contained)
        .count() as u16;

    println!("day4, part1: {:?}", answer);
}

pub fn part2() {
    let answer: u16 = input()
        .lines()
        .map(parse_ranges)
        .map(|(first, second)| overlaps(first, second))
        .filter(|overlap| *overlap)
        .count() as u16;

    println!("day4, part2: {:?}", answer);
}

fn parse_ranges(line: &str) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
    let regex = Regex::new(r#"(\d+)\-(\d+),(\d+)\-(\d+)"#).unwrap();
    let groups = regex.captures(line).unwrap();

    return (
        groups[1].parse().unwrap()..=groups[2].parse().unwrap(),
        groups[3].parse().unwrap()..=groups[4].parse().unwrap(),
    );
}

fn first_contain_second(first: &RangeInclusive<u16>, second: &RangeInclusive<u16>) -> bool {
    first.contains(&second.start()) && first.contains(&second.end())
}

fn overlaps(first: RangeInclusive<u16>, second: RangeInclusive<u16>) -> bool {
    first.into_iter().any(|section| second.contains(&section))
}

#[cfg(test)]
mod test {
    use crate::day4::{first_contain_second, overlaps, parse_ranges};

    #[test]
    fn parse_ranges_boum() {
        assert_eq!(parse_ranges("62-64,4-63"), (62..=64, 4..=63))
    }

    #[test]
    fn first_contain_second_bim() {
        assert_eq!(first_contain_second(&(1..=1), &(4..=63)), false);
        assert_eq!(first_contain_second(&(1..=50), &(4..=63)), false);
        assert_eq!(first_contain_second(&(50..=100), &(4..=63)), false);
        assert_eq!(first_contain_second(&(50..=60), &(4..=63)), false);
        assert_eq!(first_contain_second(&(1..=100), &(4..=63)), true);
        assert_eq!(first_contain_second(&(1..=100), &(1..=100)), true);
    }

    #[test]
    fn overlaps_boum() {
        assert_eq!(overlaps(1..=1, 1..=1), true);
        assert_eq!(overlaps(1..=2, 2..=3), true);
        assert_eq!(overlaps(1..=3, 2..=3), true);
        assert_eq!(overlaps(1..=100, 1..=100), true);
        assert_eq!(overlaps(1..=100, 200..=300), false);
    }
}

fn input() -> String {
    fs::read_to_string("day4").unwrap()
}
