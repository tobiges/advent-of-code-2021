use std::collections::HashMap;

type Coordinate = (i32, i32);

struct VentLines {
    from: Coordinate,
    to: Coordinate,
}

impl VentLines {
    fn from_string(line: &str) -> Self {
        let (from_str, to_str) = line.split_once(" -> ").unwrap();
        VentLines {
            from: coordinate_from_str(from_str),
            to: coordinate_from_str(to_str),
        }
    }

    fn get_all_cords_on_line(&self, consider_diagonal_lines: bool) -> Vec<Coordinate> {
        let &Self {
            from: (x1, y1),
            to: (x2, y2),
        } = self;
        if x1 == x2 {
            range_between_nums_inclusive(y1, y2)
                .map(|y| (x1, y))
                .collect()
        } else if y1 == y2 {
            range_between_nums_inclusive(x1, x2)
                .map(|x| (x, y1))
                .collect()
        } else if consider_diagonal_lines {
            range_between_nums_inclusive(x1, x2)
                .zip(range_between_nums_inclusive(y1, y2))
                .collect()
        } else {
            vec![]
        }
    }
}

pub fn part1(input: String) {
    solve(input, false);
}

pub fn part2(input: String) {
    solve(input, true);
}

fn solve(input: String, consider_diagonal_lines: bool) {
    let mut number_of_lines_at_point = HashMap::new();
    input
        .lines()
        .map(VentLines::from_string)
        .flat_map(|vent_line| vent_line.get_all_cords_on_line(consider_diagonal_lines))
        .for_each(|cord| {
            number_of_lines_at_point
                .entry(cord)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
    let two_or_more_lines_overlap = number_of_lines_at_point
        .iter()
        .filter(|&(_, &line_count)| line_count >= 2)
        .count();
    println!("Answer: {}", two_or_more_lines_overlap);
}

fn range_between_nums_inclusive(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
    if start < end {
        Box::new(start..=end)
    } else {
        Box::new((end..=start).rev())
    }
}

fn coordinate_from_str(s: &str) -> Coordinate {
    let (x_str, y_str) = s.split_once(',').unwrap();
    (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap())
}
