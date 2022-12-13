#[derive(Debug)]
enum Packet {
    Number(char),
    List(Vec<Packet>),
}

fn parse_packets(s: &[char]) -> Vec<Vec<Packet>> {
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

    lists
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

    for (left, right) in &pairs {
        println!("{left:?}");
        let left_packets = parse_packets(&left);
        println!("{:?}", left_packets);

        println!("{right:?}");
        parse_packets(&right);

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
