use std::collections::HashSet;

#[derive(Default)]
struct Cave {
    rocks: HashSet<(usize, usize)>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    source: (usize, usize),
}

impl Cave {
    fn set_source(&mut self, x: usize, y: usize) {
        self.source = (x, y);
        self.min_x = x;
        self.max_x = x;
        self.min_y = y;
        self.max_y = y;
        self.add_rock(x, y);
    }

    fn add_rock(&mut self, x: usize, y: usize) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.rocks.insert((x, y));
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if x == self.source.0 && y == self.source.1 {
                    write!(f, "+")?;
                } else {
                    if self.rocks.get(&(x, y)).is_none() {
                        write!(f, ".")?;
                    } else {
                        write!(f, "#")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> usize {
    let paths = input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    let mut cave = Cave::default();
    cave.set_source(500, 0);
    for path in &paths {
        path
            .windows(2)
            .for_each(|coords| {
                let (sx, sy) = coords[0];
                let (ex, ey) = coords[1];
                if sx == ex {
                    let min = sy.min(ey);
                    let max = sy.max(ey);
                    for y in min..=max {
                        cave.add_rock(sx, y);
                    }
                } else /* sy == ey */ {
                    let min = sx.min(ex);
                    let max = sx.max(ex);
                    for x in min..=max {
                        cave.add_rock(x, sy);
                    }
                }
            });
    }

    println!("{cave}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 0);
    }
}
