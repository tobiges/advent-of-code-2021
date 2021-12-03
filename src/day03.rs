#[derive(PartialEq, Debug, Copy, Clone)]
enum MostCommonBit {
    One,
    Zero,
    Equal,
}

impl MostCommonBit {
    fn from_relative_one_count(relative_one_count: i32) -> MostCommonBit {
        match relative_one_count {
            0 => MostCommonBit::Equal,
            c if c < 0 => MostCommonBit::Zero,
            _ => MostCommonBit::One,
        }
    }
}

pub fn part1(input: String) {
    let most_common_bits =
        calculate_all_most_common_bits(&input_to_u64vec(&input), input.find('\n').unwrap());

    let gamma_rate = most_common_bit_vec_to_dec(&most_common_bits, MostCommonBit::One);
    let epsilon_rate = most_common_bit_vec_to_dec(&most_common_bits, MostCommonBit::Zero);

    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon_rate: {}", epsilon_rate);
    println!("Product (answer): {}", gamma_rate * epsilon_rate)
}

pub fn part2(input: String) {
    let line_length = input.find('\n').unwrap();
    let mapped_input = input_to_u64vec(&input);

    let oxygen_generator_rating =
        filter_equal_until_only_one_left(&mapped_input, line_length, |mcb, current_bit_is_one| {
            match mcb {
                MostCommonBit::One | MostCommonBit::Equal => current_bit_is_one,
                MostCommonBit::Zero => !current_bit_is_one,
            }
        });
    let co2_scrubber_rating =
        filter_equal_until_only_one_left(&mapped_input, line_length, |mcb, current_bit_is_one| {
            match mcb {
                MostCommonBit::One | MostCommonBit::Equal => !current_bit_is_one,
                MostCommonBit::Zero => current_bit_is_one,
            }
        });

    println!("oxygen_generator_rating: {}", oxygen_generator_rating);
    println!("co2_scrubber_rating: {}", co2_scrubber_rating);
    println!(
        "Product (answer): {}",
        oxygen_generator_rating * co2_scrubber_rating
    )
}

fn input_to_u64vec(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| u64::from_str_radix(line, 2).unwrap())
        .collect()
}

fn calculate_all_most_common_bits(input: &Vec<u64>, line_length: usize) -> Vec<MostCommonBit> {
    let mut res: Vec<MostCommonBit> = Vec::with_capacity(line_length);
    (0..line_length)
        .rev()
        .for_each(|i| res.push(calculate_most_common_bit_for_row(input, i)));
    res
}

fn calculate_most_common_bit_for_row(input: &Vec<u64>, row: usize) -> MostCommonBit {
    let mut relative_one_count = 0;
    let bit_filter = 1 << row;
    for line in input {
        if line & bit_filter != 0 {
            relative_one_count += 1;
        } else {
            relative_one_count -= 1;
        }
    }
    MostCommonBit::from_relative_one_count(relative_one_count)
}

fn most_common_bit_vec_to_dec(v: &Vec<MostCommonBit>, set_bit_one_on: MostCommonBit) -> u64 {
    v.iter()
        .zip((0..v.len()).rev())
        .fold(0, |acc, item| match item {
            (mcb, i) if *mcb == set_bit_one_on => acc | 1 << i,
            _ => acc,
        })
}

fn filter_equal_until_only_one_left<F>(input: &Vec<u64>, line_length: usize, filter: F) -> u64
where
    F: Fn(MostCommonBit, bool) -> bool,
{
    let mut filtered_input: Vec<u64> = input.to_vec();
    for row in (0..line_length).rev() {
        let current_mcb = calculate_most_common_bit_for_row(&filtered_input, row);
        filtered_input = filtered_input
            .into_iter()
            .filter(|x| filter(current_mcb, x & (1 << row) != 0))
            .collect();
        if filtered_input.len() == 1 {
            return *filtered_input.first().unwrap();
        }
    }
    panic!(
        "not exactly one element left in vector, vec hat {} elements.",
        filtered_input.len()
    );
}
