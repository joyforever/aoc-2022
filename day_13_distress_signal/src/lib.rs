use std::cmp::Ordering;

#[derive(Debug, Eq, PartialOrd)]
enum Packet {
    Number(char),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let mut lists = vec![vec![]];

        let s = &s[1..s.len()-1];
        for c in s.chars() {
            match c {
                '[' => {
                    lists.push(vec![]);
                }
                ']' => {
                    let list = lists.pop().unwrap();
                    lists.last_mut().unwrap().push(Packet::List(list));
                }
                ',' => {
                    // do nothing, just skip
                }
                n => {
                    lists.last_mut().unwrap().push(Packet::Number(n as char));
                }
            }
        }

        Packet::List(std::mem::take(&mut lists[0]))
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Packet::Number(n) => {
                if *n == 'A' {
                    write!(f, "10")?;
                } else {
                    write!(f, "{n}")?;
                }
            },
            &Packet::List(v) => {
                write!(f, "[")?;
                for (i, p) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    p.fmt(f)?;
                }
                write!(f, "]")?;
             }
        }
        Ok(())
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Number(a), Self::List(b)) => {
                &vec![Packet::Number(*a)] == b
            },
            (Self::List(a), Self::Number(b)) => {
                a == &vec![Packet::Number(*b)]
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) =>  a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => {
                a.cmp(&vec![Packet::Number(*b)])
            },
            (Packet::Number(a), Packet::List(b)) => {
                vec![Packet::Number(*a)].cmp(&b)
            },
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let pairs = input
        .trim()
        .split("\n\n")
        .map(|pair| pair.split_once('\n').unwrap())
        .map(|(left, right)| (left.replace("10", "A"), right.replace("10", "A")))
        .collect::<Vec<_>>();

    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, (left, right))| {
            let left = Packet::from(left.as_str());
            let right = Packet::from(right.as_str());
            if left.cmp(&right).is_lt() {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut packets = input
        .trim()
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.replace("10", "A"))
            }
        })
        .collect::<Vec<_>>();

    let divider_packets = vec!["[[2]]", "[[6]]"];
    for dp in &divider_packets {
        packets.push(dp.to_string());
    }

    packets.sort_by(|first, second| {
        let first = Packet::from(first.as_str());
        let second = Packet::from(second.as_str());
        first.cmp(&second)
    });

    let mut indices = [0; 2];
    let mut found_count = 0;

    for (index, packet) in packets.iter().enumerate() {
        //println!("{index}: {packet}");
        if packet == divider_packets[0] {
            indices[0] = index + 1;
            found_count += 1;
            if found_count == 2 {
                break;
            }
        } else if packet == divider_packets[1] {
            indices[1] = index + 1;
            found_count += 1;
            if found_count == 2 {
                break;
            }
        }
        
    }

    indices[0] * indices[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 140);
    }
}
