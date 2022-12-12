use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
    prelude::DiGraphMap,
};

struct Grid(Vec<Vec<char>>);

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self(grid)
    }
}

type Node = (usize, usize, char);

impl Grid {
    fn find_char(&self, ch: char) -> Option<Node> {
        for (i, v) in self.0.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if *c == ch {
                    return Some((i, j, ch));
                }
            }
        }
        None
    }

    fn to_edges(&self) -> Vec<(Node, Node)> {
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut edges = vec![];
        for i in 0..rows {
            for j in 0..cols {
                let translate = |c| match c {
                    'S' => (c, 'a'),
                    'E' => (c, 'z'),
                    c => (c, c),
                };

                // o: original
                // m: mapped
                let (oc, mc) = translate(self.0[i][j]);

                // left
                if j > 0 {
                    let (left_oc, left_mc) = translate(self.0[i][j - 1]);
                    if left_mc as i32 - mc as i32 <= 1 {
                        edges.push(((i, j, oc), (i, j - 1, left_oc)));
                    }
                }
                // right
                if j < cols - 1 {
                    let (right_oc, right_mc) = translate(self.0[i][j + 1]);
                    if right_mc as i32 - mc as i32 <= 1 {
                        edges.push(((i, j, oc), (i, j + 1, right_oc)));
                    }
                }
                // up
                if i > 0 {
                    let (up_oc, up_mc) = translate(self.0[i - 1][j]);
                    if up_mc as i32 - mc as i32 <= 1 {
                        edges.push(((i, j, oc), (i - 1, j, up_oc)));
                    }
                }
                // down
                if i < rows - 1 {
                    let (down_oc, down_mc) = translate(self.0[i + 1][j]);
                    if down_mc as i32 - mc as i32 <= 1 {
                        edges.push(((i, j, oc), (i + 1, j, down_oc)));
                    }
                }
            }
        }

        edges
    }
}

#[allow(dead_code)]
fn print_graph(graph: &DiGraphMap<Node, ()>) {
    // see graph on: http://viz-js.com/
    println!("{:?}", Dot::with_config(graph, &[Config::EdgeNoLabel]));
}

pub fn part_one(input: &str) -> usize {
    let grid = Grid::from(input);
    let start = grid.find_char('S').unwrap();
    let end = grid.find_char('E').unwrap();

    let edges = grid.to_edges();
    let graph = DiGraphMap::<(usize, usize, char), ()>::from_edges(&edges);
    //print_graph(&graph);

    let res = dijkstra(&graph, start, None, |_| 1usize);
    *res.get(&end).unwrap()
}

pub fn part_two(input: &str) -> usize {
    let grid = Grid::from(input);
    let end = grid.find_char('E').unwrap();

    let edges = grid.to_edges();
    let edges = edges.iter().map(|(v1, v2)| (*v2, *v1)).collect::<Vec<_>>();
    let graph = DiGraphMap::<(usize, usize, char), ()>::from_edges(&edges);

    let res = dijkstra(&graph, end, None, |_| 1usize);
    res.iter()
        .filter_map(|(index, value)| {
            if index.2 == 'a' || index.2 == 'S' {
                Some(*value)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 31);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 29);
    }
}
