#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn parse(s: &str) -> Self {
        if s == "A" || s == "X" {
            Shape::Rock
        } else if s == "B" || s == "Y" {
            Shape::Paper
        } else {
            Shape::Scissors
        }
    }

    fn score(&self) -> u32 {
        *self as u32
    }

    fn compete(&self, opponent: Shape) -> u32 {
        match (*self, opponent) {
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Paper) => 6,
            (_, _) => 3,
        }
    }

    fn choose(&self, guide: Guide) -> Shape {
        match (*self, guide) {
            (Shape::Rock, Guide::Lose) => Shape::Scissors,
            (Shape::Rock, Guide::Win) => Shape::Paper,
            (Shape::Paper, Guide::Lose) => Shape::Rock,
            (Shape::Paper, Guide::Win) => Shape::Scissors,
            (Shape::Scissors, Guide::Lose) => Shape::Paper,
            (Shape::Scissors, Guide::Win) => Shape::Rock,
            (shape, _) => shape,
        }
    }
}

#[derive(Clone, Copy)]
enum Guide {
    Lose,
    Draw,
    Win,
}

impl Guide {
    fn parse(s: &str) -> Self {
        if s == "X" {
            Guide::Lose
        } else if s == "Y" {
            Guide::Draw
        } else {
            Guide::Win
        }
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, choose) = line.split_once(' ').unwrap();
            let opponent = Shape::parse(opponent);
            let choose = Shape::parse(choose);
            choose.score() + choose.compete(opponent)
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, guide) = line.split_once(' ').unwrap();
            let opponent = Shape::parse(opponent);
            let guide = Guide::parse(guide);
            let choose = opponent.choose(guide);
            choose.score() + choose.compete(opponent)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 15);
        assert_eq!(part_two(EXAMPLE), 12);
    }
}
