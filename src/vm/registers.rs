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

const GENERAL_REGISTER_COUNT: usize = 8;
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

#[derive(Clone, Copy)]
pub enum ConditionFlag {
    Pos = 0b001,
    Zro = 0b010,
    Neg = 0b100,
}

impl From<u16> for ConditionFlag {
    fn from(value: u16) -> Self {
        match value {
            0b001 => ConditionFlag::Pos,
            0b010 => ConditionFlag::Zro,
            0b100 => ConditionFlag::Neg,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<ConditionFlag> for u16 {
    fn from(value: ConditionFlag) -> Self {
        value as u16
    }
}

pub struct Registers {
    general_regs: [Register; GENERAL_REGISTER_COUNT],
    program_counter_reg: Register,
    /// Condition register stores condition flags about most recently executed calcs.
    /// This allows comparisons, etc
    condition_reg: Register
}

impl Registers {
    pub fn new() -> Self {
        let general_regs: [Register; GENERAL_REGISTER_COUNT] = [Register(0); GENERAL_REGISTER_COUNT];
        let program_counter_reg = Register(0);
        let condition_reg = Register(0);
        Self {general_regs, program_counter_reg, condition_reg}
    }

    pub fn get_reg_value(&self, register: RegisterName) -> u16 {
        let index = register as usize;
        self.general_regs[index].value()
    }

    pub fn set_reg_value(&mut self, register: RegisterName, value: u16) {
        let index = register as usize;
        self.general_regs[index].set(value);
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter_reg.value()
    }

    pub fn increment_program_counter(&mut self) {
        let current_val = self.program_counter_reg.value() + 1;
        self.program_counter_reg.set(current_val);
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter_reg.set(value);
    }

    pub fn cond_reg(&self) -> u16 {
        self.condition_reg.value()
    }

    pub fn set_cond_reg(&mut self, flag: ConditionFlag) {
        self.condition_reg.set(flag.into());
    } 
}
