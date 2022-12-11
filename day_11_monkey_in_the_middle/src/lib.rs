use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Num(usize),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        if s == "old" {
            Operand::Old
        } else {
            Operand::Num(s.parse::<usize>().unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add, Mul,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        if s == "+" {
            Operator::Add
        } else {
            Operator::Mul
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible_by: usize,
    throw_if_true: usize,
    throw_if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<usize>,
    operation: (Operand, Operator, Operand),
    test: Test,
}

impl Monkey {
    fn parse_starting_items(line: &str) -> Vec<usize> {
        let (_, starting_items) = line.split_once(':').unwrap();
        starting_items.split(',')
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }

    fn parse_operation(line: &str) -> (Operand, Operator, Operand) {
        let operation = line
            .split(' ')
            .rev()
            .take(3)
            .collect::<Vec<_>>();
        (Operand::from(operation[2]), Operator::from(operation[1]), Operand::from(operation[0]))
    }
}

fn parse_last_number(s: &str) -> usize {
    s
        .split(' ')
        .rev()
        .take(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1);
        
        let starting_items = Monkey::parse_starting_items(lines.next().unwrap());
        let operation = Monkey::parse_operation(lines.next().unwrap());

        let divisible_by = parse_last_number(lines.next().unwrap());
        let throw_if_true = parse_last_number(lines.next().unwrap());
        let throw_if_false = parse_last_number(lines.next().unwrap());
        let test = Test { divisible_by, throw_if_true, throw_if_false, };

        Self { starting_items, operation, test, }
    }
}

fn calculate_worry_level(op: &(Operand, Operator, Operand), old: usize) -> usize {
    let x = match op.0 {
        Operand::Old => old,
        Operand::Num(value) => value,
    };
    let y = match op.2 {
        Operand::Old => old,
        Operand::Num(value) => value,
    };
    let level = match op.1 {
        Operator::Add => x + y,
        Operator::Mul => x * y,
    };
    level / 3
}

pub fn part_one(input: &str) -> usize {
    let mut monkeys = input.split("\n\n")
        .map(Monkey::from)
        .collect::<Vec<_>>();
    
    let mut inspect_count = BTreeMap::new();
    
    for round in 1..=20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].starting_items.clone();
            monkeys[i].starting_items.clear();
            let operation = monkeys[i].operation;
            let test = monkeys[i].test;

            let count = inspect_count.entry(i).or_insert(0usize);
            *count += items.len();
    
            for item in items {
                let worry_level = calculate_worry_level(&operation, item);
                let monkey_index = if worry_level % test.divisible_by == 0 {
                    test.throw_if_true
                } else {
                    test.throw_if_false
                };
                monkeys[monkey_index].starting_items.push(worry_level);
            }
        }

        println!("After round {round}, the monkeys are holding items with these worry levels:");
        for (index, monkey) in monkeys.iter().enumerate() {
            print!("Monkey {index}:");
            for (index, item) in monkey.starting_items.iter().enumerate() {
                if index != 0 {
                    print!(" {item}");
                } else {
                    print!(", {item}");
                }
            }
            println!();
        }
        println!();
    }

    for (index, count) in inspect_count.iter() {
        println!("Monkey {index} inspected items {count} times");
    }

    let mut counts = inspect_count
        .iter()
        .map(|(_, &count)| count)
        .collect::<Vec<_>>();
    counts.sort();
    println!("{counts:?}");
    counts.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 10605);
    }

    #[test]
    fn part_two_works() {}
}
