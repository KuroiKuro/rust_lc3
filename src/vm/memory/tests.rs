use crate::vm::Lc3Vm;

use super::*;

#[test]
fn test_read_kbsr() {
    let vm = Lc3Vm::new();
    let mut input = "y".as_bytes();
    let value = vm.memory.read_kbsr(&mut input);
    assert_eq!(value, 0x8000);

    let mut input = "".as_bytes();
    let value = vm.memory.read_kbsr(&mut input);
    assert_eq!(value, 0);
}
