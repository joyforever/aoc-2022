pub fn part_one(input: &str) -> usize {
    let paths = input
        .trim()
        .lines()
        .map(|line| line
            .split(" -> ")
            .map(|s| s.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    println!("{paths:?}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 0);
    }
}
