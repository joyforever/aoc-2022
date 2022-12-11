#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

pub fn part_one(input: &str) -> u32 {
    let monkeys = input.split("\n\n")
        .map(Monkey::from)
        .collect::<Vec<_>>();

    dbg!(monkeys);

    0
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
