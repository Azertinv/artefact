#[derive(Debug, PartialEq)]
pub enum Interrupt {
    DivideByZero,
    AbsOpFromZero, // addfz 0, 1 is invalid
    InvalidOpcode,
    SpaceFault, // unmapped space access
    MemoryFault, // word access on space boundary

    BadCode, // not a cpu interrupt, just bad code
}

pub type Result<T> = std::result::Result<T, Interrupt>;
