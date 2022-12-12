use petgraph::{algo::dijkstra, prelude::DiGraphMap};

struct HeightMap {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    graph: DiGraphMap<(usize, usize), ()>,
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut start = (0, 0);
        let mut end = (0, 0);

        let rows = grid.len();
        let cols = grid[0].len();

        let mut edges = vec![];
        for i in 0..rows {
            for j in 0..cols {
                let c = grid[i][j];

                let c = match c {
                    'S' => {
                        start.0 = i;
                        start.1 = j;
                        'a'
                    }
                    'E' => {
                        end.0 = i;
                        end.1 = j;
                        'z'
                    }
                    c => c,
                };

                let translate = |c| match c {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                };

                // left
                if j > 0 {
                    let left_c = translate(grid[i][j - 1]);
                    if left_c as i32 - c as i32 <= 1 {
                        edges.push(((i, j), (i, j - 1)));
                    }
                }
                // right
                if j < cols - 1 {
                    let right_c = translate(grid[i][j + 1]);
                    if right_c as i32 - c as i32 <= 1 {
                        edges.push(((i, j), (i, j + 1)));
                    }
                }
                // up
                if i > 0 {
                    let up_c = translate(grid[i - 1][j]);
                    if up_c as i32 - c as i32 <= 1 {
                        edges.push(((i, j), (i - 1, j)));
                    }
                }
                // down
                if i < rows - 1 {
                    let down_c = translate(grid[i + 1][j]);
                    if down_c as i32 - c as i32 <= 1 {
                        edges.push(((i, j), (i + 1, j)));
                    }
                }
            }
        }

        let graph = DiGraphMap::<(usize, usize), ()>::from_edges(&edges);
        //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

        Self {
            grid,
            start,
            end,
            graph,
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let map = HeightMap::from(input);
    let res = dijkstra(&map.graph, map.start, None, |_| 1usize);

    *res.get(&map.end).unwrap()
}

pub fn part_two(input: &str) -> usize {
    let map = HeightMap::from(input);
    let grid = &map.grid;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut paths = vec![];
    for i in 0..rows {
        for j in 0..cols {
            let c = grid[i][j];
            if c == 'a' || c == 'S' {
                let res = dijkstra(&map.graph, (i, j), None, |_| 1usize);
                if let Some(v) = res.get(&map.end) {
                    paths.push(*v);
                }
            }
        }
    }

    paths.sort();
    paths[0]
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
