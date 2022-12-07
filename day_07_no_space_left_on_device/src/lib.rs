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

pub fn part_one(input: &str) -> usize {
    let lines = input.lines().skip(1).collect::<Vec<_>>();

    let mut root = Item::Dir { name: "/".to_string(), size: 0, items: vec![], };
    parse_item(1, &lines, &mut root);

    find_directories(&root)
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
