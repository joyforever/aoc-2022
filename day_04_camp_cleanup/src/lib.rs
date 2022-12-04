#[derive(Clone, Copy)]
struct SectionRange {
    min: u32,
    max: u32,
}

impl SectionRange {
    fn contains(&self, sr: SectionRange) -> bool {
        self.min <= sr.min && self.max >= sr.max
    }
}

impl From<&str> for SectionRange {
    fn from(s: &str) -> Self {
        let (min, max) = s.split_once('-').unwrap();
        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (sr1, sr2) = line.split_once(',').unwrap();
            let sr1 = SectionRange::from(sr1);
            let sr2 = SectionRange::from(sr2);
            sr1.contains(sr2) || sr2.contains(sr1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 2);
    }
}
