use crate::vm::{Lc3Vm, registers::RegisterName};
use ascii::AsciiChar;
use std::{io::Write};
use std::str::from_utf8;

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn test_puts() {
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
    vm.puts(&mut output);
    let printed_string = from_utf8(&output).unwrap();
    assert_eq!(test_string, printed_string);
}
