use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            _ => Direction::Down,
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

impl Position {
    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Left  => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up    => self.y += 1,
            Direction::Down  => self.y -= 1,
        }
    }
}

fn parse_motions(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(Motion::from)
        .collect()
}

fn move_tail(head: Position, tail: &mut Position, direction: Direction) -> Position {
    if head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1 {
        return *tail;
    }

    tail.step(direction);

    match direction {
        Direction::Left | Direction::Right => {
            if tail.y > head.y && tail.y - head.y >= 1 {
                tail.y -= 1;
            } else if tail.y < head.y && head.y - tail.y >= 1 {
                tail.y += 1;
            }
        },
        Direction::Up | Direction::Down => {
            if tail.x > head.x && tail.x - head.x >= 1 {
                tail.x -= 1;
            } else if tail.x < head.x && head.x - tail.x >= 1 {
                tail.x += 1;
            }
        },
    }

    *tail
}

pub fn part_one(input: &str) -> usize {
    let motions = parse_motions(input);

    let mut head = Position { x: 0, y: 0, };
    let mut tail = Position { x: 0, y: 0, };

    let mut map = HashSet::new();
    map.insert(tail); // starting position

    for motion in &motions {
        (0..motion.steps).for_each(|_| {
            head.step(motion.direction);
            map.insert(move_tail(head, &mut tail, motion.direction));
            //println!("{head:?} {tail:?}");
        });
    }

    map.len()
}

pub fn part_two(input: &str) -> usize {
    let motions = parse_motions(input);

    let mut knots = Vec::new();
    for _ in 0..10 {
        let pos = Position { x: 0, y: 0, };
        let mut set = HashSet::new();
        set.insert(pos);
        knots.push((pos, set));
    }

    for motion in &motions {
        (0..motion.steps).for_each(|_| {
            knots[0].0.step(motion.direction);
            for i in 1..=9 {
                let tail = move_tail(knots[i-1].0, &mut knots[i].0, motion.direction);
                knots[i].1.insert(tail);
            }
        });
    }

    knots[9].1.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 1);
    }
}
