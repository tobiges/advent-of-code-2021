use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    solve(input, 10)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 40)
}

fn solve(input: &str, iterations: usize) -> u64 {
    let template: Vec<_> = input.lines().next().unwrap().chars().collect();
    let rules: HashMap<_, _> = input
        .lines()
        .skip(2)
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(pair, insertion)| {
            (
                (pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()),
                insertion.chars().next().unwrap(),
            )
        })
        .collect();

    let mut combination_count: HashMap<(char, char), u64> = HashMap::new();
    for w in template.windows(2) {
        *combination_count.entry((w[0], w[1])).or_default() += 1;
    }
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for &c in template.iter() {
        *char_counts.entry(c).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_combination_count = HashMap::new();
        for ((left, right), count) in combination_count {
            let inserted = *rules.get(&(left, right)).unwrap();
            *char_counts.entry(inserted).or_default() += count;
            *new_combination_count.entry((left, inserted)).or_default() += count;
            *new_combination_count.entry((inserted, right)).or_default() += count;
        }
        combination_count = new_combination_count;
    }

    let mut char_counts: Vec<_> = char_counts.into_values().collect();
    char_counts.sort_unstable();
    char_counts.last().unwrap() - char_counts.first().unwrap()
}
