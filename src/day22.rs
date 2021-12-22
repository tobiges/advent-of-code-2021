use std::ops::Range;

pub fn part1(input: &str) -> u64 {
    solve(input, true)
}

pub fn part2(input: &str) -> u64 {
    solve(input, false)
}

fn solve(input: &str, cap_at_50: bool) -> u64 {
    let input = parse_input(input, cap_at_50.then(|| -50..51));
    let mut cubes_on = Vec::new();
    for (on, cube) in input {
        cubes_on = cubes_on
            .into_iter()
            .flat_map(|c: CubeOfCubes| c.remove(&cube))
            .collect();
        if on {
            cubes_on.push(cube);
        }
    }
    cubes_on.iter().map(CubeOfCubes::count_cubes).sum::<usize>() as u64
}

fn parse_input(input: &str, cap_at: Option<Range<i32>>) -> Vec<(bool, CubeOfCubes)> {
    input
        .lines()
        .map(|line| {
            let (command, cube) = line.split_once(' ').unwrap();
            let command = match command {
                "on" => true,
                "off" => false,
                _ => panic!("invalid command: {}", command),
            };
            (command, CubeOfCubes::parse(cube, cap_at.as_ref()))
        })
        .collect()
}

struct CubeOfCubes {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}
impl CubeOfCubes {
    fn parse(s: &str, cap_at: Option<&Range<i32>>) -> Self {
        let mut cube = s
            .split(',')
            .map(|cord| cord.split_once('=').unwrap().1)
            .map(Self::range_str_to_range_inclusive)
            .map(|range| match cap_at {
                Some(cap_at) => Self::range_intersection(cap_at, &range),
                None => range,
            });
        Self {
            x: cube.next().unwrap(),
            y: cube.next().unwrap(),
            z: cube.next().unwrap(),
        }
    }
    fn from_references(x: &Range<i32>, y: &Range<i32>, z: &Range<i32>) -> Self {
        Self {
            x: x.to_owned(),
            y: y.to_owned(),
            z: z.to_owned(),
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }
    fn range_str_to_range_inclusive(range: &str) -> Range<i32> {
        let (start, end) = range.split_once("..").unwrap();
        let (start, end) = (start.parse().unwrap(), end.parse::<i32>().unwrap() + 1);
        start..end
    }
    fn remove(self, to_remove: &Self) -> impl Iterator<Item = CubeOfCubes> {
        let mut res = Vec::with_capacity(24);
        let to_remove_ranges = [&to_remove.x, &to_remove.y, &to_remove.z];
        let self_ranges = [&self.x, &self.y, &self.z];
        if self_ranges
            .into_iter()
            .zip(to_remove_ranges)
            .any(|(r1, r2)| r1.end <= r2.start || r2.end <= r1.start)
        {
            res.push(self);
        } else {
            let adjusted_ranges: Vec<_> = self_ranges
                .into_iter()
                .zip(to_remove_ranges)
                .map(|(range_self, range_remove)| {
                    Self::range_intersection(range_self, range_remove)
                })
                .collect();
            let removed_ranges: Vec<_> = self_ranges
                .into_iter()
                .zip(&adjusted_ranges)
                .map(|(r1, r2)| Self::remove_range(r1, r2))
                .collect();

            for x in &removed_ranges[0] {
                for y in &removed_ranges[1] {
                    for z in &removed_ranges[2] {
                        res.push(Self::from_references(x, y, z));
                    }
                }
            }
            for i in 0..3 {
                for r1 in &removed_ranges[(i + 1) % 3] {
                    for r2 in &removed_ranges[(i + 2) % 3] {
                        let mut ranges = [&adjusted_ranges[i], r1, r2];
                        ranges.rotate_right(i);
                        res.push(Self::from_references(ranges[0], ranges[1], ranges[2]));
                    }
                }
            }
            for i in 0..3 {
                for r2 in &removed_ranges[(i + 2) % 3] {
                    let mut ranges = [&adjusted_ranges[i], &adjusted_ranges[(i + 1) % 3], r2];
                    ranges.rotate_right(i);
                    res.push(Self::from_references(ranges[0], ranges[1], ranges[2]));
                }
            }
        }

        res.into_iter().filter(|c| !Self::is_empty(c))
    }
    fn remove_range(range: &Range<i32>, to_remove: &Range<i32>) -> [Range<i32>; 2] {
        [range.start..to_remove.start, to_remove.end..range.end]
    }
    fn range_intersection(r1: &Range<i32>, r2: &Range<i32>) -> Range<i32> {
        r1.start.max(r2.start)..r1.end.min(r2.end)
    }
    fn count_cubes(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }
}
