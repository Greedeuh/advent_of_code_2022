use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs, slice::SliceIndex};

lazy_static! {
    static ref CD_REGEX: Regex = Regex::new(r"\$ cd (.+)").unwrap();
    static ref LS_REGEX: Regex = Regex::new(r"\$ ls").unwrap();
    static ref DIR_REGEX: Regex = Regex::new(r"dir (.+)").unwrap();
    static ref FILE_REGEX: Regex = Regex::new(r"(\d+) (.+)").unwrap();
    static ref PREVIOUS_DIR_REGEX: Regex = Regex::new(r"(.+)/.+").unwrap();
}

pub fn part1() -> usize {
    part1_run(&input())
}
fn part1_run(input: &str) -> usize {
    let resulting_line: Vec<(String, Line)> = Vec::new();
    let (_, resulting_line) = input
        .split('\n')
        .into_iter()
        .map(str::trim)
        .map(Line::from)
        .fold(
            ("root".to_string(), resulting_line),
            |(current_dir, mut resulting_lines), line| {
                let next_dir = associate_dir(&current_dir, &line);

                resulting_lines.push((next_dir.clone(), line));
                (next_dir, resulting_lines)
            },
        );
    let files = resulting_line.into_iter().filter_map(|(dir, line)| {
        if let Line::File(size, ..) = line {
            Some((dir, size))
        } else {
            None
        }
    });
    let dirs = files.flat_map(|(dir, size)| all_dirs(&dir).into_iter().map(move |dir| (dir, size)));
    let dirs = dirs
        .fold(HashMap::<String, usize>::new(), |map, (dir, size)| {
            size_by_dir(map, dir, size)
        })
        .into_iter();
    let answer: usize = dirs
        .filter(|(_, size)| size <= &100000)
        .map(|(_, size)| size)
        .sum();
    println!("day5, part1: {:?}", answer);
    answer
}

pub fn part2() {
    let resulting_line: Vec<(String, Line)> = Vec::new();
    let (_, resulting_line) = input()
        .split('\n')
        .into_iter()
        .map(str::trim)
        .map(Line::from)
        .fold(
            ("root".to_string(), resulting_line),
            |(current_dir, mut resulting_lines), line| {
                let next_dir = associate_dir(&current_dir, &line);

                resulting_lines.push((next_dir.clone(), line));
                (next_dir, resulting_lines)
            },
        );
    let files = resulting_line.into_iter().filter_map(|(dir, line)| {
        if let Line::File(size, ..) = line {
            Some((dir, size))
        } else {
            None
        }
    });
    let dirs = files.flat_map(|(dir, size)| all_dirs(&dir).into_iter().map(move |dir| (dir, size)));
    let dirs = dirs.fold(HashMap::<String, usize>::new(), |map, (dir, size)| {
        size_by_dir(map, dir, size)
    });

    let unused_space = 70000000 - dirs["root"];
    let space_to_find = 30000000 - unused_space;

    let mut dirs_size = dirs
        .into_iter()
        .map(|(_, size)| size as isize)
        .collect::<Vec<_>>();
    dirs_size.sort_by(|size_a, size_b| {
        positive_close_to_0(
            size_a - space_to_find as isize,
            size_b - space_to_find as isize,
        )
    });

    let answer = dirs_size.first();
    println!("day5, part1: {:?}", answer);
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(usize, String),
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        if let Some(captures) = CD_REGEX.captures(value) {
            Self::Cd(captures[1].to_string())
        } else if LS_REGEX.is_match(value) {
            Self::Ls
        } else if let Some(captures) = DIR_REGEX.captures(value) {
            Self::Dir(captures[1].to_string())
        } else if let Some(captures) = FILE_REGEX.captures(value) {
            Self::File(captures[1].parse().unwrap(), captures[2].to_string())
        } else {
            panic!("parsing line \"{}\" failed", value)
        }
    }
}

fn associate_dir(current_dir: &str, line: &Line) -> String {
    match &line {
        Line::Cd(dir) if dir == "/" => "root".to_string(),
        Line::Cd(dir) if dir == ".." => {
            PREVIOUS_DIR_REGEX.captures(current_dir).unwrap()[1].to_string()
        }
        Line::Cd(dir) => format!("{}/{}", current_dir, dir),
        _ => current_dir.to_string(),
    }
}

fn all_dirs(dir: &str) -> Vec<String> {
    let mut dirs = vec!["root".to_string()];

    let mut current_dir = dir.to_string();
    while let Some(captures) = PREVIOUS_DIR_REGEX.captures(&current_dir) {
        dirs.push(current_dir.clone());
        current_dir = captures[1].to_string();
    }

    dirs
}

fn size_by_dir(
    mut map: HashMap<String, usize>,
    dir: String,
    size: usize,
) -> HashMap<String, usize> {
    if let Some(computed_size) = map.get_mut(&dir) {
        *computed_size += size;
    } else {
        map.insert(dir, size);
    }
    map
}

fn positive_close_to_0(a: isize, b: isize) -> Ordering {
    if a < 0 {
        return Ordering::Greater;
    }
    if b < 0 {
        return Ordering::Less;
    }
    a.cmp(&b)
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use crate::day7::{all_dirs, associate_dir, part1_run, size_by_dir, Line};

    use super::positive_close_to_0;

    #[test]
    fn parse_line_bim() {
        assert_eq!(Line::from("$ cd boum"), Line::Cd("boum".to_string()));
        assert_eq!(Line::from("$ ls"), Line::Ls);
        assert_eq!(Line::from("dir bim"), Line::Dir("bim".to_string()));
        assert_eq!(
            Line::from("1996 boum"),
            Line::File(1996, "boum".to_string())
        );
    }

    #[test]
    fn associate_dir_boum() {
        assert_eq!(
            associate_dir("root", &Line::Cd("boum".to_string())),
            "root/boum".to_string()
        );
        assert_eq!(
            associate_dir("root/bim", &Line::Cd("boum".to_string())),
            "root/bim/boum".to_string()
        );
        assert_eq!(
            associate_dir("root/bim", &Line::Cd("..".to_string())),
            "root".to_string()
        );
        assert_eq!(
            associate_dir("root/bim", &Line::Cd("/".to_string())),
            "root".to_string()
        );
        assert_eq!(
            associate_dir("root/bim/boum", &Line::Cd("/".to_string())),
            "root".to_string()
        );

        assert_eq!(associate_dir("/boum", &Line::Ls), "/boum".to_string(),);
        assert_eq!(associate_dir("/", &Line::Ls), "/".to_string());
    }

    #[test]
    fn all_dirs_bim() {
        assert_eq!(all_dirs("root"), vec!["root".to_string()]);
        assert_eq!(
            all_dirs("root/boum"),
            vec!["root".to_string(), "root/boum".to_string()]
        );
        assert_eq!(
            all_dirs("root/boum/bim"),
            vec![
                "root".to_string(),
                "root/boum/bim".to_string(),
                "root/boum".to_string()
            ]
        );
    }

    #[test]
    fn size_by_dir_boum() {
        assert_eq!(
            size_by_dir(vec![].into_iter().collect(), "root".to_string(), 1),
            vec![("root".to_string(), 1)].into_iter().collect()
        );

        assert_eq!(
            size_by_dir(
                vec![("root".to_string(), 2)].into_iter().collect(),
                "root".to_string(),
                1
            ),
            vec![("root".to_string(), 3)].into_iter().collect()
        );
    }

    fn part1_bim() {
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                272080 dncdssn.hdr
                4679 lmw.wmp"
            ),
            272080 + 4679
        );
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                272080 dncdssn.hdr
                $ cd azd
                4679 lmw.wmp"
            ),
            272080 + 4679
        );
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                100000 dncdssn.hdr
                $ cd azd
                100000 lmw.wmp
                $ cd azd
                100000 lmw.wmp"
            ),
            100000 + 200000 + 300000
        );
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                100000 dncdssn.hdr
                $ cd a
                100000 lmw.wmp
                $ cd aa
                100000 lmw.wmp
                $ cd ..
                $ cd ..
                $ cd b
                100000 lmw.wmp"
            ),
            100000 + 100000 + 200000 + 400000
        );
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                100000 dncdssn.hdr
                $ cd a
                100000 lmw.wmp
                $ cd aa
                100000 lmw.wmp
                100000 lmw.wmp
                $ cd ..
                $ cd ..
                $ cd b
                100000 lmw.wmp"
            ),
            100000 + 200000 + 300000 + 500000
        );
        assert_eq!(
            part1_run(
                "$ cd /
                $ ls
                100000 dncdssn.hdr
                $ cd a
                100000 lmw.wmp
                $ cd aa
                100000 lmw.wmp
                100000 lmw.wmp
                $ cd ..
                $ cd ..
                $ cd b
                100000 lmw.wmp"
            ),
            100000 + 200000 + 300000 + 500000
        );
    }

    #[test]
    fn positive_close_to_0_bmou() {
        assert_eq!(positive_close_to_0(-1, 10), Ordering::Greater);
        assert_eq!(positive_close_to_0(10, -1), Ordering::Less);
        assert_eq!(positive_close_to_0(10, 10), Ordering::Equal);
        assert_eq!(positive_close_to_0(1, 10), Ordering::Less);
    }
}

fn input() -> String {
    fs::read_to_string("day7").unwrap()
}
