use std::collections::{HashSet, HashMap};

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

fn count(m: &mut HashMap<char, u32>, s: &HashSet<char>) {
    for c in s {
        let e = m.entry(*c).or_default();
        *e += 1;
    }
}

pub fn part_two(input: &str) -> u32 {
    let lines: Vec<HashSet<char>> = input.lines().map(|line| str_to_set(line)).collect();

    let mut priorities = vec![];

    for i in (0..lines.len()).step_by(3) {
        let mut m = HashMap::new();

        count(&mut m, &lines[i]);
        count(&mut m, &lines[i+1]);
        count(&mut m, &lines[i+2]);

        if let Some((c, _)) = m.iter().filter(|(_, count)| **count == 3).next() {
            priorities.push(char_to_priority(*c));
        } else {
            priorities.push(0);
        }
    }

    priorities.iter().sum()
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
