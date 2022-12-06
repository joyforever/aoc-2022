pub fn part_one(input: &str) -> usize {
    let b = input.as_bytes();
    let n = b.len() - 4;
    for i in 0..=n {
        let all_different = 
            b[i] != b[i+1] &&
            b[i] != b[i+2] &&
            b[i] != b[i+3] &&
            b[i+1] != b[i+2] &&
            b[i+1] != b[i+3] &&
            b[i+2] != b[i+3];
        if all_different {
            return i + 4;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 7);
    }
}
