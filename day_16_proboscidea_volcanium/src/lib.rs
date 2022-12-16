use std::collections::BTreeMap;
use petgraph::prelude::DiGraphMap;

pub fn part_one(input: &str) -> i32 {
    let mut valves = BTreeMap::new();
    let mut edges = Vec::new();

    for line in input.trim().lines() {
        let (valve, tunnels) = line.split_once("; ").unwrap();

        let tokens = valve.split(' ').collect::<Vec<&str>>();
        let name = tokens[1];
        let rate = *&tokens[4][5..].parse::<i32>().unwrap();
        valves.insert(name, rate);

        if tunnels.starts_with("tunnels") {
            let prefix = "tunnels lead to valves ";
            tunnels[prefix.len()..].split(", ").for_each(|t| {
                edges.push((name, t));
            });
        } else {
            let prefix = "tunnel leads to valve ";
            edges.push((name, &tunnels[prefix.len()..]))
        }
    }

    println!("{valves:?}");
    println!("{edges:?}");

    let edges = edges
        .iter()
        .map(|&(x, y)| ((x, valves[x]), (y, valves[y])))
        .collect::<Vec<_>>();

    let graph = DiGraphMap::<(&str, i32), ()>::from_edges(&edges);
    println!("{graph:?}");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 1651);
    }
}
