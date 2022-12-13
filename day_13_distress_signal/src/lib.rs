use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    Number(char),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let mut lists = vec![vec![]];

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

fn is_ordered(first: &Packet, second: &Packet) -> Option<bool> {
    match (&first, &second) {
        (&Packet::Number(v1), &Packet::Number(v2)) => {
            if *v1 < *v2 {
                Some(true)
            } else if *v1 > *v2 {
                Some(false)
            } else {
                None
            }
        }
        (&Packet::Number(v), &Packet::List(..)) => {
            let list = Packet::List(vec![Packet::Number(*v)]);
            is_ordered(&list, second)
        }
        (&Packet::List(..), &Packet::Number(v)) => {
            let list = Packet::List(vec![Packet::Number(*v)]);
            is_ordered(first, &list)
        }
        (&Packet::List(v1), &Packet::List(v2)) => {
            let v1_len = v1.len();
            let v2_len = v2.len();
            for i in 0..v1_len.min(v2_len) {
                if let Some(o) = is_ordered(&v1[i], &v2[i]) {
                    return Some(o);
                }
            }
            if v1_len < v2_len {
                return Some(true);
            } else if v1_len > v2_len {
                return Some(false);
            }
            None
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
            if let Some(ordered) = is_ordered(&left, &right) {
                if ordered {
                    Some(index + 1)
                } else {
                    None
                }
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
        if let Some(o) = is_ordered(&first, &second) {
            if o {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
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
