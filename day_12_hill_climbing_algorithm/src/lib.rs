use petgraph::{prelude::DiGraphMap, dot::{Dot, Config}, algo::dijkstra};

pub fn part_one(input: &str) -> usize {
    let grid = input
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

            let translate = |c| {
                match c {
                    'S' => 'a',
                    'E' => 'z',
                    c => c,
                }
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
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let res = dijkstra(&graph, start, None, |_| 1usize);
    for (index, value) in &res {
        println!("{:?} = {}", index, value);
    }

    *res.get(&end).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 31);
    }
}
