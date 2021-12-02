enum SubmarineCommand {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl SubmarineCommand {
    pub fn from_string(command_string: &str) -> SubmarineCommand {
        if let Some(n) = command_string.strip_prefix("forward ") {
            return SubmarineCommand::Forward(n.parse::<i32>().unwrap());
        } else if let Some(n) = command_string.strip_prefix("up ") {
            return SubmarineCommand::Up(n.parse::<i32>().unwrap());
        } else if let Some(n) = command_string.strip_prefix("down ") {
            return SubmarineCommand::Down(n.parse::<i32>().unwrap());
        }
        panic!("invalid command!, got {}.", command_string)
    }
}

pub fn part1(input: String) {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    input
        .split("\n")
        .map(SubmarineCommand::from_string)
        .for_each(|cmd| match cmd {
            SubmarineCommand::Forward(x) => horizontal_pos += x,
            SubmarineCommand::Up(x) => depth -= x,
            SubmarineCommand::Down(x) => depth += x,
        });

    print_answer(horizontal_pos, depth);
}

pub fn part2(input: String) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .split("\n")
        .map(SubmarineCommand::from_string)
        .for_each(|cmd| match cmd {
            SubmarineCommand::Forward(x) => {
                horizontal_pos += x;
                depth += aim * x;
            }
            SubmarineCommand::Up(x) => aim -= x,
            SubmarineCommand::Down(x) => aim += x,
        });

    print_answer(horizontal_pos, depth);
}

fn print_answer(horizontal_pos: i32, depth: i32) {
    println!("final horizontal position: {}", horizontal_pos);
    println!("final depth: {}", depth);
    println!("Product (answer): {}", horizontal_pos * depth)
}
