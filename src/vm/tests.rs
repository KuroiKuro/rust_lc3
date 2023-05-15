use std::io::Write;

use super::*;
use tempfile::NamedTempFile;

#[test]
fn test_load_program() {
    let mut vm = Lc3Vm::new();
    let start_address = 0x3001;
    // Create a tempfile to contain the testing program logic
    let mut temp_file = NamedTempFile::new().unwrap();
    let program_data: Vec<u16> = vec![start_address, 0x39ab, 0x341f, 0x3333, 0x31cb];

    let write_data = program_data
        .iter()
        .flat_map(|prog_data_piece| prog_data_piece.to_le_bytes())
        .collect::<Vec<u8>>();
    temp_file.write_all(&write_data).unwrap();

    let temp_file_path = temp_file.path();
    vm.load_program(temp_file_path).unwrap();

    // Check that 0x3000 is empty, since we configured the start to be another
    // address
    let check_addr = vm.memory.read(0x3000);
    assert_eq!(check_addr, 0);

    let mut current_address = start_address;
    // Remove start address from data to check
    for (i, data) in program_data.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let check_data = vm.memory.read(current_address);
        assert_eq!(check_data, *data);
        current_address += 1;
    }
    // Check the address after is empty, as nothing should be written after the program
    // is written
    let after_data = vm.memory.read(current_address);
    assert_eq!(after_data, 0);
}

#[test]
fn test_read_u16() {
    let le_bytes: [u8; 2] = [0x2b, 0x2e];
    let be_u16 = Lc3Vm::read_u16(&le_bytes);
    assert_eq!(be_u16, 0x2e2b);
}

#[test]
fn test_validate_file_len() {
    let invalid_file_len: u64 = 65536;
    let valid_file_len: u64 = 2000;
    let origin = 0x3000;

    let invalid_check = Lc3Vm::validate_file_len(invalid_file_len, origin);
    let valid_check = Lc3Vm::validate_file_len(valid_file_len, origin);
    assert!(valid_check.is_ok());
    assert!(invalid_check.is_err());
    let err = invalid_check.unwrap_err();
    assert_eq!(err, invalid_file_len);
}
