use std::cmp::Ordering;

use crate::bitwise_utils::sign_extend;
use super::{Lc3Vm, registers::ConditionFlag};

// https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf
impl Lc3Vm {
    /// Performs the `ADD` operation
    fn add_op(&mut self, instr: u16) {
        // Use bitwise AND to retrieve only the bit that we are interested in
        let dest_reg = (instr >> 9) & 0b111;
        let first_reg = (instr >> 6) & 0b111;
        let first_val: i16 = self.get_reg_val_by_id(first_reg)
            .try_into()
            .unwrap();
    
        // Check bit[5], if 0 then register mode else immediate mode
        let register_mode = ((instr >> 5) & 0b1) == 0;
        let second_val: i16 = if register_mode {
            // Get the 2nd register for register mode
            let second_reg = instr & 0b111;
            self.get_reg_val_by_id(second_reg).try_into().unwrap()
        } else {
            let imm_val = instr & 0b11111;
            // Imm val is 5 bits, according to the spec
            sign_extend(imm_val, 5)
            // imm_val
        };
    
        let added = first_val + second_val;
        self.set_reg_val_by_id(dest_reg, added.try_into().unwrap());
        let cond_flag = match added.cmp(&0) {
            Ordering::Equal => ConditionFlag::Zro,
            Ordering::Greater => ConditionFlag::Pos,
            Ordering::Less => ConditionFlag::Neg,
        };
        self.registers.set_cond_reg(cond_flag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_add_op_register_mode() {
        let mut vm = Lc3Vm::new();
        // ADD R2, R3, R4
        let instr: u16 = 0b0001_001_010_000_011;

        // Setup data in registers
        let first = 5;
        let second = 3;
        vm.set_reg_val_by_id(2, first);
        vm.set_reg_val_by_id(3, second);
        vm.add_op(instr);
        let added_val = vm.get_reg_val_by_id(1);
        assert_eq!(added_val, first + second);
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_add_op_imm_mode() {
        let mut vm = Lc3Vm::new();
        // ADD R0, R1, 3
        let val = 3;
        let instr: u16 = 0b0001_000_001_1_00011;

        // Test with adding to 0
        vm.add_op(instr);
        let added_val = vm.get_reg_val_by_id(0);
        assert_eq!(added_val, val);

        // ADD R1, R0, -1
        let instr: u16 = 0b0001_001_000_1_11111;
        vm.add_op(instr);
        let added_val = vm.get_reg_val_by_id(1);
        assert_eq!(added_val, 3 - 1);
    }
}
