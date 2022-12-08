pub fn part_one(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let rows = map.len();
    let cols = map[0].len();

    let edge_visible = rows * 2 + (cols - 2) * 2;
    let mut interior_visible = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let height = map[row][col];

            let is_visiable = 
                (0..row).all(|r| map[r][col] < height) ||
                (row+1..rows).all(|r| map[r][col] < height) ||
                (0..col).all(|c| map[row][c] < height) ||
                (col+1..cols).all(|c| map[row][c] < height);
            if is_visiable {
                interior_visible += 1;
            }
        }
    }

    edge_visible + interior_visible
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 21);
    }
}
