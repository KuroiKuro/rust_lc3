mod memory;
mod ops;
mod registers;

use std::cmp::Ordering;

use memory::Memory;
use registers::Registers;

use self::registers::{ConditionFlag, RegisterName};

pub struct Lc3Vm {
    registers: Registers,
    memory: Memory,
}

impl Lc3Vm {
    const DEFAULT_PC_START: u16 = 0x3000;

    pub fn new() -> Self {
        let registers = Registers::new();
        let memory = Memory::new();
        let mut vm = Self { registers, memory };
        vm.registers.set_program_counter(Self::DEFAULT_PC_START);
        vm
    }

    pub fn run(&mut self) {
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
