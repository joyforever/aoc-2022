use day_15_beacon_exclusion_zone::{part_one, part_two};

const INPUT: &str = include_str!("../data/input.txt");

fn main() {
    println!("part one answer: {}", part_one(INPUT, 2000000));
    println!("part two answer: {}", part_two(INPUT, 4000000));
}
