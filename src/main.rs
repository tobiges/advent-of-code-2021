use std::env;
use std::io;
use std::io::Write;
use std::time::{Duration, Instant};

use advent_of_code_2021::{get_day,get_input_for_day, DayFn};

fn main() {
    let mut args = env::args().skip(1);
    let day = args
        .next()
        .unwrap_or_else(|| {
            print!("Enter day: ");
            io::stdout().flush().unwrap();
            let mut day = String::new();
            io::stdin()
                .read_line(&mut day)
                .expect("Failed to read line");
            day
        })
        .trim()
        .parse::<u32>()
        .unwrap();

    run_day(day);
}

pub fn run_day(day: u32) {
    let input = get_input_for_day(day);
    let to_run = get_day(day);
    run_part(to_run.0, 1, &input);
    run_part(to_run.1, 2, &input);
}

fn run_part(part_fn: DayFn, part: u8, input: &str) {
    println!("Running Part {}", part);
    let start = Instant::now();
    println!("Answer: {}", part_fn(input));
    println!("Took {}", fmt_dur(start.elapsed()));
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{}ms ", whole_ms) + &fmt_time(rem_ms);
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}
