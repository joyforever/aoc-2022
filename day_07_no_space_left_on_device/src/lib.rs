#![allow (dead_code)]

#[derive(Debug)]
enum Item {
    Dir {
        name: String,
        size: usize,
        items: Vec<Item>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl Item {
    fn size(&self) -> usize {
        match self {
            Item::Dir { size, name: _, items: _, } => *size,
            Item::File { size, name: _, } => *size,
        }
    }
}

fn parse_item(mut index: usize, lines: &[&str], dir: &mut Item) -> usize {
    loop {
        if index >= lines.len() {
            break;
        }

        let line = lines[index];
        if line.starts_with("$ ls") {
            index += 1;
        } else if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == ".." {
                index += 1;
                return index;
            }

            let mut dir_item = Item::Dir {
                name: name.to_string(),
                size: 0,
                items: vec![],
            };
            index = parse_item(index + 1, lines, &mut dir_item);
            if let Item::Dir { name: _, size, items, } = dir {
                *size += dir_item.size();
                items.push(dir_item);
            }
        } else if line.starts_with("dir ") {
            index += 1;
        } else {
            let (file_size, file_name) = line.split_once(' ').unwrap();
            let file_size = file_size.parse::<usize>().unwrap();
            let file_name = file_name.to_string();
            if let Item::Dir { name: _, size, items, } = dir {
                *size += file_size;
                items.push(Item::File { name: file_name, size: file_size, });
            }
            index += 1;
        }
    }
    index
}

fn find_directories(item: &Item) -> usize {
    let mut count = 0;
    if let Item::Dir { name: _, size, items } = item {
        if *size <= 100000 {
            count += *size;
        }
        for sub_item in items {
            count += find_directories(sub_item);
        }
    }
    count
}

fn collect_dir_sizes(item: &Item) -> Vec<usize> {
    let mut v = Vec::new();
    if let Item::Dir { name: _, size, items, } = item {
        v.push(*size);
        for i in items {
            if let Item::Dir { name: _, size: _, items: _ } = i {
                let mut dirs = collect_dir_sizes(i);
                v.append(&mut dirs);
            }
        }
    }
    v
}

fn parse_file_system(input: &str) -> Item {
    let lines = input.lines().skip(1).collect::<Vec<_>>();

    let mut root = Item::Dir { name: "/".to_string(), size: 0, items: vec![], };
    parse_item(1, &lines, &mut root);

    root
}

pub fn part_one(input: &str) -> usize {
    let root = parse_file_system(input);
    find_directories(&root)
}

pub fn part_two(input: &str) -> usize {
    let root = parse_file_system(input);

    let mut sizes = collect_dir_sizes(&root);
    sizes.sort();
    let left_size = sizes.last().unwrap();
    *sizes.iter().find(|s| left_size - **s < 40000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn it_works() {
        assert_eq!(part_one(EXAMPLE), 95437);
        assert_eq!(part_two(EXAMPLE), 24933642);
    }
}
