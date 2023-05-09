#[cfg(test)]
mod tests;

use super::{registers::RegisterName, Lc3Vm};
use ascii::AsciiChar;
use std::io::{self, Read, Write};

const IN_TROUTINE_PROMPT: &str = "Enter a character: ";

impl Lc3Vm {
    /// Read a single character from the keyboard. The character is not echoed onto
    /// the console. Its ASCII code is copied into R0.
    /// The high eight bits of R0 are cleared.
    fn getc_troutine<R>(&mut self, input_reader: &mut R)
    where
        R: Read,
    {
        let mut read_char: [u8; 1] = [0];
        input_reader.read_exact(&mut read_char).unwrap();
        let ascii_char = AsciiChar::new(read_char[0] as char);
        self.registers
            .set_reg_value(RegisterName::R0, ascii_char as u16);
    }

    /// Write a character in R0[7:0] to the console display.
    fn out_troutine<W>(&mut self, output_writer: &mut W)
    where
        W: Write
    {
        let read_data = self.registers.get_reg_value(RegisterName::R0);
        // Read least significant bits for parsing ascii character to print
        let byte_slice: [u8; 2] = read_data.to_be_bytes();
        let char_byte = byte_slice[1];
        let ascii_char = AsciiChar::from_ascii(char_byte).unwrap();
        write!(output_writer, "{}", ascii_char).unwrap();
    }

    /// Write a string of ASCII characters to the console display.
    /// The characters are contained in consecutive memory locations,
    /// one character per memory location, starting with the address
    /// specified in R0.
    /// Writing terminates with the occurrence of x0000 in a memory location
    fn puts_troutine<W>(&mut self, output_writer: &mut W)
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

            write!(output_writer, "{}", ascii_char).unwrap();
            current_addr += 1;
        }
    }

    /// Print a prompt on the screen and read a single character from the keyboard.
    /// The character is echoed onto the console monitor, and its ASCII code is
    /// copied into R0. The high eight bits of R0 are cleared.
    fn in_troutine<R, W>(&mut self, input_reader: &mut R, output_writer: &mut W)
    where
        R: Read,
        W: Write,
    {
        // We specify our own prompt
        write!(output_writer, "{}", IN_TROUTINE_PROMPT).unwrap();
        let mut input_buf: [u8; 1] = [0];
        input_reader.read_exact(&mut input_buf).unwrap();
        output_writer.write_all(&input_buf).unwrap();
        let ascii_char = AsciiChar::new(input_buf[0] as char);
        self.registers.set_reg_value(RegisterName::R0, ascii_char as u16);
    }

    /// Write a string of ASCII characters to the console. The characters are
    /// contained in consecutive memory locations, two characters per memory location,
    /// starting with the address specified in R0.
    /// The ASCII code contained in bits [7:0] of a memory location is written to the
    /// console first. Then the ASCII code contained in bits [15:8] of that memory
    /// location is written to the console.
    /// 
    /// (A character string consisting of an odd number of characters to be written
    /// will have x00 in bits [15:8] of the memory location containing the last
    /// character to be written.)
    /// 
    /// Writing terminates with the occurrence of x0000 in a memory location
    fn putsp_troutine(&mut self, output_writer: &mut impl Write) {
        let start_address = self.registers.get_reg_value(RegisterName::R0);
        let mut current_address = start_address;
        loop {
            let mem_data = self.memory.read(current_address);
            if mem_data == 0 {
                break;
            }

            let bytes_slice: [u8; 2] = mem_data.to_be_bytes();
            let first_char = AsciiChar::new(bytes_slice[1] as char);
            let second_char = AsciiChar::new(bytes_slice[0] as char);
            
            write!(output_writer, "{}{}", first_char, second_char).unwrap();
            current_address += 1;
        }
    }
}
