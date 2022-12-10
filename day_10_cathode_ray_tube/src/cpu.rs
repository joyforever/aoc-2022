use crate::instruction::Instruction;

pub(crate) struct CPU {
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
    pub(crate) fn execute(&mut self, instruction: &Instruction) {
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

    pub(crate) fn get_signals(&self) -> &[i32] {
        &self.signals
    }
}
