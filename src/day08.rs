pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|line| line.split_once('|').unwrap().1.split_ascii_whitespace())
        .fold(0, |acc, num| match num.len() {
            2 | 3 | 4 | 7 => acc + 1,
            _ => acc,
        })
}

pub fn part2(input: &str) -> u64 {
    input.lines().map(parse_line).map(solve).sum::<u32>().into()
}

fn parse_line(combination: &str) -> Vec<u8> {
    combination
        .split(&[' ', '|'][..])
        .filter(|comb| !comb.is_empty())
        .map(|comb| comb.chars().fold(0, |acc, c| acc | 1 << (c as u8 - b'a')))
        .collect()
}

#[derive(Default)]
struct SolveInfo {
    one: u8,
    four_without_one: u8,
}

fn solve(line: Vec<u8>) -> u32 {
    let mut solve_info = SolveInfo::default();
    for &comb in &line[0..10] {
        match comb.count_ones() {
            2 => solve_info.one = comb,
            4 => solve_info.four_without_one = comb,
            _ => (),
        }
    }
    solve_info.four_without_one &= !solve_info.one;

    let mut sum = 0;
    for &digit in &line[10..14] {
        sum = sum * 10 + solve_digit(&solve_info, digit);
    }
    sum
}

fn solve_digit(solve_info: &SolveInfo, digit: u8) -> u32 {
    match digit.count_ones() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 if digit & solve_info.one == solve_info.one => 3,
        5 if digit & solve_info.four_without_one == solve_info.four_without_one => 5,
        5 => 2,
        6 if digit & solve_info.one != solve_info.one => 6,
        6 if digit & solve_info.four_without_one != solve_info.four_without_one => 0,
        6 => 9,
        7 => 8,
        _ => panic!("invalid one count"),
    }
}
