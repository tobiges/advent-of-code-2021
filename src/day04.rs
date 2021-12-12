use std::str::Lines;

type BingoFieldWithMarks = [[(u32, bool); 5]; 5];

pub fn part1(input: &str) -> u64 {
    let (drawn_numbers, mut bingos) = parse_input(input);
    for called_num in drawn_numbers {
        for bingo in &mut bingos {
            if mark_number_and_check_for_winner(bingo, called_num) {
                return print_result(bingo, called_num);
            }
        }
    }
    panic!("no winning bingo found!");
}

pub fn part2(input: &str) -> u64 {
    let (drawn_numbers, mut bingos) = parse_input(input);
    let mut bingo_refs: Vec<&mut BingoFieldWithMarks> = bingos.iter_mut().collect();
    for called_num in drawn_numbers {
        let remaining_bingos = bingo_refs.len();
        let mut filtered_bingos: Vec<&mut BingoFieldWithMarks> =
            Vec::with_capacity(remaining_bingos);
        for bingo in bingo_refs {
            if mark_number_and_check_for_winner(bingo, called_num) {
                if remaining_bingos == 1 {
                    return print_result(bingo, called_num);
                }
            } else {
                filtered_bingos.push(bingo);
            }
        }
        bingo_refs = filtered_bingos;
    }
    panic!("no winning bingo found!");
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<BingoFieldWithMarks>) {
    let mut lines = input.lines();
    let drawn_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let mut bingos: Vec<BingoFieldWithMarks> = Vec::new();
    while lines.next().is_some() {
        bingos.push(parse_bingo5x5(&mut lines))
    }
    (drawn_numbers, bingos)
}

fn parse_bingo5x5(lines: &mut Lines) -> BingoFieldWithMarks {
    let mut bingo: BingoFieldWithMarks = [[(0, false); 5]; 5];
    for bingo_row in &mut bingo {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .enumerate()
            .for_each(|(col, num)| bingo_row[col].0 = num);
    }
    bingo
}

fn mark_number_and_check_for_winner(bingo: &mut BingoFieldWithMarks, num: u32) -> bool {
    for row in 0..bingo.len() {
        for col in 0..bingo[row].len() {
            if bingo[row][col].0 == num {
                bingo[row][col].1 = true;
                return check_for_winner(bingo, row, col);
            }
        }
    }
    false
}

fn check_for_winner(bingo: &BingoFieldWithMarks, row_to_check: usize, col_to_check: usize) -> bool {
    (0..bingo.len()).all(|row| bingo[row][col_to_check].1)
        || (0..bingo[row_to_check].len()).all(|col| bingo[row_to_check][col].1)
}

fn get_sum_of_unmarked_numbers(bingo: &BingoFieldWithMarks) -> u32 {
    bingo
        .iter()
        .flatten()
        .fold(0, |acc, &(current_num, is_marked)| match is_marked {
            true => acc,
            _ => acc + current_num,
        })
}

fn print_result(bingo: &BingoFieldWithMarks, called_num: u32) -> u64 {
    let sum_of_unmarked_numbers = get_sum_of_unmarked_numbers(bingo);
    println!("sum of all unmarked numbers: {}", sum_of_unmarked_numbers);
    println!("the number that was just called: {}", called_num);
    (sum_of_unmarked_numbers * called_num).into()
}
