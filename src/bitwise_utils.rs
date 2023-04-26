/// Sign extension of numbers, when the number is smaller than 16 bits
/// TODO: Research https://en.wikipedia.org/wiki/Two%27s_complement
pub fn sign_extend(x: u16, bit_count: u16) -> i16 {
    let mut x_arg: i16 = x.try_into().unwrap();

    // Check if the most significant bit is a 1. If it is then fill in 1s for the
    // left padding. We do AND 1 on the number to remove the possibility that the
    // most significant digits are 1
    if ((x_arg >> (bit_count - 1)) & 1) == 1 {
        x_arg |= i16::MAX << bit_count;
    }
    x_arg
}