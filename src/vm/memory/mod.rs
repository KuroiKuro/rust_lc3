#[cfg(test)]
mod tests;

use std::io::{Read, Write, stdin, stdout};

use ascii::AsciiChar;

/// Maximum size a `u16` can hold
const MEMORY_MAX: usize = 1 << 16;

/// Enum representing the memory mapped device registers specified by LC3. Reading
/// and writing to these registers involved a memory read/write operation to the
/// specific memory address specified by each of these registers.
pub enum DeviceRegister {
    /// Keyboard status register. The ready bit (bit [15]) indicates if the keyboard has
    /// received a new character. Mapped to `0xFE00`.
    Kbsr = 0,
    /// Keyboard data register. Bits [7:0] contain the last character typed on the
    /// keyboard. Mapped to `0xFE02`.
    Kbdr = 1,
    /// Display status register. The ready bit (bit [15]) indicates if the display
    /// device is ready to receive another character to print on the screen.
    /// Mapped to `0xFE04`.
    Dsr = 2,
    /// Display data register. A character written in the low byte of this register
    /// will be displayed on the screen. Mapped to `0xFE06`.
    Ddr = 3,
    /// Machine control register. Bit [15] is the clock enable bit. When cleared,
    /// instruction processing stops. Mapped to `0XFFFE`.
    Mcr = 4,
}

impl DeviceRegister {
    const KBSR_ADDR: u16 = 0xfe00;
    const KBDR_ADDR: u16 = 0xfe02;
    const DSR_ADDR: u16 = 0xfe04;
    const DDR_ADDR: u16 = 0xfe06;
    const MCR_ADDR: u16 = 0xfffe;

    pub fn from_address(address: u16) -> Option<Self> {
        match address {
            Self::KBSR_ADDR => Some(Self::Kbsr),
            Self::KBDR_ADDR => Some(Self::Kbdr),
            Self::DSR_ADDR => Some(Self::Dsr),
            Self::DDR_ADDR => Some(Self::Ddr),
            Self::MCR_ADDR => Some(Self::Mcr),
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

/// A struct containing the values of certain device registers where the value has to be saved
struct MmapRegisters {
    kbdr: MemorySlice,
    mcr: MemorySlice,
}

impl MmapRegisters {
    const MCR_DEFAULT_VALUE: u16 = 0x8000;
    fn new() -> Self {
        Self {
            kbdr: MemorySlice(0),
            mcr: MemorySlice(Self::MCR_DEFAULT_VALUE),
        }
    }
}

pub struct Memory {
    mem_arr: [MemorySlice; MEMORY_MAX],
    mmap_registers: MmapRegisters,
}

impl Memory {
    pub fn new() -> Self {
        let mem_arr: [MemorySlice; MEMORY_MAX] = [MemorySlice(0); MEMORY_MAX];
        let mmap_registers = MmapRegisters::new();
        Self {
            mem_arr,
            mmap_registers,
        }
    }

    /// Reads the value at the given memory address. If the address corresponds to
    /// a memory mapped device register, the read action of that specific register
    /// will be performed
    ///
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn read(&mut self, address: u16) -> u16 {
        match DeviceRegister::from_address(address) {
            None => self.mem_arr[address as usize].read(),
            Some(device_register) => self.read_device_register(device_register),
        }
    }

    /// A handler function that calls the correct function to read the given `DeviceRegister`
    fn read_device_register(&mut self, device_register: DeviceRegister) -> u16 {
        match device_register {
            DeviceRegister::Kbsr => {
                let mut stdin = stdin().lock();
                self.read_kbsr(&mut stdin)
            },
            DeviceRegister::Kbdr => self.read_kbdr(),
            DeviceRegister::Dsr => self.read_dsr(),
            DeviceRegister::Ddr => self.read_ddr(),
            DeviceRegister::Mcr => self.read_mcr(),
        }
    }

    /// Writes the value at the given memory address
    ///
    /// # Panics
    /// This method will panic if the given address is larger than the memory size
    /// configured in the constant `MEMORY_MAX`
    pub fn write(&mut self, address: u16, value: u16) {
        match DeviceRegister::from_address(address) {
            None => self.mem_arr[address as usize].write(value),
            Some(device_register) => self.write_device_register(device_register, value)
        }
    }

    fn write_device_register(&mut self, device_register: DeviceRegister, value: u16) {
        match device_register {
            DeviceRegister::Ddr => {
                let mut stdout = stdout().lock();
                self.write_ddr(value, &mut stdout);
            },
            DeviceRegister::Mcr => self.write_mcr(value),
            // Other registers don't have any specified write behaviour, so nothing
            // will happen in our implementation
            _ => ()
        }
    }

    pub fn mcr_is_cleared(&self) -> bool {
        let mcr_value = self.mmap_registers.mcr.read();
        let clock_bit = mcr_value >> 15;
        clock_bit == 0
    }

    pub fn clear_mcr(&mut self) {
        self.mmap_registers.mcr.write(0);
    }

    fn read_kbsr(&mut self, input_reader: &mut impl Read) -> u16 {
        // We need to check if the input has any new character
        let mut buf: [u8; 1] = [0];
        // If read_exact returns Err, then it means there's nothing
        match input_reader.read_exact(&mut buf) {
            Err(_) => 0,
            Ok(_) => {
                // If the read was successfuly, then we must save the character we
                // read to KBDR because it's no longer available in the input.
                // We reuse the actual memory address reserved for Kbdr since it's
                // reserved and nothing else can read it (normally)
                self.mmap_registers.kbdr.write(buf[0] as u16);
                0x8000
            }
        }
    }

    fn read_kbdr(&self) -> u16 {
        self.mmap_registers.kbdr.read()
    }

    /// Handles reading of the `DeviceRegister::Dsr` (Display Status Register). In our
    /// simulated version, the display will always be ready so we always return `0x8000`
    fn read_dsr(&self) -> u16 {
        0x8000
    }

    /// Handles reading of the `DeviceRegister::Ddr` (Display Data Register). The DDR
    /// is designed for writing, so in this implementation, reading from DDR will always
    /// return `0`
    fn read_ddr(&self) -> u16 {
        0
    }

    /// When the program is running, the Machine control register should always return 0x8000,
    /// unless the value has been set to some other value using `write_mcr`.
    fn read_mcr(&self) -> u16 {
        self.mmap_registers.mcr.read()
    }

    /// Writes a value to the Machine control register. Note that you can pass any arbitrary
    /// number here, but in order to actually cause machine processing to stop, the most
    /// significant bit must be `0` in order to work. It is not advised to write anything to
    /// this device register for any other purpose apart from clearing the most signifcant bit
    fn write_mcr(&mut self, value: u16) {
        self.mmap_registers.mcr.write(value);
    }

    fn write_ddr(&mut self, value: u16, output_writer: &mut impl Write) {
        let byte_slice = value.to_be_bytes();
        let ascii_char = AsciiChar::from_ascii(byte_slice[1]).unwrap();
        write!(output_writer, "{}", ascii_char).unwrap();
    }
}
