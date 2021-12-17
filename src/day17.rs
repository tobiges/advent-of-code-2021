use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    let launcher = ProbeLauncher::new(input);
    solve(&launcher).max().unwrap().try_into().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let launcher = ProbeLauncher::new(input);
    solve(&launcher).count().try_into().unwrap()
}

fn solve(launcher: &ProbeLauncher) -> impl Iterator<Item = i32> + '_ {
    (0..=*launcher.x_range.end()).flat_map(move |x_velocity| {
        (*launcher.y_range.start()..=*launcher.x_range.end())
            .filter_map(move |y_velocity| launcher.launch(Probe::new(x_velocity, y_velocity)))
    })
}

#[derive(Default)]
struct Probe {
    x: i32,
    y: i32,
    y_max: i32,
    x_velocity: i32,
    y_velocity: i32,
}

impl Probe {
    fn new(x_velocity: i32, y_velocity: i32) -> Self {
        Self {
            x_velocity,
            y_velocity,
            ..Default::default()
        }
    }
    fn step(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        self.y_max = self.y_max.max(self.y);
        self.x_velocity = 0.max(self.x_velocity - 1);
        self.y_velocity -= 1;
    }
}

struct ProbeLauncher {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl ProbeLauncher {
    fn new(target_area: &str) -> Self {
        let (x_range, y_range) = target_area
            .strip_prefix("target area: x=")
            .unwrap()
            .split_once(", y=")
            .unwrap();
        let (x_range, y_range) = (
            Self::range_str_to_range_inclusive(x_range),
            Self::range_str_to_range_inclusive(y_range),
        );
        Self { x_range, y_range }
    }
    fn range_str_to_range_inclusive(range: &str) -> RangeInclusive<i32> {
        let (start, end) = range.split_once("..").unwrap();
        let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
        start..=end
    }
    fn launch(&self, mut probe: Probe) -> Option<i32> {
        loop {
            if self.x_range.contains(&probe.x) && self.y_range.contains(&probe.y) {
                return Some(probe.y_max);
            } else if probe.x > *self.x_range.end() || probe.y < *self.y_range.start() {
                return None;
            }
            probe.step();
        }
    }
}
