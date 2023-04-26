mod registers;
mod memory;
mod ops;

use std::cmp::Ordering;

use memory::Memory;
use registers::Registers;

use self::registers::{RegisterName, ConditionFlag};

const DEFAULT_PC_START: u16 = 0x3000;

pub struct Lc3Vm {
    registers: Registers,
    memory: Memory,
}

impl Lc3Vm {
    pub fn new() -> Self {
        let registers = Registers::new();
        let memory = Memory::new();
        Self { registers, memory }
    }

    pub fn run(&mut self) {
        self.registers.set_program_counter(DEFAULT_PC_START);

        loop {
            let instr = self.memory.read(self.registers.program_counter());
            self.registers.increment_program_counter();
            // First 4 bits of an instruction are the opcodes
            let opcode = instr >> 12;

        }
    }

    /// TODO: Implement error handling
    pub fn get_reg_val_by_id(&self, reg_id: u16) -> u16 {
        // Will panic if id is invalid!
        let register = RegisterName::from(reg_id);
        self.registers.get_reg_value(register)
    }

    /// TODO: Implement error handling
    pub fn set_reg_val_by_id(&mut self, reg_id: u16, value: u16) {
        // Will panic if id is invalid!
        let register = RegisterName::from(reg_id);
        self.registers.set_reg_value(register, value);
    }
}
