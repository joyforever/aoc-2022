use std::collections::{HashMap, HashSet};

fn parse_position(tag: &str, s: &str) -> (i32, i32) {
    let n = tag.len();
    let s = &s[n..];
    let (x, y) = s.split_once(", ").unwrap();
    let x = x[2..].parse::<i32>().unwrap();
    let y = y[2..].parse::<i32>().unwrap();
    (x, y)
}

pub fn part_one(input: &str) -> usize {
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
        map.insert(sensor, 'S');
        map.insert(beacon, 'B');
    }

    let mut set = HashSet::new();

    for (sensor, beacon) in &pairs {
        let n = sensor.0.abs_diff(beacon.0) as i32 + sensor.1.abs_diff(beacon.1) as i32;
        for i in -n..=n {
            for j in -(n-i)..=n-i {
                let x = sensor.0 + i;
                let y = sensor.1 + j;
                if y == 10 {
                    set.insert((x, y));
                }
            }
        }
    }

    for s in &set {
        println!("{s:?}");
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 26);
    }
}
