use crate::vm::{registers::RegisterName, Lc3Vm};
use ascii::AsciiChar;
use std::str::from_utf8;

#[test]
fn test_getc_troutine() {
    let mut vm = Lc3Vm::new();
    let mut input = "g".as_bytes();
    vm.getc_troutine(&mut input);
    let read_char = vm.registers.get_reg_value(RegisterName::R0);
    assert_eq!(read_char, 'g' as u16);

    let mut input = "rs".as_bytes();
    vm.getc_troutine(&mut input);
    let read_char = vm.registers.get_reg_value(RegisterName::R0);
    assert_eq!(read_char, 'r' as u16);
}

#[test]
fn test_out_troutine() {
    let mut vm = Lc3Vm::new();
    let test_char = 'w' as u16;
    vm.registers.set_reg_value(RegisterName::R0, test_char);
    let mut output: Vec<u8> = Vec::new();
    vm.out_troutine(&mut output);
    assert_eq!(output.len(), 1);
    let read_char = output[0] as u16;
    assert_eq!(test_char, read_char);
}

#[test]
fn test_puts_troutine() {
    let mut vm = Lc3Vm::new();
    let start_address = 0x303b;
    vm.registers.set_reg_value(RegisterName::R0, start_address);

    let test_string = "Hello world!";
    let str_chars = test_string.chars();
    let ascii_chars = str_chars.map(AsciiChar::new);

    // Write test string
    let mut current_address = start_address;
    for ascii_char in ascii_chars {
        vm.memory.write(current_address, ascii_char as u16);
        current_address += 1;
    }
    vm.memory.write(current_address, 0);

    let mut output: Vec<u8> = Vec::new();
    vm.puts_troutine(&mut output);
    let printed_string = from_utf8(&output).unwrap();
    assert_eq!(test_string, printed_string);
}
