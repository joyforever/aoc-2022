use day_02_rock_paper_scissors::{part_one, part_two};

const INPUT: &str = include_str!("../data/input.txt");

fn main() {
    println!("part one answer: {}", part_one(INPUT));
    println!("part two answer: {}", part_two(INPUT));
}