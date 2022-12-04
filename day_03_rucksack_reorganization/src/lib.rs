use std::collections::HashSet;

fn str_to_set(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    set
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
            let n = line.len() / 2;
            let first = str_to_set(&line[..n]);
            let second = str_to_set(&line[n..]);
            let common = first.iter().find(|c| second.contains(*c)).unwrap();
            char_to_priority(*common)
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines().map(|line| str_to_set(line)).collect::<Vec<_>>();

    lines
        .chunks_exact(3)
        .map(|v| {
            let s1 = &v[0];
            let s2 = &v[1];
            let s3 = &v[2];
            let common = s1.iter().find(|c| s2.contains(*c) && s3.contains(*c)).unwrap();
            char_to_priority(*common)
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
        assert_eq!(part_two(EXAMPLE), 70);
    }
}
