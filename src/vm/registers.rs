#[derive(Clone, Copy)]
pub struct Register(u16);

impl Register {
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, value: u16) {
        self.0 = value;
    }
}

const REGISTER_COUNT: usize = 10;
pub enum RegisterName {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8, // Program counter
    Cond = 9,
}

pub struct Registers {
    regs: [Register; REGISTER_COUNT],
}

impl Registers {
    pub fn new() -> Self {
        let regs: [Register; REGISTER_COUNT] = [Register(0); REGISTER_COUNT];
        Self {regs}
    }

    pub fn get_register_value(&self, register: RegisterName) -> u16 {
        let index = register as usize;
        self.regs[index].value()
    }

    pub fn set_register_value(&mut self, register: RegisterName, value: u16) {
        let index = register as usize;
        self.regs[index].set(value);
    }
}
