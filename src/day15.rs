use std::{cmp::Ordering, collections::BinaryHeap};

pub fn part1(input: &str) -> u64 {
    solve(parse_input(input)).unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut input = parse_input(input);
    let original_input_rows = input.len();
    input.reserve_exact(4 * original_input_rows);

    for row in input.iter_mut() {
        let original_input_cols = row.len();
        row.reserve_exact(4 * original_input_cols);
        for i in 1..5 {
            row.extend_from_within(..original_input_cols);
            for risk in &mut row[i * original_input_cols..(i + 1) * original_input_cols] {
                *risk = risk_fn(*risk, i);
            }
        }
    }

    for i in 1..5 {
        for row_idx in 0..original_input_rows {
            input.push(input[row_idx].iter().map(|&r| risk_fn(r, i)).collect());
        }
    }

    solve(input).unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

fn risk_fn(old_risk: usize, increase: usize) -> usize {
    let new_risk = old_risk + increase;
    if new_risk >= 10 {
        new_risk % 10 + 1
    } else {
        new_risk
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    risk: usize,
    cord: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.cord.cmp(&other.cord))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: Vec<Vec<usize>>) -> Option<u64> {
    let mut risks = vec![vec![usize::MAX; input[0].len()]; input.len()];
    let mut heap = BinaryHeap::new();

    risks[0][0] = 0;
    heap.push(State {
        risk: 0,
        cord: (0, 0),
    });

    while let Some(State { risk, cord: (y, x) }) = heap.pop() {
        if y == input.len() - 1 && x == input[0].len() - 1 {
            return Some(risk.try_into().unwrap());
        }
        if risk > risks[y][x] {
            continue;
        }

        for (y_n, x_n) in get_neighbors(&input, y, x) {
            let neighbor = State {
                risk: risk + input[y_n][x_n],
                cord: (y_n, x_n),
            };
            if neighbor.risk < risks[y_n][x_n] {
                heap.push(neighbor);
                risks[y_n][x_n] = neighbor.risk;
            }
        }
    }
    None
}

fn get_neighbors(input: &[Vec<usize>], y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut neighbor_cords = Vec::with_capacity(4);
    if y > 0 {
        neighbor_cords.push((y - 1, x));
    }
    if y + 1 < input.len() {
        neighbor_cords.push((y + 1, x));
    }
    if x > 0 {
        neighbor_cords.push((y, x - 1));
    }
    if x + 1 < input[y].len() {
        neighbor_cords.push((y, x + 1));
    }
    neighbor_cords
}
