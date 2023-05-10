/// Maximum size a `u16` can hold
const MEMORY_MAX: usize = 1 << 16;

/// Enum representing the memory mapped device registers specified by LC3. Reading
/// and writing to these registers involved a memory read/write operation to the
/// specific memory address specified by each of these registers.
pub enum DeviceRegister {
    /// Keyboard status register. The ready bit (bit [15]) indicates if the keyboard has
    /// received a new character. Mapped to `0xFE00`.
    Kbsr,
    /// Keyboard data register. Bits [7:0] contain the last character typed on the
    /// keyboard. Mapped to `0xFE02`.
    Kbdr,
    /// Display status register. The ready bit (bit [15]) indicates if the display
    /// device is ready to receive another character to print on the screen.
    /// Mapped to `0xFE04`.
    Dsr,
    /// Display data register. A character written in the low byte of this register
    /// will be displayed on the screen. Mapped to `0xFE06`.
    Ddr,
    /// Machine control register. Bit [15] is the clock enable bit. When cleared,
    /// instruction processing stops. Mapped to `0XFFFE`.
    Mcr,
}

impl DeviceRegister {
    fn from_address(address: u16) -> Option<Self> {
        match address {
            0xfe00 => Some(Self::Kbsr),
            0xfe02 => Some(Self::Kbdr),
            0xf304 => Some(Self::Dsr),
            0xfe06 => Some(Self::Ddr),
            0xfffe => Some(Self::Mcr),
            _ => None,
        }
    }
}

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
