pub fn part1(input: &str) -> u64 {
    solve(input, |s| s)
}

pub fn part2(input: &str) -> u64 {
    solve(input, |s| s * (s + 1) / 2)
}

fn solve<F>(input: &str, crab_fuel_burn_fn: F) -> u64
where
    F: Fn(u32) -> u32,
{
    let initial_crab_h_pos: Vec<u32> = input
        .split(',')
        .map(|h_pos| h_pos.parse::<u32>().unwrap())
        .collect();

    let min_h_pos = *initial_crab_h_pos.iter().min().unwrap();
    let max_h_pos = *initial_crab_h_pos.iter().max().unwrap();

    (min_h_pos..=max_h_pos)
        .map(|target_pos| {
            initial_crab_h_pos.iter().fold(0, |acc, &current_pos| {
                let diff = if target_pos > current_pos {
                    target_pos - current_pos
                } else {
                    current_pos - target_pos
                };
                acc + crab_fuel_burn_fn(diff)
            })
        })
        .min()
        .unwrap()
        .into()
}
