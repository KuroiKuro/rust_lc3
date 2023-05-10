mod memory;
mod ops;
mod registers;
#[cfg(test)]
mod tests;
mod trap_vecs;

use std::{fs::File, io::Read, path::Path};

use memory::Memory;
use registers::Registers;

use self::registers::{ConditionFlag, RegisterName};

pub struct Lc3Vm {
    registers: Registers,
    memory: Memory,
    running: bool,
}

impl Lc3Vm {
    const DEFAULT_PC_START: u16 = 0x3000;

    pub fn new() -> Self {
        let registers = Registers::new();
        let memory = Memory::new();
        let running = false;
        let mut vm = Self {
            registers,
            memory,
            running,
        };
        vm.registers.set_program_counter(Self::DEFAULT_PC_START);
        vm
    }

    /// Load a compiled LC3 program for execution.
    ///
    /// A given LC3 program will have its first 16 bits set to the memory address
    /// where the start of the program instructions should be loaded to. Subsequent
    /// bytes are then the program instructions
    pub fn load_program(&mut self, file_path: &Path) {
        let mut program_file = File::open(file_path).unwrap();
        let mut file_contents: Vec<u8> = Vec::new();
        program_file.read_to_end(&mut file_contents).unwrap();

        // Read the origin first
        let mut chunked = file_contents.as_slice().chunks(2);

        let chunk = chunked.next().unwrap();
        let origin = Self::read_u16(chunk);
        let file_len = program_file.metadata().unwrap().len();
        if let Err(e) = Self::validate_file_len(file_len, origin) {
            panic!(
                "The length of the file ({}) will be too large to fit into VM memory!",
                e
            );
        }

        let mut current_address = origin;
        for chunk in chunked {
            let mem_data = Self::read_u16(chunk);
            self.memory.write(current_address, mem_data);
            current_address += 1;
        }
    }

    /// Reads two bytes from file data that has been converted into a `Chunks<u8>`,
    /// and then parse it into a `u16` and return it
    fn read_u16(program_data_slice: &[u8]) -> u16 {
        // Program file is in little endian, so convert it to big endian, which
        // is required in LC3
        let buf: [u8; 2] = [program_data_slice[0], program_data_slice[1]];
        u16::from_le_bytes(buf)
    }

    /// Validate that the program file is small enough to fit in the memory of
    /// the LC3 VM. The `file_len` argument should be the size of the files generated
    /// by the file's `std::fs::Metadata::len()` function
    fn validate_file_len(file_len: u64, origin: u16) -> Result<(), u64> {
        if file_len > (u16::MAX - origin) as u64 {
            Err(file_len)
        } else {
            Ok(())
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            let instr = self.memory.read(self.registers.program_counter());
            self.registers.increment_program_counter();
            // First 4 bits of an instruction are the opcodes
            self.run_op(instr);
        }
    }

    /// TODO: Implement error handling
    pub fn get_reg_val_by_id(&self, reg_id: u16) -> u16 {
        // Will panic if id is invalid!
        let register = RegisterName::from(reg_id);
        self.registers.get_reg_value(register)
    }

    /// TODO: Implement error handling
    pub fn set_reg_val_by_id(&mut self, reg_id: u16, value: u16) {
        // Will panic if id is invalid!
        let register = RegisterName::from(reg_id);
        self.registers.set_reg_value(register, value);
    }

    pub fn get_cond_flag(&self) -> ConditionFlag {
        let flag_reg_val = self.registers.cond_reg();
        ConditionFlag::from(flag_reg_val)
    }
}
