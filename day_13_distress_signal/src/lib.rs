#[derive(Debug)]
enum Packet {
    Number(char),
    List(Vec<Packet>),
}

fn parse_packets(s: &[char]) -> Packet {
    let mut lists = vec![vec![]];

    for &c in s {
        match c {
            '[' => {
                lists.push(vec![]);
            },
            ']' => {
                let list = lists.pop().unwrap();
                lists.last_mut().unwrap().push(Packet::List(list));
            },
            ',' => {
                // do nothing
            },
            n => {
                lists.last_mut().unwrap().push(Packet::Number(n));
            },
        }
    }

    Packet::List(std::mem::take(&mut lists[0]))
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
        },
        (&Packet::Number(v), &Packet::List(..)) => {
            let list = Packet::List(vec![Packet::Number(*v)]);
            is_ordered(&list, second)
        },
        (&Packet::List(..), &Packet::Number(v)) => {
            let list = Packet::List(vec![Packet::Number(*v)]);
            is_ordered(first, &list)
        },
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
        .map(|(left, right)| {
            let left = left.replace("10", "A");
            let right = right.replace("10", "A");
            (left.chars().collect::<Vec<_>>(), right.chars().collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();
    
    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, (left, right))| {
            let left = parse_packets(left);
            let right = parse_packets(right);
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13);
    }
}
