pub fn part1(input: String) {
    solve(input, 80);
}

pub fn part2(input: String) {
    solve(input, 256);
}

fn solve(input: String, days: usize) {
    let mut fish_timers = [0usize; 9];
    input
        .split(',')
        .map(|f| f.parse::<usize>().unwrap())
        .for_each(|f| fish_timers[f] += 1);
    for _ in 0..days {
        fish_timers.rotate_left(1);
        fish_timers[6] += fish_timers[8];
    }
    let final_fish_count: usize = fish_timers.iter().sum();
    println!("Answer: {}", final_fish_count);
}
