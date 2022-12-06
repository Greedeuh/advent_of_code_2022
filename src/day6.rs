use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs;

pub fn part1() {
    let answer = find_marker(&input());
    println!("day5, part1: {:?}", answer);
}

pub fn part2() {
    println!("day5, part2: {:?}", "answer");
}

fn find_marker(input: &str) -> (usize, String) {
    input
        .chars()
        .fold_while((0, "".to_string()), |(index, mut potential_marker), c| {
            let index = index + 1;
            let potential_marker = if let Some(index_of) = potential_marker.find(c) {
                format!(
                    "{}{}",
                    potential_marker.drain((index_of + 1)..).collect::<String>(),
                    c
                )
            } else {
                format!("{}{}", potential_marker, c)
            };

            if potential_marker.len() == 4 {
                Done((index, potential_marker))
            } else {
                Continue((index, potential_marker))
            }
        })
        .into_inner()
}

#[cfg(test)]
mod test {

    use super::find_marker;

    #[test]
    fn find_marker_bim() {
        assert_eq!(find_marker("azer"), (4, "azer".to_string()));
        assert_eq!(find_marker("aazer"), (5, "azer".to_string()));
        assert_eq!(find_marker("azeazer"), (7, "azer".to_string()));
        assert_eq!(find_marker("azeazerazer"), (7, "azer".to_string()));
        assert_eq!(find_marker("azeazerqsdf"), (7, "azer".to_string()));
        assert_eq!(find_marker("srlsrsnnwh"), (10, "nwh".to_string()));
    }
}

fn input() -> String {
    fs::read_to_string("day6").unwrap()
}
