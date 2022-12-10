use crate::instruction::Instruction;

pub(crate) struct CRT {
    register_x: i32,
    pixels: Vec<Vec<char>>,
    sprite: usize,
}

impl Default for CRT {
    fn default() -> Self {
        let mut pixels = Vec::new();
        for _ in 0..6 {
            pixels.push(vec![' '; 40]);
        }

        Self {
            register_x: 1,
            pixels,
            sprite: 0,
        }
    }
}

impl CRT {
    pub(crate) fn draw(&mut self, instruction: &Instruction) {
        let row = self.sprite / 40;
        let col = self.sprite % 40;
        if self.is_visiable(col) {
            self.pixels[row][col] = '#';
        } else {
            self.pixels[row][col] = '.';
        }

        self.sprite += 1;
        if let Instruction::Addx(value) = instruction {
            self.register_x += value;
        }
    }

    pub(crate) fn print(&self) {
        for v in &self.pixels {
            for p in v {
                print!("{p}");
            }
            println!();
        }
    }

    fn is_visiable(&self, pos: usize) -> bool {
        let pos = pos as i32;
        (self.register_x - 1..=self.register_x + 1).contains(&pos)
    }
}
