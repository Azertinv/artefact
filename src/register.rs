use crate::word::Word;

type Reg = Word;

#[derive(Debug)]
pub struct Registers {
    pc: Reg, // program counter
    sp: Reg, // stack pointer
    flags: Reg, // comparaison result
    a: Reg,
    b: Reg,
    c: Reg,
    d: Reg,
    e: Reg,
    f: Reg,
}

impl Registers {
    pub fn new() -> Registers {
        Registers{
            pc: Word::ZERO,
            sp: Word::ZERO,
            flags: Word::ZERO,
            a: Word::ZERO,
            b: Word::ZERO,
            c: Word::ZERO,
            d: Word::ZERO,
            e: Word::ZERO,
            f: Word::ZERO,
        }
    }
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"pc: {}, sp: {}, fl: {}\n\
                  a:  {}, b:  {}, c:  {}\n\
                  d:  {}, e:  {}, f:  {}",
                  self.pc, self.sp, self.flags,
                  self.a, self.b, self.c,
                  self.d, self.e, self.f,
                  )
    }
}
