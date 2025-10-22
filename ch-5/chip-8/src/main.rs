struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_point: usize,
}

impl CPU {
    fn run(&mut self) {
        let op_byte1 = self.memory[self.position_in_memory] as u16;
        let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
        let op_code: u16 = op_byte1 << 8 | op_byte2;

        let x = ((op_code & 0x0F00) >> 8) as u8;
        let y = ((op_code & 0x00F0) >> 4) as u8;
        let kk = (op_code & 0x00FF) as u8;
        let op_minor = (op_code & 0x000F) as u8;
        let addr = op_code & 0x0FFF;

        self.position_in_memory += 2;

        match op_code {
            0x0000 => {
                return;
            }
            0x00E0 => {}
            0x00EE => {
                self.ret();
            }
            0x1000..=0x1FFF => self.jump(addr),
            0x2000..=0x2FFF => self.call(addr),
            0x3000..=0x3FFF => self.se(x, kk),
            0x4000..=0x4FFF => self.sne(x, kk),
            0x5000..=0x5FFF => self.se(x, y),
            0x6000..=0x6FFF => self.ld(x, kk),
            0x7000..=0x7FFF => self.add(x, kk),
            0x8000..=0x8FFF => match op_minor {
                0 => self.ld(x, self.registers[y as usize]),
                1 => self.or_xy(x, y),
                2 => self.and_xy(x, y),
                3 => self.xor_xy(x, y),
                4 => self.add(x, y),
                _ => todo!("op_code: {:04x}", op_code),
            },
            _ => todo!("op_code: {:04x}", op_code),
        }
    }

    fn ret(&mut self) {
        todo!()
    }

    fn jump(&mut self, addr: u16) {
        todo!()
    }

    fn call(&mut self, addr: u16) {
        todo!()
    }

    fn se(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn sne(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn ld(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn add(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        todo!()
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
