use std::collections::HashMap;

enum Unit {
    Rock,
    Sand,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Unit::Rock => write!(f, "#")?,
            &Unit::Sand => write!(f, "o")?,
        }
        Ok(())
    }
}

#[derive(Default)]
struct Cave {
    units: HashMap<(i32, i32), Unit>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    source: (i32, i32),
}

impl Cave {
    fn new(paths: &Vec<Vec<(i32, i32)>>) -> Self {
        let mut cave = Cave::default();
        cave.add_source(500, 0);
        for path in paths {
            path.windows(2).for_each(|coords| {
                let (sx, sy) = coords[0];
                let (ex, ey) = coords[1];
                if sx == ex {
                    let min = sy.min(ey);
                    let max = sy.max(ey);
                    for y in min..=max {
                        cave.add_rock(sx, y);
                    }
                } else {
                    let min = sx.min(ex);
                    let max = sx.max(ex);
                    for x in min..=max {
                        cave.add_rock(x, sy);
                    }
                }
            });
        }
        cave
    }

    fn add_source(&mut self, x: i32, y: i32) {
        self.source = (x, y);
        self.min_x = x;
        self.max_x = x;
        self.min_y = y;
        self.max_y = y;
    }

    fn add_rock(&mut self, x: i32, y: i32) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.units.insert((x, y), Unit::Rock);
    }

    fn is_out(&self, x: i32, y: i32) -> bool {
        x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y
    }

    fn is_air(&self, x: i32, y: i32) -> bool {
        self.units.get(&(x, y)).is_none()
    }

    fn add_sand(&mut self, x: i32, y: i32) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.units.insert((x, y), Unit::Sand);
    }

    fn fall_sand(&mut self) -> bool {
        let mut x = self.source.0;
        let mut y = self.source.1;

        loop {
            // down
            if self.is_out(x, y + 1) {
                return false;
            }
            if self.is_air(x, y + 1) {
                y += 1;
                continue;
            }

            // left down
            if self.is_out(x - 1, y + 1) {
                return false;
            }
            if self.is_air(x - 1, y + 1) {
                x -= 1;
                y += 1;
                continue;
            }

            // right down
            if self.is_out(x + 1, y + 1) {
                return false;
            }
            if self.is_air(x + 1, y + 1) {
                x += 1;
                y += 1;
                continue;
            }

            if (x, y) == self.source {
                if !self.is_air(x, y) {
                    return false;
                }
            }
            self.add_sand(x, y);
            return true;
        }
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if x == self.source.0 && y == self.source.1 {
                    write!(f, "+")?;
                } else {
                    if x == self.source.0 && y == self.source.1 {
                        write!(f, "+")?;
                    } else if let Some(unit) = self.units.get(&(x, y)) {
                        match unit {
                            &Unit::Rock => write!(f, "#")?,
                            &Unit::Sand => write!(f, "o")?,
                        }
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_paths(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> usize {
    let paths = parse_paths(input);

    let mut cave = Cave::new(&paths);

    //println!("{cave}\n");
    let mut count = 0;
    while cave.fall_sand() {
        count += 1;
        //println!("{cave}\n");
    }
    count
}

pub fn part_two(input: &str) -> usize {
    let paths = parse_paths(input);

    let mut cave = Cave::new(&paths);
    let y = cave.max_y + 2;
    for x in 500-y..=500+y {
        cave.add_rock(x, y);
    }

    //println!("{cave}\n");
    let mut count = 0;
    while cave.fall_sand() {
        count += 1;
        //println!("{cave}\n");
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 24);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(EXAMPLE), 93);
    }
}
