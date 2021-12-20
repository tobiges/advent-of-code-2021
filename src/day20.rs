pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 50)
}

fn solve(input: &str, n: usize) -> u64 {
    let mut e = Enhancer::from_str(input);
    for _ in 0..n {
        e.enhance()
    }
    e.count_lit_pixel().try_into().unwrap()
}

struct Enhancer {
    algorithm: Vec<bool>,
    image: Vec<Vec<bool>>,
    surrounding_lit: bool,
}

impl Enhancer {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        Self {
            algorithm: lines.next().unwrap().chars().map(Self::map_pixel).collect(),
            image: lines
                .skip(1)
                .map(|line| line.chars().map(Self::map_pixel).collect())
                .collect(),
            surrounding_lit: false,
        }
    }
    fn map_pixel(c: char) -> bool {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("invalid char in input: {}", c),
        }
    }
    fn enhance(&mut self) {
        let mut res = Vec::with_capacity(self.image.len() + 2);
        for y in 0..(self.image.len() + 2) {
            let mut row = Vec::with_capacity(self.image[0].len() + 2);
            for x in 0..(self.image[0].len() + 2) {
                let mut e_idx = 0;
                for y_o in 0..=2 {
                    for x_o in 0..=2 {
                        e_idx <<= 1;
                        e_idx |= match ((x + x_o).checked_sub(2), (y + y_o).checked_sub(2)) {
                            (Some(x), Some(y))
                                if y < self.image.len() && x < self.image[0].len() =>
                            {
                                self.image[y][x] as usize
                            }
                            _ => self.surrounding_lit as usize,
                        };
                    }
                }
                row.push(self.algorithm[e_idx]);
            }
            res.push(row);
        }
        self.surrounding_lit =
            self.algorithm[self.surrounding_lit.then(|| 0x1FFusize).unwrap_or_default()];
        self.image = res;
    }
    fn count_lit_pixel(&self) -> usize {
        self.image.iter().flatten().filter(|&&p| p).count()
    }
}
