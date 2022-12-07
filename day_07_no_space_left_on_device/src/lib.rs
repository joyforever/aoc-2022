use std::collections::BTreeMap;

fn to_string(v: &Vec<String>) -> String {
    if v.is_empty() {
        "".to_string()
    } else if v.len() == 1 {
        "/".to_string()
    } else {
        let mut s = String::from("/");
        s.push_str(v[1..].join("/").as_str());
        s
    }
}

pub fn part_one(input: &str) -> usize {
    let mut files_map = BTreeMap::new();
    let mut curr_path = Vec::new();
    let mut files = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            if !files.is_empty() {
                files_map.insert(to_string(&curr_path), files.clone());
                files.clear();
            }

            let path = &line[5..];
            if path == ".." {
                curr_path.pop();
            } else {
                curr_path.push(path.to_string());
            }
        } else if line.starts_with("$ ls") {
            files.clear();
        } else if line.starts_with("dir ") {
            // skip
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            files.push((size, name.to_string()));
        }
    }

    files_map
        .iter()
        .map(|(path, v)| {
            let total = v.iter().fold(0, |total, (size, _)| total + *size);
            println!("path={}, total={}", path, total);
            total
        })
        .filter(|&size| size <= 100000)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 95437);
    }
}
