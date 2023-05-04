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
    let flag = vm.get_cond_flag();
    assert_eq!(flag, ConditionFlag::Pos);

    // ADD R1, R0, -5
    let instr: u16 = 0b0001_001_000_1_11011;
    vm.add_op(instr);
    let added_val = vm.get_reg_val_by_id(1);
    assert_eq!(added_val, 0b1111_1111_1111_1110);
    let flag = vm.get_cond_flag();
    assert_eq!(flag, ConditionFlag::Neg);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_ldi_op() {
    let data: u16 = 1234;
    let final_address: u16 = 0x3085;
    let pointer_address: u16 = 0x3050;
    // let offset = desired_address - Lc3Vm::DEFAULT_PC_START;
    // LDI R2, ${offset}
    let instr: u16 = 0b1010_010_001010000;

    let mut vm = Lc3Vm::new();
    vm.memory.write(pointer_address, final_address);
    vm.memory.write(final_address, data);
    vm.ldi_op(instr);
    let reg_val = vm.get_reg_val_by_id(2);
    assert_eq!(reg_val, data);
    // Test flag
    let flag = vm.get_cond_flag();
    assert_eq!(flag, ConditionFlag::Pos);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_and_op_register_mode() {
    // AND R2, R4, R3
    let instr: u16 = 0b0101_010_100_0_00_011;
    let val1: u16 = 3433;
    let val2: u16 = 128;

    let mut vm = Lc3Vm::new();
    vm.set_reg_val_by_id(4, val1);
    vm.set_reg_val_by_id(3, val2);

    vm.and_op(instr);
    let result = vm.get_reg_val_by_id(2);
    assert_eq!(result, val1 & val2);
    let flag = ConditionFlag::from(vm.registers.cond_reg());
    let correct_flag: ConditionFlag = ConditionFlag::parse_u16(result);
    assert_eq!(flag, correct_flag);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_n() {
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_100_001010000;

    let mut vm = Lc3Vm::new();
    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_z() {
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_010_001010000;

    let mut vm = Lc3Vm::new();
    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_p() {
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_001_001010000;

    let mut vm = Lc3Vm::new();
    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_nz() {
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_110_001010000;

    let mut vm = Lc3Vm::new();

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
    vm.registers.set_program_counter(Lc3Vm::DEFAULT_PC_START);

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_zp() {
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_011_001010000;

    let mut vm = Lc3Vm::new();

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
    vm.registers.set_program_counter(Lc3Vm::DEFAULT_PC_START);

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_np() {
    // Note: Not sure if this combo will ever happen irl but it is implemented anyway
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_101_001010000;

    let mut vm = Lc3Vm::new();

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
    vm.registers.set_program_counter(Lc3Vm::DEFAULT_PC_START);

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_nzp() {
    // Note: Not sure if this combo will ever happen irl but it is implemented anyway
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_111_001010000;

    let mut vm = Lc3Vm::new();

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
    vm.registers.set_program_counter(Lc3Vm::DEFAULT_PC_START);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
    vm.registers.set_program_counter(Lc3Vm::DEFAULT_PC_START);

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_br_op_no_set() {
    // Note: Not sure if this combo will ever happen irl but it is implemented anyway
    let desired_address: u16 = 0x3050;
    let instr: u16 = 0b0000_000_001010000;

    let mut vm = Lc3Vm::new();

    vm.registers.set_cond_reg(ConditionFlag::Zro);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Pos);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);

    vm.registers.set_cond_reg(ConditionFlag::Neg);
    vm.br_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_ne!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_jmp_ret_op() {
    let mut vm = Lc3Vm::new();
    // RET is a special form of JMP, which is the equivalent of:
    // JMP R7
    let instr: u16 = 0b1100_000_111_000000;
    let value = 0x3085;
    vm.set_reg_val_by_id(7, value);
    vm.jmp_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, value);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_jsr_op() {
    let mut vm = Lc3Vm::new();
    let desired_address = 0x3085;
    let instr: u16 = 0b0100_1_00010000101;

    vm.jsr_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_jsrr_op() {
    let mut vm = Lc3Vm::new();
    let desired_address = 0x3085;
    // JSRR R3
    let instr: u16 = 0b0100_0_011_00000000;

    vm.set_reg_val_by_id(3, desired_address);
    vm.jsr_op(instr);
    let current_pc = vm.registers.program_counter();
    assert_eq!(current_pc, desired_address);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_ld_op() {
    let mut vm = Lc3Vm::new();
    let desired_address = 0x3085;
    // LD R4, VALUE
    let instr: u16 = 0b0010_100_010000101;

    let stored_value = 0xF1FA;
    vm.memory.write(desired_address, stored_value);
    vm.ld_op(instr);
    let value = vm.get_reg_val_by_id(4);
    assert_eq!(value, stored_value);
    let flag = ConditionFlag::from(vm.registers.cond_reg());
    let correct_flag: ConditionFlag = ConditionFlag::parse_u16(value);
    assert_eq!(flag, correct_flag);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_ldr_op() {
    let mut vm = Lc3Vm::new();
    let desired_address = 0x4100;
    let br_val: u16 = 0x40FC;
    let stored_value = 9913;
    vm.memory.write(desired_address, stored_value);

    // LDR R4, R2, offset
    let instr: u16 = 0b0110_100_010_000100;
    vm.set_reg_val_by_id(2, br_val);
    vm.ldr_op(instr);
    let value = vm.get_reg_val_by_id(4);
    assert_eq!(value, stored_value);
    let flag = ConditionFlag::from(vm.registers.cond_reg());
    let correct_flag: ConditionFlag = ConditionFlag::parse_u16(value);
    assert_eq!(flag, correct_flag);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_lea_op() {
    let data: u16 = 1234;
    let address: u16 = 0x3050;
    // let offset = desired_address - Lc3Vm::DEFAULT_PC_START;
    // LEA R2, ${offset}
    let instr: u16 = 0b1010_010_001010000;

    let mut vm = Lc3Vm::new();
    vm.memory.write(address, data);
    vm.lea_op(instr);
    let reg_val = vm.get_reg_val_by_id(2);
    assert_eq!(reg_val, data);
    // Test flag
    let flag = vm.get_cond_flag();
    assert_eq!(flag, ConditionFlag::Pos);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_not_op() {
    let mut vm = Lc3Vm::new();
    // NOT R4, R2
    let instr: u16 = 0b1001_100_010_1_11111;
    let num = 1413;
    vm.set_reg_val_by_id(2, num);
    vm.not_op(instr);
    let value = vm.get_reg_val_by_id(4);
    assert_eq!(value, !num);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_st_op() {
    let mut vm = Lc3Vm::new();
    let data = 7741;
    vm.set_reg_val_by_id(4, data);
    let address: u16 = 0x3050;
    // ST R4,
    let instr: u16 = 0b0011_100_001010000;
    vm.st_op(instr);
    let value = vm.memory.read(address);
    assert_eq!(data, value);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_sti_op() {
    let mut vm = Lc3Vm::new();
    let data = 19111;
    let final_address: u16 = 0x3085;
    let pointer_address: u16 = 0x3050;

    // STI R4,
    let instr: u16 = 0b1011_100_001010000;
    vm.set_reg_val_by_id(4, data);
    vm.memory.write(pointer_address, final_address);
    vm.sti_op(instr);
    let value = vm.memory.read(final_address);
    assert_eq!(data, value);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_str_op() {
    let mut vm = Lc3Vm::new();

    let data: u16 = 47;
    vm.set_reg_val_by_id(4, data);

    // The final address 0x3085 should be computed from adding
    // base_reg_addr_data to the offset in the instruction
    let desired_address: u16 = 0x3085;
    let base_reg_addr_data: u16 = 0x3070;
    vm.set_reg_val_by_id(2, base_reg_addr_data);

    // STR R4, R2
    let instr: u16 = 0b0111_100_010_010101;
    vm.str_op(instr);
    let value = vm.memory.read(desired_address);
    assert_eq!(data, value);
}
