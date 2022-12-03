use std::collections::HashSet;

fn str_to_set(s: &str) -> HashSet<char> {
    s.chars().fold(HashSet::new(), |mut s, c| {
        s.insert(c);
        s
    })
}

fn char_to_priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let n = line.len();
            let first = str_to_set(&line[..n/2]);
            let second = str_to_set(&line[n/2..]);
            let common = first.intersection(&second).collect::<HashSet<_>>();
            if let Some(&c) = common.iter().next() {
                char_to_priority(*c)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 157);
    }
}
