pub fn part1(input: String) {
    println!(
        "{}",
        get_increase_count(
            input
                .split("\n")
                .map(|line| line.parse::<i32>().unwrap())
                .collect()
        )
    );
}

pub fn part2(input: String) {
    let input_nums: Vec<i32> = input
        .split("\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    let pair_of_3_sums = input_nums
        .iter()
        .zip(input_nums.iter().skip(1))
        .zip(input_nums.iter().skip(2))
        .map(|pair_of3| pair_of3.0 .0 + pair_of3.0 .1 + pair_of3.1)
        .collect();

    println!("{}", get_increase_count(pair_of_3_sums));
}

fn get_increase_count(nums: Vec<i32>) -> i32 {
    nums.iter().zip(nums.iter().skip(1)).fold(0, |acc, e| {
        if e.1 > e.0 {
            return acc + 1;
        }
        acc
    })
}
