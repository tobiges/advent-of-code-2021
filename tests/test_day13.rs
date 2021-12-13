use advent_of_code_2021::{get_day, get_input_for_day};

const DAY: u32 = 13;

#[test]
fn test_input_part1() {
    let input = get_input_for_day(DAY);
    assert_eq!(610, get_day(DAY).0(&input));
}
