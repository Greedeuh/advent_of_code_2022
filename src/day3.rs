use std::fs;

pub fn part1() {
    let answer: u16 = input()
        .lines()
        .map(split_by_compartment)
        .map(|(first, second)| find_error_between_compartment(first, second))
        .map(priority)
        .sum();

    println!("day3, part1: {:?}", answer);
}

pub fn part2() {
    let input = input();
    let answer: u16 = input
        .lines()
        .array_chunks()
        .map(|[first, second, third]| find_team(first, second, third))
        .map(priority)
        .sum();

    println!("day3, part2: {:?}", answer);
}

fn split_by_compartment<'a>(rucksack: &'a str) -> (&'a str, &'a str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn find_error_between_compartment<'a>(first: &str, second: &str) -> char {
    first
        .chars()
        .find(|item| second.contains(&item.to_string()))
        .unwrap()
}

fn find_team<'a>(first: &str, second: &str, third: &str) -> char {
    first
        .chars()
        .find(|item| {
            let item = item.to_string();
            second.contains(&item) && third.contains(&item)
        })
        .unwrap()
}

fn priority(item: char) -> u16 {
    let index = "abcdefghijklmnopqrstuvwxyz"
        .find(&item.to_lowercase().to_string())
        .unwrap() as u16
        + 1;

    if item.is_uppercase() {
        index + 26
    } else {
        index
    }
}

#[cfg(test)]
mod test {
    use super::{find_error_between_compartment, find_team, priority, split_by_compartment};

    #[test]
    fn split_by_compartment_boum() {
        assert_eq!(split_by_compartment("rucksack"), ("ruck", "sack"))
    }

    #[test]
    fn find_error_between_compartment_bim() {
        assert_eq!(find_error_between_compartment("ruck", "sacl"), 'c')
    }

    #[test]
    fn find_team_boum() {
        assert_eq!(find_team("ruck", "sacl", "bace"), 'c')
    }

    #[test]
    fn priority_boum() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('c'), 3);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('C'), 29);
    }
}

fn input() -> String {
    fs::read_to_string("day3").unwrap()
}
