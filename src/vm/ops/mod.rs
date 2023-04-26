#[cfg(test)]
mod tests;

use std::num::Wrapping;

use super::{registers::ConditionFlag, Lc3Vm};
use crate::bitwise_utils::sign_extend;

// https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf
impl Lc3Vm {
    /// Performs the `ADD` operation
    fn add_op(&mut self, instr: u16) {
        // Use bitwise AND to retrieve only the bit that we are interested in
        let dest_reg = (instr >> 9) & 0b111;
        let sr1 = (instr >> 6) & 0b111;
        let first_val = self.get_reg_val_by_id(sr1);

        // Check bit[5], if 0 then register mode else immediate mode
        let register_mode = ((instr >> 5) & 0b1) == 0;
        let second_val = if register_mode {
            // Get the 2nd register for register mode
            let sr2 = instr & 0b111;
            self.get_reg_val_by_id(sr2)
        } else {
            let imm_val = instr & 0b11111;
            // Imm val is 5 bits, according to the spec
            sign_extend(imm_val, 5)
            // imm_val
        };

        // Instead of using an i16, we use a u16 even though the numbers can
        // be negative. This is to simulate working on the raw binary data
        // instead of using Rust's datatype-related functionality.
        // 
        // It's possible that one or more of the u16s are negative, in which case
        // the most significant bit will be `1`. Adding such a u16 to another one
        // can result in an integer overflow, causing Rust to panic. However,
        // in this case since we are not using the i16 datatype to do this
        // arithmetic, the overflow is intended behaviour, and due to how the
        // number is encoded with 2's complement, the resulting u16 will be
        // the correct result
        let wrapped_first = Wrapping(first_val);
        let wrapped_second = Wrapping(second_val);
        let added = wrapped_first + wrapped_second;
        self.set_reg_val_by_id(dest_reg, added.0);
        let flag = ConditionFlag::parse_u16(added.0);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `AND` operation
    fn and_op(&mut self, instr: u16) {
        let dest_reg = (instr >> 9) & 0b111;
        let sr1 = (instr >> 6) & 0b111;
        let val1 = self.get_reg_val_by_id(sr1);

        let register_mode = (instr >> 5) & 1 == 0;
        let val2 = if register_mode {
            let sr2 = instr & 0b111;
            self.get_reg_val_by_id(sr2)
        } else {
            let imm_val = instr & 0b11111;
            sign_extend(imm_val, 5)
        };

        let result = val1 & val2;
        self.set_reg_val_by_id(dest_reg, result);
        let flag = ConditionFlag::parse_u16(result);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `LDI` operation
    fn ldi_op(&mut self, instr: u16) {
        let dest_reg = (instr >> 9) & 0b111;
        // After doing &, it is already automatically sign extended by Rust's u16 type
        let pc_offset = instr & 0b111111111;
        let current_pc = self.registers.program_counter();
        let address = pc_offset + current_pc;
        let value = self.memory.read(address);
        self.set_reg_val_by_id(dest_reg, value);
        // Check if value is positive or negative to set the flags
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }
}
