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

fn move_tail(head: Position, tail: &mut Position, direction: Direction) {
    if head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1 {
        return
    }

    tail.step(direction);

    match direction {
        Direction::Left | Direction::Right => {
            tail.y = head.y;
        },
        Direction::Up | Direction::Down => {
            tail.x = head.x;
        },
    }
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
            move_tail(head, &mut tail, motion.direction);
            map.insert(tail);
        });
    }

    map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }
}
