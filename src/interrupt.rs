#[derive(Debug, PartialEq)]
pub enum Interrupt {
    DivideByZero = 1,
    AbsOpFromZero = 2, // addfz 0, 1 is invalid
    InvalidOpcode = 3,
    SpaceFault = 4, // unmapped space access
    MemoryFault = 5, // word access on space boundary
    Halted = 6,

    BadCode = -1, // not a cpu interrupt, just bad code
    Breakpoint = -2,
}

pub type Result<T> = std::result::Result<T, Interrupt>;
