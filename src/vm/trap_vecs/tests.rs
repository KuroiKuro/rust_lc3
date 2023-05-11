use super::IN_TROUTINE_PROMPT;
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

#[test]
fn test_in_troutine() {
    let mut vm = Lc3Vm::new();

    let expected_char = 'F';
    let mut input = "F".as_bytes();
    let mut output: Vec<u8> = Vec::new();

    vm.in_troutine(&mut input, &mut output);
    let expected_output = format!("{}{}", IN_TROUTINE_PROMPT, expected_char);
    let printed_output = from_utf8(&output).unwrap();
    assert_eq!(printed_output, expected_output);

    let saved_char = vm.registers.get_reg_value(RegisterName::R0);
    assert_eq!(saved_char, expected_char as u16);
}

#[test]
fn test_putsp_troutine() {
    let mut vm = Lc3Vm::new();
    let start_address = 0x30BA;
    vm.registers.set_reg_value(RegisterName::R0, start_address);

    let test_str = "Hesitation is defeat";
    let mem_vec = test_str
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|char_arr| {
            if char_arr.len() == 2 {
                let first_char = char_arr[0];
                let second_char = char_arr[1];
                // Write the first char as the least significant bits
                (second_char as u16) << 8 | first_char as u16
            } else {
                char_arr[0] as u16
            }
        })
        .collect::<Vec<u16>>();

    let mut current_address = start_address;
    for data in mem_vec {
        vm.memory.write(current_address, data);
        current_address += 1;
    }
    vm.memory.write(current_address, 0);

    let mut output: Vec<u8> = Vec::new();
    vm.putsp_troutine(&mut output);
    let print_str = from_utf8(&output).unwrap();
    assert_eq!(test_str, print_str);
}

#[test]
fn test_halt_troutine() {
    let mut vm = Lc3Vm::new();
    vm.halt_troutine();
    let running = vm.running();
    assert!(!running);
}
