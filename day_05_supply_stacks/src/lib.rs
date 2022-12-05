use std::collections::HashMap;

type StackCrates = HashMap<usize, Vec<char>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}


fn line_to_crates(line: &str) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    for i in (1..).step_by(4) {
        if i > line.len() {
            break;
        }
        v.push(line.as_bytes()[i] as char)
    }
    v
}

fn insert_map(mut m: StackCrates, line: &str) -> StackCrates {
    let crates = line_to_crates(line);
    for (i, c) in crates.iter().enumerate() {
        let v = m.entry(i + 1).or_default();
        if *c != ' ' {
            v.push(*c);
        }
    }
    m
}

fn to_move(line: &str) -> Move {
    let vs = line
        .split_ascii_whitespace()
        .filter_map(|s| {
            match s.parse::<usize>() {
                Ok(num) => Some(num),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>();
    Move { count: vs[0], from: vs[1], to: vs[2], }
}

pub fn part_one(input: &str) -> String {
    let (stacks, moves) = input
        .split_once("\n\n")
        .unwrap();

    let mut stacks = stacks
        .lines()
        .rev()
        .skip(1)
        .fold(HashMap::new(), |m, line| {
            insert_map(m, line)
        });

    let moves = moves
        .lines()
        .map(|line| to_move(line))
        .collect::<Vec<_>>();

    for m in moves.iter() {
        for _ in 0..m.count {
            let char = stacks.get_mut(&m.from).unwrap().pop().unwrap();
            stacks.get_mut(&m.to).unwrap().push(char);
        }
    }

    let mut tops = Vec::new();
    for i in 1..=stacks.len() {
        let v = stacks.get(&i).unwrap();
        tops.push(*v.get(v.len() - 1).unwrap());
    }
    
    String::from_iter(tops)
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
