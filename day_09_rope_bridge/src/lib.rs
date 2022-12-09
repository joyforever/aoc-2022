use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    L, R, U, D,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Direction::L,
            "R" => Direction::R,
            "U" => Direction::U,
            _ => Direction::D,
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: u32,
}

impl From<&str> for Motion {
    fn from(s: &str) -> Self {
        let (direction, steps) =  s.split_once(' ').unwrap();
        let direction = Direction::from(direction);
        let steps = steps.parse::<u32>().unwrap();
        Self { direction, steps }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_motions(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(Motion::from)
        .collect()
}

struct Knot {
    current: Position,
    visited: HashSet<Position>,
}

impl Default for Knot {
    fn default() -> Self {
        let current = Position::default();
        let mut visited = HashSet::new();
        visited.insert(current);
        Self { current, visited }
    }
}

impl Knot {
    fn forward(&mut self, direction: Direction) -> Position {
        match direction {
            Direction::L => self.current.x -= 1,
            Direction::R => self.current.x += 1,
            Direction::U => self.current.y += 1,
            Direction::D => self.current.y -= 1,
        }
        self.current
    }

    fn follow(&mut self, head: Position) {
        if head.x.abs_diff(self.current.x) <= 1 && head.y.abs_diff(self.current.y) <= 1 {
            return;
        }
    
        if self.current.x > head.x && self.current.x - head.x >= 1 {
            self.current.x -= 1;
        } else if self.current.x < head.x && head.x - self.current.x >= 1 {
            self.current.x += 1;
        }
    
        if self.current.y > head.y && self.current.y - head.y >= 1 {
            self.current.y -= 1;
        } else if self.current.y < head.y && head.y - self.current.y >= 1 {
            self.current.y += 1;
        }

        self.visited.insert(self.current);
    }
}

struct Rope {
    knots: Vec<Knot>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Rope {
    fn new(count: usize) -> Self {
        let mut knots = vec![];
        for _ in 0..count {
            knots.push(Knot::default());
        }
        Self {
            knots,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn step(&mut self, direction: Direction) {
        let pos = self.knots[0].forward(direction);
        self.min_x = self.min_x.min(pos.x);
        self.max_x = self.max_x.max(pos.x);
        self.min_y = self.min_y.min(pos.y);
        self.max_y = self.max_y.max(pos.y);

        for i in 1..self.knots.len() {
            let pos = self.knots[i-1].current;
            self.knots[i].follow(pos);
        }
    }

    fn visited_positions(&self, tail_index: usize) -> usize {
        self.knots[tail_index].visited.len()
    }

    #[allow(dead_code)]
    fn print_positions(&self) {
        for y in (self.min_y..=self.max_y).rev() {
            for x in self.min_x..=self.max_x {
                let mut found = false;
                for (i, k) in self.knots.iter().enumerate() {
                    if k.current.x == x && k.current.y == y {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{i}");
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn tail_visited_positions<const N: usize>(input: &str) -> usize {
    let motions = parse_motions(input);

    let mut rope = Rope::new(N);

    for motion in motions.iter() {
        //println!("== {:?} {} ==", motion.direction, motion.steps);
        (0..motion.steps).for_each(|_| {
            rope.step(motion.direction);
        });
        //rope.print_positions();
    }

    rope.visited_positions(N-1)
}

pub fn part_one(input: &str) -> usize {
    tail_visited_positions::<2>(input)
}

pub fn part_two(input: &str) -> usize {
    tail_visited_positions::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");
    const LARGE_EXAMPLE: &str = include_str!("../data/large_example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 1);
        assert_eq!(part_two(LARGE_EXAMPLE), 36);
    }
}
