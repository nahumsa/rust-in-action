struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn run(&mut self) {
        loop {
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
    }

    // (00EE) RET return from current sub-routine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow")
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    // (1nnn) jumps to a given address
    fn jump(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    // (2nnn) CALL sub-routine at addr
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        if sp >= self.stack.len() {
            panic!("Stack overflow!")
        }

        self.stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    // (x3nnn),(x5nnn)  store if equal
    fn se(&mut self, x: u8, y: u8) {
        if x == y {
            self.position_in_memory += 2;
        }
    }

    // (x4nnn) store if not equal
    fn sne(&mut self, x: u8, y: u8) {
        if x != y {
            self.position_in_memory += 2;
        }
    }

    // (x6nnn) sets y value at register x
    fn ld(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = y;
    }

    // (x7nnn) adds value y on register x
    fn add(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    // (x8nn1) applies or operator on register x using y
    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    // (x8nn2) applies xor operator on register x using y
    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    // (x8nn3) applies and operator on register x using y
    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.memory[0x000] = 0x21;
    cpu.memory[0x001] = 0x00;
    cpu.memory[0x002] = 0x21;
    cpu.memory[0x003] = 0x00;

    cpu.memory[0x100] = 0x80;
    cpu.memory[0x101] = 0x14;
    cpu.memory[0x102] = 0x80;
    cpu.memory[0x103] = 0x14;
    cpu.memory[0x104] = 0x00;
    cpu.memory[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
