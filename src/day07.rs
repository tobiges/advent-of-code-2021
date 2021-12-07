pub fn part1(input: String) {
    solve(&input, |s| s);
}

pub fn part2(input: String) {
    solve(&input, |s| s * (s + 1) / 2);
}

fn solve<F>(input: &str, crab_fuel_burn_fn: F)
where
    F: Fn(i32) -> i32,
{
    let initial_crab_h_pos: Vec<i32> = input
        .split(',')
        .map(|h_pos| h_pos.parse::<i32>().unwrap())
        .collect();

    let min_h_pos = *initial_crab_h_pos.iter().min().unwrap();
    let max_h_pos = *initial_crab_h_pos.iter().max().unwrap();

    let min_total_fuel = (min_h_pos..=max_h_pos)
        .map(|target_pos| {
            initial_crab_h_pos.iter().fold(0, |acc, &current_pos| {
                acc + crab_fuel_burn_fn(i32::abs(current_pos - target_pos))
            })
        })
        .min()
        .unwrap();
    println!("Answer: {}", min_total_fuel);
}
