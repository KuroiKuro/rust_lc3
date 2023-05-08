#[cfg(test)]
mod tests;

use super::{registers::RegisterName, Lc3Vm};
use ascii::AsciiChar;
use std::io::{self, Write};

impl Lc3Vm {
    /// Write a string of ASCII characters to the console display.
    /// The characters are contained in consecutive memory locations,
    /// one character per memory location, starting with the address
    /// specified in R0.
    /// Writing terminates with the occurrence of x0000 in a memory location
    fn puts<W>(&mut self, output_writer: &mut W)
    where
        W: Write,
    {
        let str_start_addr = self.registers.get_reg_value(RegisterName::R0);

        let mut current_addr = str_start_addr;
        loop {
            let mem_data = self.memory.read(current_addr);
            // If null character (0) then terminate
            if mem_data == 0 {
                break;
            }

            // Convert the u16 to u8, truncating the most significant bits
            let byte_slice: [u8; 2] = mem_data.to_be_bytes();
            let char_byte = byte_slice[1];
            let ascii_char = match AsciiChar::from_ascii(char_byte) {
                Ok(ch) => ch,
                Err(_) => panic!("Invalid ASCII character encountered in puts trap vector!"),
            };

            let print_char = ascii_char.as_char();
            write!(output_writer, "{}", print_char).unwrap();
            current_addr += 1;
        }
    }
}
