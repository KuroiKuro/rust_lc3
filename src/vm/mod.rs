mod registers;
mod memory;

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

// https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf

/// Sign extension of numbers, when the number is smaller than 16 bits
/// TODO: Research https://en.wikipedia.org/wiki/Two%27s_complement
fn sign_extend(x: u16, bit_count: u16) -> u16 {
    let mut x_arg = x;

    // Check if the most significant bit is a 1. If it is then fill in 1s for the
    // left padding
    if ((x_arg >> (bit_count - 1)) & 1) == 1 {
        x_arg |= 0xFFFF << bit_count;
    }
    x_arg
}

fn add_op(instr: u16, vm: &mut Lc3Vm) {
    // Use bitwise AND to retrieve only the bit that we are interested in
    let dest_reg = (instr >> 9) & 0b111;
    let first_reg = (instr >> 6) & 0b111;
    let first_val = vm.get_reg_val_by_id(first_reg);

    // Check bit[5], if 0 then register mode else immediate mode
    let register_mode = ((instr >> 5) & 0b1) == 0;
    let second_val = if register_mode {
        // Get the 2nd register for register mode
        let second_reg = instr & 0b111;
        vm.get_reg_val_by_id(second_reg)
    } else {
        let imm_val = instr & 0b11111;
        // Imm val is 5 bits, according to the spec
        sign_extend(imm_val, 5)
    };

    let added = first_val + second_val;
    vm.set_reg_val_by_id(dest_reg, added);
    let cond_flag = match added.cmp(&0) {
        Ordering::Equal => ConditionFlag::Zro,
        Ordering::Greater => ConditionFlag::Pos,
        Ordering::Less => ConditionFlag::Neg,
    };
    vm.registers.set_cond_reg(cond_flag);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_add_op() {
        let mut vm = Lc3Vm::new();
        // ADD R2, R3, R4
        let instr: u16 = 0b0001_001_010_000_011;

        // Setup data in registers
        let first = 5;
        let second = 3;
        vm.set_reg_val_by_id(2, first);
        vm.set_reg_val_by_id(3, second);
        add_op(instr, &mut vm);
        let added_val = vm.get_reg_val_by_id(1);
        assert_eq!(added_val, first + second);
    }
}
