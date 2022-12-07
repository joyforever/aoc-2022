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
            Item::Dir { size, .. } => *size,
            Item::File { size, .. } => *size,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            &Item::Dir { .. } => true,
            _ => false,
        }
    }

    fn default_dir() -> Item {
        Item::Dir { name: "".to_string(), size: 0, items: vec![], }
    }
}

fn parse_item(mut index: usize, lines: &[&str], item: &mut Item) -> usize {
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

            let mut dir_item = Item::default_dir();
            index = parse_item(index + 1, lines, &mut dir_item);
            if let Item::Dir { size, items, .. } = item {
                *size += dir_item.size();
                items.push(dir_item);
            }
        } else if line.starts_with("dir ") {
            index += 1;
        } else {
            let (file_size, file_name) = line.split_once(' ').unwrap();
            let file_size = file_size.parse::<usize>().unwrap();
            let file_name = file_name.to_string();
            if let Item::Dir { size, items, .. } = item {
                *size += file_size;
                items.push(Item::File { name: file_name, size: file_size, });
            }
            index += 1;
        }
    }
    index
}

fn collect_dir_sizes(item: &Item) -> Vec<usize> {
    let mut v = Vec::new();
    if let Item::Dir { size, items, .. } = item {
        v.push(*size);
        for i in items {
            if i.is_dir() {
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
    let item = parse_file_system(input);

    let sizes = collect_dir_sizes(&item);
    sizes
        .iter()
        .filter(|&&size| size <= 100000)
        .sum::<usize>()
}

pub fn part_two(input: &str) -> usize {
    let item = parse_file_system(input);

    let mut sizes = collect_dir_sizes(&item);
    sizes.sort();
    let disk_space = sizes.last().unwrap();
    let size = sizes
        .iter()
        .find(|&&size| disk_space - size < 40000000)
        .unwrap();
    *size
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
