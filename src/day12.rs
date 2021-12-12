use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u64 {
    solve(input, false)
}

pub fn part2(input: &str) -> u64 {
    solve(input, true)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Big(&'a str),
}

impl<'a> Cave<'a> {
    fn from_str(s: &'a str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.chars().all(|c| c.is_ascii_lowercase()) => Self::Small(s),
            _ if s.chars().all(|c| c.is_ascii_uppercase()) => Self::Big(s),
            _ => panic!("invalid cave string"),
        }
    }
}

fn solve(input: &str, allow_second_visit: bool) -> u64 {
    let mut cave_paths = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let (a, b) = (Cave::from_str(a), Cave::from_str(b));
        cave_paths.entry(a).or_insert_with(Vec::new).push(b);
        cave_paths.entry(b).or_insert_with(Vec::new).push(a);
    }

    dfs(
        &cave_paths,
        Cave::Start,
        &mut HashSet::new(),
        allow_second_visit,
    )
}

fn dfs<'a>(
    cave_paths: &HashMap<Cave<'a>, Vec<Cave<'a>>>,
    current_cave: Cave<'a>,
    visited: &mut HashSet<Cave<'a>>,
    allow_second_visit: bool,
) -> u64 {
    let mut added_cave_to_visited = false;
    if let Cave::Small(_) = current_cave {
        added_cave_to_visited = visited.insert(current_cave);
    }

    let mut path_counter = 0;
    for &cave in cave_paths.get(&current_cave).unwrap() {
        path_counter += match cave {
            Cave::End => 1,
            Cave::Big(_) => dfs(cave_paths, cave, visited, allow_second_visit),
            Cave::Small(_) if !visited.contains(&cave) => {
                dfs(cave_paths, cave, visited, allow_second_visit)
            }
            Cave::Small(_) if allow_second_visit => dfs(cave_paths, cave, visited, false),
            _ => 0,
        };
    }

    if added_cave_to_visited {
        visited.remove(&current_cave);
    }
    path_counter
}
