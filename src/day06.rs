pub fn part1(input: &str) -> u64 {
    solve(input, 80)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 256)
}

fn solve(input: &str, days: usize) -> u64 {
    let mut fish_timers = [0u64; 9];
    input
        .split(',')
        .map(|f| f.parse::<usize>().unwrap())
        .for_each(|f| fish_timers[f] += 1);
    for _ in 0..days {
        fish_timers.rotate_left(1);
        fish_timers[6] += fish_timers[8];
    }
    fish_timers.iter().sum()
}
