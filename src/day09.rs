use std::collections::HashSet;

type Coordinate = (usize, usize);

pub fn part1(input: String) {
    let input = parse_input(input);
    let low_points = get_all_lowpoints(&input);
    let answer: u32 = low_points.iter().map(|&(y, x)| input[y][x]).sum();
    println!("Answer: {}", answer);
}

pub fn part2(input: String) {
    let input = parse_input(input);
    let low_points = get_all_lowpoints(&input);
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&lp| get_size_of_basin(&input, lp))
        .collect();
    basin_sizes.sort_unstable();
    let answer: usize = basin_sizes.iter().rev().take(3).fold(1, |acc, bs| acc * bs);
    println!("Answer: {}", answer);
}

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_all_lowpoints(input: &Vec<Vec<u32>>) -> Vec<Coordinate> {
    let mut low_points = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if is_lowpoint(input, (y, x)) {
                low_points.push((y, x));
            }
        }
    }
    low_points
}

fn get_neighbour_cords(input: &Vec<Vec<u32>>, (y, x): Coordinate) -> Vec<Coordinate> {
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

fn is_lowpoint(input: &Vec<Vec<u32>>, cord: Coordinate) -> bool {
    input[cord.0][cord.1]
        < get_neighbour_cords(input, cord)
            .iter()
            .map(|&(y, x)| input[y][x])
            .min()
            .unwrap()
}

fn get_size_of_basin(input: &Vec<Vec<u32>>, low_point: Coordinate) -> usize {
    let mut visited = HashSet::new();
    let mut cords_to_check = vec![low_point];
    while let Some(cord) = cords_to_check.pop() {
        if input[cord.0][cord.1] != 9 && visited.insert(cord) {
            cords_to_check.append(&mut get_neighbour_cords(input, cord));
        }
    }
    visited.len()
}
