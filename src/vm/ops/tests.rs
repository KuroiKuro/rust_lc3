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