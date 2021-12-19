use std::str::Chars;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(Snailfish::parse)
        .reduce(Snailfish::add)
        .unwrap()
        .magnitude()
}

pub fn part2(input: &str) -> u64 {
    let snail_fish: Vec<_> = input.lines().map(Snailfish::parse).collect();
    let mut max = 0;
    for a in 0..snail_fish.len() {
        for b in 0..snail_fish.len() {
            if a != b {
                max = max.max(snail_fish[a].clone().add(snail_fish[b].clone()).magnitude());
            }
        }
    }
    max
}

#[derive(Clone)]
enum Snailfish {
    Literal(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    fn parse(s: &str) -> Self {
        Self::parse_chars(&mut s.chars())
    }
    fn parse_chars(chars: &mut Chars) -> Self {
        match chars.next().unwrap() {
            c @ '0'..='9' => Self::Literal(c.to_digit(10).unwrap().into()),
            '[' => {
                let l = Self::parse_chars(chars);
                chars.next();
                let r = Self::parse_chars(chars);
                chars.next();
                Self::Pair(Box::new(l), Box::new(r))
            }
            c => panic!("invalid char found: {}", c),
        }
    }
    fn add(self, other: Self) -> Self {
        let mut res = Self::Pair(Box::new(self), Box::new(other));
        while res.explode() || res.split() {}
        res
    }
    fn explode(&mut self) -> bool {
        let mut left = None;
        let mut curr = self;
        let mut depth: usize = 0;
        let mut stack = Vec::new();
        loop {
            match curr {
                Self::Literal(val) => {
                    left = Some(val);
                    if let Some((sf, d)) = stack.pop() {
                        curr = sf;
                        depth = d;
                    } else {
                        return false;
                    }
                }
                Self::Pair(l, r) if depth >= 4 => {
                    if let Some(l_val) = left {
                        *l_val += match **l {
                            Self::Literal(val) => val,
                            _ => panic!("Left snailfish is not a literal!"),
                        };
                    }
                    if let Some((mut right, _)) = stack.pop() {
                        loop {
                            match right {
                                Self::Pair(l, _) => right = l,
                                Self::Literal(r_val) => {
                                    *r_val += match **r {
                                        Self::Literal(val) => val,
                                        _ => panic!("Right snailfish is not a literal!"),
                                    };
                                    break;
                                }
                            }
                        }
                    }
                    *curr = Self::Literal(0);
                    return true;
                }
                Self::Pair(l, r) => {
                    depth += 1;
                    curr = l;
                    stack.push((r, depth));
                }
            };
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Self::Pair(l, r) => l.split() || r.split(),
            &mut Self::Literal(val) if val >= 10 => {
                *self = Self::Pair(
                    Box::new(Snailfish::Literal(val / 2)),
                    Box::new(Snailfish::Literal((val + 1) / 2)),
                );
                true
            }
            _ => false,
        }
    }
    fn magnitude(self) -> u64 {
        match self {
            Self::Literal(v) => v,
            Self::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}
