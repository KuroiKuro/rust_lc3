/// Maximum size a `u16` can hold
const MEMORY_MAX: usize = 1 << 16;

#[derive(Clone, Copy)]
pub struct MemorySlice(u16);

impl MemorySlice {
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set_value(&mut self, value: u16) {
        self.0 = value;
    }
}

pub struct Memory {
    mem_arr: [MemorySlice; MEMORY_MAX]
}

impl Memory {
    pub fn new() -> Self {
        let mem_arr: [MemorySlice; MEMORY_MAX] = [MemorySlice(0); MEMORY_MAX];
        Self {mem_arr}
    }

    /// Gets the value at the given memory address
    /// 
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn get_mem_value(&self, address: usize) -> u16 {
        if address > MEMORY_MAX {
            panic!("Attempted to access invalid memory address: {}", address);
        }
        self.mem_arr[address].value()
    }

    /// Sets the value at the given memory address
    /// 
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn set_mem_value(&mut self, address: usize, value: u16) {
        if address > MEMORY_MAX {
            panic!("Attempted to write to invalid memory address: {}", address);
        }
        self.mem_arr[address].set_value(value);
    }
}
