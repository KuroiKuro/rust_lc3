mod registers;

const MEMORY_MAX: usize = 1 << 16;

pub struct Lc3Vm {
    memory: [u16; MEMORY_MAX],
}