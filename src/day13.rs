use std::collections::HashSet;

type Dot = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}

pub fn part1(input: &str) -> u64 {
    let (dots, instructions) = parse_input(input);
    execute_instruction(dots, *instructions.first().unwrap())
        .len()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (mut dots, instructions) = parse_input(input);
    for instruction in instructions {
        dots = execute_instruction(dots, instruction);
    }

    let max_y = dots.iter().map(|&(y, _)| y).max().unwrap();
    let max_x = dots.iter().map(|&(_, x)| x).max().unwrap();
    let mut output = vec![vec!['.'; max_x + 1]; max_y + 1];
    for (y, x) in dots {
        output[y][x] = '#';
    }
    for row in output {
        println!("{}", row.into_iter().collect::<String>());
    }
    0
}

fn parse_input(input: &str) -> (HashSet<Dot>, Vec<Instruction>) {
    let mut lines = input.lines();
    let dots = lines
        .by_ref()
        .map_while(|line| line.split_once(','))
        .map(|(x, y)| (y.parse().unwrap(), x.parse().unwrap()))
        .collect();
    let instructions = lines
        .map(|line| line.split_once('=').unwrap())
        .map(|(i, v)| (i, v.parse().unwrap()))
        .map(|(i, v)| match i {
            "fold along x" => Instruction::FoldX(v),
            "fold along y" => Instruction::FoldY(v),
            _ => panic!("invalid fold command"),
        })
        .collect();
    (dots, instructions)
}

fn execute_instruction(dots: HashSet<Dot>, instruction: Instruction) -> HashSet<Dot> {
    dots.into_iter()
        .map(|(y, x)| match instruction {
            Instruction::FoldX(v) if x > v => (y, v - (x - v)),
            Instruction::FoldY(v) if y > v => (v - (y - v), x),
            _ => (y, x),
        })
        .collect()
}
