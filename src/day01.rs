pub fn part1(input: &str) -> u64 {
    get_increase_count(parse_input(input))
}

pub fn part2(input: &str) -> u64 {
    let input_numbers = parse_input(input);
    let pair_of_3_sums = input_numbers
        .iter()
        .zip(input_numbers.iter().skip(1))
        .zip(input_numbers.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    get_increase_count(pair_of_3_sums)
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn get_increase_count(numbers: Vec<i32>) -> u64 {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}
