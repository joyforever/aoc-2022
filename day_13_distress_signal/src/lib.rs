pub fn part_one(input: &str) -> usize {
    let pairs = input
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .map(|(left, right)| {
            (left.chars().collect::<Vec<_>>(), right.chars().collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();
    
    for (left, right) in &pairs {
        println!("{left:?}");
        println!("{right:?}");
        println!();
    }    

    13
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }
}
