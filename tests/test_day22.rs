use advent_of_code_2021::{get_day, get_input_for_day};

const DAY: u32 = 22;

#[test]
fn test_input_part1() {
    let input = get_input_for_day(DAY);
    assert_eq!(642125, get_day(DAY).0(&input));
}

#[test]
fn test_input_part2() {
    let input = get_input_for_day(DAY);
    assert_eq!(1235164413198198, get_day(DAY).1(&input));
}
