pub fn part1(input: String) {
    let answer: u64 = input
        .lines()
        .filter_map(|line| find_first_incorrect_char(line, &mut Vec::new()))
        .map(incorrect_char_to_points)
        .sum();
    println!("Answer: {}", answer);
}

pub fn part2(input: String) {
    let mut all_points: Vec<u64> = input
        .lines()
        .filter_map(get_completion_for_line)
        .map(|completion| {
            completion
                .iter()
                .fold(0, |acc, &c| acc * 5 + completed_char_to_points(c))
        })
        .collect();
    all_points.sort_unstable();
    let answer = all_points.get(all_points.len() / 2).unwrap();
    println!("Answer: {}", answer);
}

fn find_first_incorrect_char(line: &str, stack: &mut Vec<char>) -> Option<char> {
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(open_c) if c == get_closing_char(open_c) => (),
                _ => return Some(c),
            },
            _ => panic!("invalid char in input"),
        }
    }
    None
}

fn get_completion_for_line(line: &str) -> Option<Vec<char>> {
    let mut open_chars = Vec::new();
    if find_first_incorrect_char(line, &mut open_chars) == None {
        Some(open_chars.into_iter().rev().map(get_closing_char).collect())
    } else {
        None
    }
}

fn get_closing_char(opening_char: char) -> char {
    match opening_char {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid opening_char"),
    }
}

fn incorrect_char_to_points(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char in input"),
    }
}

fn completed_char_to_points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char in input"),
    }
}
