use day_01_calorie_counting::part_one;
use day_01_calorie_counting::part_two;

const INPUT: &str = include_str!("../data/input.txt");

fn main() {
    println!("part one answer: {}", part_one(INPUT));
    println!("part two answer: {}", part_two(INPUT));
}