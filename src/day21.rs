use std::{iter::Cycle, ops::RangeInclusive};

pub fn part1(input: &str) -> u64 {
    let mut die = DieDetermenistic::new();
    let mut players = parse_input(input);
    while players.iter().map(|p| p.score).max().unwrap() < 1000 {
        let p_idx = ((die.total_rolls / 3) % 2) as usize;
        players[p_idx].play((0..3).map(|_| die.roll()).sum());
    }
    die.total_rolls * players.iter().map(|p| p.score).min().unwrap()
}

pub fn part2(input: &str) -> u64 {
    solve_quantum(parse_input(input), 0)
        .into_iter()
        .max()
        .unwrap()
}

fn parse_input(input: &str) -> [Player; 2] {
    let mut lines = input.lines();
    [
        parse_input_line(lines.next().unwrap()),
        parse_input_line(lines.next().unwrap()),
    ]
}

fn parse_input_line(line: &str) -> Player {
    Player::new(line.split_once(": ").unwrap().1.parse().unwrap())
}

fn solve_quantum(players: [Player; 2], player: usize) -> [u64; 2] {
    let quantum_nums = [1, 3, 6, 7, 6, 3, 1];
    let mut res = [0; 2];
    if let Some(p_idx) = players.iter().position(|p| p.score >= 21) {
        res[p_idx] = 1;
    } else {
        for (c, v) in quantum_nums.iter().zip(3u64..) {
            let mut new_players = players.clone();
            new_players[player].play(v);
            let tmp_res = solve_quantum(new_players, player ^ 0x1);
            for (r, t) in res.iter_mut().zip(tmp_res) {
                *r += t * c;
            }
        }
    }
    res
}

struct DieDetermenistic {
    range: Cycle<RangeInclusive<u64>>,
    total_rolls: u64,
}

impl DieDetermenistic {
    fn new() -> Self {
        Self {
            range: (1..=100).cycle(),
            total_rolls: 0,
        }
    }
    fn roll(&mut self) -> u64 {
        self.total_rolls += 1;
        self.range.next().unwrap()
    }
}

#[derive(Clone)]
struct Player {
    space: u64,
    score: u64,
}

impl Player {
    fn new(space: u64) -> Self {
        Player { space, score: 0 }
    }
    fn play(&mut self, roll: u64) {
        self.space += roll % 10;
        if self.space > 10 {
            self.space %= 10;
        }
        self.score += self.space;
    }
}
