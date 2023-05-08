//! This module contains the implementation code for the "opcodes" instructions
//! provided in the LC3 ISA specifications

#[cfg(test)]
mod tests;

use std::num::Wrapping;

use super::{registers::ConditionFlag, Lc3Vm};
use crate::bitwise_utils::sign_extend;

enum OpCode {
    Add,
    And,
    Br,
    Jmp,
    Jsr,
    Ld,
    Ldi,
    Ldr,
    Lea,
    Not,
    Rti,
    St,
    Sti,
    Str,
    Trap,
}

impl TryFrom<u16> for OpCode {
    // Use empty type, the only reason it will fail is if the opcode is unrecognized
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let opcode = match value {
            0b0001 => Self::Add,
            0b0101 => Self::And,
            0b0000 => Self::Br,
            0b1100 => Self::Jmp,
            0b0100 => Self::Jsr,
            0b0010 => Self::Ld,
            0b1010 => Self::Ldi,
            0b0110 => Self::Ldr,
            0b1110 => Self::Lea,
            0b1001 => Self::Not,
            0b1000 => Self::Rti,
            0b0011 => Self::St,
            0b1011 => Self::Sti,
            0b0111 => Self::Str,
            0b1111 => Self::Trap,
            _ => return Err(()),
        };
        Ok(opcode)
    }
}

enum TrapVector {
    Getc = 0x20,
    Out = 0x21,
    Puts = 0x22,
    In = 0x23,
    Putsp = 0x24,
    Halt = 0x25,
}

impl TryFrom<u16> for TrapVector {
    type Error = ();
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let trap_vec = match value {
            0x20 => Self::Getc,
            0x21 => Self::Out,
            0x22 => Self::Puts,
            0x23 => Self::In,
            0x24 => Self::Putsp,
            0x25 => Self::Halt,
            _ => return Err(()),
        };
        Ok(trap_vec)
    }
}

// https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf
impl Lc3Vm {
    /// Determine the correct operation to run, and run it
    pub fn run_op(&mut self, instr: u16) {
        let opcode_raw = instr >> 12;
        let opcode = OpCode::try_from(opcode_raw).expect("Invalid opcode detected");
        match opcode {
            OpCode::Add => self.add_op(instr),
            OpCode::And => self.and_op(instr),
            OpCode::Br => self.br_op(instr),
            OpCode::Jmp => self.jmp_op(instr),
            OpCode::Jsr => self.jsr_op(instr),
            OpCode::Ld => self.ld_op(instr),
            OpCode::Ldi => self.ldi_op(instr),
            OpCode::Ldr => self.ldr_op(instr),
            OpCode::Lea => self.lea_op(instr),
            OpCode::Not => self.not_op(instr),
            OpCode::Rti => self.rti_op(instr),
            OpCode::St => self.st_op(instr),
            OpCode::Sti => self.sti_op(instr),
            OpCode::Str => self.str_op(instr),
            OpCode::Trap => self.trap_op(instr),
        };
    }

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

    /// Performs the `BR` operation, include the `znp` variants
    fn br_op(&mut self, instr: u16) {
        let flag_bits: u16 = instr >> 9;
        let test_neg = ((flag_bits >> 2) & 1) == 1;
        let test_zro = ((flag_bits >> 1) & 1) == 1;
        let test_pos = (flag_bits & 1) == 1;

        let offset = instr & 0b111111111;
        let current_pc = self.registers.program_counter();
        let br_address = offset + current_pc;

        let flag = self.get_cond_flag();
        let will_br = match (test_neg, test_zro, test_pos) {
            (true, true, true) => true,
            (true, false, false) => flag == ConditionFlag::Neg,
            (true, true, false) => flag == ConditionFlag::Neg || flag == ConditionFlag::Zro,
            (false, false, true) => flag == ConditionFlag::Pos,
            (false, true, true) => flag == ConditionFlag::Pos || flag == ConditionFlag::Zro,
            (false, true, false) => flag == ConditionFlag::Zro,
            (true, false, true) => flag == ConditionFlag::Neg || flag == ConditionFlag::Pos,
            (false, false, false) => false,
        };

        if will_br {
            self.registers.set_program_counter(br_address);
        }
    }

    /// Implements the `JMP` op
    fn jmp_op(&mut self, instr: u16) {
        let base_reg = (instr >> 6) & 0b111;
        let base_reg_val = self.get_reg_val_by_id(base_reg);
        self.registers.set_program_counter(base_reg_val);
    }

    /// Implements the `JSR` op
    fn jsr_op(&mut self, instr: u16) {
        // Save PC into R7, PC should have been incremented already before
        // calling this op
        let current_pc = self.registers.program_counter();
        self.set_reg_val_by_id(7, current_pc);
        // Implement both the JSR and JSRR operation
        let jsr_mode = ((instr >> 11) & 1) == 1;
        let new_pc_addr = if jsr_mode {
            let offset = sign_extend(instr & 0x7ff, 11);
            current_pc + offset
        } else {
            let base_reg = (instr >> 8) & 0b111;
            self.get_reg_val_by_id(base_reg)
        };
        self.registers.set_program_counter(new_pc_addr);
    }

    /// Performs the `LD` operation
    fn ld_op(&mut self, instr: u16) {
        let offset = sign_extend(instr & 0x1ff, 9);
        let dest_reg = (instr >> 9) & 0x7;
        let current_pc = self.registers.program_counter();
        let load_addr = current_pc + offset;
        let value = self.memory.read(load_addr);
        self.set_reg_val_by_id(dest_reg, value);
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `LDI` operation
    fn ldi_op(&mut self, instr: u16) {
        let dest_reg = (instr >> 9) & 0b111;
        let pc_offset = Wrapping(sign_extend(instr & 0x1ff, 9));
        let current_pc = Wrapping(self.registers.program_counter());
        let pointer_address = pc_offset + current_pc;
        let final_address = self.memory.read(pointer_address.0);
        let value = self.memory.read(final_address);
        self.set_reg_val_by_id(dest_reg, value);
        // Check if value is positive or negative to set the flags
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `LDR` operation
    fn ldr_op(&mut self, instr: u16) {
        let offset = instr & 0x3F;
        let offset = Wrapping(sign_extend(offset, 6));
        let base_reg = (instr >> 6) & 0x7;
        let dest_reg = (instr >> 9) & 0x7;

        let br_val = Wrapping(self.get_reg_val_by_id(base_reg));
        let address = br_val + offset;
        let value = self.memory.read(address.0);
        self.set_reg_val_by_id(dest_reg, value);
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `LEA` operation
    fn lea_op(&mut self, instr: u16) {
        let dest_reg = (instr >> 9) & 0b111;
        let pc_offset = Wrapping(sign_extend(instr & 0x1ff, 9));
        let current_pc = Wrapping(self.registers.program_counter());
        let address = pc_offset + current_pc;

        let value = self.memory.read(address.0);
        self.set_reg_val_by_id(dest_reg, value);
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `NOT` operation
    fn not_op(&mut self, instr: u16) {
        let sr = (instr >> 6) & 0x7;
        let dr = (instr >> 9) & 0x7;
        let sr_val = self.get_reg_val_by_id(sr);
        let value = !sr_val;
        self.set_reg_val_by_id(dr, value);
        let flag = ConditionFlag::parse_u16(value);
        self.registers.set_cond_reg(flag);
    }

    /// Performs the `RTI` operation
    fn rti_op(&mut self, _instr: u16) {
        todo!()
    }

    /// Performs the `ST` operation
    fn st_op(&mut self, instr: u16) {
        let pc_offset = Wrapping(sign_extend(instr & 0x1ff, 9));
        let sr = (instr >> 9) & 0x7;
        let sr_val = self.get_reg_val_by_id(sr);

        let current_pc = Wrapping(self.registers.program_counter());
        let address = current_pc + pc_offset;
        self.memory.write(address.0, sr_val);
    }

    /// Performs the `STI` operation
    fn sti_op(&mut self, instr: u16) {
        let pc_offset = Wrapping(sign_extend(instr & 0x1ff, 9));
        let sr = (instr >> 9) & 0x7;
        let sr_val = self.get_reg_val_by_id(sr);

        let current_pc = Wrapping(self.registers.program_counter());
        let pointer_address = current_pc + pc_offset;
        let final_address = self.memory.read(pointer_address.0);
        self.memory.write(final_address, sr_val);
    }

    /// Performs the `STR` operation
    fn str_op(&mut self, instr: u16) {
        let offset = Wrapping(sign_extend(instr & 0x3f, 6));
        let base_reg = (instr >> 6) & 0x7;
        let sr = (instr >> 9) & 0x7;

        let sr_val = self.get_reg_val_by_id(sr);
        let base_reg_val = Wrapping(self.get_reg_val_by_id(base_reg));
        let address = base_reg_val + offset;
        self.memory.write(address.0, sr_val);
    }

    /// Performs the `TRAP` operation
    fn trap_op(&mut self, instr: u16) {
        let current_pc = self.registers.program_counter();
        self.set_reg_val_by_id(7, current_pc);

        let trap_vec_raw = instr & 0xff;
        // Use the enum to parse the raw trap vector code, to make sure it is a valid
        // trap vector code
        let trap_vec = TrapVector::try_from(trap_vec_raw).expect("Invalid trap vector");
        self.registers.set_program_counter(trap_vec as u16);
        // Run code
        todo!();

        self.registers.set_program_counter(current_pc);
    }
}
