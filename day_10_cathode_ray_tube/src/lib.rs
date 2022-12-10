#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s.starts_with("noop") {
            Instruction::Noop
        } else {
            let (_, value) = s.split_once(' ').unwrap();
            let value = value.parse::<i32>().unwrap();
            Instruction::Addx(value)
        }
    }
}

struct CPU {
    register_x: i32,
    cycles: i32,
    signals: Vec<i32>,
}

impl Default for CPU {
    fn default() -> Self {
        Self { register_x: 1, cycles: 0, signals: vec![0], }
    }
}

impl CPU {
    fn execute(&mut self, instruction: &Instruction) {
        self.cycles += 1;
        self.signals.push(self.cycles * self.register_x);
        //self.print_signal_strengths();

        if let &Instruction::Addx(value) = instruction {
            self.register_x += value;
        }
    }

    #[allow(dead_code)]
    fn print_signal_strengths(&self) {
        if self.cycles >= 20 && (self.cycles - 20) % 40 == 0 {
            println!("cycles:{}, x:{}, signal:{}", self.cycles, self.register_x, self.cycles * self.register_x);
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let insructions = input
        .lines()
        .map(Instruction::from)
        .flat_map(|i| match i {
            Instruction::Addx(value) => vec![Instruction::Noop, Instruction::Addx(value)],
            _ => vec![Instruction::Noop],
        })
        .collect::<Vec<_>>();

    let mut cpu = CPU::default();
    for instruction in &insructions {
        cpu.execute(instruction);
    }

    let cycles = vec![20, 60, 100, 140, 180, 220];
    cycles
        .iter()
        .map(|&index| cpu.signals[index])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13140);
    }
}
