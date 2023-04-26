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
    let desired_address: u16 = 0x3050;
    // let offset = desired_address - Lc3Vm::DEFAULT_PC_START;
    // LDI R3, ${offset}
    let instr: u16 = 0b1010_010_001010000;

    let mut vm = Lc3Vm::new();
    vm.memory.write(desired_address, data);
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
    // AND R3, R5, R4
    let instr: u16 = 0b0101_010_100_0_00_011;
    let val1: u16 = 3433;
    let val2: u16 = 128;

    let mut vm = Lc3Vm::new();
    vm.set_reg_val_by_id(4, val1);
    vm.set_reg_val_by_id(3, val2);

    vm.and_op(instr);
    let result = vm.get_reg_val_by_id(2);
    assert_eq!(result, val1 & val2);
    let flag = ConditionFlag::parse_u16(result);
    assert_eq!(flag, ConditionFlag::Zro);
}
