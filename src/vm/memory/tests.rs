use ascii::AsciiChar;

use crate::vm::Lc3Vm;

#[test]
fn test_read_kbsr() {
    let mut vm = Lc3Vm::new();
    let mut input = "y".as_bytes();
    let value = vm.memory.read_kbsr(&mut input);
    assert_eq!(value, 0x8000);

    // Test that kbdr has been updated
    let kbdr_val = vm.memory.mmap_registers.kbdr.read();
    let expected_val = AsciiChar::new('y') as u16;
    assert_eq!(kbdr_val, expected_val);

    let mut input = "".as_bytes();
    let value = vm.memory.read_kbsr(&mut input);
    assert_eq!(value, 0);

    // Kbdr value should still be there
    let kbdr_val = vm.memory.mmap_registers.kbdr.read();
    assert_eq!(kbdr_val, expected_val);
}

#[test]
fn test_read_kbdr() {
    let mut vm = Lc3Vm::new();
    let ascii_char = AsciiChar::new('x') as u16;
    vm.memory.mmap_registers.kbdr.write(ascii_char);
    let read_value = vm.memory.read_kbdr();
    assert_eq!(read_value, ascii_char);
}

#[test]
fn test_read_dsr() {
    let vm = Lc3Vm::new();
    assert_eq!(vm.memory.read_dsr(), 0x8000);
}
