type Coordinate = (usize, usize);

pub fn part1(input: String) {
    let mut octopuses = parse_input(input);
    let mut flash_count = 0_u32;
    for _ in 0..100 {
        flash_count += simulate_one_iteration(&mut octopuses);
    }
    println!("Answer: {}", flash_count);
}

pub fn part2(input: String) {
    let mut octopuses = parse_input(input);
    let mut iteration = 0;
    loop {
        iteration += 1;
        if simulate_one_iteration(&mut octopuses) == 100 {
            break;
        }
    }
    println!("Answer: {}", iteration);
}

fn parse_input(input: String) -> [[u32; 10]; 10] {
    let mut octopuses = [[0_u32; 10]; 10];
    for (y, line) in input.lines().enumerate() {
        for (x, o) in line.char_indices() {
            octopuses[y][x] = o.to_digit(10).unwrap();
        }
    }
    octopuses
}

fn simulate_one_iteration(octopuses: &mut [[u32; 10]; 10]) -> u32 {
    let mut flash_count = 0;
    let mut octopuses_to_flash = Vec::new();
    for y in 0..10 {
        for x in 0..10 {
            octopuses[y][x] += 1;
            if octopuses[y][x] == 10 {
                octopuses_to_flash.push((y, x));
            }
        }
    }
    while let Some(cord) = octopuses_to_flash.pop() {
        flash_count += 1;
        for (y_n, x_n) in get_all_neighbor_cords(cord) {
            octopuses[y_n][x_n] += 1;
            if octopuses[y_n][x_n] == 10 {
                octopuses_to_flash.push((y_n, x_n));
            }
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            if octopuses[y][x] > 9 {
                octopuses[y][x] = 0;
            }
        }
    }
    flash_count
}

fn get_all_neighbor_cords((y, x): Coordinate) -> impl Iterator<Item = Coordinate> {
    let y_start = if y > 0 { y - 1 } else { 0 };
    let y_end = if y < 9 { y + 1 } else { 9 };
    let x_start = if x > 0 { x - 1 } else { 0 };
    let x_end = if x < 9 { x + 1 } else { 9 };

    (y_start..=y_end).flat_map(move |y_n| {
        (x_start..=x_end).filter_map(move |x_n| {
            if y == y_n && x == x_n {
                None
            } else {
                Some((y_n, x_n))
            }
        })
    })
}
