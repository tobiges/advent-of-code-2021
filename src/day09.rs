use std::collections::HashSet;

type Coordinate = (usize, usize);

pub fn part1(input: &str) -> u64 {
    let input = parse_input(input);
    let low_points = get_all_low_points(&input);
    low_points
        .iter()
        .map(|&(y, x)| input[y][x] + 1)
        .sum::<u32>()
        .into()
}

pub fn part2(input: &str) -> u64 {
    let input = parse_input(input);
    let low_points = get_all_low_points(&input);
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&lp| get_size_of_basin(&input, lp))
        .collect();
    basin_sizes.sort_unstable();
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>()
        .try_into()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_all_low_points(input: &[Vec<u32>]) -> Vec<Coordinate> {
    let mut low_points = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if is_low_point(input, (y, x)) {
                low_points.push((y, x));
            }
        }
    }
    low_points
}

fn get_neighbor_cords(input: &[Vec<u32>], (y, x): Coordinate) -> Vec<Coordinate> {
    let mut neighbour_heights = Vec::with_capacity(4);
    if y > 0 {
        neighbour_heights.push((y - 1, x));
    }
    if y + 1 < input.len() {
        neighbour_heights.push((y + 1, x));
    }
    if x > 0 {
        neighbour_heights.push((y, x - 1));
    }
    if x + 1 < input[y].len() {
        neighbour_heights.push((y, x + 1));
    }
    neighbour_heights
}

fn is_low_point(input: &[Vec<u32>], cord: Coordinate) -> bool {
    get_neighbor_cords(input, cord)
        .iter()
        .all(|&(y, x)| input[y][x] > input[cord.0][cord.1])
}

fn get_size_of_basin(input: &[Vec<u32>], low_point: Coordinate) -> usize {
    let mut visited = HashSet::new();
    let mut cords_to_check = vec![low_point];
    while let Some(cord) = cords_to_check.pop() {
        if input[cord.0][cord.1] != 9 && visited.insert(cord) {
            cords_to_check.append(&mut get_neighbor_cords(input, cord));
        }
    }
    visited.len()
}
