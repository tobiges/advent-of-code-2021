enum SubmarineCommand {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl SubmarineCommand {
    pub fn from_string(command_string: &str) -> SubmarineCommand {
        if let Some(n) = command_string.strip_prefix("forward ") {
            SubmarineCommand::Forward(n.parse::<u32>().unwrap())
        } else if let Some(n) = command_string.strip_prefix("up ") {
            SubmarineCommand::Up(n.parse::<u32>().unwrap())
        } else if let Some(n) = command_string.strip_prefix("down ") {
            SubmarineCommand::Down(n.parse::<u32>().unwrap())
        } else {
            panic!("invalid command!, got {}.", command_string)
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    input
        .lines()
        .map(SubmarineCommand::from_string)
        .for_each(|cmd| match cmd {
            SubmarineCommand::Forward(x) => horizontal_pos += x,
            SubmarineCommand::Up(x) => depth -= x,
            SubmarineCommand::Down(x) => depth += x,
        });

    print_answer(horizontal_pos, depth)
}

pub fn part2(input: &str) -> u64 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .lines()
        .map(SubmarineCommand::from_string)
        .for_each(|cmd| match cmd {
            SubmarineCommand::Forward(x) => {
                horizontal_pos += x;
                depth += aim * x;
            }
            SubmarineCommand::Up(x) => aim -= x,
            SubmarineCommand::Down(x) => aim += x,
        });

    print_answer(horizontal_pos, depth)
}

fn print_answer(horizontal_pos: u32, depth: u32) -> u64 {
    println!("final horizontal position: {}", horizontal_pos);
    println!("final depth: {}", depth);
    (horizontal_pos * depth).into()
}
