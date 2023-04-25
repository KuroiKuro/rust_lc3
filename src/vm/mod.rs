mod registers;
mod memory;

use memory::Memory;
use registers::Registers;


pub struct Lc3Vm {
    registers: Registers,
    memory: Memory,
}

impl Lc3Vm {
    pub fn new() -> Self {
        let registers = Registers::new();
        let memory = Memory::new();
        Self { registers, memory }
    }
}
