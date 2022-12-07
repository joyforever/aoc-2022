#[derive(Debug)]
enum Item {
    Dir {
        name: String,
        items: Vec<Item>,
    },
    File {
        name: String,
        size: usize,
    },
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
                items: vec![],
            };
            index = parse_item(index + 1, lines, &mut dir_item);
            if let Item::Dir { items, name: _ } = dir {
                items.push(dir_item);
            }
        } else if line.starts_with("dir ") {
            index += 1;
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            let size = size.parse::<usize>().unwrap();
            let name = name.to_string();
            if let Item::Dir { items, name: _ } = dir {
                items.push(Item::File { name, size });
            }
            index += 1;
        }
    }
    index
}

pub fn part_one(input: &str) -> usize {
    let lines = input.lines().skip(1).collect::<Vec<_>>();
    let mut root = Item::Dir { name: "/".to_string(), items: vec![], };
    parse_item(1, &lines, &mut root);

    dbg!(root);

    0
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
