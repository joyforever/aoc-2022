use std::collections::{HashMap, HashSet};

fn parse_position(tag: &str, s: &str) -> (i32, i32) {
    let n = tag.len();
    let s = &s[n..];
    let (x, y) = s.split_once(", ").unwrap();
    let x = x[2..].parse::<i32>().unwrap();
    let y = y[2..].parse::<i32>().unwrap();
    (x, y)
}

pub fn part_one(input: &str, line: i32) -> usize {
    let pairs = input
        .trim()
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once(": ").unwrap();
            let sensor = parse_position("Sensor at ", s1);
            let beacon = parse_position("closest beacon is at ", s2);
            (sensor, beacon)
        })
        .collect::<Vec<_>>();

    let mut map = HashMap::new();
    for (sensor, beacon) in &pairs {
        map.insert(*sensor, 'S');
        map.insert(*beacon, 'B');
    }

    let mut set = HashSet::new();

    for (sensor, beacon) in &pairs {
        let n = sensor.0.abs_diff(beacon.0) as i32 + sensor.1.abs_diff(beacon.1) as i32;
        //println!("n = {n}");
        for i in -n..=n {
            let y = sensor.1 + i;
            if y != line {
                continue;
            }

            let m = n - i.abs();
            for j in -m..=m {
                let x = sensor.0 + j;
                //println!("({i},{j}) = ({x}, {y})");
                if map.get(&(x, y)).is_none() {
                    set.insert((x, y));
                }
            }
        }
    }

    set.len()
}

fn distance(a: &(i32, i32), b: &(i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part_two(input: &str, max: i32) -> i32 {
    let pairs = input
        .trim()
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once(": ").unwrap();
            let sensor = parse_position("Sensor at ", s1);
            let beacon = parse_position("closest beacon is at ", s2);
            (sensor, beacon)
        })
        .collect::<Vec<_>>();

    let is_ok = |x: i32, y: i32| {
        x >= 0
            && x <= max
            && y >= 0
            && y <= max
            && pairs
                .iter()
                .all(|(s, b)| distance(s, &(x, y)) > distance(s, b))
    };

    for i in 1.. {
        for (_sensor, beacon) in &pairs {
            // left && right
            for x in [beacon.0 - i, beacon.1 + i] {
                for y in beacon.1 - i..=beacon.1 + i {
                    if is_ok(x, y) {
                        println!("{x}, {y}");
                        return x * 4000000 + y;
                    }
                }
            }
            // up && down
            for y in [beacon.1 - i, beacon.1 + i] {
                for x in beacon.0 - i + 1..beacon.0 + i {
                    if is_ok(x, y) {
                        println!("{x}, {y}");
                        return x * 4000000 + y;
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE, 10), 26);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE, 20), 56000011);
    }
}
