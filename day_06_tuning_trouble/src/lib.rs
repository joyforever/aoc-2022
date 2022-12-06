use std::collections::BTreeSet;

fn detect<const N: usize>(input: &str) -> usize {
    let index = input
        .as_bytes()
        .windows(N)
        .enumerate()
        .map(|(index, digits)| {
            (index, digits.iter().collect::<BTreeSet<_>>())
        })
        .find(|(_, set)| set.len() == N)
        .unwrap();
    index.0 + N
}

pub fn part_one(input: &str) -> usize {
    detect::<4>(input)
}

pub fn part_two(input: &str) -> usize {
    detect::<14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
