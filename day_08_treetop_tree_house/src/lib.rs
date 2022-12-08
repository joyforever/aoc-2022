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

pub fn part_two(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let rows = map.len();
    let cols = map[0].len();

    let mut highest_score = 0;

    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let height = map[row][col];
            let mut score = 1;

            // looking left
            if let Some(index) = (0..row).rev().find(|&i| map[i][col] >= height) {
                score *= row.abs_diff(index); // blocked
            } else {
                score *= row; // not blocked
            }
            // looking right
            if let Some(index) = (row+1..rows).find(|&i| map[i][col] >= height) {
                score *= row.abs_diff(index); // blocked
            } else {
                score *= row.abs_diff(rows - 1); // not blocked
            }
            // looking up
            if let Some(index) = (0..col).rev().find(|&i| map[row][i] >= height) {
                score *= col.abs_diff(index); // blocked
            } else {
                score *= col; // not blocked
            }
            // looking down
            if let Some(index) = (col+1..cols).find(|&i| map[row][i] >= height) {
                score *= col.abs_diff(index);
            } else {
                score *= col.abs_diff(cols - 1);
            }

            //println!("({}, {}) = {}", row, col, score);

            if highest_score < score {
                highest_score = score;
            }
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 21);
        assert_eq!(part_two(EXAMPLE), 8);
    }
}
