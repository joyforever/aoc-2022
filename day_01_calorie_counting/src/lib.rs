fn sum_calories(calories: &str) -> u32 {
    calories
        .lines()
        .map(|c| c.parse::<u32>().unwrap())
        .sum()
}

pub fn part_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(sum_calories)
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(sum_calories)
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 24000);
        assert_eq!(part_two(EXAMPLE), 45000);
    }
}
