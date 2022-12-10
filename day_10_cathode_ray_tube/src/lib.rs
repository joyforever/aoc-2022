mod instruction;
mod cpu;
mod crt;

use crate::instruction::Instruction;
use crate::cpu::CPU;
use crate::crt::CRT;

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
        .map(|&index| cpu.get_signals()[index])
        .sum()
}

pub fn part_two(input: &str) {
    let insructions = input
        .lines()
        .map(Instruction::from)
        .flat_map(|i| match i {
            Instruction::Addx(value) => vec![Instruction::Noop, Instruction::Addx(value)],
            _ => vec![Instruction::Noop],
        })
        .collect::<Vec<_>>();

    let mut crt = CRT::default();
    for ins in &insructions {
        crt.draw(ins);
    }

    crt.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../data/example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(EXAMPLE), 13140);
    }

    #[test]
    fn part_two_works() {
        part_two(EXAMPLE);
    }
}
