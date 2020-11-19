use crate::trit::Trit;
use crate::byte::Byte;
use crate::word::Word;

type Reg = Word;

#[derive(Debug)]
pub struct Registers {
    pub pc: Reg, // program counter
    pub sp: Reg, // stack pointer
    pub flags: Reg, // comparaison result
    pub a: Reg,
    pub b: Reg,
    pub c: Reg,
    pub d: Reg,
    pub e: Reg,
    pub f: Reg,
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

    pub fn get(&self, index: &[Trit]) -> Word {
        assert_eq!(index.len(), 2);
        match (index[0], index[1]) {
            (Trit::TERN, Trit::TERN) => {self.pc},
            (Trit::TERN, Trit::ZERO) => {self.sp},
            (Trit::TERN, Trit::ONE)  => {self.flags},
            (Trit::ZERO, Trit::TERN) => {self.a},
            (Trit::ZERO, Trit::ZERO) => {self.b},
            (Trit::ZERO, Trit::ONE)  => {self.c},
            (Trit::ONE,  Trit::TERN) => {self.d},
            (Trit::ONE,  Trit::ZERO) => {self.e},
            (Trit::ONE,  Trit::ONE)  => {self.f},
            _ => { panic!() },
        }
    }

    pub fn set_w(&mut self, index: &[Trit], value: Word) {
        assert_eq!(index.len(), 2);
        let reg = match (index[0], index[1]) {
            (Trit::TERN, Trit::TERN) => {&mut self.pc},
            (Trit::TERN, Trit::ZERO) => {&mut self.sp},
            (Trit::TERN, Trit::ONE)  => {&mut self.flags},
            (Trit::ZERO, Trit::TERN) => {&mut self.a},
            (Trit::ZERO, Trit::ZERO) => {&mut self.b},
            (Trit::ZERO, Trit::ONE)  => {&mut self.c},
            (Trit::ONE,  Trit::TERN) => {&mut self.d},
            (Trit::ONE,  Trit::ZERO) => {&mut self.e},
            (Trit::ONE,  Trit::ONE)  => {&mut self.f},
            _ => { panic!() },
        };
        *reg = value;
    }

    pub fn set_b(&mut self, index: &[Trit], value: Byte) {
        assert_eq!(index.len(), 2);
        let reg = match (index[0], index[1]) {
            (Trit::TERN, Trit::TERN) => {&mut self.pc},
            (Trit::TERN, Trit::ZERO) => {&mut self.sp},
            (Trit::TERN, Trit::ONE)  => {&mut self.flags},
            (Trit::ZERO, Trit::TERN) => {&mut self.a},
            (Trit::ZERO, Trit::ZERO) => {&mut self.b},
            (Trit::ZERO, Trit::ONE)  => {&mut self.c},
            (Trit::ONE,  Trit::TERN) => {&mut self.d},
            (Trit::ONE,  Trit::ZERO) => {&mut self.e},
            (Trit::ONE,  Trit::ONE)  => {&mut self.f},
            _ => { panic!() },
        };
        reg.bytes[0] = value;
    }

    pub fn set_t(&mut self, index: &[Trit], value: Byte) {
        assert_eq!(index.len(), 2);
        let reg = match (index[0], index[1]) {
            (Trit::TERN, Trit::TERN) => {&mut self.pc},
            (Trit::TERN, Trit::ZERO) => {&mut self.sp},
            (Trit::TERN, Trit::ONE)  => {&mut self.flags},
            (Trit::ZERO, Trit::TERN) => {&mut self.a},
            (Trit::ZERO, Trit::ZERO) => {&mut self.b},
            (Trit::ZERO, Trit::ONE)  => {&mut self.c},
            (Trit::ONE,  Trit::TERN) => {&mut self.d},
            (Trit::ONE,  Trit::ZERO) => {&mut self.e},
            (Trit::ONE,  Trit::ONE)  => {&mut self.f},
            _ => { panic!() },
        };
        reg.bytes[1] = value;
    }

    pub fn to_str(&mut self, index: &[Trit]) -> &'static str {
        assert_eq!(index.len(), 2);
        match (index[0], index[1]) {
            (Trit::TERN, Trit::TERN) => {"pc"},
            (Trit::TERN, Trit::ZERO) => {"sp"},
            (Trit::TERN, Trit::ONE)  => {"flags"},
            (Trit::ZERO, Trit::TERN) => {"a"},
            (Trit::ZERO, Trit::ZERO) => {"b"},
            (Trit::ZERO, Trit::ONE)  => {"c"},
            (Trit::ONE,  Trit::TERN) => {"d"},
            (Trit::ONE,  Trit::ZERO) => {"e"},
            (Trit::ONE,  Trit::ONE)  => {"f"},
            _ => { panic!() },
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
