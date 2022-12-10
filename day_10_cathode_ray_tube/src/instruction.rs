#[derive(Debug)]
pub(crate) enum Instruction {
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
