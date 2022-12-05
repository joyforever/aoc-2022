pub fn part_one(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), "CMZ".to_string());
    }
}
