/// Maximum size a `u16` can hold
const MEMORY_MAX: usize = 1 << 16;

#[derive(Clone, Copy)]
pub struct MemorySlice(u16);

impl MemorySlice {
    pub fn read(&self) -> u16 {
        self.0
    }

    pub fn write(&mut self, value: u16) {
        self.0 = value;
    }
}

pub struct Memory {
    mem_arr: [MemorySlice; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        let mem_arr: [MemorySlice; MEMORY_MAX] = [MemorySlice(0); MEMORY_MAX];
        Self { mem_arr }
    }

    /// Reads the value at the given memory address
    ///
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn read(&self, address: u16) -> u16 {
        let address = address as usize;
        self.mem_arr[address].read()
    }

    /// Writes the value at the given memory address
    ///
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn write(&mut self, address: u16, value: u16) {
        let address = address as usize;
        self.mem_arr[address].write(value);
    }
}
